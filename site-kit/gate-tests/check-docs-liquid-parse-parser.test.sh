#!/usr/bin/env bash
# spec: site-kit/SPEC.md §check-docs-liquid-parse — the parser-contract arms the good/bad pair cannot carry, each run in a throwaway repository
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECKS="$DIR/checks"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

(
    cd "$tmp" || exit 1
    git init -q
    git config user.email fixture@example.invalid
    git config user.name fixture
    mkdir -p docs
) || { echo "  FAIL: could not init fixture repo"; exit 1; }

cat > "$tmp/docs/late.md" <<'MD'
---
title: A page whose error sits below its front matter
---

# Heading

Body line.
The leg reads `${{ matrix.runner` from its entry.
MD
for n in 1 2; do printf '# Page %s\n\nBody.\n' "$n" > "$tmp/docs/p$n.md"; done
git -C "$tmp" add docs

run_case() {
    local label="$1" knobs="$2" want_rc="$3" want_line="$4"
    local out rc
    out="$( cd "$tmp" && gate_env SITE_KIT_KNOB_FILE="$knobs" \
        && gate_run check-docs-liquid-parse "$CHECKS" 2>&1 )"; rc=$?
    [[ "$rc" -eq "$want_rc" ]] \
        || { echo "  FAIL: $label: expected exit $want_rc, got $rc: $out"; fails=$((fails + 1)); return; }
    grep -qF -- "$want_line" <<<"$out" \
        || { echo "  FAIL: $label: output lacks '$want_line': $out"; fails=$((fails + 1)); }
}

printf '%s\n' 'SITE_KIT_LIQUID_PARSER[] = bash' 'SITE_KIT_LIQUID_PARSER[] = -c' \
    "SITE_KIT_LIQUID_PARSER[] = tr -cd '\\000'" > "$tmp/blind.knobs"
run_case "a parser answering every document with an empty verdict" "$tmp/blind.knobs" 2 "failed its probe"

printf '%s\n' 'SITE_KIT_LIQUID_PARSER[] = bash' 'SITE_KIT_LIQUID_PARSER[] = -c' \
    'SITE_KIT_LIQUID_PARSER[] = cat >/dev/null; printf "\000err\000"' > "$tmp/short.knobs"
run_case "a parser passing the probe and then returning a short count" "$tmp/short.knobs" 2 "verdict(s) for 3 document(s)"

printf '%s\n' 'SITE_KIT_LIQUID_PARSER[] = site-kit-no-such-liquid-parser' > "$tmp/unresolvable.knobs"
run_case "an unresolvable parser command" "$tmp/unresolvable.knobs" 2 "failed its probe"

run_case "an error below a front-matter block" "" 1 "docs/late.md: Liquid syntax error (line 8):"

if [[ "$fails" -gt 0 ]]; then
    echo "check-docs-liquid-parse-parser.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-docs-liquid-parse-parser.test: ok (a blind parser, a short count and an unresolvable command each exit 2; an error below front matter reports the file's own line)"
exit 0
