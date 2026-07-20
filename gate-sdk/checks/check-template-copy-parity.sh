#!/usr/bin/env bash
# graph: couples=*/templates/*.sh,scripts/*.sh dir=bi valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-template-copy-parity — a kit template and its vendored consumer copy agree on their declared contract surface, with copy-side additions declared
#
# usage: check-template-copy-parity.sh [root]
#   root defaults to the git toplevel; the pairing is <root>/*/templates/<n>.sh
#   <-> <root>/<gates-dir>/<n>.sh, minus the *-config.sh suffix exclusion.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-}"
if [[ -z "$ROOT" ]]; then
    ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
        || { echo "check-template-copy-parity: not a git repository and no root given" >&2; exit 2; }
fi
[[ -d "$ROOT" ]] || { echo "check-template-copy-parity: root not a directory: $ROOT" >&2; exit 2; }

GATES_DIR="$(gate_sdk_gates_dir)"

# spec: gate-sdk/SPEC.md §check-template-copy-parity — assertion A reads the resolved `<file> §<section>` target, never the trailing prose: a pair may gloss one target two ways deliberately
spec_target() {
    sed -n 's/^# spec:[[:space:]]*//p' "$1" 2>/dev/null | head -1 \
        | sed 's/[[:space:]]*—.*$//; s/[[:space:]]*$//'
}

# spec: gate-sdk/SPEC.md §check-template-copy-parity — the declared surface: four classes of *declaration*, never content. `case` arms contribute the arm's exit token (its first command word), never the arm's pattern — a consumer's rule patterns are its own vocabulary and the gate must not read them.
declared_surface() {
    local f="$1"
    {
        sed -n -E 's/^[[:space:]]*(function[[:space:]]+)?([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*\(\)[[:space:]]*\{?.*$/func:\2/p' "$f"
        # comment-tier-exempt: local parser fact, below SPEC altitude — an arm opens only where a `;;` (or the `in`) last closed one, so a `)` inside an arm body's prose is not read as a new arm
        awk '
            /(^|[[:space:];])case[[:space:]].*[[:space:]]in[[:space:]]*$/ { in_case=1; expect=1; next }
            /^[[:space:]]*esac([[:space:]]|$)/ { in_case=0; expect=0; want=0; next }
            in_case {
                line=$0
                if (expect && index(line, ")")) {
                    line=substr(line, index(line, ")")+1)   # first ")" closes the pattern
                    expect=0; want=1
                }
                if (want && match(line, /[A-Za-z_][A-Za-z0-9_]*/)) {
                    print "case:" substr(line, RSTART, RLENGTH)
                    want=0
                }
                if (index($0, ";;")) { expect=1; want=0 }
            }
        ' "$f"
        grep -oE '(^|[;&|)]|\$\(|&&|\|\|)[[:space:]]*[a-z_][a-z0-9]*_[a-z0-9_]+([[:space:]]|\))' "$f" \
            | grep -oE '[a-z_][a-z0-9]*_[a-z0-9_]+' | sed 's/^/lib:/'
        grep -oE '\$\{[A-Z][A-Z0-9]*_[A-Z0-9_]+:[-=]' "$f" \
            | sed -E 's/^\$\{//; s/:[-=]$//; s/^/knob:/'
    } 2>/dev/null | sort -u
}

# spec: gate-sdk/SPEC.md §check-template-copy-parity — the `# copy-divergence:` marker grammar assertion C reads
divergence_reasons() {
    sed -n 's/^[[:space:]]*#[[:space:]]*copy-divergence:[[:space:]]*//p' "$1" 2>/dev/null
}

pairs=0
findings=()

shopt -s nullglob
templates=("$ROOT"/*/templates/*.sh)
shopt -u nullglob

for tpl in "${templates[@]}"; do
    name="${tpl##*/}"
    [[ "$name" == *-config.sh ]] && continue      # a config template is a starting point; divergence is its contract
    copy="$ROOT/$GATES_DIR/$name"
    [[ -f "$copy" ]] || continue                  # unpaired: not vendored here, nothing to be in parity with
    pairs=$((pairs + 1))
    rel_copy="${copy#"$ROOT"/}"

    # assertion A — same resolved spec: target
    t_target="$(spec_target "$tpl")"
    c_target="$(spec_target "$copy")"
    if [[ "$t_target" != "$c_target" ]]; then
        findings+=("$rel_copy: spec: target differs from its template")
        findings+=("    template: ${t_target:-<none>}")
        findings+=("    copy:     ${c_target:-<none>}")
    fi

    t_surface="$(declared_surface "$tpl")"; st=$?
    fail_closed "$st" check-template-copy-parity "declared_surface"
    c_surface="$(declared_surface "$copy")"; st=$?
    fail_closed "$st" check-template-copy-parity "declared_surface"

    # assertion B — the template's declared surface is present in the copy
    missing="$(comm -23 <(printf '%s\n' "$t_surface") <(printf '%s\n' "$c_surface") | grep -v '^$' || true)"
    if [[ -n "$missing" ]]; then
        findings+=("$rel_copy: drops surface the template declares (a template-side change never propagated, or a copy-side removal):")
        while IFS= read -r tok; do findings+=("    $tok"); done <<<"$missing"
    fi

    # assertion C — copy-side additions are declared by a marker naming them
    extra="$(comm -13 <(printf '%s\n' "$t_surface") <(printf '%s\n' "$c_surface") | grep -v '^$' || true)"
    if [[ -n "$extra" ]]; then
        reasons="$(divergence_reasons "$copy")"
        undeclared=()
        while IFS= read -r tok; do
            bare="${tok#*:}"
            grep -qF -- "$bare" <<<"$reasons" || undeclared+=("$tok")
        done <<<"$extra"
        if [[ ${#undeclared[@]} -gt 0 ]]; then
            findings+=("$rel_copy: adds surface no '# copy-divergence:' marker names:")
            for tok in "${undeclared[@]}"; do findings+=("    $tok"); done
        fi
    fi
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-template-copy-parity: template <-> consumer-copy contract surface diverged:"
    printf '%s\n' "${findings[@]}"
    echo "  help: assertion A — point both copies' 'spec:' line at the same '<file> §<section>'"
    echo "        (trailing prose may differ). Assertion B — propagate the template's"
    echo "        declaration to the copy, or retire it from the template. Assertion C —"
    echo "        add a '# copy-divergence: <reason>' line to the copy whose reason names"
    echo "        the added token and says why the copy needs it."
    exit 1
fi
echo "TEMPLATE-COPY-PARITY: clean ($pairs template<->copy pair(s) agree on spec: target, template-declared surface present, copy additions declared)"
exit 0
