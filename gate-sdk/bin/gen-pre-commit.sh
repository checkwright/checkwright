#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §gen-pre-commit — emit <hooks-dir>/pre-commit from the per-gate graph: manifests; check-graph asserts the committed hook equals --emit
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
HOOKS_DIR="${GATE_SDK_HOOKS_DIR:-$GATES_DIR/git-hooks}"
HOOK="$HOOKS_DIR/pre-commit"
MSG_HOOK="$HOOKS_DIR/commit-msg"
[[ -f "$LIST" ]] || { echo "gen-pre-commit: no registry at $LIST" >&2; exit 2; }

mapfile -t CHECKS < <(gates_list_members "$LIST")

REL_DIRS=("$GATES_DIR")
while IFS= read -r k; do
    REL_DIRS+=("$(realpath --relative-to="$REPO_ROOT" "$k")/checks")
done < <(gate_kit_roots)

resolve_rel() {
    gate_resolve "$1" "${REL_DIRS[@]}"
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
    local gate="$1" key="$2" src
    src="$(resolve_rel "$gate")" || return 0
    gate_manifest_field "$src" "$key"
}

# spec: gate-sdk/SPEC.md §run-gates — the hook's staged_matches is this lib body emitted verbatim; check-graph freshness holds them in sync
emit_staged_matcher_body() {
    local body
    body="$(awk '
        $0 == "gate_staged_matches() {" { grab = 1; next }
        grab && $0 == "}" { exit }
        grab { print }
    ' "$SDK/lib/gate.sh")"
    [[ -n "$body" ]] || { echo "gen-pre-commit: could not extract gate_staged_matches body from lib/gate.sh" >&2; exit 2; }
    printf '%s\n' "$body"
}

emit_block() {
    local gate="$1" couples trigger mode gen relpath
    couples="$(manifest_field "$gate" couples)"
    trigger="$(manifest_field "$gate" trigger)"; trigger="${trigger:-$couples}"
    trigger="$(gate_expand_couples "$trigger")"
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
        printf '%s\n' "run_gate $gate $relpath"
    elif [[ "$mode" == staged ]]; then
        printf 'mapfile -t _staged < <(git diff --cached --name-only --diff-filter=ACMR -- %s)\n' "$quoted"
        printf '%s\n' "_targets=()"
        printf '%s\n' 'for _f in "${_staged[@]}"; do [[ -f "$_f" ]] && _targets+=("$_f"); done'
        printf '%s\n' 'if [[ ${#_targets[@]} -gt 0 ]]; then'
        printf '    run_gate %s %s "${_targets[@]}"\n' "$gate" "$relpath"
        printf '%s\n' 'fi'
    else
        printf 'if staged_matches %s; then\n' "$quoted"
        printf '    run_gate %s %s\n' "$gate" "$relpath"
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
HEAD
    emit_staged_matcher_body
    cat <<'HEAD'
}

# Uniform failure: the captured output was already reprinted above.
hook_fail() {
    echo ""
    echo "pre-commit: $1 failed (see above)."
    echo "  Bypass once (use sparingly): git commit --no-verify"
    exit 1
}

GATE_SDK_VERBOSE="${GATE_SDK_VERBOSE:-}"
_ran=0
# Capture a gate's output; reprint it only on failure, or with GATE_SDK_VERBOSE.
run_gate() {
    local name="$1"; shift
    local out ok=1
    out="$("$@" 2>&1)" || ok=0
    _ran=$((_ran + 1))
    if (( ! ok )); then
        [[ -n "$out" ]] && printf '%s\n' "$out"
        hook_fail "$name"
    fi
    if [[ -n "$GATE_SDK_VERBOSE" ]]; then
        [[ -n "$out" ]] && printf '%s\n' "$out"
        printf '  PASS: %s\n' "$name"
    fi
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

    cat <<'TAIL'

printf 'pre-commit: %d gate(s) passed.\n' "$_ran"
exit 0
TAIL
}

# spec: gate-sdk/SPEC.md §gen-pre-commit — the commit-msg surface: every tier=commit-msg gate becomes one unconditional invocation passing the hook's $1 (message path)
commit_msg_gates() {
    local c tier
    local -A seen=()
    for c in "${CHECKS[@]}"; do
        [[ -n "${seen[$c]+x}" ]] && continue
        seen[$c]=1
        tier="$(manifest_field "$c" tier)"
        [[ "$tier" == commit-msg ]] && printf '%s\n' "$c"
    done
}

emit_commit_msg() {
    cat <<'HEAD'
#!/usr/bin/env bash
# commit-msg - GENERATED, DO NOT EDIT.
#
# Emitted from the tier=commit-msg `# graph:` manifests by:
#     bash gate-sdk/bin/gen-pre-commit.sh --write
# Edit a gate's manifest, then regenerate. check-graph asserts this file equals
# --emit-commit-msg. git feeds the prospective message file as $1; each gate
# prints its own per-finding + `help:` lines before this hook reports failure.
#
# Install (opt-in, per clone):   bash gate-sdk/bin/install-hooks.sh
# Bypass once (use sparingly):   git commit --no-verify
set -euo pipefail

msg_file="${1:?commit-msg: git did not pass the message-file path}"

# Uniform failure: the captured output was already reprinted above.
hook_fail() {
    echo ""
    echo "commit-msg: $1 failed (see above)."
    echo "  Bypass once (use sparingly): git commit --no-verify"
    exit 1
}

GATE_SDK_VERBOSE="${GATE_SDK_VERBOSE:-}"
_ran=0
# Capture a gate's output; reprint it only on failure, or with GATE_SDK_VERBOSE.
run_gate() {
    local name="$1"; shift
    local out ok=1
    out="$("$@" 2>&1)" || ok=0
    _ran=$((_ran + 1))
    if (( ! ok )); then
        [[ -n "$out" ]] && printf '%s\n' "$out"
        hook_fail "$name"
    fi
    if [[ -n "$GATE_SDK_VERBOSE" ]]; then
        [[ -n "$out" ]] && printf '%s\n' "$out"
        printf '  PASS: %s\n' "$name"
    fi
}
HEAD

    local c relpath
    while IFS= read -r c; do
        [[ -n "$c" ]] || continue
        relpath="$(resolve_rel "$c")" || relpath="$GATES_DIR/$c.sh"
        printf '\nrun_gate %s %s "$msg_file"\n' "$c" "$relpath"
    done < <(commit_msg_gates)

    cat <<'TAIL'

printf 'commit-msg: %d gate(s) passed.\n' "$_ran"
exit 0
TAIL
}

case "${1:-}" in
    --emit) emit ;;
    --emit-commit-msg) emit_commit_msg ;;
    --write)
        mkdir -p "$HOOKS_DIR"
        emit > "$HOOK"
        chmod +x "$HOOK"
        echo "gen-pre-commit: wrote $HOOK"
        if [[ -n "$(commit_msg_gates)" ]]; then
            emit_commit_msg > "$MSG_HOOK"
            chmod +x "$MSG_HOOK"
            echo "gen-pre-commit: wrote $MSG_HOOK"
        fi
        ;;
    *) echo "usage: gen-pre-commit.sh --emit|--emit-commit-msg|--write" >&2; exit 2 ;;
esac
