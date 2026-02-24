//! Output module for kanshi config

use serde::{Deserialize, Serialize};

/// Represents a display output definition in the kanshi config
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputDefinition {
    pub name: String,
    pub mode: String,
    pub position: String,
    pub scale: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// Represents a unified output that combines kanshi config data with niri detection info
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedOutput {
    pub name: String,
    pub mode: String,
    pub position: String,
    pub scale: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Whether this output is detected in niri outputs
    pub detected: bool,
    /// Whether this output is configured in kanshi config
    pub configured: bool,
}

impl UnifiedOutput {
    /// Create a new unified output from a config output
    pub fn from_config(output: OutputDefinition) -> Self {
        UnifiedOutput {
            name: output.name,
            mode: output.mode,
            position: output.position,
            scale: output.scale,
            alias: output.alias,
            detected: false,
            configured: true,
        }
    }

    /// Mark this output as detected (found in niri outputs)
    pub fn mark_detected(&mut self) {
        self.detected = true;
    }

    /// Check if this output has the DETECTED label
    pub fn is_detected(&self) -> bool {
        self.detected
    }

    /// Check if this output has the CONFIGURED label
    pub fn is_configured(&self) -> bool {
        self.configured
    }
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

    #[test]
    fn test_unified_output_creation() {
        let config_output = OutputDefinition {
            name: "Test Output".to_string(),
            mode: "1920x1080@60.000".to_string(),
            position: "0,0".to_string(),
            scale: 1.0,
            alias: None,
        };

        let unified = UnifiedOutput::from_config(config_output.clone());

        assert!(unified.is_configured());
        assert!(!unified.is_detected());
        assert_eq!(unified.name, "Test Output");
    }

    #[test]
    fn test_unified_output_mark_detected() {
        let mut unified = UnifiedOutput::from_config(OutputDefinition {
            name: "Test Output".to_string(),
            mode: "1920x1080@60.000".to_string(),
            position: "0,0".to_string(),
            scale: 1.0,
            alias: None,
        });

        assert!(!unified.is_detected());
        unified.mark_detected();
        assert!(unified.is_detected());
    }

    #[test]
    fn test_unified_output_both_flags() {
        let mut unified = UnifiedOutput::from_config(OutputDefinition {
            name: "Test Output".to_string(),
            mode: "1920x1080@60.000".to_string(),
            position: "0,0".to_string(),
            scale: 1.0,
            alias: None,
        });

        unified.mark_detected();

        assert!(unified.is_configured());
        assert!(unified.is_detected());
    }
}
