#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-price-table-age: age of the price table's priced-as-of: header and time to its optional prices-valid-through: expiry
set -uo pipefail

# spec: drift-kit/SPEC.md §The KPI plugin contract — plugins read exported env only, so the DRIFT_KIT_PRICE_TABLE default is restated here identically to bin/stage-economics.sh's rather than resolved from it
: "${DRIFT_KIT_PRICE_TABLE:=${GATE_SDK_GATES_DIR:-scripts}/price-table.tsv}"

TREND=0
[[ "${1:-}" == "--trend" ]] && TREND=1

age_row()    { (( TREND )) || printf 'lead\tprice table age\t%s\n' "$1"; }
expiry_row() { (( TREND )) || printf 'lead\tprice table expiry\t%s\n' "$1"; }

if [[ ! -f "$DRIFT_KIT_PRICE_TABLE" ]]; then
    age_row 'n/a (no price table)'
    exit 0
fi

hdr_val() {
    awk -v f="$1" '
        $0 ~ "^#[[:space:]]*" f ":" {
            sub("^#[[:space:]]*" f ":[[:space:]]*", "")
            sub("[[:space:]].*$", "")
            print; exit
        }
    ' "$DRIFT_KIT_PRICE_TABLE" 2>/dev/null
}

day_secs() {
    [[ "$1" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || return 1
    date -d "$1" +%s 2>/dev/null
}

now="$(date +%s)"
today="$(date -d "$(date +%F)" +%s 2>/dev/null || printf '%s' "$now")"

priced="$(hdr_val 'priced-as-of')"
if [[ -z "$priced" ]]; then
    age_row 'n/a (no priced-as-of: header)'
elif ! priced_s="$(day_secs "$priced")"; then
    age_row 'n/a (unparseable priced-as-of date)'
else
    age_days=$(( (now - priced_s) / 86400 ))
    (( age_days < 0 )) && age_days=0
    if (( TREND )); then
        printf 'price %dd\n' "$age_days"
        exit 0
    fi
    age_row "$(printf 'priced %dd ago (as-of %s)' "$age_days" "$priced")"
fi

(( TREND )) && exit 0

through="$(hdr_val 'prices-valid-through')"
if [[ -z "$through" ]]; then
    expiry_row 'n/a (no prices-valid-through: header)'
elif ! through_s="$(day_secs "$through")"; then
    expiry_row 'n/a (unparseable prices-valid-through date)'
else
    left_days=$(( (through_s - today) / 86400 ))
    if (( left_days >= 0 )); then
        expiry_row "$(printf 'expires in %dd (through %s)' "$left_days" "$through")"
    else
        expiry_row "$(printf 'EXPIRED %dd ago — re-verify (through %s)' "$(( -left_days ))" "$through")"
    fi
fi

exit 0
