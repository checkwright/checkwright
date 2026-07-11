#!/usr/bin/env bash
# Behavioral test of checks/check-doctrine-registration.sh — the scenarios the
# one-pair good/bad harness cannot hold. The good/bad fixture pair covers the
# lockstep-clean case and the digest-missing-a-rule case (assertion B); the
# harness admits one bad/ dir, so this drives the digest-extra-line case
# (assertion C), the declared-trim case (assertion B satisfied by a trim marker),
# the link-absent case (assertion A), and the three fail-closed exits.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # doctrine-kit/
GATE="$DIR/checks/check-doctrine-registration.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=sandbox-dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && "$GATE" AGENT.md DOCTRINE.md 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# A three-rule doctrine reused across cases (an engineering-craft rule sits
# outside the methodology section, so the digest never owes it a bullet).
write_doctrine() {  # $1=dir
    cat >"$1/DOCTRINE.md" <<'EOF'
# DOCTRINE.md — fixture doctrine

## Methodology-maintenance rules

1. **Content-tiering / SSOT.** One content tier per surface.
2. **Enforcement-first.** The fix and the gate land in one unit.
3. **De-literalization.** Prose cites names; code owns values.

## Engineering-craft rules

4. **Rename is a full-surface sweep.** Behind the link — never digested.
EOF
}

# --- extra: a digest bullet that owns no doctrine rule (assertion C) ---
x="$SANDBOX/extra"; mkdir -p "$x"; write_doctrine "$x"
cat >"$x/AGENT.md" <<'EOF'
# Agent (fixture)

## Delivery doctrine

Rules live in [DOCTRINE.md](DOCTRINE.md) — re-vendor to upgrade.

- **Content-tiering / SSOT** — one tier per surface.
- **Enforcement-first** — fix and gate in one unit.
- **De-literalization** — prose cites names.
- **Ghost rule** — a bullet the doctrine retired but the digest kept.
EOF
check_case "digest-extra-line" "$x" 1 "digest bullet owns no doctrine rule: Ghost rule"

# --- trim: an absent rule declared via a trim marker (assertion B satisfied) ---
t="$SANDBOX/trim"; mkdir -p "$t"; write_doctrine "$t"
cat >"$t/AGENT.md" <<'EOF'
# Agent (fixture)

## Delivery doctrine

Rules live in [DOCTRINE.md](DOCTRINE.md) — re-vendor to upgrade.

- **Content-tiering / SSOT** — one tier per surface.
- **Enforcement-first** — fix and gate in one unit.
<!-- doctrine-digest-trim: De-literalization — this consumer keeps it behind the link -->
EOF
check_case "declared-trim-clean" "$t" 0 "1 declared trim(s)"

# --- link-absent: no markdown link to the doctrine file (assertion A) ---
l="$SANDBOX/nolink"; mkdir -p "$l"; write_doctrine "$l"
cat >"$l/AGENT.md" <<'EOF'
# Agent (fixture)

## Delivery doctrine

The doctrine reference block was never installed — no link.
EOF
check_case "link-absent" "$l" 1 "carries no markdown link to the doctrine file"

# --- fail-closed: the doctrine file is missing ---
d="$SANDBOX/nodoctrine"; mkdir -p "$d"
cat >"$d/AGENT.md" <<'EOF'
# Agent (fixture)

## Delivery doctrine

Rules live in [DOCTRINE.md](DOCTRINE.md) — re-vendor to upgrade.

- **Content-tiering / SSOT** — one tier per surface.
EOF
check_case "doctrine-missing-failclosed" "$d" 2 "doctrine file not found"

# --- fail-closed: the digest section heading is absent ---
s="$SANDBOX/nosection"; mkdir -p "$s"; write_doctrine "$s"
cat >"$s/AGENT.md" <<'EOF'
# Agent (fixture)

## Resident rules

Rules live in [DOCTRINE.md](DOCTRINE.md) — the link is present, but there is no
digest section under the configured heading.
EOF
check_case "digest-section-missing-failclosed" "$s" 2 "DOCTRINE_KIT_DIGEST_SECTION"

# --- fail-closed: the doctrine's methodology section is absent ---
m="$SANDBOX/nomethod"; mkdir -p "$m"
cat >"$m/DOCTRINE.md" <<'EOF'
# DOCTRINE.md — fixture doctrine

## Engineering-craft rules

1. **Rename is a full-surface sweep.** No methodology section at all.
EOF
cat >"$m/AGENT.md" <<'EOF'
# Agent (fixture)

## Delivery doctrine

Rules live in [DOCTRINE.md](DOCTRINE.md) — re-vendor to upgrade.

- **Content-tiering / SSOT** — one tier per surface.
EOF
check_case "methodology-section-missing-failclosed" "$m" 2 "Methodology-maintenance rules"

if [[ "$fails" -gt 0 ]]; then
    echo "check-doctrine-registration.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-doctrine-registration.test.sh: clean (extra-line + declared-trim + link-absent + 3 fail-closed, 6 cases)"
exit 0
