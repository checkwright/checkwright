#!/usr/bin/env bash
# graph: couples=.workflow/gap-inbox.md dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-gap-inbox-neutrality — the capture surface records observations: every bullet is '- <YYYY-MM-DD> — <prose>', and no bullet's prose opens with the retired 'recurrence of `<slug>`:' verdict
#
# usage: check-gap-inbox-neutrality.sh [inbox-file]
#   bare drives LIFECYCLE_KIT_GAP_INBOX_FILE; an explicit file argument drives it hermetically
#   (the check-survey-record precedent) so the fixture pair runs against a copied inbox.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

if [[ -n "${1:-}" ]]; then
    INBOX="$1"
    [[ -f "$INBOX" ]] || { echo "check-gap-inbox-neutrality: inbox file not found: $INBOX" >&2; exit 2; }
else
    INBOX="$LIFECYCLE_KIT_GAP_INBOX_FILE"
    # spec: lifecycle-kit/SPEC.md §check-gap-inbox-neutrality — an absent inbox is clean, not fail-closed: never having filed a gap is a legal state for a fresh consumer, unlike check-lifecycle-registration's missing agent file
    [[ -f "$INBOX" ]] || { echo "GAP-INBOX-NEUTRALITY: clean (no inbox at $INBOX — no gap filed this iteration)"; exit 0; }
fi
[[ -r "$INBOX" ]] || { echo "check-gap-inbox-neutrality: inbox file not readable: $INBOX" >&2; exit 2; }

parsed="$(awk '
    FNR == 1 && /^#[[:space:]]/ { next }
    /^[[:space:]]*$/ { next }
    {
        bullets++
        if ($0 !~ /^-[[:space:]][0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9][[:space:]]—[[:space:]]*[^[:space:]]/) {
            print "BAD\t" FNR "\tnot a gap bullet — the grammar is '\''- <YYYY-MM-DD> — <prose>'\'' with non-empty prose"
            next
        }
        prose = $0
        sub(/^-[[:space:]][0-9-]+[[:space:]]—[[:space:]]*/, "", prose)
        if (prose ~ /^recurrence of `[a-z0-9][a-z0-9-]*`:/)
            print "BAD\t" FNR "\tthe prose opens with a retired '\''recurrence of `<slug>`:'\'' verdict — the capture surface carries observations, not conclusions"
    }
    END { print "COUNT\t" bullets + 0 "\t" }
' "$INBOX")"; st=$?
fail_closed "$st" GAP-INBOX-NEUTRALITY awk

findings=(); bullets=0
while IFS=$'\t' read -r kind where what; do
    case "$kind" in
        BAD) findings+=("$INBOX:$where: $what") ;;
        COUNT) bullets="$where" ;;
    esac
done <<<"$parsed"

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-gap-inbox-neutrality: ${#findings[@]} malformed or verdict-bearing bullet(s) in $INBOX:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: the gap inbox records what a filer observed, and the closing stage's drain is what judges it. File with 'bash lifecycle-kit/bin/file-gap.sh \"<gap prose>\"', which stamps the one legal bullet shape. A bullet that re-files a live entry says so in its own prose — write why you believe it re-occurred and let the drain rule on it; never open the prose with a 'recurrence of <slug>:' verdict, which states a conclusion the capture channel has no standing to reach."
    exit 1
fi
echo "GAP-INBOX-NEUTRALITY: clean ($bullets bullet(s) in $INBOX; every bullet is dated prose and none opens with a recurrence verdict)"
exit 0
