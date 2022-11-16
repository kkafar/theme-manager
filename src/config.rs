use std::{path::PathBuf, fs::File, error::Error, io::BufReader};

use serde::Deserialize;

use crate::theme::Theme;

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
}

impl TryFrom<PathBuf> for Config {
	type Error = Box<dyn Error>;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		Self::from_file(path)
	}
}
