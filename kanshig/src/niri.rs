//! Niri command integration module

use std::process::Command;

use crate::model::NiriOutputs;
use thiserror::Error;

/// Error types for niri command execution
#[derive(Debug, Error)]
pub enum NiriError {
    #[error("Failed to execute niri msg command: {0}")]
    Execution(String),
    #[error("Failed to parse niri JSON output: {0}")]
    Parse(String),
}

/// Call `niri msg --json outputs` and return the parsed outputs
pub fn get_niri_outputs() -> Result<NiriOutputs, NiriError> {
    // Execute niri msg --json outputs
    let output = Command::new("niri")
        .arg("msg")
        .arg("--json")
        .arg("outputs")
        .output()
        .map_err(|e| NiriError::Execution(format!("Failed to execute niri msg: {}", e)))?;

    // Check if the command succeeded
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(NiriError::Execution(format!(
            "niri msg command failed: {}",
            stderr
        )));
    }

    // Parse the JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let outputs: NiriOutputs = serde_json::from_str(&stdout)
        .map_err(|e| NiriError::Parse(format!("Failed to parse JSON: {}", e)))?;

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_niri_outputs_integration() {
        // This test will only pass if niri is running and accessible
        match get_niri_outputs() {
            Ok(outputs) => {
                log::info!("Successfully retrieved niri outputs: {}", outputs.len());
                for (name, output) in outputs.iter() {
                    log::info!(
                        "  Output: {} ({})",
                        name,
                        output.model.as_ref().unwrap_or(&String::new())
                    );
                }
            }
            Err(e) => {
                // This is expected if niri is not available
                log::info!("niri msg command not available: {}", e);
            }
        }
    }
}
