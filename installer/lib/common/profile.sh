# shellcheck shell=bash
# spec: installer/README.md §Profiles — sourceable owner of the profile rosters: it parses profiles.list, derives `full` from the payload rather than reading a row, and is the one place a profile name resolves to a kit set

# spec: installer/README.md §Profiles — the per-profile gate set is a union over per-kit recipes, so this module reaches the one that owns them rather than leaving the dependency to whichever verb happens to source both: a caller holding only this module would otherwise get an empty union where it should get an error
# shellcheck source=./recipe.sh
source "$(dirname "${BASH_SOURCE[0]}")/recipe.sh"

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

# spec: installer/README.md §Profiles — the order is derived from set inclusion over the kit rosters and never declared beside them: a declared parent and the kit sets could disagree, and then two surfaces would assert the containment again. The profiles form a lattice rather than a chain, so a pair comparing in neither direction is legitimate and simply absent from this output
profile_order() {   # $1 = installer root -> one '<a><TAB><b>' line per ordered pair of distinct profiles whose kit set is contained in the other's
    local root="$1" a b sub sup k ok
    local -a names
    mapfile -t names < <(profile_names "$root")
    for a in "${names[@]}"; do
        sub="$(profile_kits "$root" "$a")"
        for b in "${names[@]}"; do
            [[ "$a" == "$b" ]] && continue
            sup="$(profile_kits "$root" "$b")"
            ok=1
            while IFS= read -r k; do
                [[ -n "$k" ]] || continue
                grep -qxF "$k" <<<"$sup" || { ok=0; break; }
            done <<<"$sub"
            (( ok )) && printf '%s\t%s\n' "$a" "$b"
        done
    done
}

# spec: installer/README.md §Profiles — what an adopter meets is the battery, not the directory list, so the profile's gate set is a derivation in its own right: one function answers it for the registry init writes and for the smoke's monotonicity assertion, rather than each unioning the per-kit recipes itself and the two drifting apart the day an arm varies by profile
profile_gates() {   # $1 = installer root, $2 = profile -> the sorted, de-duplicated union of the gates its kits register in a fresh consumer
    local root="$1" profile="$2" kit
    while IFS= read -r kit; do
        [[ -n "$kit" ]] || continue
        recipe_gates "$kit" "$profile"
    done < <(profile_kits "$root" "$profile") | LC_ALL=C sort -u
}
