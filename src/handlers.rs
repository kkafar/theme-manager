use std::{path::Path, process::Command};

use log::{info, warn};

pub fn handle_edit_cmd(editor: &str, config_path: &Path) {
    info!("Handling edit cmd with editor: {} for config: {}", editor, config_path.clone().to_str().unwrap_or("Failed to parse config path"));
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
