#!/usr/bin/env bash
# spec: RELEASING.md §The front-door rehearsal — the Unix legs' driver: a stranger's first session, first upgrade and package install of one published release, from a seat holding no checkout
# no-port: directed 2026-10-10 by the operator, asked and answered in a `/lead` session and lead-relayed: a direction and no ruling. The subject is the line a stranger types into a shell and the bootstrap that places the compiled binary, so a compiled driver would test the artifact with itself. The cause is this file's and never a class: nothing else may cite it, and a second driver is a new file with its own disposition.
# usage: ci-front-door.sh <version> [<previous-version>]
#   Run outside every git work tree, on a seat with no `checkwright` on PATH.
#   Reads nothing from a checkout, and runs under bash 3.2, stock macOS's.
set -uo pipefail

usage() {
    printf 'usage: %s <version> [<previous-version>]\n' "${0##*/}"
    printf '  <version>           the published release to rehearse, as X.Y.Z\n'
    printf '  <previous-version>  the release before it; empty or absent skips the upgrade session\n'
}

case "${1:-}" in
    -h|--help) usage; exit 0 ;;
esac
if [ "$#" -lt 1 ] || [ "$#" -gt 2 ] || [ -z "$1" ]; then
    usage >&2
    exit 2
fi
case "$1" in
    -*) usage >&2; exit 2 ;;
esac
version="$1"
previous="${2:-}"

if command -v checkwright >/dev/null 2>&1; then
    echo "front-door: seat: 'checkwright' already resolves on PATH ($(command -v checkwright)), so this seat is not clean" >&2
    exit 2
fi
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "front-door: seat: the working directory $PWD is inside a git work tree, so this seat is not clean" >&2
    exit 2
fi

install_url='https://checkwright.dev/install.sh'
package='checkwright'
lock='checkwright.lock'

base="$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/front-door.XXXXXX")" || {
    echo "front-door: seat: could not make a scratch directory" >&2
    exit 2
}
trap 'rm -rf "$base"' EXIT
log="$base/step.log"
: > "$log"
session=''
repo=''
gates_dir=''
notes=0

run() { ( cd "$repo" && "$@" ) >> "$log" 2>&1; }

ok() {
    printf 'front-door: %s: %s: ok\n' "$session" "$1"
    : > "$log"
}

bad() {
    printf 'front-door: %s: %s: FAILED: %s\n' "$session" "$1" "$2"
    cat "$log"
    : > "$log"
    return 1
}

one_line() {
    local at="$1"
    shift
    curl -fsSL "$install_url" | CHECKWRIGHT_VERSION="$at" sh -s -- "$@"
}

new_repo() {
    session="$1"
    repo="$base/$2"
    git init -q "$repo" >> "$log" 2>&1 || return 1
    git -C "$repo" config user.name 'front door' || return 1
    git -C "$repo" config user.email 'front-door@example.invalid' || return 1
}

lock_field() {
    sed -n 's/^[[:space:]]*"'"$1"'":[[:space:]]*"\([^"]*\)".*$/\1/p' "$repo/$lock" | head -n 1
}

install_at() {
    local name="$1" at="$2"
    shift 2
    run one_line "$at" "$@" || { bad "$name" "the one-line install at $at exited non-zero"; return 1; }
    [ -f "$repo/$lock" ] || { bad "$name" "the install left no $lock"; return 1; }
    [ "$(lock_field version)" = "$at" ] || { bad "$name" "$lock names version '$(lock_field version)', not $at"; return 1; }
    printf 'front-door: %s: %s: ok\n' "$session" "$name"
}

# spec: installer/SPEC.md §init — the follow-up block's stated grammar: the `next:` banner, one command per indented line, commentary from the first `#`
follow_up() {
    local name="$1" block="$base/follow-up" count=0 hooks=0 cmd target
    awk '
        on && /^[ \t]+[^ \t]/ { sub(/#.*/, ""); sub(/^[ \t]+/, ""); sub(/[ \t]+$/, ""); if ($0 != "") print; next }
        on { exit }
        $0 == "next:" { on = 1 }
    ' "$log" > "$block"
    : > "$log"
    while IFS= read -r cmd <&3; do
        count=$((count + 1))
        case "$cmd" in
            *--install-hooks*) hooks=1 ;;
        esac
        if [ -z "$gates_dir" ]; then
            for target in $cmd; do
                case "$target" in
                    */*) gates_dir="${target%/*}"; break ;;
                esac
            done
        fi
        run sh -c "$cmd" || { bad "$name" "the printed command '$cmd' exited non-zero"; return 1; }
    done 3< "$block"
    [ "$count" -gt 0 ] || { bad "$name" "init printed no follow-up block"; return 1; }
    [ "$hooks" = 1 ] || { bad "$name" "no printed command places the hooks"; return 1; }
    ok "$name ($count printed command(s))"
}

commit_lands() {
    local name="$1"
    : > "$log"
    notes=$((notes + 1))
    printf 'front-door rehearsal note %s\n' "$notes" > "$repo/front-door-note-$notes.txt"
    run git add -- "front-door-note-$notes.txt" || { bad "$name" "git add failed"; return 1; }
    run git commit -m "docs: add front-door rehearsal note $notes" || { bad "$name" "a clean commit was refused"; return 1; }
    grep -q '^pre-commit: ' "$log" || { bad "$name" "the commit landed with no pre-commit hook line, so no placed hook ran"; return 1; }
    ok "$name"
}

# spec: installer/SPEC.md §The consumer smoke — the planted defect is that arm's: a pipeline that can lose its match, the refusing gate read off the hook's own line and held to the installed registry
commit_refused() {
    local name="$1" plant='front-door-plant.sh' gate registry
    : > "$log"
    printf '%s\n' '#!/usr/bin/env bash' 'set -o pipefail' 'names=(a b c)' \
        'if printf "%s\n" "${names[@]}" | grep -q b; then echo found; fi' > "$repo/$plant"
    run git add -- "$plant" || { bad "$name" "git add failed"; return 1; }
    if run git commit -m "chore: add a pipeline that can lose its match"; then
        bad "$name" "a commit carrying the planted defect landed"
        return 1
    fi
    gate="$(sed -n 's/^pre-commit: \([^[:space:]]*\) failed.*$/\1/p' "$log" | head -n 1)"
    [ -n "$gate" ] || { bad "$name" "the refusal names no gate"; return 1; }
    registry="$repo/$gates_dir/gates.list"
    grep -qxF "$gate" "$registry" || { bad "$name" "the refusal names $gate, which $gates_dir/gates.list does not register"; return 1; }
    run git rm -q -f --cached -- "$plant" || { bad "$name" "could not unstage the plant"; return 1; }
    rm -f "$repo/$plant"
    ok "$name (refused by $gate)"
}

profile_move() {
    local name="$1" installed roster other='' p
    : > "$log"
    installed="$(lock_field profile)"
    run one_line "$version" init --help || { bad "$name" "init --help exited non-zero"; return 1; }
    roster="$(sed -n 's/^profiles:[[:space:]]*//p' "$log" | head -n 1)"
    for p in $roster; do
        [ "$p" = "$installed" ] || other="$p"
    done
    [ -n "$other" ] || { bad "$name" "the artifact's roster '$roster' names no profile other than the installed '$installed'"; return 1; }
    : > "$log"
    run one_line "$version" init --profile "$other" || { bad "$name" "init --profile $other over the hooked $installed install exited non-zero"; return 1; }
    [ "$(lock_field profile)" = "$other" ] || { bad "$name" "$lock names profile '$(lock_field profile)', not $other"; return 1; }
    ok "$name ($installed -> $other)"
}

first_session() {
    new_repo 'one-line first session' first || { bad 'repository' 'git init failed'; return 1; }
    gates_dir=''
    install_at 'install' "$version" || return 1
    follow_up 'follow-up block' || return 1
    commit_lands 'clean commit' || return 1
    commit_refused 'first refusal' || return 1
    profile_move 'profile move' || return 1
    commit_lands 'commit after the profile move' || return 1
}

first_upgrade() {
    new_repo 'one-line first upgrade' upgrade || { bad 'repository' 'git init failed'; return 1; }
    gates_dir=''
    install_at "install at $previous" "$previous" || return 1
    follow_up "follow-up block at $previous" || return 1
    commit_lands "commit at $previous" || return 1
    install_at "update to $version" "$version" update || return 1
    commit_lands 'commit after the update' || return 1
}

package_route() {
    new_repo 'package' package || { bad 'repository' 'git init failed'; return 1; }
    run npx --yes "$package@$version" init || { bad 'init' "npx $package@$version init exited non-zero"; return 1; }
    [ -f "$repo/$lock" ] || { bad 'init' "the install left no $lock"; return 1; }
    [ "$(lock_field version)" = "$version" ] || { bad 'init' "$lock names version '$(lock_field version)', not $version"; return 1; }
    ok 'init'
}

findings=0
first_session || findings=1
if [ -n "$previous" ]; then
    first_upgrade || findings=1
else
    echo "front-door: one-line first upgrade: skipped, no earlier release was given"
fi
package_route || findings=1

if [ "$findings" -ne 0 ]; then
    echo "front-door: v$version: a step above failed"
    exit 1
fi
echo "front-door: v$version: every session clean"
