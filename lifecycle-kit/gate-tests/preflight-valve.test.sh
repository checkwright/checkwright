#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the one-shot pre-flight valve end-to-end through a sandboxed enter-stage: an armed line admits the entry past a refusing LIFECYCLE_KIT_ENTRY_PREFLIGHT command and is rewritten to used; an arming for another stage, another iteration, or one already used admits nothing; both malformed ledger shapes are exit 2 with nothing written; a configured-but-absent path is not-armed rather than an error and the refusal names it; and --simulate reports the would-be admission leaving the ledger byte-identical.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
ENTER="$DIR/bin/enter-stage.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=case dir  $2=ledger body (may be empty)  $3=set to 'noledger' to leave the path absent
    local sb="$SANDBOX/$1"
    mkdir -p "$sb/.workflow" "$sb/scratch"
    cat >"$sb/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

## Technical Debt

## Done
EOF
    printf '# contract: lifecycle-kit/SPEC.md §check-stage-evidence\n\n---\n\ndemo-iteration scope aaaaaaaa 2026-06-01 none\n' \
        >"$sb/.workflow/WORKFLOW-STATE.txt"
    cat >"$sb/refuse.sh" <<'STUB'
#!/usr/bin/env bash
echo "the manifest carries a non-clean suite verdict"
exit 1
STUB
    chmod +x "$sb/refuse.sh"
    cat >"$sb/config.sh" <<'CFG'
# shellcheck shell=bash
LIFECYCLE_KIT_ENTRY_PREFLIGHT=('build=./refuse.sh')
LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE=.workflow/preflight-valve.txt
CFG
    if [[ "${3:-}" != noledger ]]; then
        { printf '# contract: lifecycle-kit/SPEC.md §bin/enter-stage.sh\n'; printf '%s' "$2"; } \
            >"$sb/.workflow/preflight-valve.txt"
    fi
    printf '%s' "$sb"
}

run_enter() {  # $1=case dir, rest=argv
    ( cd "$1" && env GATE_SDK_TMP_DIR=scratch \
                     LIFECYCLE_KIT_CONFIG_FILE=config.sh \
                     LIFECYCLE_KIT_SESSION_ID=deadbeef04 \
                     bash "$ENTER" "${@:2}" 2>&1 )
}

cursor() { awk '/^---[[:space:]]*$/{f=1;next} f && NF {l=$2} END{print l}' "$1/.workflow/WORKFLOW-STATE.txt"; }

# --- armed and admitted: the entry proceeds, stamps, and the line flips to used ---
ad="$(seed admitted 'demo-iteration build armed installer_smoke is red on the init no-op and the fix is close-stage work
')"
out="$(run_enter "$ad" build)"; rc=$?
[[ "$rc" -eq 0 ]] || note admitted-rc "want exit 0 on an armed entry, got $rc -- $out"
grep -qF 'valve admitted this entry' <<<"$out" || note admitted-report "the admission printed no valve report: $out"
grep -qF 'the manifest carries a non-clean suite verdict' <<<"$out" \
    || note admitted-relay "the admission did not relay the pre-flight's own findings: $out"
grep -qF 'installer_smoke is red' <<<"$out" || note admitted-reason "the admission did not print the reason: $out"
grep -qF 'carried 0 used valve line(s) before this one' <<<"$out" \
    || note admitted-count "the admission did not print this iteration's prior used count: $out"
[[ "$(cursor "$ad")" == build ]] || note admitted-stamp "the admitted entry did not stamp"
grep -q '^demo-iteration build used installer_smoke is red' "$ad/.workflow/preflight-valve.txt" \
    || note admitted-consume "the admitted entry did not rewrite the line's state token to used"
grep -q ' armed ' "$ad/.workflow/preflight-valve.txt" && note admitted-residue "an armed line survived the admission"

# --- the second reach announces itself: a used line already present is counted ---
tw="$(seed twice 'demo-iteration build used the first reach
demo-iteration build armed the second reach
')"
out="$(run_enter "$tw" build)"; rc=$?
[[ "$rc" -eq 0 ]] || note twice-rc "want exit 0 on the second reach, got $rc -- $out"
grep -qF 'carried 1 used valve line(s) before this one' <<<"$out" \
    || note twice-count "the second reach did not announce the prior use: $out"

# --- arming twice does not admit twice: only the first matching armed line is consumed ---
dbl="$(seed doubled 'demo-iteration build armed the first arming
demo-iteration build armed the second arming
')"
run_enter "$dbl" build >/dev/null
[[ "$(grep -c ' armed ' "$dbl/.workflow/preflight-valve.txt")" -eq 1 ]] \
    || note doubled-consume "one entry consumed more or fewer than one armed line"

# --- armed for another stage: no admission, the refusal stands, nothing written ---
os="$(seed other-stage 'demo-iteration close armed aimed at the wrong stage
')"
cp "$os/.workflow/WORKFLOW-STATE.txt" "$os/s.before"
cp "$os/.workflow/preflight-valve.txt" "$os/v.before"
out="$(run_enter "$os" build)"; rc=$?
[[ "$rc" -eq 1 ]] || note other-stage-rc "an arming for another stage must not admit, got $rc -- $out"
cmp -s "$os/s.before" "$os/.workflow/WORKFLOW-STATE.txt" || note other-stage-state "a refused entry wrote the state file"
cmp -s "$os/v.before" "$os/.workflow/preflight-valve.txt" || note other-stage-ledger "a refused entry wrote the ledger"

# --- armed for another iteration: same verdict, even though truncation should make it unreachable ---
oi="$(seed other-iteration 'prior-iteration build armed left over from the iteration before
')"
cp "$oi/.workflow/preflight-valve.txt" "$oi/v.before"
out="$(run_enter "$oi" build)"; rc=$?
[[ "$rc" -eq 1 ]] || note other-iteration-rc "an arming from another iteration must not admit, got $rc -- $out"
cmp -s "$oi/v.before" "$oi/.workflow/preflight-valve.txt" || note other-iteration-ledger "a refused entry wrote the ledger"

# --- already used: a spent line admits nothing ---
us="$(seed used 'demo-iteration build used already spent on an earlier entry
')"
out="$(run_enter "$us" build)"; rc=$?
[[ "$rc" -eq 1 ]] || note used-rc "a used line must not admit a second entry, got $rc -- $out"

# --- fail-closed: a data line under four fields ---
sh="$(seed short 'demo-iteration build armed
')"
cp "$sh/.workflow/WORKFLOW-STATE.txt" "$sh/s.before"
out="$(run_enter "$sh" build)"; rc=$?
[[ "$rc" -eq 2 ]] || note short-rc "a reason-less line must be exit 2, got $rc -- $out"
grep -qF 'fewer than the four' <<<"$out" || note short-msg "the fail-closed refusal did not name the shape: $out"
cmp -s "$sh/s.before" "$sh/.workflow/WORKFLOW-STATE.txt" || note short-write "a fail-closed refusal wrote the state file"

# --- fail-closed: a state token that is neither armed nor used ---
bt="$(seed bad-token 'demo-iteration build pending a token no reader knows
')"
out="$(run_enter "$bt" build)"; rc=$?
[[ "$rc" -eq 2 ]] || note token-rc "an unknown state token must be exit 2, got $rc -- $out"
grep -qF 'neither armed nor used' <<<"$out" || note token-msg "the fail-closed refusal did not name the token: $out"

# --- a configured path that does not exist is NOT ARMED, not an error, and the refusal names it ---
ab="$(seed absent '' noledger)"
out="$(run_enter "$ab" build)"; rc=$?
[[ "$rc" -eq 1 ]] || note absent-rc "an absent ledger must refuse rather than error, got $rc -- $out"
grep -qF '.workflow/preflight-valve.txt' <<<"$out" \
    || note absent-names "the refusal did not name the configured valve path: $out"
grep -qF 'does not exist' <<<"$out" || note absent-why "the refusal did not say the path is absent: $out"

# --- an unarmed ledger refuses and the refusal still names the path and the sanctioned cause ---
ua="$(seed unarmed '')"
out="$(run_enter "$ua" build)"; rc=$?
[[ "$rc" -eq 1 ]] || note unarmed-rc "a header-only ledger must refuse, got $rc -- $out"
grep -qF "carries no 'armed' line for 'demo-iteration build'" <<<"$out" \
    || note unarmed-why "the refusal did not say the ledger carries no matching arming: $out"
grep -qF 'only a later stage can clear' <<<"$out" \
    || note unarmed-cause "the refusal's valve help did not name its single sanctioned cause: $out"

# --- --simulate reports the would-be admission and leaves the ledger byte-identical ---
sm="$(seed simulate 'demo-iteration build armed the reason a simulate must relay
')"
cp "$sm/.workflow/preflight-valve.txt" "$sm/v.before"
cp "$sm/.workflow/WORKFLOW-STATE.txt" "$sm/s.before"
out="$(run_enter "$sm" --simulate build)"; rc=$?
[[ "$rc" -eq 0 ]] || note sim-rc "--simulate of an armed entry should exit 0, got $rc -- $out"
grep -qF 'would be consumed' <<<"$out" || note sim-report "--simulate did not name the line it would consume: $out"
grep -qF 'the reason a simulate must relay' <<<"$out" || note sim-reason "--simulate did not relay the reason: $out"
grep -qF 'would proceed' <<<"$out" || note sim-verdict "--simulate did not report would-proceed: $out"
if grep -qv '^enter-stage (simulate): ' <<<"$out"; then
    note sim-prefix "--simulate emitted an unprefixed line: $out"
fi
cmp -s "$sm/v.before" "$sm/.workflow/preflight-valve.txt" || note sim-ledger "--simulate wrote the ledger"
cmp -s "$sm/s.before" "$sm/.workflow/WORKFLOW-STATE.txt" || note sim-state "--simulate wrote the state file"

[[ "$fails" -eq 0 ]] || { echo "preflight-valve.test: $fails assertion(s) failed"; exit 1; }
echo "preflight-valve.test: clean (armed admits once and flips to used with the prior-use count printed; another stage, another iteration and a spent line admit nothing; both malformed shapes exit 2 writing nothing; an absent path refuses and is named; --simulate relays the would-be admission and writes nothing)"
exit 0
