//! `gh qwt list` — list repositories and their worktrees.
//!
//! Output is modeled on `ghq list`: a flat, sorted list of `owner/repo/branch`
//! (or absolute paths with `--full-path`), one entry per line, with no
//! repository headers or indentation. This makes it safe to pipe directly
//! into tools like `fzf`, `grep`, or `xargs`.
//!
//! See `docs/development/specification/README.md` (`list`).

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{config, git, repo};

/// Arguments for `gh qwt list`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Only list entries whose `owner/repo/branch` contains this text.
    ///
    /// Matching is case-insensitive unless QUERY contains an uppercase
    /// letter (smartcase). See `--exact` for exact matching.
    #[arg(value_name = "QUERY")]
    pub query: Option<String>,

    /// Require QUERY to exactly match `branch`, `repo/branch`, or
    /// `owner/repo/branch`, instead of a substring match.
    #[arg(short = 'e', long = "exact")]
    pub exact: bool,

    /// Print absolute paths instead of `owner/repo/branch`.
    #[arg(short = 'p', long = "full-path")]
    pub full_path: bool,
}

/// Run the `list` command.
pub fn run(args: Args) -> Result<()> {
    let root = config::resolve_root()?;
    if !root.exists() {
        return Ok(());
    }

    let mut lines = Vec::new();

    for (repo_name, repo_dir) in qwt_repositories(&root) {
        let worktrees = match git::worktree_list(&repo_dir) {
            Ok(worktrees) => worktrees,
            Err(err) => {
                eprintln!("warning: failed to inspect {}: {err:#}", repo_dir.display());
                continue;
            }
        };

        for worktree in worktrees {
            let branch = relative_branch(&repo_dir, &worktree);
            let spec = format!("{repo_name}/{branch}");

            if !matches_query(
                &spec,
                &repo_name,
                &branch,
                args.query.as_deref(),
                args.exact,
            ) {
                continue;
            }

            if args.full_path {
                lines.push(worktree.path.display().to_string());
            } else {
                lines.push(spec);
            }
        }
    }

    lines.sort();
    for line in lines {
        println!("{line}");
    }

    Ok(())
}

/// The worktree's path relative to its repository directory, joined with `/`
/// regardless of platform — this is the `branch` segment of an
/// `owner/repo/branch` spec.
///
/// Uses the on-disk path rather than the branch `git worktree list` reports,
/// so detached-HEAD worktrees still produce a clean, reusable spec instead of
/// no branch at all.
fn relative_branch(repo_dir: &Path, worktree: &git::Worktree) -> String {
    let joined = worktree.path.strip_prefix(repo_dir).ok().map(|rel| {
        rel.components()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/")
    });

    match joined {
        Some(joined) if !joined.is_empty() => joined,
        _ => worktree
            .branch
            .clone()
            .unwrap_or_else(|| file_name_string(&worktree.path)),
    }
}

/// Whether an entry should be included given an optional QUERY and
/// `--exact`.
///
/// With no QUERY, everything matches (an `--exact` with no QUERY is a no-op,
/// matching `ghq list`). With `--exact`, QUERY must equal `branch`,
/// `repo/branch`, or `owner/repo/branch` exactly (case-sensitive). Otherwise,
/// QUERY must be a smartcase substring of the full `owner/repo/branch` spec:
/// case-insensitive unless QUERY contains an uppercase letter.
fn matches_query(
    full_spec: &str,
    repo_name: &str,
    branch: &str,
    query: Option<&str>,
    exact: bool,
) -> bool {
    let Some(query) = query else {
        return true;
    };

    if exact {
        let repo_only = repo_name.rsplit('/').next().unwrap_or(repo_name);
        let repo_branch = format!("{repo_only}/{branch}");
        return query == branch || query == repo_branch || query == full_spec;
    }

    if query.chars().any(char::is_uppercase) {
        full_spec.contains(query)
    } else {
        full_spec.to_lowercase().contains(&query.to_lowercase())
    }
}

fn qwt_repositories(root: &Path) -> Vec<(String, PathBuf)> {
    let mut repositories = Vec::new();

    for owner_dir in sorted_child_dirs(root) {
        let owner = file_name_string(&owner_dir);

        for repo_dir in sorted_child_dirs(&owner_dir) {
            if repo::is_qwt_repo(&repo_dir) {
                let repo = file_name_string(&repo_dir);
                repositories.push((format!("{owner}/{repo}"), repo_dir));
            }
        }
    }

    repositories
}

fn sorted_child_dirs(dir: &Path) -> Vec<PathBuf> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("warning: failed to read {}: {err}", dir.display());
            return Vec::new();
        }
    };

    let mut dirs = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("warning: failed to read entry in {}: {err}", dir.display());
                continue;
            }
        };

        match entry.file_type() {
            Ok(file_type) if file_type.is_dir() => dirs.push(entry.path()),
            Ok(_) => {}
            Err(err) => eprintln!(
                "warning: failed to inspect {}: {err}",
                entry.path().display()
            ),
        }
    }

    dirs.sort_by_key(|path| file_name_string(path));
    dirs
}

fn file_name_string(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}
