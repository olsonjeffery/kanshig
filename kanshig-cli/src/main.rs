#![deny(warnings)]

use clap::Parser;
use std::path::Path;
//use std::fs;

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

    // Log the config path if provided-c agent.model.model_kwargs.api_base=${API_BASE}
    if let Some(config_path) = &args.config {
        log::info!("Loading kanshi config from: {}", config_path);

        // Check if the file exists
        let path = Path::new(config_path);
        if path.exists() {
            log::info!("Config file found at: {}", config_path);
        } else {
            log::warn!("Config file not found: {}", config_path);
        }
        //let config_file =
    } else {
        log::info!("No config path provided, using default kanshi config location");
    }

    // TODO: Implement the rest of the application
    log::info!("kanshig CLI initialized successfully");
}
