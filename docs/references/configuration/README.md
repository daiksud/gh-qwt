# Configuration reference

Exhaustive configuration reference for how `gh qwt` resolves the **qwt root**: the top-level directory that contains qwt-managed repositories and worktrees.

For task-oriented setup steps, see the [configuration guide](../../guides/configuration/).

## Table of contents

- [Settings](#settings)
- [Resolution precedence](#resolution-precedence)
- [Path expansion](#path-expansion)
- [Independence from ghq](#independence-from-ghq)
- [See also](#see-also)

## Settings

| Name | Type | Default | How to set | Notes |
| --- | --- | --- | --- | --- |
| `QWT_ROOT` | Environment variable | Not set | `export QWT_ROOT="$HOME/qwt"` | Highest-priority source. Overrides `qwt.root` when set. |
| `qwt.root` | Git config key | Not set | `git config --global qwt.root "$HOME/qwt"` | Used only when `QWT_ROOT` is not set. Usually configured globally. |

If neither setting is present, `gh qwt` uses `~/qwt`.

## Resolution precedence

`gh qwt` resolves the qwt root in this order, from highest priority to lowest:

1. `QWT_ROOT` environment variable.
2. `git config --get qwt.root`.
3. Default `~/qwt`.

The environment variable always wins over git config, and git config always wins over the default.

| `QWT_ROOT` | `qwt.root` | Resolved qwt root |
| --- | --- | --- |
| `/work/qwt` | `/src/qwt` | `/work/qwt` |
| Not set | `/src/qwt` | `/src/qwt` |
| Not set | Not set | `~/qwt` |

```text
QWT_ROOT set?
├─ yes → use QWT_ROOT
└─ no
   └─ qwt.root set?
      ├─ yes → use qwt.root
      └─ no  → use ~/qwt
```

## Path expansion

A leading `~` is expanded when `gh qwt` resolves the qwt root. For example, with the default root and repo spec `cli/cli`, the repository directory is:

```text
~/qwt/cli/cli
```

Repository paths do not include a host segment.

## Independence from ghq

> [!NOTE]
> The qwt root is independent from ghq's `GHQ_ROOT` environment variable and `ghq.root` git configuration.

Set `QWT_ROOT` or `qwt.root` specifically for `gh qwt`. Existing ghq configuration does not affect where qwt stores repositories.

## See also

- [Configuration guide](../../guides/configuration/) for step-by-step setup.
- [Directory layout reference](../directory-layout/) for the paths created under the qwt root.
- [Glossary](../glossary/) for definitions of qwt terms.
