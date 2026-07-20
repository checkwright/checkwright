#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §upgrade-smoke — the two-phase upgrade proof on the consumer-smoke mechanics; the 'upgrade' validate suite each validate stage re-runs, and (TO=HEAD, the default) the standing pre-release assertion that the working tree upgrades cleanly from the last tag. Harness-less: bare bash + git, never the network.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/consumer-smoke.sh
source "$SDK/lib/consumer-smoke.sh"

# spec: gate-sdk/SPEC.md §upgrade-smoke — resolve the source repo, FROM, and TO (each knob read exactly here)
REPO="${GATE_SDK_UPGRADE_REPO:-$(git -C "$SDK" rev-parse --show-toplevel 2>/dev/null)}"
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
cleanup() { rm -rf "$WORK" "$SCRATCH"; }
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

# spec: gate-sdk/SPEC.md §upgrade-smoke — step 1: vendor + install + baseline at FROM via the shared scratch-consumer builder, then run the battery (a red FROM baseline is a broken tag: exit 2, not an upgrade finding)
csmoke_vendor_and_install "${fromroots[@]}" \
    || { echo "upgrade-smoke: vendoring the FROM baseline ($FROM) failed — a broken tag, not an upgrade finding" >&2; exit 2; }
CONS="$SCRATCH"

run_battery() { ( cd "$CONS" && bash gate-sdk/bin/run-gates.sh ) 2>&1; }

out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    echo "upgrade-smoke: FAIL(env) — the FROM baseline ($FROM) is not green under zero config; the tag itself is broken, not an upgrade finding" >&2
    printf '%s\n' "$out" >&2
    exit 2
fi

# spec: gate-sdk/SPEC.md §upgrade-smoke — phase A: replace the vendored kit directories wholesale at TO and regenerate the generated artifacts (the contract's consumer steps, docs/install.md §The upgrade contract)
declare -A kitname_seen=()
for r in "${fromroots[@]}"; do rm -rf "${CONS:?}/$(basename "$r")"; done
for r in "${toroots[@]}"; do
    k="$(basename "$r")"; kitname_seen["$k"]=1
    cp -R "$r" "$CONS/$k"
done
for r in "${fromroots[@]}"; do kitname_seen["$(basename "$r")"]=1; done

( cd "$CONS" && bash gate-sdk/bin/gen-pre-commit.sh --write >/dev/null ) \
    || { echo "upgrade-smoke: phase A gen-pre-commit failed at TO ($TO)" >&2; exit 2; }
( cd "$CONS" && bash gate-sdk/checks/check-graph.sh --emit > .workflow/CHECK-GRAPH.html ) \
    || { echo "upgrade-smoke: phase A check-graph --emit failed at TO ($TO)" >&2; exit 2; }
if [[ -f "$CONS/doctrine-kit/bin/install-doctrine.sh" ]]; then
    ( cd "$CONS" && bash doctrine-kit/bin/install-doctrine.sh >/dev/null ) \
        || { echo "upgrade-smoke: phase A install-doctrine failed at TO ($TO)" >&2; exit 2; }
fi

# spec: gate-sdk/SPEC.md §upgrade-smoke — determinism: git status shows changes only under kit roots and the two regenerated artifacts; anything else means phase A edited a consumer file (the wholesale-sync invariant broke)
git -C "$CONS" add -A
stray=()
while IFS= read -r p; do
    [[ -n "$p" ]] || continue
    case "$p" in
        scripts/git-hooks/pre-commit|.workflow/CHECK-GRAPH.html|CLAUDE.md) continue ;;
    esac
    top="${p%%/*}"
    [[ -n "${kitname_seen[$top]:-}" ]] || stray+=("$p")
done < <(git -C "$CONS" diff --cached --name-only)

if [[ ${#stray[@]} -gt 0 ]]; then
    echo "upgrade-smoke: FAIL — phase A is non-deterministic: it changed consumer files outside the kit roots and the regenerated artifacts:" >&2
    for p in "${stray[@]}"; do echo "  $p" >&2; done
    echo "  the wholesale kit-sync must lose nothing a consumer owns (docs/install.md §The upgrade contract)." >&2
    exit 1
fi
git -C "$CONS" -c user.email=smoke@example.invalid -c user.name=smoke \
    commit -q --no-verify --allow-empty -m "phase A: kits at $TO"

# spec: gate-sdk/SPEC.md §upgrade-smoke — step 3: the red set must be a subset of TO's tightened-gates declaration (the docs/posts note whose front-matter release: names TO's version). TO unreleased (HEAD) resolves no version → no note → the red set must be empty.
ver="$(git -C "$REPO" tag --points-at "$TO" --list 'v*' 2>/dev/null | head -1)"
note=""
if [[ -n "$ver" ]]; then
    shopt -s nullglob
    for f in "$TO_TREE"/docs/posts/*.md; do
        if grep -qE "^release:[[:space:]]+${ver}[[:space:]]*\$" "$f"; then note="$f"; break; fi
    done
    shopt -u nullglob
fi

allowed=()
if [[ -n "$note" ]]; then
    body="$(awk '/^##[[:space:]]+Tightened gates[[:space:]]*$/{f=1;next} /^##[[:space:]]/{f=0} f' "$note")"
    if ! grep -qiE '^[[:space:]]*none\b' <<<"$body"; then
        mapfile -t allowed < <(printf '%s\n' "$body" \
            | sed -nE 's/^[[:space:]]*[-*][[:space:]]+`?([A-Za-z][A-Za-z0-9-]*)`?.*/\1/p' | sort -u)
    fi
fi

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

if [[ ${#red[@]} -gt 0 && -z "$note" ]]; then
    echo "upgrade-smoke: FAIL — TO (${ver:-$TO}) reddened gate(s) but no release note declares a tightened-gates set:" >&2
    printf '  %s\n' "${red[@]}" >&2
    echo "  an unreleased TO must upgrade green; a red gate needs a note bullet (docs/install.md §The upgrade contract)." >&2
    exit 1
fi

undeclared=()
for g in "${red[@]+"${red[@]}"}"; do
    printf '%s\n' "${allowed[@]+"${allowed[@]}"}" | grep -qxF "$g" || undeclared+=("$g")
done
if [[ ${#undeclared[@]} -gt 0 ]]; then
    echo "upgrade-smoke: FAIL — gate(s) went red that TO's tightened-gates declaration does not name:" >&2
    for g in "${undeclared[@]}"; do echo "  $g" >&2; done
    echo "  each red must be a bullet in the note's Tightened gates section, or the tree fixed (docs/install.md §The upgrade contract)." >&2
    exit 1
fi

echo "UPGRADE-SMOKE: clean ($FROM → $TO; ${#fromroots[@]}→${#toroots[@]} kits vendored, phase A deterministic, red set ${#red[@]} ⊆ ${#allowed[@]} declared)"
exit 0
