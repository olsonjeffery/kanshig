//! Profile module for kanshi config

use serde::{Deserialize, Serialize};

/// Represents an assignment of an output in a profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputAssignment {
    pub alias: String,
    pub enabled: bool,
}

/// Represents a kanshi profile that groups outputs with enable/disable states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
    pub outputs: Vec<OutputAssignment>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_assignment_serialization() {
        let assignment = OutputAssignment {
            alias: "$INTERNAL".to_string(),
            enabled: true,
        };

        let json = serde_json::to_string(&assignment).unwrap();
        assert!(json.contains("$INTERNAL"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_profile_serialization() {
        let profile = Profile {
            name: "undocked".to_string(),
            outputs: vec![
                OutputAssignment {
                    alias: "$INTERNAL".to_string(),
                    enabled: true,
                },
                OutputAssignment {
                    alias: "$HOME_0".to_string(),
                    enabled: false,
                },
            ],
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("undocked"));
        assert!(json.contains("$INTERNAL"));
    }

    #[test]
    fn test_profile_deserialization() {
        let json = r#"{"name":"home_dock","outputs":[{"alias":"$INTERNAL","enabled":false},{"alias":"$HOME_0","enabled":true}]}"#;
        let profile: Profile = serde_json::from_str(json).unwrap();

        assert_eq!(profile.name, "home_dock");
        assert_eq!(profile.outputs.len(), 2);
        assert_eq!(profile.outputs[0].alias, "$INTERNAL");
        assert!(!profile.outputs[0].enabled);
        assert_eq!(profile.outputs[1].alias, "$HOME_0");
        assert!(profile.outputs[1].enabled);
    }
}
