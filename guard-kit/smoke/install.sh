#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — guard-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored guard-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30. Leg 2: this is an executable install recipe by stated contract, and check-install-disposition assertion B reads its body as text, so a crate table crosses harder the recipe-into-derivation boundary §Consumer smoke already declined to cross, and ADDS violations rather than removing them. Leg 3: it vendors to an adopter with its kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers in existence are this repo own validate suites — so it costs an adopter no interpreter dependency. Structural, not a sizing judgment, and stated rather than cited-by-example because the class had no precedent in either direction before that ruling.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-gates.sh --run-consumer-smoke}"

cp "$SMOKE_KIT_ROOT/templates/bash-guard.sh"     scripts/bash-guard.sh
cp "$SMOKE_KIT_ROOT/templates/guard-config.knobs" scripts/guard-config.knobs

mkdir -p .claude
if [[ -f .claude/settings.json ]]; then
    prior_events="$(jq -c '.hooks // {} | keys' .claude/settings.json)"
    jq -s '.[0] * .[1] | del(.["//"])' \
        .claude/settings.json "$SMOKE_KIT_ROOT/templates/settings-hooks.json" \
        > .claude/settings.json.new
    mv .claude/settings.json.new .claude/settings.json
    if ! jq -e --argjson prior "$prior_events" '$prior - (.hooks // {} | keys) == []' \
        .claude/settings.json >/dev/null; then
        echo "guard-kit/smoke/install.sh: merging the hook wiring dropped a hook event a co-vendored kit had wired (had $prior_events)" >&2
        exit 1
    fi
else
    jq 'del(.["//"])' "$SMOKE_KIT_ROOT/templates/settings-hooks.json" > .claude/settings.json
fi

# spec: guard-kit/SPEC.md §The recommended allowlist — a union into permissions.allow, never a replacement
sentinel='Bash(smoke-sentinel --prior-grant)'
jq --arg s "$sentinel" '.permissions.allow = ((.permissions.allow // []) + [$s])' \
    .claude/settings.json > .claude/settings.json.new
mv .claude/settings.json.new .claude/settings.json
jq --slurpfile t "$SMOKE_KIT_ROOT/templates/settings-allow.json" \
    '(.permissions.allow // []) as $a
     | .permissions.allow = ($a + [$t[0].permissions.allow[] | select(. as $e | $a | index([$e]) | not)])
     | del(.["//"])' \
    .claude/settings.json > .claude/settings.json.new
mv .claude/settings.json.new .claude/settings.json
if ! jq -e --arg s "$sentinel" --slurpfile t "$SMOKE_KIT_ROOT/templates/settings-allow.json" \
    '.permissions.allow as $a | ($a | index([$s])) != null
     and ($t[0].permissions.allow - $a == [])
     and ($a | length) == ($a | unique | length)' .claude/settings.json >/dev/null; then
    echo "guard-kit/smoke/install.sh: the allow union dropped a prior grant, missed a template entry or duplicated one" >&2
    exit 1
fi

{
    echo '.workflow/prompt-friction.log'
    echo '.workflow/wakeup-attempts.log'
} >> .gitignore

set +e
msg="$(printf '%s' '{"tool_input":{"command":"cd deploy && ls"}}' | bash scripts/bash-guard.sh 2>&1 >/dev/null)"
rc=$?
set -e
# spec: guard-kit/SPEC.md §Testing — the block must be rule 1's, since a knob load the binary refuses also exits 2
if [[ "$rc" -ne 2 || "$msg" != *"'cd'"* ]]; then
    echo "guard-kit/smoke/install.sh: installed guard did not block a compound-cd payload with rule 1's steer (exit $rc, want 2): $msg" >&2
    exit 1
fi

# spec: guard-kit/SPEC.md §The recommended allowlist — the ruleset blocks no form the template grants
while IFS= read -r entry; do
    cmd="${entry#Bash(}"
    cmd="${cmd%)}"
    cmd="${cmd//\*/x}"
    set +e
    msg="$(jq -cn --arg c "$cmd" '{tool_input:{command:$c}}' | bash scripts/bash-guard.sh 2>&1 >/dev/null)"
    rc=$?
    set -e
    if [[ "$rc" -eq 2 ]]; then
        echo "guard-kit/smoke/install.sh: the installed guard blocks '$cmd', a form templates/settings-allow.json grants ($entry): $msg" >&2
        exit 1
    fi
done < <(jq -r '.permissions.allow[]' "$SMOKE_KIT_ROOT/templates/settings-allow.json")
