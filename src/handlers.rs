use std::{process::Command, path::Path};

pub fn handle_edit_cmd(editor: &str, config_path: &Path) {
    if editor.is_empty() {
        return;
    }

    Command::new(editor)
      .arg(config_path)
      .status();
}
