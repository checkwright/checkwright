# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — pin every kit's <KIT>_KNOB_FILE (and its retired <KIT>_CONFIG_FILE) to one shared empty file so a bespoke gate-tests/*.test.sh runs on kit defaults, never the invoker's cwd config; knob-free by design (a config-pinning tool cannot be configured by the surface it pins)
# no-port: gate-sdk/SPEC.md §lib/test-hermetic.sh — this library's whole API is three shell functions, gate_env, gate_run and gate_arm_run, called *inside* the caller's own shell, and a binary arm cannot be sourced into bash: 92 of the 94 files matching */gate-tests/*.test.sh source this file as their first act, and there is no in-crate arm a source line can name. gate_env exists rather than an env prefix for the same reason a compiled form cannot recover — env cannot invoke a shell function, and a bridged knob is resolved when the argv is built, so an override set around the binary arrives after the value it was meant to change has been read. What the two composing functions compose is the bridge itself: gate_run calls gate_command and gate_arm_run calls gate_native_bin and gate_knob_env, all from lib/gate.sh, which is permanently shell, so this file sits inside the bridge rather than beside it. It resolves no knob of its own and computes no default. Structural, not a sizing judgment.
_th_root="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
_th_empty="${TMPDIR:-/tmp}/gate-sdk-hermetic-empty.sh"
: >"$_th_empty"
_th_empty_knobs="${TMPDIR:-/tmp}/gate-sdk-hermetic-empty.knobs"
: >"$_th_empty_knobs"
for _th_kit in "$_th_root"/gate-sdk "$_th_root"/*-kit; do
    [[ -d "$_th_kit" ]] || continue
    _th_var="$(printf '%s' "${_th_kit##*/}" | tr '[:lower:]-' '[:upper:]_')"
    export "${_th_var}_CONFIG_FILE=${_th_empty}"
    export "${_th_var}_KNOB_FILE=${_th_empty_knobs}"
done
GATE_SDK_TEST_LIB_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the gate-sdk root locator, absolute from this library's own anchor, because a suite drives its subject from a sandbox cwd the default cannot reach
export GATE_SDK_ROOT="${GATE_SDK_TEST_LIB_DIR%/lib}"
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the accessor's answer absolutized against this
# library's own anchor, never a second default; the already-set guard keeps a value the invoker
# pinned, which that section owns.
if [[ -z "${GATE_SDK_NATIVE_BIN:-}" ]]; then
    # shellcheck source=./gate.sh
    source "$GATE_SDK_TEST_LIB_DIR/gate.sh"
    _th_bin="$(gate_native_bin)"
    [[ "$_th_bin" == /* ]] || _th_bin="$_th_root/$_th_bin"
    export GATE_SDK_NATIVE_BIN="$_th_bin"
fi
unset _th_root _th_empty _th_empty_knobs _th_kit _th_var _th_bin

# spec: gate-sdk/SPEC.md §run-gate-tests — invoke a gate through its declared dispatch rather
# than by script path, so a bespoke test names a gate and never a substrate: gate_command
# resolves a `.sh` path or the binary, under the caller's cwd and env.
gate_env() {  # $1.. = NAME=VALUE — a case's environment, applied in the caller's subshell
    local _ge_kv
    for _ge_kv in "$@"; do export "${_ge_kv?}"; done
    return 0
}

# spec: gate-sdk/SPEC.md §The non-gate arm — the arm counterpart of gate_run below: invoke an arm through the binary rather than through bin/run-gates.sh. That front-end cds to the git toplevel and refuses outside a repository, and a hermetic harness drives its subject from a non-git sandbox cwd, so the harness locates the binary itself — the sanctioned second caller, not a second entry point into the emission path. An absent binary is exit 2, the verdict the dispatcher gives it.
gate_arm_run() {  # $1=arm flag  $2.. = the arm's own argv
    local _ar_bin
    # shellcheck source=./gate.sh
    source "$GATE_SDK_TEST_LIB_DIR/gate.sh"
    _ar_bin="$(gate_native_bin)"
    [[ -x "$_ar_bin" ]] || return 2
    "$_ar_bin" "$@"
}

gate_run() {  # $1=gate-name  $2=checks-dir (absolute)  $3.. = gate args
    local _gr_gate="$1" _gr_dir="$2"
    shift 2
    local -a _gr_argv=()
    # shellcheck source=./gate.sh
    source "$GATE_SDK_TEST_LIB_DIR/gate.sh"
    mapfile -t _gr_argv < <(gate_command "$_gr_gate" "$_gr_dir")
    # spec: gate-sdk/SPEC.md §Fail-closed contract — an argv gate_command refused to build is
    # exit 2, the same verdict the dispatcher gives it, never a skipped assertion
    [[ ${#_gr_argv[@]} -gt 0 ]] || return 2
    "${_gr_argv[@]}" "$@"
}
