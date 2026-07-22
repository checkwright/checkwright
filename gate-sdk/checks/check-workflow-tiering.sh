#!/usr/bin/env bash
# graph: couples=.workflow/*,.gitignore dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §The workflow directory — every workflow-dir member is tracked or ignored, and every tracked member opens with a '# contract: ' header carrying a ruled payload
#
# usage: check-workflow-tiering.sh [scan-root]   (default '.')
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-workflow-tiering: not a directory: $ROOT" >&2; exit 2; }
git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-workflow-tiering: $ROOT is not a git repository — the tracked/ignored partition is unreadable" >&2; exit 2; }

WF="${GATE_SDK_WORKFLOW_DIR:-.workflow}"
[[ -d "$ROOT/$WF" ]] || {
    echo "check-workflow-tiering: workflow directory not found: $ROOT/$WF" >&2; exit 2; }

# spec: gate-sdk/SPEC.md §The workflow directory — the two ruled payload forms
POINTER_RE='^[A-Za-z0-9._/-]+\.md[[:space:]]+§[^[:space:]]'
VERSION_RE='^[a-z0-9-]+ v[0-9]+$'

errors=()
tracked=0
ignored=0

shopt -s nullglob dotglob
members=("$ROOT/$WF"/*)
shopt -u nullglob dotglob

for m in "${members[@]}"; do
    rel="${m#"$ROOT"/}"
    is_tracked=0
    if [[ -d "$m" ]]; then
        listing="$(git -C "$ROOT" ls-files -- "$rel")"; st=$?
        fail_closed "$st" WORKFLOW-TIERING "git ls-files ($rel)"
        [[ -n "$listing" ]] && is_tracked=1
    else
        git -C "$ROOT" ls-files --error-unmatch -- "$rel" >/dev/null 2>&1 && is_tracked=1
    fi

    is_ignored=0
    git -C "$ROOT" check-ignore -q -- "$rel"; st=$?
    case "$st" in
        0) is_ignored=1 ;;
        1) ;;
        *) echo "check-workflow-tiering: git check-ignore exited $st on $rel" >&2; exit 2 ;;
    esac

    # assertion A: partition totality
    if [[ "$is_tracked" == 0 && "$is_ignored" == 0 ]]; then
        errors+=("$rel: neither tracked nor ignored — no tier holds it")
        continue
    fi
    if [[ "$is_tracked" == 0 ]]; then ignored=$((ignored + 1)); continue; fi
    tracked=$((tracked + 1))
    [[ -d "$m" ]] && continue

    # assertion B: header presence and ruled payload form
    [[ -r "$m" ]] || { echo "check-workflow-tiering: member not readable: $rel" >&2; exit 2; }
    first=""
    IFS= read -r first < "$m" || true
    if [[ "$first" != "# contract: "* ]]; then
        errors+=("$rel: tracked member's first line is not a '# contract: ' header: ${first:0:60}")
        continue
    fi
    payload="${first#"# contract: "}"
    sig="${payload%% — *}"; sig="${sig%% -- *}"
    sig="${sig%"${sig##*[![:space:]]}"}"
    if [[ ! "$sig" =~ $POINTER_RE ]] && [[ ! "$sig" =~ $VERSION_RE ]]; then
        errors+=("$rel: '# contract: ' payload is neither '<owner-path>.md §<section>' nor '<format-name> v<N>': $sig")
    fi
done

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "check-workflow-tiering: ${#errors[@]} workflow-surface violation(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: every workflow-dir member belongs to one of two tiers — a tracked checked projection or a gitignored local capture; add the member to .gitignore or commit it. A tracked member's first line is '# contract: <owner-path>.md §<section>' (optionally ' — <grammar or gloss>'), or '# contract: <format-name> v<N>' where a gate parses the header as a wire-format version and its owning SPEC says so."
    exit 1
fi
echo "WORKFLOW-TIERING: clean ($tracked tracked projection(s) with a ruled '# contract: ' header, $ignored capture member(s); every member holds a tier)"
exit 0
