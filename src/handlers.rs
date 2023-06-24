use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
    process::Command,
};

use chrono::Local;
use log::{error, info, trace, warn};

use crate::{cli::Args, command::Commands, config::Config, context::Context, gsettings::Gsettings};

pub fn handle_cmd(ctx: &mut Context, args: Args, cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let gsettings = Gsettings::new();
    match args.command {
        Commands::Set { name, lock, unlock } => handle_set_cmd(ctx, name, lock, unlock, cfg, &gsettings),
        Commands::Get => handle_get_cmd(ctx, &gsettings),
        Commands::Edit { editor } => handle_edit_cmd(ctx, editor, args.config),
    }
    Ok(())
}

fn handle_set_cmd(
    ctx: &mut Context,
    theme_name: Option<String>,
    lock_theme: bool,
    unlock_theme: bool,
    cfg: Config,
    gset: &Gsettings,
) {
    info!("Running Set command");

    // First we check whether user specified a concrete theme
    // If no concrete theme was specified we look for theme assigned to current time
    // If no such theme is found we log error and exit gracefully
    if let Some(name) = theme_name {
        // If so, we check wheter theme of given name is present in config file
        // In case such theme does not exist we print error and exit gracefully
        if let Some(theme) = cfg.theme_for_name(&name) {
            gset.set_theme(theme);
            maybe_lock_or_unlock(ctx, name.borrow(), lock_theme, unlock_theme)
        } else {
            error!("Failed to find theme for given name: {}", name);
        }
    } else if let Some(theme) = cfg.theme_for_time(Local::now()) {
        if !is_theme_locked(ctx) {
            gset.set_theme(theme);
        } else {
            info!("Theme is locked. Do not performing any changes");
        }
        maybe_lock_or_unlock(ctx, theme.name.borrow(), lock_theme, unlock_theme)
    } else {
        error!("Failed to find theme for current time -- not taking any action");
    }
}

fn handle_get_cmd(_ctx: &mut Context, gset: &Gsettings) {
    info!("Running Get command");
    let theme = gset.get_theme();
    info!("Current theme spec\n{:?}", theme);
}

fn handle_edit_cmd(_ctx: &mut Context, editor: Option<String>, cli_cfg_path: Option<PathBuf>) {
    info!("Running Edit command");

    trace!("Resolving config path");
    let config_path = if let Some(ref path) = cli_cfg_path {
        path.clone()
    } else if let Some(ref path) = crate::config::default_path() {
        path.clone()
    } else {
        warn!("Failed to resolve config path");
        return;
    };

    trace!("Resolving editor name");
    if let Some(ref editor_name) = editor {
        open_editor(editor_name, config_path.borrow());
    } else if let Ok(editor_name) = std::env::var("EDITOR") {
        open_editor(editor_name.borrow(), config_path.borrow());
    } else {
        warn!("Failed to resolve editor name");
    }
}

fn open_editor(editor: &str, config_path: &Path) {
    info!(
        "Handling edit cmd with editor: {} for config: {}",
        editor,
        config_path
            .to_owned()
            .to_str()
            .unwrap_or("Failed to parse config path")
    );
    if editor.is_empty() {
        warn!("Handling interrupted due to empty editor path");
        return;
    }

    match Command::new(editor).arg(config_path).status() {
        Ok(status) => {
            if status.success() {
                info!("Editor closed properly with status code 0");
            } else {
                warn!(
                    "Editor closed with error status code {} (-1 means unknown)",
                    status.code().unwrap_or(-1)
                );
            }
        }
        Err(err) => {
            warn!("Failed to open the editor, reported error: {}", err);
        }
    }
}

fn maybe_lock_or_unlock(ctx: &mut Context, theme: &str, lock: bool, unlock: bool) {
    if lock {
        info!("LOCKING");
        let _ = ctx.data.lock_theme(theme);
    } else if unlock {
        let _ = ctx.data.unlock_theme();
    }
}

fn is_theme_locked(ctx: &mut Context) -> bool {
    ctx.data.theme_lock_file.is_file()
}
