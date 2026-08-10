#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §build-native — the one spelling of the crate build, so its readers cite a script rather than each carrying a copy of the command (a tool, not a gate; no # graph: manifest)
#
# usage: build-native.sh [cargo-arg…]
#   Run from the repo root. Trailing arguments reach cargo unchanged, so a
#   per-target build passes --target <triple>. Exit status is cargo's.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

# spec: gate-sdk/SPEC.md §build-native — the crate is resolved relative to cwd, never off this
# script's own location: a caller running from a scratch tree sets cwd and gets that tree's crate.
CRATE="$(gate_native_crate)"

# spec: gate-sdk/SPEC.md §Fail-closed contract — an absent toolchain is exit 2 naming the floor,
# never a bare command-not-found from the shell
command -v cargo >/dev/null 2>&1 || {
    echo "build-native: cargo is not on PATH, so the gate binary cannot be built." >&2
    echo "  help: cargo is the contributor-side toolchain floor for this tree" >&2
    echo "        (context-kit/SPEC.md §bin/env-probe). Install a Rust toolchain, then re-run." >&2
    exit 2
}

# spec: gate-sdk/SPEC.md §build-native — the message names the consumer case, so a vendored copy
# reached by a tree that has no crate reads as a misuse rather than as a broken tool
[[ -f "$CRATE/Cargo.toml" ]] || {
    echo "build-native: no crate at $CRATE — $CRATE/Cargo.toml is absent, so there is nothing to build." >&2
    echo "  help: in a consumer tree this script has been reached by mistake. A consumer receives a" >&2
    echo "        prebuilt, digest-verified binary per declared target and never the crate source" >&2
    echo "        (gate-sdk/SPEC.md §Consumer payload), so nothing there builds it." >&2
    echo "        In a tree that does carry the crate, run this from the repo root, or point" >&2
    echo "        GATE_SDK_NATIVE_CRATE at the crate directory." >&2
    exit 2
}

cargo build --release --manifest-path "$CRATE/Cargo.toml" "$@"
