//! Niri output module for parsing niri msg --json outputs

use serde::{Deserialize, Serialize};

/// Represents a single niri output/display
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NiriOutput {
    pub name: String,
    pub make: Option<String>,
    pub model: Option<String>,
    pub serial: Option<String>,
    #[serde(rename = "physical_size")]
    pub physical_size: Option<Vec<u32>>,
    pub modes: Vec<NiriMode>,
    #[serde(rename = "current_mode")]
    pub current_mode: Option<usize>,
    #[serde(rename = "is_custom_mode")]
    pub is_custom_mode: bool,
    #[serde(rename = "vrr_supported")]
    pub vrr_supported: bool,
    #[serde(rename = "vrr_enabled")]
    pub vrr_enabled: bool,
    pub logical: Option<NiriLogical>,
}

/// Represents a display mode in niri
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NiriMode {
    pub width: u32,
    pub height: u32,
    #[serde(rename = "refresh_rate")]
    pub refresh_rate: u32,
    #[serde(rename = "is_preferred")]
    pub is_preferred: bool,
}

/// Represents logical properties of a display
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NiriLogical {
    pub x: i32,
    pub y: i32,
    #[serde(rename = "width")]
    pub width: u32,
    #[serde(rename = "height")]
    pub height: u32,
    pub scale: f64,
    pub transform: String,
}

/// Represents the collection of outputs from niri
pub type NiriOutputs = std::collections::HashMap<String, NiriOutput>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_niri_output_serialization() {
        let output = NiriOutput {
            name: "DP-8".to_string(),
            make: Some("LG Electronics".to_string()),
            model: Some("LG ULTRAGEAR".to_string()),
            serial: Some("112NTBKD6701".to_string()),
            physical_size: Some(vec![700, 390]),
            modes: vec![NiriMode {
                width: 2560,
                height: 1440,
                refresh_rate: 119998,
                is_preferred: true,
            }],
            current_mode: Some(0),
            is_custom_mode: false,
            vrr_supported: false,
            vrr_enabled: false,
            logical: Some(NiriLogical {
                x: 0,
                y: 0,
                width: 2048,
                height: 1152,
                scale: 1.25,
                transform: "Normal".to_string(),
            }),
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("DP-8"));
        assert!(json.contains("LG Electronics"));
    }

    #[test]
    fn test_niri_output_deserialization() {
        let json = r#"{"name":"DP-8","make":"LG Electronics","model":"LG ULTRAGEAR","serial":"112NTBKD6701","physical_size":[700,390],"modes":[{"width":2560,"height":1440,"refresh_rate":119998,"is_preferred":true}],"current_mode":0,"is_custom_mode":false,"vrr_supported":false,"vrr_enabled":false,"logical":{"x":0,"y":0,"width":2048,"height":1152,"scale":1.25,"transform":"Normal"}}"#;
        let output: NiriOutput = serde_json::from_str(json).unwrap();

        assert_eq!(output.name, "DP-8");
        assert_eq!(output.make, Some("LG Electronics".to_string()));
        assert_eq!(output.model, Some("LG ULTRAGEAR".to_string()));
        assert_eq!(output.serial, Some("112NTBKD6701".to_string()));
        assert_eq!(output.physical_size, Some(vec![700, 390]));
        assert_eq!(output.modes.len(), 1);
        assert_eq!(output.modes[0].width, 2560);
    }

    #[test]
    fn test_niri_outputs_map_deserialization() {
        let json = r#"{"DP-8":{"name":"DP-8","make":"LG Electronics","model":"LG ULTRAGEAR","serial":"112NTBKD6701","physical_size":[700,390],"modes":[{"width":2560,"height":1440,"refresh_rate":119998,"is_preferred":true}],"current_mode":0,"is_custom_mode":false,"vrr_supported":false,"vrr_enabled":false,"logical":{"x":0,"y":0,"width":2048,"height":1152,"scale":1.25,"transform":"Normal"}},"eDP-1":{"name":"eDP-1","make":"Lenovo Group Limited","model":"B140UAN02.7 ","serial":null,"physical_size":[300,190],"modes":[{"width":1920,"height":1200,"refresh_rate":60000,"is_preferred":true}],"current_mode":null,"is_custom_mode":false,"vrr_supported":true,"vrr_enabled":false,"logical":null}}"#;
        let outputs: NiriOutputs = serde_json::from_str(json).unwrap();

        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains_key("DP-8"));
        assert!(outputs.contains_key("eDP-1"));
        assert_eq!(outputs.get("DP-8").unwrap().name, "DP-8");
    }
}
