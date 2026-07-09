# gh-qwt

> ghq, reimagined around **git worktrees** — packaged as a GitHub CLI extension.

`gh qwt` clones a repository **once** as a bare git database and gives every branch its own
**worktree directory**, so you can work on many branches side by side without stashing,
re-cloning, or thrashing a single working tree.

```console
$ gh qwt get cli/cli
~/qwt/cli/cli/trunk          # worktree for the default branch

$ gh qwt add fix/parser
~/qwt/cli/cli/fix/parser     # worktree for a new branch
```

> [!NOTE]
> **Project status — documentation & specification phase.**
> This repository currently contains the design docs, specification, and ADRs under
> [`docs/`](docs/). The Rust implementation follows in a later step, built to match these docs.

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
| 🛠️ [Development](docs/development/) | Architecture, specification, building & releasing, testing, ADRs |

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

## License

To be decided.
