#!/usr/bin/env bash
# graph: couples=scripts/gates.list,kit:checks/*,gate-sdk/SPEC.md,native/*,.github/workflows/publish.yml dir=one valve=none tier=precommit
# install: on-surface
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — one declaration per member, descriptor/subcommand parity both ways, a recorded disposition for every substrate-sensitive member, no implementation source inside the vendoring set, and one owner for the target roster the artifact path derives from
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

# spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — the one
# disposition surface, read by assertion B's reference-only allowance and assertion C
section="$(awk -v s="$SECTION" '
    $0 == s { inb = 1; next }
    inb && /^## / { inb = 0 }
    inb { print }
' "$DOC")"; st=$?
fail_closed "$st" check-gate-substrate-parity awk
[[ -n "$section" ]] || { echo "check-gate-substrate-parity: no '$SECTION' section in $DOC" >&2; exit 2; }

findings=()

# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F's publishing-tree test: the
# crate's *tracked source* is here, step 1 of §check-gate-binary-fresh's source stamp reused rather
# than spelled twice — source, so build output under the crate root cannot read as a publisher
crate_source_here() {
    [[ -d "$1" ]] || return 1
    [[ -n "$(git -C "$1" ls-files 2>/dev/null)" ]]
}

# assertion A: each member resolves to exactly one declaration — a dir carrying
# both <name>.sh and <name>.gate is ambiguous dispatch, never resolved by order
declared=0
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — a descriptor on disk is a declaration;
# a registered member resolving to one is a dispatch, and only a dispatch makes the binary
# load-bearing. Derived here because assertion A already resolves every member.
dispatching=0
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
    [[ "$src" == *.gate ]] && dispatching=$((dispatching + 1))
    declared=$((declared + 1))
done

# assertion B: the .gate descriptors on disk and the binary's reported subcommand
# roster are the same set, with 'reference-only' the one dispositioned exception
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity
mapfile -t DESCRIPTORS < <(
    for d in "${RESOLVE_DIRS[@]}"; do
        [[ -d "$d" ]] || continue
        for f in "$d"/*.gate; do
            [[ -f "$f" ]] || continue
            b="${f##*/}"; printf '%s\n' "${b%.gate}"
        done
    done | sort -u
)
BIN="$(gate_native_bin)"
subcommands=()
refonly=0
roster_read=0
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the fail-closed arm is on the corrected
# predicate: descriptors nothing registered dispatches to leave the binary not load-bearing
if [[ "$dispatching" -gt 0 && ! -x "$BIN" ]]; then
    echo "check-gate-substrate-parity: $BIN is absent or not executable, but $dispatching registered member(s) dispatch to it — the check could not run; treating as failure (not clean)" >&2
    echo "  help: build it — bash gate-sdk/bin/build-native.sh" >&2
    exit 2
fi
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the roster half is gated on a
# readable binary, never on descriptor count
if [[ -x "$BIN" ]]; then
    roster_read=1
    listing="$("$BIN" --list)"; st=$?
    fail_closed "$st" check-gate-substrate-parity "$BIN --list"
    mapfile -t subcommands < <(printf '%s\n' "$listing" | grep -v '^$' | sort -u)
    for g in "${DESCRIPTORS[@]+"${DESCRIPTORS[@]}"}"; do
        printf '%s\n' "${subcommands[@]}" | grep -qx -- "$g" \
            || findings+=("descriptor names no subcommand: $g.gate declares a gate the binary does not carry")
    done
    for s in "${subcommands[@]}"; do
        printf '%s\n' "${DESCRIPTORS[@]+"${DESCRIPTORS[@]}"}" | grep -qx -- "$s" && continue
        if grep -F -- "\`$s\`" <<<"$section" | grep -qi -- 'reference-only'; then
            refonly=$((refonly + 1))
            continue
        fi
        findings+=("subcommand nothing declares: the binary carries '$s' with no $s.gate descriptor and no reference-only disposition in $SECTION")
    done
fi

# assertion C: every derived substrate-sensitive member carries a disposition in
# the conservation section — the anti-vacuity assertion, so a new meta-gate over
# gate source reds until its disposition is recorded
mapfile -t DECLPATHS < <(
    for m in "${MEMBERS[@]}"; do gate_resolve "$m" "${RESOLVE_DIRS[@]}" || true; done
)
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

# assertion E: opacity is held by structure — a ported gate's implementation source
# may not reach the vendoring set, whose members are exactly the kit roots
# spec: gate-sdk/SPEC.md §Consumer payload
CRATE="$(gate_native_crate)"
declare -A DESCRIPTOR_SET=()
for g in "${DESCRIPTORS[@]+"${DESCRIPTORS[@]}"}"; do DESCRIPTOR_SET["$g"]=1; done
kit_scanned=0
while IFS= read -r root; do
    root="${root%/}"
    if [[ "$CRATE" == "$root" || "$CRATE" == "$root"/* ]]; then
        findings+=("crate root inside the vendoring set: $CRATE sits under kit root $root — a kit root vendors whole, so the implementation source would ship with it")
    fi
    [[ ${#DESCRIPTOR_SET[@]} -gt 0 && -d "$root" ]] || continue
    kit_scanned=$((kit_scanned + 1))
    while IFS= read -r f; do
        [[ -n "$f" ]] || continue
        base="${f##*/}"
        stem="${base%.*}"
        # spec: gate-sdk/SPEC.md §check-gate-substrate-parity — an extensionless name is out of reach by contract, and .gate/.sh are owned by the descriptor itself and by assertion A
        [[ "$stem" == "$base" ]] && continue
        ext="${base##*.}"
        [[ "$ext" == gate || "$ext" == sh ]] && continue
        if [[ -n "${DESCRIPTOR_SET[$stem]:-}" ]]; then
            findings+=("implementation sibling in the vendoring set: $f shares its name with the $stem.gate descriptor — a ported gate's implementation may not sit under a kit root")
        fi
    done < <(gate_find "$root" -type f)
done < <(gate_kit_roots_rel)

# assertion F: the target roster has one owner and the publish path derives from it
# — the roster is what asserts platform support, so a second spelling of it (a
# platform literal in the build matrix) or a second producer of a published digest
# is the failure this assertion exists to make impossible
# spec: gate-sdk/SPEC.md §Consumer payload
ROSTER="$(gate_native_targets_file)"
roster_targets=0
roster_state="absent"
if [[ -f "$ROSTER" ]]; then
    roster_state="read"
    mapfile -t TARGETS < <(gates_list_members "$ROSTER")
    if [[ ${#TARGETS[@]} -eq 0 ]]; then
        findings+=("empty target roster: $ROSTER declares no target — a roster asserting no platform support cannot be the surface that asserts it")
    fi
    for t in "${TARGETS[@]}"; do
        roster_targets=$((roster_targets + 1))
        [[ "$t" =~ ^[A-Za-z0-9_]+(-[A-Za-z0-9_.]+){2,3}$ ]] \
            || findings+=("malformed target triple: '$t' in $ROSTER is not <arch>-<vendor>-<os>[-<env>]")
    done
elif [[ "$dispatching" -gt 0 ]] && crate_source_here "$CRATE"; then
    # spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F rides the corrected
    # predicate *and* the publishing-tree test: declaring platform support is the act of the tree
    # that builds and publishes the artifact, and a consumer receives kit roots but never the crate
    findings+=("no target roster: $ROSTER is absent, but $dispatching registered member(s) dispatch to the binary and $CRATE carries tracked source here — a tree that builds the artifact declares the platforms it carries one for")
fi

WORKFLOW="${GATE_SDK_NATIVE_PUBLISH_WORKFLOW:-.github/workflows/publish.yml}"
wf_state="absent"
wf_matrix=0
wf_jobs=0
if [[ -f "$WORKFLOW" ]]; then
    wf_state="read"
    # spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F, roster-derived matrix: every value in a matrix declaration is a GitHub expression, never a literal
    matrix_lits="$(awk '
        function ind(s,   n) { n = match(s, /[^ ]/); return (n == 0) ? -1 : n - 1 }
        inb {
            if ($0 ~ /^[[:space:]]*$/) next
            if (ind($0) > keycol) {
                if ($0 !~ /^[[:space:]]*#/ && index($0, "${{") == 0)
                    printf "%d\t%s\n", FNR, $0
                next
            }
            inb = 0
        }
        /^[[:space:]]*matrix:[[:space:]]*$/ { inb = 1; keycol = ind($0); n++; next }
        /^[[:space:]]*matrix:[[:space:]]*[^[:space:]]/ {
            n++
            if (index($0, "${{") == 0) printf "%d\t%s\n", FNR, $0
        }
        END { printf "#\t%d\n", n }
    ' "$WORKFLOW")"; st=$?
    fail_closed "$st" check-gate-substrate-parity "awk matrix($WORKFLOW)"
    while IFS=$'\t' read -r ln text; do
        [[ -n "$ln" ]] || continue
        if [[ "$ln" == "#" ]]; then wf_matrix="$text"; continue; fi
        text="${text#"${text%%[![:space:]]*}"}"
        findings+=("matrix declaration not roster-derived: $WORKFLOW:$ln '$text' is a literal where an expression over $ROSTER belongs — a hand-written platform in a build matrix is a second spelling of the support commitment")
    done <<<"$matrix_lits"

    # spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F, one producer per digest: a job computes at most one, and a job that downloads a run artifact and uploads none computes none
    digests="$(awk '
        function emit() { if (job != "") printf "%s\t%d\t%d\t%d\n", job, d, dl, ul }
        /^jobs:[[:space:]]*$/ { injobs = 1; next }
        injobs && /^[^[:space:]#]/ { emit(); job = ""; injobs = 0 }
        injobs && /^  [A-Za-z0-9_-]+:[[:space:]]*$/ {
            emit()
            job = $0; sub(/^ +/, "", job); sub(/:.*$/, "", job)
            d = 0; dl = 0; ul = 0
            next
        }
        job != "" {
            if ($0 !~ /^[[:space:]]*#/ && $0 ~ /sha256sum/ && $0 !~ /sha256sum[[:space:]]+-c/) d++
            if ($0 ~ /uses:[[:space:]]*[^[:space:]]*download-artifact/) dl = 1
            if ($0 ~ /uses:[[:space:]]*[^[:space:]]*upload-artifact/) ul = 1
        }
        END { emit() }
    ' "$WORKFLOW")"; st=$?
    fail_closed "$st" check-gate-substrate-parity "awk digests($WORKFLOW)"
    while IFS=$'\t' read -r job d dl ul; do
        [[ -n "$job" ]] || continue
        wf_jobs=$((wf_jobs + 1))
        if [[ "$d" -gt 1 ]]; then
            findings+=("digest recomputed: job '$job' in $WORKFLOW computes $d digests — each is emitted once, where its bytes are produced, and moved thereafter")
        elif [[ "$d" -gt 0 && "$dl" == 1 && "$ul" == 0 ]]; then
            findings+=("digest computed by a consumer: job '$job' in $WORKFLOW downloads a run artifact, produces none, and still computes a digest — it must move the sidecar it received, never re-derive it")
        fi
    done <<<"$digests"
fi

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-gate-substrate-parity: the gate substrate seam is not conserved:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: one declaration per member — delete the stale .sh or .gate where a dir"
    echo "        carries both. Keep the descriptor set and the binary's --list roster equal:"
    echo "        add the missing .gate, or drop the subcommand nothing declares — or,"
    echo "        for an implementation deliberately kept ahead of any live port, give it"
    echo "        a 'reference-only' disposition naming it in $SECTION."
    echo "        A substrate-sensitive member with no disposition is recorded in"
    echo "        $DOC $SECTION — say ported, retained, or retired with cause;"
    echo "        an unrecorded one silently stops asserting when a gate ports."
    echo "  help: delete a manifest-class annotation from implementation source — the"
    echo "        '# graph:' manifest has exactly one writable home, the declaration"
    echo "        path, so that every reader of it works with no build and no execution."
    echo "  help: move a ported gate's implementation out of every kit root, and keep the"
    echo "        crate root outside them too — a kit root vendors whole, so anything"
    echo "        under one ships, and the payload withholds the predicate by structure."
    echo "  help: the target roster is the one surface asserting platform support — keep"
    echo "        every live line a well-formed target triple, derive the publish"
    echo "        workflow's matrix from it rather than spelling a platform there, and"
    echo "        emit each artifact's digest in exactly one step, where its bytes are"
    echo "        produced. A runner mapping may name a platform; a matrix may not."
    exit 1
fi

if [[ "$roster_read" == 1 ]]; then
    roster="${#DESCRIPTORS[@]} descriptor(s) in parity with the ${#subcommands[@]}-subcommand roster, $refonly reference-only"
else
    roster="${#DESCRIPTORS[@]} descriptor(s), no binary at $BIN so no subcommand roster to compare"
fi
echo "GATE-SUBSTRATE-PARITY: clean ($declared member(s) with one declaration each, $dispatching of them dispatching to the binary; $roster; $sensitive substrate-sensitive member(s) all dispositioned; $impl_scanned implementation source(s) free of manifest-class annotation; $kit_scanned kit root(s) scanned for an implementation sibling, crate root $CRATE outside every kit root; target roster $roster_state at $ROSTER with $roster_targets well-formed target(s); publish workflow $wf_state at $WORKFLOW, $wf_matrix matrix declaration(s) roster-derived across $wf_jobs job(s) with one producer per digest)"
exit 0
