//! `log` module initializes and configures logger for this application.
//!

pub use log::Level;
use log::{log, LevelFilter};

use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

/// Initializes and configures logger.
///
pub fn init_logger() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build(dotenv::var("LOG_FILE").expect("LOG_FILE should be provided"))
        .expect("Logfile should be created correctly");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .expect("Config should be created correctly");

    let _ = log4rs::init_config(config).expect("Logger should be initalized correctly");
}

/// Logs and prints message in stdout.
///  
pub fn log(level: Level, message: &str) {
    log!(level, "{}", message);
    println!("{}", message);
}
