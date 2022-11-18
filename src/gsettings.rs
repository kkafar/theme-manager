use std::process::Command;
use log::{info, error};

use crate::theme::Theme;

fn set_desktop(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.theme")
		.arg("name")
		.arg(theme)
		.status();

	match result {
		Ok(status) => {
			if status.success() {
				info!("Desktop theme set to: {}", theme);
			} else if let Some(ret_code) = status.code() {
				error!("Failed to set desktop theme to: {}. Process returned non-zero return code: {}", theme, ret_code);
			} else {
				error!("Failed to set desktop theme to: {}. Process was most likely interrupted", theme);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
}

fn set_mouse(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("cursor-theme")
		.arg(theme)
		.status();

	match result {
		Ok(status) => {
			if status.success() {
				info!("Mouse theme set to: {}", theme);
			} else if let Some(ret_code) = status.code() {
				error!("Failed to set mouse theme to: {}. Process returned non-zero return code: {}", theme, ret_code);
			} else {
				error!("Failed to set mouse theme to: {}. Process was most likely interrupted", theme);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
}

fn set_controls(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("gtk-theme")
		.arg(theme)
		.status();

	match result {
		Ok(status) => {
			if status.success() {
				info!("Controls theme set to: {}", theme);
			} else if let Some(ret_code) = status.code() {
				error!("Failed to set controls theme to: {}. Process returned non-zero return code: {}", theme, ret_code);
			} else {
				error!("Failed to set controls theme to: {}. Process was most likely interrupted", theme);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
}

fn set_icons(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("icon-theme")
		.arg(theme)
		.status();

	match result {
		Ok(status) => {
			if status.success() {
				info!("Icons theme set to: {}", theme);
			} else if let Some(ret_code) = status.code() {
				error!("Failed to set icons theme to: {}. Process returned non-zero return code: {}", theme, ret_code);
			} else {
				error!("Failed to set icons theme to: {}. Process was most likely interrupted", theme);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
}

fn set_borders(theme: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.wm.preferences")
		.arg("theme")
		.arg(theme)
		.status();

	match result {
		Ok(status) => {
			if status.success() {
				info!("Borders theme set to: {}", theme);
			} else if let Some(ret_code) = status.code() {
				error!("Failed to set borders theme to: {}. Process returned non-zero return code: {}", theme, ret_code);
			} else {
				error!("Failed to set borders theme to: {}. Process was most likely interrupted", theme);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
}

fn set_wallpaper(path: &str) {
	let result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.background")
		.arg("picture-uri")
		.arg(path)
		.status();

	match result {
		Ok(status) => {
			if status.success() {
				info!("Wallpaper set to: {}", path);
			} else if let Some(ret_code) = status.code() {
				error!("Failed to set wallpaper to: {}. Process returned non-zero return code: {}", path, ret_code);
			} else {
				error!("Failed to set wallpaper to: {}. Process was most likely interrupted", path);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
}

fn set_kitty(theme: &str) {
	let result = Command::new("kitty")
		.arg("+kitten")
		.arg("themes")
		.arg("--reload-in=all")
		.arg(theme)
		.status();

	match result {
		Ok(status) => {
			if status.success() {
				info!("Kitty theme set to: {}", theme);
			} else if let Some(ret_code) = status.code() {
				error!("Failed to set kitty theme to: {}. Process returned non-zero return code: {}", theme, ret_code);
			} else {
				error!("Failed to set kitty theme to: {}. Process was most likely interrupted", theme);
			}
		}
		Err(err) => {
			error!("Failed to execute the process with error: {}", err);
		}
	}
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
