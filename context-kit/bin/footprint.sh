#!/usr/bin/env bash
# spec: context-kit/SPEC.md §bin/footprint — per-kit two-tier context footprint, a deterministic projection of tracked kit surfaces
# usage: footprint.sh [--emit]   (bare: human header + table; --emit: the committed docs/footprint.md page)
set -uo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || { echo "footprint: cannot enter repo root" >&2; exit 2; }

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/context-config.sh}"
if [[ -f "$_ck_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
fi
unset _ck_cfg

declare -p CONTEXT_KIT_SURFACES >/dev/null 2>&1 || CONTEXT_KIT_SURFACES=("CLAUDE.md")

emit=0
[[ "${1:-}" == "--emit" ]] && emit=1

# spec: context-kit/SPEC.md §bin/footprint — the measured set is the kit roster, derived not maintained: top-level dirs carrying a SPEC.md
mapfile -t KITS < <(for _s in */SPEC.md; do [[ -f "$_s" ]] && printf '%s\n' "${_s%/SPEC.md}"; done | sort)

# spec: context-kit/SPEC.md §bin/footprint — a file set collapses to exact line+word counts and a byte total; the token estimate is bytes/4, computed at render
sum_files() {   # args: files... -> "<lines> <words> <bytes>"
    local L=0 W=0 B=0 f n
    for f in "$@"; do
        [[ -f "$f" ]] || continue
        n="$(wc -l < "$f")"; L=$(( L + n ))
        n="$(wc -w < "$f")"; W=$(( W + n ))
        n="$(wc -c < "$f")"; B=$(( B + n ))
    done
    printf '%d %d %d' "$L" "$W" "$B"
}

# spec: context-kit/SPEC.md §bin/footprint — always-loaded tier: the content between a kit's generated begin/end markers in the configured surface files, the agent-file block the kit injects
kit_always() {   # $1 = kit -> "<lines> <words> <bytes>"
    local kit="$1" L=0 W=0 B=0 sf block bl bw bb
    for sf in "${CONTEXT_KIT_SURFACES[@]}"; do
        [[ -f "$sf" ]] || continue
        block="$(awk -v k="$kit" '
            index($0, "<!-- " k ":begin -->") { inb=1; next }
            index($0, "<!-- " k ":end -->")   { inb=0; next }
            inb { print }
        ' "$sf")"
        [[ -n "$block" ]] || continue
        bl="$(printf '%s\n' "$block" | wc -l)"; L=$(( L + bl ))
        bw="$(printf '%s\n' "$block" | wc -w)"; W=$(( W + bw ))
        bb="$(printf '%s\n' "$block" | wc -c)"; B=$(( B + bb ))
    done
    printf '%d %d %d' "$L" "$W" "$B"
}

# spec: context-kit/SPEC.md §bin/footprint — load-triggered tier: the kit's shipped skill/template markdown under its templates tree (gate-test fixtures sit outside it, so they never enter the count)
kit_triggered() {   # $1 = kit -> "<lines> <words> <bytes>"
    local kit="$1"
    mapfile -t _tf < <(find "$kit/templates" -type f -name '*.md' 2>/dev/null | sort)
    sum_files "${_tf[@]+"${_tf[@]}"}"
}

cell() {   # $1 $2 $3 = lines words bytes -> "<n>l · <n>w · ~<n>t" or an em dash when empty
    local l="$1" w="$2" b="$3"
    if [[ "$l" -eq 0 && "$w" -eq 0 && "$b" -eq 0 ]]; then printf '%s' "—"; return; fi
    printf '%dl · %dw · ~%dt' "$l" "$w" $(( b / 4 ))
}

ROWS=()
tAL=0 tAW=0 tAB=0 tTL=0 tTW=0 tTB=0
for kit in "${KITS[@]}"; do
    read -r al aw ab <<<"$(kit_always "$kit")"
    read -r tl tw tb <<<"$(kit_triggered "$kit")"
    tAL=$(( tAL + al )); tAW=$(( tAW + aw )); tAB=$(( tAB + ab ))
    tTL=$(( tTL + tl )); tTW=$(( tTW + tw )); tTB=$(( tTB + tb ))
    ROWS+=("| $kit | $(cell "$al" "$aw" "$ab") | $(cell "$tl" "$tw" "$tb") |")
done

print_table() {
    echo "| kit | always-loaded | load-triggered |"
    echo "| --- | --- | --- |"
    printf '%s\n' "${ROWS[@]}"
    echo "| **total** | $(cell "$tAL" "$tAW" "$tAB") | $(cell "$tTL" "$tTW" "$tTB") |"
}

if [[ "$emit" -eq 0 ]]; then
    echo "=== Context footprint (context-kit — kit-shipped share only) ==="
    echo "Per-kit always-loaded vs load-triggered cost; token column a bytes/4 estimate."
    echo
    print_table
    exit 0
fi

cat <<'EOF'
---
title: Footprint
---

# Context footprint

What vendoring Checkwright costs a consumer's context budget, measured per kit
and split by when the cost is paid. Every number here is generated from the
tracked kit surfaces by `context-kit/bin/footprint.sh` and held current by a
freshness gate, so the page cannot drift from what the kits actually ship.

## What is measured

Each kit's footprint splits by when its cost lands in a session:

- **Always-loaded** — the fixed block a kit injects into the consumer's
  always-loaded agent file, so it rides every session's context. Measured as the
  content a kit generates between its own `begin`/`end` markers in the configured
  surface files.
- **Load-triggered** — the kit's shipped skill and template markdown, pulled
  into context only when its trigger fires. Measured over the markdown the kit
  ships under its templates directory.

Line and word counts are exact. The token column is a labeled estimate — a
bytes-over-four heuristic, marked with a leading `~` because the true count is
model-tokenizer-dependent; read it as an order of magnitude, never a precise
figure.

## What is excluded

The figures are kit-share only — what a kit itself ships. A consumer's own
bindings (the skill shims that point at a vendored template), consumer
configuration, the reference SPEC and README pages a reader opens on demand, and
the session hook's dynamic body (which is consumer state, not fixed kit text) are
all left out, so each number reflects the kit's advertised cost rather than a
host repository's residue.

## Per-kit footprint

EOF
print_table
exit 0
