---
type: adr
title: "ADR-0008: Provide shell integration for cd via path output"
description: "Provide shell integration for cd via a path command."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0008]
timestamp: 2026-07-09
---

# ADR-0008: Provide shell integration for cd via path output

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0007](../0007-command-set-v1/)

## Context

A common reason to use gh-qwt is to jump into a branch worktree quickly. Users naturally want a command that changes their current directory, but a binary runs as a child process and cannot directly change the current working directory of its parent shell. This is the same universal constraint behind tools that pair path-printing commands with shell functions, such as ghq plus peco or directory jumpers like z.

Shells also differ in function syntax, completion behavior, and evaluation rules. gh-qwt should provide a portable primitive that shells can compose rather than baking one shell’s behavior into the executable.

## Decision

We will provide a `path` command that prints the resolved worktree path. We will document a shell function such as `qcd` that calls `gh qwt path` and then runs `cd` in the parent shell. The gh-qwt binary will not attempt to change the parent shell’s directory itself.

## Consequences

### Positive
- `path` is scriptable and works across shells, editors, and automation.
- Users can define shell-specific `qcd` functions that fit their environment.
- The binary avoids impossible or brittle parent-shell manipulation.

### Negative
- Directory changing requires users to install or copy a shell function.
- The experience may vary slightly across shells until documented integrations mature.

### Neutral
- `path` can also support fuzzy pickers and other navigation tools without depending on them.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Only print paths | This is the core primitive, but documenting `qcd` makes the intended navigation workflow clear. |
| Emit shell-eval snippets | Eval-based integration is more fragile, shell-specific, and harder to reason about safely. |
| Ship a wrapper script | Wrappers add installation complexity and still need shell-specific handling to affect the parent shell. |

## References

- POSIX process model for current working directories
- ghq and picker-based navigation patterns
