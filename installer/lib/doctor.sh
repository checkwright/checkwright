#!/usr/bin/env bash
# spec: installer/README.md §doctor — renders the toolchain floor as an exit status so init and a CI step can gate on the verdict without parsing a report, and reads the payload's own copy of the roster because at init time nothing is vendored in the consumer's tree yet
#
# usage: checkwright doctor
#   No --dry-run: doctor writes nothing, so it has no mutating form to guard.
set -uo pipefail

INSTALLER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FLOOR="$INSTALLER/payload/context-kit/lib/toolfloor.sh"

case "${1:-}" in
    '') ;;
    -h|--help)
        printf 'usage: checkwright doctor\n\n'
        printf 'Reports whether this machine meets the toolchain contract, and — when run\n'
        printf 'inside a repository that has been vendored into — what is installed there.\n'
        printf 'Exit status is the verdict: 0 meets the contract, 1 below it.\n'
        exit 0 ;;
    *) printf 'checkwright doctor: unknown argument: %s\n' "$1" >&2; exit 2 ;;
esac

[[ -f "$FLOOR" ]] || {
    printf 'checkwright doctor: no toolchain roster in this package (%s)\n' "${FLOOR#"$INSTALLER"/}" >&2
    printf '  help: doctor reads the roster from the package payload, which is assembled at pack time — run it from an installed package, not from a source checkout.\n' >&2
    exit 2
}
# shellcheck source=/dev/null  # payload path, assembled at pack time
source "$FLOOR"
# shellcheck source=./common/lock.sh
source "$INSTALLER/lib/common/lock.sh"

# spec: context-kit/SPEC.md §bin/env-probe — both version probes read from /dev/null, and `-V` is only the fallback: a tool rejecting `--version` would otherwise reach a `-V` that reads inherited stdin and hangs
probe_banner() {   # $1 = tool -> its raw version banner, empty when the tool is absent
    local raw
    command -v "$1" >/dev/null 2>&1 || return 0
    raw="$("$1" --version 2>/dev/null </dev/null)"
    [[ -n "$raw" ]] || raw="$("$1" -V 2>/dev/null </dev/null)"
    [[ -n "$raw" ]] || raw="present"
    printf '%s' "$raw"
}

# spec: installer/README.md §doctor — doctor defines no floor of its own: it renders whatever verdict the payload roster's own predicate returns, so the contract has one owner and this is a display of it
render() {   # $1 = tool, $2 = verdict words -> one report line; sets FAILED when the member is not clean
    local tool="$1" kind found floor
    read -r kind found floor <<<"$2"
    case "$kind" in
        ok)           printf '  %-12s %s\n' "$tool" "${FOUND_VERSION:-present}" ;;
        absent)       printf '  %-12s %s\n' "$tool" "NOT FOUND"; FAILED=1 ;;
        below)        printf '  %-12s %s (below the floor of %s)\n' "$tool" "$found" "$floor"; FAILED=1 ;;
        wrong-impl)   printf '  %-12s %s (not the %s implementation the contract requires)\n' "$tool" "${FOUND_VERSION:-$found}" "$TOOL_FLOOR_IMPL"; FAILED=1 ;;
        *)            printf '  %-12s could not be compared against the floor of %s\n' "$tool" "$TOOL_FLOOR_MIN"; FAILED=1 ;;
    esac
}

FAILED=0
printf 'toolchain\n'
for elem in "${PROBE_SET[@]}"; do
    tool_floor_parse "$elem"
    banner="$(probe_banner "$TOOL_FLOOR_NAME")"
    FOUND_VERSION="$(tool_floor_version "$banner")"
    render "$TOOL_FLOOR_NAME" "$(tool_floor_check "$elem" "$banner")"
done

ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || ROOT=""
[[ -n "$ROOT" ]] || ROOT="$PWD"
LOCK="$(lock_path "$ROOT")"

if [[ ! -f "$LOCK" ]]; then
    printf '\nNo %s here — nothing has been vendored into this directory.\n' "$CHECKWRIGHT_LOCK_FILE"
elif ! command -v jq >/dev/null 2>&1; then
    printf '\nFound %s, but jq is absent, so it cannot be read.\n' "$CHECKWRIGHT_LOCK_FILE"
elif ! lock_schema_ok "$LOCK"; then
    printf 'checkwright doctor: %s carries a schema this build does not know.\n' "$LOCK" >&2
    printf '  help: this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for.\n' >&2
    exit 2
else
    printf '\ninstalled\n'
    printf '  %-12s %s\n' version "$(lock_field "$LOCK" version)"
    printf '  %-12s %s\n' commit "$(lock_field "$LOCK" commit)"
    printf '  %-12s %s\n' profile "$(lock_field "$LOCK" profile)"
    printf '  %-12s %s\n' kits "$(lock_field "$LOCK" kits)"
fi

if (( FAILED )); then
    printf '\nDOCTOR: below contract\n'
    printf '  help: install or upgrade each tool reported above; the floors are the ones the gate battery needs to run, not preferences.\n'
    exit 1
fi
printf '\nDOCTOR: clean\n'
exit 0
