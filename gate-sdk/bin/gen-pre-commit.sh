#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §gen-pre-commit — generated pre-commit hook (emit from the
# gate manifests; check-graph asserts the committed hook equals --emit).
#
# Emits <hooks-dir>/pre-commit from the per-gate `# graph:` manifests. A
# tier=precommit gate becomes one trigger block; trigger=/mode= shape it; a
# gen=manual gate's body is preserved from the sentinel region in the current
# hook.
#
#   bash gate-sdk/bin/gen-pre-commit.sh --emit    # print to stdout (check-graph uses this)
#   bash gate-sdk/bin/gen-pre-commit.sh --write   # rewrite <hooks-dir>/pre-commit
set -euo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

REPO_ROOT="$(git rev-parse --show-toplevel)" || {
    echo "gen-pre-commit: not inside a git repository" >&2
    exit 2
}
cd "$REPO_ROOT" || exit 2

GATES_DIR="$(gate_sdk_gates_dir)"
LIST="$GATES_DIR/gates.list"
HOOK="${GATE_SDK_HOOKS_DIR:-$GATES_DIR/git-hooks}/pre-commit"
SDK_REL="$(realpath --relative-to="$REPO_ROOT" "$SDK")"

[[ -f "$LIST" ]] || { echo "gen-pre-commit: no registry at $LIST" >&2; exit 2; }

mapfile -t CHECKS < <(gates_list_members "$LIST")

# Resolve a member to the repo-relative path the emitted hook will invoke.
resolve_rel() {
    gate_resolve "$1" "$GATES_DIR" "$SDK_REL/checks"
}

declare -A MANUAL=()
read_manual() {
    local line name="" buf=""
    [[ -f "$HOOK" ]] || return 0
    while IFS= read -r line; do
        if [[ "$line" =~ ^[[:space:]]*#\ \>\>\>\ manual:\ ([a-z0-9-]+) ]]; then
            name="${BASH_REMATCH[1]}"; buf=""; continue
        fi
        if [[ -n "$name" && "$line" =~ ^[[:space:]]*#\ \<\<\<\ manual:\ ([a-z0-9-]+) ]]; then
            MANUAL["$name"]="$buf"; name=""; continue
        fi
        [[ -n "$name" ]] && buf+="$line"$'\n'
    done < "$HOOK"
    return 0
}
read_manual

manifest_field() {
    local gate="$1" key="$2" src man kv
    src="$(resolve_rel "$gate")" || return 0
    man="$(grep -m1 '^# graph: ' "$src" 2>/dev/null || true)"
    for kv in ${man#\# graph: }; do
        [[ "$kv" == "$key="* ]] && { printf '%s' "${kv#"$key"=}"; return 0; }
    done
    return 0
}

emit_block() {
    local gate="$1" couples trigger mode gen relpath
    couples="$(manifest_field "$gate" couples)"
    trigger="$(manifest_field "$gate" trigger)"; trigger="${trigger:-$couples}"
    mode="$(manifest_field "$gate" mode)"
    gen="$(manifest_field "$gate" gen)"
    relpath="$(resolve_rel "$gate")" || relpath="$GATES_DIR/$gate.sh"

    printf '\n'
    if [[ "$gen" == manual ]]; then
        printf '# >>> manual: %s\n' "$gate"
        if [[ -n "${MANUAL[$gate]+x}" ]]; then
            printf '%s' "${MANUAL[$gate]}"
        else
            printf '%s\n' "    # TODO: fill this manual region, then re-run --emit"
        fi
        printf '# <<< manual: %s\n' "$gate"
        return 0
    fi

    local -a globs; IFS=',' read -ra globs <<<"$trigger"
    local quoted=""; local g
    for g in "${globs[@]}"; do quoted+=" '$g'"; done
    quoted="${quoted# }"

    if [[ "$trigger" == '*' ]]; then
        printf '%s\n' "$relpath || hook_fail $gate"
    elif [[ "$mode" == staged ]]; then
        printf 'mapfile -t _staged < <(git diff --cached --name-only --diff-filter=ACMR -- %s)\n' "$quoted"
        printf '%s\n' "_targets=()"
        printf '%s\n' 'for _f in "${_staged[@]}"; do [[ -f "$_f" ]] && _targets+=("$_f"); done'
        printf '%s\n' 'if [[ ${#_targets[@]} -gt 0 ]]; then'
        printf '    %s "${_targets[@]}" || hook_fail %s\n' "$relpath" "$gate"
        printf '%s\n' 'fi'
    else
        printf 'if staged_matches %s; then\n' "$quoted"
        printf '    %s || hook_fail %s\n' "$relpath" "$gate"
        printf '%s\n' 'fi'
    fi
}

emit() {
    cat <<'HEAD'
#!/usr/bin/env bash
# pre-commit - GENERATED, DO NOT EDIT (except gen=manual regions between sentinels).
#
# Emitted from the per-gate `# graph:` manifests by:
#     bash gate-sdk/bin/gen-pre-commit.sh --write
# Edit a gate's manifest (couples=/trigger=/mode=/gen=), or a gen=manual region
# below, then regenerate. check-graph asserts this file equals --emit. Each gate
# prints its own per-finding + `help:` lines before this hook reports the failure.
#
# Install (opt-in, per clone):   bash gate-sdk/bin/install-hooks.sh
# Bypass once (use sparingly):   git commit --no-verify
#
# This is the *triggered subset* of the gates.list battery: every check here
# also runs whole-tree via gate-sdk/bin/run-gates.sh.
set -euo pipefail

mapfile -t staged_all < <(git diff --cached --name-only --diff-filter=ACMR)
[[ ${#staged_all[@]} -eq 0 ]] && exit 0

# True if any staged path matches one of the given globs (bash glob: `*` spans '/').
staged_matches() {
    local f pat
    for f in "${staged_all[@]}"; do
        for pat in "$@"; do
            # shellcheck disable=SC2053
            [[ "$f" == $pat ]] && return 0
        done
    done
    return 1
}

# Uniform failure: the gate already printed its findings + help: line above.
hook_fail() {
    echo ""
    echo "pre-commit: $1 failed (see above)."
    echo "  Bypass once (use sparingly): git commit --no-verify"
    exit 1
}
HEAD

    local c tier
    local -A seen=()
    for c in "${CHECKS[@]}"; do
        [[ -n "${seen[$c]+x}" ]] && continue
        seen[$c]=1
        tier="$(manifest_field "$c" tier)"
        [[ "$tier" == precommit ]] || continue
        emit_block "$c"
    done

    printf '\nexit 0\n'
}

case "${1:-}" in
    --emit) emit ;;
    --write)
        mkdir -p "$(dirname "$HOOK")"
        emit > "$HOOK"
        chmod +x "$HOOK"
        echo "gen-pre-commit: wrote $HOOK"
        ;;
    *) echo "usage: gen-pre-commit.sh --emit|--write" >&2; exit 2 ;;
esac
