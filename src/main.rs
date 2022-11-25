#![allow(dead_code)]

mod config;
mod theme;
mod gsettings;
mod util;

use std::path::PathBuf;
use chrono::{Local};
use clap::{Parser, Subcommand};
use log::error;
use log4rs::{append::{console::ConsoleAppender, file::FileAppender}, encode::pattern::PatternEncoder, config::{Logger, Root, Appender}, Handle};

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
	/// Path to config file - see project readme for config file description
	#[arg(long, value_name = "FILE", value_parser = util::file_exists)]
	config: PathBuf,

	/// Run in verbose mode
	#[arg(short, long, default_value_t = false)]
	verbose: bool,

	/// Path to log file - if not specified logs are printed to stdout
	#[arg(long, value_name = "FILE")]
	log_file: Option<PathBuf>,

	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// Sets theme by name or basing on current time
	Set {
		/// Name of the theme to apply
		name: Option<String>
	}
}

fn init_logging(cli: &Cli) -> Handle {
	let log_pattern = String::from("[{d(%Y-%m-%d %H:%M:%S)}] [{l}] {m}{n}");

	let mut config_builder = log4rs::Config::builder();

	if let Some(file) = &cli.log_file {
		let file_appender = FileAppender::builder()
			.encoder(Box::new(PatternEncoder::new(&log_pattern)))
			.build(file)
			.unwrap();

			config_builder = config_builder
				.appender(Appender::builder().build("main", Box::new(file_appender)));
	} else {
		let stdout_appender = ConsoleAppender::builder()
			.encoder(Box::new(PatternEncoder::new(&log_pattern)))
			.build();

			config_builder = config_builder
				.appender(Appender::builder().build("main", Box::new(stdout_appender)));
	}

	let config = config_builder
		.logger(Logger::builder()
			.appender("main")
			.additive(false)
			.build("mainlog", log::LevelFilter::Info))
		.build(Root::builder().appender("main").build(log::LevelFilter::Info))
		.unwrap();

	log4rs::init_config(config).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = Cli::parse();

	let _log_handle = init_logging(&cli);

	let config = match Config::try_from(cli.config) {
		Ok(config) => config,
		Err(err) => {
			error!("Failed to load config with err: {}", err);
			return Err(err);
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
		}
	}
	Ok(())
}
