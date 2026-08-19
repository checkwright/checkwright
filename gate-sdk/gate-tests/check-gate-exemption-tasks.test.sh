#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the scope rule's other half, which the
# good/bad pair structurally cannot express: the pair runs in a case dir carrying no crate source,
# so every case there is the adopter reading. This stands the AUTHORING tree up — a sandbox whose
# crate path holds tracked source — and asserts that a kit-shipped declaration's stale disposition
# is a violation there, on both arms. Without this, the loosening the adopter cases prove would be
# indistinguishable from the gate having stopped asserting kit declarations altogether.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/

fails=0
sb="$(mktemp -d)"
trap 'rm -rf "$sb"' EXIT

mkdir -p "$sb/scripts" "$sb/kitroot/checks" "$sb/native/src"
cat > "$sb/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE — synthetic queue

## New Features

- **a-live-task** — the only slug this queue carries.

## Done

- **retired-task** — completed.
EOF
cat > "$sb/kitroot/checks/check-vendored.sh" <<'EOF'
#!/usr/bin/env bash
# graph: couples=docs/*.md dir=one valve=none tier=precommit
# port-until: retired-task
# exception-list: surfaces excused
EXEMPT=(
    "kit-surface"   # until: retired-task
)
EOF

run() { ( cd "$sb" && gate_run check-gate-exemption-tasks "$DIR/checks" TASK-QUEUE.md scripts kitroot/checks 2>&1 ); }

# --- adopter reading: no crate source, so the kit-shipped declaration is out of scope ---
out="$(run)"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [adopter-clean]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF '1 kit-shipped declaration(s) out of scope' <<<"$out"; then
    echo "  FAIL [adopter-counted]: the skip is not reported, so it is indistinguishable from an empty corpus: $out"
    fails=$((fails + 1))
fi

# --- authoring reading: the crate's tracked source is here, so the same declaration is asserted ---
git -C "$sb" init -q
git -C "$sb" config user.email t@example.invalid
git -C "$sb" config user.name t
printf 'fn main() {}\n' > "$sb/native/src/main.rs"
git -C "$sb" add native/src/main.rs
out="$(run)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [authoring-reds]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
else
    for want in '# port-until: retired-task does not resolve' '# until: retired-task does not resolve'; do
        grep -qF -- "$want" <<<"$out" \
            || { echo "  FAIL [authoring-arms]: output lacks '$want': $out"; fails=$((fails + 1)); }
    done
fi

# --- untracked crate source is not authorship: a build artifact under the crate path is not source ---
git -C "$sb" rm -q --cached native/src/main.rs
out="$(run)"; rc=$?
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL [untracked-crate-is-not-authorship]: want exit 0, got $rc -- $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-exemption-tasks.test.sh: $fails assertion(s) failed"
    exit 1
fi
echo "check-gate-exemption-tasks.test.sh: clean (a kit-shipped declaration's stale disposition is out of scope and counted in an adopter tree, a violation on both arms in the authoring tree, and an untracked crate path is not authorship — 3 cases)"
exit 0
