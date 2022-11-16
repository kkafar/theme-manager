use std::process::Command;
use crate::theme::Theme;

fn set_desktop(theme: &str) {
	let _result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.theme")
		.arg("name")
		.arg(theme)
		.status();
}

fn set_mouse(theme: &str) {
	let _result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("cursor-theme")
		.arg(theme)
		.status();
}

fn set_controls(theme: &str) {
	let _result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("gtk-theme")
		.arg(theme)
		.status();
}

fn set_icons(theme: &str) {
	let _result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.interface")
		.arg("icon-theme")
		.arg(theme)
		.status();
}

fn set_borders(theme: &str) {
	let _result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.wm.preferences")
		.arg("theme")
		.arg(theme)
		.status();
}

fn set_wallpaper(path: &str) {
	assert!(path.starts_with("file://"));

	let _result = Command::new("gsettings")
		.arg("set")
		.arg("org.cinnamon.desktop.background")
		.arg("picture-uri")
		.arg(path)
		.status();
}

fn set_kitty(theme: &str) {
	let _result = Command::new("kitty")
		.arg("+kitten")
		.arg("themes")
		.arg("--reload-in=all")
		.arg(theme)
		.status();
}

pub fn set_theme(theme: &Theme) {
	log::info!("Setting theme: {}", theme.name);
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
