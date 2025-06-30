use log::LevelFilter;
use serde::{de::Error, Deserialize, Deserializer, Serializer};
use tracing_appender::rolling;
use tracing_subscriber::{fmt::{self}, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() {
    let file_appender = rolling::daily("logs", "orchestrator.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    let stdout_log = fmt::layer()
        .with_line_number(true)
        .with_thread_names(true)
        .without_time()
        .with_writer(std::io::stdout);

    let file_log = fmt::layer()
        .with_thread_names(true)
        .with_line_number(true)
        .without_time()
        .with_writer(non_blocking_file);


    tracing_subscriber::registry()
        .with(stdout_log)
        .with(file_log)
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
