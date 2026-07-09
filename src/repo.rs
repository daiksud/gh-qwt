//! Repo-spec parsing, path building, and qwt repository discovery.
//!
//! See `docs/development/specification/README.md`
//! (Repo-spec parsing, Path building, Repo discovery, Bare repository requirements).

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Name of the bare-repository directory inside a qwt-managed repository.
pub const BARE_DIR: &str = ".bare";

/// Exact contents of the `.git` pointer file written into a qwt-managed repository.
///
/// The pointer is intentionally relative so the repository tree is relocatable.
pub const GITDIR_POINTER: &str = "gitdir: ./.bare\n";

/// A parsed repository specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoSpec {
    /// Host, e.g. `github.com`.
    pub host: String,
    /// Repository owner, e.g. `cli`.
    pub owner: String,
    /// Repository name (without any trailing `.git`), e.g. `cli`.
    pub repo: String,
    /// URL suitable for `git clone`.
    pub clone_url: String,
}

impl RepoSpec {
    /// Parse `input` in one of these forms:
    /// - `owner/repo` (uses `default_host`)
    /// - HTTPS URL, e.g. `https://github.com/cli/cli(.git)`
    /// - SSH URL, e.g. `git@github.com:cli/cli.git`
    ///
    /// Invalid or malformed specs are rejected with an error.
    pub fn parse(input: &str, default_host: &str) -> Result<RepoSpec> {
        let _ = (input, default_host);
        // TODO(repo-spec): implement per specification.
        anyhow::bail!("repo::RepoSpec::parse not yet implemented")
    }
}

/// Build the repository directory path: `<root>/<owner>/<repo>`.
pub fn repo_dir(root: &Path, owner: &str, repo: &str) -> PathBuf {
    let _ = (root, owner, repo);
    // TODO(repo-spec): implement per specification.
    unimplemented!("repo::repo_dir")
}

/// Build a worktree path: `<root>/<owner>/<repo>/<branch>`.
///
/// A `branch` containing `/` produces nested directories.
pub fn worktree_path(root: &Path, owner: &str, repo: &str, branch: &str) -> PathBuf {
    let _ = (root, owner, repo, branch);
    // TODO(repo-spec): implement per specification.
    unimplemented!("repo::worktree_path")
}

/// Return `true` if `dir` is a qwt-managed repository: it contains `.bare` and a
/// `.git` pointer file whose contents identify `./.bare`.
pub fn is_qwt_repo(dir: &Path) -> bool {
    let _ = dir;
    // TODO(repo-spec): implement per specification.
    unimplemented!("repo::is_qwt_repo")
}

/// Discover the qwt repository root by walking up from `start` to the first
/// ancestor directory that [`is_qwt_repo`]. Fails if none is found.
pub fn discover_repo_root(start: &Path) -> Result<PathBuf> {
    let _ = start;
    // TODO(repo-spec): implement per specification.
    anyhow::bail!("repo::discover_repo_root not yet implemented")
}

/// Ensure adding a worktree for `branch` under `repo_dir` does not collide by
/// path prefix with an existing worktree (e.g. `fix` vs `fix/parser`).
pub fn check_prefix_collision(repo_dir: &Path, branch: &str) -> Result<()> {
    let _ = (repo_dir, branch);
    // TODO(repo-spec): implement per specification.
    anyhow::bail!("repo::check_prefix_collision not yet implemented")
}
