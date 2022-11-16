use std::{path::PathBuf, error::Error, string::ParseError, num::ParseIntError};

use chrono::{DateTime, Utc};
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
pub struct TimeSpec {
	hour: i32,
	minutes: i32,
}

// FIXME: Don't rely on happy path
impl From<String> for TimeSpec {
	fn from(data: String) -> Self {
		let mut dets = data.split(':').map(|t| t.parse::<i32>().unwrap());
		TimeSpec { hour: dets.next().unwrap(), minutes: dets.next().unwrap() }
	}
}

#[derive(Deserialize, Debug)]
pub struct TimeSpan {
	#[serde(with = "timespec")]
	start: TimeSpec,
	#[serde(with = "timespec")]
	stop: TimeSpec,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
	pub name: String,
	pub spec: ThemeSpec,
	pub span: Option<TimeSpan>,
}

mod timespec {
	use serde::{Deserializer, Deserialize};
	use super::TimeSpec;

	pub fn deserialize<'de, D>(deserializer: D) -> Result<TimeSpec, D::Error>
	where
		D: Deserializer<'de>
	{
		let s = String::deserialize(deserializer)?;
		Ok(TimeSpec::from(s))
	}
}
