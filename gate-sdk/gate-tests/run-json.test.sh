#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — `--run --json` end to end through the front end: on a registry
# with one passing and one failing member, stdout is the one run document whose verdict is fail,
# stderr carries the failing member's banner, and the status is 1. Run by the --run-gate-tests arm.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUN="$ROOT/gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "run-json.test: runner not found: $RUN"; exit 2; }

scratch="$(cd -P "$(mktemp -d)" && pwd -P)"
trap 'rm -rf "$scratch"' EXIT

fails=0
fail() { echo "FAIL [$1]: $2"; [[ -n "${3:-}" ]] && printf '%s\n' "$3"; fails=$((fails + 1)); }

repo="$scratch/repo" gates="$scratch/gates"
mkdir -p "$repo" "$gates"
printf 'a\n' > "$repo/a.txt"
git -C "$repo" init -q
git -C "$repo" add -A
git -C "$repo" -c user.name=t -c user.email=t@e.invalid commit -qm base

{ printf '#!/usr/bin/env bash\n'
  printf '# graph: couples=a.txt dir=one valve=none tier=precommit\n'
  printf 'echo "A: clean (one file)"\n'
} > "$gates/a.sh"
{ printf '#!/usr/bin/env bash\n'
  printf '# graph: couples=a.txt dir=one valve=none tier=precommit\n'
  printf 'echo "a.txt:1: red"; exit 1\n'
} > "$gates/b.sh"
chmod +x "$gates/a.sh" "$gates/b.sh"
printf 'a\nb\n' > "$gates/gates.list"

out="$(cd "$repo" && env GATE_SDK_GATES_DIR="$gates" GATE_SDK_KIT_DIRS="$gates" GATE_SDK_TMP_DIR="$scratch/tmp" \
    bash "$RUN" --run --json 2>"$scratch/stderr")"; rc=$?
err="$(cat "$scratch/stderr")"

[[ "$rc" -eq 1 ]] || fail "status" "expected exit 1, got $rc" "$err"
lines="$(printf '%s\n' "$out" | grep -c '')"
[[ "$lines" -eq 1 ]] || fail "one document" "stdout holds $lines lines, not the document alone" "$out"
[[ "$out" == '{"contract":"1.0.0",'*'}' ]] || fail "one document" "stdout is not one contract 1.0.0 object" "$out"
[[ "$out" == *'"members":[{"exit":0,"kind":"pass","name":"a"},{"exit":1,"kind":"fail","name":"b"}],"omitted":[],'* ]] \
    || fail "members" "the members are not a pass then a fail in gate-set order" "$out"
[[ "$out" == *'"scope":"registry","verdict":"fail"}' ]] || fail "verdict" "the verdict is not fail on a registry run" "$out"
[[ "$err" == *'===== b ====='* && "$err" == *'FAIL: b (exit 1)'* ]] || fail "stderr" "stderr lacks the failing member's banner" "$err"
[[ "$err" == *'1 of 2 gates FAILED: b'* ]] || fail "stderr" "stderr lacks the summary" "$err"

[[ "$fails" -eq 0 ]] || { echo "run-json.test: $fails assertion(s) failed"; exit 1; }
echo "run-json.test: clean (--run --json printed one contract 1.0.0 document on stdout with a pass and a fail member and the verdict fail, the transcript and summary on stderr, at exit 1)"
exit 0
