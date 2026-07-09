# gh-qwt documentation

Welcome to the `gh-qwt` documentation. This site is organized in the
[Diátaxis](https://diataxis.fr/) spirit: **guides** get you productive, **references** give you
exact detail, and **development** docs explain how the tool is built and why.

> [!IMPORTANT]
> **Documentation conventions**
> - **Every page is a directory containing a `README.md`** (`<page-name>/README.md`). Section
>   landing pages and every ADR follow the same rule.
> - **Assets live beside their page** in `<page-name>/assets/` (e.g. `assets/layout.svg`).
>   Diagrams are authored as inline [`mermaid`](https://github.blog/2022-02-14-include-diagrams-markdown-files-mermaid/)
>   where possible, and as **SVG** files under `assets/` when an image is needed.
> - Documents use the full **GitHub Flavored Markdown** toolkit (alerts, tables, task lists,
>   collapsible `<details>`, footnotes).

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

### 🛠️ Development — [`development/`](development/)
| Page | Contents |
| --- | --- |
| [Architecture](development/architecture/) | Module map and data flow |
| [Specification](development/specification/) | Normative per-command behavior |
| [Building & releasing](development/building-and-releasing/) | Build, install, cross-compile, release |
| [Contributing](development/contributing/) | Dev setup and conventions |
| [Testing](development/testing/) | Unit and offline integration testing strategy |
| [ADRs](development/adr/) | Architecture Decision Records |

## What is gh-qwt?

`gh qwt` is a [GitHub CLI extension](https://docs.github.com/en/github-cli/github-cli/using-github-cli-extensions)
that combines an [ghq](https://github.com/x-motemen/ghq)-style "clone into a predictable path"
workflow with **git worktrees**. Each repository is cloned once as a bare database; each branch
gets its own directory. See [ADR-0004](development/adr/0004-bare-repo-plus-per-branch-worktree-layout/)
for the reasoning.
