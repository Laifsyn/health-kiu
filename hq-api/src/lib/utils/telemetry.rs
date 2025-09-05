use color_eyre::eyre::eyre;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::ChronoLocal;

/// The time format used in log messages or timestamps
pub const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f%:z";

/// The environment variable used to set the log level filter.
pub const FILTER_VAR: &str = "APP_RUST_LOG";
/// The environment variable used to set the log level filter when debug mode.
pub const DBG_FILTER_VAR: &str = "APP_RUST_LOG_DEV";

pub fn logger_init() {
    let env = if cfg!(debug_assertions) { DBG_FILTER_VAR } else { FILTER_VAR };
    let mut errs = Option::None;
    let filter = EnvFilter::try_from_env(env).unwrap_or_else(|e| {
        errs = Some(eyre!(
            "Failed to read `{env}`. Defaulting to `info` filtering. Err:{e}"
        ));
        EnvFilter::new("info")
    });
    if cfg!(test) || cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_ansi(true)
            .with_file(true)
            .with_line_number(true)
            .with_level(true)
            .with_thread_ids(true)
            .with_env_filter(filter)
            .with_target(true)
            .with_timer(ChronoLocal::new(TIME_FORMAT.to_string()))
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_ansi(true)
            .with_file(false)
            .with_line_number(false)
            .with_level(true)
            .with_env_filter(filter)
            .with_target(true)
            .with_timer(ChronoLocal::new(TIME_FORMAT.to_string()))
            .init();
    }
    if let Some(e) = errs {
        tracing::error!("{}", e);
    }
}
