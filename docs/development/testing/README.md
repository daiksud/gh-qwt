# Testing

Testing strategy for `gh-qwt`, including unit tests for pure logic and offline integration tests for git/filesystem behavior.

## Table of contents

- [Testing philosophy](#testing-philosophy)
- [Unit tests](#unit-tests)
- [Offline integration tests](#offline-integration-tests)
- [How to run](#how-to-run)

## Testing philosophy

`gh-qwt` manages a bare repository plus per-branch worktrees, while shelling out to `gh` and `git`. Tests should separate pure logic from command-driven filesystem behavior:

- Pure logic is unit-tested directly.
- Git and filesystem behavior is covered by offline integration tests.
- Integration tests must not require network access.
- Tests should exercise user-visible `gh qwt` behavior rather than private implementation details when possible.

> [!IMPORTANT]
> Offline integration tests should use local repositories and `file://` URLs. They must not depend on GitHub network access or external repositories.

## Unit tests

Unit tests should cover deterministic logic without invoking network calls or relying on a user's machine state.

| Area | Concrete cases |
| --- | --- |
| Repository spec parsing | Parse `OWNER/REPO`; reject missing owner or repository; preserve valid `-`, `_`, and `.` characters. |
| URL parsing | Accept HTTPS GitHub URLs; accept SSH GitHub URLs; accept local `file://` URLs for tests; reject unsupported or malformed inputs. |
| Path building | Build paths under the qwt root; map repository identity to a stable directory; produce `.bare/` and branch worktree paths. |
| Root resolution | Respect explicit configuration when present; fall back to the default qwt root; report helpful errors for invalid roots. |
| Branch naming | Preserve branch names that are valid path segments; handle slash-containing branch names consistently. |
| Command planning | Ensure `get`, `add`, `list`, `rm`, `root`, `path`, and `prune` compute the expected git operations before execution. |

Example test shape:

```rust
#[test]
fn parses_owner_repo_spec() {
    let spec = RepoSpec::parse("daiksud/gh-qwt").unwrap();
    assert_eq!(spec.owner(), "daiksud");
    assert_eq!(spec.name(), "gh-qwt");
}
```

## Offline integration tests

Offline integration tests should create a local source repository in a temporary directory, then clone from it with a `file://` URL. This allows `gh qwt get` to exercise git clone and worktree behavior without network access.

When the source is a `file://` URL, default-branch detection should use the `git ls-remote --symref` fallback because `gh api` needs GitHub.

### Source repository setup

One simple setup is a non-bare local repository with a renamed default branch:

```bash
source_dir="$test_dir/source"
mkdir -p "$source_dir"
cd "$source_dir"

git init
git config user.name "gh-qwt test"
git config user.email "gh-qwt-test@example.invalid"
printf 'hello\n' > README.md
git add README.md
git commit -m 'Initial commit'
git branch -m trunk
```

Use its absolute path as a `file://` clone source:

```bash
source_url="file://$source_dir"
gh qwt get "$source_url"
```

Another realistic setup uses a bare remote and pushes `trunk` into it:

```bash
remote_dir="$test_dir/source.git"
work_dir="$test_dir/source-work"

git init --bare "$remote_dir"
git init "$work_dir"
cd "$work_dir"
git config user.name "gh-qwt test"
git config user.email "gh-qwt-test@example.invalid"
printf 'hello\n' > README.md
git add README.md
git commit -m 'Initial commit'
git branch -m trunk
git remote add origin "$remote_dir"
git push -u origin trunk

source_url="file://$remote_dir"
gh qwt get "$source_url"
```

### Assertions

After `gh qwt get`, assert the managed checkout shape:

| Assertion | Why it matters |
| --- | --- |
| `.bare/` exists | Confirms the repository was cloned as the bare git database. |
| `.git` is a pointer file | Confirms the worktree points at the bare repository instead of embedding a full `.git/` directory. |
| `.git` contains a `gitdir:` entry | Confirms Git can resolve the worktree metadata. |
| `trunk/` exists | Confirms the default branch worktree was created. |
| `trunk` is on a real branch | Confirms the checkout is not detached. |
| `git -C trunk status --short` succeeds | Confirms the worktree is usable by normal git commands. |

Then exercise the rest of the command surface offline:

```bash
gh qwt add feature/example
gh qwt list
gh qwt path feature/example
gh qwt rm feature/example
gh qwt prune
```

Suggested assertions for those commands:

- `add` creates a new worktree for the requested branch.
- `list` includes existing worktrees and omits removed ones.
- `path` prints the expected path inside the qwt root.
- `rm` removes the worktree without deleting the bare repository.
- `prune` removes stale git worktree metadata and is safe to run repeatedly.

## How to run

Run all tests with:

```bash
cargo test
```

Run a single test by name with:

```bash
cargo test parses_owner_repo_spec
```

Run integration tests with output visible when debugging:

```bash
cargo test --test cli -- --nocapture
```

> [!NOTE]
> `assert_cmd` is useful for CLI integration tests, and `tempfile` is useful for isolated temporary test directories. Add test dependencies only when the Rust test suite is introduced.
