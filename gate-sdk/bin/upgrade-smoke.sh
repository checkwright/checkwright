#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §upgrade-smoke — the two-phase upgrade proof on the consumer-smoke mechanics; the 'upgrade' validate suite each validate stage re-runs, and (TO=HEAD, the default) the standing pre-release assertion that the working tree upgrades cleanly from the last tag. Each phase runs its own ref's gate binary, built from a detached worktree at that ref. Harness-less: bare bash + git + cargo. Not network-free since the crate took its first dependency — a worktree build shares the host cargo home, so a warm registry cache needs no fetch and a cold one does.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/consumer-smoke.sh
source "$SDK/lib/consumer-smoke.sh"
# shellcheck source=../lib/declaration.sh
source "$SDK/lib/declaration.sh"

# spec: gate-sdk/SPEC.md §upgrade-smoke — resolve the source repo, FROM, and TO (each knob read exactly here)
REPO="${GATE_SDK_UPGRADE_REPO:-$( { cd "$(git -C "$SDK" rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )}"
[[ -n "$REPO" && -d "$REPO/.git" ]] \
    || { echo "upgrade-smoke: GATE_SDK_UPGRADE_REPO is not a git repository: ${REPO:-<unset>}" >&2; exit 2; }

FROM="${GATE_SDK_UPGRADE_FROM:-$(git -C "$REPO" tag --list 'v*' --sort=-v:refname | head -1)}"
[[ -n "$FROM" ]] \
    || { echo "upgrade-smoke: no FROM ref — GATE_SDK_UPGRADE_FROM unset and no v* tag in $REPO; the baseline is unresolvable" >&2; exit 2; }
git -C "$REPO" rev-parse --verify -q "$FROM^{commit}" >/dev/null \
    || { echo "upgrade-smoke: FROM ref does not resolve to a commit: $FROM" >&2; exit 2; }

TO="${GATE_SDK_UPGRADE_TO:-HEAD}"
git -C "$REPO" rev-parse --verify -q "$TO^{commit}" >/dev/null \
    || { echo "upgrade-smoke: TO ref does not resolve to a commit: $TO" >&2; exit 2; }

# spec: gate-sdk/SPEC.md §upgrade-smoke — scratch base is GATE_SDK_TMP_DIR; the trees + consumer are mktemp-created and trap-removed like the demo's
BASE="${GATE_SDK_TMP_DIR:-.tmp}"
mkdir -p "$BASE" || { echo "upgrade-smoke: cannot create scratch base $BASE" >&2; exit 2; }
BASE="$(cd "$BASE" && pwd)"
export TMPDIR="$BASE"   # csmoke_vendor_and_install mktemps the consumer under TMPDIR — pin it to the knob
WORK="$(mktemp -d "$BASE/upgrade-smoke.XXXXXX")" || exit 2
SCRATCH=""              # csmoke_vendor_and_install sets this to the consumer dir
# spec: gate-sdk/SPEC.md §upgrade-smoke — a per-ref worktree outlives the run if the trap does not take it, and the repo's worktree-prune mechanism should not have to collect after a validate suite
WORKTREES=()
cleanup() {
    local w
    for w in "${WORKTREES[@]+"${WORKTREES[@]}"}"; do
        git -C "$REPO" worktree remove --force "$w" >/dev/null 2>&1
    done
    rm -rf "$WORK" "$SCRATCH"
}
trap cleanup EXIT

FROM_TREE="$WORK/from"; TO_TREE="$WORK/to"
mkdir -p "$FROM_TREE" "$TO_TREE"
git -C "$REPO" archive "$FROM" | tar -x -C "$FROM_TREE" \
    || { echo "upgrade-smoke: git archive of FROM ($FROM) failed" >&2; exit 2; }
git -C "$REPO" archive "$TO" | tar -x -C "$TO_TREE" \
    || { echo "upgrade-smoke: git archive of TO ($TO) failed" >&2; exit 2; }

# spec: gate-sdk/SPEC.md §upgrade-smoke — a ref's vendorable kits are the dirs shipping smoke/install.sh (§Consumer smoke's per-kit contract), gate-sdk first
kit_dirs_in() {
    local tree="$1" d base
    [[ -f "$tree/gate-sdk/smoke/install.sh" ]] && printf '%s\n' "$tree/gate-sdk"
    for d in "$tree"/*/; do
        base="$(basename "$d")"
        [[ "$base" == gate-sdk ]] && continue
        [[ -f "$d/smoke/install.sh" ]] && printf '%s\n' "${d%/}"
    done
}

mapfile -t fromroots < <(kit_dirs_in "$FROM_TREE")
mapfile -t toroots < <(kit_dirs_in "$TO_TREE")
[[ ${#fromroots[@]} -gt 0 ]] || { echo "upgrade-smoke: no vendorable kits at FROM ($FROM)" >&2; exit 2; }
[[ ${#toroots[@]} -gt 0 ]] || { echo "upgrade-smoke: no vendorable kits at TO ($TO)" >&2; exit 2; }

# spec: gate-sdk/SPEC.md §upgrade-smoke — a ref's binary is built from a detached worktree at that ref, never from the archive tree its kits come from: native/build.rs stamps its source with `git ls-files` and panics outside a checkout, so an archive build dies in the build script and reads as a broken tag. The build lands in the worktree's own native/target/, which is already under the scratch base — so the host's build output is untouched and gate_native_bin's relative path still resolves against the checkout this names
REF_TREE=""
ref_binary_tree() {   # $1 = ref, $2 = phase label -> sets REF_TREE to a checkout whose native/ is built
    local ref="$1" label="$2" wt out
    REF_TREE=""
    command -v cargo >/dev/null 2>&1 || {
        echo "upgrade-smoke: FAIL(env) — the $label ref ($ref) dispatches gate(s) to the binary and cargo is not on PATH; this suite builds one binary per ref" >&2
        return 2
    }
    wt="$WORK/checkout-$label"
    git -C "$REPO" worktree add --detach -q "$wt" "$ref" >/dev/null 2>&1 || {
        echo "upgrade-smoke: FAIL(env) — could not add a detached worktree at the $label ref ($ref)" >&2
        return 2
    }
    WORKTREES+=("$wt")
    # spec: gate-sdk/SPEC.md §upgrade-smoke — a ref dispatching to a binary it carries no crate for is a tag fact, and the one thing this must not do is fall back to the host's binary, which is the present behavior wearing a fallback's clothes
    [[ -d "$wt/native" ]] || {
        echo "upgrade-smoke: FAIL(env) — the $label ref ($ref) dispatches gate(s) to the binary and carries no crate to build one from; a broken tag, not an upgrade finding" >&2
        return 2
    }
    out="$( cd "$wt/native" && cargo build --release 2>&1 )" || {
        echo "upgrade-smoke: FAIL(env) — the $label ref ($ref) will not build its gate binary under this toolchain; an environment or tag fact, never an upgrade finding" >&2
        printf '%s\n' "$out" >&2
        return 2
    }
    REF_TREE="$wt"
}

# spec: gate-sdk/SPEC.md §upgrade-smoke — step 1: vendor + install + baseline at FROM via the shared scratch-consumer builder, paired with FROM's own binary so phase 1's claim is about FROM alone, then run the battery (a red FROM baseline is a broken tag: exit 2, not an upgrade finding)
FROM_BIN=""
if [[ "$(csmoke_gate_descriptors "${fromroots[@]}")" -gt 0 ]]; then
    ref_binary_tree "$FROM" from || exit 2
    FROM_BIN="$REF_TREE"
fi
csmoke_vendor_and_install "$FROM_BIN" "${fromroots[@]}" \
    || { echo "upgrade-smoke: vendoring the FROM baseline ($FROM) failed — a broken tag, not an upgrade finding" >&2; exit 2; }
CONS="$SCRATCH"

run_battery() { ( cd "$CONS" && bash gate-sdk/bin/run-gates.sh ) 2>&1; }

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "upgrade-smoke: FAIL(env) — the FROM baseline ($FROM) is not green under zero config; the tag itself is broken, not an upgrade finding" >&2
    printf '%s\n' "$out" >&2
    exit 2
fi

# spec: gate-sdk/SPEC.md §upgrade-smoke — phase A, step 1 of 2: replace the vendored kit directories wholesale at TO (the contract's consumer steps, docs/install.md §The upgrade contract)
declare -A kitname_seen=()
for r in "${fromroots[@]}"; do rm -rf "${CONS:?}/$(basename "$r")"; done
for r in "${toroots[@]}"; do
    k="$(basename "$r")"; kitname_seen["$k"]=1
    cp -R "$r" "$CONS/$k"
done
for r in "${fromroots[@]}"; do kitname_seen["$(basename "$r")"]=1; done

# spec: gate-sdk/SPEC.md §upgrade-smoke — the binary is re-placed in the same motion that swaps the kit directories, because that swap is the upgrade transition: phase B's claim — TO's shell against TO's binary — then holds by construction rather than by the host tree happening to be TO
if [[ "$(csmoke_gate_descriptors "${toroots[@]}")" -gt 0 ]]; then
    ref_binary_tree "$TO" to || exit 2
    csmoke_place_binary "$REF_TREE" "${toroots[@]}" \
        || { echo "upgrade-smoke: FAIL(env) — could not place TO ($TO)'s gate binary in the scratch consumer" >&2; exit 2; }
fi

# spec: gate-sdk/SPEC.md §upgrade-smoke — determinism is measured on the sync alone, before a regen step has run: the sync's whole claim is that it loses nothing a consumer owns, so its staged set must fall under the kit roots and nowhere else, with no exemption to state
git -C "$CONS" add -A
stray=()
while IFS= read -r p; do
    [[ -n "$p" ]] || continue
    top="${p%%/*}"
    [[ -n "${kitname_seen[$top]:-}" ]] || stray+=("$p")
done < <(git -C "$CONS" diff --cached --name-only)

if [[ ${#stray[@]} -gt 0 ]]; then
    echo "upgrade-smoke: FAIL — the phase-A kit sync is non-deterministic: it changed consumer files outside the kit roots:" >&2
    for p in "${stray[@]}"; do echo "  $p" >&2; done
    echo "  the wholesale kit-sync must lose nothing a consumer owns (docs/install.md §The upgrade contract)." >&2
    exit 1
fi

# spec: gate-sdk/SPEC.md §upgrade-smoke — phase A, step 2 of 2: regenerate the generated artifacts, run after the sync has been judged. Which paths each emitter writes is that emitter's own contract, held by that emitter's own freshness gate, so nothing here names the set
( cd "$CONS" && bash gate-sdk/bin/gen-pre-commit.sh --write >/dev/null ) \
    || { echo "upgrade-smoke: phase A gen-pre-commit failed at TO ($TO)" >&2; exit 2; }
# spec: gate-sdk/SPEC.md §upgrade-smoke — the artifact's path is resolved rather than spelled again here: check-graph resolves the same knob for itself, and a second spelling mis-writes under a GATE_SDK_GRAPH_ARTIFACT or GATE_SDK_GATES_DIR the consumer does not share. It is resolved in the consumer's library and not in this tool's own — this tool sources the host repo's gate.sh, so the host's value is this repo's docs path while the scratch consumer is zero-config, and reading the host's would write the artifact where that consumer's own gate will not look. The emitter is a binary arm, so it is reached through the front-end that resolves its bridged knobs (§The non-gate arm), never by a path into the kit.
CONS_ARTIFACT="$( cd "$CONS" && bash -c 'source gate-sdk/lib/gate.sh; printf "%s" "$GATE_SDK_GRAPH_ARTIFACT"' )" \
    || { echo "upgrade-smoke: could not resolve the graph artifact path at TO ($TO)" >&2; exit 2; }
[[ -n "$CONS_ARTIFACT" ]] \
    || { echo "upgrade-smoke: the consumer's library resolved an empty graph artifact path at TO ($TO)" >&2; exit 2; }
( cd "$CONS" && bash gate-sdk/bin/run-gates.sh --emit graph > "$CONS_ARTIFACT" ) \
    || { echo "upgrade-smoke: phase A graph emit failed at TO ($TO)" >&2; exit 2; }
if [[ -f "$CONS/doctrine-kit/bin/install-doctrine.sh" ]]; then
    ( cd "$CONS" && bash doctrine-kit/bin/install-doctrine.sh >/dev/null ) \
        || { echo "upgrade-smoke: phase A install-doctrine failed at TO ($TO)" >&2; exit 2; }
fi

git -C "$CONS" add -A
git -C "$CONS" -c user.email=smoke@example.invalid -c user.name=smoke \
    commit -q --no-verify --allow-empty -m "phase A: kits at $TO"

# spec: gate-sdk/SPEC.md §upgrade-smoke — step 3: the red set must be a subset of TO's tightened-gates declaration, resolved on two arms over lib/declaration.sh's one token predicate. TO tagged: the docs/posts note whose front-matter release: names the tag, its Tightened-gates lead tokens. TO untagged (the HEAD default): the declaration surface in TO's tree, appended by the build stage that landed or tightened the gate — so an untagged TO proves containment rather than emptiness.
ver="$(git -C "$REPO" tag --points-at "$TO" --list 'v*' 2>/dev/null | head -1)"
DECL_FILE="$TO_TREE/${GATE_SDK_WORKFLOW_DIR:-.workflow}/tightened-gates.txt"

decl_src=""; decl_out=""; decl_st=0
if [[ -n "$ver" ]]; then
    shopt -s nullglob
    for f in "$TO_TREE"/docs/posts/*.md; do
        if grep -qE "^release:[[:space:]]+${ver}[[:space:]]*\$" "$f"; then decl_src="$f"; break; fi
    done
    shopt -u nullglob
    if [[ -n "$decl_src" ]]; then
        decl_out="$(decl_section_tokens "$decl_src" "Tightened gates")"; decl_st=$?
    fi
elif [[ -f "$DECL_FILE" ]]; then
    decl_src="$DECL_FILE"
    decl_out="$(decl_record_tokens "$DECL_FILE")"; decl_st=$?
fi

if [[ "$decl_st" -eq 2 ]]; then
    echo "upgrade-smoke: FAIL — TO ($ver) resolves note $decl_src, which carries no 'Tightened gates' section:" >&2
    echo "  every release note carries the fixed sections its note grammar rosters (docs/install.md §The upgrade contract)." >&2
    exit 1
fi
if [[ "$decl_st" -ne 0 ]]; then
    echo "upgrade-smoke: FAIL — TO (${ver:-$TO})'s tightened-gates declaration does not parse, so it would resolve to a silently empty allowed-red set — $decl_src:" >&2
    [[ -n "$decl_out" ]] && printf '  %s\n' "$decl_out" >&2
    echo "  a declaration is either an explicit 'None' or a non-empty set of bare gate names; in a note each is the backticked, unbolded lead token of a bullet (docs/install.md §The upgrade contract)." >&2
    exit 1
fi

allowed=()
mapfile -t allowed < <(printf '%s\n' "$decl_out" | grep -v '^[[:space:]]*$' | sort -u)

out="$(run_battery)"; rc=$?
red=()
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    line="$(grep -E '^[0-9]+ of [0-9]+ gates FAILED:' <<<"$out" | tail -1)"
    if [[ -z "$line" ]]; then
        echo "upgrade-smoke: FAIL — the phase-B battery is red but printed no 'FAILED:' summary line to read the red set from" >&2
        printf '%s\n' "$out" >&2
        exit 1
    fi
    read -r -a red <<<"${line#*FAILED: }"
fi

if [[ ${#red[@]} -gt 0 && -z "$decl_src" ]]; then
    echo "upgrade-smoke: FAIL — TO (${ver:-$TO}) reddened gate(s) but declares no tightened-gates set anywhere:" >&2
    printf '  %s\n' "${red[@]}" >&2
    if [[ -n "$ver" ]]; then
        echo "  no docs/posts note carries 'release: $ver'; a red gate needs a note bullet (docs/install.md §The upgrade contract)." >&2
    else
        echo "  an untagged TO reads ${DECL_FILE#"$TO_TREE"/}, which TO's tree does not carry; the build stage that lands or tightens a gate appends its name there (gate-sdk/SPEC.md §upgrade-smoke)." >&2
    fi
    exit 1
fi

undeclared=()
for g in "${red[@]+"${red[@]}"}"; do
    # spec: gate-sdk/SPEC.md §run-gates — membership without a pipe: an abandoned in-process producer's SIGPIPE becomes the pipeline's status under `set -o pipefail`, which would flip this verdict
    _allowed=0
    for _a in ${allowed[@]+"${allowed[@]}"}; do [[ "$_a" == "$g" ]] && { _allowed=1; break; }; done
    (( _allowed )) || undeclared+=("$g")
done
if [[ ${#undeclared[@]} -gt 0 ]]; then
    echo "upgrade-smoke: FAIL — gate(s) went red that TO's tightened-gates declaration does not name:" >&2
    for g in "${undeclared[@]}"; do echo "  $g" >&2; done
    echo "  each red must be named in $decl_src — a bullet in the note's Tightened gates section, or a data line of the declaration surface — or the tree fixed (docs/install.md §The upgrade contract)." >&2
    exit 1
fi

echo "UPGRADE-SMOKE: clean ($FROM → $TO; ${#fromroots[@]}→${#toroots[@]} kits vendored, phase A deterministic, red set ${#red[@]} ⊆ ${#allowed[@]} declared by ${decl_src:-no declaration})"
exit 0
