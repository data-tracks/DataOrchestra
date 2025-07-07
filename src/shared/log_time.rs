use std::{sync::Mutex, time::Instant};

use once_cell::sync::Lazy;

pub static TIMESTAMPS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

#[macro_export]
macro_rules! log_time {
    ($label:expr) => {
        let current = std::thread::current();
        let name = current.name().unwrap_or("anonymous");
        let entry = format!("{} | {} | {:?}", name, $label, $crate::shared::log_time::START_TIME.elapsed());
        debug!("{entry}");
        let mut timestamps = $crate::shared::log_time::TIMESTAMPS.lock().unwrap();
        timestamps.push(entry);
        std::mem::drop(timestamps);
    };
}
