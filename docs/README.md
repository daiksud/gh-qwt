# gh-qwt documentation

Documentation for **using** `gh qwt` — a GitHub CLI extension that clones each repository once as a
**bare** git database and gives every branch its own **worktree** directory
(`<qwt_root>/<owner>/<repo>/<branch>`), so you can work on many branches side by side.

**New here?** Start with **[Getting started](guides/getting-started/)**.

## Map

### 📖 Guides — [`guides/`](guides/)
| Page | Read this to… |
| --- | --- |
| [Getting started](guides/getting-started/) | Install `gh qwt`, clone your first repo, and `cd` into a worktree |
| [Working with worktrees](guides/working-with-worktrees/) | Add, list, and remove per-branch worktrees |
| [Configuration](guides/configuration/) | Choose where repositories live (`QWT_ROOT` / `qwt.root`) |
| [Shell integration](guides/shell-integration/) | Jump into worktrees with a `qcd` function and a fuzzy picker |

### 📚 References — [`references/`](references/)
| Page | Contents |
| --- | --- |
| [CLI](references/cli/) | Every command, argument, flag, and exit code |
| [Configuration](references/configuration/) | Config keys and environment variables (with precedence) |
| [Directory layout](references/directory-layout/) | The normative on-disk layout |
| [Glossary](references/glossary/) | Definitions of key terms |

## Contributing

Building `gh-qwt` or contributing changes? See the **[development documentation](development/)** —
architecture, specification, building & releasing, contributing, testing, and ADRs.
