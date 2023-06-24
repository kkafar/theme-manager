use std::path::PathBuf;

use clap::Parser;

use crate::{command::Commands, util};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
    /// Path to config file - see project readme for config file description
    #[arg(long, value_name = "FILE", value_parser = util::file_exists)]
    pub config: Option<PathBuf>,

    /// Run in verbose mode
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Path to log file - if not specified logs are printed to stdout
    #[arg(long, value_name = "FILE")]
    pub log_file: Option<PathBuf>,

    /// Log level to run the program with. Available: trace, info, warn, error
    #[arg(long, default_value_t = String::from("info"))]
    pub log_level: String,

    #[command(subcommand)]
    pub command: Commands,
}
