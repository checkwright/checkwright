#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — expected-output runner for the advisory bin tools
#
# The three index tools and the meter are advisory and speak plain text, so the
# gate contracts (output/fail-closed/fixture-pair/self-lint) do not fit. This
# runner drives each over the index-tests/ corpus and asserts exact output,
# failing on any diff. `check-brevity` is a gate and carries the standard pair.
#
#   run-index-tests.sh            diff each tool's output against its golden
#   run-index-tests.sh --update   rewrite the goldens from current output
#
# Run from the checkwright repo (the kit's own dev test):
#   bash context-kit/bin/run-index-tests.sh
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORPUS="$KIT/index-tests/corpus"
EXPECTED="$KIT/index-tests/expected"
BIN="$KIT/bin"

UPDATE=0
[[ "${1:-}" == "--update" ]] && UPDATE=1

mkdir -p "$EXPECTED"

# spec: context-kit/SPEC.md §Testing — goldens carry corpus-relative paths so
# they stay location-independent; normalize the git-top prefix, keeping from
# `corpus/` on.
norm() { sed 's#^[^ ]*/corpus/#corpus/#'; }

pass=0; fail=0; harness=0

check() {
    local name="$1" expected="$2"; shift 2
    local actual rc
    actual="$("$@" 2>/dev/null | norm)"; rc="${PIPESTATUS[0]}"
    if [[ "$rc" -ne 0 ]]; then
        echo "  HARNESS: $name — tool exited $rc"
        harness=$((harness + 1)); return
    fi
    if [[ "$UPDATE" -eq 1 ]]; then
        printf '%s\n' "$actual" > "$expected"
        echo "  UPDATED: $name -> ${expected#"$KIT"/}"
        return
    fi
    if [[ ! -f "$expected" ]]; then
        echo "  HARNESS: $name — no golden at ${expected#"$KIT"/} (run --update)"
        harness=$((harness + 1)); return
    fi
    if diff -u "$expected" <(printf '%s\n' "$actual") >/dev/null 2>&1; then
        pass=$((pass + 1))
    else
        echo "  FAIL: $name — output differs from ${expected#"$KIT"/}:"
        diff -u "$expected" <(printf '%s\n' "$actual") | sed 's/^/    /'
        fail=$((fail + 1))
    fi
}

check md-index   "$EXPECTED/md-index.txt"   bash "$BIN/md-index.sh"   "$CORPUS/sample.md"
check md-section "$EXPECTED/md-section.txt" bash "$BIN/md-section.sh" "$CORPUS/sample.md" "Code First"
check pub-index  "$EXPECTED/pub-index.txt"  bash "$BIN/pub-index.sh"  "$CORPUS/sample.rs"

# spec: context-kit/SPEC.md §Testing — always-loaded needs its array/command
# knobs, so drive it through a throwaway config pointing every knob at the
# corpus (absolute paths survive its cd).
cfg="$(mktemp)"
trap 'rm -f "$cfg"' EXIT
{
    echo "CONTEXT_KIT_SURFACES=(\"$CORPUS/surface.md\")"
    echo "CONTEXT_KIT_HOOK_CMD=\"cat $CORPUS/hook-sample.txt\""
    echo "CONTEXT_KIT_BASELINE_FILE=\"$CORPUS/baseline.txt\""
} > "$cfg"
check always-loaded "$EXPECTED/always-loaded.txt" \
    env "CONTEXT_KIT_CONFIG_FILE=$cfg" bash "$BIN/always-loaded.sh"

echo
if [[ "$UPDATE" -eq 1 ]]; then
    echo "INDEX-TESTS: goldens rewritten"
    exit 0
fi
if [[ "$harness" -gt 0 ]]; then
    echo "INDEX-TESTS: $harness harness error(s)"
    exit 2
fi
if [[ "$fail" -gt 0 ]]; then
    echo "INDEX-TESTS: $fail of $((pass + fail)) tool(s) differ from golden"
    exit 1
fi
echo "INDEX-TESTS: clean ($pass tools match golden)"
exit 0
