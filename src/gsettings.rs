use std::process::{Command, ExitStatus};
use libc::geteuid;
use log::{info, error, debug};

use crate::theme::Theme;

const DBUS_SESSION_BUS_ADDRESS_KEY: &str = "DBUS_SESSION_BUS_ADDRESS";

/// Handles command result / command failure
fn handle_result(
	result: Result<ExitStatus, std::io::Error>,
	success_msg: String,
	failure_msg: String)
{
	match result {
		Ok(status) => {
			if status.success() {
				info!("{}", success_msg);
			} else if let Some(ret_code) = status.code() {
				error!("{}. Process returned non-zero return code: {}", failure_msg, ret_code);
			} else {
				error!("{}. Process was most likely interrupted", failure_msg);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
}

pub struct Gsettings {
	euid: String
}

impl Gsettings {
	pub fn new() -> Self {
		// Can I check somehow whether this call failed?
		// Or does in not fail?
		let euid = unsafe {
			geteuid()
		};
		debug!("Creating Gsettings insance with euid: {}", euid);
		Gsettings { euid: euid.to_string() }
	}

	fn set_desktop(&self, theme: &str) {
		let result = Command::new("gsettings")
			.arg("set")
			.arg("org.cinnamon.theme")
			.arg("name")
			.arg(theme)
			.env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.euid)
			.status();

		handle_result(
			result,
			format!("Desktop theme set to: {}", theme),
			format!("Failed to set desktop theme to: {}", theme));
	}

	fn set_mouse(&self, theme: &str) {
		let result = Command::new("gsettings")
			.arg("set")
			.arg("org.cinnamon.desktop.interface")
			.arg("cursor-theme")
			.arg(theme)
			.env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.euid)
			.status();

		handle_result(
			result,
			format!("Mouse theme set to: {}", theme),
			format!("Failed to set mouse theme to: {}", theme));
	}

	fn set_controls(&self, theme: &str) {
		let result = Command::new("gsettings")
			.arg("set")
			.arg("org.cinnamon.desktop.interface")
			.arg("gtk-theme")
			.arg(theme)
			.env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.euid)
			.status();

		handle_result(
			result,
			format!("Controls theme set to: {}", theme),
			format!("Failed to set controls theme to: {}", theme));
	}

	fn set_icons(&self, theme: &str) {
		let result = Command::new("gsettings")
			.arg("set")
			.arg("org.cinnamon.desktop.interface")
			.arg("icon-theme")
			.arg(theme)
			.env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.euid)
			.status();

		handle_result(
			result,
			format!("Icons theme set to: {}", theme),
			format!("Failed to set icons theme to: {}", theme));
	}

	fn set_borders(&self, theme: &str) {
		let result = Command::new("gsettings")
			.arg("set")
			.arg("org.cinnamon.desktop.wm.preferences")
			.arg("theme")
			.arg(theme)
			.env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.euid)
			.status();

		handle_result(
			result,
			format!("Borders theme set to: {}", theme),
			format!("Failed to set borders theme to: {}", theme));
	}

	fn set_wallpaper(&self, path: &str) {
		let result = Command::new("gsettings")
			.arg("set")
			.arg("org.cinnamon.desktop.background")
			.arg("picture-uri")
			.arg(path)
			.env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.euid)
			.status();

		handle_result(
			result,
			format!("Wallpaper set to: {}", path),
			format!("Failed to set wallpaper to: {}", path));
	}

	fn set_kitty(&self, theme: &str) {
		let result = Command::new("kitty")
			.arg("+kitten")
			.arg("themes")
			.arg("--reload-in=all")
			.arg(theme)
			.status();

		handle_result(
			result,
			format!("Kitty theme set to: {}", theme),
			format!("Failed to set kitty theme to: {}", theme));
	}

	pub fn set_theme(&self, theme: &Theme) {
		self.set_desktop(&theme.spec.desktop);
		self.set_mouse(&theme.spec.mouse);
		self.set_controls(&theme.spec.controls);
		self.set_icons(&theme.spec.icons);
		self.set_borders(&theme.spec.borders);
		self.set_wallpaper(theme.spec.wallpaper.to_str().unwrap());

		if let Some(kitty_theme) = &theme.spec.kitty {
			self.set_kitty(kitty_theme);
		}
	}
}
