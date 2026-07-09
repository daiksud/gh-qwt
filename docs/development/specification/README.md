---
type: specification
title: "Specification"
description: "Normative behavior specification for each gh qwt command that the implementation must satisfy."
resource: gh-qwt
tags: [gh-qwt, development, specification]
timestamp: 2026-07-09
---

# Specification

Normative implementation contract for `gh-qwt`, the `gh qwt` GitHub CLI extension.

> [!NOTE]
> The keywords **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** are to be interpreted as described in RFC 2119.

## Table of contents

- [Status and scope](#status-and-scope)
- [Terminology](#terminology)
- [Global requirements](#global-requirements)
- [Root resolution](#root-resolution)
- [Repo-spec parsing](#repo-spec-parsing)
- [Path building](#path-building)
- [Repo discovery](#repo-discovery)
- [Default-branch detection](#default-branch-detection)
- [Bare repository requirements](#bare-repository-requirements)
- [Command specifications](#command-specifications)
  - [`get`](#get)
  - [`add`](#add)
  - [`list`](#list)
  - [`rm`](#rm)
  - [`root`](#root)
  - [`path`](#path)
  - [`prune`](#prune)
- [Exit codes](#exit-codes)
- [Edge cases](#edge-cases)
- [Related documents](#related-documents)

## Status and scope

This page defines the behavior that the Phase-2 implementation MUST satisfy. It is documentation only and does not imply code exists yet.

`gh-qwt` MUST be invokable as:

```console
$ gh qwt <command> [flags] [args]
```

The command set is `get`, `add`, `list`, `rm`, `root`, `path`, and `prune`. Command-line flags and examples are listed in the [CLI reference](../../references/cli/).

## Terminology

| Term | Definition |
| --- | --- |
| qwt root | Base directory under which qwt-managed repositories are stored. |
| repo spec | User input identifying a repository, such as `cli/cli`, `https://github.com/cli/cli.git`, or `git@github.com:cli/cli.git`. |
| bare repository | The git database stored at `<qwt_root>/<owner>/<repo>/.bare`. |
| worktree | A checked-out branch directory under `<qwt_root>/<owner>/<repo>/<branch>`. |
| default branch | The repository's default branch as reported by GitHub or remote `HEAD`. |

## Global requirements

- `gh-qwt` MUST shell out to `git` for git operations.
- `gh-qwt` MUST shell out to `gh` for auth-aware GitHub API operations.
- Paths MUST NOT include the host segment.
- User-facing success output that returns a path MUST print the resolved path on stdout.
- Runtime diagnostics MUST be written to stderr.
- Invalid command-line usage MUST return exit code `2`.

## Root resolution

The implementation MUST resolve the qwt root in this order:

1. If environment variable `QWT_ROOT` is set and non-empty, use its value.
2. Otherwise, run `git config --get qwt.root`; if it exits successfully and prints a non-empty value, use that value.
3. Otherwise, use `~/qwt`.

The selected value MUST expand a leading `~` to the user's home directory. The resolved qwt root SHOULD be normalized for filesystem operations while preserving user-friendly path output where appropriate.

Example:

```console
$ gh qwt root
~/qwt
```

## Repo-spec parsing

A repo spec MUST identify `host`, `owner`, `repo`, and a clone URL.

Accepted forms:

| Form | Example | Rules |
| --- | --- | --- |
| `owner/repo` | `cli/cli` | Host defaults to `github.com` unless a command flag supplies another host. Clone URL uses that host. |
| HTTPS URL | `https://github.com/cli/cli.git` | Host, owner, and repo are parsed from the URL path. A trailing `.git` suffix is ignored for the repo name. |
| SSH URL | `git@github.com:cli/cli.git` | Host is parsed before `:`, owner/repo after `:`. A trailing `.git` suffix is ignored for the repo name. |

Examples:

| Input | Host | Owner | Repo | Clone URL |
| --- | --- | --- | --- | --- |
| `cli/cli` | `github.com` | `cli` | `cli` | `https://github.com/cli/cli.git` |
| `cli/cli` with `--host github.example.com` | `github.example.com` | `cli` | `cli` | `https://github.example.com/cli/cli.git` |
| `https://github.com/cli/cli.git` | `github.com` | `cli` | `cli` | `https://github.com/cli/cli.git` |
| `https://github.com/cli/cli` | `github.com` | `cli` | `cli` | `https://github.com/cli/cli.git` |
| `git@github.com:cli/cli.git` | `github.com` | `cli` | `cli` | `git@github.com:cli/cli.git` |

Invalid specs, missing owner, missing repo, unsupported URL schemes, and malformed SSH forms MUST be rejected as invalid usage.

## Path building

Given qwt root `~/qwt`, owner `cli`, repo `cli`, and branch `fix/parser`:

| Entity | Path |
| --- | --- |
| Repository directory | `~/qwt/cli/cli` |
| Bare repository | `~/qwt/cli/cli/.bare` |
| Default branch worktree | `~/qwt/cli/cli/trunk` |
| Feature branch worktree | `~/qwt/cli/cli/fix/parser` |

The repository directory MUST be:

```text
<qwt_root>/<owner>/<repo>
```

A worktree path MUST be:

```text
<qwt_root>/<owner>/<repo>/<branch>
```

Branch names containing `/` MUST create nested directories. The host MUST NOT be included in any qwt-managed path.

## Repo discovery

Commands that operate on the current repository, such as `add` and `rm`, MUST discover the qwt repository root unless an explicit `--repo` flag is supported and provided.

Discovery MUST:

1. Start at the current working directory.
2. Inspect that directory and each parent directory.
3. Stop at the first directory that contains `.bare` and a `.git` pointer file for the qwt bare repository.
4. Return that directory as the repository directory.
5. Fail with a runtime error if no such directory is found before the filesystem root.

The `.git` pointer file MUST be recognized only when it points to `./.bare` as defined in [Bare repository requirements](#bare-repository-requirements).

## Default-branch detection

Default-branch detection MUST use this order:

1. Run `gh api repos/{owner}/{repo} -q .default_branch`.
2. If that fails, run `git ls-remote --symref <url> HEAD` and parse the line shaped like:

   ```text
   ref: refs/heads/<name>	HEAD
   ```

If both methods fail or no branch name can be parsed, the command MUST fail with a runtime error.

## Bare repository requirements

After `git clone --bare`, the repository directory MUST contain a `.git` pointer file at:

```text
<qwt_root>/<owner>/<repo>/.git
```

The file contents MUST be exactly:

```text
gitdir: ./.bare
```

Before creating worktrees, the implementation MUST configure the bare repository to fetch remote-tracking refs:

```console
$ git --git-dir=.bare config remote.origin.fetch '+refs/heads/*:refs/remotes/origin/*'
$ git --git-dir=.bare fetch origin
```

This is REQUIRED because bare clones omit the fetch refspec needed to populate `refs/remotes/origin/*` for later tracking worktrees.

## Command specifications

### `get`

#### Preconditions

- A valid repo spec MUST be provided.
- The target repository directory `<qwt_root>/<owner>/<repo>` MUST NOT already contain an incompatible repository.
- `git` MUST be available.
- `gh` SHOULD be available for the primary default-branch lookup; fallback to `git ls-remote` MUST be attempted when `gh` lookup fails.

#### Steps and effects

1. Parse the repo spec into host, owner, repo, and clone URL.
2. Resolve the qwt root.
3. Determine the default branch unless a branch flag selects another branch.
4. Create the parent path `<qwt_root>/<owner>/<repo>` as needed.
5. Run:

   ```console
   $ git clone --bare <url> <qwt_root>/<owner>/<repo>/.bare
   ```

6. Write `.git` with exactly `gitdir: ./.bare`.
7. Configure `remote.origin.fetch` to `+refs/heads/*:refs/remotes/origin/*` and fetch `origin`.
8. Add a worktree for the selected branch.
9. The created worktree MUST be a real local branch tracking `origin/<branch>` when the branch exists on origin. It MUST NOT be a detached `HEAD`.

#### Output

On success, `get` MUST print the worktree path, for example:

```console
$ gh qwt get cli/cli
~/qwt/cli/cli/trunk
```

#### Errors and exit codes

- Invalid repo spec or invalid flags: exit `2`.
- Clone failure, default-branch detection failure, existing incompatible destination, fetch failure, or worktree creation failure: exit `1`.

### `add`

#### Preconditions

- A branch name MUST be provided.
- A qwt-managed repository MUST be discoverable from the current directory unless `--repo <owner>/<repo>` is provided.
- The destination worktree path MUST NOT already exist.

#### Steps and effects

1. Resolve the target repository by discovery or `--repo`.
2. Build the destination path `<qwt_root>/<owner>/<repo>/<branch>`.
3. Detect branch path prefix collisions.
4. Determine whether `origin/<branch>` exists.
5. If the remote branch exists, run the equivalent of:

   ```console
   $ git worktree add --track -b fix/parser ~/qwt/cli/cli/fix/parser origin/fix/parser
   ```

6. If the remote branch does not exist, choose the base ref from `--from <ref>` or the default branch, then run the equivalent of:

   ```console
   $ git worktree add -b fix/parser ~/qwt/cli/cli/fix/parser <base>
   ```

7. Slash branch names MUST create nested directories.

#### Output

On success, `add` MUST print the new worktree path:

```console
$ gh qwt add fix/parser
~/qwt/cli/cli/fix/parser
```

#### Errors and exit codes

- Missing branch, malformed `--repo`, or invalid flags: exit `2`.
- Repository discovery failure, existing worktree, branch already checked out elsewhere, prefix collision, invalid base ref, or git failure: exit `1`.

### `list`

#### Preconditions

- No positional arguments MUST be required.
- The qwt root MAY be empty or missing.

#### Steps and effects

1. Resolve the qwt root.
2. Iterate candidate repository directories under `<qwt_root>/<owner>/<repo>`.
3. For each qwt-managed repository, run `git worktree list` or equivalent.
4. Do not modify the filesystem.

#### Output

Default output SHOULD list repo names and branch-relative worktrees:

```console
$ gh qwt list
cli/cli
  trunk
  fix/parser
```

With `--full-path`, output SHOULD include full worktree paths:

```console
$ gh qwt list --full-path
cli/cli
  ~/qwt/cli/cli/trunk
  ~/qwt/cli/cli/fix/parser
```

#### Errors and exit codes

- Invalid flags: exit `2`.
- Failure to inspect a qwt-managed repository: exit `1` unless the implementation explicitly treats unreadable entries as skippable warnings.

### `rm`

#### Preconditions

- A branch name MUST be provided.
- A qwt-managed repository MUST be discoverable from the current directory.
- The target worktree MUST exist.

#### Steps and effects

1. Discover the repository directory.
2. Build the target worktree path for the branch.
3. Run `git worktree remove <path>`.
4. If `--force` is provided, pass the appropriate git force option.
5. If `--delete-branch` is provided, delete the local branch with `git branch -D <branch>` after successful worktree removal.

#### Output

`rm` MAY print the removed worktree path or a concise confirmation. It MUST write errors to stderr.

#### Errors and exit codes

- Missing branch or invalid flags: exit `2`.
- Discovery failure, missing worktree, uncommitted changes without `--force`, branch deletion failure, or git failure: exit `1`.

### `root`

#### Preconditions

- No positional arguments are accepted.

#### Steps and effects

1. Resolve the qwt root using the required root resolution algorithm.
2. Do not modify the filesystem.

#### Output

`root` MUST print the resolved qwt root:

```console
$ gh qwt root
~/qwt
```

#### Errors and exit codes

- Positional arguments or invalid flags: exit `2`.
- Home-directory resolution failure or subprocess failure while reading git config: exit `1` only when the failure prevents fallback behavior.

### `path`

#### Preconditions

- Zero or one path argument is accepted.
- The argument, when provided, MUST be interpreted as `owner/repo` or `owner/repo/branch`.

#### Steps and effects

1. Resolve the qwt root.
2. With no argument, select the qwt root.
3. With `owner/repo`, build `<qwt_root>/<owner>/<repo>`.
4. With `owner/repo/branch`, build `<qwt_root>/<owner>/<repo>/<branch>`; branch MAY contain `/`.
5. Do not create files or directories.

#### Output

```console
$ gh qwt path
~/qwt
$ gh qwt path cli/cli
~/qwt/cli/cli
$ gh qwt path cli/cli/fix/parser
~/qwt/cli/cli/fix/parser
```

#### Errors and exit codes

- Malformed arguments or invalid flags: exit `2`.
- Root resolution failure: exit `1`.

### `prune`

#### Preconditions

- A repo argument `owner/repo` MUST be provided.
- The target MUST resolve under the qwt root.

#### Steps and effects

1. Resolve the qwt root.
2. Build `<qwt_root>/<owner>/<repo>`.
3. Verify that the target is qwt-managed by checking `.bare` and the `.git` pointer.
4. Ask for confirmation unless `-y` or `--force` is provided.
5. Remove the entire repository directory, including all worktrees and `.bare`.

#### Output

Without force, `prune` SHOULD prompt:

```console
$ gh qwt prune cli/cli
Remove ~/qwt/cli/cli and all worktrees? [y/N]
```

With force, output MAY be a concise confirmation.

#### Errors and exit codes

- Missing repo, malformed repo, or invalid flags: exit `2`.
- Target not found, target not qwt-managed, confirmation declined, or filesystem removal failure: exit `1`.

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Success. |
| `1` | Runtime error, including git failure, gh failure after fallback, missing repository, missing worktree, not-found condition, or filesystem failure. |
| `2` | Invalid usage or invalid arguments. |

## Edge cases

| Edge case | Required behavior |
| --- | --- |
| Existing destination | Commands MUST fail rather than overwrite unrelated files or directories. |
| Branch already checked out | `add` MUST fail if git reports that the branch is already checked out in another worktree. |
| Worktree already exists | `get` and `add` MUST fail if the target worktree path already exists. |
| Prefix collisions | `add` MUST detect path-prefix conflicts such as existing `fix` versus requested `fix/parser`, and existing `fix/parser` versus requested `fix`. |
| Detached `HEAD` avoidance | `get` and remote-branch `add` MUST create local branches, not detached worktrees. |
| Slash branches | Slash names such as `fix/parser` MUST map to nested directories. |
| Host omission | Paths MUST NOT include `github.com` or any other host segment. |
| Windows paths | The implementation SHOULD use platform path APIs and MUST account for the extension executable being named `gh-qwt.exe` on Windows. |
| `.git` pointer mismatch | Discovery and destructive commands SHOULD reject repositories whose `.git` pointer does not exactly identify the qwt `.bare` directory. |
| Missing `gh` | Default-branch lookup MUST fall back to `git ls-remote --symref` when possible. |

## Related documents

- [Architecture](../architecture/)
- [CLI reference](../../references/cli/)
- [Directory layout reference](../../references/directory-layout/)
- [ADR 0003: Language: Rust precompiled binary](../adr/0003-language-rust-precompiled-binary/)
- [ADR 0004: Bare repo plus per-branch worktree layout](../adr/0004-bare-repo-plus-per-branch-worktree-layout/)
- [ADR 0009: Default branch detection strategy](../adr/0009-default-branch-detection-strategy/)
