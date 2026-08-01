#!/usr/bin/env bash
# graph: couples=docs/install.md,.github/workflows/publish.yml dir=one valve=none tier=precommit
# spec: docs/install.md §Versioning — the declared release channel agrees with the publish workflow's prerelease posture (A) and with the project's own version line (B)
#
# usage: check-release-channel-parity.sh [install-md [publish-yml [version]]]
#   version defaults to the newest tag by creator date; pass one explicitly to
#   compare against a fixed line.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

INSTALL_MD="${1:-docs/install.md}"
PUBLISH_YML="${2:-.github/workflows/publish.yml}"
VERSION_ARG="${3-}"

[[ -f "$INSTALL_MD" ]] || { echo "check-release-channel-parity: not found: $INSTALL_MD" >&2; exit 2; }
[[ -f "$PUBLISH_YML" ]] || { echo "check-release-channel-parity: not found: $PUBLISH_YML" >&2; exit 2; }

decls="$(grep -n '^Release channel:' "$INSTALL_MD")"; st=$?
[[ "$st" -le 1 ]] || fail_closed "$st" RELEASE-CHANNEL-PARITY grep
n_decl="$(grep -c . <<<"${decls}")"
[[ -n "$decls" ]] || n_decl=0

if [[ "$n_decl" -eq 0 ]]; then
    echo "check-release-channel-parity: $INSTALL_MD carries no 'Release channel:' declaration line — the channel cannot be established (docs/install.md §Versioning owns the declaration)" >&2
    exit 2
fi
if [[ "$n_decl" -gt 1 ]]; then
    echo "check-release-channel-parity: $INSTALL_MD carries $n_decl 'Release channel:' declaration lines; exactly one is admissible:" >&2
    echo "$decls" >&2
    exit 2
fi

channel="$(sed -n -E 's/^Release channel:[[:space:]]*\*\*([a-z]+)\*\*[[:space:]]*$/\1/p' "$INSTALL_MD")"; st=$?
fail_closed "$st" RELEASE-CHANNEL-PARITY sed
case "$channel" in
    preview|stable) ;;
    *)
        echo "check-release-channel-parity: $INSTALL_MD declares an unrecognized channel value (${channel:-<unparseable>}); the two admissible values are 'preview' and 'stable' (docs/install.md §Versioning)" >&2
        exit 2
        ;;
esac

create_step="$(grep -n 'gh release create' "$PUBLISH_YML")"; st=$?
[[ "$st" -le 1 ]] || fail_closed "$st" RELEASE-CHANNEL-PARITY grep
if [[ -z "$create_step" ]]; then
    echo "check-release-channel-parity: $PUBLISH_YML has no recognizable Release-creating step ('gh release create'); the prerelease posture cannot be established" >&2
    exit 2
fi

has_prerelease=0
grep -qF -- '--prerelease' <<<"$create_step" && has_prerelease=1

findings=()
if [[ "$channel" == "preview" && "$has_prerelease" -eq 0 ]]; then
    findings+=("  invariant A: channel 'preview' demands --prerelease on the Release-creating step, and $PUBLISH_YML carries none:")
    findings+=("$(sed 's/^/    /' <<<"$create_step")")
elif [[ "$channel" == "stable" && "$has_prerelease" -eq 1 ]]; then
    findings+=("  invariant A: channel 'stable' demands the absence of --prerelease, and $PUBLISH_YML carries it:")
    findings+=("$(sed 's/^/    /' <<<"$create_step")")
fi

# spec: docs/install.md §Versioning — invariant B is dormant, and says so, where no tag exists
version="$VERSION_ARG"
if [[ -z "$version" ]]; then
    version="$(git for-each-ref --sort=-creatordate --count=1 --format='%(refname:strip=2)' refs/tags 2>/dev/null)"
fi

b_state=""
if [[ -z "$version" ]]; then
    b_state="dormant"
else
    if [[ ! "$version" =~ ^v?([0-9]+)\.([0-9]+)\.([0-9]+)([-+].*)?$ ]]; then
        echo "check-release-channel-parity: the version line ('$version') does not parse as semver, so the channel cannot be compared against it" >&2
        exit 2
    fi
    major="${BASH_REMATCH[1]}"
    if [[ "$major" -eq 0 ]]; then
        b_state="v${version#v} is 0.x, which demands channel 'preview'"
        [[ "$channel" == "preview" ]] || findings+=("  invariant B: version line $b_state, but '$channel' is declared")
    else
        b_state="v${version#v} is 1.x or later, which demands channel 'stable'"
        [[ "$channel" == "stable" ]] || findings+=("  invariant B: version line $b_state, but '$channel' is declared")
    fi
fi

if [[ "${#findings[@]}" -gt 0 ]]; then
    echo "check-release-channel-parity: the declared release channel disagrees with a surface it governs (docs/install.md §Versioning):"
    printf '%s\n' "${findings[@]}"
    echo "  help: bring the declaration, the Release-creating step's --prerelease posture, and the version line into agreement — the channel is derived from the version line, so 'preview' belongs to a 0.x line and 'stable' from v1.0.0 onward."
    exit 1
fi

if [[ "$b_state" == "dormant" ]]; then
    echo "RELEASE-CHANNEL-PARITY: clean (channel '$channel' agrees with $PUBLISH_YML; invariant B dormant — no tags, so there is no version line to compare)"
else
    echo "RELEASE-CHANNEL-PARITY: clean (channel '$channel' agrees with $PUBLISH_YML and with the version line — $b_state)"
fi
exit 0
