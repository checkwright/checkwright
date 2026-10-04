#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §Layout and configuration — a fixture suite's derived run member binds its door and its directories to the one working directory, so a binary run from a tree root that is not the repository's toplevel runs the suite there.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
SDK="$(cd "$DIR/../gate-sdk" && pwd)"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# The tree root sits one directory below the toplevel, and its door is a relative pin, so a door
# bound to the toplevel names a path that does not exist.
p="$tmp/proj"
mkdir -p "$p/probe-kit/gate-tests" "$p/probe-kit/checks" "$p/scripts" "$p/.workflow" "$p/.tmp" "$p/bin"
git -C "$tmp" init -q . >/dev/null 2>&1
ln -s "$GATE_SDK_NATIVE_BIN" "$p/bin/cw"
printf '#!/usr/bin/env bash\ntouch "%s/ran"\n' "$tmp" >"$p/probe-kit/gate-tests/ok.test.sh"
printf 'GATE_SDK_KIT_DIRS = probe-kit\nGATE_SDK_NATIVE_BIN = bin/cw\n' >"$p/scripts/gate-sdk-config.knobs"
printf 'EVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES\nEVIDENCE_KIT_RUN_ID = door-test\n' >"$p/scripts/evidence-config.knobs"
printf '# baseline\n' >"$p/.workflow/validate-baseline.txt"
printf '# contract: evidence-manifest v1\n' >"$p/.workflow/validate-evidence.txt"

prefix="$(git -C "$p" rev-parse --show-prefix 2>&1)"
if [[ "$prefix" != "proj/" ]]; then
    echo "  FAIL: the tree root must sit below its repository's toplevel, git answers prefix '$prefix'"; fails=$((fails + 1))
fi

out="$( cd "$p" && env -u GATE_SDK_NATIVE_BIN -u GATE_SDK_TMP_DIR -u GATE_SDK_GATES_DIR -u GATE_SDK_KIT_DIRS \
    GATE_SDK_ROOT="$SDK" GATE_SDK_KNOB_FILE=scripts/gate-sdk-config.knobs \
    EVIDENCE_KIT_KNOB_FILE=scripts/evidence-config.knobs ./bin/cw --run-validate 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]] || [[ "$out" != *"probe_kit -> clean"* ]]; then
    echo "  FAIL: the derived member did not run clean from a tree root below the toplevel (rc=$rc): $out"; fails=$((fails + 1))
fi
if [[ ! -e "$tmp/ran" ]]; then
    echo "  FAIL: the suite's unit test never ran, so the clean verdict is vacuous: $out"; fails=$((fails + 1))
fi
if ! grep -q '^door-test probe_kit .* verdict=clean ' "$p/.workflow/validate-evidence.txt"; then
    echo "  FAIL: no clean evidence line was folded: $(cat "$p/.workflow/validate-evidence.txt")"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "run-member-door.test: $fails assertion(s) failed"
    exit 1
fi
echo "run-member-door.test: ok (a fixture suite's derived member runs through a relative door from a tree root below the toplevel)"
exit 0
