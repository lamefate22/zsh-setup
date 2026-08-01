use tracing_subscriber::{fmt, fmt::time::UtcTime};
use crate::errors::{AppError, Log};

// Initialize stdout logger
pub fn initialize_logger() -> Result<(), AppError> {
    let time_format = time::format_description::parse_borrowed::<1>("[hour]:[minute]:[second]")
        .map_err(Log::InvalidTimeFormat)?;

    fmt().with_timer(UtcTime::new(time_format))
        .with_thread_names(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_level(true)
        .with_ansi(true)
        .with_file(true)
        .compact()
        .init();

    Ok(())
}
