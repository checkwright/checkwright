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

# spec: doctrine-kit/SPEC.md §install-doctrine — the two reported findings, each carried *and* named. Both ride stderr, so this captures that channel and not stdout
SMOKE_ORPHAN="<!-- doctrine-digest-trim: Rule-that-left — smoke: renamed upstream -->"
SMOKE_DUP="<!-- doctrine-digest-trim: $SMOKE_TRIMMED — smoke: a second declaration for one rule -->"
awk -v e="$SMOKE_END" -v orphan="$SMOKE_ORPHAN" -v dup="$SMOKE_DUP" '
    $0 == e { print orphan; print dup }
    { print }
' CLAUDE.md > CLAUDE.md.smoke && mv CLAUDE.md.smoke CLAUDE.md

SMOKE_ERR="$(bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" 2>&1 >/dev/null)"
if ! smoke_block | grep -qF -- "$SMOKE_ORPHAN"; then
    echo "doctrine smoke: the re-run dropped a trim naming no live rule — a rule renamed upstream must survive to the re-vendor moment, not vanish before the consumer can reconcile it" >&2
    exit 1
fi
case "$SMOKE_ERR" in
    *Rule-that-left*) : ;;
    *) echo "doctrine smoke: a trim naming no live rule was carried but never reported (stderr was: $SMOKE_ERR)" >&2; exit 1 ;;
esac
if [[ "$(smoke_block | grep -cF -- "doctrine-digest-trim: $SMOKE_TRIMMED")" != "1" ]]; then
    echo "doctrine smoke: the duplicate trim for '$SMOKE_TRIMMED' was not carried exactly once" >&2
    exit 1
fi
case "$SMOKE_ERR" in
    *duplicate*"$SMOKE_TRIMMED"*) : ;;
    *) echo "doctrine smoke: the duplicate trim for '$SMOKE_TRIMMED' was collapsed silently (stderr was: $SMOKE_ERR)" >&2; exit 1 ;;
esac
# spec: doctrine-kit/SPEC.md §install-doctrine — carrying an unmatched trim forward is only safe if the gate tolerates one; a gate that red on it would hand the consumer a broken battery instead of a reconciliation
bash "$SMOKE_KIT_ROOT/checks/check-doctrine-registration.sh" >/dev/null \
    || { echo "doctrine smoke: check-doctrine-registration reds on a carried-forward trim naming no live rule — carrying it forward would break the consumer's battery" >&2; exit 1; }

# spec: doctrine-kit/SPEC.md §install-doctrine — the other direction, and the reason the smoke's steady state is an untrimmed consumer: withdrawing the declarations restores the bullet where it was
grep -v -F -e "$SMOKE_TRIM" -e "$SMOKE_ORPHAN" -e "$SMOKE_DUP" CLAUDE.md > CLAUDE.md.smoke && mv CLAUDE.md.smoke CLAUDE.md
bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" >/dev/null
if ! smoke_block | grep -qF -- "- **$SMOKE_TRIMMED**"; then
    echo "doctrine smoke: withdrawing the trim for '$SMOKE_TRIMMED' did not restore its bullet" >&2
    exit 1
fi

# spec: doctrine-kit/SPEC.md §install-doctrine — the --remove path's acceptor. remove_marker_block (gate-sdk/SPEC.md §lib/inject.sh) is a sourced library with no gate surface, so its removal half is exercised here through the one caller that drives it: --remove must strip the block entirely, a second --remove must be an idempotent no-op, and a reinstall afterward must restore the same steady-state block this script's own baseline expects
bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" --remove >/dev/null
if smoke_block | grep -q .; then
    echo "doctrine smoke: --remove left doctrine block content in the agent file" >&2
    exit 1
fi
if grep -qF -- "$SMOKE_BEGIN" CLAUDE.md; then
    echo "doctrine smoke: --remove left the begin marker in the agent file" >&2
    exit 1
fi

SMOKE_REMOVED="$(cat CLAUDE.md)"
bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" --remove >/dev/null
if [[ "$(cat CLAUDE.md)" != "$SMOKE_REMOVED" ]]; then
    echo "doctrine smoke: a second --remove changed an already-clean agent file" >&2
    exit 1
fi

bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" >/dev/null
if ! smoke_block | grep -q .; then
    echo "doctrine smoke: a reinstall after --remove did not restore the doctrine block" >&2
    exit 1
fi
bash "$SMOKE_KIT_ROOT/checks/check-doctrine-registration.sh" >/dev/null \
    || { echo "doctrine smoke: check-doctrine-registration is not green after the --remove/reinstall round trip" >&2; exit 1; }

# spec: doctrine-kit/SPEC.md §install-doctrine — the derivation's refusals. The digest is derived from DOCTRINE.md's *Digest:* trailers, so a rule carrying none (or two) leaves its bullet undecidable; the installer must refuse rather than emit a digest silently one rule short, which is the exact defect the derivation replaced. Driven through the positional overrides so the refusal is *run*, not inspected
mkdir -p .doctrine-refusal
cat > .doctrine-refusal/AGENT.md <<'EOF'
# CLAUDE.md — refusal fixture

<!-- doctrine-kit:begin -->
sentinel
<!-- doctrine-kit:end -->
EOF
SMOKE_SENTINEL="$(cat .doctrine-refusal/AGENT.md)"

smoke_refusal() {   # $1=label  $2=stderr substring; the scratch doctrine on stdin
    local rc=0 err
    cat > .doctrine-refusal/DOCTRINE.md
    err="$(bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" \
        .doctrine-refusal/AGENT.md .doctrine-refusal/DOCTRINE.md 2>&1 >/dev/null)" || rc=$?
    if [[ "$rc" -ne 2 ]]; then
        echo "doctrine smoke: $1 — the installer exited $rc, want 2; a digest it cannot derive must fail closed, not ship short" >&2
        exit 1
    fi
    case "$err" in
        *"$2"*) : ;;
        *) echo "doctrine smoke: $1 — the refusal never named the offending rule (stderr was: $err)" >&2; exit 1 ;;
    esac
    if [[ "$(cat .doctrine-refusal/AGENT.md)" != "$SMOKE_SENTINEL" ]]; then
        echo "doctrine smoke: $1 — the installer refused but still wrote the agent file; a refusal must leave the block untouched" >&2
        exit 1
    fi
}

smoke_refusal "untrailered rule" "Untrailered" <<'EOF'
# DOCTRINE.md — refusal fixture

## Methodology-maintenance rules

1. **Kept.** Has a trailer.
   *Digest:* kept.
2. **Untrailered.** The rule that would ship a digest one bullet short.

## Engineering-craft rules

3. **Craft.** Behind the link.
   *Stages:* build
EOF

smoke_refusal "doubled trailer" "Doubled" <<'EOF'
# DOCTRINE.md — refusal fixture

## Methodology-maintenance rules

1. **Kept.** Has a trailer.
   *Digest:* kept.
2. **Doubled.** Two trailers, so the bullet is undecidable.
   *Digest:* first.
   *Digest:* second.

## Engineering-craft rules

3. **Craft.** Behind the link.
   *Stages:* build
EOF

rm -rf .doctrine-refusal

cat >> scripts/gates.list <<'EOF'
# doctrine-kit
check-doctrine-registration
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > scripts/CHECK-GRAPH.html
