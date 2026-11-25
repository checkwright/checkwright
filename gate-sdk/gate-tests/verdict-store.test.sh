#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — the verdict store end to end through the front end: a
# registry run on a clean tree keeps its verdict under the key its SARIF log's provenance carries,
# --verdict-lookup answers it there and answers absent under another digest, a tree that moves while
# members run or a modified tree keeps nothing, and an unset store is refused with its error
# document. Run by the --run-gate-tests arm.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUN="$ROOT/gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "verdict-store.test: runner not found: $RUN"; exit 2; }

scratch="$(cd -P "$(mktemp -d)" && pwd -P)"
trap 'rm -rf "$scratch"' EXIT

fails=0
fail() { echo "FAIL [$1]: $2"; [[ -n "${3:-}" ]] && printf '%s\n' "$3"; fails=$((fails + 1)); }

repo="$scratch/repo" gates="$scratch/gates" store="$scratch/store" sarif="$scratch/run.sarif"
mkdir -p "$repo" "$gates"
printf 'a\n' > "$repo/a.txt"
git -C "$repo" init -q
git -C "$repo" add -A
git -C "$repo" -c user.name=t -c user.email=t@e.invalid commit -qm base

# One member, red exactly where the tree holds a file named red.
{ printf '#!/usr/bin/env bash\n'
  printf '# graph: couples=a.txt dir=one valve=none tier=precommit\n'
  printf '[[ -n "${MOVE_TREE:-}" ]] && { printf "m\\n" >> a.txt; git -c user.name=t -c user.email=t@e.invalid commit -qam moved; }\n'
  printf '[[ -e red ]] && { echo "red: present"; exit 1; }\n'
  printf 'echo "g: clean (no red file)"\n'
} > "$gates/g.sh"
chmod +x "$gates/g.sh"
printf 'g\n' > "$gates/gates.list"

# battery <VAR=value>... — the front end's --run on the scratch registry, extra environment first
battery() {
    (cd "$repo" && env GATE_SDK_GATES_DIR="$gates" GATE_SDK_KIT_DIRS="$gates" GATE_SDK_TMP_DIR="$scratch/tmp" \
        GATE_SDK_SARIF_FILE="$sarif" "$@" bash "$RUN" --run 2>&1)
}
lookup() {
    (cd "$repo" && env "$@" bash "$RUN" --verdict-lookup "${key[@]}" 2>/dev/null)
}
# The key the run's provenance carries: treeId, tool.version and gateSet, read off the log.
read_key() {
    local log
    log="$(cat "$sarif")"
    key=(
        "$(grep -oE '"treeId": "[0-9a-f]+"' <<<"$log" | grep -oE '[0-9a-f]{40,64}')"
        "$(awk '/"provenance": \{/ { p = 1 } p && /"tool": \{/ { t = 1 } t && /"version":/ { gsub(/[",]/, "", $2); print $2; exit } t && /\}/ { t = 0 }' <<<"$log")"
        "$(grep -oE '"gateSet": "[0-9a-f]+"' <<<"$log" | grep -oE '[0-9a-f]{64}')"
    )
}

out="$(battery GATE_SDK_VERDICT_STORE="$store")"; rc=$?
[[ "$rc" -eq 0 ]] || fail "pass run" "expected exit 0, got $rc" "$out"
read_key
# spec: gate-sdk/SPEC.md §run-gates — an unstamped binary keeps nothing, so it fails here rather
# than passing on an empty store
[[ -n "${key[1]}" ]] || fail "stamp" "the provenance carries no tool.version: the binary is unstamped"
[[ -n "${key[0]}" && -n "${key[2]}" ]] || fail "key" "the provenance carries no treeId or gateSet" "$(cat "$sarif")"

got="$(lookup GATE_SDK_VERDICT_STORE="$store")"; rc=$?
[[ "$rc" -eq 0 && "$got" == '{"contract":"1.0.0","verdict":"pass"}' ]] || fail "lookup pass" "exit $rc: $got"
pass_key=("${key[@]}")

printf 'r\n' > "$repo/red"
git -C "$repo" add red
git -C "$repo" -c user.name=t -c user.email=t@e.invalid commit -qm red
out="$(battery GATE_SDK_VERDICT_STORE="$store")"; rc=$?
[[ "$rc" -eq 1 ]] || fail "fail run" "expected exit 1, got $rc" "$out"
read_key
got="$(lookup GATE_SDK_VERDICT_STORE="$store")"; rc=$?
[[ "$rc" -eq 0 && "$got" == '{"contract":"1.0.0","verdict":"fail"}' ]] || fail "lookup fail" "a stored fail is an answer at 0: exit $rc: $got"

key=("${pass_key[0]}" "${pass_key[1]}" "$(printf 'f%.0s' {1..64})")
got="$(lookup GATE_SDK_VERDICT_STORE="$store")"; rc=$?
[[ "$rc" -eq 0 && "$got" == '{"contract":"1.0.0","verdict":"absent"}' ]] || fail "another digest" "exit $rc: $got"

# spec: gate-sdk/SPEC.md §run-gates — a commit landing while members run moves the tree under the
# run, so its outcomes key nothing
out="$(battery GATE_SDK_VERDICT_STORE="$scratch/moved" MOVE_TREE=1)"; rc=$?
[[ "$rc" -eq 1 ]] || fail "moved run" "expected exit 1, got $rc" "$out"
[[ ! -e "$scratch/moved" ]] || fail "moved run" "a run whose tree moved kept a verdict" "$(ls -A "$scratch/moved")"
grep -q 'moved while members ran; nothing stored' <<<"$out" || fail "moved run" "no line names the unkept verdict" "$out"

printf 'b\n' >> "$repo/a.txt"
out="$(battery GATE_SDK_VERDICT_STORE="$scratch/modified")"; rc=$?
[[ "$rc" -eq 1 ]] || fail "modified run" "expected exit 1, got $rc" "$out"
[[ ! -e "$scratch/modified" ]] || fail "modified run" "a modified tree kept a verdict" "$(ls -A "$scratch/modified")"

key=("${pass_key[@]}")
got="$(lookup)"; rc=$?
[[ "$rc" -eq 2 && "$got" == '{"contract":"1.0.0","error":"refused"}' ]] || fail "unset store" "exit $rc: $got"

[[ "$fails" -eq 0 ]] || { echo "verdict-store.test: $fails assertion(s) failed"; exit 1; }
echo "verdict-store.test: clean (a registry run on a clean tree kept its pass and its fail under the SARIF log's key, --verdict-lookup answered each at exit 0 and absent under another digest, a tree moved mid-run and a modified tree kept nothing, and an unset store was refused at exit 2 with its error document)"
exit 0
