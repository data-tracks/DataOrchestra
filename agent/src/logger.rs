use tracing::level_filters::LevelFilter;
use tracing_appender::rolling;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logger(level: LevelFilter) {
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
        .with(level)
        .with(stdout_log)
        .with(file_log)
        .init();
}
