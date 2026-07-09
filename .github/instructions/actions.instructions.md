---
applyTo: ".github/workflows/*.yml,.github/workflows/*.yaml"
---

# GitHub Actions version pinning

Rules for how `uses:` refs in workflow files must be pinned, following GitHub's
[secure use guidance for third-party actions](https://docs.github.com/en/actions/reference/security/secure-use#using-third-party-actions).
Keep `docs/development/building-and-releasing/README.md` in sync with any change made here.

## Rule

1. Determine whether the action's owner is **GitHub official or Marketplace-Verified**:
   - Anything under the [`actions`](https://github.com/actions) organization (`actions/checkout`,
     `actions/upload-artifact`, `actions/download-artifact`, etc.) is GitHub official.
   - Otherwise, check the action's Marketplace listing page for a "Verified creator" badge, or query
     the Marketplace API, e.g.:
     `curl -s https://github.com/marketplace/actions/<slug> | grep -o '"isVerifiedOwner":[a-z]*'`
2. **Official or Verified** → pin with the **major version tag** only: `owner/repo@vN`.
   - Do not use minor/patch tags, branch names, or `@main`/`@master`.
3. **Everything else (untrusted/unverified third-party actions)** → pin to the **full-length commit
   SHA**, with the human-readable version as a trailing comment: `owner/repo@<40-char-sha> # vX.Y.Z`.
   - A tag or branch can be repointed to different code after the fact; a commit SHA cannot.
   - Resolve a release tag to its commit SHA with:
     `gh api repos/<owner>/<repo>/commits/<tag> --jq .sha`
4. **Special case — `dtolnay/rust-toolchain`**: this action has no `vX.Y.Z` release tags; the ref
   itself selects the Rust toolchain to install (`@stable`, `@nightly`, `@1.89.0`, ...). Pin to the
   commit SHA at the tip of the desired branch (e.g. `stable`), commented with that branch/ref name
   instead of a version, e.g. `dtolnay/rust-toolchain@<sha> # stable`. This keeps installing the
   latest matching Rust toolchain while fixing the action code that runs.

## When updating a pinned action

- Re-resolve the tag/branch to its current commit SHA and update both the workflow YAML and
  `docs/development/building-and-releasing/README.md` (the embedded workflow example and any
  version references) together.
- Re-verify official/Verified status before switching an action from SHA-pinning to a major-version
  tag, or vice versa.

## Current pins (for reference)

| Action | Owner status | Pin |
| --- | --- | --- |
| `actions/checkout` | Official | `@v7` |
| `actions/upload-artifact` | Official | `@v7` |
| `actions/download-artifact` | Official | `@v8` |
| `dtolnay/rust-toolchain` | Unverified | `@<sha> # stable` |
| `Swatinem/rust-cache` | Unverified | `@<sha> # v2.9.1` |
