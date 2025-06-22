use std::io::Write;
use log::LevelFilter;
use ansi_term::Colour;
use serde::{de::Error, Deserialize, Deserializer, Serializer};

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

pub fn deserialize_levelfilter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.to_lowercase().trim() {
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        "off" => Ok(LevelFilter::Off),
        _ => Err(Error::custom("No Value exists"))
    }

}

pub fn serialize_levelfilter<S>(level: &LevelFilter, s: S) -> Result<S::Ok, S::Error> 
where
    S: Serializer,
{
    let level_str = match level {
        LevelFilter::Off => "off",
        LevelFilter::Error => "error",
        LevelFilter::Warn => "warn",
        LevelFilter::Info => "info",
        LevelFilter::Debug => "debug",
        LevelFilter::Trace => "trace",
    };

    s.serialize_str(level_str)
}
