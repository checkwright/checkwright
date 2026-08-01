# shellcheck shell=bash
# spec: context-kit/SPEC.md §bin/env-probe — sourceable owner of the probe roster and its floor predicate; defines names and executes nothing, so a second reader obtains the roster by sourcing instead of grepping a script that does its work on execution

# spec: context-kit/SPEC.md §bin/env-probe — the roster, `<name>[:<min-version>[:<impl-token>]]`; a member gains a floor only where a construct the battery runs forces one, and the SPEC records that construct beside the token
# shellcheck disable=SC2034  # read by whoever sources this file (bin/env-probe.sh), never here
PROBE_SET=(bash:4.3 git jq awk::GNU sort::coreutils shellcheck)

# spec: context-kit/SPEC.md §bin/env-probe — positional fields, an empty field meaning unconstrained on that axis exactly as an omitted trailing one does, so `awk`, `awk:` and `awk::` parse to one member
tool_floor_parse() {   # $1 = roster element -> TOOL_FLOOR_NAME / _MIN / _IMPL
    local elem="$1" rest
    TOOL_FLOOR_NAME="${elem%%:*}"
    rest="${elem#"$TOOL_FLOOR_NAME"}"
    rest="${rest#:}"
    TOOL_FLOOR_MIN="${rest%%:*}"
    TOOL_FLOOR_IMPL=""
    if [[ "$rest" == *:* ]]; then
        rest="${rest#*:}"
        TOOL_FLOOR_IMPL="${rest%%:*}"
    fi
}

tool_floor_version() {   # $1 = version banner -> its first dotted-version token, empty when it carries none
    [[ "$1" =~ ([0-9]+(\.[0-9]+)+) ]] && printf '%s' "${BASH_REMATCH[1]}"
}

# spec: context-kit/SPEC.md §bin/env-probe — the floor predicate: one verdict from the closed set, `uncomparable` the fail-closed arm so an unparseable banner or a `sort` without `-V` is reported unverified and never as `ok`
tool_floor_check() {   # $1 = roster element, $2 = probed banner ("" when absent) -> ok | absent | below <found> <floor> | wrong-impl <found> | uncomparable
    local elem="$1" banner="$2" found sorted
    tool_floor_parse "$elem"
    [[ -n "$banner" ]] || { printf 'absent'; return 0; }

    if [[ -n "$TOOL_FLOOR_IMPL" && "$banner" != *"$TOOL_FLOOR_IMPL"* ]]; then
        printf 'wrong-impl %s' "${banner%%[[:space:]]*}"
        return 0
    fi

    [[ -n "$TOOL_FLOOR_MIN" ]] || { printf 'ok'; return 0; }
    found="$(tool_floor_version "$banner")"
    [[ -n "$found" ]] || { printf 'uncomparable'; return 0; }
    sorted="$(printf '%s\n%s\n' "$TOOL_FLOOR_MIN" "$found" | sort -V 2>/dev/null)" \
        || { printf 'uncomparable'; return 0; }
    if [[ "${sorted%%$'\n'*}" != "$TOOL_FLOOR_MIN" ]]; then
        printf 'below %s %s' "$found" "$TOOL_FLOOR_MIN"
        return 0
    fi
    printf 'ok'
}
