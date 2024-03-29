#![allow(dead_code)]

mod cli;
mod command;
mod config;
mod constant;
mod context;
mod gsettings;
mod handlers;
mod logging;
mod theme;
mod util;

use clap::Parser;
use context::{data::DataRepo, Context};
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

    let mut ctx = Context::new(DataRepo::default());
    handle_cmd(&mut ctx, cli_args, config)
}
