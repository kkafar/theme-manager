#![allow(dead_code)]

mod config;
mod theme;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

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

fn main() {
	let cli = Cli::parse();

	let config = Config::try_from(cli.config).unwrap();

	println!("{:?}", config);
}
