#!/usr/bin/env bash
#
# Local smoke test for gh-qwt.
#
# Builds the binary and exercises the full command lifecycle
# (root -> get -> add -> list -> path -> rm -> prune) against a LOCAL source
# repository. It is fully offline: no network and no GitHub authentication are
# required, and it never touches your real qwt root or gh extension state
# (everything runs in an isolated temporary directory that is cleaned up on
# exit).
#
# Usage:
#   script/smoke-test.sh
#
# Exits non-zero if any check fails.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

if [ -t 1 ]; then
  GREEN=$'\033[32m'; RED=$'\033[31m'; BOLD=$'\033[1m'; DIM=$'\033[2m'; RESET=$'\033[0m'
else
  GREEN=""; RED=""; BOLD=""; DIM=""; RESET=""
fi

pass=0
fail=0
ok()   { printf '  %s✓%s %s\n' "$GREEN" "$RESET" "$1"; pass=$((pass + 1)); }
bad()  { printf '  %s✗%s %s\n' "$RED" "$RESET" "$1"; fail=$((fail + 1)); }
step() { printf '\n%s== %s ==%s\n' "$BOLD" "$1" "$RESET"; }

# assert_eq <actual> <expected> <description>
assert_eq() {
  if [ "$1" = "$2" ]; then ok "$3"; else bad "$3 ${DIM}(got '$1', want '$2')${RESET}"; fi
}
# assert_dir <path> ; assert_gone <path>
assert_dir()  { if [ -d "$1" ]; then ok "exists: ${1#"$WORK"/}"; else bad "missing: ${1#"$WORK"/}"; fi; }
assert_gone() { if [ ! -e "$1" ]; then ok "removed: ${1#"$WORK"/}"; else bad "still present: ${1#"$WORK"/}"; fi; }
# assert_exit <expected-code> <description> <command...>
assert_exit() {
  local want=$1 desc=$2; shift 2
  local got=0
  if "$@" >/dev/null 2>&1; then got=0; else got=$?; fi
  assert_eq "$got" "$want" "$desc"
}

step "Build"
cargo build --quiet
BIN="$REPO_ROOT/target/debug/gh-qwt"
# cargo appends .exe on Windows.
[ -x "$BIN" ] || BIN="$BIN.exe"
[ -x "$BIN" ] || { echo "build failed: gh-qwt binary not found under target/debug"; exit 1; }
echo "binary: ${BIN#"$REPO_ROOT"/}"
qwt() { "$BIN" "$@"; }

# Normalize a path so the (possibly native-Windows) gh-qwt binary and this shell
# agree on it. On Git Bash / MSYS (Windows), `cygpath -m` yields a mixed path
# (e.g. C:/Users/foo) understood by both the native binary and MSYS tools;
# elsewhere the path passes through unchanged.
to_native() {
  if command -v cygpath >/dev/null 2>&1; then cygpath -m "$1"; else printf '%s' "$1"; fi
}

# Isolated workspace, cleaned up on exit.
WORK="$(mktemp -d)"
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT

# Use a path form that both the shell and the native binary understand.
WORK="$(to_native "$WORK")"

# A local source repository so the test needs no network or gh auth.
# owner/repo are derived from the path tail -> acme/widget.
SRC="$WORK/acme/widget"
mkdir -p "$SRC"
git -C "$SRC" init -q -b main
git -C "$SRC" \
  -c user.email=dev@example.com -c user.name=dev -c commit.gpgsign=false \
  commit -q --allow-empty -m "init"
git -C "$SRC" branch feature/login
# Build a file:// URL with a leading slash before the path so both POSIX
# (/abs -> file:///abs) and Windows mixed (C:/x -> file:///C:/x) forms are valid.
case "$SRC" in
  /*) SRC_URL="file://$SRC" ;;
  *) SRC_URL="file:///$SRC" ;;
esac

export QWT_ROOT="$WORK/qwt"
REPO="$QWT_ROOT/acme/widget"

step "root"
assert_eq "$(qwt root)" "$QWT_ROOT" "root prints the resolved qwt root"

step "get (bare clone + default-branch worktree, default detected)"
# No -b: default-branch detection falls back to 'git ls-remote --symref', which
# works offline against a file:// source.
qwt get "$SRC_URL" >/dev/null
assert_dir "$REPO/.bare"
assert_eq "$(cat "$REPO/.git")" "gitdir: ./.bare" ".git pointer contents"
assert_dir "$REPO/main"
assert_eq "$(git -C "$REPO/main" branch --show-current)" "main" "default worktree is on branch main"

step "get error handling"
assert_exit 1 "second get into an existing repo fails" qwt get "$SRC_URL"
assert_exit 2 "invalid repo spec exits 2" qwt get "a/b/c"

step "add (existing branch, new branch, collision)"
qwt add --repo acme/widget feature/login >/dev/null
assert_dir "$REPO/feature/login"
assert_eq "$(git -C "$REPO/feature/login" branch --show-current)" "feature/login" "existing branch attached"
( cd "$REPO/main" && qwt add topic/demo >/dev/null )
assert_dir "$REPO/topic/demo"
assert_eq "$(git -C "$REPO/topic/demo" branch --show-current)" "topic/demo" "new branch created via discovery"
assert_exit 1 "prefix collision (topic vs topic/demo) is rejected" qwt add --repo acme/widget topic

step "list / path"
echo "${DIM}-- gh qwt list --${RESET}"
qwt list
assert_eq "$(qwt path acme/widget/feature/login)" "$REPO/feature/login" "path owner/repo/branch"
assert_eq "$(qwt path acme/widget)" "$REPO" "path owner/repo"
assert_exit 2 "malformed path argument exits 2" qwt path solo

step "rm (with --delete-branch)"
( cd "$REPO/main" && qwt rm --delete-branch feature/login >/dev/null )
assert_gone "$REPO/feature/login"
assert_eq "$(git -C "$REPO" branch --list feature/login)" "" "local branch feature/login deleted"
assert_exit 1 "removing a non-existent worktree fails" bash -c "cd '$REPO/main' && '$BIN' rm nope"

step "prune"
qwt prune -y acme/widget >/dev/null
assert_gone "$REPO"
assert_exit 1 "pruning a non-existent repo fails" qwt prune -y no/such

printf '\n%s%d passed, %d failed%s\n' "$BOLD" "$pass" "$fail" "$RESET"
if [ "$fail" -ne 0 ]; then
  printf '%sSmoke test FAILED.%s\n' "$RED" "$RESET"
  exit 1
fi
printf '%sSmoke test passed.%s\n' "$GREEN" "$RESET"
