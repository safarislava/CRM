use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{Builder, Rotation};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub trait AppLogs {
    fn attach(&self) -> WorkerGuard;
}

pub struct RollingLogs {
    dir: String,
    retention_days: usize,
    prefix: String,
    filter: String,
}

impl RollingLogs {
    pub fn new(dir: String, retention_days: usize, prefix: String, filter: String) -> Self {
        Self {
            dir,
            retention_days,
            prefix,
            filter,
        }
    }

    pub fn default_crm() -> Self {
        Self::new(
            "./logs".to_string(),
            7,
            "dailycrm".to_string(),
            "info,actix_web=info,sqlx=warn".to_string(),
        )
    }
}

impl AppLogs for RollingLogs {
    fn attach(&self) -> WorkerGuard {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&self.filter));

        let console_layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_filter(env_filter.clone());

        let file_appender = Builder::new()
            .rotation(Rotation::DAILY)
            .max_log_files(self.retention_days)
            .filename_prefix(&self.prefix)
            .filename_suffix("log")
            .build(&self.dir)
            .expect("Failed to initialize rolling file appender");

        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

        let file_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(non_blocking_appender)
            .with_filter(env_filter);

        tracing_subscriber::registry()
            .with(console_layer)
            .with(file_layer)
            .init();

        guard
    }
}
