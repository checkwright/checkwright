#!/usr/bin/env bash
# spec: context-kit/SPEC.md §The session-context hook — a configured command is started as a program with no shell: a brief command's stdout is printed whatever status it exits, one that cannot be started is silent, the stage-rules command takes the stage as its last argument, and one that cannot be started is reported at 127 with the hook still exiting 0
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

# spec: gate-sdk/SPEC.md §The path-dialect contract — every path the member reads is relative to the repository the front end enters, so no dialect of the sandbox's own path reaches it
repo="$SANDBOX/repo"
mkdir -p "$repo"
git -C "$repo" init -q
printf -- '---\nwitness-iteration build witness1 2000-01-01 0000000\n' >"$repo/state.txt"
knobs() {  # $1 = the stage-rules command
    {
        printf 'CONTEXT_KIT_STATE_FILE = state.txt\n'
        printf 'CONTEXT_KIT_BRIEF_COMMANDS[] = git rev-parse --sq-quote brief-command-ran\n'
        printf 'CONTEXT_KIT_BRIEF_COMMANDS[] = git rev-parse brief-nonzero-ran\n'
        printf 'CONTEXT_KIT_BRIEF_COMMANDS[] = no-such-brief-command\n'
        printf 'CONTEXT_KIT_STAGE_RULES = %s\n' "$1"
    } >"$repo/witness.knobs"
}
brief() {
    ( cd "$repo" && CONTEXT_KIT_KNOB_FILE=witness.knobs bash "$GATE_SDK_ROOT/bin/run-gates.sh" --hook session-context </dev/null 2>"$SANDBOX/stderr.txt" ) | tr -d '\r'
    return "${PIPESTATUS[0]}"
}

# 1. each configured program is started and its stdout lands in the brief
knobs 'git rev-parse --sq-quote stage-rules-ran'
out="$(brief)"; rc=$?
[[ "$rc" -eq 0 ]] || note started-status "want exit 0 from the hook, got $rc -- $out"
grep -qF "'brief-command-ran'" <<<"$out" || note brief-started "a brief command named as a program printed nothing: $out"
grep -qxF 'brief-nonzero-ran' <<<"$out" || note brief-nonzero "a brief command exiting non-zero lost its line: $out"
grep -qF 'no-such-brief-command' <<<"$out" && note brief-unstartable "a brief command that cannot be started was not silent: $out"
grep -qF 'Craft rules for the build stage' <<<"$out" || note rules-block "the stage-rules block is missing: $out"
grep -qF "'stage-rules-ran' 'build'" <<<"$out" || note rules-stage "the stage-rules program did not take the stage as its last argument: $out"

# 2. a stage-rules program that cannot be started is reported at 127 and the hook still exits 0
knobs 'no-such-stage-rules-command'
out="$(brief)"; rc=$?
[[ "$rc" -eq 0 ]] || note unstartable-status "want exit 0 from the hook, got $rc -- $out"
grep -qF 'the CONTEXT_KIT_STAGE_RULES command exited 127' <<<"$out" || note unstartable-text "the unstartable stage-rules command is not reported at 127: $out"
grep -qF "'brief-command-ran'" <<<"$out" || note unstartable-brief "the brief commands stopped printing beside a broken stage-rules knob: $out"

[[ "$fails" -eq 0 ]] || { echo "session-context-spawn.test: $fails assertion(s) failed; the last run's stderr: $(cat "$SANDBOX/stderr.txt" 2>/dev/null)"; exit 1; }
echo "session-context-spawn.test: clean (a configured program is started with no shell: a brief command prints whatever status it exits and is silent when it cannot be started, the stage-rules program takes the stage last, and one that cannot be started is reported at 127 with the hook exiting 0)"
exit 0
