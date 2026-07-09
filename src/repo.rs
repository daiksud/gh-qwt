//! Repo-spec parsing, path building, and qwt repository discovery.
//!
//! See `docs/development/specification/README.md`
//! (Repo-spec parsing, Path building, Repo discovery, Bare repository requirements).

use anyhow::{bail, Result};
use std::fs;
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
        let input = input.trim();
        if input.is_empty() {
            bail!("repository spec must not be empty");
        }

        if let Some(rest) = input.strip_prefix("https://") {
            let (host, path) = rest.split_once('/').ok_or_else(|| {
                anyhow::anyhow!("HTTPS repository spec must include owner/repo path")
            })?;
            let (owner, repo) = parse_owner_repo(path)?;
            let repo = strip_git_suffix(repo);
            validate_parts(host, owner, repo)?;

            return Ok(RepoSpec {
                host: host.to_string(),
                owner: owner.to_string(),
                repo: repo.to_string(),
                clone_url: format!("https://{host}/{owner}/{repo}.git"),
            });
        }

        if let Some(rest) = input.strip_prefix("file://") {
            // Local repository. There is no host or owner in a filesystem path,
            // so derive `owner` from the parent directory and `repo` from the
            // final path component. Primarily used for local clones and offline
            // integration tests.
            let segments: Vec<&str> = rest.split('/').filter(|s| !s.is_empty()).collect();
            let repo = segments
                .last()
                .map(|s| strip_git_suffix(s))
                .filter(|repo| !repo.is_empty())
                .ok_or_else(|| anyhow::anyhow!("file:// repository spec is missing a repo name"))?;
            let owner = if segments.len() >= 2 {
                segments[segments.len() - 2]
            } else {
                "local"
            };

            return Ok(RepoSpec {
                host: "local".to_string(),
                owner: owner.to_string(),
                repo: repo.to_string(),
                clone_url: input.to_string(),
            });
        }

        if input.contains("://") {
            bail!("unsupported repository URL scheme");
        }

        if let Some(rest) = input.strip_prefix("git@") {
            let (host, path) = rest
                .split_once(':')
                .ok_or_else(|| anyhow::anyhow!("malformed SSH repository spec"))?;
            let (owner, repo) = parse_owner_repo(path)?;
            let repo = strip_git_suffix(repo);
            validate_parts(host, owner, repo)?;

            return Ok(RepoSpec {
                host: host.to_string(),
                owner: owner.to_string(),
                repo: repo.to_string(),
                clone_url: format!("git@{host}:{owner}/{repo}.git"),
            });
        }

        let (owner, repo) = parse_owner_repo(input)?;
        validate_parts(default_host, owner, repo)?;
        Ok(RepoSpec {
            host: default_host.to_string(),
            owner: owner.to_string(),
            repo: repo.to_string(),
            clone_url: format!("https://{default_host}/{owner}/{repo}.git"),
        })
    }
}

/// Build the repository directory path: `<root>/<owner>/<repo>`.
pub fn repo_dir(root: &Path, owner: &str, repo: &str) -> PathBuf {
    root.join(owner).join(repo)
}

/// Build a worktree path: `<root>/<owner>/<repo>/<branch>`.
///
/// A `branch` containing `/` produces nested directories.
pub fn worktree_path(root: &Path, owner: &str, repo: &str, branch: &str) -> PathBuf {
    worktree_in(&repo_dir(root, owner, repo), branch)
}

/// Build a worktree path under an existing repository directory.
///
/// A `branch` containing `/` produces nested directories. Useful when the
/// repository directory is already known (e.g. from [`discover_repo_root`]).
pub fn worktree_in(repo_dir: &Path, branch: &str) -> PathBuf {
    branch
        .split('/')
        .filter(|segment| !segment.is_empty())
        .fold(repo_dir.to_path_buf(), |path, segment| path.join(segment))
}

/// Return `true` if `dir` is a qwt-managed repository: it contains `.bare` and a
/// `.git` pointer file whose contents identify `./.bare`.
pub fn is_qwt_repo(dir: &Path) -> bool {
    if !dir.join(BARE_DIR).is_dir() {
        return false;
    }

    let git_file = dir.join(".git");
    if !git_file.is_file() {
        return false;
    }

    match fs::read_to_string(git_file) {
        Ok(contents) => matches!(contents.trim(), "gitdir: ./.bare" | "gitdir: .bare"),
        Err(_) => false,
    }
}

/// Discover the qwt repository root by walking up from `start` to the first
/// ancestor directory that [`is_qwt_repo`]. Fails if none is found.
pub fn discover_repo_root(start: &Path) -> Result<PathBuf> {
    let mut current = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());

    // `std::fs::canonicalize` on Windows returns an extended-length path
    // (\\?\C:\...). Strip that prefix so downstream paths and git invocations
    // use the conventional form. This has no effect on non-Windows paths.
    if let Some(stripped) = current.to_str().and_then(|s| s.strip_prefix(r"\\?\")) {
        current = PathBuf::from(stripped);
    }

    loop {
        if is_qwt_repo(&current) {
            return Ok(current);
        }

        if !current.pop() {
            bail!("no qwt repository found from {}", start.display());
        }
    }
}

/// Ensure adding a worktree for `branch` under `repo_dir` does not collide by
/// path prefix with an existing worktree (e.g. `fix` vs `fix/parser`).
pub fn check_prefix_collision(repo_dir: &Path, branch: &str) -> Result<()> {
    let segments: Vec<&str> = branch
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.is_empty() {
        bail!("branch name must not be empty");
    }

    let mut current = repo_dir.to_path_buf();
    for (index, segment) in segments.iter().enumerate() {
        current.push(segment);
        let is_target = index + 1 == segments.len();

        let exists = current
            .try_exists()
            .map_err(|err| anyhow::anyhow!("cannot inspect {}: {err}", current.display()))?;
        if !exists {
            continue;
        }

        let metadata = fs::metadata(&current)
            .map_err(|err| anyhow::anyhow!("cannot inspect {}: {err}", current.display()))?;

        if !metadata.is_dir() {
            bail!(
                "worktree path {} collides with existing non-directory",
                current.display()
            );
        }

        if has_git_marker(&current) {
            bail!(
                "worktree path {} collides with existing worktree",
                current.display()
            );
        }

        if is_target && contains_branch_descendant(&current)? {
            bail!(
                "worktree path {} is a prefix of an existing worktree",
                current.display()
            );
        }
    }

    Ok(())
}

fn parse_owner_repo(path: &str) -> Result<(&str, &str)> {
    let mut parts = path.split('/');
    let owner = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("repository spec is missing owner"))?;
    let repo = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("repository spec is missing repo"))?;
    if parts.next().is_some() {
        bail!("repository spec must contain exactly owner/repo");
    }
    Ok((owner, repo))
}

fn strip_git_suffix(repo: &str) -> &str {
    repo.strip_suffix(".git").unwrap_or(repo)
}

fn validate_parts(host: &str, owner: &str, repo: &str) -> Result<()> {
    if host.is_empty() {
        bail!("repository spec is missing host");
    }
    if owner.is_empty() {
        bail!("repository spec is missing owner");
    }
    if repo.is_empty() {
        bail!("repository spec is missing repo");
    }
    Ok(())
}

fn has_git_marker(dir: &Path) -> bool {
    let git = dir.join(".git");
    git.is_file() || git.is_dir()
}

fn contains_branch_descendant(dir: &Path) -> Result<bool> {
    for entry in
        fs::read_dir(dir).map_err(|err| anyhow::anyhow!("cannot read {}: {err}", dir.display()))?
    {
        let entry = entry.map_err(|err| anyhow::anyhow!("cannot read directory entry: {err}"))?;
        let file_name = entry.file_name();
        if file_name == ".git" || file_name == BARE_DIR {
            continue;
        }

        return Ok(true);
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn assert_spec(
        input: &str,
        default_host: &str,
        host: &str,
        owner: &str,
        repo: &str,
        clone_url: &str,
    ) {
        let spec = RepoSpec::parse(input, default_host).expect("spec should parse");
        assert_eq!(spec.host, host);
        assert_eq!(spec.owner, owner);
        assert_eq!(spec.repo, repo);
        assert_eq!(spec.clone_url, clone_url);
    }

    #[test]
    fn parse_examples() {
        assert_spec(
            "cli/cli",
            "github.com",
            "github.com",
            "cli",
            "cli",
            "https://github.com/cli/cli.git",
        );
        assert_spec(
            "cli/cli",
            "github.example.com",
            "github.example.com",
            "cli",
            "cli",
            "https://github.example.com/cli/cli.git",
        );
        assert_spec(
            "https://github.com/cli/cli.git",
            "github.example.com",
            "github.com",
            "cli",
            "cli",
            "https://github.com/cli/cli.git",
        );
        assert_spec(
            "https://github.com/cli/cli",
            "github.example.com",
            "github.com",
            "cli",
            "cli",
            "https://github.com/cli/cli.git",
        );
        assert_spec(
            "git@github.com:cli/cli.git",
            "github.example.com",
            "github.com",
            "cli",
            "cli",
            "git@github.com:cli/cli.git",
        );
        // Local file URLs: owner from parent dir, repo from final component.
        assert_spec(
            "file:///tmp/acme/src",
            "github.com",
            "local",
            "acme",
            "src",
            "file:///tmp/acme/src",
        );
        assert_spec(
            "file:///tmp/acme/src.git",
            "github.com",
            "local",
            "acme",
            "src",
            "file:///tmp/acme/src.git",
        );
    }

    #[test]
    fn parse_rejects_invalid_specs() {
        for input in ["", "a/b/c", "ftp://x/y", "foo", "/foo", "foo/"] {
            assert!(
                RepoSpec::parse(input, "github.com").is_err(),
                "{input:?} should be rejected"
            );
        }
    }

    #[test]
    fn builds_repo_and_worktree_paths_without_host() {
        let root = Path::new("/qwt");

        assert_eq!(repo_dir(root, "cli", "cli"), PathBuf::from("/qwt/cli/cli"));
        assert_eq!(
            worktree_path(root, "cli", "cli", "fix/parser"),
            PathBuf::from("/qwt/cli/cli/fix/parser")
        );
        assert_eq!(
            worktree_path(root, "cli", "cli", "feature/login"),
            PathBuf::from("/qwt/cli/cli/feature/login")
        );
    }

    #[test]
    fn identifies_and_discovers_qwt_repo() -> Result<()> {
        let temp = tempdir()?;
        let repo = temp.path().join("owner").join("repo");
        fs::create_dir_all(repo.join(BARE_DIR))?;
        fs::write(repo.join(".git"), GITDIR_POINTER)?;

        assert!(is_qwt_repo(&repo));

        let nested = repo.join("feature").join("login");
        fs::create_dir_all(&nested)?;
        // Compare canonical forms: `discover_repo_root` strips the Windows
        // `\\?\` verbatim prefix for git-friendliness, while `Path::canonicalize`
        // keeps it, so normalize both sides to assert they name the same dir.
        assert_eq!(
            discover_repo_root(&nested)?.canonicalize()?,
            repo.canonicalize()?
        );

        Ok(())
    }

    #[test]
    fn rejects_non_qwt_repo_and_missing_discovery() -> Result<()> {
        let temp = tempdir()?;
        assert!(!is_qwt_repo(temp.path()));
        assert!(discover_repo_root(temp.path()).is_err());
        Ok(())
    }

    #[test]
    fn detects_ancestor_prefix_collision() -> Result<()> {
        let temp = tempdir()?;
        let repo = temp.path();
        let existing = repo.join("feat");
        fs::create_dir_all(&existing)?;
        fs::write(existing.join(".git"), "gitdir: ../.bare/worktrees/feat\n")?;

        assert!(check_prefix_collision(repo, "feat/x").is_err());
        Ok(())
    }

    #[test]
    fn detects_requested_branch_prefix_collision() -> Result<()> {
        let temp = tempdir()?;
        let repo = temp.path();
        let existing = repo.join("feat").join("x");
        fs::create_dir_all(&existing)?;
        fs::write(
            existing.join(".git"),
            "gitdir: ../../.bare/worktrees/feat-x\n",
        )?;

        assert!(check_prefix_collision(repo, "feat").is_err());
        Ok(())
    }

    #[test]
    fn detects_requested_branch_prefix_collision_for_existing_child_dir() -> Result<()> {
        let temp = tempdir()?;
        let repo = temp.path();
        fs::create_dir_all(repo.join("feat").join("x"))?;

        assert!(check_prefix_collision(repo, "feat").is_err());
        Ok(())
    }

    #[test]
    fn allows_non_colliding_nested_branch() -> Result<()> {
        let temp = tempdir()?;
        let repo = temp.path();
        fs::create_dir_all(repo.join("feat"))?;
        File::create(repo.join("main.rs"))?;

        check_prefix_collision(repo, "feat/x")?;
        check_prefix_collision(repo, "new/branch")?;
        Ok(())
    }
}
