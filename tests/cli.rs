//! Offline integration tests for the `gh qwt` CLI.
//!
//! These tests build the real binary and run it against a local `file://`
//! source repository, so they need `git` but no network access. See
//! `docs/development/testing/README.md`.

use assert_cmd::Command;
use predicates::prelude::PredicateStrExt;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use tempfile::TempDir;

/// A prepared test environment: a local source repo plus a qwt root.
struct Env {
    _tmp: TempDir,
    /// `file://` URL of the source repository.
    src_url: String,
    /// Path to the source repository (for direct git assertions).
    src: PathBuf,
    /// The qwt root passed to the CLI via `QWT_ROOT`.
    root: PathBuf,
}

impl Env {
    /// Create a source repo at `<tmp>/acme/widget` with default branch `main`
    /// and an extra branch `feature/x`, plus an empty qwt root at `<tmp>/root`.
    fn new() -> Env {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("acme").join("widget");
        std::fs::create_dir_all(&src).unwrap();

        git(&src, &["init", "-q", "-b", "main"]);
        // Disable signing and set an identity so commits work in any environment.
        git(
            &src,
            &[
                "-c",
                "user.email=test@example.com",
                "-c",
                "user.name=Test",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-q",
                "--allow-empty",
                "-m",
                "init",
            ],
        );
        git(&src, &["branch", "feature/x"]);

        let root = tmp.path().join("root");
        let src_url = format!("file://{}", src.display());
        Env {
            _tmp: tmp,
            src_url,
            src,
            root,
        }
    }

    /// A `gh-qwt` command with `QWT_ROOT` pointed at this env's root.
    fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("gh-qwt").unwrap();
        cmd.env("QWT_ROOT", &self.root);
        cmd
    }

    /// A `gh-qwt` command whose working directory is inside the repo tree, to
    /// exercise repository discovery.
    fn cmd_in(&self, dir: &Path) -> Command {
        let mut cmd = self.cmd();
        cmd.current_dir(dir);
        cmd
    }

    fn repo_dir(&self) -> PathBuf {
        self.root.join("acme").join("widget")
    }

    fn worktree(&self, branch: &str) -> PathBuf {
        branch
            .split('/')
            .fold(self.repo_dir(), |p, seg| p.join(seg))
    }
}

fn git(dir: &Path, args: &[&str]) {
    let status = StdCommand::new("git")
        .current_dir(dir)
        .args(args)
        .status()
        .expect("spawn git");
    assert!(status.success(), "git {args:?} failed in {}", dir.display());
}

/// The branch a worktree currently has checked out.
fn current_branch(worktree: &Path) -> String {
    let out = StdCommand::new("git")
        .current_dir(worktree)
        .args(["branch", "--show-current"])
        .output()
        .expect("spawn git");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn get_creates_bare_repo_and_default_worktree() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();

    let repo = env.repo_dir();
    assert!(repo.join(".bare").is_dir(), ".bare should be a directory");

    let pointer = std::fs::read_to_string(repo.join(".git")).unwrap();
    assert_eq!(pointer, "gitdir: ./.bare\n", ".git pointer contents");

    let wt = env.worktree("main");
    assert!(wt.is_dir(), "default-branch worktree should exist");
    assert_eq!(current_branch(&wt), "main", "worktree is on a real branch");
}

#[test]
fn get_without_branch_detects_default() {
    // No `-b`: default-branch detection tries `gh` then falls back to
    // `git ls-remote --symref`, which works offline for a file:// source.
    let env = Env::new();
    env.cmd().args(["get", &env.src_url]).assert().success();
    assert!(env.worktree("main").is_dir());
    assert_eq!(current_branch(&env.worktree("main")), "main");
}

#[test]
fn get_rejects_existing_repository() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    // A second get into the same destination must fail (exit 1).
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn get_rejects_invalid_spec_with_exit_2() {
    let env = Env::new();
    env.cmd().args(["get", "a/b/c"]).assert().code(2);
}

#[test]
fn add_creates_existing_and_new_branch_worktrees() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();

    // Existing branch present at clone time -> attach.
    let feature_x = env.worktree("feature/x");
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success()
        .stdout(format!("{}\n", feature_x.display()));
    assert_eq!(current_branch(&feature_x), "feature/x");

    // Brand-new branch, discovered from within the repo tree.
    let topic_new = env.worktree("topic/new");
    let assertion = env
        .cmd_in(&env.worktree("main"))
        .args(["add", "topic/new"])
        .assert()
        .success();
    // Repository discovery starts from `current_dir`, which resolves macOS's
    // `/var` -> `/private/var` symlink before `add` prints the destination.
    let topic_new_output = std::fs::canonicalize(&topic_new).unwrap();
    assertion.stdout(format!("{}\n", topic_new_output.display()));
    assert_eq!(current_branch(&topic_new), "topic/new");
}

#[test]
fn add_rejects_prefix_collision() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "topic"])
        .assert()
        .success();
    // `topic` already exists as a worktree; `topic/child` would nest under it.
    env.cmd()
        .args(["add", "--repo", "acme/widget", "topic/child"])
        .assert()
        .failure()
        .code(1)
        .stdout("");
}

#[test]
fn list_prints_flat_sorted_spec_and_full_path() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    // Flat, sorted `owner/repo/branch` per line: no repository header and no
    // indentation, so the output is safe to pipe straight into a fuzzy
    // finder like fzf and use the selected line for `cd`.
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::diff("acme/widget/feature/x\nacme/widget/main\n").from_utf8());

    // --full-path prints the same flat shape with absolute paths instead.
    // `git worktree list` reports canonicalized paths (e.g. resolving
    // macOS's `/var` -> `/private/var` symlink), so canonicalize the
    // expected side too.
    let expected_full_path = format!(
        "{}\n{}\n",
        env.worktree("feature/x").canonicalize().unwrap().display(),
        env.worktree("main").canonicalize().unwrap().display(),
    );
    env.cmd()
        .args(["list", "--full-path"])
        .assert()
        .success()
        .stdout(predicates::str::diff(expected_full_path).from_utf8());
}

#[test]
fn list_query_filters_by_substring_with_smartcase() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    // A lowercase query is a case-insensitive substring match.
    env.cmd()
        .args(["list", "feature"])
        .assert()
        .success()
        .stdout(predicates::str::diff("acme/widget/feature/x\n").from_utf8());

    // A query containing an uppercase letter is case-sensitive (smartcase),
    // so it no longer matches the (lowercase) "feature/x" branch.
    env.cmd()
        .args(["list", "Feature"])
        .assert()
        .success()
        .stdout(predicates::str::diff("").from_utf8());

    // A query that matches nothing prints nothing.
    env.cmd()
        .args(["list", "nope"])
        .assert()
        .success()
        .stdout(predicates::str::diff("").from_utf8());
}

#[test]
fn list_exact_matches_branch_repo_branch_or_full_spec() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    // Matches at the `branch` level.
    env.cmd()
        .args(["list", "--exact", "main"])
        .assert()
        .success()
        .stdout(predicates::str::diff("acme/widget/main\n").from_utf8());

    // Matches at the `repo/branch` level.
    env.cmd()
        .args(["list", "-e", "widget/main"])
        .assert()
        .success()
        .stdout(predicates::str::diff("acme/widget/main\n").from_utf8());

    // Matches at the full `owner/repo/branch` level.
    env.cmd()
        .args(["list", "-e", "acme/widget/main"])
        .assert()
        .success()
        .stdout(predicates::str::diff("acme/widget/main\n").from_utf8());

    // A plain substring is no longer sufficient under --exact.
    env.cmd()
        .args(["list", "-e", "mai"])
        .assert()
        .success()
        .stdout(predicates::str::diff("").from_utf8());

    // `--exact` with no query is a no-op: everything is listed (matches
    // `ghq list`'s behavior).
    env.cmd()
        .args(["list", "--exact"])
        .assert()
        .success()
        .stdout(predicates::str::diff("acme/widget/feature/x\nacme/widget/main\n").from_utf8());
}

#[test]
fn list_output_round_trips_through_path() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    // Each line printed by `list` is a valid `path` spec that resolves back
    // to the worktree it came from -- the core "select in fzf, then cd" use
    // case this feature exists for.
    let assert = env.cmd().arg("list").assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines, vec!["acme/widget/feature/x", "acme/widget/main"]);

    for line in lines {
        let branch = line.strip_prefix("acme/widget/").expect("spec prefix");
        env.cmd()
            .args(["path", line])
            .assert()
            .success()
            .stdout(predicates::str::diff(format!(
                "{}\n",
                env.worktree(branch).display()
            )));
    }
}

#[test]
fn path_prints_root_repo_and_worktree() {
    let env = Env::new();

    env.cmd()
        .arg("root")
        .assert()
        .success()
        .stdout(predicates::str::contains(
            env.root.to_string_lossy().as_ref(),
        ));

    env.cmd()
        .arg("path")
        .assert()
        .success()
        .stdout(predicates::str::contains(
            env.root.to_string_lossy().as_ref(),
        ));

    env.cmd()
        .args(["path", "acme/widget"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            env.repo_dir().to_string_lossy().as_ref(),
        ));

    env.cmd()
        .args(["path", "acme/widget/feature/x"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            env.worktree("feature/x").to_string_lossy().as_ref(),
        ));

    // Malformed argument -> exit 2.
    env.cmd().args(["path", "solo"]).assert().code(2);
}

#[test]
fn rm_removes_worktree_and_optionally_branch() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    let wt = env.worktree("feature/x");
    assert!(wt.is_dir());

    // `rm` is an alias of `remove`; a bare branch name is resolved by
    // discovering the repository from the current directory, exactly like
    // the original `rm`.
    env.cmd_in(&env.worktree("main"))
        .args(["rm", "--delete-branch", "feature/x"])
        .assert()
        .success();
    assert!(!wt.exists(), "worktree directory should be gone");

    // The local branch should have been deleted too.
    let out = StdCommand::new("git")
        .current_dir(env.repo_dir())
        .args(["branch", "--list", "feature/x"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        "branch feature/x should be deleted"
    );

    // Removing a non-existent worktree fails (exit 1).
    env.cmd_in(&env.worktree("main"))
        .args(["rm", "does-not-exist"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn remove_primary_name_also_removes_worktree_via_cwd_discovery() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    let wt = env.worktree("feature/x");

    // Same behavior under the primary `remove` name (not just the `rm` alias).
    env.cmd_in(&env.worktree("main"))
        .args(["remove", "feature/x"])
        .assert()
        .success();
    assert!(!wt.exists(), "worktree directory should be gone");
}

#[test]
fn remove_owner_repo_from_outside_removes_entire_repository() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    assert!(env.repo_dir().exists());

    // Run from outside any qwt repository (the test process's own cwd), so
    // the 2-segment spec is resolved explicitly rather than discovered.
    env.cmd()
        .args(["remove", "--force", "acme/widget"])
        .assert()
        .success();
    assert!(
        !env.repo_dir().exists(),
        "repository tree should be removed"
    );

    // Removing a non-existent repository fails (exit 1).
    env.cmd()
        .args(["remove", "--force", "no/such"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn remove_owner_repo_branch_from_outside_removes_only_that_worktree() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    // Explicit owner/repo/branch spec, run from outside any qwt repository:
    // only that worktree is removed, the rest of the repository stays intact.
    env.cmd()
        .args(["remove", "--delete-branch", "acme/widget/feature/x"])
        .assert()
        .success();
    assert!(!env.worktree("feature/x").exists());
    assert!(env.repo_dir().exists(), "repository should still exist");
    assert!(env.worktree("main").exists(), "main worktree should remain");

    let out = StdCommand::new("git")
        .current_dir(env.repo_dir())
        .args(["branch", "--list", "feature/x"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        "branch feature/x should be deleted"
    );
}

#[test]
fn remove_declined_keeps_repository() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();

    // Answering "n" at the confirmation prompt leaves the repo intact.
    env.cmd()
        .args(["remove", "acme/widget"])
        .write_stdin("n\n")
        .assert()
        .success();
    assert!(env.repo_dir().exists(), "repository should still exist");
}

#[test]
fn remove_rejects_malformed_spec_outside_repository_with_exit_2() {
    let env = Env::new();

    // A single segment with no slash, run from outside any qwt repository,
    // is neither a discoverable branch nor a valid owner/repo[/branch] spec.
    env.cmd().args(["remove", "solo"]).assert().code(2);
}

#[test]
fn prune_removes_worktree_whose_remote_branch_is_gone() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    // Simulate the remote deleting the branch (e.g. after a squash-merged PR).
    git(&env.src, &["branch", "-D", "feature/x"]);

    env.cmd_in(&env.worktree("main"))
        .args(["prune", "-y"])
        .assert()
        .success()
        .stdout(predicates::str::contains("feature/x"));

    assert!(
        !env.worktree("feature/x").exists(),
        "worktree with a remote-deleted branch should be pruned"
    );
    assert!(
        env.worktree("main").exists(),
        "the default branch worktree must never be pruned"
    );

    let out = StdCommand::new("git")
        .current_dir(env.repo_dir())
        .args(["branch", "--list", "feature/x"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        "the local branch should be deleted along with the worktree"
    );
}

#[test]
fn prune_never_touches_branch_without_upstream() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    // `totally/new` doesn't exist on the source at all, so `add` creates it
    // fresh with no upstream configured -- it must never look "prunable"
    // just because `origin/totally/new` also doesn't exist.
    env.cmd()
        .args(["add", "--repo", "acme/widget", "totally/new"])
        .assert()
        .success();

    env.cmd_in(&env.worktree("main"))
        .args(["prune", "-y"])
        .assert()
        .success();

    assert!(
        env.worktree("totally/new").exists(),
        "a branch that was never pushed must never be pruned"
    );
}

#[test]
fn prune_skips_dirty_candidate() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();

    git(&env.src, &["branch", "-D", "feature/x"]);

    // An uncommitted (untracked) file makes the worktree dirty.
    std::fs::write(env.worktree("feature/x").join("untracked.txt"), "wip").unwrap();

    env.cmd_in(&env.worktree("main"))
        .args(["prune", "-y"])
        .assert()
        .success()
        .stderr(predicates::str::contains("feature/x"));

    assert!(
        env.worktree("feature/x").exists(),
        "a dirty candidate must be skipped, not removed"
    );
}

#[test]
fn prune_prints_nothing_to_prune_when_clean() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();

    env.cmd_in(&env.worktree("main"))
        .args(["prune", "-y"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Nothing to prune"));
}

#[test]
fn prune_declined_keeps_worktree() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();
    env.cmd()
        .args(["add", "--repo", "acme/widget", "feature/x"])
        .assert()
        .success();
    git(&env.src, &["branch", "-D", "feature/x"]);

    env.cmd_in(&env.worktree("main"))
        .arg("prune")
        .write_stdin("n\n")
        .assert()
        .success();

    assert!(
        env.worktree("feature/x").exists(),
        "declining the prompt must keep the worktree"
    );
}

#[test]
fn prune_fails_outside_repository() {
    let env = Env::new();
    env.cmd()
        .args(["get", &env.src_url, "-b", "main"])
        .assert()
        .success();

    // Run from outside any qwt repository (the test process's own cwd).
    env.cmd().arg("prune").assert().failure().code(1);
}

#[test]
fn source_repo_is_initialized() {
    let env = Env::new();
    assert!(env.src.join(".git").exists());
}
