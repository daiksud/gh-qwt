---
type: adr
title: "ADR-0012: Flat, query-filterable `list` output modeled on `ghq list`"
description: "Replace list's grouped, indented output with a flat, sorted, query-filterable list modeled on ghq list."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0012]
timestamp: 2026-07-09
---

# ADR-0012: Flat, query-filterable `list` output modeled on `ghq list`

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0005](../0005-path-layout-without-host-segment/), [ADR-0007](../0007-command-set-v1/), [ADR-0008](../0008-shell-integration-for-cd/)

## Context

`gh qwt list` printed a grouped, indented tree: a repository header line (`owner/repo`) followed
by indented branch lines, optionally as full paths with `--full-path`. That shape is convenient to
read but unsafe to pipe into a fuzzy finder such as `fzf`: selecting a header line yields text that
is not a path at all, and selecting a branch line yields a value with leading whitespace that
breaks `cd` even after resolving it through `gh qwt path`. The documented "fuzzy worktree picker"
recipe in the shell integration guide (`gh qwt list -p | fzf`) was affected by exactly this
problem.

ADR-0007 already anticipated this kind of change: it explicitly says the command set "should feel
familiar to ghq users where that familiarity helps" and that "command behavior can evolve through
later ADRs if v1 usage reveals missing primitives." `ghq list` is the direct precedent here: it
prints a flat, sorted list of `host/user/project` (or absolute paths with `-p`/`--full-path`), with
no grouping, and accepts an optional query argument plus an `-e`/`--exact` flag for exact matching.
That output shape is exactly what a fuzzy finder needs.

`gh-qwt` differs from `ghq` in two structural ways that affect how directly its `list` command can
copy `ghq`'s: `gh-qwt` never includes a host segment in paths (ADR-0005, GitHub-only by design),
and `gh-qwt` adds a branch/worktree level that `ghq` doesn't have at all (`ghq` manages one checkout
per repository, not one per branch).

## Decision

We will change `gh qwt list` to print a flat, sorted list of `owner/repo/branch` (or absolute paths
with `-p`/`--full-path`, unchanged in meaning), one entry per line, with no repository header and
no indentation. This is a breaking change to `list`'s default output.

We will also add `ghq list`'s query filtering, adapted to `gh-qwt`'s one extra path level:

- An optional `<query>` positional argument. Without `--exact`, it is a smartcase substring match
  (case-insensitive unless `<query>` contains an uppercase letter, exactly like `ghq`) against the
  full `owner/repo/branch` string.
- `-e`, `--exact`: `<query>` must exactly equal `branch`, `repo/branch`, or `owner/repo/branch`
  (case-sensitive, no smartcase — mirroring `ghq`'s `Matches()`, which checks membership in the
  same three tail forms: `project`, `user/project`, `host/user/project`). A branch name containing
  `/` (such as `fix/parser`) is treated as one atomic segment for this purpose, not split further.
- `--exact` with no `<query>` is a no-op (lists everything), matching `ghq`'s behavior.
- Filtering always operates on the `owner/repo/branch` spec, even when `--full-path` changes what
  gets printed.

We will not port `ghq list`'s `--unique`, `--vcs`, or `--bare` flags, or its URL-to-query
conversion: `gh-qwt` has a single root (no multi-root deduplication scenario), manages only Git
repositories (no VCS backend to select), always stores a bare repository (not an optional mode),
and `list`'s query is not intended to accept full repository URLs.

We will keep the `path` command unchanged. It still covers two things the new `list` does not:
printing the bare repository directory for a 2-segment `owner/repo` spec (which is not itself a
worktree `list` reports), and computing a worktree path that does not exist yet. ADR-0008, which
introduced `path`, is not superseded by this decision.

## Consequences

### Positive
- `gh qwt list`'s output can be piped directly into `fzf`, `grep`, `xargs`, and similar tools
  without any text processing to strip headers or indentation.
- The previously-documented `gh qwt list -p | fzf` shell recipe now actually works end to end.
- Users coming from `ghq` get a familiar `list` query/`--exact` vocabulary.

### Negative
- Breaking change to `list`'s default output shape; any existing scripts or muscle memory built
  around the grouped/indented format must adapt.
- Two commands (`list -p` and `path`) can now resolve the same existing worktree to the same
  absolute path, which is some conceptual overlap, accepted because `path` retains non-overlapping
  uses (see Decision).

### Neutral
- Sorting is a simple lexicographic sort of the final printed lines (spec or full path), matching
  `ghq`'s implementation; because `gh-qwt` has a single root, sorting either form produces the same
  relative order.
- Detached-HEAD worktrees are listed using their on-disk relative path instead of a `(detached)`
  annotation, keeping every line a clean, directly reusable `owner/repo/branch` value.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Add an opt-in `--flat` flag, keep the grouped tree as the default | Leaves the broken default behavior in place and adds a second output mode to maintain; the user's own workflow (piping `list` into `fzf`) is the primary reason `list` exists, not a secondary one. |
| Remove `path` entirely | `path` still uniquely resolves a 2-segment `owner/repo` bare-repository directory and prospective (not-yet-created) worktree paths; removing it would regress those use cases and reverse ADR-0008 without a full replacement. |
| Keep grouping but fix indentation/headers to be `cd`-safe | Still not flat/sortable/query-filterable the way `ghq` users expect, and would still require positional bookkeeping (which line is a header vs. a worktree) for any script consuming the output. |

## References

- [`ghq` `cmd_list.go`](https://github.com/x-motemen/ghq/blob/master/cmd_list.go)
- [`ghq` `local_repository.go`](https://github.com/x-motemen/ghq/blob/master/local_repository.go) (`Subpaths`, `Matches`, `NonHostPath`)
- [ADR-0007: v1 command set](../0007-command-set-v1/)
- [ADR-0008: Shell integration for `cd`](../0008-shell-integration-for-cd/)
