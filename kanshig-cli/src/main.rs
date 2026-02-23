#![deny(warnings)]

use clap::Parser;
use std::fs;

mod model;
mod niri;
mod parser;
mod validation;

/// kanshig - A TUI application for generating and updating Kanshi configs
#[derive(Parser, Debug)]
#[command(name = "kanshig")]
#[command(author = "jeff")]
#[command(version = "0.1.0")]
#[command(about = "Generate and update Kanshi configs from window manager state", long_about = None)]
struct Args {
    /// Load the kanshi config from a custom location
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    // Initialize logging
    env_logger::init();

    let args = Args::parse();

    // Determine the config path to load
    let config_path = if let Some(path) = &args.config {
        path.clone()
    } else {
        // Default kanshi config location
        format!(
            "{}/.config/kanshi/config",
            std::env::var("HOME").unwrap_or_else(|_| String::from("/"))
        )
    };

    log::info!("Loading kanshi config from: {}", config_path);

    // Check if the file exists and load it
    let path = std::path::Path::new(&config_path);
    if path.exists() {
        log::info!("Config file found at: {}", config_path);

        // Load the file as a string
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                log::info!("Config file content loaded successfully");

                // Validate the config
                match validation::validate_config(&content) {
                    Ok(_) => {
                        log::info!("Config validation passed");

                        // Parse into data model structs
                        match parser::parse_config(&content) {
                            Ok(config) => {
                                log::info!("Config parsed into data model structs");

                                // Display the parsed config
                                log::info!("Parsed Kanshi Config:");
                                log::info!("  Outputs: {}", config.outputs.len());
                                for output in &config.outputs {
                                    log::info!(
                                        "    - {}: {} (scale: {}, position: {})",
                                        output.name,
                                        output.mode,
                                        output.scale,
                                        output.position
                                    );
                                    if let Some(alias) = &output.alias {
                                        log::info!("      Alias: {}", alias);
                                    }
                                }

                                log::info!("  Profiles: {}", config.profiles.len());
                                for profile in &config.profiles {
                                    log::info!(
                                        "    - {}: {} outputs",
                                        profile.name,
                                        profile.outputs.len()
                                    );
                                    for output in &profile.outputs {
                                        let status = if output.enabled {
                                            "enabled"
                                        } else {
                                            "disabled"
                                        };
                                        log::info!("      - {} {}", output.alias, status);
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed to parse config into data model structs: {}",
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Config validation failed: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read config file: {}", e);
            }
        }
    } else {
        log::warn!("Config file not found: {}", config_path);
    }

    // Call niri msg --json outputs and display the results
    log::info!("Calling niri msg --json outputs...");
    match niri::get_niri_outputs() {
        Ok(outputs) => {
            log::info!("Successfully retrieved {} niri outputs:", outputs.len());
            for (name, output) in outputs.iter() {
                log::info!(
                    "  - {}: {} {}",
                    name,
                    output.make.as_ref().unwrap_or(&String::new()),
                    output.model.as_ref().unwrap_or(&String::new())
                );
            }
        }
        Err(e) => {
            log::warn!("Failed to retrieve niri outputs: {}", e);
        }
    }

    log::info!("kanshig CLI initialized successfully");
}
