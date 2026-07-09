//! Thin wrappers over the `gh` process (auth-aware GitHub operations).
//!
//! See `docs/development/specification/README.md` (Default-branch detection)
//! and ADR 0009.

use crate::repo::RepoSpec;
use anyhow::{bail, Context, Result};
use std::process::Command;

/// Look up the default branch via `gh api repos/{owner}/{repo} -q .default_branch`.
pub fn default_branch(owner: &str, repo: &str) -> Result<String> {
    let api_path = format!("repos/{owner}/{repo}");
    let output = Command::new("gh")
        .args(["api", &api_path, "-q", ".default_branch"])
        .output()
        .with_context(|| format!("failed to spawn gh api {api_path} -q .default_branch"))?;

    if !output.status.success() {
        bail!(
            "gh api {api_path} -q .default_branch failed: {}",
            String::from_utf8_lossy(&output.stderr).trim_end()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Detect the default branch, preferring `gh` and falling back to git.
///
/// Order (ADR 0009):
/// 1. [`default_branch`] via `gh api`.
/// 2. [`crate::git::ls_remote_default_branch`] via `git ls-remote --symref`.
///
/// Fails only if both methods fail.
pub fn default_branch_with_fallback(spec: &RepoSpec) -> Result<String> {
    match default_branch(&spec.owner, &spec.repo) {
        Ok(branch) => Ok(branch),
        Err(gh_err) => match crate::git::ls_remote_default_branch(&spec.clone_url) {
            Ok(branch) => Ok(branch),
            Err(git_err) => Err(anyhow::anyhow!(
                "failed to detect default branch via gh ({gh_err}); fallback git ls-remote also failed ({git_err})"
            )),
        },
    }
}
