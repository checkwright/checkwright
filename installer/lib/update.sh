#!/usr/bin/env bash
# spec: installer/README.md §init — update is init with one added precondition and its argv forwarded verbatim: checkwright.lock must already exist and carry a known schema, so a verb named update can manage an install but never perform the first one. Every init flag stays valid, including --profile, --force, --no-commit and --dry-run, so the mutating-verb --dry-run obligation is discharged by delegation rather than a second implementation that could drift from it
#
# usage: checkwright update [--profile <name>] [--dry-run] [--force] [--no-commit]
#   Every init flag stays valid; see 'checkwright init --help' for what each one does.
set -uo pipefail

INSTALLER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=./common/lock.sh
source "$INSTALLER/lib/common/lock.sh"

# spec: installer/README.md §init — -h/--help is intercepted first and answers on its own, outside any repository precondition, exactly as every other verb's --help does
for arg in "$@"; do
    case "$arg" in
        -h|--help)
            printf 'usage: checkwright update [--profile <name>] [--dry-run] [--force] [--no-commit]\n\n'
            printf 'Runs checkwright init with the same arguments, refusing when there is no\n'
            printf 'existing install for it to update. Every init flag is valid here — see\n'
            printf "'checkwright init --help' for what each one does.\n"
            exit 0 ;;
    esac
done

die() { printf 'checkwright update: %s\n' "$1" >&2; [[ -n "${2:-}" ]] && printf '  help: %s\n' "$2" >&2; exit "${3:-2}"; }

# spec: installer/README.md §The manifest — the one added precondition: checkwright.lock must exist at the repository root and carry a known schema before update ever delegates to init, so a verb named update cannot perform a first install. Not being inside a git work tree at all reads the same way as an absent manifest — either way there is no existing install here for update to manage
ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"
LOCK=""
[[ -n "$ROOT" ]] && LOCK="$(lock_path "$ROOT")"
[[ -n "$LOCK" && -f "$LOCK" ]] || die "no $CHECKWRIGHT_LOCK_FILE here" \
    "update manages an install init already made; there isn't one yet. Run 'checkwright init' first."
lock_schema_ok "$LOCK" || die "$CHECKWRIGHT_LOCK_FILE carries a schema this build does not know" \
    "this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for."

# spec: installer/README.md §init — the whole behavioral difference from init ends at the precondition above; everything else, including --dry-run, --force and --no-commit, is init's own contract, unchanged and unrepeated here
exec bash "$INSTALLER/lib/init.sh" "$@"
