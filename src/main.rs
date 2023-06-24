#![allow(dead_code)]

mod config;
mod gsettings;
mod handlers;
mod theme;
mod util;
mod logging;

use chrono::Local;
use clap::{Parser, Subcommand};
use config::default_path;
use handlers::handle_edit_cmd;
use log::{error, info, trace, warn};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Handle,
};
use std::{borrow::Borrow, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to config file - see project readme for config file description
    #[arg(long, value_name = "FILE", value_parser = util::file_exists)]
    config: Option<PathBuf>,

    /// Run in verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Path to log file - if not specified logs are printed to stdout
    #[arg(long, value_name = "FILE")]
    log_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Sets theme by name or basing on current time
    Set {
        /// Name of the theme to apply
        name: Option<String>,
    },

    /// Retrieves current configuration and prints it to logfile or stdout
    Get,

    /// Opens config file in a editor allowing for modification
    Edit {
        /// Path do editor binary. It will be called in a following way:
        /// editor PATH_TO_CONFIG_FILE
        /// If not specified $EDITOR env var will be used. If that is not defined the operation is a
        /// noop.
        editor: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let _log_handle = logging::init_logging(&cli);

    let config = match config::load_config(&cli) {
        Ok(config) => config,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    let gsettings = gsettings::Gsettings::new();

    match cli.command {
        Commands::Set { name } => {
            // First we check whether user specified a concrete theme
            // If no concrete theme was specified we look for theme assigned to current time
            // If no such theme is found we log error and exit gracefully
            if let Some(name) = name {
                // If so, we check wheter theme of given name is present in config file
                // In case such theme does not exist we print error and exit gracefully
                if let Some(theme) = config.theme_for_name(&name) {
                    gsettings.set_theme(theme)
                } else {
                    error!("Failed to find theme for given name: {}", name);
                }
            } else if let Some(theme) = config.theme_for_time(Local::now()) {
                gsettings.set_theme(theme);
            } else {
                error!("Failed to find theme for current time -- not taking any action");
            }
        }
        Commands::Get => {
            let theme = gsettings.get_theme();
            info!("Current theme spec\n{:?}", theme);
        }
        Commands::Edit { editor } => {
            info!("Running Edit command");

            trace!("Resolving config path");
            let config_path = if let Some(ref path) = cli.config {
                path.clone()
            } else if let Some(ref path) = default_path() {
                path.clone()
            } else {
                warn!("Failed to resolve config path");
                return Ok(());
            };

            trace!("Resolving editor name");
            if let Some(ref editor_name) = editor {
                handle_edit_cmd(editor_name, config_path.borrow());
            } else if let Ok(editor_name) = std::env::var("EDITOR") {
                handle_edit_cmd(editor_name.borrow(), config_path.borrow());
            } else {
                warn!("Failed to resolve editor name");
                return Ok(());
            }
        }
    }
    Ok(())
}
