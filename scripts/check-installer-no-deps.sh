#!/usr/bin/env bash
# graph: couples=installer/package.json dir=one valve=none tier=precommit
# spec: CLAUDE.md §Housekeeping — the installer package declares no resolvable-dependency field and no install-time lifecycle script, the two shapes that would turn a one-shot vendoring installer into a dependency channel or a run-on-install code path
#
# usage: check-installer-no-deps.sh [package-json]
#   default: installer/package.json; one positional arg steers onto a fixture copy.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

PKG="${1:-installer/package.json}"
[[ -f "$PKG" ]] || { echo "check-installer-no-deps: package file not found: $PKG" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || {
    echo "check-installer-no-deps: jq not found on PATH — the gate cannot run." >&2
    echo "  A gate that cannot run is not clean (fail-closed)." >&2
    exit 2
}

# spec: CLAUDE.md §Housekeeping — the field's presence is the finding, not its emptiness: an empty dependency map declares a channel with nothing in it yet, and a lifecycle hook is code that runs on install whatever its body
findings="$(jq -r '
    . as $p
    | ( ["dependencies", "peerDependencies", "optionalDependencies"][] as $k
        | select($p | has($k))
        | "resolvable-dependency field declared: \($k)" ),
      ( ["preinstall", "install", "postinstall"][] as $k
        | select(($p.scripts // {}) | has($k))
        | "install-time lifecycle script declared: scripts.\($k)" )
' "$PKG")"; st=$?
fail_closed "$st" check-installer-no-deps jq

if [[ -n "$findings" ]]; then
    echo "check-installer-no-deps: $PKG would make the installer a resolved-dependency channel rather than a one-shot vendoring:"
    echo ""
    printf '  %s\n' "$findings"
    echo "  help: drop the field. The installer copies bundled source and commits it —"
    echo "        nothing may resolve at an adopter's build time and nothing may run at"
    echo "        install time. A payload the package needs is assembled at pack time and"
    echo "        shipped inside the tarball, never fetched."
    exit 1
fi

echo "INSTALLER-NO-DEPS: clean ($PKG declares no dependency field and no install-time lifecycle script)"
exit 0
