#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §git-hook — the binary under a served hook's name is that hook's launcher: git starts it with no interpreter, a clean commit lands on the summary line, a red member refuses the commit by gate name, a registry with no commit-msg member leaves that hook silent, a binary that cannot be started refuses naming the path and both remedies, a knob naming a hook's own name is refused at 2, and a registered member whose declaration resolves and cannot be read refuses either hook at 2
# spec: gate-sdk/SPEC.md §install-hooks — the arm places both hooks untracked in gate-hooks under the common git directory with an absolute core.hooksPath, so a linked worktree runs them; a re-run replaces both; an absent binary refuses at 2 and places nothing; a failed core.hooksPath write refuses at 2 ahead of the receipt; a failed blame.ignoreRevsFile write is reported with the status unmoved; a hook left behind by a replaced binary starts the binary now installed
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
# shellcheck source=../lib/gate.sh
source "$HERE/gate-sdk/lib/gate.sh"
PINNED="$GATE_SDK_NATIVE_BIN"
[[ -x "$PINNED" ]] || { echo "native-git-hooks.test: the gate binary $PINNED is absent — build it first"; exit 2; }
unset GATE_SDK_NATIVE_BIN GATE_SDK_GATES_DIR GATE_SDK_KIT_DIRS GATE_SDK_ROOT GATE_SDK_VERBOSE

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
sfx="$(gate_exe_suffix)"

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }
g() { git -C "$repo" -c user.email=t@example.invalid -c user.name=t "$@"; }

repo="$SANDBOX/repo"
bin_rel="native/target/release/checkwright-gates$sfx"
mkdir -p "$repo/gate-sdk" "$repo/scripts" "$repo/${bin_rel%/*}"
cp -R "$HERE/gate-sdk/bin" "$HERE/gate-sdk/lib" "$repo/gate-sdk/"
cp "$PINNED" "$repo/$bin_rel"
printf 'native/target/\n' >"$repo/.gitignore"
printf 'check-probe-word\n' >"$repo/scripts/gates.list"
cat >"$repo/scripts/check-probe-word.sh" <<'EOF'
#!/bin/sh
# graph: couples=*.txt dir=one valve=none tier=precommit
if grep -l FORBIDDEN ./*.txt 2>/dev/null; then
    echo "check-probe-word: a tracked note carries the forbidden word"
    exit 1
fi
echo "PROBE-WORD: clean"
EOF
chmod +x "$repo/scripts/check-probe-word.sh"
git -C "$repo" init -q -b main
g add -A
g commit -qm seed

hooks="$repo/.git/gate-hooks"

# 1. an absent binary refuses the opt-in at 2 and places nothing
mv "$repo/$bin_rel" "$SANDBOX/held"
out="$( cd "$repo" && "$SANDBOX/held" --install-hooks 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] || note absent-install-status "want exit 2 with no binary at the knob's path, got $rc -- $out"
grep -qF "no gate binary at $bin_rel" <<<"$out" || note absent-install-text "the refusal does not name the path: $out"
[[ -e "$hooks" ]] && note absent-install-wrote "a refused opt-in left a hooks directory"
[[ -z "$(git -C "$repo" config --get core.hooksPath)" ]] || note absent-install-config "a refused opt-in set core.hooksPath"
mv "$SANDBOX/held" "$repo/$bin_rel"

# 2. the opt-in places both hooks under the common git directory and points the clone at them
out="$( cd "$repo" && "./$bin_rel" --install-hooks 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note install-status "want exit 0 from --install-hooks, got $rc -- $out"
for h in pre-commit commit-msg; do
    [[ -x "$hooks/$h$sfx" ]] || note "placed-$h" "no executable $h$sfx in $hooks"
    grep -qxF "  $h$sfx" <<<"$out" || note "receipt-$h" "the receipt does not list $h$sfx: $out"
done
path="$(git -C "$repo" config --get core.hooksPath)"
gate_path_rooted "$path" || note hooks-path-absolute "core.hooksPath is not absolute: $path"
[[ "$(cd "$path" 2>/dev/null && pwd -P)" == "$(cd "$hooks" && pwd -P)" ]] || note hooks-path "core.hooksPath names $path, not $hooks"
[[ -z "$(g status --porcelain)" ]] || note untracked "the opt-in changed the work tree: $(g status --porcelain)"

# 3. a clean commit lands through the native hook, the commit-msg hook silent with no member at its tier
printf 'a note\n' >"$repo/a.txt"
g add a.txt
out="$(g commit -m 'docs: a clean note' 2>&1)"; rc=$?
[[ "$rc" -eq 0 ]] || note clean-status "a clean commit was refused ($rc): $out"
grep -qF 'pre-commit: 1 gate(s) passed.' <<<"$out" || note clean-summary "the pre-commit summary line is missing: $out"
grep -qF 'commit-msg:' <<<"$out" && note msg-silent "the commit-msg hook printed with no member at its tier: $out"

# 4. a violating commit is refused with the gate's name
printf 'FORBIDDEN\n' >"$repo/b.txt"
g add b.txt
out="$(g commit -m 'docs: a bad note' 2>&1)"; rc=$?
[[ "$rc" -ne 0 ]] || note red-status "a violating commit landed: $out"
grep -qF 'pre-commit: check-probe-word failed (see above).' <<<"$out" || note red-name "the refusal does not name the gate: $out"
g reset -q -- b.txt
rm -f "$repo/b.txt"

# 5. a linked worktree runs the same hooks, its own binary at the knob's path
wt="$SANDBOX/wt"
g worktree add -q -b side "$wt" HEAD
mkdir -p "$wt/${bin_rel%/*}"
cp "$PINNED" "$wt/$bin_rel"
printf 'FORBIDDEN\n' >"$wt/c.txt"
git -C "$wt" add c.txt
out="$(git -C "$wt" -c user.email=t@example.invalid -c user.name=t commit -m 'docs: from a worktree' 2>&1)"; rc=$?
[[ "$rc" -ne 0 ]] || note worktree-status "a violating commit landed from a linked worktree: $out"
grep -qF 'pre-commit: check-probe-word failed (see above).' <<<"$out" || note worktree-name "the worktree's refusal does not name the gate: $out"
g worktree remove --force "$wt"

# 6. a registered commit-msg member is reached with no re-install
cat >"$repo/scripts/check-probe-msg.sh" <<'EOF'
#!/bin/sh
# graph: couples=* dir=one valve=none tier=commit-msg
if grep -q WIP "$1"; then
    echo "check-probe-msg: a WIP subject"
    exit 1
fi
echo "PROBE-MSG: clean"
EOF
chmod +x "$repo/scripts/check-probe-msg.sh"
printf 'check-probe-word\ncheck-probe-msg\n' >"$repo/scripts/gates.list"
g add -A
out="$(g commit -m 'WIP: a subject' 2>&1)"; rc=$?
[[ "$rc" -ne 0 ]] || note msg-red-status "a commit-msg member's red did not refuse the commit: $out"
grep -qF 'commit-msg: check-probe-msg failed (see above).' <<<"$out" || note msg-red-name "the refusal does not name the commit-msg gate: $out"
out="$(g commit -m 'chore: register a message gate' 2>&1)"; rc=$?
[[ "$rc" -eq 0 ]] || note msg-green-status "a clean message was refused ($rc): $out"
grep -qF 'commit-msg: 1 gate(s) passed.' <<<"$out" || note msg-green-summary "the commit-msg summary line is missing: $out"

# 7. a hook left behind by a replaced binary starts the binary now installed
cat >"$SANDBOX/stub" <<'EOF'
#!/bin/sh
echo "stub started: $*"
exit 7
EOF
chmod +x "$SANDBOX/stub"
mv "$repo/$bin_rel" "$SANDBOX/held"
cp "$SANDBOX/stub" "$repo/$bin_rel"
if [[ -z "$sfx" ]]; then
    out="$( cd "$repo" && "$hooks/commit-msg" some-file 2>&1 )"; rc=$?
    [[ "$rc" -eq 7 ]] || note stale-status "the placed hook did not return the installed binary's status, got $rc -- $out"
    grep -qxF 'stub started: --git-hook commit-msg some-file' <<<"$out" || note stale-argv "the placed hook did not start the installed binary on the arm with git's operands: $out"
fi

# 8. a binary that cannot be started refuses the commit, naming the path and both remedies
rm -f "$repo/$bin_rel"
printf 'another note\n' >"$repo/d.txt"
g add d.txt
out="$(g commit -m 'docs: with no binary' 2>&1)"; rc=$?
[[ "$rc" -ne 0 ]] || note absent-status "a commit landed with no gate binary to judge it: $out"
grep -qF "pre-commit: cannot start the gate binary at $bin_rel" <<<"$out" || note absent-path "the refusal does not name the resolved path: $out"
grep -qF 'place or build the binary GATE_SDK_NATIVE_BIN names' <<<"$out" || note absent-remedy "the refusal does not name the place-or-build remedy: $out"
grep -qF 'git commit --no-verify' <<<"$out" || note absent-bypass "the refusal does not name the bypass: $out"
out="$(g commit --no-verify -m 'docs: with no binary' 2>&1)" || note absent-bypass-works "the bypass did not land the commit: $out"
mv "$SANDBOX/held" "$repo/$bin_rel"

# 9. a knob naming a hook's own name is refused at 2, since the launcher would start itself
out="$( cd "$repo" && GATE_SDK_NATIVE_BIN="$hooks/pre-commit$sfx" "$hooks/pre-commit$sfx" 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] || note self-status "want exit 2 on a knob naming a hook's own name, got $rc -- $out"
grep -qF 'the launcher would start itself' <<<"$out" || note self-text "the refusal does not say why: $out"

# 10. a re-run replaces both hooks and leaves the same two
rm -f "$hooks/commit-msg$sfx" "$hooks/pre-commit$sfx"
printf 'stale\n' >"$hooks/pre-commit$sfx"
out="$( cd "$repo" && "./$bin_rel" --install-hooks 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note rerun-status "want exit 0 from a re-run, got $rc -- $out"
for h in pre-commit commit-msg; do
    cmp -s "$hooks/$h$sfx" "$repo/$bin_rel" || note "rerun-$h" "a re-run did not replace $h$sfx with the binary"
done
[[ "$(find "$hooks" -type f | wc -l | tr -d ' ')" -eq 2 ]] || note rerun-count "a re-run left other than the two hooks: $(ls "$hooks")"

# 11. a failed core.hooksPath write refuses the opt-in at 2, ahead of the receipt
: >"$repo/.git/config.lock"
out="$( cd "$repo" && "./$bin_rel" --install-hooks 2>&1 )"; rc=$?
rm -f "$repo/.git/config.lock"
[[ "$rc" -eq 2 ]] || note unwired-status "want exit 2 when core.hooksPath cannot be written, got $rc -- $out"
grep -qF 'could not set core.hooksPath' <<<"$out" || note unwired-text "the refusal does not name the key: $out"
grep -qF 'Active hooks:' <<<"$out" && note unwired-receipt "a refused opt-in printed its receipt: $out"

# 13. a failed blame.ignoreRevsFile write is reported and moves no status; a key already holding two values fails that write alone
: >"$repo/.git-blame-ignore-revs"
git -C "$repo" config --add blame.ignoreRevsFile one
git -C "$repo" config --add blame.ignoreRevsFile two
out="$( cd "$repo" && "./$bin_rel" --install-hooks 2>&1 )"; rc=$?
git -C "$repo" config --unset-all blame.ignoreRevsFile
rm -f "$repo/.git-blame-ignore-revs"
[[ "$rc" -eq 0 ]] || note blame-status "want exit 0 when only blame.ignoreRevsFile cannot be written, got $rc -- $out"
grep -qF 'could not set blame.ignoreRevsFile' <<<"$out" || note blame-text "the failed write is not reported by key: $out"
grep -qF 'Active hooks:' <<<"$out" || note blame-receipt "the opt-in printed no receipt: $out"

# 12. a registered member that resolves and cannot be read refuses either hook at 2; a host that reads through the mode (an administrator, a filesystem with no mode bits) cannot build the case
printf 'subject\n' >"$SANDBOX/msg"
printf 'a staged note\n' >"$repo/e.txt"
g add e.txt
for pair in "commit-msg:check-probe-msg" "pre-commit:check-probe-word"; do
    hook="${pair%%:*}"; member="$repo/scripts/${pair##*:}.sh"
    chmod 000 "$member"
    if ! head -c 1 "$member" >/dev/null 2>&1; then
        if [[ "$hook" == commit-msg ]]; then
            out="$( cd "$repo" && "./$bin_rel" --git-hook commit-msg "$SANDBOX/msg" 2>&1 )"; rc=$?
        else
            out="$( cd "$repo" && "./$bin_rel" --git-hook pre-commit 2>&1 )"; rc=$?
        fi
        [[ "$rc" -eq 2 ]] || note "unreadable-$hook-status" "want exit 2 over an unreadable resolved member, got $rc -- $out"
        grep -qF "$hook: cannot read the declaration scripts/${pair##*:}.sh" <<<"$out" || note "unreadable-$hook-text" "the refusal does not name the declaration: $out"
    fi
    chmod 755 "$member"
done
g reset -q -- e.txt
rm -f "$repo/e.txt"

[[ "$fails" -eq 0 ]] || { echo "native-git-hooks.test: $fails assertion(s) failed"; exit 1; }
echo "native-git-hooks.test: clean (--install-hooks places both hooks untracked under the common git directory, absolute in core.hooksPath, refusing with nothing placed when the binary is absent; git starts the placed binary as each hook with no interpreter: a clean commit lands on the summary line, a red member refuses by gate name from the main checkout and from a linked worktree, the commit-msg hook is silent until a member registers at its tier and reaches one with no re-install; a hook outliving a replaced binary starts the installed one, an unstartable binary refuses naming the path and both remedies, a knob naming a hook is refused at 2, a re-run replaces both, a failed core.hooksPath write refuses at 2 with no receipt, a failed blame.ignoreRevsFile write is reported at exit 0, and an unreadable resolved member refuses either hook at 2)"
exit 0
