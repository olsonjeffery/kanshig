//! Output module for kanshi config

use serde::{Deserialize, Serialize};

/// Represents a display output definition in the kanshi config
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputDefinition {
    pub name: String,
    pub mode: String,
    pub position: String,
    pub scale: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_definition_serialization() {
        let output = OutputDefinition {
            name: "LG Electronics LG ULTRAGEAR 112NTKFD6717".to_string(),
            mode: "2560x1440@119.998".to_string(),
            position: "0,1".to_string(),
            scale: 1.25,
            alias: Some("$HOME_0".to_string()),
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("LG Electronics"));
        assert!(json.contains("2560x1440@119.998"));
    }

    #[test]
    fn test_output_definition_without_alias() {
        let output = OutputDefinition {
            name: "Test Output".to_string(),
            mode: "1920x1080@60.000".to_string(),
            position: "0,0".to_string(),
            scale: 1.0,
            alias: None,
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("Test Output"));
        assert!(!json.contains("alias"));
    }
}
