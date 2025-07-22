//! Logging configuration for PiCode

use tracing::{Level, metadata::LevelFilter};
use tracing_subscriber::{
    EnvFilter,
    FmtSubscriber,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    fmt::format::{Format, Full},
    fmt::time::UtcTime,
};

/// Configure logging for PiCode
pub fn configure_logger() {
    // Create a custom time format
    let timer = UtcTime::rfc_3339();
    
    // Build the subscriber
    let subscriber = FmtSubscriber::builder()
        .with_timer(timer)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_level(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("picode=info,picode_core=info"))
        )
        .finish();
    
    // Initialize the subscriber
    let _ = tracing::subscriber::set_global_default(subscriber);
}

/// Configure logger with custom level
pub fn configure_logger_with_level(level: Level) {
    let timer = UtcTime::rfc_3339();
    
    let subscriber = FmtSubscriber::builder()
        .with_timer(timer)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_level(true)
        .with_max_level(level)
        .finish();
    
    let _ = tracing::subscriber::set_global_default(subscriber);
}

/// Configure logger for testing (less verbose)
pub fn configure_test_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_level(false)
        .with_test_writer()
        .with_max_level(Level::ERROR)
        .finish();
    
    let _ = tracing::subscriber::set_global_default(subscriber);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, error, info, trace, warn};

    #[test]
    fn logger_configuration() {
        configure_test_logger();
        
        // These should not panic
        trace!("Trace message");
        debug!("Debug message");
        info!("Info message");
        warn!("Warning message");
        error!("Error message");
    }

    #[test]
    fn logger_with_level() {
        configure_logger_with_level(Level::WARN);
        
        // These should not panic
        warn!("Warning message");
        error!("Error message");
    }
}