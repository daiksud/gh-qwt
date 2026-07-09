//! qwt root resolution.
//!
//! See `docs/development/specification/README.md` (Root resolution).

use anyhow::Result;
use std::path::PathBuf;

/// Resolve the qwt root directory.
///
/// Resolution order:
/// 1. `QWT_ROOT` environment variable, if set and non-empty.
/// 2. `git config --get qwt.root`, if it prints a non-empty value.
/// 3. `~/qwt`.
///
/// A leading `~` in the resolved value is expanded to the home directory.
pub fn resolve_root() -> Result<PathBuf> {
    // TODO(config-root): implement per specification.
    anyhow::bail!("config::resolve_root not yet implemented")
}

/// Expand a leading `~` or `~/` in `path` to the user's home directory.
///
/// Paths that do not start with `~` are returned unchanged.
pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    let _ = path;
    // TODO(config-root): implement per specification.
    anyhow::bail!("config::expand_tilde not yet implemented")
}
