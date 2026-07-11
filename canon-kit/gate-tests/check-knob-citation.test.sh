#!/usr/bin/env bash
# Behavioral test of the paths the one-pair good/bad harness cannot hold with a
# fixture SPEC that is never a real kit's owning SPEC: the owning-SPEC exemption
# (a kit may state its own knob's value in its own SPEC.md, but not in its
# README), and that a foreign knob named inside a ${...} shell expansion is a
# name citation the gate must never flag. A GATE_SDK_KIT_DIRS override plus an
# explicit CANON_KIT_MANIFEST_FILES set makes a sandbox 'widget-kit' the owning
# kit deterministically, independent of this repo's real kit roster.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
GATE="$DIR/checks/check-knob-citation.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
mkdir -p "$SANDBOX/widget-kit"

cat >"$SANDBOX/cfg.sh" <<'EOF'
CANON_KIT_MANIFEST_FILES=("widget-kit/SPEC.md" "widget-kit/README.md")
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring
    local label="$1" want="$2" sub="$3" out rc
    out="$(cd "$SANDBOX" && env GATE_SDK_KIT_DIRS="widget-kit" CANON_KIT_CONFIG_FILE="$SANDBOX/cfg.sh" "$GATE" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# The owning kit states its own knob's value in its own SPEC.md — the exemption
# holds, the run is clean.
cat >"$SANDBOX/widget-kit/SPEC.md" <<'EOF'
# widget-kit — SPEC

- `WIDGET_KIT_STAGES_FILE` — default `stages.list`.
EOF
: >"$SANDBOX/widget-kit/README.md"
check_case "own-spec-exempt" 0 "KNOB-CITATION: clean"

# The same value in the same kit's README is a second home the SPEC should own
# alone — README is not the owning SPEC, so it fires and names the SPEC target.
cat >"$SANDBOX/widget-kit/README.md" <<'EOF'
# widget-kit — README

Set `WIDGET_KIT_STAGES_FILE=stages.list` to override the default.
EOF
check_case "own-readme-fires" 1 "widget-kit/SPEC.md"

# A foreign knob named inside a ${...} shell expansion (another knob's default
# expression) is a name citation, never a value statement of itself — clean.
cat >"$SANDBOX/widget-kit/README.md" <<'EOF'
# widget-kit — README

The queue resolves through `${WIDGET_KIT_QUEUE_FILE:-TASK-QUEUE.md}` when unset.
EOF
check_case "expansion-name-citation-clean" 0 "KNOB-CITATION: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-knob-citation.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-knob-citation.test.sh: clean (own-SPEC exemption + owning-kit README fires + \${...} name-citation clean)"
exit 0
