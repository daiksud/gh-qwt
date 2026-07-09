# Getting started with gh-qwt

Use this guide to install `gh qwt`, create your first qwt-managed checkout, enter the default-branch worktree, and verify that the expected bare repository and worktree layout were created under your qwt root.

## Table of contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [First clone](#first-clone)
- [Enter the worktree](#enter-the-worktree)
- [Verify](#verify)
- [Next steps](#next-steps)

## Prerequisites

Before you start, make sure you have:

- [ ] GitHub CLI installed and authenticated.

  ```console
  $ gh auth status
  ```

- [ ] `git` installed and available on your `PATH`.

  ```console
  $ git --version
  ```

## Installation

### Published install

When `gh-qwt` is released, install it as a GitHub CLI extension:

```console
$ gh extension install daiksud/gh-qwt
```

> [!NOTE]
> `gh-qwt` is not yet released. The project is currently in the documentation and specification phase, so for now you should build and install it locally from source.

### Local install from source

For local development, clone the repository, build the release binary, place it at the repository root as `gh-qwt`, and install the current directory as a GitHub CLI extension:

```console
$ git clone https://github.com/daiksud/gh-qwt.git
$ cd gh-qwt
$ cargo build --release
$ cp target/release/gh-qwt ./gh-qwt
$ gh extension install .
```

For the full build and release workflow, see [Building and releasing](../../development/building-and-releasing/).

## First clone

Run `gh qwt get` with an `owner/repo` name. This clones the repository once as a bare repository, creates a worktree for the default branch, and prints the worktree path.

```console
$ gh qwt get cli/cli
~/qwt/cli/cli/trunk
```

With the example qwt root `~/qwt`, repository `cli/cli`, and default branch `trunk`, the resulting layout is:

```text
~/qwt/cli/cli/
  .bare/    # bare git database created by git clone --bare
  .git      # file containing: gitdir: ./.bare
  trunk/    # default-branch worktree created by gh qwt get
```

| Path | What it is |
| --- | --- |
| `.bare/` | The bare repository. `gh-qwt` keeps one shared git database here for the repository. |
| `.git` | A pointer file containing `gitdir: ./.bare`, so git commands can resolve the bare repository from the repo directory. |
| `trunk/` | The default-branch worktree created by `gh qwt get cli/cli`. |

> [!TIP]
> `gh-qwt` uses the path layout `<qwt_root>/<owner>/<repo>/<branch>` without a host segment. For example, a future `fix/parser` worktree for `cli/cli` would live under the same repository directory.

## Enter the worktree

Use `gh qwt path` when you want a path suitable for `cd`:

```console
$ cd "$(gh qwt path cli/cli)"
```

A child process cannot change your parent shell's current working directory, so `gh-qwt` also provides a shell function pattern for convenient navigation:

```bash
qcd() { cd "$(gh qwt path "$1")" || return; }
```

After adding that function to your shell, you can run:

```console
$ qcd cli/cli
```

For setup details, see [Shell integration](../shell-integration/).

## Verify

- [ ] `gh qwt get cli/cli` printed `~/qwt/cli/cli/trunk`.
- [ ] `~/qwt/cli/cli/.bare/` exists and is the shared bare repository.
- [ ] `~/qwt/cli/cli/.git` exists and contains `gitdir: ./.bare`.
- [ ] `~/qwt/cli/cli/trunk/` exists and is the default-branch worktree.
- [ ] `cd "$(gh qwt path cli/cli)"` enters the `trunk` worktree.

## Next steps

- [Working with worktrees](../working-with-worktrees/)
- [Configuration](../configuration/)
- [CLI reference](../../references/cli/)
