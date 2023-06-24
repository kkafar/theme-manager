use log4rs::{Handle, append::{file::FileAppender, console::ConsoleAppender}, encode::pattern::PatternEncoder, config::{Appender, Root, Logger}};

use crate::Cli;

pub fn init_logging(cli: &Cli) -> Handle {
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

        config_builder =
            config_builder.appender(Appender::builder().build("main", Box::new(stdout_appender)));
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

