use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Sets theme by name or basing on current time
    Set {
        /// Name of the theme to apply
        name: Option<String>,

        /// Whether to disable automatic theme switching.
        /// This option takes priority over --unlock option.
        #[arg(long, default_value_t = false)]
        lock: bool,

        /// Whether to enable automatic theme switching
        #[arg(long, default_value_t = false)]
        unlock: bool,
    },

    /// Retrieves current configuration and prints it to logfile or stdout
    Get,

    /// Opens config file in a editor allowing for modification
    Edit {
        /// Path do editor binary. It will be called in a following way:
        /// editor PATH_TO_CONFIG_FILE
        /// If not specified $EDITOR env var will be used. If that is not defined the operation is a
        /// noop.
        editor: Option<String>,
    },
}
