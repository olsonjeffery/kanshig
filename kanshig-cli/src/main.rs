#![deny(warnings)]

use clap::Parser;
use std::path::Path;
use std::fs;

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
        format!("{}/.config/kanshi/config", std::env::var("HOME").unwrap_or_else(|_| String::from("/")))
    };

    log::info!("Loading kanshi config from: {}", config_path);

    // Check if the file exists and load it
    let path = Path::new(&config_path);
    if path.exists() {
        log::info!("Config file found at: {}", config_path);
        
        // Load the file as a string
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                log::info!("Config file content:\n{}", content);
            }
            Err(e) => {
                log::error!("Failed to read config file: {}", e);
            }
        }
    } else {
        log::warn!("Config file not found: {}", config_path);
    }

    log::info!("kanshig CLI initialized successfully");
}
