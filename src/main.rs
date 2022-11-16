#![allow(dead_code)]

mod config;
mod theme;
mod gsettings;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use log4rs::{append::{console::ConsoleAppender, file::FileAppender}, encode::pattern::PatternEncoder, config::{Logger, Root, Appender}, Handle};

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
	/// Path to config file - see project readme for config file description
	#[arg(long, value_name = "FILE")]
	config: PathBuf,

	/// Run in verbose mode
	#[arg(short, long)]
	verbose: Option<bool>,

	/// Path to log file - if not specified logs are printed to stdout
	#[arg(long, value_name = "FILE")]
	log_file: Option<PathBuf>,

	#[command(subcommand)]
	command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
	// Sets theme by name or basing on current time
	Set {
		/// Name of the theme to apply
		name: Option<String>
	}
}

fn init_logging(cli: &Cli) -> Handle {
	let log_pattern = String::from("[{d(%Y-%m-%d %H:%M:%S)}] [{l}] {m}{n}");

	let mut config_builder = log4rs::Config::builder();

	// FIXME: Sanitize this path
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

fn main() {
	let cli = Cli::parse();

	let _log_handle = init_logging(&cli);

	let config = Config::try_from(cli.config).unwrap();
	println!("{:?}", config);

	match cli.command {
		Commands::Set { name } => {
			if let Some(name) = name {
				let theme_opt = config.theme_for_name(&name);
				if let Some(theme) = theme_opt {
					gsettings::set_theme(theme)
				} else {
					log::error!("Failed to find theme for given name: {}", name);
				}
			}
		}
	}
}
