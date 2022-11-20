use std::{path::PathBuf, fs::File, error::Error, io::BufReader};

use serde::Deserialize;

use crate::theme::{Theme, TimeSpec};

#[derive(Debug, Deserialize)]
pub struct Config {
	themes: Vec<Theme>,
	default: Option<String>,
}

impl Config {
	pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn Error>> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let config: Config = serde_json::from_reader(reader)?;
		Ok(config)
	}

	pub fn theme_for_name(&self, name: &str) -> Option<&Theme> {
		for theme in &self.themes {
			if theme.name == name {
				return Some(theme)
			}
		}
		None
	}

	pub fn theme_for_time(&self, date: chrono::DateTime<chrono::Local>) -> Option<&Theme> {
		let timespec = TimeSpec::from(date);

		for theme in &self.themes {
			if let Some(span) = &theme.span {
				if span.contains(&timespec) {
					return Some(theme)
				}
			}
		}
		None
	}
}

impl TryFrom<PathBuf> for Config {
	type Error = Box<dyn Error>;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		Self::from_file(path)
	}
}
