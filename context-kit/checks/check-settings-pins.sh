#!/usr/bin/env bash
# graph: couples=.claude/settings.json,scripts/settings-pins.conf dir=one valve=none tier=precommit
# spec: context-kit/SPEC.md §check-settings-pins — every pin in the pins file holds against the tracked harness settings file
#
# usage: check-settings-pins.sh [--fixture <dir>]
#   live: pins CONTEXT_KIT_SETTINGS_PINS against CONTEXT_KIT_SETTINGS_FILE;
#   --fixture <dir> reads <dir>/settings.json against <dir>/settings-pins.conf
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ck_cfg" ]]; then
    [[ -f "$_ck_cfg" ]] || {
        echo "context-kit: CONTEXT_KIT_CONFIG_FILE not found: $_ck_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
else
    _ck_cfg="${GATE_SDK_GATES_DIR:-scripts}/context-config.sh"
    if [[ -f "$_ck_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_ck_cfg"
    fi
fi
unset _ck_cfg

MODE=live
FIXTURE_DIR=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --fixture) MODE=fixture; FIXTURE_DIR="${2:-}"; shift 2 ;;
        -*) echo "check-settings-pins: unknown argument: $1" >&2; exit 2 ;;
        *)  echo "check-settings-pins: unexpected argument: $1" >&2; exit 2 ;;
    esac
done

if [[ "$MODE" == fixture ]]; then
    [[ -d "$FIXTURE_DIR" ]] || { echo "check-settings-pins: fixture dir not found: $FIXTURE_DIR" >&2; exit 2; }
    SETTINGS_FILE="$FIXTURE_DIR/settings.json"
    PINS_FILE="$FIXTURE_DIR/settings-pins.conf"
else
    : "${CONTEXT_KIT_SETTINGS_FILE:=.claude/settings.json}"
    : "${CONTEXT_KIT_SETTINGS_PINS:=${GATE_SDK_GATES_DIR:-scripts}/settings-pins.conf}"
    SETTINGS_FILE="$CONTEXT_KIT_SETTINGS_FILE"
    PINS_FILE="$CONTEXT_KIT_SETTINGS_PINS"
fi

# spec: context-kit/SPEC.md §check-settings-pins — absent pins file is the opt-in-off state, not a failure (the identity.conf precedent)
if [[ ! -e "$PINS_FILE" ]]; then
    echo "SETTINGS-PINS: clean (no pins file at $PINS_FILE — optional consumer config absent)"
    exit 0
fi
[[ -r "$PINS_FILE" ]] || { echo "check-settings-pins: pins file not readable: $PINS_FILE" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || { echo "check-settings-pins: jq not found — cannot read $SETTINGS_FILE" >&2; exit 2; }
[[ -r "$SETTINGS_FILE" ]] || { echo "check-settings-pins: settings file not readable: $SETTINGS_FILE" >&2; exit 2; }
jq -e . "$SETTINGS_FILE" >/dev/null 2>&1 || { echo "check-settings-pins: $SETTINGS_FILE is not valid JSON" >&2; exit 2; }

malformed=(); absent=(); mismatches=(); checked=0
while IFS= read -r line; do
    path="${line%%=*}"; path="${path#"${path%%[![:space:]]*}"}"; path="${path%"${path##*[![:space:]]}"}"
    expected="${line#*=}"; expected="${expected#"${expected%%[![:space:]]*}"}"; expected="${expected%"${expected##*[![:space:]]}"}"
    if [[ "$line" != *=* || -z "$path" || -z "$expected" || "$path" != .* ]]; then
        malformed+=("$line")
        continue
    fi
    actual="$(jq -c "$path" "$SETTINGS_FILE" 2>/dev/null)"; st=$?
    if [[ "$st" -ne 0 ]]; then
        malformed+=("$line  (jq could not evaluate path '$path')")
        continue
    fi
    checked=$((checked + 1))
    if [[ "$actual" == "$expected" ]]; then
        continue
    fi
    # spec: context-kit/SPEC.md §check-settings-pins — a pin naming an absent key is a desynced manifest (fail-closed), a present-but-wrong value is the legible violation
    if [[ "$actual" == "null" ]]; then
        absent+=("$path — pin expects $expected, but $SETTINGS_FILE has no such key")
    else
        mismatches+=("$path — pin expects $expected, settings has $actual")
    fi
done < <(gates_list_members "$PINS_FILE")

if [[ ${#malformed[@]} -gt 0 ]]; then
    echo "check-settings-pins: malformed pin(s) in $PINS_FILE (expected '<jq path> = <expected JSON>'):" >&2
    printf '  %s\n' "${malformed[@]}" >&2
    exit 2
fi
if [[ ${#absent[@]} -gt 0 ]]; then
    echo "check-settings-pins: pinned key absent from $SETTINGS_FILE — the manifest and settings have desynced:" >&2
    printf '  %s\n' "${absent[@]}" >&2
    echo "  help: add the key to $SETTINGS_FILE, or drop the pin from $PINS_FILE if the key was retired" >&2
    exit 2
fi

if [[ ${#mismatches[@]} -gt 0 ]]; then
    echo "check-settings-pins: $SETTINGS_FILE does not match $PINS_FILE:"
    printf '  %s\n' "${mismatches[@]}"
    echo "  help: restore each key in $SETTINGS_FILE to its pinned value, or — if the"
    echo "        expectation itself moved — update the matching line in $PINS_FILE."
    exit 1
fi

echo "SETTINGS-PINS: clean ($checked pin(s) hold against $SETTINGS_FILE)"
exit 0
