#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §check-reads-couples — the census arm's properties that a crate unit test
# cannot reach, because each is a property of the arm as a *dispatched* member rather than of the
# function behind it. (1) It resolves through the bridged-arm table, so `--emit reads-census` is a
# live spelling and not an unreachable function. (2) Its two derivable totals agree with `--reads`,
# the other arm over the same registry field — the two-oracle agreement that makes a census an
# oracle rather than a second transcription. (3) The report carries no count line and no header,
# which is the property that keeps the totals derivable rather than stated.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

fails=0
checks=0
tab=$'\t'
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

# --- (1) the arm dispatches through the bridge, at the spelling the roster publishes ----------
checks=$((checks + 1))
census="$(gate_arm_run --emit-reads-census)"
rc=$?
[[ "$rc" -eq 0 ]] || note dispatch "the arm exited $rc rather than 0"
[[ -n "$census" ]] || note dispatch "the arm printed nothing"

# --- (3) every line is four tab-separated columns: no header, no count line, no blank ----------
checks=$((checks + 1))
while IFS= read -r line; do
    [[ -n "$line" ]] || { note shape "the report carries a blank line"; continue; }
    cols="$(awk -F'\t' '{print NF}' <<<"$line")"
    [[ "$cols" -eq 4 ]] || note shape "a line carries $cols columns rather than 4: $line"
    name="$(cut -f1 <<<"$line")"
    count="$(cut -f2 <<<"$line")"
    [[ "$name" == check-* ]] || note shape "column 1 is not a member name: $line"
    [[ "$count" =~ ^[1-9][0-9]*$ ]] || note shape "column 2 is not a positive count: $line"
done <<<"$census"

# --- (2) both derivable totals agree with --reads, member by member ---------------------------
# The census is only an oracle if it answers what a per-member sweep answers. This is the
# assertion the hand sweeps this arm replaces would have failed.
checks=$((checks + 1))
bin="$GATE_SDK_NATIVE_BIN"
sweep_members=0
sweep_roots=0
while read -r g _rest; do
    [[ -n "$g" ]] || continue
    n="$("$bin" --reads "$g" | grep -c "^?$tab")"
    if [[ "$n" -gt 0 ]]; then
        sweep_members=$((sweep_members + 1))
        sweep_roots=$((sweep_roots + n))
    fi
done < <("$bin" --list)

census_members="$(grep -c . <<<"$census")"
census_roots="$(awk -F'\t' '{s += $2} END {print s + 0}' <<<"$census")"

[[ "$census_members" -eq "$sweep_members" ]] || note agreement \
    "the census reports $census_members members where --reads sweeps $sweep_members"
[[ "$census_roots" -eq "$sweep_roots" ]] || note agreement \
    "the census sums to $census_roots ? roots where --reads sweeps $sweep_roots"

# --- (2b) each censused member's own count is that member's --reads count ----------------------
checks=$((checks + 1))
while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    name="$(cut -f1 <<<"$line")"
    count="$(cut -f2 <<<"$line")"
    actual="$("$bin" --reads "$name" | grep -c "^?$tab")"
    [[ "$count" -eq "$actual" ]] || note per-member \
        "$name is censused at $count ? roots but --reads reports $actual"
    # (2d) the ground column is the grounds --reads prints on that member's ? lines, in order
    grounds="$(cut -f4 <<<"$line")"
    reads_grounds="$("$bin" --reads "$name" | awk -F'\t' '$1 == "?" {printf "%s%s", sep, $2; sep = ","}')"
    [[ "$grounds" == "$reads_grounds" ]] || note ground \
        "$name is censused with grounds '$grounds' but --reads prints '$reads_grounds'"
done <<<"$census"

# --- (2c) a member the census omits declares no ? at all --------------------------------------
# The omission half is what makes the line count the population; without it a census that
# dropped members would still pass every assertion above.
checks=$((checks + 1))
omitted_with_q=0
while read -r g _rest; do
    [[ -n "$g" ]] || continue
    grep -qx "$g" < <(cut -f1 <<<"$census") && continue
    n="$("$bin" --reads "$g" | grep -c "^?$tab")"
    [[ "$n" -eq 0 ]] || { omitted_with_q=$((omitted_with_q + 1)); note omission \
        "$g declares $n ? root(s) and is absent from the census"; }
done < <("$bin" --list)

if [[ "$fails" -gt 0 ]]; then
    echo "reads-census.test.sh: $fails case(s) failed"
    exit 1
fi
echo "reads-census.test.sh: clean (the arm dispatches through the bridge, every line is four columns with no header or count line, and both derivable totals plus every per-member count and ground column agree with a --reads sweep over all members — $census_members members, $census_roots ? roots; $checks checks)"
exit 0
