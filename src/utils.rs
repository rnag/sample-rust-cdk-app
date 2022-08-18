//! Shared utilities
//!
use std::env::var;
use tracing_subscriber::EnvFilter;

pub fn setup_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // Setup from the environment (RUST_LOG)
        .with_env_filter(EnvFilter::from_default_env())
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
}

// Skip serializing a field with `serde` if all fields are a default value
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub fn get_env_var(name: &str) -> String {
    var(name).expect(
        format!(
            "A {} must be set in this app's Lambda environment variables.",
            name
        )
        .as_str(),
    )
}

pub fn env_or_default(name: &str, default: impl Into<String>) -> String {
    match var(name) {
        Ok(value) => value,
        _ => default.into(),
    }
}
