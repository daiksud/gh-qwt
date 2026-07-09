//! Thin wrappers over the `gh` process (auth-aware GitHub operations).
//!
//! See `docs/development/specification/README.md` (Default-branch detection)
//! and ADR 0009.

use crate::repo::RepoSpec;
use anyhow::Result;

/// Look up the default branch via `gh api repos/{owner}/{repo} -q .default_branch`.
pub fn default_branch(owner: &str, repo: &str) -> Result<String> {
    let _ = (owner, repo);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("gh::default_branch not yet implemented")
}

/// Detect the default branch, preferring `gh` and falling back to git.
///
/// Order (ADR 0009):
/// 1. [`default_branch`] via `gh api`.
/// 2. [`crate::git::ls_remote_default_branch`] via `git ls-remote --symref`.
///
/// Fails only if both methods fail.
pub fn default_branch_with_fallback(spec: &RepoSpec) -> Result<String> {
    let _ = spec;
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("gh::default_branch_with_fallback not yet implemented")
}
