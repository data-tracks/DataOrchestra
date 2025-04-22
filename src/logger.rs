use std::io::Write;
use log::LevelFilter;
use ansi_term::Colour;

use std::thread;

/// Initialise custom logger with specified logging `level`
pub fn init_logger(level: LevelFilter) {
    env_logger::builder()
        .filter_level(level)
        .format(|buf, record| {
            let thread_name = thread::current()
                .name()
                .unwrap_or("unknown")
                .to_string();

            let mut colored_level = match record.level() {
                log::Level::Error => Colour::Red.paint(record.level().to_string()).to_string(),
                log::Level::Warn => Colour::Yellow.paint(record.level().to_string()).to_string(),
                log::Level::Info => Colour::Green.paint(record.level().to_string()).to_string(),
                log::Level::Debug => Colour::Blue.paint(record.level().to_string()).to_string(),
                log::Level::Trace => Colour::Purple.paint(record.level().to_string()).to_string(),
            };

            // Color with string makes 4 => 13 and 5 => 14
            if colored_level.len() == 13 {
                colored_level = format!("{colored_level} ");
            }

            let file = record.file().unwrap().split('\\').last().unwrap_or("unknown");

            writeln!(
                buf,
                "[{:<5}] [{}] [{}:{}] {}",
                colored_level,
                thread_name,
                file,
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
