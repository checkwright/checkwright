#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — inherited kit variables are scrubbed before library pins
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

out="$(bash -c '
    export GATE_SDK_VERBOSE=foreign CANON_KIT_SPEC_NAME=foreign
    export GATE_SDK_GATES_DIR=foreign EVIDENCE_KIT_LOCK_FILE=foreign
    export GATE_SDK_NATIVE_BIN=/pinned/gates
    source "$1"
    [[ ! -v GATE_SDK_VERBOSE && ! -v CANON_KIT_SPEC_NAME && ! -v GATE_SDK_GATES_DIR && ! -v EVIDENCE_KIT_LOCK_FILE ]] || exit 1
    [[ "$GATE_SDK_NATIVE_BIN" == /pinned/gates ]] || exit 1
    [[ -f "$GATE_SDK_KNOB_FILE" && ! -s "$GATE_SDK_KNOB_FILE" ]] || exit 1
    [[ -f "$CANON_KIT_KNOB_FILE" && ! -s "$CANON_KIT_KNOB_FILE" ]] || exit 1
    [[ "$GATE_SDK_ROOT" == "$2" ]] || exit 1
    export GATE_SDK_VERBOSE=deliberate
    [[ "$GATE_SDK_VERBOSE" == deliberate ]] || exit 1
    echo survived
' bash "$GATE_SDK_TEST_LIB_DIR/test-hermetic.sh" "$GATE_SDK_ROOT")" || {
    echo 'FAIL: inherited kit variables survived or library pins were lost'
    exit 1
}
[[ "$out" == survived ]] || exit 1

hostile="$(mktemp -d)"
trap 'rm -rf "$hostile"' EXIT
mkdir -p "$hostile/home/.config/git" "$hostile/plain"
printf '[core]\n\thooksPath = %s\n' /hostile-system >"$hostile/system.config"
printf '[core]\n\thooksPath = %s\n' /hostile-global >"$hostile/global.config"
printf '[core]\n\thooksPath = %s\n' /hostile-home >"$hostile/home/.gitconfig"
printf '[core]\n\thooksPath = %s\n' /hostile-xdg >"$hostile/home/.config/git/config"
system=GIT_CONFIG_SYSTEM
case "${OSTYPE:-}" in msys* | cygwin* | win32*) system=GIT_CONFIG_HOSTILE_UNREAD ;; esac
leaked="$(env -u GIT_CONFIG_NOSYSTEM "$system=$hostile/system.config" GIT_CONFIG_GLOBAL="$hostile/global.config" \
    HOME="$hostile/home" XDG_CONFIG_HOME="$hostile/home/.config" \
    bash -c 'source "$1"; git -C "$2" config --get-all core.hooksPath' \
    bash "$GATE_SDK_TEST_LIB_DIR/test-hermetic.sh" "$hostile/plain")"
[[ -z "$leaked" ]] || {
    printf 'FAIL: a sourcing suite read host git config: %s\n' "$leaked"
    exit 1
}

# A name under the temp dir that is not private to this user is refused at 3 and never written:
# a link to another directory, and off native Windows a directory open to others.
mkdir -p "$hostile/tmp-link" "$hostile/target" "$hostile/tmp-open/gate-sdk-hermetic.$(id -u)"
ln -s "$hostile/target" "$hostile/tmp-link/gate-sdk-hermetic.$(id -u)"
TMPDIR="$hostile/tmp-link" bash -c 'source "$1"' bash "$GATE_SDK_TEST_LIB_DIR/test-hermetic.sh" 2>/dev/null
rc=$?
[[ "$rc" == 3 && -z "$(ls -A "$hostile/target")" ]] || {
    printf 'FAIL: a linked private-dir name was used (exit %s): %s\n' "$rc" "$(ls -A "$hostile/target")"
    exit 1
}
case "${OSTYPE:-}" in
    msys* | cygwin* | win32*) ;;
    *)
        open="$hostile/tmp-open/gate-sdk-hermetic.$(id -u)"
        chmod 755 "$open"
        TMPDIR="$hostile/tmp-open" bash -c 'source "$1"' bash "$GATE_SDK_TEST_LIB_DIR/test-hermetic.sh" 2>/dev/null
        rc=$?
        [[ "$rc" == 3 && -z "$(ls -A "$open")" ]] || {
            printf 'FAIL: an open private-dir name was used (exit %s)\n' "$rc"
            exit 1
        }
        ;;
esac
fresh="$hostile/tmp-fresh"
mkdir -p "$fresh"
TMPDIR="$fresh" bash -c 'source "$1"' bash "$GATE_SDK_TEST_LIB_DIR/test-hermetic.sh" || {
    echo 'FAIL: a fresh temp dir refused the private dir'
    exit 1
}
[[ -f "$fresh/gate-sdk-hermetic.$(id -u)/empty.knobs" ]] || {
    echo 'FAIL: the empty files are not under the private dir'
    exit 1
}
twice="$(TMPDIR="$fresh" bash -ec 'source "$1"; source "$1"; echo twice' bash "$GATE_SDK_TEST_LIB_DIR/test-hermetic.sh")"
[[ "$twice" == twice ]] || {
    echo 'FAIL: a second source under errexit died on the private dir already present'
    exit 1
}
echo 'LIB-TEST-HERMETIC: clean (inherited kit variables cleared, binary kept, library and later pins applied, host git config unread, a non-private temp name refused, a second source under errexit survived)'
