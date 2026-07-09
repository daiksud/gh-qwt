# gh-qwt

> ghq, reimagined around **git worktrees** — packaged as a GitHub CLI extension.

`gh qwt` clones a repository **once** as a bare git database and gives every branch its own
**worktree directory**, so you can work on many branches side by side without stashing,
re-cloning, or thrashing a single working tree.

> [!NOTE]
> **Built for GitHub.** Unlike [ghq](https://github.com/x-motemen/ghq), which manages repositories
> across many hosts and version-control systems, `gh-qwt` targets **GitHub exclusively**: it ships as
> a [GitHub CLI](https://cli.github.com/) (`gh`) extension and relies on `gh` for authentication and
> GitHub API access.

```console
$ gh qwt get cli/cli
~/qwt/cli/cli/trunk          # worktree for the default branch

$ gh qwt add fix/parser
~/qwt/cli/cli/fix/parser     # worktree for a new branch
```

## Installation

Install it as a GitHub CLI extension:

```console
$ gh extension install daiksud/gh-qwt
```

Building from source instead? See [Getting started](docs/guides/getting-started/).

## Layout at a glance

```text
~/qwt/<owner>/<repo>/
├── .bare/               # the bare git database (git clone --bare)
├── .git                 # a FILE containing: gitdir: ./.bare
├── <default_branch>/    # worktree created by `gh qwt get`
└── <feature_branch>/    # worktree created by `gh qwt add`
```

See the normative [directory layout reference](docs/references/directory-layout/) for details.

## Documentation

| Section | What's inside |
| --- | --- |
| 📖 [Guides](docs/guides/) | Task-oriented walkthroughs: install, worktrees, configuration, shell |
| 📚 [References](docs/references/) | CLI, configuration, directory layout, glossary |
| 📝 [Changelog](CHANGELOG.md) | Release history and notable changes |

**New here?** Start with **[Getting started](docs/guides/getting-started/)**.

## Commands (v1)

| Command | Purpose |
| --- | --- |
| `gh qwt get <owner>/<repo>` | Clone (bare) and create the default-branch worktree |
| `gh qwt add <branch>` | Create a worktree for a new or existing branch |
| `gh qwt list` | List repositories and their worktrees |
| `gh qwt rm <branch>` | Remove a worktree |
| `gh qwt root` | Print the resolved qwt root |
| `gh qwt path [<spec>]` | Print a worktree path (for `cd`) |
| `gh qwt prune <owner>/<repo>` | Remove an entire repository tree |

Full details in the [CLI reference](docs/references/cli/).

## Contributing

Building `gh-qwt` or contributing changes? See the **[development documentation](docs/development/)**.

## License

[MIT](LICENSE)
