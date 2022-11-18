use std::process::{Command, ExitStatus};
use log::{info, error};

use crate::theme::Theme;

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

fn set_desktop(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.theme")
		.arg("name")
		.arg(theme)
		.status();

	handle_result(
		result,
		format!("Desktop theme set to: {}", theme),
		format!("Failed to set desktop theme to: {}", theme));
}

fn set_mouse(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("cursor-theme")
		.arg(theme)
		.status();

	handle_result(
		result,
		format!("Mouse theme set to: {}", theme),
		format!("Failed to set mouse theme to: {}", theme));
}

fn set_controls(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("gtk-theme")
		.arg(theme)
		.status();

	handle_result(
		result,
		format!("Controls theme set to: {}", theme),
		format!("Failed to set controls theme to: {}", theme));
}

fn set_icons(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("icon-theme")
		.arg(theme)
		.status();

	handle_result(
		result,
		format!("Icons theme set to: {}", theme),
		format!("Failed to set icons theme to: {}", theme));
}

fn set_borders(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.wm.preferences")
		.arg("theme")
		.arg(theme)
		.status();

	handle_result(
		result,
		format!("Borders theme set to: {}", theme),
		format!("Failed to set borders theme to: {}", theme));
}

fn set_wallpaper(path: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.background")
		.arg("picture-uri")
		.arg(path)
		.status();

	handle_result(
		result,
		format!("Wallpaper set to: {}", path),
		format!("Failed to set wallpaper to: {}", path));
}

fn set_kitty(theme: &str) {
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

pub fn set_theme(theme: &Theme) {
	set_desktop(&theme.spec.desktop);
	set_mouse(&theme.spec.mouse);
	set_controls(&theme.spec.controls);
	set_icons(&theme.spec.icons);
	set_borders(&theme.spec.borders);
	set_wallpaper(theme.spec.wallpaper.to_str().unwrap());

	if let Some(kitty_theme) = &theme.spec.kitty {
		set_kitty(kitty_theme);
	}
}
