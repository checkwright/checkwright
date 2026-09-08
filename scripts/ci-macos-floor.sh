#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer payload — the macOS build prerequisite the publish path and the CI producer both owe, spelled once so neither bootstraps a host the other did not
# no-port: ruled 2026-09-08 by the operator, asked and answered in a `/lead` session and lead-relayed. This body installs the interpreter and GNU userland that the build of the gate binary requires, and it runs on a checkout-only runner where nothing has been compiled — so a ported arm would have to be executed by the artifact whose build it is the precondition for. It is the runner-side face of the same irreducible the install bootstrap records (installer/README.md §The install boundary). The cause is this file's and never a class: nothing else may cite it, and a second bootstrap is a new file with its own disposition.
# usage: ci-macos-floor.sh
#   A workflow step body its caller guards on a macOS runner. It writes
#   GITHUB_PATH, which reaches later steps only, so it stays its own step.
set -euo pipefail

if [ -z "${GITHUB_PATH:-}" ]; then
    echo "ci-macos-floor.sh: GITHUB_PATH is unset — this bootstraps a runner and" >&2
    echo "  has nowhere to publish the ordering it installs." >&2
    exit 2
fi

# spec: docs/install.md §Requirements — the declared bash floor, installed rather than worked around; the stock interpreter is 3.2 and gate-sdk/lib/gate.sh's knob defaults use a 4.2 unary
brew install bash coreutils gawk

prefix="$(brew --prefix)"
# spec: gate-sdk/SPEC.md §Consumer payload — gnubin carries the members under their unprefixed names, ahead of /usr/bin, which is what ordering means for a path calling `sort` and `awk` rather than `gsort` and `gawk`
{
    echo "$prefix/opt/coreutils/libexec/gnubin"
    echo "$prefix/opt/gawk/libexec/gnubin"
    echo "$prefix/bin"
} >> "$GITHUB_PATH"
