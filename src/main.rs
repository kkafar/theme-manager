#![allow(dead_code)]

mod config;
mod gsettings;
mod theme;
mod util;

use chrono::Local;
use clap::{Parser, Subcommand};
use log::{error, info};
use log4rs::{
  append::{console::ConsoleAppender, file::FileAppender},
  config::{Appender, Logger, Root},
  encode::pattern::PatternEncoder,
  Handle,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
  /// Path to config file - see project readme for config file description
  #[arg(long, value_name = "FILE", value_parser = util::file_exists)]
  config: Option<PathBuf>,

  /// Run in verbose mode
  #[arg(short, long, default_value_t = false)]
  verbose: bool,

  /// Path to log file - if not specified logs are printed to stdout
  #[arg(long, value_name = "FILE")]
  log_file: Option<PathBuf>,

  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Sets theme by name or basing on current time
  Set {
    /// Name of the theme to apply
    name: Option<String>,
  },

	/// Retrieves current configuration and prints it to logfile or stdout
	Get,
}

fn init_logging(cli: &Cli) -> Handle {
  let log_pattern = String::from("[{d(%Y-%m-%d %H:%M:%S)}] [{l}] {m}{n}");

  let mut config_builder = log4rs::Config::builder();

  if let Some(file) = &cli.log_file {
    let file_appender = FileAppender::builder()
      .encoder(Box::new(PatternEncoder::new(&log_pattern)))
      .build(file)
      .unwrap();

    config_builder = config_builder.appender(Appender::builder().build("main", Box::new(file_appender)));
  } else {
    let stdout_appender = ConsoleAppender::builder()
      .encoder(Box::new(PatternEncoder::new(&log_pattern)))
      .build();

    config_builder = config_builder.appender(Appender::builder().build("main", Box::new(stdout_appender)));
  }

  let config = config_builder
    .logger(
      Logger::builder()
        .appender("main")
        .additive(false)
        .build("mainlog", log::LevelFilter::Info),
    )
    .build(Root::builder().appender("main").build(log::LevelFilter::Info))
    .unwrap();

  log4rs::init_config(config).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  let _log_handle = init_logging(&cli);

  let config = match config::load_config(&cli) {
    Ok(config) => config,
    Err(err) => {
      return Err(Box::new(err));
    }
  };

  let gsettings = gsettings::Gsettings::new();

  match cli.command {
    Commands::Set { name } => {
      // First we check whether user specified a concrete theme
      // If no concrete theme was specified we look for theme assigned to current time
      // If no such theme is found we log error and exit gracefully
      if let Some(name) = name {
        // If so, we check wheter theme of given name is present in config file
        // In case such theme does not exist we print error and exit gracefully
        if let Some(theme) = config.theme_for_name(&name) {
          gsettings.set_theme(theme)
        } else {
          error!("Failed to find theme for given name: {}", name);
        }
      } else if let Some(theme) = config.theme_for_time(Local::now()) {
        gsettings.set_theme(theme);
      } else {
        error!("Failed to find theme for current time -- not taking any action");
      }
    },
		Commands::Get => {
			let theme = gsettings.get_theme();
			info!("Current theme spec\n{:?}", theme);
		}
  }
  Ok(())
}
