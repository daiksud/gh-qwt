---
type: index
title: "Development"
description: "Developer and maintainer documentation for gh-qwt: architecture, specification, building, contributing, testing, and ADRs."
resource: gh-qwt
tags: [gh-qwt, development, index]
timestamp: 2026-07-09
---

# Development

Documentation for contributors and maintainers: how `gh-qwt` is designed, specified, built,
and released.

| Page | Contents |
| --- | --- |
| [Architecture](architecture/) | Module map, responsibilities, and data flow |
| [Specification](specification/) | Normative per-command behavior (the contract the code must meet) |
| [Building & releasing](building-and-releasing/) | Local builds, `gh extension install`, cross-compilation, release automation |
| [Contributing](contributing/) | Dev environment, code style, PR and commit conventions |
| [Testing](testing/) | Unit tests and offline integration tests |
| [ADRs](adr/) | Architecture Decision Records — the "why" behind key choices |

> [!IMPORTANT]
> The [specification](specification/) and [ADRs](adr/) are the source of truth for the
> implementation. When behavior and code disagree, treat the spec as authoritative and fix the code
> (or amend the spec via a new ADR).
