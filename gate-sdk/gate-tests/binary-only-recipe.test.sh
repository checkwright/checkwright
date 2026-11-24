#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — the binary-only recipe's oracle (docs/binary-only.md): on a scratch
# checkout holding no kit, the direct form's exit codes on a read-only tree, the root positional's
# narrowing, exit 2 for a missing <KIT>_KNOB_FILE and outside a worktree, GATE_SDK_KIT_DIRS keeping a
# top-level checks/ directory in the scan, and --run over a registry
# outside the checkout writing its SARIF log there and leaving the checkout's status empty.
# Run by the --run-gate-tests arm.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SRC="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="$GATE_SDK_NATIVE_BIN"
[[ -x "$BIN" ]] || { echo "binary-only-recipe.test: gate binary not found: $BIN"; exit 2; }

scratch="$(cd -P "$(mktemp -d)" && pwd -P)"
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT

fails=0
assert_rc()    { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; [[ -n "${4:-}" ]] && printf '%s\n' "$4"; fails=$((fails + 1)); }; }
assert_clean() {
    local st
    st="$(git -C "$repo" status --porcelain --untracked-files=all --ignored)"
    [[ -z "$st" ]] || { echo "FAIL [$1]: the checkout's status is not empty:"; printf '%s\n' "$st"; fails=$((fails + 1)); }
}

# The container: the binary, git, PATH and HOME, and nothing a vendored kit or a knob file would set.
mkdir -p "$scratch/home"
container() {
    env -i PATH="$PATH" HOME="$scratch/home" TMPDIR="$scratch" \
        GIT_CONFIG_GLOBAL="$GIT_CONFIG_GLOBAL" GIT_CONFIG_NOSYSTEM=1 \
        GIT_CEILING_DIRECTORIES="$scratch" "$@"
}

repo="$scratch/repo"
mkdir -p "$repo/crates/a" "$repo/owned" "$repo/.github/workflows"
printf '# A spec\n\nbody\n\n## Definition of Done\n\n- x\n' > "$repo/crates/a/SPEC.md"
printf '# a\n\n## What changes\n\n### (1) x\n\ny\n\n## Retired spellings\n\n- None — nothing\n' > "$repo/crates/a/SPEC-x.md"
sha=0123456789abcdef0123456789abcdef01234567
printf 'on: push\njobs:\n  a:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@%s\n' "$sha" > "$repo/owned/ci.yml"
printf 'on: push\njobs:\n  a:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n' > "$repo/.github/workflows/upstream.yml"
git -C "$repo" init -q
git -C "$repo" add -A
git -C "$repo" -c user.name=t -c user.email=t@e.invalid commit -qm base

# ---- the direct form, on a read-only checkout ------------------------------------
chmod -R a-w "$repo"
for g in check-spec-dod-singleton check-amendment-update-target check-amendment-retired-spelling check-spec-fence-balance; do
    out="$(cd "$repo" && container "$BIN" "$g" 2>&1)"; rc=$?
    assert_rc "direct $g" "$rc" 0 "$out"
done
# spec: gate-sdk/SPEC.md §The install disposition — the scan root is the adopter's scope: the
# upstream workflow reds a whole-tree scan, and the owned directory alone passes
out="$(cd "$repo" && container "$BIN" check-action-pinning 2>&1)"; rc=$?
assert_rc "root positional, whole tree" "$rc" 1 "$out"
out="$(cd "$repo" && container "$BIN" check-action-pinning owned 2>&1)"; rc=$?
assert_rc "root positional, owned" "$rc" 0 "$out"
out="$(cd "$repo" && container CANON_KIT_KNOB_FILE="$scratch/absent.knobs" "$BIN" check-spec-dod-singleton 2>&1)"; rc=$?
assert_rc "missing knob file" "$rc" 2 "$out"
chmod -R u+w "$repo"
assert_clean direct

# a finding is exit 1, and a knob file outside the checkout configures the gate away from it
printf '# A spec\n\nbody\n' > "$repo/crates/a/SPEC.md"
out="$(cd "$repo" && container "$BIN" check-spec-dod-singleton 2>&1)"; rc=$?
assert_rc "finding" "$rc" 1 "$out"
printf 'CANON_KIT_DOD_MODE = at-most-one\n' > "$scratch/canon.knobs"
out="$(cd "$repo" && container CANON_KIT_KNOB_FILE="$scratch/canon.knobs" "$BIN" check-spec-dod-singleton 2>&1)"; rc=$?
assert_rc "knob file outside the checkout" "$rc" 0 "$out"
git -C "$repo" checkout -q -- crates/a/SPEC.md

# a top-level directory holding checks/ reads as a kit root and leaves the scan; the override keeps it in
mkdir -p "$repo/tools/checks"
printf '# A tool spec\n\nbody\n' > "$repo/tools/SPEC.md"
out="$(cd "$repo" && container "$BIN" check-spec-dod-singleton 2>&1)"; rc=$?
assert_rc "derived kit roots" "$rc" 0 "$out"
out="$(cd "$repo" && container GATE_SDK_KIT_DIRS=gate-sdk "$BIN" check-spec-dod-singleton 2>&1)"; rc=$?
assert_rc "kit-dirs override" "$rc" 1 "$out"
rm -rf "$repo/tools"

# a gate that reads history needs a worktree: a retired spelling declared on a tree without git is exit 2
mkdir -p "$scratch/nogit"
cp -R "$repo/crates" "$scratch/nogit/"
printf '# a\n\n## What changes\n\n### (1) x\n\ny\n\n## Retired spellings\n\n- `oldname` — gone (delta 1)\n' > "$scratch/nogit/crates/a/SPEC-x.md"
out="$(cd "$scratch/nogit" && container "$BIN" check-amendment-retired-spelling 2>&1)"; rc=$?
assert_rc "no worktree" "$rc" 2 "$out"

# ---- a set with a SARIF verdict, through a registry outside the checkout ---------
gates="$scratch/gates"
mkdir -p "$gates"
printf 'check-spec-dod-singleton\ncheck-spec-fence-balance\n' > "$gates/gates.list"
cp "$SRC/canon-kit/checks/check-spec-dod-singleton.gate" "$SRC/canon-kit/checks/check-spec-fence-balance.gate" "$gates/"
sarif="$scratch/gates.sarif"
out="$(cd "$repo" && container GATE_SDK_SARIF_FILE="$sarif" GATE_SDK_TMP_DIR="$scratch/run" "$BIN" --run --gates-dir "$gates" 2>&1)"; rc=$?
assert_rc "run" "$rc" 0 "$out"
[[ -s "$sarif" ]] || { echo "FAIL [run]: no SARIF log at $sarif"; fails=$((fails + 1)); }
assert_clean run

# spec: gate-sdk/SPEC.md §The install disposition — a registered member scans from the tree's root, so
# an action gate is scoped inside the battery by a member of the adopter's own calling the direct form
printf 'check-action-pinning\n' > "$gates/gates.list"
cp "$SRC/gate-sdk/checks/check-action-pinning.gate" "$gates/"
out="$(cd "$repo" && container GATE_SDK_TMP_DIR="$scratch/run" "$BIN" --run --gates-dir "$gates" 2>&1)"; rc=$?
assert_rc "run, registered action gate" "$rc" 1 "$out"
{ printf '#!/usr/bin/env bash\n'
  printf '# graph: couples=owned dir=one valve=none tier=precommit trigger=owned/*\n'
  printf 'exec %q check-action-pinning owned\n' "$BIN"
} > "$gates/owned-action-pinning.sh"
chmod +x "$gates/owned-action-pinning.sh"
printf 'owned-action-pinning\n' > "$gates/gates.list"
out="$(cd "$repo" && container GATE_SDK_TMP_DIR="$scratch/run" "$BIN" --run --gates-dir "$gates" 2>&1)"; rc=$?
assert_rc "run, scoped member" "$rc" 0 "$out"
assert_clean "run, scoped member"
printf 'check-spec-dod-singleton\ncheck-spec-fence-balance\n' > "$gates/gates.list"
rm -f "$gates/owned-action-pinning.sh" "$gates/check-action-pinning.gate"

# the descriptor is the registry's: a list naming compiled-in gates with no descriptor beside it resolves nothing
rm -f "$gates"/*.gate
out="$(cd "$repo" && container GATE_SDK_TMP_DIR="$scratch/run" "$BIN" --run --gates-dir "$gates" 2>&1)"; rc=$?
assert_rc "run, no descriptor" "$rc" 1 "$out"
grep -qF '(unresolved)' <<<"$out" || { echo "FAIL [run, no descriptor]: no member reported unresolved"; printf '%s\n' "$out"; fails=$((fails + 1)); }
assert_clean "run, no descriptor"

[[ "$fails" -eq 0 ]] || { echo "binary-only-recipe.test: $fails assertion(s) failed"; exit 1; }
echo "binary-only-recipe.test: clean (the direct form's four kit-free gates green on a read-only checkout, a finding at exit 1, a missing knob file and a worktree-less tree at exit 2, the root positional narrowing an action gate's scan to the owned directory, a knob file outside the checkout read, a top-level checks/ directory scanned once GATE_SDK_KIT_DIRS names the kit set, and --run over an out-of-tree registry writing its SARIF log outside a checkout left clean, an action gate scoped there by a member of the adopter's own, its members unresolved once its descriptors are gone)"
exit 0
