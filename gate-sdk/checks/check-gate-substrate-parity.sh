#!/usr/bin/env bash
# graph: couples=scripts/gates.list,kit:checks/*.sh,kit:checks/*.gate,gate-sdk/SPEC.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — one declaration per member, descriptor/subcommand parity both ways, and a recorded disposition for every substrate-sensitive member
#
# usage: check-gate-substrate-parity.sh [gates-dir] [conservation-doc]
#   two args: steer onto hermetic fixture copies of each surface.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR="${1:-$(gate_sdk_gates_dir)}"
DOC="${2:-$SDK/SPEC.md}"
LIST="$GATES_DIR/gates.list"
SECTION="## Meta-gate conservation for the binary substrate"

[[ -f "$LIST" ]] || { echo "check-gate-substrate-parity: no registry at $LIST" >&2; exit 2; }
[[ -f "$DOC" ]] || { echo "check-gate-substrate-parity: conservation doc not found: $DOC" >&2; exit 2; }

mapfile -t MEMBERS < <(gates_list_members "$LIST")
[[ ${#MEMBERS[@]} -gt 0 ]] || { echo "check-gate-substrate-parity: $LIST names no gates" >&2; exit 2; }

RESOLVE_DIRS=("$GATES_DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots_rel)

findings=()

# assertion A: each member resolves to exactly one declaration — a dir carrying
# both <name>.sh and <name>.gate is ambiguous dispatch, never resolved by order
declared=0
for m in "${MEMBERS[@]}"; do
    for d in "${RESOLVE_DIRS[@]}"; do
        if [[ -f "$d/$m.sh" && -f "$d/$m.gate" ]]; then
            findings+=("ambiguous dispatch: $d carries both $m.sh and $m.gate")
        fi
    done
    if ! src="$(gate_resolve "$m" "${RESOLVE_DIRS[@]}")"; then
        findings+=("unresolvable member: $m declares in none of: ${RESOLVE_DIRS[*]}")
        continue
    fi
    declared=$((declared + 1))
done

# assertion B: the .gate descriptors on disk and the binary's reported subcommand
# roster are the same set — a descriptor naming no subcommand is a gate that
# cannot run; a subcommand with no descriptor is a gate nothing declares
mapfile -t DESCRIPTORS < <(
    for d in "${RESOLVE_DIRS[@]}"; do
        [[ -d "$d" ]] || continue
        for f in "$d"/*.gate; do
            [[ -f "$f" ]] || continue
            b="${f##*/}"; printf '%s\n' "${b%.gate}"
        done
    done | sort -u
)
BIN="${GATE_SDK_NATIVE_BIN:-native/target/release/checkwright-gates}"
subcommands=()
if [[ ${#DESCRIPTORS[@]} -gt 0 ]]; then
    if [[ ! -x "$BIN" ]]; then
        echo "check-gate-substrate-parity: $BIN is absent or not executable, but ${#DESCRIPTORS[@]} .gate descriptor(s) dispatch to it — the check could not run; treating as failure (not clean)" >&2
        echo "  help: build it — cargo build --release --manifest-path native/Cargo.toml" >&2
        exit 2
    fi
    listing="$("$BIN" --list)"; st=$?
    fail_closed "$st" check-gate-substrate-parity "$BIN --list"
    mapfile -t subcommands < <(printf '%s\n' "$listing" | grep -v '^$' | sort -u)
    for g in "${DESCRIPTORS[@]}"; do
        printf '%s\n' "${subcommands[@]}" | grep -qx -- "$g" \
            || findings+=("descriptor names no subcommand: $g.gate declares a gate the binary does not carry")
    done
    for s in "${subcommands[@]}"; do
        printf '%s\n' "${DESCRIPTORS[@]}" | grep -qx -- "$s" \
            || findings+=("subcommand nothing declares: the binary carries '$s' with no $s.gate descriptor")
    done
fi

# assertion C: every derived substrate-sensitive member carries a disposition in
# the conservation section — the anti-vacuity assertion, so a new meta-gate over
# gate source reds until its disposition is recorded
mapfile -t DECLPATHS < <(
    for m in "${MEMBERS[@]}"; do gate_resolve "$m" "${RESOLVE_DIRS[@]}" || true; done
)
section="$(awk -v s="$SECTION" '
    $0 == s { inb = 1; next }
    inb && /^## / { inb = 0 }
    inb { print }
' "$DOC")"; st=$?
fail_closed "$st" check-gate-substrate-parity awk
[[ -n "$section" ]] || { echo "check-gate-substrate-parity: no '$SECTION' section in $DOC" >&2; exit 2; }

sensitive=0
for m in "${MEMBERS[@]}"; do
    src="$(gate_resolve "$m" "${RESOLVE_DIRS[@]}")" || continue
    couples="$(gate_manifest_field "$src" couples)"
    [[ -n "$couples" ]] || continue
    gate_expand_couples_var couples "$couples"
    IFS=',' read -ra globs <<<"$couples"
    hit=0
    for g in "${globs[@]}"; do
        for p in "${DECLPATHS[@]}"; do
            # shellcheck disable=SC2053
            if [[ "$p" == $g ]]; then hit=1; break 2; fi
        done
    done
    (( hit )) || continue
    sensitive=$((sensitive + 1))
    grep -qF -- "\`$m\`" <<<"$section" \
        || findings+=("no recorded disposition: $m is substrate-sensitive (its couples= covers a gate declaration path) but $SECTION does not name it")
done

# assertion D: manifest-class annotations live in the declaration only — a second
# writable copy in the implementation is an SSOT violation that drifts silently,
# and the manifest must stay readable with no build and no execution
IMPL_DIR="${GATE_SDK_NATIVE_SRC:-native/src}"
impl_scanned=0
if [[ -d "$IMPL_DIR" ]]; then
    while IFS= read -r f; do
        [[ -n "$f" ]] || continue
        impl_scanned=$((impl_scanned + 1))
        while IFS= read -r hit; do
            [[ -n "$hit" ]] || continue
            findings+=("manifest-class annotation in implementation source: $f:${hit%%:*} — the '# graph:' manifest belongs to the declaration path alone")
        done < <(grep -nE '^[[:space:]]*(#|//|/\*)[[:space:]]*graph:[[:space:]]' "$f" || true)
    done < <(gate_find "$IMPL_DIR" -type f)
fi

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-gate-substrate-parity: the gate substrate seam is not conserved:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: one declaration per member — delete the stale .sh or .gate where a dir"
    echo "        carries both. Keep the descriptor set and the binary's --list roster equal:"
    echo "        add the missing .gate, or drop the subcommand nothing declares."
    echo "        A substrate-sensitive member with no disposition is recorded in"
    echo "        $DOC $SECTION — say ported, retained, or retired with cause;"
    echo "        an unrecorded one silently stops asserting when a gate ports."
    echo "  help: delete a manifest-class annotation from implementation source — the"
    echo "        '# graph:' manifest has exactly one writable home, the declaration"
    echo "        path, so that every reader of it works with no build and no execution."
    exit 1
fi

echo "GATE-SUBSTRATE-PARITY: clean ($declared member(s) with one declaration each; ${#DESCRIPTORS[@]} descriptor(s) in parity with the subcommand roster; $sensitive substrate-sensitive member(s) all dispositioned; $impl_scanned implementation source(s) free of manifest-class annotation)"
exit 0
