#!/usr/bin/env bash
# spec: installer/README.md §uninstall — reverses an install against the roster init recorded and nothing else: an entry whose content is still what init wrote is removed, an entry you have edited since is kept and reported, and the manifest is narrowed over the survivors rather than deleted, so ownership of a file that is still on disk never lapses
#
# usage: checkwright uninstall [--dry-run] [--force] [--no-commit]
#   Each flag is described by --help.
set -uo pipefail

INSTALLER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PAYLOAD="$INSTALLER/payload"
# shellcheck source=./common/lock.sh
source "$INSTALLER/lib/common/lock.sh"
# spec: installer/README.md §What init seeds — sourced for the consumer-layout names, so the agent file this verb trims is the one init wrote rather than a second spelling of the path
# shellcheck source=./common/recipe.sh
source "$INSTALLER/lib/common/recipe.sh"

DRY=0; FORCE=0; DO_COMMIT=1
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY=1; shift ;;
        --force) FORCE=1; shift ;;
        --no-commit) DO_COMMIT=0; shift ;;
        -h|--help)
            printf 'usage: checkwright uninstall [--dry-run] [--force] [--no-commit]\n\n'
            printf 'Removes the files init recorded in %s and commits the removal.\n' "$CHECKWRIGHT_LOCK_FILE"
            printf 'Nothing outside that roster is touched. A file you have edited since\n'
            printf 'init wrote it is kept and reported; --force removes it anyway.\n'
            exit 0 ;;
        *) printf 'checkwright uninstall: unknown argument: %s\n' "$1" >&2; exit 2 ;;
    esac
done

die() { printf 'checkwright uninstall: %s\n' "$1" >&2; [[ -n "${2:-}" ]] && printf '  help: %s\n' "$2" >&2; exit "${3:-2}"; }

# spec: installer/README.md §init — every precondition refuses rather than warns and all of them are checked before anything is removed, for init's own reason: a partial removal is the outcome none of them may produce
ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
    || die "not inside a git work tree" \
       "uninstall stages and commits the removal the same way init committed the install, so it needs the repository it is reversing."

LOCK="$(lock_path "$ROOT")"
[[ -f "$LOCK" ]] || die "no $CHECKWRIGHT_LOCK_FILE at $ROOT" \
    "init is the verb that makes an install, and the manifest it writes is the only record of which files are this installer's to remove. Without one there is nothing here to reverse."
lock_schema_ok "$LOCK" || die "$CHECKWRIGHT_LOCK_FILE carries a schema this build does not know" \
    "this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for."

# spec: installer/README.md §init — the clean-worktree precondition is init's, on the same terms: one commit is made, and a dirty tree would fold your work into it; --no-commit is the same valve
if (( ! DRY )) && (( DO_COMMIT )) && [[ -n "$(git -C "$ROOT" status --porcelain)" ]]; then
    die "the worktree is not clean" \
        "uninstall makes one commit, and a dirty tree would fold your work into it — so a reviewer's diff would no longer be the whole of what was removed. Commit or stash first, or pass --no-commit to stage the removal yourself." 1
fi

PROFILE="$(lock_field "$LOCK" profile)"
VERSION="$(lock_field "$LOCK" version)"
KITS=()
read -r -a KITS <<<"$(lock_field "$LOCK" kits)"
GATES_LIST="$(lock_own_file "$LOCK" "$GATES_DIR/gates.list")"

kits_include() {   # $1 = kit name -> 0 iff the manifest records it as vendored
    local k
    for k in ${KITS[@]+"${KITS[@]}"}; do [[ "$k" == "$1" ]] && return 0; done
    return 1
}

# spec: installer/README.md §uninstall — the removal rule is claim() seen from the other side, and it needs no new data: init records each path at the hash it last wrote there, so a hash that still matches marks a file that is init's to remove and one that differs marks a file that is yours to keep. A recorded path already off the tree is a no-op rather than an error — it left the roster the moment it left the tree
REMOVE=(); KEEP=(); GONE=()
declare -A KEPT_HASH=(); declare -A ROSTER=()
while IFS=$'\t' read -r p h; do
    [[ -n "$p" ]] || continue
    ROSTER["$p"]=1
    if [[ ! -f "$ROOT/$p" ]]; then GONE+=("$p"); continue; fi
    if (( FORCE )) || [[ "$(lock_hash "$ROOT/$p")" == "$h" ]]; then
        REMOVE+=("$p")
    else
        KEEP+=("$p"); KEPT_HASH["$p"]="$h"
    fi
done < <(jq -r 'if (.files | type) == "object" then (.files | to_entries[] | "\(.key)\t\(.value)") else empty end' "$LOCK")

# spec: installer/README.md §uninstall — the agent file is the one entry that is a span rather than a file, so the branch where it is kept still owes the doctrine block a removal: the block is prose you did not write, in the file whose purpose is to steer agent sessions, pointing at a doctrine file this verb just removed. The trim is scoped to that span alone — a marker block init never wrote is never a files[] entry, so it is never this verb's to touch
TRIM_AGENT=0
DOCTRINE_REMOVER="$PAYLOAD/doctrine-kit/bin/install-doctrine.sh"
if [[ -n "${KEPT_HASH[$AGENT_FILE]:-}" ]] && kits_include doctrine-kit; then
    TRIM_AGENT=1
    [[ -f "$DOCTRINE_REMOVER" ]] || die "this package carries no payload copy of doctrine-kit's installer" \
        "the doctrine block is trimmed through the kit's own installer so the removal path adds no second copy of the marker strings, and this run needs it because $AGENT_FILE is being kept. Run uninstall from an installed package rather than a source checkout."
fi

# spec: installer/README.md §uninstall — the hook opt-in is reported, not rewritten: git config is outside the ownership roster, and a core.hooksPath naming a directory that no longer exists is inert rather than breaking, so there is nothing to justify writing outside the contract. The resolver needs no kits guard: it asks the manifest for the exact path init writes, which a residual manifest simply does not record
HOOKS_LINE=""
hooks_path="$(git -C "$ROOT" config --get core.hooksPath 2>/dev/null)"
if [[ -n "$hooks_path" && -n "$GATES_LIST" && "$GATES_LIST" == */* ]]; then
    gates_dir="${GATES_LIST%/*}"
    rel="${hooks_path#"$ROOT"/}"
    [[ "$rel" == "$gates_dir" || "$rel" == "$gates_dir"/* ]] \
        && HOOKS_LINE="git config --unset core.hooksPath"
fi

# spec: installer/README.md §The manifest — the survivors carry the hash init recorded, never the tree's: an entry rewritten at your hash would read as unchanged on the next install and let it write straight through you, which is the one defect the residual manifest exists to prevent
residual() {
    local p
    for p in ${KEEP[@]+"${KEEP[@]}"}; do printf '%s\t%s\n' "$p" "${KEPT_HASH[$p]}"; done | lock_emit
}

ancestors() {   # $1 = repo-relative path -> each of its directories, deepest first
    local d="$1"
    while [[ "$d" == */* ]]; do d="${d%/*}"; printf '%s\n' "$d"; done
}

# spec: installer/README.md §uninstall — a file you added inside a vendored directory is not on the roster, so it is never removed; the plan names it because a directory left behind holding only your own files is a surprise worth spending a line on before the run rather than after
adopter_added() {
    local kit f
    for kit in ${KITS[@]+"${KITS[@]}"}; do
        [[ -d "$ROOT/$kit" ]] || continue
        while IFS= read -r f; do
            [[ -n "$f" && -z "${ROSTER[$f]:-}" ]] && printf '%s\n' "$f"
        done < <(cd "$ROOT" && find "$kit" -type f | LC_ALL=C sort)
    done
}

by_top_dir() {   # repo-relative paths on stdin -> '<top-level dir or file> <count>' lines
    awk -F/ '{ print (NF > 1 ? $1 "/" : $0) }' | LC_ALL=C sort | uniq -c \
        | awk '{ printf "  %-32s %d file(s)\n", $2, $1 }'
}

# spec: installer/README.md §uninstall — a run with nothing to remove says so and exits 0: the install is still there, so narrowing the manifest here would disown an install that has not ended
if [[ ${#REMOVE[@]} -eq 0 ]]; then
    printf 'UNINSTALL: nothing to remove — of %d recorded file(s), %d have changed since init wrote them and %d are already gone.\n' \
        "$(( ${#KEEP[@]} + ${#GONE[@]} ))" "${#KEEP[@]}" "${#GONE[@]}"
    [[ ${#KEEP[@]} -gt 0 ]] && printf '  help: they are yours, so uninstall leaves them. Pass --force to remove them anyway.\n'
    exit 0
fi

if (( DRY )); then
    printf 'checkwright uninstall --dry-run (profile: %s, version: %s)\n\n' "$PROFILE" "$VERSION"
    printf 'would remove %d file(s):\n' "${#REMOVE[@]}"
    printf '%s\n' "${REMOVE[@]}" | by_top_dir
    printf 'would then remove every directory those files empty.\n'
    if [[ ${#KEEP[@]} -gt 0 ]]; then
        printf '\nwould keep %d file(s) you have changed since init wrote them (--force to remove them anyway):\n' "${#KEEP[@]}"
        printf '  %s\n' "${KEEP[@]}"
    fi
    [[ ${#GONE[@]} -gt 0 ]] && printf '\n%d recorded file(s) are already off the tree — nothing to do for them.\n' "${#GONE[@]}"
    added="$(adopter_added)"
    if [[ -n "$added" ]]; then
        printf '\nwould leave %d file(s) you added inside a vendored directory:\n' "$(grep -c . <<<"$added")"
        while IFS= read -r f; do printf '  %s\n' "$f"; done <<<"$added"
    fi
    (( TRIM_AGENT )) && printf '\nwould trim the doctrine block out of %s, which is being kept, and leave the rest of it alone.\n' "$AGENT_FILE"
    if [[ ${#KEEP[@]} -eq 0 ]]; then
        printf '\nwould delete %s — nothing recorded would survive.\n' "$CHECKWRIGHT_LOCK_FILE"
    else
        printf '\nwould rewrite %s over the %d kept file(s), at the hashes init recorded:\n' "$CHECKWRIGHT_LOCK_FILE" "${#KEEP[@]}"
        residual
    fi
    [[ -n "$HOOKS_LINE" ]] && printf '\nwould print this for you to run yourself:\n  %s\n' "$HOOKS_LINE"
    printf '\nDRY RUN: nothing was written.\n'
    exit 0
fi

for p in "${REMOVE[@]}"; do rm -f "$ROOT/$p" || die "could not remove $p"; done

# spec: installer/README.md §uninstall — pruning is bottom-up and only ever removes a directory that is now empty: uninstall removes files it owns, never directories it merely emptied around, so one left holding anything at all is left alone
while IFS= read -r d; do
    rmdir "$ROOT/$d" 2>/dev/null
done < <(for p in "${REMOVE[@]}"; do ancestors "$p"; done | LC_ALL=C sort -ru)

TRIMMED=0
if (( TRIM_AGENT )); then
    # spec: doctrine-kit/SPEC.md §install-doctrine — the trim runs through the payload's copy of the kit's own installer, exactly as init's injection does, so the marker strings keep their one writer and the removal works once the vendored kit is gone
    trim_out="$( cd "$ROOT" && bash "$DOCTRINE_REMOVER" --remove "$AGENT_FILE" 2>&1 )" \
        || die "the doctrine block could not be trimmed out of $AGENT_FILE: $trim_out"
    TRIMMED=1
fi

if [[ ${#KEEP[@]} -eq 0 ]]; then
    rm -f "$LOCK" || die "could not remove $CHECKWRIGHT_LOCK_FILE"
elif residual > "$LOCK.tmp"; then
    mv "$LOCK.tmp" "$LOCK" || die "could not rewrite $CHECKWRIGHT_LOCK_FILE"
else
    rm -f "$LOCK.tmp"
    die "could not rewrite $CHECKWRIGHT_LOCK_FILE over the kept file(s)"
fi

# spec: installer/README.md §uninstall — the staged set is the removals and the manifest disposition, and a kept file is never among them: staging a file left for the adopter is the same defect init's written-set/roster split exists to prevent, and it stays one when the write is a removal
STAGE=()
while IFS= read -r -d '' p; do STAGE+=("$p"); done < <(git -C "$ROOT" ls-files -z -- "${REMOVE[@]}")
if [[ -f "$LOCK" ]] || git -C "$ROOT" ls-files --error-unmatch -- "$CHECKWRIGHT_LOCK_FILE" >/dev/null 2>&1; then
    STAGE+=("$CHECKWRIGHT_LOCK_FILE")
fi
[[ ${#STAGE[@]} -gt 0 ]] && { git -C "$ROOT" add -- "${STAGE[@]}" || die "could not stage the removal"; }

if [[ ${#KEEP[@]} -gt 0 ]]; then
    printf '\n%d file(s) have changed since init wrote them and were kept:\n' "${#KEEP[@]}"
    printf '  %s\n' "${KEEP[@]}"
    printf '  help: they are yours. %s now records those paths and nothing else, at the hashes init wrote, so a future init still protects them rather than writing through them.\n' \
        "$CHECKWRIGHT_LOCK_FILE"
fi
(( TRIMMED )) && printf '\ntrimmed the doctrine block out of %s and left the change unstaged — the rest of that file is yours to review.\n' "$AGENT_FILE"

if git -C "$ROOT" diff --cached --quiet; then
    printf '\nUNINSTALL: removed %d file(s), none of which this repository was tracking — nothing to commit.\n' "${#REMOVE[@]}"
else
    if (( DO_COMMIT )); then
        git -C "$ROOT" commit -q -m "chore: remove Checkwright kits ($PROFILE, v$VERSION)" \
            || die "the commit failed — the removal is staged; commit it yourself to finish" "" 1
        printf '\nUNINSTALL: removed %d file(s), kept %d, and committed the removal.\n' "${#REMOVE[@]}" "${#KEEP[@]}"
    else
        printf '\nUNINSTALL: removed %d file(s), kept %d and staged the removal — --no-commit, so the commit is yours.\n' \
            "${#REMOVE[@]}" "${#KEEP[@]}"
    fi
fi

[[ -n "$HOOKS_LINE" ]] && printf '\nthis clone still points at the hooks directory that was just removed. Undo that yourself with:\n  %s\n' "$HOOKS_LINE"
exit 0
