# Configuration

Use this guide to choose where `gh qwt` stores cloned repositories and their worktrees.

## Contents

- [Understand the qwt root](#understand-the-qwt-root)
- [Resolution precedence](#resolution-precedence)
- [Set the root with an environment variable](#set-the-root-with-an-environment-variable)
- [Set the root with git config](#set-the-root-with-git-config)
- [Verify the resolved root](#verify-the-resolved-root)
- [See also](#see-also)

## Understand the qwt root

The **qwt root** is the top-level directory where `gh qwt` stores its bare git databases and
per-branch worktrees. With the default root `~/qwt`, the `cli/cli` repository and its `trunk`
worktree use paths like:

```console
~/qwt/cli/cli/trunk
```

Change the qwt root when you want worktrees under another directory, such as `~/src`, or when
you want all `gh qwt` repositories on a larger or faster disk.

> [!TIP]
> Choose a stable root that is easy to type and back up. Avoid putting it inside an existing
> repository, because `gh qwt` creates its own owner/repository/branch directory layout under it.

> [!NOTE]
> The qwt root is independent from ghq's `GHQ_ROOT` environment variable and `ghq.root` git
> configuration. Set `QWT_ROOT` or `qwt.root` separately for `gh qwt`.

## Resolution precedence

`gh qwt` resolves the qwt root in this order, from highest priority to lowest:

1. Environment variable `QWT_ROOT`.
2. `git config --get qwt.root`, typically set globally.
3. Default `~/qwt`.

| Source | How to set | Example | Priority |
| --- | --- | --- | --- |
| Environment variable `QWT_ROOT` | Export it in your shell or shell startup file | `export QWT_ROOT="$HOME/src"` | 1, highest |
| Git config `qwt.root` | Set it with `git config`, usually using `--global` | `git config --global qwt.root "$HOME/src"` | 2 |
| Default | No setup required | `~/qwt` | 3, lowest |

`~` is expanded when `gh qwt` resolves the root.

## Set the root with an environment variable

Use `QWT_ROOT` when you want the shell environment to override other configuration.

```bash
export QWT_ROOT="$HOME/src"
```

To make the setting persistent, add it to your shell startup file:

- [ ] Bash: add it to `~/.bashrc`.
- [ ] Zsh: add it to `~/.zshrc`.

```bash
# ~/.bashrc or ~/.zshrc
export QWT_ROOT="$HOME/src"
```

Open a new shell, or source the file you edited, before verifying the result.

## Set the root with git config

Use git config when you want a persistent default that does not depend on exporting an
environment variable.

```bash
git config --global qwt.root "$HOME/src"
```

Because `QWT_ROOT` has higher priority, unset it if you want `qwt.root` to take effect:

```bash
unset QWT_ROOT
```

## Verify the resolved root

Run:

```console
$ gh qwt root
/Users/you/src
```

`gh qwt root` prints the absolute resolved root after applying the precedence rules above.

## See also

- [Configuration reference](../../references/configuration/) for exhaustive keys and precedence.
- [Shell integration](../shell-integration/) for `cd` helpers that use `gh qwt path`.
