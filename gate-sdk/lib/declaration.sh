#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §lib/declaration.sh — the tightened-gates declaration grammar: two container arms over one token predicate, reporting the resolve/refuse trichotomy on the statuses named there. Sourced, never executed.

DECL_TOKEN_RE='^[A-Za-z][A-Za-z0-9-]*$'

decl_section_bullets() {
    awk -v sec="$2" '
        /^##[[:space:]]/ { insec = ($0 ~ "^##[[:space:]]+" sec "[[:space:]]*$"); if (insec) found = 1; next }
        insec && /^[[:space:]]*[-*][[:space:]]+/ { print }
        END { exit(found ? 0 : 1) }
    ' "$1"
}

decl_section_is_none() {
    awk -v sec="$2" '
        /^##[[:space:]]/ { insec = ($0 ~ "^##[[:space:]]+" sec "[[:space:]]*$"); if (insec) found = 1; next }
        insec && !seen && /[^[:space:]]/ { seen = 1; none = ($0 ~ /^[[:space:]]*[Nn]one([^A-Za-z0-9]|$)/) }
        END { exit(found && none ? 0 : 1) }
    ' "$1"
}

decl_section_tokens() {
    local file="$1" sec="$2" bullets st line tok n=0 bad=()
    bullets="$(decl_section_bullets "$file" "$sec")"; st=$?
    [[ "$st" -eq 0 ]] || return 2
    decl_section_is_none "$file" "$sec" && return 0
    while IFS= read -r line; do
        [[ "$line" =~ [^[:space:]] ]] || continue
        tok=""
        [[ "$line" =~ ^[[:space:]]*[-*][[:space:]]+\`([^\`]*)\` ]] && tok="${BASH_REMATCH[1]}"
        if [[ -n "$tok" && "$tok" =~ $DECL_TOKEN_RE ]]; then
            printf '%s\n' "$tok"; n=$((n + 1))
        else
            bad+=("$line")
        fi
    done <<<"$bullets"
    [[ ${#bad[@]} -eq 0 && "$n" -gt 0 ]] && return 0
    [[ ${#bad[@]} -gt 0 ]] && printf '%s\n' "${bad[@]}"
    return 1
}

decl_record_tokens() {
    local file="$1" line bad=()
    [[ -f "$file" ]] || return 0
    while IFS= read -r line; do
        [[ "$line" == '#'* ]] && continue
        [[ "$line" =~ [^[:space:]] ]] || continue
        if [[ "$line" =~ $DECL_TOKEN_RE ]]; then
            printf '%s\n' "$line"
        else
            bad+=("$line")
        fi
    done < "$file"
    if [[ ${#bad[@]} -gt 0 ]]; then
        printf '%s\n' "${bad[@]}"
        return 1
    fi
    return 0
}
