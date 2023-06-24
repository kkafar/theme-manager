use std::{path::Path, process::Command, borrow::Borrow};

use chrono::Local;
use log::{info, warn, error, trace};

use crate::{command::Commands, cli::Args, gsettings, config::Config};

pub fn handle_cmd(args: Args, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let gsettings = gsettings::Gsettings::new();
    match args.command {
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
        },
        Commands::Get => {
            let theme = gsettings.get_theme();
            info!("Current theme spec\n{:?}", theme);
        },
        Commands::Edit { editor } => {
            info!("Running Edit command");

            trace!("Resolving config path");
            let config_path = if let Some(ref path) = args.config {
                path.clone()
            } else if let Some(ref path) = crate::config::default_path() {
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

pub fn handle_edit_cmd(editor: &str, config_path: &Path) {
    info!("Handling edit cmd with editor: {} for config: {}", editor, config_path.to_owned().to_str().unwrap_or("Failed to parse config path"));
    if editor.is_empty() {
        warn!("Handling interrupted due to empty editor path");
        return;
    }

    match Command::new(editor).arg(config_path).status() {
        Ok(status) => {
            if status.success() {
                info!("Editor closed properly with status code 0");
            } else {
                warn!("Editor closed with error status code {} (-1 means unknown)", status.code().unwrap_or(-1));
            }
        },
        Err(err) => {
            warn!("Failed to open the editor, reported error: {}", err);
        }
    }
}
