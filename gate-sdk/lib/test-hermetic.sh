# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — pin every kit's <KIT>_CONFIG_FILE to one shared empty file so a bespoke gate-tests/*.test.sh runs on kit defaults, never the invoker's cwd config; knob-free by design (a config-pinning tool cannot be configured by the surface it pins)
_th_root="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
_th_empty="${TMPDIR:-/tmp}/gate-sdk-hermetic-empty.sh"
: >"$_th_empty"
for _th_kit in "$_th_root"/gate-sdk "$_th_root"/*-kit; do
    [[ -d "$_th_kit" ]] || continue
    _th_var="$(printf '%s' "${_th_kit##*/}" | tr '[:lower:]-' '[:upper:]_')_CONFIG_FILE"
    export "${_th_var}=${_th_empty}"
done
unset _th_root _th_empty _th_kit _th_var
