#!/usr/bin/env bash
# graph: couples=docs/*/SPEC.md,docs/*/README.md,docs/doctrine-kit/DOCTRINE.md,kit:SPEC.md,kit:README.md,doctrine-kit/DOCTRINE.md dir=one valve=none tier=precommit trigger=docs/*/SPEC.md,docs/*/README.md,docs/doctrine-kit/DOCTRINE.md,*/SPEC.md,*/README.md,doctrine-kit/DOCTRINE.md,scripts/gen-docs-mirror.sh
# spec: canon-kit/SPEC.md §The reference-link grammar — docs/<kit>/{SPEC,README}.md and docs/doctrine-kit/DOCTRINE.md are the byte-fresh projection of gen-docs-mirror.sh backing on-site reference reading; a stale, missing, or orphaned mirror page reds
#
# usage: check-docs-mirror-fresh.sh [root]   (default .; the fixture points it at a synthetic tree)
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-.}"; ROOT="${ROOT%/}"
[[ -d "$ROOT" ]] || { echo "check-docs-mirror-fresh: not a directory: $ROOT" >&2; exit 2; }
GEN="${BASH_SOURCE[0]%/*}/gen-docs-mirror.sh"
[[ -x "$GEN" ]] || { echo "check-docs-mirror-fresh: generator not found: $GEN" >&2; exit 2; }

mapfile -t srcs < <(bash "$GEN" --list --root "$ROOT"); st=$?
fail_closed "$st" check-docs-mirror-fresh generator

bad=(); n=0
declare -A expected_dest=()
for src in "${srcs[@]+"${srcs[@]}"}"; do
    dest="docs/$src"
    expected_dest["$dest"]=1
    n=$((n + 1))
    if [[ ! -f "$ROOT/$dest" ]]; then
        bad+=("$dest: missing — the generator emits it but the tree has no such mirror page")
        continue
    fi
    emitted="$(bash "$GEN" --emit "$src" --root "$ROOT")"; est=$?
    fail_closed "$est" check-docs-mirror-fresh emit
    if [[ "$emitted" != "$(cat "$ROOT/$dest")" ]]; then
        bad+=("$dest: stale vs gen-docs-mirror.sh --emit $src")
    fi
done

while IFS= read -r f; do                          # orphans: a mirror page whose source is gone
    rel="${f#"$ROOT"/}"
    [[ -n "${expected_dest[$rel]:-}" ]] || bad+=("$rel: orphaned — no source doc maps to this mirror page (delete it and rerun the generator)")
done < <(find "$ROOT/docs" -type f \( -name SPEC.md -o -name README.md -o -name DOCTRINE.md \) 2>/dev/null | sort)

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-docs-mirror-fresh: the on-site SPEC mirror is out of sync with its sources:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: regenerate — bash scripts/gen-docs-mirror.sh --write — and stage docs/."
    exit 1
fi
echo "DOCS-MIRROR-FRESH: clean ($n mirror page(s) byte-match gen-docs-mirror.sh; no orphans)"
exit 0
