#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The survey record — the capture affordance; stamps the block grammar, no caller-side redirect (the file-gap.sh pattern)
# usage: file-survey.sh "<question>" "<corpus>" "<oracle>" "<edges>" "<finding>"   (five, each non-empty)
#   appends one block '## <YYYY-MM-DD> <stage> — <question>' plus its corpus/oracle/rev/edges/finding
#   lines to the committed survey record; exit 2 on misuse
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 1

usage() {
    printf 'usage: %s [-h|--help] [--] "<question>" "<corpus>" "<oracle>" "<edges>" "<finding>"\n' "$(basename "$0")"
    printf '  appends one dated block to %s; "--" files a field beginning with "-"\n' \
        "$LIFECYCLE_KIT_SURVEY_RECORD_FILE"
}

# spec: gate-sdk/SPEC.md §The bin/-tool contract — free-text positionals validate shape, not only arity: the refusal scans every positional, since arity alone leaves a flag safe in no slot but the first
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
fi
if [[ "${1:-}" == "--" ]]; then
    shift
else
    for _fs_arg in "$@"; do
        [[ "$_fs_arg" == -* ]] || continue
        printf '%s: unrecognized option: %s — a field beginning with "-" is filed after a "--" separator\n' \
            "$(basename "$0")" "$_fs_arg" >&2
        usage >&2
        exit 2
    done
fi

# spec: lifecycle-kit/SPEC.md §The survey record — the edges slot takes no default: an omitted fifth argument is the arity misuse the tool already refuses, so a session that forgot the field is told at filing time rather than at commit time
if [[ $# -ne 5 || -z "${1:-}" || -z "${2:-}" || -z "${3:-}" || -z "${4:-}" || -z "${5:-}" ]]; then
    usage >&2
    exit 2
fi

# spec: lifecycle-kit/SPEC.md §The survey record — rev is machine-stamped because it is the field the whole re-use protocol turns on and the one an author gets wrong (a short sha, the rev they started at, or none at all); a tree with no HEAD cannot ground a witness, so it is a refusal rather than a blank field
if ! _fs_rev="$(git rev-parse HEAD 2>/dev/null)" || [[ ! "$_fs_rev" =~ ^[0-9a-f]{40}$ ]]; then
    printf 'file-survey: no HEAD commit to stamp as the survey rev — the witness would have nothing to diff against; commit first.\n' >&2
    exit 2
fi

RECORD="$LIFECYCLE_KIT_SURVEY_RECORD_FILE"
mkdir -p "$(dirname "$RECORD")" 2>/dev/null || true
# spec: lifecycle-kit/SPEC.md §The survey record — seed the contract header when the record does not yet exist (a fresh consumer's first filing); the iteration-boundary entry truncates back to this header
[[ -f "$RECORD" ]] \
    || printf '# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.\n' > "$RECORD"

# spec: lifecycle-kit/SPEC.md §The survey record — the stage is derived from the cursor rather than asked for; a tree with no cursor yet stamps the never-named form the queue header already uses
_fs_stage="$(lifecycle_current_stage)"
[[ -n "$_fs_stage" ]] || _fs_stage="—"

printf '\n## %s %s — %s\n- corpus: %s\n- oracle: %s\n- rev: %s\n- edges: %s\n- finding: %s\n' \
    "$(date +%F)" "$_fs_stage" "$1" "$2" "$3" "$_fs_rev" "$4" "$5" >> "$RECORD"

printf 'file-survey: ## %s %s — %s (rev %s)\n' "$(date +%F)" "$_fs_stage" "$1" "$_fs_rev"

if [[ "$3" == "none" ]]; then
    printf 'file-survey: oracle "none" — this block is a note, not a re-usable survey: a later stage may read it for orientation and must re-derive before relying on it.\n' >&2
else
    printf 'file-survey: the witness a later stage runs — git diff --quiet %s..HEAD -- %s, then re-run: %s\n' \
        "$_fs_rev" "$2" "$3" >&2
fi
