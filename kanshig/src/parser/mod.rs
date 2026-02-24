//! Parser module for kanshi config files

use crate::model::profile::OutputAssignment;
use crate::model::{KanshiConfig, OutputDefinition, Profile};
use crate::validation::{ValidationError, validate_config};

/// Parse a validated kanshi config string into data model structs
pub fn parse_config(content: &str) -> Result<KanshiConfig, ParseError> {
    // First validate the config
    validate_config(content)?;

    // Then parse it into structs
    Ok(parse_config_unsafe(content))
}

/// Parse a kanshi config string into data model structs (unsafe, no validation)
fn parse_config_unsafe(content: &str) -> KanshiConfig {
    let mut outputs: Vec<OutputDefinition> = Vec::new();
    let mut profiles: Vec<Profile> = Vec::new();

    let mut current_section_type: Option<String> = None;
    let mut current_section_name = String::new();
    let mut current_params: Vec<(String, String)> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Check for section start (output or profile)
        // Only treat as new section if we're not already inside a section
        if (trimmed.starts_with("output ") || trimmed.starts_with("profile "))
            && current_section_type.is_none()
        {
            // Parse new section
            if trimmed.starts_with("output ") {
                current_section_type = Some("output".to_string());
                // Extract name between quotes
                if let Some(start) = trimmed.find('"')
                    && let Some(end) = trimmed[start + 1..].find('"')
                {
                    current_section_name = trimmed[start + 1..start + 1 + end].to_string();
                }
            } else if trimmed.starts_with("profile ") {
                current_section_type = Some("profile".to_string());
                // Extract name after "profile "
                let rest = trimmed.trim_start_matches("profile ");
                current_section_name = rest.split_whitespace().next().unwrap_or("").to_string();
            }

            current_params.clear();
        } else if trimmed.starts_with("}") {
            // Section ended - save it
            if let Some(ref section_type) = current_section_type {
                match section_type.as_str() {
                    "output" => {
                        if !current_section_name.is_empty() {
                            outputs.push(OutputDefinition {
                                name: current_section_name.clone(),
                                mode: get_param_value(&current_params, "mode").unwrap_or_default(),
                                position: get_param_value(&current_params, "position")
                                    .unwrap_or_default(),
                                scale: get_param_f64(&current_params, "scale").unwrap_or(1.0),
                                alias: get_param_value(&current_params, "alias"),
                            });
                        }
                    }
                    "profile" => {
                        if !current_section_name.is_empty() {
                            let profile_outputs = current_params
                                .iter()
                                .filter_map(|(param, value)| {
                                    if param == "output" {
                                        let parts: Vec<&str> = value.split_whitespace().collect();
                                        if parts.len() >= 2 {
                                            Some(OutputAssignment {
                                                alias: parts[0].to_string(),
                                                enabled: parts[1] == "enable",
                                            })
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            profiles.push(Profile {
                                name: current_section_name.clone(),
                                outputs: profile_outputs,
                            });
                        }
                    }
                    _ => {}
                }
            }

            // Reset for next section
            current_section_type = None;
            current_section_name.clear();
            current_params.clear();
        } else {
            // Parse parameters (only when we're inside a section)
            if !current_section_name.is_empty() {
                let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
                if parts.len() >= 2 {
                    current_params.push((parts[0].to_string(), parts[1].to_string()));
                }
            }
        }
    }

    KanshiConfig { outputs, profiles }
}

fn get_param_value(params: &[(String, String)], key: &str) -> Option<String> {
    params
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
}

fn get_param_f64(params: &[(String, String)], key: &str) -> Option<f64> {
    params
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

/// Error types for parsing
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONFIG: &str = r#"
output "LG Electronics LG ULTRAGEAR 112NTKFD6717" {
 mode 2560x1440@119.998
 position 0,1
 scale 1.25
 alias $HOME_0
}

output "Lenovo Group Limited B140UAN02.7  Unknown" {
 mode 1920x1200@60.000
 scale 1
 alias $INTERNAL
}

profile undocked {
 output $INTERNAL enable
}

profile home_dock {
 output $INTERNAL disable
 output $HOME_0 enable
}
"#;

    #[test]
    fn test_parse_config() {
        let config = parse_config(SAMPLE_CONFIG).unwrap();

        assert_eq!(config.outputs.len(), 2);
        assert_eq!(config.profiles.len(), 2);

        // Check first output
        assert_eq!(
            config.outputs[0].name,
            "LG Electronics LG ULTRAGEAR 112NTKFD6717"
        );
        assert_eq!(config.outputs[0].mode, "2560x1440@119.998");
        assert_eq!(config.outputs[0].position, "0,1");
        assert_eq!(config.outputs[0].scale, 1.25);
        assert_eq!(config.outputs[0].alias.as_deref(), Some("$HOME_0"));

        // Check second output
        assert_eq!(
            config.outputs[1].name,
            "Lenovo Group Limited B140UAN02.7  Unknown"
        );
        assert_eq!(config.outputs[1].mode, "1920x1200@60.000");
        assert_eq!(config.outputs[1].scale, 1.0);
        assert_eq!(config.outputs[1].alias.as_deref(), Some("$INTERNAL"));

        // Check profiles
        assert_eq!(config.profiles[0].name, "undocked");
        assert_eq!(config.profiles[0].outputs.len(), 1);
        assert_eq!(config.profiles[0].outputs[0].alias, "$INTERNAL");
        assert!(config.profiles[0].outputs[0].enabled);

        assert_eq!(config.profiles[1].name, "home_dock");
        assert_eq!(config.profiles[1].outputs.len(), 2);
        assert_eq!(config.profiles[1].outputs[0].alias, "$INTERNAL");
        assert!(!config.profiles[1].outputs[0].enabled);
        assert_eq!(config.profiles[1].outputs[1].alias, "$HOME_0");
        assert!(config.profiles[1].outputs[1].enabled);
    }

    #[test]
    fn test_empty_config() {
        let config = parse_config("").unwrap();
        assert_eq!(config.outputs.len(), 0);
        assert_eq!(config.profiles.len(), 0);
    }

    #[test]
    fn test_single_output() {
        let config = parse_config(
            r#"
output "Test" {
 mode 1920x1080@60.000
 position 0,0
 scale 1.0
}
"#,
        )
        .unwrap();

        assert_eq!(config.outputs.len(), 1);
        assert_eq!(config.outputs[0].name, "Test");
    }

    #[test]
    fn test_single_profile() {
        let config = parse_config(
            r#"
profile test {
 output $TEST enable
}
"#,
        )
        .unwrap();

        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles[0].name, "test");
        assert_eq!(config.profiles[0].outputs.len(), 1);
    }

    #[test]
    fn test_invalid_config_fails() {
        // Missing closing brace
        let result = parse_config(
            r#"
output "Test" {
 mode 1920x1080@60.000
"#,
        );
        assert!(result.is_err());
    }
}
