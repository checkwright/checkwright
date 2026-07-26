# shellcheck shell=bash
# spec: installer/README.md §Profiles — sourceable owner of the profile rosters: it parses profiles.list, derives `full` from the payload rather than reading a row, and is the one place a profile name resolves to a kit set

# spec: installer/README.md §Profiles — the derived profile is never a row in profiles.list; a hand-maintained "all the kits" roster is drift the day a kit lands
PROFILE_DERIVED=full

profile_payload_kits() {   # $1 = installer root -> every kit root the payload carries, sorted
    local d
    shopt -s nullglob
    for d in "$1"/payload/*/; do
        d="${d%/}"
        printf '%s\n' "${d##*/}"
    done
    shopt -u nullglob
}

profile_rows() {   # $1 = installer root -> the `<profile><TAB><kit>` rows, comments and blanks dropped
    local f="$1/profiles.list"
    [[ -f "$f" ]] || return 0
    while IFS= read -r line; do
        line="${line%%#*}"
        [[ -n "${line//[[:space:]]/}" ]] || continue
        printf '%s\n' "$line"
    done < "$f"
}

profile_names() {   # $1 = installer root -> every selectable profile, the derived one last
    local seen="" p
    while IFS=$'\t' read -r p _; do
        [[ -n "$p" && "$seen" != *"|$p|"* ]] || continue
        seen="$seen|$p|"
        printf '%s\n' "$p"
    done < <(profile_rows "$1")
    printf '%s\n' "$PROFILE_DERIVED"
}

profile_kits() {   # $1 = installer root, $2 = profile -> its kit set in payload order, empty when the profile is unknown
    local root="$1" want="$2" p k members="" kit
    if [[ "$want" == "$PROFILE_DERIVED" ]]; then
        profile_payload_kits "$root"
        return 0
    fi
    while IFS=$'\t' read -r p k; do
        [[ "$p" == "$want" ]] || continue
        members="$members|$k|"
    done < <(profile_rows "$root")
    [[ -n "$members" ]] || return 0
    # spec: installer/README.md §Profiles — emit in payload order so a roster's line order never decides install order
    while IFS= read -r kit; do
        [[ "$members" == *"|$kit|"* ]] && printf '%s\n' "$kit"
    done < <(profile_payload_kits "$root")
}

profile_known() {   # $1 = installer root, $2 = profile -> 0 iff it is selectable
    local n
    while IFS= read -r n; do [[ "$n" == "$2" ]] && return 0; done < <(profile_names "$1")
    return 1
}
