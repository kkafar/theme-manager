use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ColorSchemePreference {
    Default,
    Light,
    Dark
}

impl From<&ColorSchemePreference> for &str {
    fn from(value: &ColorSchemePreference) -> Self {
        // These values match options available for "gsettings describe org.gnome.desktop.interface color-scheme" key.
        match value {
            ColorSchemePreference::Dark => "prefer-dark",
            ColorSchemePreference::Default => "default",
            ColorSchemePreference::Light => "prefer-light",
        }
    }
}
