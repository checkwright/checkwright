#!/usr/bin/env bash
# spec: site-kit/SPEC.md §templates/site-health.yml — executes the template's probe and report run blocks under PATH stubs
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEMPLATE="$DIR/templates/site-health.yml"
PROBE_STEP="probe live site"
REPORT_STEP="report the probe result"

fails=0
cases=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

extract_run() {
    awk -v name="$1" '
        function indent(s) { match(s, /^ */); return RLENGTH }
        !step { t = $0; sub(/^[[:space:]]+/, "", t); if (t == "- name: " name) { step = 1; sindent = indent($0) }; next }
        step && !run {
            if ($0 ~ /^[[:space:]]*- name: / && indent($0) <= sindent) exit
            if ($0 ~ /^[[:space:]]*run: \|[[:space:]]*$/) run = 1
            next
        }
        run {
            if ($0 ~ /^[[:space:]]*$/) { print ""; next }
            if (!bindent) bindent = indent($0)
            if (indent($0) < bindent) exit
            print substr($0, bindent + 1)
        }
    ' "$TEMPLATE"
}

for step in "$PROBE_STEP" "$REPORT_STEP"; do
    body="$(extract_run "$step")"
    if [[ -z "${body//[[:space:]]/}" ]]; then
        echo "site-health-probe.test: no run block for a step named '$step' in $TEMPLATE"
        exit 1
    fi
    if [[ "$step" == "$PROBE_STEP" ]]; then printf '%s\n' "$body" > "$tmp/probe.sh"; else printf '%s\n' "$body" > "$tmp/report.sh"; fi
done

STUBS="$tmp/stubs"
mkdir -p "$STUBS"
cat > "$STUBS/curl" <<'STUB'
#!/usr/bin/env bash
printf 'curl %s\n' "$*" >> "$STUB_LOGS/curl.log"
url="" fmt="" verbose=0
while [ $# -gt 0 ]; do
    case "$1" in
        -w) fmt="$2"; shift ;;
        -o|-m) shift ;;
        -sSv) verbose=1 ;;
        http*) url="$1" ;;
    esac
    shift
done
if [ "$verbose" = 1 ]; then
    echo "* expire date: Jan  1 00:00:00 2100 GMT" >&2
    exit 0
fi
case "$url" in
    "https://$STUB_APEX/")
        if [ "$fmt" = '%{http_code}' ]; then
            n=$(( $(cat "$STUB_LOGS/apex.count" 2>/dev/null || echo 0) + 1 ))
            echo "$n" > "$STUB_LOGS/apex.count"
            if [ "$STUB_APEX_FAILS" = all ] || [ "$n" -le "$STUB_APEX_FAILS" ]; then printf 503; else printf 200; fi
        fi ;;
    "https://www.$STUB_APEX/") printf '301 https://%s/' "$STUB_APEX" ;;
    "http://$STUB_APEX/") printf '301 https://%s/' "$STUB_APEX" ;;
    "https://$ALT_DOMAIN/install.html") printf '301 https://%s/install.html' "$STUB_APEX" ;;
    "https://$STUB_APEX/"*) printf 200 ;;
    *) printf 000 ;;
esac
STUB
cat > "$STUBS/gh" <<'STUB'
#!/usr/bin/env bash
printf 'gh %s\n' "$*" >> "$STUB_LOGS/gh.log"
case "$*" in
    "api --paginate repos/{owner}/{repo}/tags "*)
        if [ "$STUB_TAGS_FAIL" = 1 ]; then echo "HTTP 502: stubbed outage" >&2; exit 1; fi
        echo "$STUB_TAG" ;;
    "api repos/{owner}/{repo}/releases/tags/$STUB_TAG "*)
        echo "Release notes: https://$STUB_APEX/posts/$STUB_SLUG" ;;
    "issue list "*) [ -z "$STUB_OPEN_ISSUE" ] || echo "$STUB_OPEN_ISSUE" ;;
    "label create "*|"issue create "*|"issue comment "*|"issue close "*) ;;
    *) echo "gh stub: unexpected call: $*" >&2; exit 1 ;;
esac
STUB
cat > "$STUBS/sleep" <<'STUB'
#!/usr/bin/env bash
printf 'sleep %s\n' "$*" >> "$STUB_LOGS/sleep.log"
STUB
cat > "$STUBS/date" <<'STUB'
#!/usr/bin/env bash
case "$*" in
    "-d "*" +%s") echo 4102444800 ;;
    "+%s") echo 1767225600 ;;
    "-u +"*) echo "2026-01-01 00:00Z" ;;
    *) echo "date stub: unexpected call: $*" >&2; exit 1 ;;
esac
STUB
chmod +x "$STUBS"/*

export STUB_APEX=site.example.test STUB_TAG=v1.0.0 STUB_SLUG=v1-0-0
export ALT_DOMAIN=alt.example.test RELEASE_NOTE_GLOB='docs/posts/*.md' RELEASE_NOTE_TAG_KEY=release
export RELEASE_NOTE_URL_PATH='/posts/{slug}' HIGHLIGHT_CSS_PATH='' HIGHLIGHT_TOKENS_FILE='' GH_TOKEN=stub-token
export RUN_URL=https://example.test/run/1

new_case() {
    cases=$((cases + 1))
    case_dir="$tmp/case$cases"
    mkdir -p "$case_dir/docs/posts" "$case_dir/logs"
    echo "$STUB_APEX" > "$case_dir/docs/CNAME"
    printf -- '---\nrelease: %s\n---\n\nThe note.\n' "$STUB_TAG" > "$case_dir/docs/posts/$STUB_SLUG.md"
    : > "$case_dir/logs/curl.log"; : > "$case_dir/logs/gh.log"; : > "$case_dir/logs/sleep.log"
    : > "$case_dir/output"
}

run_probe() {
    local apex_fails="$1" tags_fail="$2" force="$3" attempts="$4" pause="$5"
    new_case
    out="$( cd "$case_dir" && PATH="$STUBS:$PATH" STUB_LOGS="$case_dir/logs" STUB_APEX_FAILS="$apex_fails" \
        STUB_TAGS_FAIL="$tags_fail" STUB_OPEN_ISSUE="" FORCE_FAIL="$force" GITHUB_OUTPUT="$case_dir/output" \
        RESAMPLE_ATTEMPTS="$attempts" RESAMPLE_PAUSE="$pause" bash "$tmp/probe.sh" 2>&1 )"; rc=$?
    status="$(sed -n 's/^status=//p' "$case_dir/output")"
    report="$(awk '/^report<<PROBE_EOF$/ { r = 1; next } /^PROBE_EOF$/ { r = 0 } r' "$case_dir/output")"
    n_sleep="$(grep -c . "$case_dir/logs/sleep.log")"
}

run_report() {
    local tracking="$1" status_in="$2" open_issue="$3"
    new_case
    out="$( cd "$case_dir" && PATH="$STUBS:$PATH" STUB_LOGS="$case_dir/logs" STUB_OPEN_ISSUE="$open_issue" \
        STUB_TAGS_FAIL=0 ISSUE_TRACKING="$tracking" STATUS="$status_in" \
        REPORT="apex https did not answer 200 (got '503')" bash "$tmp/report.sh" 2>&1 )"; rc=$?
    gh_log="$(cat "$case_dir/logs/gh.log")"
}

verdict() {
    local label="$1"; shift
    local reason
    for reason in "$@"; do
        if [[ -n "$reason" ]]; then
            echo "  FAIL: $label: $reason"
            echo "$out" | sed 's/^/      | /'
            fails=$((fails + 1))
            return
        fi
    done
    echo "  ok: $label"
}

want() { [[ "$1" == "$2" ]] || echo "$3: expected '$2', got '$1'"; }
has() { grep -qF -- "$2" <<<"$1" || echo "$3: lacks '$2'"; }

run_probe 0 0 false 3 30
verdict "healthy set" \
    "$(want "$status" pass status)" "$(has "$out" "settled on attempt 1 of 3" log)" "$(want "$n_sleep" 0 "sleep calls")"

run_probe 1 0 false 3 30
verdict "one transient failure" \
    "$(want "$status" pass status)" "$(has "$out" "settled on attempt 2 of 3" log)" \
    "$(want "$n_sleep" 1 "sleep calls")" "$(want "$(cat "$case_dir/logs/sleep.log")" "sleep 30" "sleep operand")"

run_probe 2 0 false 3 30
verdict "two transient failures" \
    "$(want "$status" pass status)" "$(has "$out" "settled on attempt 3 of 3" log)" "$(want "$n_sleep" 2 "sleep calls")"

run_probe all 0 false 3 30
verdict "a persistent failure" \
    "$(want "$status" fail status)" "$(want "$(grep -c "apex https did not answer 200" <<<"$report")" 1 "apex findings in the report")" \
    "$(has "$out" "settled on attempt 3 of 3" log)" "$(want "$n_sleep" 2 "sleep calls")"

run_probe 0 0 true 3 30
verdict "force_fail on a healthy set" \
    "$(want "$status" fail status)" "$(want "$report" "synthetic failure (force_fail dispatch input)" report)" \
    "$(want "$n_sleep" 0 "sleep calls")"

run_probe all 0 false 1 30
verdict "resample off under a persistent failure" \
    "$(want "$status" fail status)" "$(has "$out" "settled on attempt 1 of 1" log)" "$(want "$n_sleep" 0 "sleep calls")"

for bad in "0 30 RESAMPLE_ATTEMPTS got '0'" "x 30 RESAMPLE_ATTEMPTS got 'x'" "3 -1 RESAMPLE_PAUSE got '-1'"; do
    read -r attempts pause knob _ value <<<"$bad"
    run_probe 0 0 false "$attempts" "$pause"
    verdict "a misconfigured bound, $knob=${value//\'/}" \
        "$(want "$status" fail status)" "$(has "$report" "$knob" report)" "$(has "$report" "(got $value)" report)" \
        "$(want "$(grep -c . <<<"$report")" 1 "report lines")" "$(want "$(grep -c . "$case_dir/logs/curl.log")" 0 "curl calls")"
done

run_probe 0 1 false 3 30
verdict "the tag list fails" \
    "$(want "$status" fail status)" "$(has "$report" "could not read the tag list (gh exit 1): HTTP 502: stubbed outage" report)"

run_report off fail ""
verdict "issue path off, a failure" \
    "$(want "$rc" 1 exit)" "$(has "$out" "apex https did not answer 200" log)" "$(want "$gh_log" "" "gh log")"

run_report off pass ""
verdict "issue path off, a pass" "$(want "$rc" 0 exit)" "$(want "$gh_log" "" "gh log")"

run_report on fail ""
verdict "issue path on, a failure, no open issue" \
    "$(want "$rc" 1 exit)" "$(has "$gh_log" "gh label create site-health" "gh log")" \
    "$(has "$gh_log" "gh issue list --label site-health" "gh log")" "$(has "$gh_log" "gh issue create --label site-health" "gh log")"

run_report on pass 7
verdict "issue path on, a pass, an open issue" "$(want "$rc" 0 exit)" "$(has "$gh_log" "gh issue close 7" "gh log")"

run_report yes fail ""
verdict "a misconfigured ISSUE_TRACKING" \
    "$(want "$rc" 1 exit)" "$(has "$out" "ISSUE_TRACKING must be 'on' or 'off' (got 'yes')" log)" "$(want "$gh_log" "" "gh log")"

if [[ "$fails" -gt 0 ]]; then
    echo "site-health-probe.test: $fails of $cases case(s) failed"
    exit 1
fi
echo "site-health-probe.test: ok ($cases case(s) over the template's '$PROBE_STEP' and '$REPORT_STEP' steps)"
exit 0
