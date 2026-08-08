#!/usr/bin/env bash
# spec: installer/README.md §init — update is init with one added precondition and its argv forwarded verbatim: checkwright.lock must already exist, so a verb named update can manage an install but never perform the first one. Every init flag stays valid, including --profile, --force, --no-commit and --dry-run, so the mutating-verb --dry-run obligation is discharged by delegation rather than a second implementation that could drift from it
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

# spec: installer/README.md §The manifest — the one added precondition, and the whole behavioral difference from init: checkwright.lock must already exist at the repository root, so a verb named update never performs a first install. It checks existence only and no more — a present-but-unreadable schema, a stale downgrade, a below-contract toolchain, and every other init precondition are init's own checks, one call away, and repeating any of them here would be a second copy that could drift from the original. Not being inside a git work tree at all is not this verb's precondition to own either: init's own "not inside a git work tree" refusal already exists and already names the accurate remedy, so an unresolvable root here just falls through to it rather than being reported as an absent manifest
ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"
if [[ -n "$ROOT" ]] && [[ ! -f "$(lock_path "$ROOT")" ]]; then
    die "no $CHECKWRIGHT_LOCK_FILE at $ROOT" \
        "update manages an install init already made; there isn't one yet. Run 'checkwright init' first."
fi

# spec: installer/README.md §init — everything else, including --dry-run, --force and --no-commit, is init's own contract, unrepeated here
exec bash "$INSTALLER/lib/init.sh" "$@"
