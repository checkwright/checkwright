#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §scratch-run — the runner echoes an in-scratch script's contents, executes a snapshot of exactly those bytes, passes the child's args and exit code through verbatim, and refuses (exit 2, nothing echoed, nothing executed) any target resolving outside GATE_SDK_TMP_DIR or stating an interpreter its extension and GUARD_KIT_SCRATCH_POWERSHELL do not admit. Every case here is a property of the SEAM — the front end resolving the arm, the binary reading its knobs, and a real child process — which a crate unit test cannot see; the shebang classifier's own cases and the containment predicate's are pinned in the ported module's #[cfg(test)] tests, where check-crate-arms runs them.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" || exit 2
RUN="gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "scratch-run.test: front end not found: $RUN"; exit 2; }
unset GUARD_KIT_SCRATCH_POWERSHELL

# Under Git Bash the binary is a native Windows program, so every path it is handed is spelled in
# the mixed dialect both sides read (the liveness step's precedent); elsewhere cygpath is absent.
spell() { if command -v cygpath >/dev/null 2>&1; then cygpath -m "$1"; else printf '%s' "$1"; fi; }
scratch="$(spell "$(mktemp -d)")"; outside="$(spell "$(mktemp -d)")"
trap 'rm -rf "$scratch" "$outside"' EXIT

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }
assert_rc()     { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }; }
# A run leaves no snapshot behind: nothing dot-led in the scratch dir but what the case put there.
assert_no_snapshot() {
    local left; left="$(find "$scratch" -maxdepth 1 -name '.*' ! -name '.' 2>/dev/null)"
    [[ -z "$left" ]] || { echo "FAIL [$1]: a snapshot remains: $left"; fails=$((fails + 1)); }
}

# The echoed body is what makes an allowlisted (prompt-free) execution
# self-documenting, so the assertion is on the script's own text, not just a
# header: a runner that printed only the path would satisfy a weaker check.
printf '#!/usr/bin/env bash\necho "ran with: $*"\nexit 7\n' > "$scratch/probe.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/probe.sh" alpha beta 2>&1)"; rc=$?
assert_rc  echo-then-exec "$rc" 7
assert_has echo-then-exec 'echo "ran with: $*"' "$out"
assert_has echo-then-exec "scratch-run: $scratch/probe.sh" "$out"
assert_has echo-then-exec 'ran with: alpha beta' "$out"
assert_no_snapshot snapshot-removed-on-7

# Ordering is the evidence property: the contents must appear before the run's
# own output, else the transcript reads the effect ahead of the cause.
body_line="$(grep -n 'echo "ran with' <<<"$out" | head -1 | cut -d: -f1)"
ran_line="$(grep -n '^ran with: alpha beta' <<<"$out" | head -1 | cut -d: -f1)"
[[ -n "$body_line" && -n "$ran_line" && "$body_line" -lt "$ran_line" ]] || {
    echo "FAIL [echo-before-exec]: script body did not precede its output"; fails=$((fails + 1)); }

# The runner executes the bytes it echoed: the body's first line rewrites, in place, a line of the
# target sitting past bash's read buffer. Run off the target, bash would read the rewritten line;
# run off the snapshot, the echoed line is the one that runs.
pad=102400
head_fmt="printf 'echo INJECTED' | dd of='%s' bs=1 seek=%06d conv=notrunc 2>/dev/null\n"
hdr_len="$(printf "$head_fmt" "$scratch/window.sh" 0 | wc -c)"
{
    printf "$head_fmt" "$scratch/window.sh" "$((hdr_len + pad))"
    head -c "$((pad - 1))" /dev/zero | tr '\0' '#'
    printf '\necho ORIGINAL\n'
} > "$scratch/window.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/window.sh" 2>&1)"; rc=$?
assert_rc     window "$rc" 0
assert_has    window "ORIGINAL" "$(grep -v '^echo ' <<<"$out" | grep -v '^#' || true)"
assert_absent window "INJECTED" "$(grep -v '^printf ' <<<"$out" || true)"
grep -qF 'echo INJECTED' "$scratch/window.sh" || { echo "FAIL [window]: the body did not rewrite its target"; fails=$((fails + 1)); }

# The snapshot sits beside the target under another, dot-led name, and is gone after a clean exit.
printf 'echo "zero=$0"\n' > "$scratch/zero.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/zero.sh" 2>&1)"; rc=$?
assert_rc     snapshot "$rc" 0
assert_has    snapshot "zero=$scratch/.zero." "$out"
assert_absent snapshot "zero=$scratch/zero.sh" "$out"
assert_has    snapshot "executing $scratch/zero.sh as $scratch/.zero." "$out"
assert_no_snapshot snapshot-removed-on-0

# Fail-closed: an out-of-scratch target is refused before any echo or
# execution — the side-effect file proves the child never ran.
printf '#!/usr/bin/env bash\ntouch "%s/EXECUTED"\n' "$outside" > "$outside/evil.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$outside/evil.sh" 2>&1)"; rc=$?
assert_rc     refuse-outside "$rc" 2
assert_has    refuse-outside "refusing" "$out"
assert_absent refuse-outside "touch" "$out"
[[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-outside]: the child ran"; fails=$((fails + 1)); }

# A path spelled under the scratch dir but resolving above it is the same
# refusal — the guard tests the resolved path, never the spelling.
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/../$(basename "$outside")/evil.sh" 2>&1)"; rc=$?
assert_rc  refuse-traversal "$rc" 2
assert_has refuse-traversal "refusing" "$out"
[[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-traversal]: the child ran"; fails=$((fails + 1)); }

# A symlink planted INSIDE the scratch dir pointing out of it is the case a
# lexical `..`-normalizing compare would pass: the test reads the resolved path,
# so the containment control stays fail-closed through the port. Git Bash copies
# on a plain `ln -s`, so the native form is asked for and a copy is a skip.
MSYS=winsymlinks:nativestrict ln -s "$outside" "$scratch/escape" 2>/dev/null
if [[ -L "$scratch/escape" ]]; then
    out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/escape/evil.sh" 2>&1)"; rc=$?
    assert_rc  refuse-symlink "$rc" 2
    assert_has refuse-symlink "refusing" "$out"
    [[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-symlink]: the child ran"; fails=$((fails + 1)); }
else
    echo "scratch-run.test: skip [refuse-symlink]: this host made no symlink"
fi
rm -rf "$scratch/escape"

out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/absent.sh" 2>&1)"; rc=$?
assert_rc  missing-target "$rc" 2
assert_has missing-target "no such script" "$out"

out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run 2>&1)"; rc=$?
assert_rc  no-args "$rc" 2
assert_has no-args "usage" "$out"

# Scratch execution runs in bash, or in PowerShell where the project names a host, and the runner
# reads the file's own shebang rather than a roster: a target naming another interpreter is refused
# unexecuted, and the side-effect file proves it never ran.
printf '#!/usr/bin/env python3\nopen("%s/PYRAN", "w").close()\n' "$scratch" > "$scratch/py.py"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/py.py" 2>&1)"; rc=$?
assert_rc     refuse-shebang "$rc" 2
assert_has    refuse-shebang "runs in bash" "$out"
assert_absent refuse-shebang 'open("' "$out"
[[ -e "$scratch/PYRAN" ]] && { echo "FAIL [refuse-shebang]: the child ran"; fails=$((fails + 1)); }

# The env-indirected spelling resolves to the same interpreter, and a bash
# shebang — direct or through env — still runs, which is what keeps every .sh
# the runner handles today working.
printf '#!/bin/sh\necho sh-ran\n' > "$scratch/posix.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/posix.sh" 2>&1)"; rc=$?
assert_rc  allow-sh-shebang "$rc" 0
assert_has allow-sh-shebang "sh-ran" "$out"

# A target with no shebang at all is unaffected: nothing states an interpreter,
# so nothing contradicts the rule.
printf 'echo no-shebang-ran\n' > "$scratch/plain.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/plain.sh" 2>&1)"; rc=$?
assert_rc  allow-no-shebang "$rc" 0
assert_has allow-no-shebang "no-shebang-ran" "$out"

# The PowerShell path is off until the project names a host: a .ps1 is refused unexecuted, printing
# no body, and the refusal names the knob.
printf "New-Item -ItemType File -Path '%s/PSRAN' | Out-Null\n" "$scratch" > "$scratch/off.ps1"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/off.ps1" 2>&1)"; rc=$?
assert_rc     ps-off "$rc" 2
assert_has    ps-off "GUARD_KIT_SCRATCH_POWERSHELL" "$out"
assert_absent ps-off "New-Item" "$out"
[[ -e "$scratch/PSRAN" ]] && { echo "FAIL [ps-off]: the child ran"; fails=$((fails + 1)); }

# A file whose shebang contradicts its extension is refused unexecuted, either way round.
printf '#!/bin/bash\ntouch "%s/CONTRA1"\n' "$scratch" > "$scratch/contra.ps1"
out="$(GATE_SDK_TMP_DIR="$scratch" GUARD_KIT_SCRATCH_POWERSHELL=pwsh bash "$RUN" --scratch-run "$scratch/contra.ps1" 2>&1)"; rc=$?
assert_rc     contra-ps1 "$rc" 2
assert_has    contra-ps1 "PowerShell body" "$out"
assert_absent contra-ps1 "touch" "$out"
printf '#!/usr/bin/env pwsh\nNew-Item -ItemType File -Path "%s/CONTRA2"\n' "$scratch" > "$scratch/contra.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/contra.sh" 2>&1)"; rc=$?
assert_rc     contra-sh "$rc" 2
assert_has    contra-sh "named .ps1" "$out"
assert_absent contra-sh "New-Item" "$out"
[[ -e "$scratch/CONTRA1" || -e "$scratch/CONTRA2" ]] && { echo "FAIL [contra]: a child ran"; fails=$((fails + 1)); }

# A host that does not resolve is refused unexecuted; a value naming no PowerShell host is refused
# by the knob validator before anything is read.
out="$(GATE_SDK_TMP_DIR="$scratch" GUARD_KIT_SCRATCH_POWERSHELL="$outside/pwsh" bash "$RUN" --scratch-run "$scratch/off.ps1" 2>&1)"; rc=$?
assert_rc     ps-host-absent "$rc" 2
assert_has    ps-host-absent "does not resolve" "$out"
assert_absent ps-host-absent "New-Item" "$out"
out="$(GATE_SDK_TMP_DIR="$scratch" GUARD_KIT_SCRATCH_POWERSHELL=python3 bash "$RUN" --scratch-run "$scratch/plain.sh" 2>&1)"; rc=$?
assert_rc     ps-host-invalid "$rc" 2
assert_has    ps-host-invalid "must be empty, or name pwsh or powershell (got 'python3')" "$out"
assert_absent ps-host-invalid "no-shebang-ran" "$out"
[[ -e "$scratch/PSRAN" ]] && { echo "FAIL [ps-host]: the child ran"; fails=$((fails + 1)); }
assert_no_snapshot snapshot-none-on-refusal

# Per PowerShell host on PATH: the echo precedes the run, a quoted argument arrives whole, the exit
# code passes through, the script sees the snapshot as its own path, and no snapshot remains.
cat > "$scratch/ps.ps1" <<'PS1'
Write-Output "ps-args: $($args.Count) [$($args[0])] [$($args[1])]"
Write-Output "ps-path: $PSCommandPath"
exit 7
PS1
hosts_run=()
for host in pwsh powershell; do
    if ! command -v "$host" >/dev/null 2>&1; then
        echo "scratch-run.test: skip [ps-$host]: no $host on PATH"
        continue
    fi
    hosts_run+=("$host")
    out="$(GATE_SDK_TMP_DIR="$scratch" GUARD_KIT_SCRATCH_POWERSHELL="$host" bash "$RUN" --scratch-run "$scratch/ps.ps1" alpha 'b c' 2>&1 | tr -d '\r')"; rc=$?
    assert_rc  "ps-$host-exit" "$rc" 7
    assert_has "ps-$host-args" "ps-args: 2 [alpha] [b c]" "$out"
    body_line="$(grep -n '^Write-Output "ps-args' <<<"$out" | head -1 | cut -d: -f1)"
    ran_line="$(grep -n '^ps-args: 2' <<<"$out" | head -1 | cut -d: -f1)"
    [[ -n "$body_line" && -n "$ran_line" && "$body_line" -lt "$ran_line" ]] || {
        echo "FAIL [ps-$host-echo-before-exec]: script body did not precede its output"; fails=$((fails + 1)); }
    self="$(grep '^ps-path: ' <<<"$out" | head -1)"; self="${self##*[/\\]}"
    [[ "$self" == .ps.*.ps1 ]] || { echo "FAIL [ps-$host-snapshot]: \$PSCommandPath names '$self', not a snapshot"; fails=$((fails + 1)); }
    assert_no_snapshot "ps-$host-snapshot-removed"
done

[[ "$fails" -eq 0 ]] || { echo "scratch-run.test: $fails assertion(s) failed"; exit 1; }
echo "scratch-run.test: clean (echo precedes exec; the echoed bytes are the bytes run, off a dot-led snapshot beside the target that no run leaves behind; args and exit code pass through; out-of-scratch, traversal and symlink-escape targets refused unexecuted; a shebang its extension does not admit refused unexecuted; the PowerShell path refused while off, on an unresolved host and on an invalid knob; PowerShell hosts run: ${hosts_run[*]:-none})"
exit 0
