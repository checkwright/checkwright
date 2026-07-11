#!/usr/bin/env bash
# Behavioral test of checks/check-skill-binding.sh — the scenarios the one-pair
# good/bad harness cannot hold. The good/bad fixture pair covers the unbound-slot
# case; the harness admits one bad/ dir, so this drives the orphan-binding case,
# the missing-template case, and the two skip cases (a skill with no binding
# directive, and a template with no slots) that must all leave the gate green.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-skill-binding.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=sandbox-dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && "$GATE" commands 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# --- orphan: a binding naming no slot in the template ---
o="$SANDBOX/orphan"
mkdir -p "$o/commands" "$o/templates"
cat >"$o/templates/build.md" <<'EOF'
Exit condition: *<exit-condition: your exit condition>*.
EOF
cat >"$o/commands/build.md" <<'EOF'
Execute the template at templates/build.md, applying the bindings below.

## Bindings

**exit-condition** — the queue is empty.

**ritual** — pick the first task.
EOF
check_case "orphan-binding" "$o" 1 "binding 'ritual' names no slot"

# --- missing template: the directive names a file that does not exist ---
m="$SANDBOX/missing"
mkdir -p "$m/commands"
cat >"$m/commands/build.md" <<'EOF'
Execute the template at templates/nope.md, applying the bindings below.

## Bindings
EOF
check_case "missing-template" "$m" 1 "no such file"

# --- skip: a skill with no binding directive is not read (copy-and-specialize) ---
s="$SANDBOX/skip"
mkdir -p "$s/commands"
cat >"$s/commands/release-sweep.md" <<'EOF'
A copy-and-specialize skill with an ordinary <placeholder> and no directive.
EOF
check_case "no-directive-skipped" "$s" 0 "SKILL-BINDING: clean"

# --- degenerate: a template with no slots binds cleanly to an empty ## Bindings ---
z="$SANDBOX/zero"
mkdir -p "$z/commands" "$z/templates"
cat >"$z/templates/close.md" <<'EOF'
The close stage. Exit condition: Done cleared. No consumer slots here.
EOF
cat >"$z/commands/close.md" <<'EOF'
Execute the template at templates/close.md, applying the bindings below.

## Bindings
EOF
check_case "no-slots-clean" "$z" 0 "SKILL-BINDING: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-skill-binding.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-skill-binding.test.sh: clean (orphan-binding + missing-template + no-directive-skip + no-slots, 4 cases)"
exit 0
