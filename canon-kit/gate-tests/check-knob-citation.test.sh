#!/usr/bin/env bash
# Behavioral test of the paths the one-pair good/bad harness cannot hold with a
# fixture SPEC that is never a real kit's owning SPEC: the owning-SPEC exemption
# (a kit may state its own knob's value in its own SPEC.md, but not in its
# README), and that a foreign knob named inside a ${...} shell expansion is a
# name citation the gate must never flag. A GATE_SDK_KIT_DIRS override plus an
# explicit CANON_KIT_MANIFEST_FILES set makes a sandbox 'widget-kit' the owning
# kit deterministically, independent of this repo's real kit roster.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
mkdir -p "$SANDBOX/widget-kit"

cat >"$SANDBOX/cfg.knobs" <<'EOF'
CANON_KIT_MANIFEST_FILES[] = widget-kit/SPEC.md
CANON_KIT_MANIFEST_FILES[] = widget-kit/README.md
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring  [$4=extra knob line]
    local label="$1" want="$2" sub="$3" extra="${4:-}" out rc
    { cat "$SANDBOX/cfg.knobs"; [[ -n "$extra" ]] && echo "$extra"; } >"$SANDBOX/case.knobs"
    out="$(cd "$SANDBOX" && gate_env GATE_SDK_KIT_DIRS="widget-kit" CANON_KIT_KNOB_FILE="$SANDBOX/case.knobs" \
        && gate_run check-knob-citation "$DIR/checks" 2>&1)"; rc=$?
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

# The reach: a token and the default stated after it sit 149 code points apart
# inside one sentence, beyond the default reach and inside a wider one; the
# sentence reach binds them until a terminator falls between; off reads no
# default shape while the '=' shape still fires.
filler="$(printf 'word %.0s' {1..25})"
printf '# widget-kit — README\n\n`WIDGET_KIT_STAGES_FILE` %sdefault `stages.list`.\n' "$filler" \
    >"$SANDBOX/widget-kit/README.md"
check_case "reach-default-misses-149" 0 "KNOB-CITATION: clean"
check_case "reach-200-binds-149" 1 "WIDGET_KIT_STAGES_FILE stated with a default value" "CANON_KIT_KNOB_CITATION_REACH = 200"
check_case "reach-sentence-binds-within-a-sentence" 1 "WIDGET_KIT_STAGES_FILE stated with a default value" "CANON_KIT_KNOB_CITATION_REACH = sentence"
check_case "reach-off-reads-no-default" 0 "the default-marker leg is off" "CANON_KIT_KNOB_CITATION_REACH = off"
printf '# widget-kit — README\n\n`WIDGET_KIT_STAGES_FILE` word word. %sdefault `stages.list`.\n' "$filler" \
    >"$SANDBOX/widget-kit/README.md"
check_case "reach-sentence-stops-at-a-terminator" 0 "KNOB-CITATION: clean" "CANON_KIT_KNOB_CITATION_REACH = sentence"
cat >"$SANDBOX/widget-kit/README.md" <<'EOF'
# widget-kit — README

Set `WIDGET_KIT_STAGES_FILE=stages.list` to override it.
EOF
check_case "reach-off-keeps-the-equals-shape" 1 "stated with an '=' value" "CANON_KIT_KNOB_CITATION_REACH = off"
check_case "reach-unknown-value-exits-2" 2 "CANON_KIT_KNOB_CITATION_REACH" "CANON_KIT_KNOB_CITATION_REACH = near"

# The literal span: the backticked value closes 34 bytes after the word, past
# the default span and inside a wider one.
cat >"$SANDBOX/widget-kit/README.md" <<'EOF'
# widget-kit — README

`WIDGET_KIT_STAGE_CAP` defaults, as the kit table has it, to `7`.
EOF
check_case "span-default-misses-34-bytes" 0 "KNOB-CITATION: clean"
check_case "span-40-binds-34-bytes" 1 "WIDGET_KIT_STAGE_CAP stated with a default value" "CANON_KIT_KNOB_CITATION_LITERAL_SPAN = 40"
check_case "span-unknown-value-exits-2" 2 "CANON_KIT_KNOB_CITATION_LITERAL_SPAN" "CANON_KIT_KNOB_CITATION_LITERAL_SPAN = wide"

if [[ "$fails" -gt 0 ]]; then
    echo "check-knob-citation.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-knob-citation.test.sh: clean (own-SPEC exemption + owning-kit README fires + \${...} name-citation clean + the reach at a count, sentence and off + the literal span)"
exit 0
