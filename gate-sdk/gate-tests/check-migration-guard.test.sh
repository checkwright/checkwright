#!/usr/bin/env bash
# Behavioral test of check-migration-guard's git arm, which the good/+bad/ pair cannot reach: the
# pair stands a directory in for the merge base, since a committed fixture is identical to HEAD.
# Each case runs in a sandbox repository whose history the case builds.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
cases=0

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    cases=$((cases + 1))
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

LIST="$SANDBOX/destructive.list"
printf '# the template floor\nDROP (TABLE|INDEX|VIEW|TRIGGER)\nALTER TABLE .* DROP\nALTER TABLE .* RENAME\n' > "$LIST"

# --- the knob empty: clean, not applicable, before any repository is needed.
mkdir -p "$SANDBOX/plain"
out="$( cd "$SANDBOX/plain" && gate_run check-migration-guard "$CHECKS" 2>&1 )"; rc=$?
expect not-applicable 0 'not applicable: no migration directory configured' "$rc" "$out"

# A fresh repository on main with two merged migrations, a topic branch checked out over it.
fresh() {  # fresh <name> — prints the repository path
    local r="$SANDBOX/$1"
    mkdir -p "$r/db/migrations"
    git -C "$r" init -q -b main
    printf 'CREATE TABLE a (id INTEGER);\n' > "$r/db/migrations/0001_a.sql"
    printf 'CREATE TABLE b (id INTEGER);\n' > "$r/db/migrations/0002_b.sql"
    git -C "$r" add db
    git -C "$r" -c user.name=t -c user.email=t@e.invalid commit -qm base
    git -C "$r" checkout -qb topic
    printf '%s' "$r"
}
guard() {  # guard <repo> [gate args…] — the gate in <repo> over db/migrations
    local r="$1"
    shift
    ( cd "$r" && gate_env GATE_SDK_MIGRATION_DIR=db/migrations GATE_SDK_MIGRATION_DESTRUCTIVE="$LIST" \
        && gate_run check-migration-guard "$CHECKS" "$@" 2>&1 )
}

# --- a tree whose base is its own HEAD, then an uncommitted addition counted from a subdirectory.
r="$(fresh own-head)"
out="$(guard "$r" --base HEAD)"; rc=$?
expect own-head 0 'clean (2 migration(s), 0 added, against merge base' "$rc" "$out"
printf 'CREATE TABLE c (id INTEGER);\n' > "$r/db/migrations/0003_c.sql"
out="$(guard "$r")"; rc=$?
expect untracked-addition 0 'clean (3 migration(s), 1 added, against merge base' "$rc" "$out"
out="$( cd "$r/db" && gate_env GATE_SDK_MIGRATION_DIR=migrations GATE_SDK_MIGRATION_DESTRUCTIVE="$LIST" \
    && gate_run check-migration-guard "$CHECKS" 2>&1 )"; rc=$?
expect from-subdirectory 0 'clean (3 migration(s), 1 added, against merge base' "$rc" "$out"

# --- an edit, uncommitted and then committed on the branch.
r="$(fresh edit)"
printf 'CREATE TABLE a (id INTEGER, x TEXT);\n' > "$r/db/migrations/0001_a.sql"
out="$(guard "$r")"; rc=$?
expect edit-uncommitted 1 'db/migrations/0001_a.sql: [MIGRATION-EDITED]' "$rc" "$out"
git -C "$r" -c user.name=t -c user.email=t@e.invalid commit -qam edit
out="$(guard "$r")"; rc=$?
expect edit-committed 1 'db/migrations/0001_a.sql: [MIGRATION-EDITED]' "$rc" "$out"

# --- a delete and a rename.
r="$(fresh delete)"
git -C "$r" rm -q db/migrations/0002_b.sql
out="$(guard "$r")"; rc=$?
expect delete 1 'db/migrations/0002_b.sql: [MIGRATION-EDITED]' "$rc" "$out"
r="$(fresh rename)"
git -C "$r" mv db/migrations/0002_b.sql db/migrations/0002_bee.sql
out="$(guard "$r")"; rc=$?
expect rename-gone 1 'db/migrations/0002_b.sql: [MIGRATION-EDITED]' "$rc" "$out"
expect rename-order 1 'db/migrations/0002_bee.sql: [MIGRATION-ORDER] 0002 <= 0002' "$rc" "$out"

# --- a misnumbered addition.
r="$(fresh order)"
printf 'CREATE TABLE z (id INTEGER);\n' > "$r/db/migrations/0001_z.sql"
out="$(guard "$r")"; rc=$?
expect misnumbered 1 'db/migrations/0001_z.sql: [MIGRATION-ORDER] 0001 <= 0002' "$rc" "$out"

# --- a destructive addition without a reason, then with one; the list absent turns it off.
r="$(fresh destructive)"
printf 'drop table a;\n' > "$r/db/migrations/0003_drop.sql"
out="$(guard "$r")"; rc=$?
expect destructive 1 "db/migrations/0003_drop.sql:1: [MIGRATION-DESTRUCTIVE] matches 'DROP (TABLE|INDEX|VIEW|TRIGGER)'" "$rc" "$out"
printf 'drop table a; -- migration-ok: a is folded into b\n' > "$r/db/migrations/0003_drop.sql"
out="$(guard "$r")"; rc=$?
expect declared 0 'clean (3 migration(s), 1 added, against merge base' "$rc" "$out"
printf 'drop table a; -- migration-ok:\n' > "$r/db/migrations/0003_drop.sql"
out="$(guard "$r")"; rc=$?
expect empty-reason 1 'its waiver reason empty' "$rc" "$out"
out="$( cd "$r" && gate_env GATE_SDK_MIGRATION_DIR=db/migrations GATE_SDK_MIGRATION_DESTRUCTIVE=absent.list \
    && gate_run check-migration-guard "$CHECKS" 2>&1 )"; rc=$?
expect list-absent 0 'destructive check off: no list at absent.list' "$rc" "$out"

# --- a stray file is 3; an unresolved base, a malformed list and a bad argument are 2.
r="$(fresh refusals)"
printf 'notes\n' > "$r/db/migrations/README.md"
out="$(guard "$r")"; rc=$?
expect stray 3 'db/migrations/README.md: not a migration file' "$rc" "$out"
rm "$r/db/migrations/README.md"
out="$(guard "$r" --base nope)"; rc=$?
expect base-unresolved 2 "base 'nope' names no commit" "$rc" "$out"
out="$( cd "$r" && gate_env GATE_SDK_MIGRATION_DIR=db/migrations GATE_SDK_BASE_REF=trunk \
    GATE_SDK_MIGRATION_DESTRUCTIVE="$LIST" && gate_run check-migration-guard "$CHECKS" 2>&1 )"; rc=$?
expect base-knob 2 "base 'trunk' names no commit" "$rc" "$out"
printf 'DROP (TABLE\n' > "$SANDBOX/bad.list"
out="$( cd "$r" && gate_env GATE_SDK_MIGRATION_DIR=db/migrations GATE_SDK_MIGRATION_DESTRUCTIVE="$SANDBOX/bad.list" \
    && gate_run check-migration-guard "$CHECKS" 2>&1 )"; rc=$?
expect list-malformed 2 'bad.list:1:' "$rc" "$out"
out="$(guard "$r" --base main --base-dir db)"; rc=$?
expect base-and-dir 2 'takes no --base' "$rc" "$out"
out="$(guard "$r" --since main)"; rc=$?
expect usage 2 "unknown argument '--since'" "$rc" "$out"

# --- outside a work tree there is no corpus, which is refused, never clean.
out="$( cd "$SANDBOX/plain" && gate_env GATE_SDK_MIGRATION_DIR=db/migrations \
    && gate_run check-migration-guard "$CHECKS" 2>&1 )"; rc=$?
expect no-work-tree 2 'not a git repository' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-migration-guard.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-migration-guard.test.sh: clean (the git arm's edit, delete, rename, order and destructive cases, and its exit classes — $cases cases)"
exit 0
