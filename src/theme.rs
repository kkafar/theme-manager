use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThemeSpec {
	pub desktop: String,
	pub mouse: String,
	pub controls: String,
	pub icons: String,
	pub borders: String,
	pub wallpaper: PathBuf,
	pub kitty: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
	name: String,
	spec: ThemeSpec,
}
