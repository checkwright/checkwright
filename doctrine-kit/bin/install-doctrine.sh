#!/usr/bin/env bash
# spec: doctrine-kit/SPEC.md §install-doctrine — idempotent insert/replace of the doctrine reference block between fixed markers in the always-loaded agent file
set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/doctrine.sh
source "$KIT/lib/doctrine.sh"

AGENT_FILE="${1:-$DOCTRINE_KIT_AGENT_FILE}"
DOCTRINE_FILE="${2:-$DOCTRINE_KIT_DOCTRINE_FILE}"
[[ -f "$AGENT_FILE" ]] \
    || { echo "install-doctrine: agent file not found: $AGENT_FILE — nothing to install into" >&2; exit 2; }

BEGIN="<!-- doctrine-kit:begin -->"
END="<!-- doctrine-kit:end -->"

# spec: doctrine-kit/SPEC.md §install-doctrine — the block is the always-loaded shape applied to the doctrine itself: a one-line-per-rule digest plus the markdown link to the doctrine file
block() {
    cat <<EOF
$BEGIN
## Delivery doctrine

The cross-kit delivery rules live in [$DOCTRINE_FILE]($DOCTRINE_FILE) — re-vendor
to upgrade. The always-loaded maintenance rules, one line each; the doctrine adds
an engineering-craft section behind the link:

- **Content-tiering / SSOT** — one content tier per surface; point, never restate.
- **Enforcement-first** — the fix and the gate that catches it land in one unit; removing the duplication outranks gating it.
- **De-literalization** — prose cites names; code or the owning SPEC owns values.
- **Derivation-first** — derive the derivable (a roster, a count), never maintain it; a needed copy is generated and freshness-gated.
- **Always-loaded shape** — one line per rule here; the mechanism behind the pointer.
- **Load-trigger residency** — resident only when no stage, skill, or tool loads it.
- **Widest-true-tier placement** — the widest tier true for every reader of it.
- **Oracle-first** — run the gate, never emulate it; a red run is the feedback channel.
- **Spec-over-precedent** — the owner doc is ground truth; history answers what happened, never what is correct.
- **Gap disposition** — a gap you defer is costed and filed, never flagged-and-skipped.
$END
EOF
}

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

if grep -qF -- "$BEGIN" "$AGENT_FILE"; then
    grep -qF -- "$END" "$AGENT_FILE" \
        || { echo "install-doctrine: begin marker present but end marker missing in $AGENT_FILE — refusing to guess the block bounds" >&2; exit 2; }
    block > "$tmp.block"
    awk -v b="$BEGIN" -v e="$END" -v blockfile="$tmp.block" '
        $0 == b { skip = 1; while ((getline line < blockfile) > 0) print line; close(blockfile); next }
        $0 == e { skip = 0; next }
        !skip { print }
    ' "$AGENT_FILE" > "$tmp"
    rm -f "$tmp.block"
    action="replaced"
else
    cp "$AGENT_FILE" "$tmp"
    printf '\n' >> "$tmp"
    block >> "$tmp"
    action="appended"
fi

cp "$tmp" "$AGENT_FILE"
echo "install-doctrine: $action the doctrine reference block in $AGENT_FILE (link → $DOCTRINE_FILE)"
