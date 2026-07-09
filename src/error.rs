//! Error helpers for mapping failures to process exit codes.
//!
//! Runtime failures use plain [`anyhow::Error`] and map to exit code `1`.
//! Invalid usage (malformed arguments/specs not caught by clap) is wrapped in
//! [`UsageError`] and maps to exit code `2`, matching the specification.

use std::fmt;

/// Marks an error as invalid usage. `main` maps this to exit code `2`.
#[derive(Debug)]
pub struct UsageError(pub String);

impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for UsageError {}

/// Build an invalid-usage error (exit code `2`) from a message.
pub fn usage(msg: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(UsageError(msg.into()))
}

/// Convert an existing error into an invalid-usage error (exit code `2`),
/// preserving its message. Handy for `RepoSpec::parse(...).map_err(usage_from)`.
pub fn usage_from(err: anyhow::Error) -> anyhow::Error {
    anyhow::Error::new(UsageError(format!("{err:#}")))
}
