#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Fail-closed contract — under a non-C LC_ALL a roster tool the binary runs takes the C locale, a native gate's git included, while a consumer shell member keeps the exported value; under --init the commit's placed hook takes the caller's LC_ALL back, so a hook gate and a chained consumer hook record the caller's value, empty or unset as the caller had it
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
# shellcheck source=../lib/gate.sh
source "$HERE/gate-sdk/lib/gate.sh"
PINNED="$GATE_SDK_NATIVE_BIN"
[[ -x "$PINNED" ]] || { echo "child-locale.test: the gate binary $PINNED is absent — build it first"; exit 2; }
unset GATE_SDK_NATIVE_BIN GATE_SDK_GATES_DIR GATE_SDK_KIT_DIRS GATE_SDK_ROOT GATE_SDK_VERBOSE

if [[ -n "$(gate_exe_suffix)" ]]; then
    echo "child-locale.test: clean (skipped — its git wrapper and hook gates are sh scripts, which a host with an executable suffix cannot start)"
    exit 0
fi

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
LOCALE=el_GR.UTF-8

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

recorder() {
    printf '#!/bin/sh\n# graph: couples=%s dir=one valve=none tier=precommit\n%sprintf '"'"'%%s\\n'"'"' "${LC_ALL-<unset>}" >>"%s"\necho "%s: clean"\n' "$1" "$2" "$3" "$4"
}

# 1. a native gate's git and the runner's take C, and a consumer shell member keeps the exported value
real_git="$(command -v git)"
wrap="$SANDBOX/wrap"
mkdir -p "$wrap"
printf '#!/bin/sh\nprintf '"'"'%%s\\n'"'"' "${LC_ALL-<unset>}" >>"%s"\nexec "%s" "$@"\n' "$SANDBOX/git-rec" "$real_git" >"$wrap/git"
chmod +x "$wrap/git"
run="$SANDBOX/run"
mkdir -p "$run/scripts"
printf 'check-exec-bit\ncheck-print-locale\n' >"$run/scripts/gates.list"
cp "$HERE/gate-sdk/checks/check-exec-bit.gate" "$run/scripts/"
recorder '*.txt' '' "$SANDBOX/member-rec" PRINT-LOCALE >"$run/scripts/check-print-locale.sh"
chmod +x "$run/scripts/check-print-locale.sh"
git -C "$run" init -q -b main
git -C "$run" add -A
git -C "$run" -c user.email=t@example.invalid -c user.name=t commit -qm seed
out="$( cd "$run" && PATH="$wrap:$PATH" LC_ALL="$LOCALE" "$PINNED" --run 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note run-status "want exit 0 from --run, got $rc -- $out"
grep -qF 'All 2 gates passed.' <<<"$out" || note run-summary "the native gate and the member did not both run: $out"
git_rec="$(cat "$SANDBOX/git-rec" 2>/dev/null)"
[[ -n "$git_rec" ]] || note git-recorded "the wrapper recorded no git the binary ran: $out"
[[ -z "$(grep -vxF C <<<"$git_rec")" ]] || note git-locale "a git the binary ran kept a locale other than C: $git_rec"
[[ "$(cat "$SANDBOX/member-rec" 2>/dev/null)" == "$LOCALE" ]] || note member-locale "want the consumer member to keep $LOCALE, got: $(cat "$SANDBOX/member-rec" 2>/dev/null)"

# 2. --init's commit runs the placed hook, whose gate and chained consumer hook record the caller's LC_ALL
pkg="$SANDBOX/pkg"
mkdir -p "$pkg/payload/artifact/t" "$pkg/payload/a-kit/checks" "$SANDBOX/home"
printf '{"name":"scratch","version":"1.2.3"}\n' >"$pkg/package.json"
printf 'starter\ta-kit\n' >"$pkg/profiles.list"
cp "$PINNED" "$pkg/payload/artifact/t/checkwright-gates"
recorder checkwright.lock $'# install: zero-config\n' '$REC/gate' REC >"$pkg/payload/a-kit/checks/check-rec.sh"
chmod +x "$pkg/payload/a-kit/checks/check-rec.sh"

init_case() {
    local label="$1" want="$2"; shift 2
    local repo="$SANDBOX/init-$label" rec="$SANDBOX/rec-$label"
    mkdir -p "$repo/scripts" "$repo/native/target/release" "$rec"
    printf 'native/target/\n' >"$repo/.gitignore"
    printf 'check-rec\n' >"$repo/scripts/gates.list"
    cp "$pkg/payload/a-kit/checks/check-rec.sh" "$repo/scripts/"
    cp "$PINNED" "$repo/native/target/release/checkwright-gates"
    git -C "$repo" init -q -b main
    git -C "$repo" config user.email t@example.invalid
    git -C "$repo" config user.name t
    git -C "$repo" add -A
    git -C "$repo" commit -qm seed
    printf '#!/bin/sh\nprintf '"'"'%%s\\n'"'"' "${LC_ALL-<unset>}" >>"$REC/chained"\n' >"$repo/.git/hooks/pre-commit"
    chmod +x "$repo/.git/hooks/pre-commit"
    local out rc
    out="$( cd "$repo" && ./native/target/release/checkwright-gates --install-hooks 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] || { note "$label-hooks" "want exit 0 from --install-hooks, got $rc -- $out"; return; }
    out="$( cd "$repo" && env "$@" HOME="$SANDBOX/home" XDG_CONFIG_HOME="$SANDBOX/home" GATE_SDK_UPDATE_CHECK=off \
        REC="$rec" "$pkg/payload/artifact/t/checkwright-gates" --init 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] || { note "$label-init" "want exit 0 from --init, got $rc -- $out"; return; }
    [[ "$(git -C "$repo" rev-list --count HEAD)" -eq 2 ]] || note "$label-commit" "--init committed nothing: $out"
    local f
    for f in gate chained; do
        [[ -f "$rec/$f" && "$(cat "$rec/$f")" == "$want" ]] || note "$label-$f" "want the hook's $f to record '$want', got: $(cat "$rec/$f" 2>/dev/null || echo '<no record>')"
    done
}
init_case value "$LOCALE" LC_ALL="$LOCALE"
init_case empty '' LC_ALL=
init_case unset '<unset>' -u LC_ALL

[[ "$fails" -eq 0 ]] || { echo "child-locale.test: $fails assertion(s) failed"; exit 1; }
echo "child-locale.test: clean (under a non-C LC_ALL a native gate's git and the runner's take C while a consumer shell member keeps the exported value; --init's commit runs the placed hook in the caller's locale, its gate and a chained consumer hook recording the caller's value, empty or unset as the caller had it)"
exit 0
