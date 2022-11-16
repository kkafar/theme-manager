use std::path::PathBuf;
use chrono::{DateTime, Utc, Timelike};
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

// FIXME: Don't rely on happy path
impl From<String> for TimeSpec {
	fn from(data: String) -> Self {
		let mut dets = data.split(':').map(|t| t.parse::<u32>().unwrap());
		TimeSpec { hour: dets.next().unwrap(), minute: dets.next().unwrap() }
	}
}

impl From<DateTime<Utc>> for TimeSpec {
	fn from(date: DateTime<Utc>) -> Self {
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
