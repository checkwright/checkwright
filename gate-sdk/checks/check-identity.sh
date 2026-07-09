#!/usr/bin/env bash
# graph: couples=scripts/identity.conf dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-identity — every expectation in the identity manifest matches this clone's local git identity
#
# usage: check-identity.sh [--fixture <dir>] [manifest]
#   default manifest: GATE_SDK_IDENTITY_FILE (<gates-dir>/identity.conf);
#   --fixture <dir> injects a clone's git identity for deterministic tests
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

MODE=live
FIXTURE_DIR=""
MANIFEST=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --fixture) MODE=fixture; FIXTURE_DIR="${2:-}"; shift 2 ;;
        -*) echo "check-identity: unknown argument: $1" >&2; exit 2 ;;
        *) MANIFEST="$1"; shift ;;
    esac
done

if [[ "$MODE" == fixture ]]; then
    [[ -d "$FIXTURE_DIR" ]] || { echo "check-identity: fixture dir not found: $FIXTURE_DIR" >&2; exit 2; }
    MANIFEST="${MANIFEST:-$FIXTURE_DIR/identity.conf}"
else
    MANIFEST="${MANIFEST:-${GATE_SDK_IDENTITY_FILE:-$(gate_sdk_gates_dir)/identity.conf}}"
fi

if [[ ! -e "$MANIFEST" ]]; then
    echo "IDENTITY: clean (no manifest at $MANIFEST — optional consumer config absent)"
    exit 0
fi
[[ -r "$MANIFEST" ]] || { echo "check-identity: manifest not readable: $MANIFEST" >&2; exit 2; }

if [[ "$MODE" == live ]]; then
    # spec: gate-sdk/SPEC.md §check-identity — CI is not a committing clone (no
    # local identity to misattribute a commit/push with), so the guard steps aside.
    if [[ -n "${CI:-}" ]]; then
        echo "IDENTITY: clean (CI context — not a committing clone; identity guard skipped)"
        exit 0
    fi
    git rev-parse --git-dir >/dev/null 2>&1 || {
        echo "check-identity: not a git repository — cannot verify identity" >&2; exit 2; }
fi

actual_email() {
    if [[ "$MODE" == fixture ]]; then
        [[ -f "$FIXTURE_DIR/git-config-email" ]] || return 0
        head -n1 "$FIXTURE_DIR/git-config-email"
    else
        git config user.email 2>/dev/null || true
    fi
}

# spec: gate-sdk/SPEC.md §check-identity — a configured remote absent from this
# clone is red; return 1 signals the missing remote to the comparison below.
actual_remote_url() {
    local remote="$1" r u _rest
    if [[ "$MODE" == fixture ]]; then
        [[ -f "$FIXTURE_DIR/git-remotes" ]] || return 1
        while read -r r u _rest; do
            [[ "$r" == "$remote" ]] && { printf '%s' "$u"; return 0; }
        done < <(gates_list_members "$FIXTURE_DIR/git-remotes")
        return 1
    fi
    git remote get-url "$remote" 2>/dev/null
}

# spec: gate-sdk/SPEC.md §check-identity — the host part is the SSH alias /
# hostname that selects the identity; parse scp-like and scheme:// URL forms.
extract_host() {
    local url="$1" h
    case "$url" in
        *://*)
            h="${url#*://}"; h="${h#*@}"; h="${h%%/*}"; h="${h%%:*}" ;;
        *:*)
            h="${url%%:*}"; h="${h#*@}" ;;
        *)
            h="" ;;
    esac
    printf '%s' "$h"
}

mapfile -t members < <(gates_list_members "$MANIFEST")

malformed=(); mismatches=(); checked=0
for line in "${members[@]}"; do
    read -r -a f <<<"$line"
    case "${f[0]}" in
        email)
            if [[ ${#f[@]} -ne 2 ]]; then malformed+=("$line"); continue; fi
            checked=$((checked + 1))
            act="$(actual_email)"
            [[ "$act" == "${f[1]}" ]] ||
                mismatches+=("email: manifest expects '${f[1]}', clone has '${act:-<unset>}'")
            ;;
        remote-host)
            if [[ ${#f[@]} -ne 3 ]]; then malformed+=("$line"); continue; fi
            checked=$((checked + 1))
            if url="$(actual_remote_url "${f[1]}")"; then
                host="$(extract_host "$url")"
                [[ "$host" == "${f[2]}" ]] ||
                    mismatches+=("remote-host ${f[1]}: manifest expects '${f[2]}', clone has '${host:-<unparseable>}' (url: $url)")
            else
                mismatches+=("remote-host ${f[1]}: no such remote in this clone")
            fi
            ;;
        *)
            malformed+=("$line")
            ;;
    esac
done

if [[ ${#malformed[@]} -gt 0 ]]; then
    echo "check-identity: malformed line(s) in $MANIFEST (expected 'email <addr>' or 'remote-host <remote> <host>'):" >&2
    printf '  %s\n' "${malformed[@]}" >&2
    exit 2
fi

if [[ ${#mismatches[@]} -gt 0 ]]; then
    echo "check-identity: local git identity does not match $MANIFEST:"
    printf '  %s\n' "${mismatches[@]}"
    echo "  help: this clone commits/pushes under the wrong identity — fix the git"
    echo "        mapping (user.email via includeIf, the remote's SSH host alias via"
    echo "        core.sshCommand/remote URL), or — if the expectation itself moved —"
    echo "        update the matching line in $MANIFEST."
    exit 1
fi

echo "IDENTITY: clean ($checked expectation(s) match this clone's git identity in $MANIFEST)"
exit 0
