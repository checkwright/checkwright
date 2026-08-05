#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — doctrine-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored doctrine-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

# spec: gate-sdk/SPEC.md §Consumer smoke — seed a minimal always-loaded agent file (guarded, so it composes with any kit that already dropped one), then install the reference block into it via the shipped installer
if [[ ! -f CLAUDE.md ]]; then
    cat > CLAUDE.md <<'EOF'
# CLAUDE.md — smoke consumer

Resident bindings the consumer keeps.
EOF
fi
bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" >/dev/null

# spec: doctrine-kit/SPEC.md §install-doctrine — every assertion below reads the doctrine block alone, never the whole agent file: a consumer with several kits installed carries several marker blocks in one CLAUDE.md, and a bullet outside this kit's span is not this kit's to reason about
SMOKE_BEGIN="<!-- doctrine-kit:begin -->"
SMOKE_END="<!-- doctrine-kit:end -->"
smoke_block() { awk -v b="$SMOKE_BEGIN" -v e="$SMOKE_END" '$0 == e { inb = 0 } inb; $0 == b { inb = 1 }' CLAUDE.md; }

# spec: doctrine-kit/SPEC.md §install-doctrine — the declared-trim round-trip's acceptor. inject.sh is a sourced library with no gate surface (gate-sdk/SPEC.md §lib/inject.sh), so its read half and this installer's preservation rule are exercised here: declare a trim in the block the installer just emitted, re-run the installer, and hold that the marker survived *in the trimmed bullet's position* with the bullet gone and the gate green
mapfile -t SMOKE_RULES < <(smoke_block | awk '/^- \*\*/ { sub(/^- \*\*/, ""); sub(/\*\*.*/, ""); print }')
if [[ ${#SMOKE_RULES[@]} -lt 2 ]]; then
    echo "doctrine smoke: the installed digest has fewer than two rule bullets — nothing to trim against" >&2
    exit 2
fi
SMOKE_TRIMMED="${SMOKE_RULES[0]}"
SMOKE_NEXT="- **${SMOKE_RULES[1]}**"
SMOKE_TRIM="<!-- doctrine-digest-trim: $SMOKE_TRIMMED — smoke: the declared-trim round-trip acceptor -->"

awk -v b="$SMOKE_BEGIN" -v e="$SMOKE_END" -v bullet="- **$SMOKE_TRIMMED**" -v trim="$SMOKE_TRIM" '
    $0 == b { inb = 1 }
    $0 == e { inb = 0 }
    inb && index($0, bullet) == 1 { print trim; next }
    { print }
' CLAUDE.md > CLAUDE.md.smoke && mv CLAUDE.md.smoke CLAUDE.md

bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" >/dev/null

if ! smoke_block | grep -qF -- "$SMOKE_TRIM"; then
    echo "doctrine smoke: the re-run dropped the declared trim for '$SMOKE_TRIMMED' — init silently revokes a customization doctrine-kit's own gate grants" >&2
    exit 1
fi
if smoke_block | grep -qF -- "- **$SMOKE_TRIMMED**"; then
    echo "doctrine smoke: the re-run restored the bullet for '$SMOKE_TRIMMED' — a block carrying the trim *and* its bullet hands back the rule the consumer removed" >&2
    exit 1
fi
if [[ "$(smoke_block | grep -A1 -F -- "$SMOKE_TRIM" | tail -n1)" != "$SMOKE_NEXT"* ]]; then
    echo "doctrine smoke: the carried trim for '$SMOKE_TRIMMED' is not in the trimmed bullet's position (expected '$SMOKE_NEXT' beneath it)" >&2
    exit 1
fi
bash "$SMOKE_KIT_ROOT/checks/check-doctrine-registration.sh" >/dev/null \
    || { echo "doctrine smoke: check-doctrine-registration is not green across the trim round-trip" >&2; exit 1; }

# spec: doctrine-kit/SPEC.md §install-doctrine — the other direction, and the reason the smoke's steady state is an untrimmed consumer: withdrawing the declaration restores the bullet where it was
grep -vF -- "$SMOKE_TRIM" CLAUDE.md > CLAUDE.md.smoke && mv CLAUDE.md.smoke CLAUDE.md
bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" >/dev/null
if ! smoke_block | grep -qF -- "- **$SMOKE_TRIMMED**"; then
    echo "doctrine smoke: withdrawing the trim for '$SMOKE_TRIMMED' did not restore its bullet" >&2
    exit 1
fi

cat >> scripts/gates.list <<'EOF'
# doctrine-kit
check-doctrine-registration
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > scripts/CHECK-GRAPH.html
