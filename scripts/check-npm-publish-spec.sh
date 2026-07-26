#!/usr/bin/env bash
# graph: couples=.github/workflows/*.yml,.github/workflows/*.yaml dir=one valve=none tier=precommit
# spec: RELEASING.md §The publish spec — every `npm publish` positional spec in a workflow is unambiguously a path by its own literal text: a leading `.` or `/`, or an expansion of a proven-absolute root
#
# usage: check-npm-publish-spec.sh [workflows-dir]
#   workflows-dir: the scanned directory (default .github/workflows). Its *.yml
#   and *.yaml members are read line by line; nothing is extracted from YAML.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

WFDIR="${1:-.github/workflows}"
if [[ ! -d "$WFDIR" ]]; then
    if [[ $# -gt 0 ]]; then
        echo "check-npm-publish-spec: workflows dir not found: $WFDIR" >&2; exit 2
    fi
    echo "NPM-PUBLISH-SPEC: clean (no $WFDIR in this tree — 0 npm publish invocation(s) to judge)"
    exit 0
fi

shopt -s nullglob
files=("$WFDIR"/*.yml "$WFDIR"/*.yaml)
shopt -u nullglob
if [[ ${#files[@]} -eq 0 ]]; then
    echo "NPM-PUBLISH-SPEC: clean (no YAML under $WFDIR — 0 npm publish invocation(s) to judge)"
    exit 0
fi

# spec: RELEASING.md §The publish spec — the roster is proven-absolute by written contract, never by runner observation
ABS_ROOTS=(PWD GITHUB_WORKSPACE RUNNER_TEMP)
# spec: RELEASING.md §The publish spec — the token after one of these is a flag value, not the positional spec
VALUE_FLAGS=(--access --tag --otp --registry --workspace)

# spec: RELEASING.md §The publish spec — quote-aware split, so a quoted spec carrying spaces stays one token instead of parsing as several
split_tokens() {
    local s="$1" tok="" q="" c i
    for (( i = 0; i < ${#s}; i++ )); do
        c="${s:i:1}"
        if [[ -n "$q" ]]; then
            tok+="$c"
            [[ "$c" == "$q" ]] && q=""
            continue
        fi
        case "$c" in
            '"'|"'") q="$c"; tok+="$c" ;;
            ' '|$'\t') [[ -n "$tok" ]] && printf '%s\n' "$tok"; tok="" ;;
            *) tok+="$c" ;;
        esac
    done
    [[ -n "$tok" ]] && printf '%s\n' "$tok"
    return 0
}

# spec: RELEASING.md §The publish spec — strip one layer of shell quoting before deciding, since every real spec on this surface is quoted
strip_quotes() {
    local t="$1"
    if [[ ${#t} -ge 2 && "${t:0:1}" == '"' && "${t: -1}" == '"' ]]; then
        t="${t:1:${#t}-2}"
    elif [[ ${#t} -ge 2 && "${t:0:1}" == "'" && "${t: -1}" == "'" ]]; then
        t="${t:1:${#t}-2}"
    fi
    printf '%s' "$t"
}

# spec: RELEASING.md §The publish spec — npm's own path rule, then the proven-absolute-root arm
spec_unambiguous() {
    local s="$1" r
    [[ "$s" == .* || "$s" == /* ]] && return 0
    for r in "${ABS_ROOTS[@]}"; do
        [[ "$s" == "\$$r/"* || "$s" == "\${$r}/"* ]] && return 0
    done
    return 1
}

invocations=0
findings=()

for f in "${files[@]}"; do
    lineno=0
    while IFS= read -r line || [[ -n "$line" ]]; do
        lineno=$((lineno + 1))
        [[ "$line" == *"npm publish"* ]] || continue
        [[ "${line#"${line%%[![:space:]]*}"}" == \#* ]] && continue
        invocations=$((invocations + 1))

        if [[ "$line" == *\\ ]]; then
            echo "check-npm-publish-spec: an 'npm publish' line ends in a backslash continuation, so its" >&2
            echo "positional spec is not line-local and this gate would judge a partial invocation:" >&2
            echo "  $f:$lineno" >&2
            echo "  help: join the invocation onto one line, or move the spec into a variable assigned on" >&2
            echo "        its own line and published through a './', '/', or \$PWD/-prefixed expansion." >&2
            exit 2
        fi

        mapfile -t toks < <(split_tokens "${line#*npm publish}")
        cands=(); prev=""
        for t in "${toks[@]+"${toks[@]}"}"; do
            skip=0
            for vf in "${VALUE_FLAGS[@]}"; do [[ "$prev" == "$vf" ]] && skip=1; done
            prev="$t"
            [[ "$skip" == 1 ]] && continue
            [[ "$t" == -* ]] && continue
            cands+=("$t")
        done

        if [[ ${#cands[@]} -gt 1 ]]; then
            echo "check-npm-publish-spec: more than one positional candidate parsed out of an 'npm publish'" >&2
            echo "invocation, which accepts at most one — the parse is wrong, so the gate refuses to guess:" >&2
            echo "  $f:$lineno: ${cands[*]}" >&2
            echo "  help: put the invocation on a line of its own carrying only flags and the one spec, or" >&2
            echo "        teach the gate's value-taking-flag roster the flag whose value it mistook for a spec." >&2
            exit 2
        fi
        [[ ${#cands[@]} -eq 1 ]] || continue

        bare="$(strip_quotes "${cands[0]}")"
        spec_unambiguous "$bare" || findings+=("$f:$lineno: ${cands[0]}")
    done < "$f"
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-npm-publish-spec: an npm publish spec whose resolution depends on runtime state rather"
    echo "than on its own literal text — npm reads a positional spec as a path when it begins with '.'"
    echo "or '/', and as the GitHub 'owner/repo' shorthand otherwise:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: make the literal decide it — prefix the spec with './' or '/', or with a bare or braced"
    echo "        expansion of a proven-absolute root (\$PWD/, \$GITHUB_WORKSPACE/, \$RUNNER_TEMP/)."
    echo "        A command substitution or a glob is assigned to a variable first, then published"
    echo "        through such a prefix."
    exit 1
fi

echo "NPM-PUBLISH-SPEC: clean ($invocations npm publish invocation(s) across ${#files[@]} YAML file(s) under $WFDIR, every positional spec unambiguous by its literal)"
exit 0
