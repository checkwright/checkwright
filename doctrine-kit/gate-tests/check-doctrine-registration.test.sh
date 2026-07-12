#!/usr/bin/env bash
# Behavioral test of checks/check-doctrine-registration.sh — the scenarios the
# one-pair good/bad harness cannot hold. The good/bad fixture pair covers the
# lockstep-clean case and the digest-missing-a-rule case (assertion B); the
# harness admits one bad/ dir, so this drives the digest-extra-line case
# (assertion C), the declared-trim case (assertion B satisfied by a trim marker),
# the link-absent case (assertion A), the craft-trailer cases (assertion D:
# untagged and malformed), and the four fail-closed exits.
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
   *Stages:* build
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

# A reusable full-digest agent file: all three methodology rules present, so a
# craft-side finding fires alone.
write_full_agent() {  # $1=dir
    cat >"$1/AGENT.md" <<'EOF'
# Agent (fixture)

## Delivery doctrine

Rules live in [DOCTRINE.md](DOCTRINE.md) — re-vendor to upgrade.

- **Content-tiering / SSOT** — one tier per surface.
- **Enforcement-first** — fix and gate in one unit.
- **De-literalization** — prose cites names.
EOF
}

# --- craft-untagged: a craft rule with no *Stages:* trailer (assertion D) ---
u="$SANDBOX/untagged"; mkdir -p "$u"; write_full_agent "$u"
cat >"$u/DOCTRINE.md" <<'EOF'
# DOCTRINE.md — fixture doctrine

## Methodology-maintenance rules

1. **Content-tiering / SSOT.** One content tier per surface.
2. **Enforcement-first.** The fix and the gate land in one unit.
3. **De-literalization.** Prose cites names; code owns values.

## Engineering-craft rules

4. **Rename is a full-surface sweep.** Untagged — no *Stages:* trailer.
EOF
check_case "craft-untagged" "$u" 1 "carries 0 *Stages:* trailer(s)"

# --- craft-malformed: a *Stages:* value that breaks the grammar (assertion D) ---
f="$SANDBOX/malformed"; mkdir -p "$f"; write_full_agent "$f"
cat >"$f/DOCTRINE.md" <<'EOF'
# DOCTRINE.md — fixture doctrine

## Methodology-maintenance rules

1. **Content-tiering / SSOT.** One content tier per surface.
2. **Enforcement-first.** The fix and the gate land in one unit.
3. **De-literalization.** Prose cites names; code owns values.

## Engineering-craft rules

4. **Rename is a full-surface sweep.** Uppercase stage token.
   *Stages:* Build
EOF
check_case "craft-malformed" "$f" 1 "*Stages:* value is malformed"

# --- fail-closed: the doctrine's engineering-craft section is absent ---
c="$SANDBOX/nocraft"; mkdir -p "$c"; write_full_agent "$c"
cat >"$c/DOCTRINE.md" <<'EOF'
# DOCTRINE.md — fixture doctrine

## Methodology-maintenance rules

1. **Content-tiering / SSOT.** One content tier per surface.
2. **Enforcement-first.** The fix and the gate land in one unit.
3. **De-literalization.** Prose cites names; code owns values.
EOF
check_case "craft-section-missing-failclosed" "$c" 2 "Engineering-craft rules"

if [[ "$fails" -gt 0 ]]; then
    echo "check-doctrine-registration.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-doctrine-registration.test.sh: clean (extra-line + declared-trim + link-absent + craft-untagged + craft-malformed + 4 fail-closed, 9 cases)"
exit 0
