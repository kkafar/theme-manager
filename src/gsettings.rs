use libc::geteuid;
use log::{debug, error, info};
use std::{process::{Command, ExitStatus, Stdio, Output}, path::PathBuf};

use crate::theme::{Theme, ThemeSpec};

const DBUS_SESSION_BUS_ADDRESS_KEY: &str = "DBUS_SESSION_BUS_ADDRESS";

/// Handles command result / command failure
fn handle_result(result: Result<ExitStatus, std::io::Error>, success_msg: String, failure_msg: String) {
  match result {
    Ok(status) => {
      if status.success() {
        info!("{}", success_msg);
      } else if let Some(ret_code) = status.code() {
        error!(
          "{}. Process returned non-zero return code: {}",
          failure_msg, ret_code
        );
      } else {
        error!("{}. Process was most likely interrupted", failure_msg);
      }
    }
    Err(err) => {
      error!("Failed to execute the process with error: {}", err);
    }
  }
}

fn handle_get_result(result: std::io::Result<Output>) -> Result<String, String> {
	match result {
		Ok(output) => {
			match String::from_utf8(output.stdout) {
				Ok(stdout) => Ok(stdout.replace('\'', "").trim().to_owned()),
				Err(err) => Err(err.to_string()),
			}
		},
		Err(err) => {
			Err(err.to_string())
		}
	}
}

pub struct Gsettings {
  dbus_session_bus_address: String,
}

impl Gsettings {
  pub fn new() -> Self {
    // Can I check somehow whether this call failed?
    // Or does in not fail?
    let euid = unsafe { geteuid() };
    debug!("Creating Gsettings insance with euid: {}", euid);
    Gsettings {
      dbus_session_bus_address: format!("unix:path=/run/user/{}/bus", euid),
    }
  }

  fn set_desktop(&self, theme: &str) {
    let result = Command::new("gsettings")
      .arg("set")
      .arg("org.cinnamon.theme")
      .arg("name")
      .arg(theme)
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .status();

    handle_result(
      result,
      format!("Desktop theme set to: {}", theme),
      format!("Failed to set desktop theme to: {}", theme),
    );
  }

	fn get_desktop(&self) -> Result<String, String> {
    let result = Command::new("gsettings")
      .arg("get")
      .arg("org.cinnamon.theme")
      .arg("name")
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
			.stdout(Stdio::piped())
      .output();

		handle_get_result(result)
	}

  fn set_mouse(&self, theme: &str) {
    let result = Command::new("gsettings")
      .arg("set")
      .arg("org.cinnamon.desktop.interface")
      .arg("cursor-theme")
      .arg(theme)
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .status();

    handle_result(
      result,
      format!("Mouse theme set to: {}", theme),
      format!("Failed to set mouse theme to: {}", theme),
    );
  }

	fn get_mouse(&self) -> Result<String, String> {
    let result = Command::new("gsettings")
      .arg("get")
      .arg("org.cinnamon.desktop.interface")
      .arg("cursor-theme")
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .output();

		handle_get_result(result)
	}

  fn set_controls(&self, theme: &str) {
    let result = Command::new("gsettings")
      .arg("set")
      .arg("org.cinnamon.desktop.interface")
      .arg("gtk-theme")
      .arg(theme)
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .status();

    handle_result(
      result,
      format!("Controls theme set to: {}", theme),
      format!("Failed to set controls theme to: {}", theme),
    );
  }

	fn get_controls(&self) -> Result<String, String> {
    let result = Command::new("gsettings")
      .arg("get")
      .arg("org.cinnamon.desktop.interface")
      .arg("gtk-theme")
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .output();

		handle_get_result(result)
	}

  fn set_icons(&self, theme: &str) {
    let result = Command::new("gsettings")
      .arg("set")
      .arg("org.cinnamon.desktop.interface")
      .arg("icon-theme")
      .arg(theme)
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .status();

    handle_result(
      result,
      format!("Icons theme set to: {}", theme),
      format!("Failed to set icons theme to: {}", theme),
    );
  }

	fn get_icons(&self) -> Result<String, String> {
    let result = Command::new("gsettings")
      .arg("get")
      .arg("org.cinnamon.desktop.interface")
      .arg("icon-theme")
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .output();

		handle_get_result(result)
	}

  fn set_borders(&self, theme: &str) {
    let result = Command::new("gsettings")
      .arg("set")
      .arg("org.cinnamon.desktop.wm.preferences")
      .arg("theme")
      .arg(theme)
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .status();

    handle_result(
      result,
      format!("Borders theme set to: {}", theme),
      format!("Failed to set borders theme to: {}", theme),
    );
  }

	fn get_borders(&self) -> Result<String, String> {
    let result = Command::new("gsettings")
      .arg("get")
      .arg("org.cinnamon.desktop.wm.preferences")
      .arg("theme")
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .output();

		handle_get_result(result)
	}

  fn set_wallpaper(&self, path: &str) {
    let mut sanitized_path: String = path.to_owned();
    if !path.starts_with("file://") {
      sanitized_path = "file://".to_owned() + path;
    }

    let result = Command::new("gsettings")
      .arg("set")
      .arg("org.cinnamon.desktop.background")
      .arg("picture-uri")
      .arg(sanitized_path)
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .status();

    handle_result(
      result,
      format!("Wallpaper set to: {}", path),
      format!("Failed to set wallpaper to: {}", path),
    );
  }

	fn get_wallpaper(&self) -> Result<String, String> {
    let result = Command::new("gsettings")
      .arg("get")
      .arg("org.cinnamon.desktop.background")
      .arg("picture-uri")
      .env(DBUS_SESSION_BUS_ADDRESS_KEY, &self.dbus_session_bus_address)
      .output();

		handle_get_result(result)
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
      format!("Failed to set kitty theme to: {}", theme),
    );
  }

  pub fn set_theme(&self, theme: &Theme) {
		// First we retrieve current theme
		let current_theme_spec = self.get_theme();

		if current_theme_spec.desktop != theme.spec.desktop {
			self.set_desktop(&theme.spec.desktop);
		}

		if current_theme_spec.mouse != theme.spec.mouse {
			self.set_mouse(&theme.spec.mouse);
		}

		if current_theme_spec.controls != theme.spec.controls {
			self.set_controls(&theme.spec.controls);
		}

		if current_theme_spec.icons != theme.spec.icons {
			self.set_icons(&theme.spec.icons);
		}

		if current_theme_spec.borders != theme.spec.borders {
			self.set_borders(&theme.spec.borders);
		}

		if current_theme_spec.wallpaper != theme.spec.wallpaper {
			self.set_wallpaper(theme.spec.wallpaper.to_str().unwrap());
		}

    if let Some(kitty_theme) = &theme.spec.kitty {
      self.set_kitty(kitty_theme);
    }
  }

	pub fn get_theme(&self) -> ThemeSpec {
    let desktop = self.get_desktop().unwrap_or_else(|err| err);
    let mouse = self.get_mouse().unwrap_or_else(|err| err);
    let controls = self.get_controls().unwrap_or_else(|err| err);
    let icons = self.get_icons().unwrap_or_else(|err| err);
    let borders = self.get_borders().unwrap_or_else(|err| err);
    let wallpaper = PathBuf::from(self.get_wallpaper().unwrap_or_else(|err| err));

		ThemeSpec {
			desktop,
			mouse,
			controls,
			icons,
			borders,
			wallpaper,
			kitty: Some("<Unsupported>".to_owned()),
		}
	}
}
