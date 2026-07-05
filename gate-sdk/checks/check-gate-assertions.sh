#!/usr/bin/env bash
# graph: couples=gate-sdk/SPEC.md,lifecycle-kit/SPEC.md,queue-kit/SPEC.md,spec-kit/SPEC.md,delegation-kit/SPEC.md,scripts/*.sh,gate-sdk/*.sh,lifecycle-kit/*.sh,queue-kit/*.sh,spec-kit/*.sh,delegation-kit/*.sh dir=bi valve=none tier=align-only
# spec: gate-sdk/SPEC.md §check-gate-assertions — couple each §<gate> enumerated-assertion span+count to the gate code's `# assertion` markers
#
# usage: check-gate-assertions.sh [spec [scripts-dir]]
#   With no spec argument, scans <gates-dir>/SPEC.md when present plus each
#   vendored kit's own SPEC.md. With no scripts-dir, each §<gate> resolves
#   against the consumer gates dir, then each kit's checks/. Requires GNU awk
#   (3-arg match).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR="$(gate_sdk_gates_dir)"
SPECS=()
if [[ $# -gt 0 ]]; then
    SPECS=("$1")
else
    [[ -f "$GATES_DIR/SPEC.md" ]] && SPECS+=("$GATES_DIR/SPEC.md")
    while IFS= read -r k; do
        [[ -f "$k/SPEC.md" ]] && SPECS+=("$k/SPEC.md")
    done < <(gate_kit_roots)
fi
SCRIPTS_DIR="${2:-}"
[[ ${#SPECS[@]} -gt 0 ]] || { echo "check-gate-assertions: no SPEC.md found (run from repo root)" >&2; exit 2; }
for s in "${SPECS[@]}"; do
    [[ -f "$s" ]] || { echo "check-gate-assertions: not found: $s (run from repo root)" >&2; exit 2; }
done

extract_contracts() { awk '
  function wn(w) {
    if (w=="two")return 2; if (w=="three")return 3; if (w=="four")return 4
    if (w=="five")return 5; if (w=="six")return 6; if (w=="seven")return 7
    if (w=="eight")return 8; if (w=="nine")return 9; return 0
  }
  /^### / { if (h!="") emit(); h=$0; sub(/^### /,"",h); p=""; started=0; done=0; next }
  /^## /  { if (h!="") { emit(); h="" } next }
  {
    if (h=="" || done) next
    if (!started) { if ($0 ~ /^[[:space:]]*$/) next; started=1; p=$0; next }
    if ($0 ~ /^[[:space:]]*$/) { done=1; next }
    p = p " " $0
  }
  END { if (h!="") emit() }
  function emit(   low,rx,arr,lab,after,ns,nl,n,cnt,csv,i,labs) {
    low = tolower(p)
    rx = "(^|[^a-z])(two|three|four|five|six|seven|eight|nine)[[:space:]]+([a-z]+[[:space:]]+)?(assertion|assertions|axes|axis|checks)([^a-z]|$)"
    if (!match(low, rx)) return
    ns = RSTART; nl = RLENGTH
    if (match(substr(low, ns, nl), /(two|three|four|five|six|seven|eight|nine)/, arr)) n = wn(arr[1])
    after = substr(p, ns + nl - 1)
    if (match(after, /\([^)]*\)/, arr)) {
      if (arr[0] !~ /^\([A-Za-z0-9]\)$/) return
    } else return
    delete seen; cnt = 0; csv = ""
    while (match(after, /\(([A-Za-z0-9])\)/, arr)) {
      lab = arr[1]
      if (!(lab in seen)) { seen[lab]=1; cnt++; csv = (csv==""?lab:csv","lab) }
      after = substr(after, RSTART + RLENGTH)
    }
    if (cnt >= 2) printf "%s|%d|%s\n", h, n, csv
  }
' "$1"; }

resolve_gate_file() {
    if [[ -n "$SCRIPTS_DIR" ]]; then
        [[ -f "$SCRIPTS_DIR/$1.sh" ]] && { printf '%s\n' "$SCRIPTS_DIR/$1.sh"; return 0; }
        return 1
    fi
    local -a dirs
    mapfile -t dirs < <(gate_check_dirs)
    gate_resolve "$1" "${dirs[@]}"
}

findings=()
coupled=0
for spec in "${SPECS[@]}"; do
    contracts="$(extract_contracts "$spec")"; st=$?
    fail_closed "$st" check-gate-assertions awk

    while IFS='|' read -r gate n labels; do
        [[ -z "$gate" ]] && continue
        coupled=$((coupled + 1))

        lcount="$(awk -F, '{print NF}' <<< "$labels")"
        if [[ "$lcount" -ne "$n" ]]; then
            findings+=("$spec §$gate: count-word says $n but the (X) span enumerates $lcount label(s) [$labels] — the contract is internally inconsistent")
        fi

        if ! file="$(resolve_gate_file "$gate")"; then
            findings+=("$spec §$gate: enumerated contract but no gate code resolves for '$gate' (heading must name the script)")
            continue
        fi

        markers="$(grep -oE '#[[:space:]]*assertion[[:space:]]+[A-Za-z0-9]+:' "$file" \
            | sed -E 's/#[[:space:]]*assertion[[:space:]]+([A-Za-z0-9]+):/\1/' | sort -u | paste -sd, -)"

        if [[ -z "$markers" ]]; then
            findings+=("$spec §$gate: contract enumerates [$labels] but $file carries zero \`# assertion\` markers (retrofit obligation)")
            continue
        fi

        want="$(tr ',' '\n' <<< "$labels" | sort -u | paste -sd, -)"
        have="$markers"
        if [[ "$want" != "$have" ]]; then
            missing="$(comm -23 <(tr ',' '\n' <<< "$want") <(tr ',' '\n' <<< "$have") | paste -sd, -)"
            extra="$(comm -13 <(tr ',' '\n' <<< "$want") <(tr ',' '\n' <<< "$have") | paste -sd, -)"
            msg="$spec §$gate: marker set [$have] != contract span [$want]"
            [[ -n "$missing" ]] && msg="$msg; missing marker(s): $missing"
            [[ -n "$extra" ]] && msg="$msg; extra marker(s): $extra"
            findings+=("$msg")
        fi
    done <<< "$contracts"
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-gate-assertions: §<gate> assertion enumeration ↔ gate-code marker mismatch:"
    for f in "${findings[@]}"; do echo "  $f"; done
    echo "  help: add/align a '# assertion <label>: <tag>' marker per enumerated assertion in the gate code so its label set matches the §<gate> contract span (and the count-word), or fix the contract's enumeration"
    exit 1
fi

echo "GATE-ASSERTIONS: clean ($coupled enumerated contract(s) coupled)"
exit 0
