use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufReader, ErrorKind},
    path::PathBuf,
};

use log::{error, info};
use serde::Deserialize;

use crate::{
    constant::ConstantRepo,
    theme::{Theme, TimeSpec},
};

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Clone, Debug)]
pub enum ConfigError {
    FileError(String, ErrorKind),
    InvalidFormat(String),
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(description) => {
                write!(f, "Invalid format of config file: {}", description)
            }
            Self::FileError(path, kind) => write!(f, "Failed to read config file: {}; {}", path, kind),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    themes: Vec<Theme>,
    default: Option<String>,
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = File::open(&path).map_err(|err| {
            ConfigError::FileError(path.to_str().unwrap_or("(unknown)").to_owned(), err.kind())
        })?;
        let reader = BufReader::new(file);
        let config: Config =
            serde_json::from_reader(reader).map_err(|err| ConfigError::InvalidFormat(err.to_string()))?;
        Ok(config)
    }

    pub fn theme_for_name(&self, name: &str) -> Option<&Theme> {
        self.themes.iter().find(|&theme| theme.name == name)
    }

    pub fn theme_for_time(&self, date: chrono::DateTime<chrono::Local>) -> Option<&Theme> {
        let timespec = TimeSpec::from(date);

        for theme in &self.themes {
            if let Some(span) = &theme.span {
                if span.contains(&timespec) {
                    return Some(theme);
                }
            }
        }
        None
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = ConfigError;

    fn try_from(path: PathBuf) -> self::Result<Self> {
        Self::from_file(path)
    }
}

pub fn default_path() -> Option<PathBuf> {
    // We look for $HOME/.config/theme-manager/config.json file
    if let Some(user_config_dir) = dirs::config_dir() {
        let app_config_path = user_config_dir.join(ConstantRepo::app_name()).join("config.json");

        if app_config_path.is_file() {
            return Some(app_config_path);
        }
    }
    None
}

pub fn load_config(args: &crate::cli::Args) -> Result<Config> {
    // First we check wheter user specified path to a config
    if let Some(config_path) = args.config.clone() {
        match Config::try_from(config_path) {
            Ok(config) => return Ok(config),
            Err(config_err) => {
                error!("Failed to load config with err: {}", config_err);
            }
        };
    };

    info!("Attempting to load configuration from default location");

    if let Some(config_path) = default_path() {
        info!("Config file found in: {:?}. Loading...", config_path);
        match Config::try_from(config_path) {
            Ok(config) => Ok(config),
            Err(config_err) => {
                error!("Failed to load config with err: {}", config_err);
                Err(config_err)
            }
        }
    } else {
        Err(ConfigError::FileError(
            "(unknown)".to_owned(),
            ErrorKind::NotFound,
        ))
    }
}
