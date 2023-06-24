#![allow(dead_code)]

mod cli;
mod command;
mod config;
mod gsettings;
mod handlers;
mod logging;
mod theme;
mod util;

use clap::Parser;
use handlers::handle_cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = cli::Args::parse();
    let _log_handle = logging::init_logging(&cli_args);

    let config = match config::load_config(&cli_args) {
        Ok(config) => config,
        Err(err) => {
            return Err(Box::new(err));
        }
    };
    handle_cmd(cli_args, config)
}
