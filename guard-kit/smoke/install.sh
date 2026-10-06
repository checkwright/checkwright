#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — guard-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored guard-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

cp "$SMOKE_KIT_ROOT/templates/guard-config.knobs" scripts/guard-config.knobs

cat >> scripts/gates.list <<'EOF'
# guard-kit
check-door-binding
EOF

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

# spec: guard-kit/SPEC.md §The recommended allowlist — the binary grant's placeholder, resolved the
# way an adopter resolves it and then executed, because a grant that parses is not one that spawns
# shellcheck source=/dev/null
source "$SMOKE_KIT_ROOT/../gate-sdk/lib/gate.sh"
door="$(gate_native_bin_spelled)"
if [[ ! -x "$door" ]]; then
    echo "guard-kit/smoke/install.sh: the allowlist's binary grant resolves to '$door', which is not an executable file — the sweep re-points every adopter surface at a door this consumer cannot spawn" >&2
    exit 1
fi
if ! "$door" --emit knob-values GATE_SDK_NATIVE_BIN >/dev/null 2>&1; then
    echo "guard-kit/smoke/install.sh: the granted door '$door' is executable but did not run an arm" >&2
    exit 1
fi
allow=.claude/settings-allow.resolved.json
jq --arg d "$door" '.permissions.allow |= map(gsub("@GATE_SDK_NATIVE_BIN@"; $d))' \
    "$SMOKE_KIT_ROOT/templates/settings-allow.json" > "$allow"
# spec: guard-kit/SPEC.md §The recommended allowlist — the grants alone, never the "//" header,
# which names the placeholder because it is the surface telling the adopter to replace it
if jq -e '[.permissions.allow[] | select(test("@GATE_SDK_NATIVE_BIN@"))] | length > 0' "$allow" >/dev/null; then
    echo "guard-kit/smoke/install.sh: an unresolved @GATE_SDK_NATIVE_BIN@ placeholder survived into the merged allowlist" >&2
    exit 1
fi

# spec: gate-sdk/SPEC.md §check-graph — the graph artifact carries a row per registered gate, so
# registering one above leaves it stale until it is re-emitted
"$door" --emit-graph > scripts/CHECK-GRAPH.html

# spec: guard-kit/SPEC.md §The recommended allowlist — a union into permissions.allow, never a replacement
sentinel='Bash(smoke-sentinel --prior-grant)'
jq --arg s "$sentinel" '.permissions.allow = ((.permissions.allow // []) + [$s])' \
    .claude/settings.json > .claude/settings.json.new
mv .claude/settings.json.new .claude/settings.json
jq --slurpfile t "$allow" \
    '(.permissions.allow // []) as $a
     | .permissions.allow = ($a + [$t[0].permissions.allow[] | select(. as $e | $a | index([$e]) | not)])
     | del(.["//"])' \
    .claude/settings.json > .claude/settings.json.new
mv .claude/settings.json.new .claude/settings.json
if ! jq -e --arg s "$sentinel" --slurpfile t "$allow" \
    '.permissions.allow as $a | ($a | index([$s])) != null
     and ($t[0].permissions.allow - $a == [])
     and ($a | length) == ($a | unique | length)' .claude/settings.json >/dev/null; then
    echo "guard-kit/smoke/install.sh: the allow union dropped a prior grant, missed a template entry or duplicated one" >&2
    exit 1
fi

{
    echo '.workflow/prompt-friction.log'
    echo '.workflow/wakeup-attempts.log'
    echo '.workflow/*.drain'
    echo '.workflow/*.drain.part'
} >> .gitignore

set +e
msg="$(printf '%s' '{"tool_name":"Bash","tool_input":{"command":"cd deploy && ls"}}' | "$door" --hook shell-guard 2>&1 >/dev/null)"
rc=$?
set -e
# spec: guard-kit/SPEC.md §Testing — the block must be rule `cd_compound`'s, since a knob load the binary refuses also exits 2
if [[ "$rc" -ne 2 || "$msg" != *"'cd'"* ]]; then
    echo "guard-kit/smoke/install.sh: the wired member did not block a compound-cd payload with rule \`cd_compound\`'s steer (exit $rc, want 2): $msg" >&2
    exit 1
fi
# spec: guard-kit/SPEC.md §Testing — the merged wiring hands the PowerShell tool to the member, and
# the member reads that tool's grammar
if ! jq -e '[.hooks.PreToolUse[] | select(.matcher == "Bash|PowerShell") | .hooks[].command
             | select(test("--hook shell-guard"))] | length == 1' .claude/settings.json >/dev/null; then
    echo "guard-kit/smoke/install.sh: the merged wiring carries no Bash|PowerShell matcher group running the shell-guard" >&2
    exit 1
fi
# spec: guard-kit/SPEC.md §Testing — the wiring string itself, run as the harness runs it from a
# subdirectory, so a command resolved against the hook's working directory reds here
wired="$(jq -r '[.hooks.PreToolUse[] | select(.matcher == "Bash|PowerShell") | .hooks[].command
                 | select(test("--hook shell-guard"))][0]' .claude/settings.json)"
consumer_root="$(pwd -P)"
mkdir -p smoke-subdir/deeper
set +e
msg="$(cd smoke-subdir/deeper && printf '%s' '{"tool_name":"Bash","tool_input":{"command":"cd deploy && ls"}}' | CLAUDE_PROJECT_DIR="$consumer_root" bash -c "$wired" 2>&1 >/dev/null)"
rc=$?
set -e
rm -rf smoke-subdir
if [[ "$rc" -ne 2 || "$msg" != *"'cd'"* ]]; then
    echo "guard-kit/smoke/install.sh: the wired command '$wired', run from a subdirectory, did not block a compound-cd payload with rule \`cd_compound\`'s steer (exit $rc, want 2): $msg" >&2
    exit 1
fi
set +e
msg="$(printf '%s' '{"tool_name":"PowerShell","tool_input":{"command":"Set-Location deploy; Get-ChildItem"}}' | "$door" --hook shell-guard 2>&1 >/dev/null)"
rc=$?
set -e
if [[ "$rc" -ne 2 || "$msg" != *"'Set-Location'"* ]]; then
    echo "guard-kit/smoke/install.sh: the wired member did not block a PowerShell compound Set-Location payload with rule \`cd_compound\`'s steer (exit $rc, want 2): $msg" >&2
    exit 1
fi

# spec: guard-kit/SPEC.md §The recommended allowlist — the ruleset blocks no form the template
# grants, read from the RESOLVED allowlist so the binary grant is exercised in the spelling an
# adopter actually merges rather than as its placeholder
while IFS= read -r entry; do
    cmd="${entry#Bash(}"
    cmd="${cmd%)}"
    cmd="${cmd//\*/x}"
    set +e
    msg="$(jq -cn --arg c "$cmd" '{tool_name:"Bash",tool_input:{command:$c}}' | "$door" --hook shell-guard 2>&1 >/dev/null)"
    rc=$?
    set -e
    if [[ "$rc" -eq 2 ]]; then
        echo "guard-kit/smoke/install.sh: the wired member blocks '$cmd', a form templates/settings-allow.json grants ($entry): $msg" >&2
        exit 1
    fi
done < <(jq -r '.permissions.allow[]' "$allow")
