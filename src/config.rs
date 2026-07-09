//! qwt root resolution.
//!
//! See `docs/development/specification/README.md` (Root resolution).

use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Resolve the qwt root directory.
///
/// Resolution order:
/// 1. `QWT_ROOT` environment variable, if set and non-empty.
/// 2. `git config --get qwt.root`, if it prints a non-empty value.
/// 3. `~/qwt`.
///
/// A leading `~` in the resolved value is expanded to the home directory.
pub fn resolve_root() -> Result<PathBuf> {
    if let Ok(root) = env::var("QWT_ROOT") {
        if !root.is_empty() {
            return expand_tilde(&root);
        }
    }

    let output = Command::new("git")
        .args(["config", "--get", "qwt.root"])
        .output()
        .context("failed to run `git config --get qwt.root`")?;

    if output.status.success() {
        let root = String::from_utf8_lossy(&output.stdout);
        let root = root.trim();
        if !root.is_empty() {
            return expand_tilde(root);
        }
    }

    expand_tilde("~/qwt")
}

/// Expand a leading `~` or `~/` in `path` to the user's home directory.
///
/// Paths that do not start with `~` are returned unchanged.
pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    if path == "~" {
        return dirs::home_dir().context("could not determine home directory");
    }

    if let Some(rest) = path.strip_prefix("~/") {
        let home = dirs::home_dir().context("could not determine home directory")?;
        return Ok(home.join(rest));
    }

    Ok(PathBuf::from(path))
}

#[cfg(test)]
mod tests {
    use super::expand_tilde;
    use std::path::PathBuf;

    #[test]
    fn expands_bare_tilde_to_home_dir() {
        let Some(home) = dirs::home_dir() else {
            return;
        };

        assert_eq!(expand_tilde("~").unwrap(), home);
    }

    #[test]
    fn expands_tilde_slash_prefix_to_home_dir() {
        let Some(home) = dirs::home_dir() else {
            return;
        };

        assert_eq!(expand_tilde("~/qwt").unwrap(), home.join("qwt"));
    }

    #[test]
    fn leaves_absolute_path_unchanged() {
        assert_eq!(
            expand_tilde("/abs/path").unwrap(),
            PathBuf::from("/abs/path")
        );
    }

    #[test]
    fn leaves_relative_path_unchanged() {
        assert_eq!(
            expand_tilde("relative/path").unwrap(),
            PathBuf::from("relative/path")
        );
    }
}
