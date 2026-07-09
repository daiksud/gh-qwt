//! Thin wrappers over the `git` process.
//!
//! `run`, `output`, and `success` are the stable primitives; the higher-level
//! helpers build on them.

use anyhow::Result;
use std::path::{Path, PathBuf};

/// A worktree entry parsed from `git worktree list --porcelain`.
#[derive(Debug, Clone)]
pub struct Worktree {
    /// Absolute path of the worktree.
    pub path: PathBuf,
    /// Branch name (without `refs/heads/`), if the worktree is on a branch.
    pub branch: Option<String>,
    /// Commit the worktree HEAD points at, if reported.
    pub head: Option<String>,
    /// Whether the worktree is in detached-HEAD state.
    pub detached: bool,
}

/// Run `git` (optionally in `dir`); return an error including stderr on failure.
pub fn run(dir: Option<&Path>, args: &[&str]) -> Result<()> {
    let _ = (dir, args);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::run not yet implemented")
}

/// Run `git` (optionally in `dir`) and return trimmed stdout; error on non-zero exit.
pub fn output(dir: Option<&Path>, args: &[&str]) -> Result<String> {
    let _ = (dir, args);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::output not yet implemented")
}

/// Run `git` and report whether it exited successfully (no error on non-zero exit).
pub fn success(dir: Option<&Path>, args: &[&str]) -> Result<bool> {
    let _ = (dir, args);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::success not yet implemented")
}

/// `git clone --bare <url> <dest>`.
pub fn clone_bare(url: &str, dest: &Path) -> Result<()> {
    let _ = (url, dest);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::clone_bare not yet implemented")
}

/// Configure the bare repo's fetch refspec and fetch origin.
///
/// Equivalent to:
/// ```text
/// git --git-dir=.bare config remote.origin.fetch '+refs/heads/*:refs/remotes/origin/*'
/// git --git-dir=.bare fetch origin
/// ```
/// REQUIRED because bare clones omit the refspec needed to populate
/// `refs/remotes/origin/*` for tracking worktrees.
pub fn configure_and_fetch(repo_dir: &Path) -> Result<()> {
    let _ = repo_dir;
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::configure_and_fetch not yet implemented")
}

/// Add a worktree tracking `origin/<branch>`:
/// `git -C <repo_dir> worktree add --track -b <branch> <dest> origin/<branch>`.
///
/// The result is a real local branch, not a detached HEAD.
pub fn worktree_add_tracking(repo_dir: &Path, branch: &str, dest: &Path) -> Result<()> {
    let _ = (repo_dir, branch, dest);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::worktree_add_tracking not yet implemented")
}

/// Add a worktree with a new branch based on `base`:
/// `git -C <repo_dir> worktree add -b <branch> <dest> <base>`.
pub fn worktree_add_new(repo_dir: &Path, branch: &str, dest: &Path, base: &str) -> Result<()> {
    let _ = (repo_dir, branch, dest, base);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::worktree_add_new not yet implemented")
}

/// List the worktrees of the repository at `repo_dir`
/// (parse `git -C <repo_dir> worktree list --porcelain`).
pub fn worktree_list(repo_dir: &Path) -> Result<Vec<Worktree>> {
    let _ = repo_dir;
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::worktree_list not yet implemented")
}

/// Remove the worktree at `path` (`--force` when `force`).
pub fn worktree_remove(repo_dir: &Path, path: &Path, force: bool) -> Result<()> {
    let _ = (repo_dir, path, force);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::worktree_remove not yet implemented")
}

/// Delete a local branch: `git -C <repo_dir> branch -D <branch>`.
pub fn branch_delete(repo_dir: &Path, branch: &str) -> Result<()> {
    let _ = (repo_dir, branch);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::branch_delete not yet implemented")
}

/// Whether `origin/<branch>` exists in the repository at `repo_dir`.
pub fn remote_branch_exists(repo_dir: &Path, branch: &str) -> Result<bool> {
    let _ = (repo_dir, branch);
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::remote_branch_exists not yet implemented")
}

/// Detect the default branch via `git ls-remote --symref <url> HEAD`,
/// parsing the `ref: refs/heads/<name>\tHEAD` line.
pub fn ls_remote_default_branch(url: &str) -> Result<String> {
    let _ = url;
    // TODO(git-gh-wrappers): implement.
    anyhow::bail!("git::ls_remote_default_branch not yet implemented")
}
