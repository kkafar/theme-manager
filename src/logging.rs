use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Handle,
};

use crate::cli::Args;

fn log_level_from_string(level: &str) -> log::LevelFilter {
    match level {
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    }
}

pub fn init_logging(cli: &Args) -> Handle {
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
        .build(Root::builder().appender("main").build(log_level_from_string(&cli.log_level)))
        .unwrap();

    log4rs::init_config(config).unwrap()
}
