//! Config module for kanshi config

use serde::{Deserialize, Serialize};

use super::output::OutputDefinition;
use super::profile::Profile;

/// Represents the entire kanshi configuration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct KanshiConfig {
    pub outputs: Vec<OutputDefinition>,
    pub profiles: Vec<Profile>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::profile::OutputAssignment;

    #[test]
    fn test_kanshi_config_serialization() {
        let config = KanshiConfig {
            outputs: vec![OutputDefinition {
                name: "LG Electronics LG ULTRAGEAR 112NTKFD6717".to_string(),
                mode: "2560x1440@119.998".to_string(),
                position: "0,1".to_string(),
                scale: 1.25,
                alias: Some("$HOME_0".to_string()),
            }],
            profiles: vec![Profile {
                name: "undocked".to_string(),
                outputs: vec![OutputAssignment {
                    alias: "$INTERNAL".to_string(),
                    enabled: true,
                }],
            }],
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("LG Electronics"));
        assert!(json.contains("undocked"));
    }

    #[test]
    fn test_kanshi_config_deserialization() {
        let json = r#"{"outputs":[{"name":"Test Output","mode":"1920x1080@60.000","position":"0,0","scale":1.0,"alias":"$TEST"}],"profiles":[{"name":"test_profile","outputs":[{"alias":"$TEST","enabled":true}]}]}"#;
        let config: KanshiConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.outputs.len(), 1);
        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.outputs[0].name, "Test Output");
        assert_eq!(config.profiles[0].name, "test_profile");
    }
}
