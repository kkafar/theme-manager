use std::path::PathBuf;

use log::{error, warn};

use crate::constant::ConstantRepo;

pub struct DataRepo {
    pub app_data_dir: PathBuf,
    theme_lock_file: PathBuf,
}

impl DataRepo {
    pub fn new() -> Self {
        let default_data_dir = DataRepo::default_data_dir().unwrap();
        if !default_data_dir.is_dir() {
            warn!("Data directory at {default_data_dir:?} is missing. Attempting to create.");
            match std::fs::create_dir_all(&default_data_dir) {
                Err(err) => {
                    error!("Failed to create data directory at {default_data_dir:?}. Reported error: {err}");
                }
                _ => {}
            }
        }

        Self {
            app_data_dir: default_data_dir.clone(),
            theme_lock_file: default_data_dir.join("theme.lock"),
        }
    }

    pub fn default_data_dir() -> Option<PathBuf> {
        Some(dirs::data_dir().unwrap().join(ConstantRepo::app_name()))
    }

    pub fn lock_theme(&mut self, _theme: &str) -> std::io::Result<()> {
        if self.theme_lock_file.is_file() {
            return Ok(());
        }

        match std::fs::File::options()
            .read(false)
            .write(false)
            .create(true)
            .open(&self.theme_lock_file)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }

    pub fn unlock_theme(&mut self) {}
}

impl Default for DataRepo {
    fn default() -> Self {
        Self::new()
    }
}
