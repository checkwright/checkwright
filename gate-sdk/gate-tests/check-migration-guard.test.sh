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

refute() {  # refute <label> <substring> <output>
    if grep -qF -- "$2" <<<"$3"; then
        echo "  FAIL [$1]: output holds '$2': $3"; fails=$((fails + 1))
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

# --- the checkout form: a clean filter hiding an edit from git diff reds, an untouched file in an
# autocrlf or an LF clone is clean, and an edit in the autocrlf clone reds.
r="$(fresh filtered)"
git -C "$r" config filter.strip.clean "tr -d '#'"
git -C "$r" config filter.strip.smudge cat
printf '*.sql filter=strip\n' > "$r/.git/info/attributes"
printf 'CREATE TABLE a (id INTEGER);#\n' > "$r/db/migrations/0001_a.sql"
out="$(guard "$r")"; rc=$?
expect filter-hidden-edit 1 'db/migrations/0001_a.sql: [MIGRATION-EDITED]' "$rc" "$out"
refute filter-untouched 'db/migrations/0002_b.sql: [MIGRATION-EDITED]' "$out"
o="$(fresh clone-src)"
git -C "$o" checkout -q main
r="$SANDBOX/crlf"
git clone -q -c core.autocrlf=true "$o" "$r" && git -C "$r" checkout -qb topic
out="$(guard "$r")"; rc=$?
expect autocrlf-untouched 0 'clean (2 migration(s), 0 added, against merge base' "$rc" "$out"
printf 'CREATE TABLE b (id INTEGER, y TEXT);\r\n' > "$r/db/migrations/0002_b.sql"
out="$(guard "$r")"; rc=$?
expect autocrlf-edit 1 'db/migrations/0002_b.sql: [MIGRATION-EDITED]' "$rc" "$out"
r="$SANDBOX/lf"
git clone -q "$o" "$r" && git -C "$r" checkout -qb topic
out="$(guard "$r")"; rc=$?
expect lf-untouched 0 'clean (2 migration(s), 0 added, against merge base' "$rc" "$out"

# --- a conversion attribute changed since the merge base reds; a filter naming no configured
# driver cannot be read in its checkout form, which is 3, never a pass.
r="$(fresh attr-drift)"
printf 'db/migrations/*.sql eol=lf\n' > "$r/.gitattributes"
out="$(guard "$r")"; rc=$?
expect attr-drift 1 'db/migrations/0001_a.sql: [MIGRATION-ATTR] eol unspecified -> lf' "$rc" "$out"
refute attr-drift-edit 'MIGRATION-EDITED' "$out"
r="$(fresh no-driver)"
printf '*.sql filter=nodriver\n' > "$r/.git/info/attributes"
out="$(guard "$r")"; rc=$?
expect no-driver 3 "its filter attribute names driver 'nodriver'" "$rc" "$out"

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

# --- the width is the directory's own: a five-digit history takes a five-digit addition, and a
# four-digit history refuses a five-digit one at 3, naming it.
r="$SANDBOX/wide"
mkdir -p "$r/db/migrations"
git -C "$r" init -q -b main
printf 'CREATE TABLE a (id INTEGER);\n' > "$r/db/migrations/00001_a.sql"
git -C "$r" add db
git -C "$r" -c user.name=t -c user.email=t@e.invalid commit -qm base
git -C "$r" checkout -qb topic
printf 'CREATE TABLE b (id INTEGER);\n' > "$r/db/migrations/00002_b.sql"
out="$(guard "$r")"; rc=$?
expect five-digit-history 0 'clean (2 migration(s), 1 added, against merge base' "$rc" "$out"
r="$(fresh widened)"
printf 'CREATE TABLE c (id INTEGER);\n' > "$r/db/migrations/10003_c.sql"
out="$(guard "$r")"; rc=$?
expect width-mixed 3 'db/migrations/10003_c.sql: its number is 5 digit(s) wide' "$rc" "$out"

# A commit on <branch> adding one migration, the topic branch checked out again after it.
on() {  # on <repo> <branch> <file> <content> — branch made from main when absent
    local r="$1"
    git -C "$r" checkout -q "$2" 2>/dev/null || git -C "$r" checkout -qb "$2" main
    printf '%s\n' "$4" > "$r/db/migrations/$3"
    git -C "$r" add db
    git -C "$r" -c user.name=t -c user.email=t@e.invalid commit -qm "$3"
    git -C "$r" checkout -q topic
}

# --- a clash with an unmerged sibling, and none for one adding the same name with other content.
r="$(fresh clash)"
on "$r" sib 0003_theirs.sql 'CREATE TABLE t (id INTEGER);'
printf 'CREATE TABLE m (id INTEGER);\n' > "$r/db/migrations/0003_mine.sql"
out="$(guard "$r")"; rc=$?
expect clash 1 'db/migrations/0003_mine.sql: [MIGRATION-CLASH] 0003 refs/heads/sib 0003_theirs.sql' "$rc" "$out"
r="$(fresh same-name)"
on "$r" sib 0003_c.sql 'CREATE TABLE theirs (id INTEGER);'
printf 'CREATE TABLE mine (id INTEGER);\n' > "$r/db/migrations/0003_c.sql"
out="$(guard "$r")"; rc=$?
expect same-name 0 'clean (3 migration(s), 1 added, against merge base' "$rc" "$out"
expect same-name-read 0 ', 1 other ref(s) read' "$rc" "$out"

# --- a sibling merged into the base after the branch forked reads as ORDER, never CLASH; a file
# the base took by cherry-pick under its name is the base's, and the branch passes clean.
r="$(fresh merged)"
on "$r" sib 0003_theirs.sql 'CREATE TABLE t (id INTEGER);'
git -C "$r" checkout -q main && git -C "$r" merge -q --ff-only sib && git -C "$r" checkout -q topic
printf 'CREATE TABLE m (id INTEGER);\n' > "$r/db/migrations/0003_mine.sql"
out="$(guard "$r")"; rc=$?
expect merged-after-fork 1 'db/migrations/0003_mine.sql: [MIGRATION-ORDER] 0003 <= 0003' "$rc" "$out"
refute merged-after-fork 'MIGRATION-CLASH' "$out"
r="$(fresh picked)"
on "$r" topic 0003_c.sql 'DROP TABLE a;'
git -C "$r" checkout -q main && git -C "$r" -c user.name=t -c user.email=t@e.invalid cherry-pick topic >/dev/null \
    && git -C "$r" checkout -q topic
out="$(guard "$r")"; rc=$?
expect cherry-picked 0 'clean (3 migration(s), 0 added, against merge base' "$rc" "$out"

# --- the precedence: an addition both a duplicate and a clash prints DUP alone.
r="$(fresh precedence)"
on "$r" sib 0003_q.sql 'CREATE TABLE q (id INTEGER);'
printf 'CREATE TABLE a2 (id INTEGER);\n' > "$r/db/migrations/0003_a.sql"
printf 'CREATE TABLE b2 (id INTEGER);\n' > "$r/db/migrations/0003_b.sql"
out="$(guard "$r")"; rc=$?
expect precedence-dup 1 'db/migrations/0003_b.sql: [MIGRATION-DUP] 0003 also 0003_a.sql' "$rc" "$out"
expect precedence-clash 1 'db/migrations/0003_a.sql: [MIGRATION-CLASH] 0003 refs/heads/sib 0003_q.sql' "$rc" "$out"
refute precedence-one 'db/migrations/0003_b.sql: [MIGRATION-CLASH]' "$out"

# --- the ref set: a detached HEAD excludes no ref, an orphan is skipped, `none` turns the check
# off, a glob narrows it, and a glob opening with '-' is refused at 2.
r="$(fresh detached)"
on "$r" topic 0003_c.sql 'CREATE TABLE c (id INTEGER);'
out="$(guard "$r")"; rc=$?
expect own-ref 0 ', 0 other ref(s) read' "$rc" "$out"
git -C "$r" checkout -q --detach topic
out="$(guard "$r")"; rc=$?
expect detached 0 ', 1 other ref(s) read' "$rc" "$out"
o="$SANDBOX/orphan-src"
mkdir -p "$o/db/migrations"
git -C "$o" init -q -b main
printf 'CREATE TABLE x (id INTEGER);\n' > "$o/db/migrations/0003_x.sql"
git -C "$o" add db
git -C "$o" -c user.name=t -c user.email=t@e.invalid commit -qm lone
r="$(fresh orphan)"
git -C "$r" fetch -q "$o" main:refs/heads/lone
printf 'CREATE TABLE m (id INTEGER);\n' > "$r/db/migrations/0003_mine.sql"
out="$(guard "$r")"; rc=$?
expect orphan 0 'clean (3 migration(s), 1 added, against merge base' "$rc" "$out"
expect orphan-unread 0 ', 0 other ref(s) read' "$rc" "$out"
r="$SANDBOX/clash"
refs() {  # refs <value> — the gate in the clash repository under GATE_SDK_MIGRATION_REFS=<value>
    ( cd "$r" && gate_env GATE_SDK_MIGRATION_DIR=db/migrations GATE_SDK_MIGRATION_DESTRUCTIVE="$LIST" \
        GATE_SDK_MIGRATION_REFS="$1" && gate_run check-migration-guard "$CHECKS" 2>&1 )
}
out="$(refs none)"; rc=$?
expect refs-none 0 '; clash check off: GATE_SDK_MIGRATION_REFS is none' "$rc" "$out"
out="$(refs 'refs/heads/feat/**')"; rc=$?
expect refs-narrowed 0 ', 0 other ref(s) read' "$rc" "$out"
out="$(refs -x)"; rc=$?
expect refs-option 2 "'-x' opens with '-'" "$rc" "$out"

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
echo "check-migration-guard.test.sh: clean (the git arm's edit, checkout-form, attribute-drift, missing-driver, delete, rename, order, width, duplicate, clash, ref-set and destructive cases, and its exit classes — $cases cases)"
exit 0
