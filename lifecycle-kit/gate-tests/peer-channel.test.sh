#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The peer request channel — the four arms end-to-end over a scratch channel outside any repository: a request files from a non-git directory and lists as received, every legal move lands and an illegal one is refused leaving the reply byte-identical, both bulk forms move their matches and a bulk form matching none prints nothing, the ready notice prints a later release and stays silent on an earlier one, a malformed body, a tab in the need line or a refused reason, a multibyte filing time, a relative channel, a missing one and one inside a work tree are refused at exit 2, an unset channel reads as nothing, moves nothing in bulk and refuses a filing or a targeted move, and with no repository a stray scripts/ knob file configures neither peer arm
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

CH="$SANDBOX/channel"
FREE="$SANDBOX/free"
mkdir -p "$CH" "$FREE"

arm() {  # $1=channel value  $2..=the arm flag and its argv; stdin is the caller's; runs in the non-git dir
    local ch="$1"; shift
    ( cd "$FREE" && gate_env LIFECYCLE_KIT_PEER_CHANNEL="$ch" && gate_arm_run "$@" 2>&1 )
}

BODY=$'## need\nthe gate refuses a path\n\n## repro\n```\n## a heading in a fence\n```\n\n## wanted\nadmit it\n\n## evidence\nthe log\n'

# --- a request files from a non-git directory, and lists as received ---
id="$(printf '%s' "$BODY" | arm "$CH" --emit-peer-file peer-a)"; rc=$?
[[ "$rc" -eq 0 && "$id" =~ ^r[0-9]{8}-[0-9a-f]{8}$ ]] || note file "want exit 0 and an id, got $rc -- $id"
[[ "$(head -n 1 "$CH/$id.md" 2>/dev/null)" == "# peer-request v1" ]] || note file-head "the request has no header line"
[[ -z "$(find "$CH" -name '*.tmp')" ]] || note file-tmp "the temporary file was left behind"
row="$(arm "$CH" --emit-peer-list)"; rc=$?
day="${id:1:4}-${id:5:2}-${id:7:2}"
[[ "$rc" -eq 0 && "$row" == "$id"$'\tpeer-a\t'"$day"$'\treceived\tthe gate refuses a path' ]] \
    || note list-received "the row is not the received request (exit $rc): $row"

# --- an illegal move is refused, naming the request, and the reply is untouched ---
out="$(arm "$CH" --emit-peer-reply "$id" landed)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"$id"*"received"*"landed"* ]] || note illegal "want exit 2 naming the request, its state and the move, got $rc -- $out"
[[ -e "$CH/$id.reply" ]] && note illegal-write "a refused move wrote a reply"

# --- the legal walk: filed, re-filed, landed by the bulk form, shipped by the bulk form ---
out="$(arm "$CH" --emit-peer-reply "$id" filed first-slug)"; rc=$?
[[ "$rc" -eq 0 && "$out" == "$id filed first-slug" ]] || note filed "got $rc -- $out"
out="$(arm "$CH" --emit-peer-reply "$id" filed second-slug)"; rc=$?
[[ "$rc" -eq 0 && "$out" == "$id filed second-slug" ]] || note refiled "got $rc -- $out"
out="$(arm "$CH" --emit-peer-reply --landed first-slug)"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note bulk-none "a bulk form matching none printed or failed ($rc): $out"
out="$(arm "$CH" --emit-peer-reply --landed second-slug)"; rc=$?
[[ "$rc" -eq 0 && "$out" == "$id landed" ]] || note bulk-landed "got $rc -- $out"
out="$(arm "$CH" --emit-peer-reply --shipped v999.0.0)"; rc=$?
[[ "$rc" -eq 0 && "$out" == "$id shipped v999.0.0" ]] || note bulk-shipped "got $rc -- $out"
before="$(cat "$CH/$id.reply")"
out="$(arm "$CH" --emit-peer-reply "$id" refused "too late")"; rc=$?
[[ "$rc" -eq 2 ]] || note final "a move off shipped was admitted ($rc): $out"
[[ "$(cat "$CH/$id.reply")" == "$before" ]] || note final-write "a refused move changed the reply"
[[ "$(head -n 1 "$CH/$id.reply")" == "# peer-reply v1" && "$(wc -l <"$CH/$id.reply" | tr -d ' ')" -eq 5 ]] \
    || note reply-shape "the reply is not a header and four moves: $before"
row="$(arm "$CH" --emit-peer-list)"
[[ "$row" == *$'\tshipped v999.0.0\t'* ]] || note list-shipped "the row does not read shipped: $row"

# --- refused is final, from received ---
id2="$(printf '%s' "${BODY/the gate refuses a path/a second ask}" | arm "$CH" --emit-peer-file peer-b)"
out="$(arm "$CH" --emit-peer-reply "$id2" refused "out of scope")"; rc=$?
[[ "$rc" -eq 0 && "$out" == "$id2 refused out of scope" ]] || note refused "got $rc -- $out"
out="$(arm "$CH" --emit-peer-reply "$id2" filed late)"; rc=$?
[[ "$rc" -eq 2 ]] || note refused-final "a move off refused was admitted ($rc): $out"

# --- the ready notice: a later release prints, the peer's own requests only ---
out="$(arm "$CH" --emit-peer-ready peer-a)"; rc=$?
[[ "$rc" -eq 0 && "$out" == "peer request $id shipped in v999.0.0, not in this binary ("*"): the gate refuses a path" ]] \
    || note ready "got $rc -- $out"
out="$(arm "$CH" --emit-peer-ready peer-b)"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note ready-other "another peer's request printed ($rc): $out"

# --- an earlier release is silent unless this binary carries no stamp it can compare ---
id3="$(printf '%s' "${BODY/the gate refuses a path/an old ask}" | arm "$CH" --emit-peer-file peer-c)"
arm "$CH" --emit-peer-reply "$id3" filed old >/dev/null
arm "$CH" --emit-peer-reply --landed old >/dev/null
printf '%s\n' "$(date -u +%Y-%m-%d) shipped v0.0.1" >>"$CH/$id3.reply"
out="$(arm "$CH" --emit-peer-ready peer-c)"; rc=$?
[[ "$rc" -eq 0 && ( -z "$out" || "$out" == *"(unknown)"* ) ]] || note ready-earlier "an adopted release printed ($rc): $out"

# --- an edited request is refused by every reader, naming the file ---
cp "$CH/$id.md" "$SANDBOX/keep.md"
printf 'more\n' >>"$CH/$id.md"
out="$(arm "$CH" --emit-peer-list)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"$id.md"*"edited after filing"* ]] || note edited "got $rc -- $out"
cp "$SANDBOX/keep.md" "$CH/$id.md"

# --- a malformed body is refused naming the section, and nothing is filed ---
n="$(find "$CH" -name '*.md' | wc -l)"
out="$(printf '## need\nx\n\n## wanted\ny\n' | arm "$CH" --emit-peer-file peer-a)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"## wanted"*"## repro"* ]] || note malformed "got $rc -- $out"
[[ "$(find "$CH" -name '*.md' | wc -l)" -eq "$n" ]] || note malformed-write "a malformed body was filed"

# --- a tab is refused where peer-list prints it: the need line and a refused reason ---
TAB=$'\t'
out="$(printf '%s' "${BODY/the gate refuses a path/a tabbed${TAB}ask}" | arm "$CH" --emit-peer-file peer-a)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"holds a tab"* ]] || note tab-need "got $rc -- $out"
[[ "$(find "$CH" -name '*.md' | wc -l)" -eq "$n" ]] || note tab-need-write "a tabbed need line was filed"
out="$(arm "$CH" --emit-peer-reply "$id3" refused $'no\tthanks')"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"holds a tab"* ]] || note tab-reason "got $rc -- $out"

# --- a multibyte character across the filing time's byte 10 is malformed at exit 2, never a panic ---
bad_id=r20261124-00000000
printf '# peer-request v1\nid: %s\nfrom: peer-a\nfiled: 2026-11-2\xc3\xa910:20:30Z\n\n%s' "$bad_id" "$BODY" >"$CH/$bad_id.md"
out="$(arm "$CH" --emit-peer-list)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"$bad_id.md"*"not a UTC time"* ]] || note multibyte-filed "got $rc -- $out"
rm -f "$CH/$bad_id.md"

# --- a bad channel value is refused at exit 2 naming the knob ---
mkdir -p "$SANDBOX/repo" && git -C "$SANDBOX/repo" init -q
: >"$SANDBOX/a-file"
for bad in relative/channel "$SANDBOX/missing" "$SANDBOX/a-file" "$SANDBOX/repo"; do
    out="$(arm "$bad" --emit-peer-list)"; rc=$?
    [[ "$rc" -eq 2 && "$out" == *"LIFECYCLE_KIT_PEER_CHANNEL is '$bad'"* ]] || note "bad-channel $bad" "got $rc -- $out"
done

# --- unset: the readers and the bulk moves print nothing, the targeted writers refuse ---
out="$(arm "" --emit-peer-list)"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note unset-list "got $rc -- $out"
out="$(arm "" --emit-peer-ready peer-a)"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note unset-ready "got $rc -- $out"
out="$(arm "" --emit-peer-reply --landed a-slug)"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note unset-bulk-landed "got $rc -- $out"
out="$(arm "" --emit-peer-reply --shipped v1.0.0)"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note unset-bulk-shipped "got $rc -- $out"
out="$(printf '%s' "$BODY" | arm "" --emit-peer-file peer-a)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"is unset"* ]] || note unset-file "got $rc -- $out"
for move in "filed a-slug" "refused no" "landed"; do
    read -ra mv <<<"$move"
    out="$(arm "" --emit-peer-reply "$id" "${mv[@]}")"; rc=$?
    [[ "$rc" -eq 2 && "$out" == *"is unset"* ]] || note "unset-reply $move" "got $rc -- $out"
done

# --- with no repository a stray scripts/ knob file configures neither peer arm ---
mkdir -p "$FREE/scripts"
printf 'LIFECYCLE_KIT_PEER_CHANNEL = %s\n' "$CH" >"$FREE/scripts/lifecycle-config.knobs"
stray() { ( cd "$FREE" && unset LIFECYCLE_KIT_KNOB_FILE LIFECYCLE_KIT_LOCAL_KNOB_FILE && gate_arm_run "$@" 2>&1 ); }
n="$(find "$CH" -name '*.md' | wc -l)"
out="$(printf '%s' "$BODY" | stray --emit-peer-file peer-a)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"is unset"* ]] || note stray-file "the stray knob file configured peer-file ($rc): $out"
[[ "$(find "$CH" -name '*.md' | wc -l)" -eq "$n" ]] || note stray-write "a request was filed through the stray knob file"
out="$(stray --emit-peer-ready peer-a)"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note stray-ready "the stray knob file configured peer-ready ($rc): $out"
out="$( ( cd "$FREE" && gate_env LIFECYCLE_KIT_KNOB_FILE=scripts/lifecycle-config.knobs && gate_arm_run --emit-peer-ready peer-a 2>&1 ) )"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"LIFECYCLE_KIT_KNOB_FILE is 'scripts/lifecycle-config.knobs'"* ]] \
    || note relative-locator "a relative locator was not refused by name ($rc): $out"
out="$( ( cd "$FREE" && gate_env LIFECYCLE_KIT_KNOB_FILE="$FREE/scripts/lifecycle-config.knobs" && gate_arm_run --emit-peer-ready peer-a 2>&1 ) )"; rc=$?
[[ "$rc" -eq 0 && "$out" == *"peer request $id shipped"* ]] || note absolute-locator "an absolute locator was not honored ($rc): $out"

if [[ "$fails" -eq 0 ]]; then echo "peer-channel: all cases passed"; exit 0; fi
echo "peer-channel: $fails case(s) failed"; exit 1
