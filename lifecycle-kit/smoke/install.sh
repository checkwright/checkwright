#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — lifecycle-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored lifecycle-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# lifecycle-kit
check-stage-evidence
check-stage-entry
EOF

if [[ ! -f TASK-QUEUE.md ]]; then
    cat > TASK-QUEUE.md <<'EOF'
# TASK-QUEUE.md — smoke consumer work queue

## Iteration: —

---

## New Features

## Technical Debt

## Deferred

## Done
EOF
fi

mkdir -p .workflow
cat > .workflow/WORKFLOW-STATE.txt <<EOF
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

— scope smoke001 $(date +%F)
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — exercise enter-stage end-to-end under .tmp (advisory tool, no fixture pair)
es="$PWD/.tmp/enter-stage-smoke"
rm -rf "$es"; mkdir -p "$es/.workflow" "$es/sessions" "$es/tmp"
: > "$es/sessions/aabbccdd-0000-0000-0000-000000000000.jsonl"
cat > "$es/TASK-QUEUE.md" <<'EOF'
# smoke queue
## Iteration: prev-iter

---

## New Features
## Technical Debt
## Deferred
## Done
EOF
cat > "$es/.workflow/WORKFLOW-STATE.txt" <<'EOF'
# contract

---

prev-iter close ffffffff 2020-01-01
EOF

es_run() {
    env -u CLAUDE_CODE_SESSION_ID -u CLAUDE_CODE_CHILD_SESSION -u LIFECYCLE_KIT_SESSION_ID \
    LIFECYCLE_KIT_QUEUE_FILE="$es/TASK-QUEUE.md" \
    LIFECYCLE_KIT_STATE_FILE="$es/.workflow/WORKFLOW-STATE.txt" \
    LIFECYCLE_KIT_SESSIONS_DIR="$es/sessions" \
    GATE_SDK_TMP_DIR="$es/tmp" \
    bash "$SMOKE_KIT_ROOT/bin/enter-stage.sh" "$@"
}
esq="$es/TASK-QUEUE.md"; ess="$es/.workflow/WORKFLOW-STATE.txt"; d="$(date +%F)"

es_run scope >/dev/null
grep -q "^## Iteration: —\$" "$esq" || { echo "smoke(enter-stage): scope header wrong" >&2; exit 1; }
grep -q "^— scope aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): scope stamp missing" >&2; exit 1; }
if grep -q 'prev-iter' "$ess"; then echo "smoke(enter-stage): state not truncated on reset" >&2; exit 1; fi

# spec: lifecycle-kit/SPEC.md §The stamp protocol — a non-boundary entry writes the evidence file ONLY: the cursor advances by the appended stamp and the queue is byte-identical afterwards
cp "$esq" "$es/q.before"
es_run align >/dev/null
cmp -s "$es/q.before" "$esq" || { echo "smoke(enter-stage): a non-boundary entry wrote the queue" >&2; exit 1; }
grep -q "^— align aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): align stamp missing" >&2; exit 1; }
grep -q "^— scope aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): scope stamp lost on append" >&2; exit 1; }
[[ "$(awk '/^---[[:space:]]*$/{f=1;next} f && NF {l=$2} END{print l}' "$ess")" == align ]] \
    || { echo "smoke(enter-stage): cursor did not advance to align" >&2; exit 1; }

es_run align >/dev/null
if [[ "$(grep -c "^— align aabbccdd " "$ess")" -ne 1 ]]; then
    echo "smoke(enter-stage): idempotent re-entry duplicated the align stamp" >&2; exit 1
fi

grep -v '^## Iteration:' "$esq" > "$es/q.headerless"; cp "$es/q.headerless" "$esq"
cp "$esq" "$es/q.before"; cp "$ess" "$es/s.before"
if es_run build >/dev/null 2>&1; then echo "smoke(enter-stage): should refuse a headerless queue" >&2; exit 1; fi
cmp -s "$es/s.before" "$ess" || { echo "smoke(enter-stage): wrote state on a refusal" >&2; exit 1; }
cmp -s "$es/q.before" "$esq" || { echo "smoke(enter-stage): wrote queue on a refusal" >&2; exit 1; }

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_ENTRY_PREFLIGHT: a red entry command refuses the entry (no writes), a green one lets it through
cat > "$esq" <<'EOF'
# smoke queue
## Iteration: pf-iter

---

## New Features
## Technical Debt
## Deferred
## Done
EOF
cat > "$ess" <<EOF
# contract

---

pf-iter build aabbccdd $d
EOF
cat > "$es/preflight-stub.sh" <<'STUB'
#!/usr/bin/env bash
[[ -f "$(dirname "$0")/preflight-ok" ]]
STUB
chmod +x "$es/preflight-stub.sh"
cat > "$es/stages.sh" <<STAGES
# shellcheck shell=bash
LIFECYCLE_KIT_ENTRY_PREFLIGHT=('validate=$es/preflight-stub.sh')
STAGES

es_pf_run() {
    env -u CLAUDE_CODE_SESSION_ID -u CLAUDE_CODE_CHILD_SESSION -u LIFECYCLE_KIT_SESSION_ID \
    LIFECYCLE_KIT_QUEUE_FILE="$esq" \
    LIFECYCLE_KIT_STATE_FILE="$ess" \
    LIFECYCLE_KIT_SESSIONS_DIR="$es/sessions" \
    GATE_SDK_TMP_DIR="$es/tmp" \
    LIFECYCLE_KIT_CONFIG_FILE="$es/stages.sh" \
    bash "$SMOKE_KIT_ROOT/bin/enter-stage.sh" "$@"
}

cp "$esq" "$es/q.before"; cp "$ess" "$es/s.before"
if es_pf_run validate >/dev/null 2>&1; then echo "smoke(enter-stage): red preflight should refuse the entry" >&2; exit 1; fi
cmp -s "$es/s.before" "$ess" || { echo "smoke(enter-stage): wrote state on a preflight refusal" >&2; exit 1; }
cmp -s "$es/q.before" "$esq" || { echo "smoke(enter-stage): wrote queue on a preflight refusal" >&2; exit 1; }

touch "$es/preflight-ok"
es_pf_run validate >/dev/null || { echo "smoke(enter-stage): green preflight should let the entry through" >&2; exit 1; }
[[ "$(awk '/^---[[:space:]]*$/{f=1;next} f && NF {l=$2} END{print l}' "$ess")" == validate ]] \
    || { echo "smoke(enter-stage): green preflight did not advance the cursor to validate" >&2; exit 1; }
grep -q "^pf-iter validate aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): green preflight did not stamp" >&2; exit 1; }

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --simulate: would-no-op, would-pass, would-refuse; read-only and prefix-marked in all three
cp "$esq" "$es/q.before"; cp "$ess" "$es/s.before"
out="$(es_pf_run --simulate validate 2>&1)" || { echo "smoke(enter-stage): --simulate of a stamped stage should exit 0" >&2; exit 1; }
grep -q "idempotent no-op" <<<"$out" || { echo "smoke(enter-stage): --simulate did not report the would-be no-op: $out" >&2; exit 1; }
out="$(es_pf_run --simulate close 2>&1)" || { echo "smoke(enter-stage): --simulate of a clean entry should exit 0" >&2; exit 1; }
grep -q "would proceed" <<<"$out" || { echo "smoke(enter-stage): --simulate did not report would-proceed: $out" >&2; exit 1; }
if out="$(es_pf_run --simulate build 2>&1)"; then
    echo "smoke(enter-stage): --simulate of an unstamped predecessor should exit 1" >&2; exit 1
fi
grep -q "would refuse" <<<"$out" || { echo "smoke(enter-stage): --simulate did not relay the refusal: $out" >&2; exit 1; }
if grep -qv "^enter-stage (simulate): " <<<"$out"; then
    echo "smoke(enter-stage): --simulate emitted an unprefixed line: $out" >&2; exit 1
fi
cmp -s "$es/s.before" "$ess" || { echo "smoke(enter-stage): --simulate wrote state" >&2; exit 1; }
cmp -s "$es/q.before" "$esq" || { echo "smoke(enter-stage): --simulate wrote queue" >&2; exit 1; }

# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — exercise the injector + check-lifecycle-registration end-to-end under .tmp (advisory tool, no fixture pair)
il="$es/agent"; mkdir -p "$il"
cat > "$il/CLAUDE.md" <<'EOF'
# Scratch agent file

Resident context the consumer keeps.
EOF
il_run() { LIFECYCLE_KIT_AGENT_FILE="$il/CLAUDE.md" bash "$SMOKE_KIT_ROOT/bin/install-lifecycle.sh" "$@"; }
il_gate() { LIFECYCLE_KIT_AGENT_FILE="$il/CLAUDE.md" bash "$SMOKE_KIT_ROOT/checks/check-lifecycle-registration.sh" "$@"; }

il_run >/dev/null
grep -q "<!-- lifecycle-kit:begin -->" "$il/CLAUDE.md" || { echo "smoke(install-lifecycle): block not injected" >&2; exit 1; }
il_gate >/dev/null || { echo "smoke(install-lifecycle): a freshly installed block should pass the parity gate" >&2; exit 1; }

cp "$il/CLAUDE.md" "$il/before"
il_run >/dev/null
cmp -s "$il/before" "$il/CLAUDE.md" || { echo "smoke(install-lifecycle): re-run was not idempotent" >&2; exit 1; }

sed -i 's#`/close`##' "$il/CLAUDE.md"
if il_gate >/dev/null 2>&1; then echo "smoke(install-lifecycle): a staled block should redden the parity gate" >&2; exit 1; fi

# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — exercise the merge-attribute + driver-config steps and check-merge-attrs end-to-end in a scratch git repo (advisory tool, no fixture pair)
ma="$es/merge-attrs"; mkdir -p "$ma"
git -C "$ma" init -q
printf '# Scratch agent file\n' > "$ma/CLAUDE.md"
( cd "$ma" && bash "$SMOKE_KIT_ROOT/bin/install-lifecycle.sh" >/dev/null )
grep -q "^# lifecycle-kit:merge:begin\$" "$ma/.gitattributes" || { echo "smoke(install-lifecycle): merge-attribute block not injected into .gitattributes" >&2; exit 1; }
grep -q "^\.workflow/WORKFLOW-STATE.txt merge=iteration-scoped\$" "$ma/.gitattributes" || { echo "smoke(install-lifecycle): state-file merge attribute missing" >&2; exit 1; }
[[ "$(git -C "$ma" config --get merge.iteration-scoped.driver)" == "true" ]] || { echo "smoke(install-lifecycle): keep-ours driver not registered in git config" >&2; exit 1; }
( cd "$ma" && bash "$SMOKE_KIT_ROOT/checks/check-merge-attrs.sh" >/dev/null ) || { echo "smoke(install-lifecycle): a freshly installed .gitattributes should pass the parity gate" >&2; exit 1; }

cp "$ma/.gitattributes" "$ma/before"
( cd "$ma" && bash "$SMOKE_KIT_ROOT/bin/install-lifecycle.sh" >/dev/null )
cmp -s "$ma/before" "$ma/.gitattributes" || { echo "smoke(install-lifecycle): merge-attribute re-run was not idempotent" >&2; exit 1; }

printf 'README.md merge=iteration-scoped\n' >> "$ma/.gitattributes"   # smuggled reverse-edge line
if ( cd "$ma" && bash "$SMOKE_KIT_ROOT/checks/check-merge-attrs.sh" >/dev/null 2>&1 ); then
    echo "smoke(install-lifecycle): a smuggled out-of-set merge attribute should redden the parity gate" >&2; exit 1
fi

# spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the derivation order: env-first, agent- strip, widened + child-narrowed subagents scan (advisory tool, no fixture pair)
SID="$SMOKE_KIT_ROOT/bin/session-id.sh"
sid="$es/sid"; mkdir -p "$sid"
sid_run() { env -u CLAUDE_CODE_SESSION_ID -u CLAUDE_CODE_CHILD_SESSION -u LIFECYCLE_KIT_SESSION_ID -u LIFECYCLE_KIT_SESSIONS_DIR "$@" bash "$SID"; }

o="$(sid_run LIFECYCLE_KIT_SESSION_ID=agent-deadbeefcafe0000)"           # source 1, agent- strip
[[ "$o" == "deadbeef" ]] || { echo "smoke(session-id): override did not strip agent- (got '$o')" >&2; exit 1; }
o="$(sid_run CLAUDE_CODE_SESSION_ID=abcdef0123456789)"                    # source 2 (CHILD unset)
[[ "$o" == "abcdef01" ]] || { echo "smoke(session-id): harness session id not used (got '$o')" >&2; exit 1; }

lead="11112222-3333-4444-5555-666677778888"                              # source 2 skipped when CHILD set → narrowed scan
mkdir -p "$sid/tree/$lead/subagents"
: > "$sid/tree/$lead/subagents/agent-aaaabbbbccccdddd.jsonl"
: > "$sid/tree/$lead.jsonl"
touch -d '2020-01-01T00:00:00' "$sid/tree/$lead/subagents/agent-aaaabbbbccccdddd.jsonl"
touch -d '2025-01-01T00:00:00' "$sid/tree/$lead.jsonl"                    # lead top-level newer, yet excluded
o="$(sid_run CLAUDE_CODE_CHILD_SESSION=1 CLAUDE_CODE_SESSION_ID=$lead LIFECYCLE_KIT_SESSIONS_DIR=$sid/tree)"
[[ "$o" == "aaaabbbb" ]] || { echo "smoke(session-id): dispatched child did not narrow to subagents (got '$o')" >&2; exit 1; }

spur="ccccdddd-1111-2222-3333-444455556666"                             # spurious flag: empty subagents/ + top-level transcript → fall back to env uuid
mkdir -p "$sid/spur/$spur/subagents"                                     # subagents/ dir exists but empty
: > "$sid/spur/$spur.jsonl"
o="$(sid_run CLAUDE_CODE_CHILD_SESSION=1 CLAUDE_CODE_SESSION_ID=$spur LIFECYCLE_KIT_SESSIONS_DIR=$sid/spur)"
[[ "$o" == "ccccdddd" ]] || { echo "smoke(session-id): spurious flag did not fall back to top-level uuid (got '$o')" >&2; exit 1; }

mkdir -p "$sid/tree2/somelead/subagents"                                 # source 3 widened glob, no env id vars
: > "$sid/tree2/somelead/subagents/agent-9999888877776666.jsonl"
o="$(sid_run LIFECYCLE_KIT_SESSIONS_DIR=$sid/tree2)"
[[ "$o" == "99998888" ]] || { echo "smoke(session-id): widened subagents glob did not resolve (got '$o')" >&2; exit 1; }

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_BOUNDARY_REQUIRE: at the iteration boundary each member must carry a line naming the closing iteration, else refuse (fail-closed on a missing file); a never-named (—) closing iteration skips the check
br="$es/boundary-require"; mkdir -p "$br/.workflow"
brq="$br/TASK-QUEUE.md"; brs="$br/.workflow/WORKFLOW-STATE.txt"; brd="$br/.workflow/release-disposition.txt"
write_br_fixture() {   # $1 = header (and stamp) iteration name
    cat > "$brq" <<EOF
# smoke queue
## Iteration: $1

---

## New Features
## Technical Debt
## Deferred
## Done
EOF
    cat > "$brs" <<EOF
# contract

---

$1 close aabbccdd $d
EOF
}
cat > "$br/stages.sh" <<STAGES
# shellcheck shell=bash
LIFECYCLE_KIT_BOUNDARY_REQUIRE=($brd)
STAGES
br_run() {
    env -u CLAUDE_CODE_SESSION_ID -u CLAUDE_CODE_CHILD_SESSION -u LIFECYCLE_KIT_SESSION_ID \
    LIFECYCLE_KIT_QUEUE_FILE="$brq" \
    LIFECYCLE_KIT_STATE_FILE="$brs" \
    LIFECYCLE_KIT_SESSIONS_DIR="$es/sessions" \
    GATE_SDK_TMP_DIR="$es/tmp" \
    LIFECYCLE_KIT_CONFIG_FILE="$br/stages.sh" \
    bash "$SMOKE_KIT_ROOT/bin/enter-stage.sh" "$@"
}

write_br_fixture closing-iter                                            # (a) member missing the closing-iteration line → refuse
printf '# contract\n' > "$brd"
cp "$brq" "$br/q.before"; cp "$brs" "$br/s.before"
if br_run scope >/dev/null 2>&1; then echo "smoke(enter-stage): boundary-require should refuse a member missing the closing-iteration line" >&2; exit 1; fi
cmp -s "$br/s.before" "$brs" || { echo "smoke(enter-stage): wrote state on a boundary-require refusal" >&2; exit 1; }
cmp -s "$br/q.before" "$brq" || { echo "smoke(enter-stage): wrote queue on a boundary-require refusal" >&2; exit 1; }

write_br_fixture closing-iter                                            # (b) member absent from disk → fail-closed refusal
rm -f "$brd"
cp "$brs" "$br/s.before"
if br_run scope >/dev/null 2>&1; then echo "smoke(enter-stage): boundary-require should fail closed on an absent member" >&2; exit 1; fi
cmp -s "$br/s.before" "$brs" || { echo "smoke(enter-stage): wrote state on a fail-closed boundary-require refusal" >&2; exit 1; }

write_br_fixture closing-iter                                            # (c) member names the closing iteration → boundary entry proceeds
printf '# contract\n\nclosing-iter release none — smoke basis\n' > "$brd"
br_run scope >/dev/null || { echo "smoke(enter-stage): boundary-require should pass when the member names the closing iteration" >&2; exit 1; }
grep -q "^## Iteration: —\$" "$brq" || { echo "smoke(enter-stage): boundary-require pass did not reset the header" >&2; exit 1; }

write_br_fixture —                                                       # (d) never-named (—) closing iteration skips the check even with no line
printf '# contract\n' > "$brd"
br_run scope >/dev/null || { echo "smoke(enter-stage): boundary-require should skip a never-named closing iteration" >&2; exit 1; }

# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the iteration-boundary entry refuses while the gap inbox holds bullets (the Lessons-refusal contract); an empty (header-only) inbox passes
gp="$es/gap-inbox"; mkdir -p "$gp/.workflow"
gpq="$gp/TASK-QUEUE.md"; gps="$gp/.workflow/WORKFLOW-STATE.txt"; gpi="$gp/.workflow/gap-inbox.md"
cat > "$gpq" <<EOF
# smoke queue
## Iteration: gap-iter

---

## New Features
## Technical Debt
## Deferred
## Done
## Lessons Learned
EOF
cat > "$gps" <<EOF
# contract

---

gap-iter close aabbccdd $d
EOF
cat > "$gp/stages.sh" <<STAGES
# shellcheck shell=bash
LIFECYCLE_KIT_GAP_INBOX_FILE=$gpi
STAGES
gp_run() {
    env -u CLAUDE_CODE_SESSION_ID -u CLAUDE_CODE_CHILD_SESSION -u LIFECYCLE_KIT_SESSION_ID \
    LIFECYCLE_KIT_QUEUE_FILE="$gpq" \
    LIFECYCLE_KIT_STATE_FILE="$gps" \
    LIFECYCLE_KIT_SESSIONS_DIR="$es/sessions" \
    GATE_SDK_TMP_DIR="$es/tmp" \
    LIFECYCLE_KIT_CONFIG_FILE="$gp/stages.sh" \
    bash "$SMOKE_KIT_ROOT/bin/enter-stage.sh" "$@"
}
printf '# contract: gap inbox\n- 2026-07-17 — an untriaged gap bullet\n' > "$gpi"   # (a) non-empty inbox → refuse, nothing written
cp "$gpq" "$gp/q.before"; cp "$gps" "$gp/s.before"
if gp_run scope >/dev/null 2>&1; then echo "smoke(enter-stage): should refuse a boundary entry while the gap inbox holds bullets" >&2; exit 1; fi
cmp -s "$gp/s.before" "$gps" || { echo "smoke(enter-stage): wrote state on a gap-inbox refusal" >&2; exit 1; }
cmp -s "$gp/q.before" "$gpq" || { echo "smoke(enter-stage): wrote queue on a gap-inbox refusal" >&2; exit 1; }
printf '# contract: gap inbox\n' > "$gpi"                                            # (b) header-only inbox → boundary entry proceeds
gp_run scope >/dev/null || { echo "smoke(enter-stage): a header-only gap inbox should not refuse a boundary entry" >&2; exit 1; }
grep -q "^## Iteration: —\$" "$gpq" || { echo "smoke(enter-stage): gap-inbox pass did not reset the header" >&2; exit 1; }

rm -rf "$es"
