# Glossary

Alphabetical definitions for terms used in the `gh-qwt` documentation.

## Table of contents

- [Bare repository](#bare-repository)
- [Default branch](#default-branch)
- [Detached HEAD](#detached-head)
- [ghq](#ghq)
- [Git worktree](#git-worktree)
- [qwt root](#qwt-root)
- [Remote-tracking branch](#remote-tracking-branch)
- [Repo spec](#repo-spec)
- [Worktree directory](#worktree-directory)

### Bare repository

A Git repository that stores the Git database without a checked-out working tree. `gh qwt` stores one bare repository at `.bare/` for each managed repo; see the [directory layout reference](../directory-layout/).

### Default branch

The repository's primary branch as reported by GitHub, such as `trunk` for the examples in these docs. `gh qwt get` creates a worktree for the default branch unless another branch is selected.

### Detached HEAD

A Git state where `HEAD` points directly at a commit instead of a branch name. qwt worktrees are intended to check out real local branches, not detached `HEAD` states.

### ghq

A separate repository-management tool with its own `GHQ_ROOT` environment variable and `ghq.root` git config. qwt configuration is independent from ghq; see the [configuration reference](../configuration/).

### Git worktree

A Git checkout linked to a shared repository database. `gh qwt` uses git worktrees so each branch gets its own directory while sharing the same `.bare/` database.

### qwt root

The top-level directory where `gh qwt` stores owner/repo directories. It resolves from `QWT_ROOT`, then `qwt.root`, then the default `~/qwt`; see the [configuration reference](../configuration/).

### Remote-tracking branch

A local reference that records the state of a branch on a remote, such as `origin/trunk`. `gh qwt get` configures the fetch refspec and fetches so remote-tracking branches are populated in the bare repository.

### Repo spec

A command argument that identifies a GitHub repository. `gh qwt` accepts forms such as `owner/repo`, HTTPS URLs, and SSH URLs; see the [CLI reference](../cli/#arguments-and-repo-specs).

### Worktree directory

The on-disk directory containing files checked out for one branch, such as `~/qwt/cli/cli/trunk` or `~/qwt/cli/cli/fix/parser`. Slash-separated branch names become nested directories under the repository directory.
