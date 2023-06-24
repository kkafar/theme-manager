use std::path::PathBuf;

use log::{debug, error, info, trace, warn};

use crate::constant::ConstantRepo;

pub struct DataRepo {
    pub app_data_dir: PathBuf,
    pub theme_lock_file: PathBuf,
}

impl DataRepo {
    pub fn new() -> Self {
        let app_data_dir = DataRepo::default_data_dir().unwrap();
        if !app_data_dir.is_dir() {
            warn!("Data directory at {app_data_dir:?} is missing. Attempting to create.");
            if let Err(err) = std::fs::create_dir_all(&app_data_dir) {
                error!("Failed to create data directory at {app_data_dir:?}. Reported error: {err}");
            }
        }

        let theme_lock_file = app_data_dir.join("theme.lock");
        debug!("DataRepo data_dir: {app_data_dir:?}, theme_lock_file: {theme_lock_file:?}");

        Self {
            app_data_dir,
            theme_lock_file,
        }
    }

    pub fn default_data_dir() -> Option<PathBuf> {
        Some(dirs::data_dir().unwrap().join(ConstantRepo::app_name()))
    }

    pub fn lock_theme(&self, _theme: &str) -> std::io::Result<()> {
        trace!("Creating theme lock");
        if self.theme_lock_file.is_file() {
            return Ok(());
        }

        match std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.theme_lock_file)
        {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Failed to create theme lock file. Error: {}", err);
                Err(err)
            }
        }
    }

    pub fn unlock_theme(&self) -> std::io::Result<()> {
        trace!("Removing theme lock");

        if self.theme_lock_file.is_file() {
            return match std::fs::remove_file(&self.theme_lock_file) {
                Ok(_) => {
                    info!("Theme lock removed");
                    Ok(())
                }
                Err(err) => {
                    error!("Failed to remove theme lock with error {}", err);
                    Err(err)
                }
            };
        }
        Ok(())
    }
}

impl Default for DataRepo {
    fn default() -> Self {
        Self::new()
    }
}
