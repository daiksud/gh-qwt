---
type: reference
title: "CLI reference"
description: "Complete reference for every gh qwt command, argument, flag, and exit code."
resource: gh-qwt
tags: [gh-qwt, reference, cli]
timestamp: 2026-07-09
---

# CLI reference

Exhaustive command reference for `gh qwt`, the `gh-qwt` GitHub CLI extension.

## Table of contents

- [Global](#global)
  - [Usage](#usage)
  - [Arguments and repo specs](#arguments-and-repo-specs)
  - [Exit codes](#exit-codes)
- [get](#get)
- [add](#add)
- [list](#list)
- [remove](#remove)
- [root](#root)
- [path](#path)
- [prune](#prune)

## Global

`gh-qwt` is invoked as a GitHub CLI extension:

```console
$ gh qwt <command> [flags] [args]
```

It clones GitHub repositories as one bare repository plus per-branch worktrees. The on-disk layout is described in the [directory layout reference](../directory-layout/).

### Usage

```console
$ gh qwt <command> [flags] [args]
```

All commands support `-h` and `--help` to print command-specific help.

The qwt root is resolved in this order:

1. `QWT_ROOT` environment variable
2. `git config --get qwt.root`
3. `~/qwt`

Repository paths do not include the host name:

```text
<qwt_root>/<owner>/<repo>/<branch>
```

For example, with qwt root `~/qwt`, repo `cli/cli`, and branch `fix/parser`, the worktree path is:

```text
~/qwt/cli/cli/fix/parser
```

### Arguments and repo specs

A repo spec identifies a GitHub repository. Commands that accept a repo spec accept these formats:

| Format | Example | Notes |
| --- | --- | --- |
| `owner/repo` | `cli/cli` | Host defaults to `github.com`. |
| HTTPS URL | `https://github.com/cli/cli.git` | `.git` suffix is optional. |
| SSH URL | `git@github.com:cli/cli.git` | Uses the SSH remote form. |
| Local URL | `file:///path/to/repo` | Clones a local repository. `owner` is taken from the parent directory and `repo` from the final path component. Primarily for local repositories and offline testing. |

The `--host <host>` flag on [`get`](#get) changes the host used with the `owner/repo` form. The path layout still omits the host.

### Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Success. |
| `1` | Runtime error, such as a `git` or `gh` failure, missing repository, missing worktree, or not-found condition. |
| `2` | Invalid usage or invalid arguments. |

## get

### Synopsis

```console
$ gh qwt get [flags] <owner>/<repo>|<url>
```

### Description

Clone a repository into the qwt root as a bare repository and create a worktree for the default branch or a selected branch.

`get` resolves the default branch with `gh api repos/{owner}/{repo} -q .default_branch`. If that fails, it falls back to `git ls-remote --symref <url> HEAD`. It then:

1. Runs `git clone --bare` into `<root>/<owner>/<repo>/.bare`.
2. Writes a `.git` pointer file at `<root>/<owner>/<repo>/.git` containing:

   ```text
   gitdir: ./.bare
   ```

3. Sets the fetch refspec to `+refs/heads/*:refs/remotes/origin/*` and fetches.
4. Runs `git worktree add <default_branch>` or `git worktree add <branch>`.
5. Creates the worktree as a real local tracking branch, not a detached `HEAD`.
6. Prints the worktree path.

### Arguments

| Argument | Description |
| --- | --- |
| `<owner>/<repo>|<url>` | Repository to clone. Accepts any [repo spec](#arguments-and-repo-specs). |

### Flags

| Flag | Description | Default |
| --- | --- | --- |
| `-b, --branch <branch>` | Create a worktree for this branch instead of the default branch. | Repository default branch |
| `--host <host>` | Host to use when the repo spec is `owner/repo`. | `github.com` |
| `-h, --help` | Print help for `get`. | Off |

### Examples

```console
$ gh qwt get cli/cli
~/qwt/cli/cli/trunk
```

```console
$ gh qwt get --branch fix/parser cli/cli
~/qwt/cli/cli/fix/parser
```

```console
$ gh qwt get https://github.com/cli/cli.git
~/qwt/cli/cli/trunk
```

```console
$ gh qwt get git@github.com:cli/cli.git
~/qwt/cli/cli/trunk
```

### Notes

- The repo directory is `<qwt_root>/<owner>/<repo>`; the bare repository is stored in its `.bare` directory.
- The default branch example in this page uses `trunk` for `cli/cli`.
- The printed path is intended to be copied or consumed by shell integration.

## add

### Synopsis

```console
$ gh qwt add [flags] <branch>
```

### Description

Add a worktree for a branch in an existing qwt-managed repository.

By default, `add` discovers the repository root by walking up from the current working directory until it finds a qwt repo directory containing `.bare` and the `.git` pointer file. Use `--repo` to operate on a specific repository without discovery.

If the branch exists on the remote, `add` creates a tracking worktree for that branch. Otherwise, it creates a new local branch from the base ref selected by `--from`, or from the repository default branch when `--from` is omitted.

Branch names containing `/` create nested worktree directories. For example, `fix/parser` becomes `<root>/cli/cli/fix/parser`.

### Arguments

| Argument | Description |
| --- | --- |
| `<branch>` | Branch name for the worktree to add. Slash-separated names create nested directories. |

### Flags

| Flag | Description | Default |
| --- | --- | --- |
| `--repo <owner>/<repo>` | Operate on this repository instead of discovering the repo from the current directory. | Discover from current directory |
| `--from <ref>` | Base ref for a new branch when no matching remote branch exists. | Repository default branch |
| `-h, --help` | Print help for `add`. | Off |

### Examples

From inside `~/qwt/cli/cli/trunk`:

```console
$ gh qwt add fix/parser
~/qwt/cli/cli/fix/parser
```

From anywhere, selecting the repository explicitly:

```console
$ gh qwt add --repo cli/cli fix/parser
~/qwt/cli/cli/fix/parser
```

Create a new branch from `trunk`:

```console
$ gh qwt add --repo cli/cli --from trunk fix/parser
~/qwt/cli/cli/fix/parser
```

### Notes

> [!WARNING]
> Branch names can collide by path prefix. For example, a worktree named `feat` conflicts with the parent directory needed for `feat/x`. `add` warns on these prefix collisions.

- Existing remote branches are created as tracking worktrees.
- New branches are created from `--from <ref>` or the default branch.
- Use [`path`](#path) when you need the absolute worktree path for `cd` or scripts.

## list

### Synopsis

```console
$ gh qwt list [flags] [<query>]
```

### Description

List qwt-managed repositories and their worktrees.

`list` iterates over `<root>/*/*`, treats each matching owner/repo directory as a repository, runs `git worktree list` for each repository, and prints a flat, sorted list of `owner/repo/branch` — one entry per line, with no repository headers or indentation. This shape is modeled on [`ghq list`](https://github.com/x-motemen/ghq) and is safe to pipe directly into tools like `fzf`, `grep`, or `xargs`.

If `<query>` is given, only entries whose `owner/repo/branch` contains that text are listed (case-insensitive unless `<query>` contains an uppercase letter — smartcase). `-e`/`--exact` requires `<query>` to exactly match `branch`, `repo/branch`, or `owner/repo/branch` instead of a substring.

### Arguments

| Argument | Description |
| --- | --- |
| `<query>` | Optional filter text. Substring match by default (smartcase); see `-e`/`--exact` for exact matching. |

### Flags

| Flag | Description | Default |
| --- | --- | --- |
| `-e, --exact` | Require `<query>` to exactly match `branch`, `repo/branch`, or `owner/repo/branch`. Has no effect without `<query>`. | Off (substring match) |
| `-p, --full-path` | Print absolute paths instead of `owner/repo/branch`. | Off |
| `-h, --help` | Print help for `list`. | Off |

### Examples

```console
$ gh qwt list
cli/cli/fix/parser
cli/cli/trunk
```

```console
$ gh qwt list --full-path
/Users/you/qwt/cli/cli/fix/parser
/Users/you/qwt/cli/cli/trunk
```

```console
$ gh qwt list fix
cli/cli/fix/parser
```

```console
$ gh qwt list --exact trunk
cli/cli/trunk
```

Pipe straight into a fuzzy finder and `cd` into the selection:

```bash
d=$(gh qwt list --full-path | fzf) && cd "$d"
```

### Notes

- `list` reads repositories under the resolved qwt root.
- Repositories outside `<qwt_root>/<owner>/<repo>` are not included.
- The command reports worktrees known to `git worktree list` for each bare repository.
- Output is always sorted lexicographically, whether printing specs or full paths.
- See the [shell integration guide](../../guides/shell-integration/) for more on piping `list` into a fuzzy picker.

## remove

### Synopsis

```console
$ gh qwt remove [flags] <branch>
$ gh qwt remove [flags] <owner>/<repo>
$ gh qwt remove [flags] <owner>/<repo>/<branch>
$ gh qwt rm ...   # alias, identical behavior
```

### Description

Remove a worktree, or an entire repository. `rm` is an alias for `remove` — both names behave
identically.

The form depends on where you run it and what you pass:

- Inside a qwt-managed repository, the argument is always treated as a **branch name** for that
  repository (discovered by walking up from the current directory), and only that worktree is
  removed — the entire argument, including any `/` it contains, is the branch name.
- Outside any qwt-managed repository, the argument must be an explicit spec: `owner/repo` removes
  the **entire repository** (`.bare` plus every worktree); `owner/repo/branch` removes **only**
  that worktree.

`remove` cannot target a different repository's worktree while you're standing inside another
repository — `cd` out first (for example, to the qwt root) to use the explicit `owner/repo[/branch]`
form for a different repository.

### Arguments

| Argument | Description |
| --- | --- |
| `<branch>` | Inside a qwt repository: the worktree to remove. Slash-separated names map to nested directories. |
| `<owner>/<repo>` | Outside any qwt repository: the entire repository to remove. |
| `<owner>/<repo>/<branch>` | Outside any qwt repository: only that worktree to remove. |

### Flags

| Flag | Description | Default |
| --- | --- | --- |
| `--force` | Remove a worktree even when it has local changes, or skip the confirmation prompt when removing an entire repository. | Off |
| `--delete-branch` | Also delete the local branch with `git branch -D`. Only applies when removing a single worktree. | Off |
| `-h, --help` | Print help for `remove`. | Off |

### Examples

From inside a worktree for `cli/cli`:

```console
$ gh qwt remove fix/parser
$ gh qwt rm fix/parser
```

Force removal when local changes exist, and also delete the local branch:

```console
$ gh qwt remove --force --delete-branch fix/parser
```

From anywhere, removing a single worktree in a specific repository:

```console
$ gh qwt remove cli/cli/fix/parser
```

From anywhere, removing an entire repository:

```console
$ gh qwt remove cli/cli
Remove ~/qwt/cli/cli and all worktrees? [y/N]
```

```console
$ gh qwt remove --force cli/cli
```

### Notes

> [!WARNING]
> `--force` can discard uncommitted worktree changes. Removing an entire repository discards its
> `.bare` database and every worktree; there is no undo.

- Removing a single worktree does not touch the rest of the repository.
- To clean up worktrees whose branch is gone from the remote instead of removing them by name, see
  [`prune`](#prune).

## root

### Synopsis

```console
$ gh qwt root
```

### Description

Print the resolved qwt root path.

Resolution order is `QWT_ROOT`, then `git config --get qwt.root`, then `~/qwt`.

### Arguments

| Argument | Description |
| --- | --- |
| None | `root` does not accept positional arguments. |

### Flags

| Flag | Description | Default |
| --- | --- | --- |
| `-h, --help` | Print help for `root`. | Off |

### Examples

```console
$ gh qwt root
~/qwt
```

```bash
cd "$(gh qwt root)"
```

### Notes

- Use `root` in scripts that need to locate the qwt root without duplicating resolution logic.
- The printed path is absolute after shell expansion and configuration resolution.

## path

### Synopsis

```console
$ gh qwt path [<owner>/<repo>[/<branch>]]
```

### Description

Print an absolute path suitable for shell `cd` commands and scripts.

With no argument, `path` prints the qwt root. With `owner/repo`, it prints the repository directory. With `owner/repo/branch`, it prints the branch worktree path.

`path` is intended for use by a shell function. See the [shell integration guide](../../guides/shell-integration/).

### Arguments

| Argument | Description |
| --- | --- |
| None | Print the qwt root. |
| `<owner>/<repo>` | Print the repository directory. |
| `<owner>/<repo>/<branch>` | Print the worktree path for `branch`. Branch may contain `/`, such as `fix/parser`. |

### Flags

| Flag | Description | Default |
| --- | --- | --- |
| `-h, --help` | Print help for `path`. | Off |

### Examples

```console
$ gh qwt path
~/qwt
```

```console
$ gh qwt path cli/cli
~/qwt/cli/cli
```

```console
$ gh qwt path cli/cli/trunk
~/qwt/cli/cli/trunk
```

```console
$ gh qwt path cli/cli/fix/parser
~/qwt/cli/cli/fix/parser
```

```bash
cd "$(gh qwt path cli/cli/fix/parser)"
```

### Notes

- `path` prints paths; it does not create repositories or worktrees, and does not check whether the target exists — it works equally well for a worktree that doesn't exist yet.
- Use [`get`](#get) to clone a repository and [`add`](#add) to create additional worktrees.
- For an *existing* worktree, [`list --full-path`](#list) (optionally filtered by query or piped into a fuzzy finder) prints the same path without a second `path` lookup. `path` remains useful for the `owner/repo` (repository directory) form and for computing a worktree path before it exists.

## prune

### Synopsis

```console
$ gh qwt prune [flags]
```

### Description

Remove worktrees whose branch is gone from the remote, and clean up stale worktree metadata.
Modeled on real git's own "prune" vocabulary (`git worktree prune`, `git fetch --prune`): a safe,
automatic cleanup, not a way to delete an entire repository. For that, see
[`remove`](#remove)/`rm`.

`prune` takes no argument — like `git worktree prune`, it always operates on the repository
discovered from the current directory. It:

1. Runs `git fetch origin --prune` to refresh remote-tracking refs.
2. Runs `git worktree prune` to remove administrative metadata for worktree directories that were
   deleted outside of `gh qwt` (for example, by hand with `rm -rf`).
3. Removes each remaining worktree (and its local branch) whose branch has an upstream configured
   **and** whose upstream is no longer on `origin` — the common signal that a pull request was
   merged and the remote deleted the source branch.

A branch that was never pushed (no upstream configured) is never touched, even though it will also
look "absent from origin." A worktree with uncommitted or untracked changes is skipped and
reported, never force-removed. The repository's default branch is never a candidate. `prune` lists
the candidates and asks for confirmation once, unless `-y`/`--force`.

### Arguments

| Argument | Description |
| --- | --- |
| None | `prune` operates on the repository discovered from the current directory. |

### Flags

| Flag | Description | Default |
| --- | --- | --- |
| `-y, --force` | Skip confirmation and remove the pruned worktrees and branches. | Off |
| `-h, --help` | Print help for `prune`. | Off |

### Examples

From inside any worktree of `cli/cli`:

```console
$ gh qwt prune
Fetching origin...
The following worktrees are no longer on the remote and will be removed:
  fix/parser
Remove these worktrees and their local branches? [y/N]
```

```console
$ gh qwt prune --force
```

Nothing to clean up:

```console
$ gh qwt prune -y
Fetching origin...
Nothing to prune.
```

### Notes

- `prune` only removes worktrees whose branch *had* a remote counterpart that is now gone; it never
  guesses based on merge status, since a squash- or rebase-merged branch's commits are never
  ancestors of the result on the target branch.
- Use [`remove`](#remove)/`rm` to remove a worktree or an entire repository by name.
- `prune` operates under the resolved qwt root and does not include the host in the path.
