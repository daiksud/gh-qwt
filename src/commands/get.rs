//! `gh qwt get` — clone (bare) and create the default-branch worktree.
//!
//! See `docs/development/specification/README.md` (`get`).

use crate::{config, gh, git, repo};
use anyhow::{bail, Context, Result};

/// Arguments for `gh qwt get`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Repository: `owner/repo`, an HTTPS URL, or an SSH URL.
    #[arg(value_name = "REPO")]
    pub spec: String,

    /// Create a worktree for this branch instead of the default branch.
    #[arg(short = 'b', long = "branch", value_name = "BRANCH")]
    pub branch: Option<String>,

    /// Host to use when the repo spec is `owner/repo`.
    #[arg(long, value_name = "HOST", default_value = "github.com")]
    pub host: String,
}

/// Run the `get` command.
pub fn run(args: Args) -> Result<()> {
    let spec = repo::RepoSpec::parse(&args.spec, &args.host).map_err(crate::error::usage_from)?;
    let root = config::resolve_root()?;
    let repo_dir = repo::repo_dir(&root, &spec.owner, &spec.repo);

    let bare_dir = repo_dir.join(repo::BARE_DIR);
    if bare_dir.exists() {
        bail!(
            "repository already exists at {} (remove it or use `gh qwt add`)",
            repo_dir.display()
        );
    }

    // Choose the branch to check out: an explicit --branch, or the detected default.
    let branch = match args.branch {
        Some(branch) => branch,
        None => gh::default_branch_with_fallback(&spec)?,
    };

    std::fs::create_dir_all(&repo_dir)
        .with_context(|| format!("failed to create {}", repo_dir.display()))?;

    git::clone_bare(&spec.clone_url, &bare_dir)?;

    std::fs::write(repo_dir.join(".git"), repo::GITDIR_POINTER)
        .with_context(|| format!("failed to write {}/.git pointer", repo_dir.display()))?;

    git::configure_and_fetch(&repo_dir)?;

    let worktree = repo::worktree_path(&root, &spec.owner, &spec.repo, &branch);
    if git::local_branch_exists(&repo_dir, &branch)? {
        // `git clone --bare` already created local heads for the remote's
        // branches, so attach the existing branch instead of re-creating it.
        git::worktree_add_existing(&repo_dir, &branch, &worktree)?;
    } else if git::remote_branch_exists(&repo_dir, &branch)? {
        git::worktree_add_tracking(&repo_dir, &branch, &worktree)?;
    } else {
        bail!("branch '{branch}' not found in {}", spec.clone_url);
    }

    println!("{}", worktree.display());
    Ok(())
}
