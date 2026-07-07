#!/usr/bin/env bash
# graph: couples=gate-sdk/*.sh,lifecycle-kit/*.sh,queue-kit/*.sh,spec-kit/*.sh,delegation-kit/*.sh dir=one valve=none tier=precommit
# spec: delegation-kit/SPEC.md §check-gate-tamper — a gate-weakening commit is blocked by shape (A gate edits stay meta-isolated; B a new path-exemption can't excuse a co-staged file)
#
# Extracted from the governance meta-layer of a private production platform; the
# gate-file roster and meta-layer prefixes are config arrays (lib/delegation.sh),
# platform layout as defaults. --fixture <dir> injects staged-files /
# added-exemptions lists; live mode reads git diff --cached.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/delegation.sh
source "$KIT/lib/delegation.sh"

is_meta() {
    local p="$1" pre
    for pre in "${DELEGATION_KIT_META_PATHS[@]}"; do
        [[ "$p" == "$pre"* ]] && return 0
    done
    [[ "$p" == *.md && "$p" != */* ]] && return 0  # root-level *.md is always meta
    return 1
}

is_gate_file() {
    local p="$1" g
    for g in "${DELEGATION_KIT_GATE_FILES[@]}"; do
        # shellcheck disable=SC2053  # $g is a glob pattern by design here
        [[ "$p" == $g ]] && return 0
    done
    return 1
}

is_pathlike() {
    case "$1" in
        */* | *'*'* | *'?'* | *'['*) return 0 ;;
        *) return 1 ;;
    esac
}

extract_exemptions() {
    awk '
        /^[[:space:]]*#[[:space:]]*exception-list:/ { tag=1; next }
        tag==1 && /^[[:space:]]*[A-Za-z_][A-Za-z0-9_]*=\(/ {
            tag=0; inarr=1
            line=$0; sub(/#.*/,"",line)
            ob=gsub(/\(/,"",line); cb=gsub(/\)/,"",line)
            bal=ob-cb
            if (bal<=0) inarr=0
            next
        }
        inarr==1 {
            line=$0; sub(/#.*/,"",line)
            ob=gsub(/\(/,"",line); cb=gsub(/\)/,"",line)
            v=line; sub(/^[[:space:]]+/,"",v); sub(/[[:space:]].*$/,"",v)
            gsub(/\047/,"",v); gsub(/"/,"",v)
            if (v != "" && v !~ /^[)(]/) print v
            bal += ob - cb
            if (bal<=0) inarr=0
        }
    '
}

declare -a STAGED=() ADDED=()

collect_live() {
    mapfile -t STAGED < <(git diff --cached --name-only 2>/dev/null)
    local f new old v
    declare -A added_set=()
    for f in "${STAGED[@]}"; do
        is_gate_file "$f" || continue
        new="$(git show ":$f" 2>/dev/null)" || new=""
        old="$(git show "HEAD:$f" 2>/dev/null)" || old=""
        declare -A old_set=()
        while IFS= read -r v; do [[ -n "$v" ]] && old_set["$v"]=1; done \
            < <(printf '%s\n' "$old" | extract_exemptions)
        while IFS= read -r v; do
            [[ -n "$v" && -z "${old_set[$v]:-}" ]] && added_set["$v"]=1
        done < <(printf '%s\n' "$new" | extract_exemptions)
        unset old_set
    done
    ADDED=("${!added_set[@]}")
}

collect_fixture() {
    local dir="$1"
    [[ -d "$dir" ]] || { echo "check-gate-tamper: fixture dir not found: $dir" >&2; exit 2; }
    [[ -f "$dir/staged-files" ]] &&
        mapfile -t STAGED < <(grep -vE '^[[:space:]]*(#|$)' "$dir/staged-files")
    [[ -f "$dir/added-exemptions" ]] &&
        mapfile -t ADDED < <(grep -vE '^[[:space:]]*(#|$)' "$dir/added-exemptions")
}

MODE=live
FIXTURE_DIR=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --fixture) MODE=fixture; FIXTURE_DIR="${2:-}"; shift 2 ;;
        *) echo "check-gate-tamper: unknown argument: $1" >&2; exit 2 ;;
    esac
done

if [[ "$MODE" == fixture ]]; then
    collect_fixture "$FIXTURE_DIR"
else
    collect_live
fi

viol_a=()
viol_b=()

# assertion A: gate-edit-isolation
gate_touched=0
for f in "${STAGED[@]}"; do is_gate_file "$f" && { gate_touched=1; break; }; done
if [[ "$gate_touched" -eq 1 ]]; then
    for f in "${STAGED[@]}"; do is_meta "$f" || viol_a+=("$f"); done
fi

# assertion B: no-self-serving-exemption
for e in "${ADDED[@]}"; do
    is_pathlike "$e" || continue
    for f in "${STAGED[@]}"; do
        # shellcheck disable=SC2053  # $e is a glob pattern by design here
        [[ "$f" == $e ]] && viol_b+=("$e -> $f")
    done
done

if [[ ${#viol_a[@]} -gt 0 || ${#viol_b[@]} -gt 0 ]]; then
    if [[ ${#viol_a[@]} -gt 0 ]]; then
        echo "check-gate-tamper: gate edit not isolated — a commit touching a gate file (${DELEGATION_KIT_GATE_FILES[*]}) may touch only meta-layer paths; these co-staged paths are not:"
        printf '  %s\n' "${viol_a[@]}"
        echo "  help: split the gate change into its own commit (meta-layer paths only: ${DELEGATION_KIT_META_PATHS[*]} and root *.md); land the product change in a separate commit"
    fi
    if [[ ${#viol_b[@]} -gt 0 ]]; then
        echo "check-gate-tamper: self-serving exemption — a newly added path/glob exemption matches a file staged in the same commit:"
        printf '  %s\n' "${viol_b[@]}"
        echo "  help: an exemption must not excuse the very change it lands with; add the exemption in a separate commit, or drop the matched file from this commit"
    fi
    exit 1
fi

echo "GATE-TAMPER: clean (${#STAGED[@]} staged path(s); gate edits meta-isolated, no self-serving path-exemption)"
exit 0
