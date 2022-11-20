use std::{path::PathBuf, fmt::Display};
use chrono::{DateTime, Timelike, Local};
use itertools::Itertools;
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct TimeSpec {
	hour: u32,
	minute: u32,
}

impl TryFrom<String> for TimeSpec {
	type Error = ();
	fn try_from(data: String) -> Result<Self, ()> {
		let dets: Option<(Option<u32>, Option<u32>)> = data.split(':').map(|t| t.parse::<u32>().ok()).collect_tuple();

		if let Some(dets) = dets {
			let hour = match dets.0 {
				Some(hour) => hour,
				None => return Err(())
			};

			let minute = match dets.1 {
				Some(minute) => minute,
				None => return Err(())
			};

			return Ok(TimeSpec { hour, minute });
		}
		Err(())
	}
}

impl From<DateTime<Local>> for TimeSpec {
	fn from(date: DateTime<Local>) -> Self {
		TimeSpec { hour: date.hour(), minute: date.minute() }
	}
}

#[derive(Deserialize, Debug)]
pub struct TimeSpan {
	#[serde(with = "timespec")]
	start: TimeSpec,
	#[serde(with = "timespec")]
	stop: TimeSpec,
}

impl TimeSpan {
	pub fn contains(&self, timespec: &TimeSpec) -> bool {
		#![allow(clippy::comparison_chain)]
		if self.start.hour < self.stop.hour {
			(timespec.hour > self.start.hour && timespec.hour < self.stop.hour) ||
				(timespec.hour == self.start.hour && timespec.minute >= self.start.minute) ||
				(timespec.hour == self.stop.hour && timespec.minute < self.stop.minute)
		} else if self.start.hour > self.stop.hour {
			!((timespec.hour > self.stop.hour && timespec.hour < self.start.hour) ||
				(timespec.hour == self.stop.hour && timespec.minute >= self.stop.minute) ||
				(timespec.hour == self.start.hour && timespec.minute < self.start.minute))
		} else if self.start.hour == self.stop.hour {
			timespec.hour == self.start.hour && timespec.minute >= self.start.minute && timespec.minute < self.stop.minute
		} else {
			false
		}
	}
}

#[derive(Debug)]
pub enum ParseError {
	Message(String),
	InvalidDateFormat,
}

impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseError::Message(msg) => f.write_str(msg),
			ParseError::InvalidDateFormat => f.write_str("invalid time span format - it must be of form hh:mm")
		}
	}
}

impl std::error::Error for ParseError {}

impl serde::de::Error for ParseError {
	fn custom<T: Display>(msg: T) -> Self where T:std::fmt::Display {
		ParseError::Message(msg.to_string())
	}
}

#[derive(Deserialize, Debug)]
pub struct Theme {
	pub name: String,
	pub spec: ThemeSpec,
	pub span: Option<TimeSpan>,
}

mod timespec {
	use serde::{Deserializer, Deserialize, de::Error};
	use super::TimeSpec;
	use super::ParseError;

	pub fn deserialize<'de, D>(deserializer: D) -> Result<TimeSpec, D::Error>
	where
		D: Deserializer<'de>
	{
		let s = String::deserialize(deserializer)?;
		if let Ok(timespec) = TimeSpec::try_from(s) {
			Ok(timespec)
		} else {
			// https://stackoverflow.com/questions/66230715/make-my-own-error-for-serde-json-deserialize
			Err(ParseError::InvalidDateFormat).map_err(D::Error::custom)
		}
	}
}
