#!/usr/bin/env bash
# graph: couples=.workflow/survey-record.md dir=one valve=none tier=precommit
# install: on-surface
# spec: lifecycle-kit/SPEC.md §check-survey-record — every survey block carries its whole witness: the four keys in order, a full-sha rev naming a real commit, a non-empty corpus and a non-empty oracle
#
# usage: check-survey-record.sh [record-file]
#   bare drives LIFECYCLE_KIT_SURVEY_RECORD_FILE with the full assertion set (grammar + rev existence);
#   an explicit file argument drives it hermetically — grammar only, since a fixture's rev names no
#   commit in the tree the fixture was copied into (the check-lesson-disposition override precedent).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

if [[ -n "${1:-}" ]]; then
    RECORD="$1"; probe_rev=0
    [[ -f "$RECORD" ]] || { echo "check-survey-record: record file not found: $RECORD" >&2; exit 2; }
else
    RECORD="$LIFECYCLE_KIT_SURVEY_RECORD_FILE"; probe_rev=1
    # spec: lifecycle-kit/SPEC.md §check-survey-record — an absent record is clean and counted inert: the surface is optional, and a consumer that never files a survey must not carry a red gate
    [[ -f "$RECORD" ]] || { echo "SURVEY-RECORD: clean (no record at $RECORD — no survey filed this iteration)"; exit 0; }
    git rev-parse --git-dir >/dev/null 2>&1 || probe_rev=0
fi
[[ -r "$RECORD" ]] || { echo "check-survey-record: record file not readable: $RECORD" >&2; exit 2; }

parsed="$(awk '
    BEGIN { n = split("corpus oracle rev finding", want, " ") }
    function finish(   i) {
        if (!blk) return
        for (i = 1; i <= n; i++) {
            if (i > k) { print "BAD\t" blk "\tblock is missing its '\''- " want[i] ":'\'' line"; continue }
            if (keys[i] != want[i])
                print "BAD\t" lns[i] "\texpected the '\''- " want[i] ":'\'' line here, found '\''- " keys[i] ":'\''"
        }
        for (i = n + 1; i <= k; i++)
            print "BAD\t" lns[i] "\tblock carries a fifth key '\''- " keys[i] ":'\'' — the grammar is exactly corpus/oracle/rev/finding"
        blk = 0; k = 0
    }
    /^##[[:space:]]/ { finish(); blk = FNR; blocks++; next }
    !blk { next }
    /^[[:space:]]*$/ { next }
    {
        if ($0 !~ /^-[[:space:]]+[a-z]+:/) {
            print "BAD\t" FNR "\tstray line inside a survey block (the grammar is one '\''- <key>: <value>'\'' line per key)"
            next
        }
        key = $0; sub(/^-[[:space:]]+/, "", key); val = key
        sub(/:.*$/, "", key); sub(/^[a-z]+:[[:space:]]*/, "", val)
        sub(/[[:space:]]+$/, "", val)
        k++; keys[k] = key; vals[k] = val; lns[k] = FNR
        if (key == "corpus" && val == "")
            print "BAD\t" FNR "\tempty corpus — the witness has no pathspec to diff"
        if (key == "oracle" && val == "")
            print "BAD\t" FNR "\tempty oracle — write the grounding command, or the literal '\''none'\'' (which marks the block a note, not a re-usable survey)"
        if (key == "rev") {
            if (val !~ /^[0-9a-f]{40}$/)
                print "BAD\t" FNR "\trev is not a full 40-hex sha: '\''" val "'\''"
            else
                print "REV\t" FNR "\t" val
        }
    }
    END { finish(); print "COUNT\t" blocks + 0 "\t" }
' "$RECORD")"; st=$?
fail_closed "$st" SURVEY-RECORD awk

findings=(); revs=(); rev_lines=(); blocks=0
while IFS=$'\t' read -r kind where what; do
    case "$kind" in
        BAD) findings+=("$RECORD:$where: $what") ;;
        REV) revs+=("$what"); rev_lines+=("$where") ;;
        COUNT) blocks="$where" ;;
    esac
done <<<"$parsed"

# spec: lifecycle-kit/SPEC.md §check-survey-record — the existence probe is the assertion that catches the wrong-rev case the 40-hex shape cannot: a sha the tree does not carry makes 'git diff <rev>..HEAD' fail rather than witness anything
probed=0
if [[ "$probe_rev" == 1 ]]; then
    for i in "${!revs[@]}"; do
        if git cat-file -e "${revs[$i]}^{commit}" 2>/dev/null; then
            probed=$((probed + 1))
        else
            findings+=("$RECORD:${rev_lines[$i]}: rev names no commit in this repository: ${revs[$i]}")
        fi
    done
fi

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-survey-record: ${#findings[@]} malformed survey block(s) in $RECORD:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: each '## <date> <stage> — <question>' block carries exactly four lines — '- corpus:', '- oracle:', '- rev:', '- finding:' — in that order, with a non-empty corpus, a non-empty oracle (the literal 'none' is the honest form for a survey no oracle grounds), and a full 40-hex rev naming a real commit. File blocks with 'bash lifecycle-kit/bin/file-survey.sh \"<question>\" \"<corpus>\" \"<oracle>\" \"<finding>\"', which stamps the rev itself."
    exit 1
fi

if [[ "$probe_rev" == 1 ]]; then
    echo "SURVEY-RECORD: clean ($blocks block(s) in $RECORD; grammar holds and $probed rev(s) name a real commit)"
else
    echo "SURVEY-RECORD: clean ($blocks block(s) in $RECORD; grammar holds — hermetic file argument, so no rev-existence probe)"
fi
exit 0
