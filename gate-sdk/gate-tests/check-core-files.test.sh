#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §check-core-files — a kit: manifest line derives one path per kit root, so a root missing that path reds without any manifest edit; a kit: token carrying a wildcard is refused fail-closed because this reader requires each expanded path to exist, which is a different invariant from the glob match a couples= field performs. This lives in the bespoke unit lane rather than the good/+bad/ pair because the fixture runner reads exit 2 as a harness error and runs each case with the fixture dir as cwd, so neither the refusal nor the expansion is expressible there.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null || pwd)"
CHECKS="$ROOT/gate-sdk/checks"

sb="$(mktemp -d)"
trap 'rm -rf "$sb"' EXIT

git -C "$sb" init -q
git -C "$sb" config user.email t@example.invalid
git -C "$sb" config user.name t
mkdir -p "$sb/alpha-kit" "$sb/beta-kit"
printf 'a\n' > "$sb/alpha-kit/SPEC.md"
printf 'b\n' > "$sb/beta-kit/SPEC.md"
printf 'r\n' > "$sb/plain.txt"
git -C "$sb" add -A
git -C "$sb" commit -qm seed

fails=0
run() { ( cd "$sb" && GATE_SDK_KIT_DIRS="alpha-kit beta-kit" gate_run check-core-files "$CHECKS" "$1" 2>&1 ); }

# A kit: line expands to one path per root; every expansion exists, so the gate is clean
# and its count proves the derivation ran (2 derived + 1 hand line).
printf 'kit:SPEC.md\nplain.txt\n' > "$sb/good.list"
out="$(run good.list)"; rc=$?
[[ "$rc" -eq 0 ]] || { echo "FAIL [expand-clean]: expected exit 0, got $rc: $out"; fails=$((fails + 1)); }
grep -qF '3 manifest path(s)' <<<"$out" || { echo "FAIL [expand-count]: expected 3 expanded paths: $out"; fails=$((fails + 1)); }

# A new root with no SPEC.md reds without any manifest edit — the property Delta 2 exists for.
mkdir -p "$sb/gamma-kit"
printf 'g\n' > "$sb/gamma-kit/other.md"
git -C "$sb" add -A
git -C "$sb" commit -qm gamma
out="$( cd "$sb" && GATE_SDK_KIT_DIRS="alpha-kit beta-kit gamma-kit" gate_run check-core-files "$CHECKS" good.list 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] || { echo "FAIL [new-root-reds]: expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF 'missing:   gamma-kit/SPEC.md' <<<"$out" || { echo "FAIL [new-root-names]: $out"; fails=$((fails + 1)); }

# An untracked expansion reds on the tracked half of the invariant, not just existence.
printf 'u\n' > "$sb/gamma-kit/SPEC.md"
out="$( cd "$sb" && GATE_SDK_KIT_DIRS="alpha-kit beta-kit gamma-kit" gate_run check-core-files "$CHECKS" good.list 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] || { echo "FAIL [untracked-reds]: expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF 'untracked: gamma-kit/SPEC.md' <<<"$out" || { echo "FAIL [untracked-names]: $out"; fails=$((fails + 1)); }

# A wildcard is refused fail-closed: this reader cannot express "some file matches".
for tok in 'kit:checks/*.sh' 'kit:SPEC?.md' 'kit:[ab].md'; do
    printf '%s\n' "$tok" > "$sb/wild.list"
    out="$(run wild.list)"; rc=$?
    [[ "$rc" -eq 2 ]] || { echo "FAIL [wildcard-refused $tok]: expected exit 2, got $rc: $out"; fails=$((fails + 1)); }
    grep -qF 'carries a wildcard' <<<"$out" || { echo "FAIL [wildcard-message $tok]: $out"; fails=$((fails + 1)); }
done

# Backward compatibility: a manifest with no kit: line behaves exactly as before.
printf 'plain.txt\n' > "$sb/plainonly.list"
out="$(run plainonly.list)"; rc=$?
[[ "$rc" -eq 0 ]] || { echo "FAIL [no-token-unchanged]: expected exit 0, got $rc: $out"; fails=$((fails + 1)); }
grep -qF '1 manifest path(s)' <<<"$out" || { echo "FAIL [no-token-count]: $out"; fails=$((fails + 1)); }

[[ "$fails" -eq 0 ]] || { echo "check-core-files.test: $fails assertion(s) failed"; exit 1; }
echo "check-core-files.test: clean (kit: expands per root and reds on a root missing the path or leaving it untracked; a wildcard token is refused fail-closed; a token-free manifest is unchanged)"
exit 0
