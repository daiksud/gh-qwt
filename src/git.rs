//! Thin wrappers over the `git` process.
//!
//! `run`, `output`, and `success` are the stable primitives; the higher-level
//! helpers build on them.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A worktree entry parsed from `git worktree list --porcelain`.
#[derive(Debug, Clone)]
pub struct Worktree {
    /// Absolute path of the worktree.
    pub path: PathBuf,
    /// Branch name (without `refs/heads/`), if the worktree is on a branch.
    pub branch: Option<String>,
    /// Commit the worktree HEAD points at, if reported. Part of the parsed
    /// porcelain model; not currently surfaced by any command.
    #[allow(dead_code)]
    pub head: Option<String>,
    /// Whether the worktree is in detached-HEAD state.
    pub detached: bool,
}

/// Run `git` (optionally in `dir`); return an error including stderr on failure.
pub fn run(dir: Option<&Path>, args: &[&str]) -> Result<()> {
    let mut command = Command::new("git");
    if let Some(dir) = dir {
        command.current_dir(dir);
    }

    let output = command
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn git {}", args.join(" ")))?;

    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim_end()
        );
    }

    Ok(())
}

/// Run `git` (optionally in `dir`) and return trimmed stdout; error on non-zero exit.
pub fn output(dir: Option<&Path>, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    if let Some(dir) = dir {
        command.current_dir(dir);
    }

    let output = command
        .args(args)
        .output()
        .with_context(|| format!("failed to spawn git {}", args.join(" ")))?;

    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim_end()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end_matches(['\r', '\n'])
        .to_string())
}

/// Run `git` and report whether it exited successfully (no error on non-zero exit).
pub fn success(dir: Option<&Path>, args: &[&str]) -> Result<bool> {
    let mut command = Command::new("git");
    if let Some(dir) = dir {
        command.current_dir(dir);
    }

    let status = command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| format!("failed to spawn git {}", args.join(" ")))?;

    Ok(status.success())
}

/// `git clone --bare <url> <dest>`.
pub fn clone_bare(url: &str, dest: &Path) -> Result<()> {
    let dest = dest.to_string_lossy();
    run(None, &["clone", "--bare", url, dest.as_ref()])
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
    run(
        Some(repo_dir),
        &[
            "--git-dir=.bare",
            "config",
            "remote.origin.fetch",
            "+refs/heads/*:refs/remotes/origin/*",
        ],
    )?;
    run(Some(repo_dir), &["--git-dir=.bare", "fetch", "origin"])
}

/// Add a worktree tracking `origin/<branch>`:
/// `git -C <repo_dir> worktree add --track -b <branch> <dest> origin/<branch>`.
///
/// The result is a real local branch, not a detached HEAD.
pub fn worktree_add_tracking(repo_dir: &Path, branch: &str, dest: &Path) -> Result<()> {
    let dest = dest.to_string_lossy();
    let origin_branch = format!("origin/{branch}");
    run(
        Some(repo_dir),
        &[
            "worktree",
            "add",
            "--track",
            "-b",
            branch,
            dest.as_ref(),
            &origin_branch,
        ],
    )
}

/// Add a worktree with a new branch based on `base`:
/// `git -C <repo_dir> worktree add -b <branch> <dest> <base>`.
pub fn worktree_add_new(repo_dir: &Path, branch: &str, dest: &Path, base: &str) -> Result<()> {
    let dest = dest.to_string_lossy();
    run(
        Some(repo_dir),
        &["worktree", "add", "-b", branch, dest.as_ref(), base],
    )
}

/// Attach an existing local branch to a new worktree:
/// `git -C <repo_dir> worktree add <dest> <branch>`.
///
/// A `git clone --bare` copies the remote's branches into local heads, so the
/// default branch (and any branch present at clone time) already exists locally
/// and must be *attached* rather than re-created. Best-effort sets the branch's
/// upstream to `origin/<branch>` so it tracks the remote when that ref exists.
pub fn worktree_add_existing(repo_dir: &Path, branch: &str, dest: &Path) -> Result<()> {
    let dest = dest.to_string_lossy();
    run(Some(repo_dir), &["worktree", "add", dest.as_ref(), branch])?;
    let _ = run(
        Some(repo_dir),
        &[
            "branch",
            &format!("--set-upstream-to=origin/{branch}"),
            branch,
        ],
    );
    Ok(())
}

/// Whether a local branch `refs/heads/<branch>` exists in the repo at `repo_dir`.
pub fn local_branch_exists(repo_dir: &Path, branch: &str) -> Result<bool> {
    let local_ref = format!("refs/heads/{branch}");
    success(
        Some(repo_dir),
        &["show-ref", "--verify", "--quiet", &local_ref],
    )
}

/// The repository's default branch, read from the bare repo's HEAD via
/// `git symbolic-ref --short HEAD`.
///
/// After `git clone --bare`, the bare repo's HEAD points at the remote's
/// default branch, so this is the natural base for new branches.
pub fn default_branch_from_head(repo_dir: &Path) -> Result<String> {
    output(Some(repo_dir), &["symbolic-ref", "--short", "HEAD"])
}

/// List the worktrees of the repository at `repo_dir`
/// (parse `git -C <repo_dir> worktree list --porcelain`).
pub fn worktree_list(repo_dir: &Path) -> Result<Vec<Worktree>> {
    let porcelain = output(Some(repo_dir), &["worktree", "list", "--porcelain"])?;
    Ok(parse_worktree_list(&porcelain))
}

/// Remove the worktree at `path` (`--force` when `force`).
pub fn worktree_remove(repo_dir: &Path, path: &Path, force: bool) -> Result<()> {
    let path = path.to_string_lossy();
    if force {
        run(
            Some(repo_dir),
            &["worktree", "remove", "--force", path.as_ref()],
        )
    } else {
        run(Some(repo_dir), &["worktree", "remove", path.as_ref()])
    }
}

/// Delete a local branch: `git -C <repo_dir> branch -D <branch>`.
pub fn branch_delete(repo_dir: &Path, branch: &str) -> Result<()> {
    run(Some(repo_dir), &["branch", "-D", branch])
}

/// Whether `origin/<branch>` exists in the repository at `repo_dir`.
pub fn remote_branch_exists(repo_dir: &Path, branch: &str) -> Result<bool> {
    let remote_ref = format!("refs/remotes/origin/{branch}");
    success(
        Some(repo_dir),
        &["show-ref", "--verify", "--quiet", &remote_ref],
    )
}

/// Detect the default branch via `git ls-remote --symref <url> HEAD`,
/// parsing the `ref: refs/heads/<name>\tHEAD` line.
pub fn ls_remote_default_branch(url: &str) -> Result<String> {
    let refs = output(None, &["ls-remote", "--symref", url, "HEAD"])?;
    refs.lines()
        .find_map(|line| {
            line.strip_prefix("ref: refs/heads/")
                .and_then(|rest| rest.strip_suffix("\tHEAD"))
                .map(ToOwned::to_owned)
        })
        .with_context(|| format!("could not parse default branch from git ls-remote for {url}"))
}

fn parse_worktree_list(porcelain: &str) -> Vec<Worktree> {
    let mut worktrees = Vec::new();

    for record in porcelain.split("\n\n") {
        let mut path = None;
        let mut branch = None;
        let mut head = None;
        let mut detached = false;
        let mut bare = false;

        for line in record.lines() {
            if let Some(value) = line.strip_prefix("worktree ") {
                path = Some(PathBuf::from(value));
            } else if let Some(value) = line.strip_prefix("HEAD ") {
                head = Some(value.to_string());
            } else if let Some(value) = line.strip_prefix("branch refs/heads/") {
                branch = Some(value.to_string());
            } else if line == "detached" {
                detached = true;
            } else if line == "bare" {
                bare = true;
            }
        }

        if bare {
            continue;
        }

        if let Some(path) = path {
            if detached {
                branch = None;
            }

            worktrees.push(Worktree {
                path,
                branch,
                head,
                detached,
            });
        }
    }

    worktrees
}

#[cfg(test)]
mod tests {
    use super::parse_worktree_list;
    use std::path::PathBuf;

    #[test]
    fn parses_single_non_detached_worktree() {
        let worktrees =
            parse_worktree_list("worktree /repo/main\nHEAD abc123\nbranch refs/heads/main\n\n");

        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/repo/main"));
        assert_eq!(worktrees[0].head.as_deref(), Some("abc123"));
        assert_eq!(worktrees[0].branch.as_deref(), Some("main"));
        assert!(!worktrees[0].detached);
    }

    #[test]
    fn parses_multiple_worktree_records() {
        let worktrees = parse_worktree_list(
            "worktree /repo/main\nHEAD abc123\nbranch refs/heads/main\n\n\
             worktree /repo/fix/parser\nHEAD def456\nbranch refs/heads/fix/parser\n\n",
        );

        assert_eq!(worktrees.len(), 2);
        assert_eq!(worktrees[0].branch.as_deref(), Some("main"));
        assert_eq!(worktrees[1].path, PathBuf::from("/repo/fix/parser"));
        assert_eq!(worktrees[1].head.as_deref(), Some("def456"));
        assert_eq!(worktrees[1].branch.as_deref(), Some("fix/parser"));
        assert!(!worktrees[1].detached);
    }

    #[test]
    fn parses_detached_worktree_record() {
        let worktrees = parse_worktree_list("worktree /repo/detached\nHEAD abc123\ndetached\n\n");

        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/repo/detached"));
        assert_eq!(worktrees[0].head.as_deref(), Some("abc123"));
        assert_eq!(worktrees[0].branch, None);
        assert!(worktrees[0].detached);
    }

    #[test]
    fn skips_bare_entries() {
        let worktrees = parse_worktree_list(
            "worktree /repo\nHEAD abc123\nbare\n\n\
             worktree /repo/main\nHEAD def456\nbranch refs/heads/main\n\n",
        );

        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/repo/main"));
        assert_eq!(worktrees[0].branch.as_deref(), Some("main"));
    }
}
