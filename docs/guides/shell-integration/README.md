---
type: guide
title: "Shell integration"
description: "Jump into worktrees with a qcd shell function and an optional fuzzy picker."
resource: gh-qwt
tags: [gh-qwt, guide, shell-integration]
timestamp: 2026-07-09
---

# Shell integration

Use shell helpers to make `cd`-ing into `gh qwt` worktrees ergonomic.

## Contents

- [Why a shell function is needed](#why-a-shell-function-is-needed)
- [Add `qcd` for bash or zsh](#add-qcd-for-bash-or-zsh)
- [Add `qcd` for fish](#add-qcd-for-fish)
- [Add a fuzzy worktree picker](#add-a-fuzzy-worktree-picker)
- [See also](#see-also)

## Why a shell function is needed

`gh qwt path <owner>/<repo>[/<branch>]` prints an absolute path that a shell can use with `cd`.
For example, with qwt root `~/qwt`, `cli/cli` on the default branch `trunk` resolves to a path
like `~/qwt/cli/cli/trunk`.

> [!IMPORTANT]
> A command such as `gh qwt` runs as a subprocess. A subprocess cannot change the current working
> directory of its parent shell, so `gh qwt` cannot `cd` for you directly. Use a shell function
> that calls `gh qwt path` and then runs `cd` in your current shell.

## Add `qcd` for bash or zsh

Add this function to your shell startup file:

```bash
qcd() { cd "$(gh qwt path "$1")" || return; }
```

Put it in one of these files:

- [ ] Bash: `~/.bashrc`
- [ ] Zsh: `~/.zshrc`

Then open a new shell, or source the file you edited.

Usage:

```console
$ qcd cli/cli
$ pwd
/Users/you/qwt/cli/cli/trunk

$ qcd cli/cli/fix/parser
$ pwd
/Users/you/qwt/cli/cli/fix/parser
```

`qcd cli/cli` enters the default branch worktree, such as `trunk`. `qcd cli/cli/fix/parser`
enters the `fix/parser` branch worktree.

## Add `qcd` for fish

For fish, define the function with fish syntax:

```fish
function qcd; cd (gh qwt path $argv[1]); end
```

To make it persistent, add it to your fish configuration, such as `~/.config/fish/config.fish`.

## Add a fuzzy worktree picker

If you have many worktrees, combine `gh qwt list -p` with a fuzzy finder:

```bash
qcdf() { local d; d=$(gh qwt list -p | fzf) && cd "$d"; }
```

Usage:

```console
$ qcdf
```

The picker receives paths from `gh qwt list -p`, lets you choose one, and changes into the
selected worktree.

> [!NOTE]
> `fzf` and `peco` are optional external tools. They are not built into `gh qwt`; install one
> separately if you want fuzzy selection.

## See also

- [Configuration](../configuration/) for setting the qwt root used by `gh qwt path`.
- [CLI reference](../../references/cli/) for command details.
