#!/usr/bin/env bash
# graph: couples=kit:checks/*.gate,scripts/gates.list,native/* dir=one valve=none tier=precommit
# install: never
# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — whenever a registered member resolving to a .gate descriptor makes the binary load-bearing, the binary was built from the source now in the tree
#
# usage: check-gate-binary-fresh.sh [gates-dir] [tree-stamp-file]
#   No second argument: computes the tree-side stamp from the crate's tracked
#   source with git. With one: reads it from that file (hermetic fixtures).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR="${1:-$(gate_sdk_gates_dir)}"
STAMP_FILE="${2:-}"
LIST="$GATES_DIR/gates.list"
BIN="$(gate_native_bin)"
CRATE="$(gate_native_crate)"
REBUILD="cargo build --release --manifest-path $CRATE/Cargo.toml"

RESOLVE_DIRS=("$GATES_DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots_rel)

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the descriptor set is the trigger, derived over the resolve dirs as check-gate-substrate-parity assertion B already derives it
mapfile -t DESCRIPTORS < <(
    for d in "${RESOLVE_DIRS[@]}"; do
        [[ -d "$d" ]] || continue
        for f in "$d"/*.gate; do
            [[ -f "$f" ]] || continue
            b="${f##*/}"; printf '%s\n' "${b%.gate}"
        done
    done | sort -u
)

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a descriptor on disk is a declaration; a
# registered member resolving to one is a dispatch, and only a dispatch makes the binary
# load-bearing. The live registry is therefore an input, and an absent one is "cannot verify".
[[ -f "$LIST" ]] || {
    echo "check-gate-binary-fresh: no gate registry at $LIST — the live member set is what decides whether the binary is load-bearing, so the check could not run; treating as failure (not clean)" >&2
    echo "  help: pass the gates dir carrying gates.list as the first argument, or set GATE_SDK_GATES_DIR." >&2
    exit 2
}
mapfile -t DISPATCHING < <(
    while IFS= read -r m; do
        [[ -n "$m" ]] || continue
        src="$(gate_resolve "$m" "${RESOLVE_DIRS[@]}")" || continue
        [[ "$src" == *.gate ]] && printf '%s\n' "$m"
    done < <(gates_list_members "$LIST")
)

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — nothing dispatching is a clean report, not a
# skipped assertion: no gate dispatches to the binary, so nothing can run stale. Both counts are
# named, so a reader can tell "no descriptors" from "descriptors nothing dispatches to".
# Deliberately *not* the split-halves shape check-gate-substrate-parity assertion B uses, for
# the reason that section states.
if [[ ${#DISPATCHING[@]} -eq 0 ]]; then
    echo "GATE-BINARY-FRESH: clean (${#DESCRIPTORS[@]} .gate descriptor(s) across ${#RESOLVE_DIRS[@]} resolve dir(s), 0 dispatched to by a live member of $LIST, so nothing dispatches to $BIN and no build can be stale; crate $CRATE unread)"
    exit 0
fi

# spec: gate-sdk/SPEC.md §Fail-closed contract — with the binary load-bearing, an absent
# or unreadable one is "cannot verify", which must not share an exit code with "verified fresh"
if [[ ! -x "$BIN" ]]; then
    echo "check-gate-binary-fresh: $BIN is absent or not executable, but ${#DISPATCHING[@]} registered member(s) dispatch to it — the check could not run; treating as failure (not clean)" >&2
    echo "  help: build it — $REBUILD" >&2
    exit 2
fi

baked="$("$BIN" --source-stamp 2>/dev/null)"; st=$?
fail_closed "$st" check-gate-binary-fresh "$BIN --source-stamp"
baked="${baked%%$'\n'*}"
[[ -n "$baked" ]] || {
    echo "check-gate-binary-fresh: $BIN --source-stamp reported no stamp — the check could not run; treating as failure (not clean)" >&2
    echo "  help: rebuild it — $REBUILD" >&2
    exit 2
}

if [[ -n "$STAMP_FILE" ]]; then
    [[ -r "$STAMP_FILE" ]] || { echo "check-gate-binary-fresh: tree-stamp file not readable: $STAMP_FILE" >&2; exit 2; }
    tree="$(cat -- "$STAMP_FILE")"; st=$?
    fail_closed "$st" check-gate-binary-fresh cat
    tree="${tree%%$'\n'*}"
    source_desc="$STAMP_FILE"
else
    tree="$(gate_native_source_stamp)" || {
        echo "check-gate-binary-fresh: git could not hash the tracked source under $CRATE — the check could not run; treating as failure (not clean)" >&2
        echo "  help: the stamp is git's content identity for the crate's tracked source set, so the crate root must be a tracked directory inside a git worktree." >&2
        exit 2
    }
    source_desc="$CRATE"
fi
[[ -n "$tree" ]] || { echo "check-gate-binary-fresh: no tree-side stamp from $source_desc" >&2; exit 2; }

if [[ "$baked" != "$tree" ]]; then
    echo "check-gate-binary-fresh: the gate binary was not built from the source now in the tree:"
    echo "  $BIN reports source stamp $baked"
    echo "  $source_desc hashes to $tree"
    echo "  ${#DISPATCHING[@]} descriptor(s) dispatch to that binary: ${DISPATCHING[*]}"
    echo "  help: rebuild it — $REBUILD"
    echo "        Until then the descriptor-named gate(s) above run the old implementation"
    echo "        and pass on code that is not what is committed."
    exit 1
fi

echo "GATE-BINARY-FRESH: clean (${#DESCRIPTORS[@]} .gate descriptor(s) across ${#RESOLVE_DIRS[@]} resolve dir(s), ${#DISPATCHING[@]} dispatched to by a live member of $LIST; $BIN built from the source now in $source_desc, stamp $baked)"
exit 0
