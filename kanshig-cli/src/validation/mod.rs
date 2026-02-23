//! Validation module for kanshi config files

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing closing brace for {0}")]
    MissingClosingBrace(String),
    #[error("Invalid section type: {0}")]
    InvalidSectionType(String),
    #[error("Invalid output name format: {0}")]
    InvalidOutputName(String),
    #[error("Unexpected parameter in {0}: {1}")]
    UnexpectedParameter(String, String),
}

/// Validate a kanshi config file content
pub fn validate_config(content: &str) -> Result<(), ValidationError> {
    let mut brace_count = 0;
    let mut in_output = false;
    let mut in_profile = false;
    let mut section_name = String::new();
    let mut saw_opening_brace = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Track brace count
        for ch in trimmed.chars() {
            match ch {
                '{' => {
                    brace_count += 1;
                    if brace_count == 1 && !saw_opening_brace {
                        // Parse section name (only on first opening brace)
                        saw_opening_brace = true;
                        parse_section_name(
                            trimmed,
                            &mut in_output,
                            &mut in_profile,
                            &mut section_name,
                        )?;
                    }
                }
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        // Section ended
                        in_output = false;
                        in_profile = false;
                        section_name.clear();
                        saw_opening_brace = false;
                    }
                }
                _ => {}
            }
        }

        // Validate parameters inside sections (only when we're inside a section)
        // Skip lines that are section declarations (start with "output " or "profile ")
        // But only skip them if they contain an opening brace on the same line
        if brace_count > 0 && !trimmed.starts_with("output ") && !trimmed.starts_with("profile ") {
            validate_parameter(trimmed, in_output, in_profile)?;
        }
    }

    // Check for unmatched braces
    if brace_count != 0 {
        return Err(ValidationError::MissingClosingBrace(section_name));
    }

    Ok(())
}

fn parse_section_name(
    line: &str,
    in_output: &mut bool,
    in_profile: &mut bool,
    section_name: &mut String,
) -> Result<(), ValidationError> {
    // Extract the section type and name
    if line.starts_with("output ") {
        *in_output = true;
        // Extract name between quotes
        if let Some(start) = line.find('"')
            && let Some(end) = line[start + 1..].find('"')
        {
            let name = &line[start + 1..start + 1 + end];
            section_name.push_str(name);
            return Ok(());
        }
        return Err(ValidationError::InvalidOutputName(line.to_string()));
    } else if line.starts_with("profile ") {
        *in_profile = true;
        // Extract name after "profile "
        let rest = line.trim_start_matches("profile ");
        let name = rest.split_whitespace().next().unwrap_or("");
        section_name.push_str(name);
        return Ok(());
    }

    Err(ValidationError::InvalidSectionType(line.to_string()))
}

fn validate_parameter(
    line: &str,
    in_output: bool,
    in_profile: bool,
) -> Result<(), ValidationError> {
    let valid_output_params = ["mode", "position", "scale", "alias"];
    let valid_profile_params = ["output"];

    // Determine which params are valid based on current section
    let params = if in_output {
        &valid_output_params[..]
    } else if in_profile {
        &valid_profile_params[..]
    } else {
        return Ok(()); // Should not happen - we should be inside a section
    };

    // Parse the parameter name (first word)
    let param_name = line.split_whitespace().next().unwrap_or("");

    if !params.contains(&param_name) {
        return Err(ValidationError::UnexpectedParameter(
            if in_output { "output" } else { "profile" }.to_string(),
            param_name.to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = r#"
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
        assert!(validate_config(config).is_ok());
    }

    #[test]
    fn test_missing_closing_brace() {
        let config = r#"
output "LG Electronics LG ULTRAGEAR 112NTKFD6717" {
 mode 2560x1440@119.998
"#;
        assert!(validate_config(config).is_err());
    }

    #[test]
    fn test_invalid_parameter() {
        let config = r#"
output "LG Electronics LG ULTRAGEAR 112NTKFD6717" {
 invalid_param value
}
"#;
        assert!(validate_config(config).is_err());
    }

    #[test]
    fn test_empty_config() {
        let config = "";
        assert!(validate_config(config).is_ok());
    }
}
