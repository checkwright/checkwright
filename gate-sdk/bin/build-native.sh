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

# spec: gate-sdk/SPEC.md §build-native — the released artifact carries no builder path. rustc
# records a panic location per crate it compiles, and a dependency's is absolute under the cargo
# home, so an unremapped binary ships the builder's home directory to every adopter.
BN_REMAP=()
for _bn_from in "${CARGO_HOME:-$HOME/.cargo}" "${HOME:-}"; do
    # spec: gate-sdk/SPEC.md §build-native — a remap of `/` would rewrite every absolute path in
    # the binary, so an empty or root prefix contributes no flag rather than a catastrophic one
    [[ -n "$_bn_from" && "$_bn_from" != "/" ]] || continue
    BN_REMAP+=("--remap-path-prefix=${_bn_from%/}=/builder")
done
unset _bn_from
# spec: gate-sdk/SPEC.md §build-native — appended to the caller's RUSTFLAGS rather than replacing
# them, or a caller passing its own flags would silently lose them to this one
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }${BN_REMAP[*]}"
export RUSTFLAGS

cargo build --release --manifest-path "$CRATE/Cargo.toml" "$@" || exit $?

# spec: gate-sdk/SPEC.md §build-native — the remap is verified on the artifact rather than trusted
# from the flag: this is the one place every shipped build passes through, and a leak that reaches
# a consumer's tracked tree reds their first battery instead of this one
BN_OUT="$CRATE/target"
# spec: gate-sdk/SPEC.md §build-native — the --target value earns a name because two derivations now
# read it: the output directory, and the artifact's executable suffix below
BN_TARGET=""
for _bn_i in "$@"; do
    case "$_bn_i" in
        --target=*) BN_TARGET="${_bn_i#--target=}"; BN_OUT="$CRATE/target/$BN_TARGET" ;;
        --target)   _bn_want_target=1 ;;
        *)          [[ "${_bn_want_target:-}" == 1 ]] && { BN_TARGET="$_bn_i"; BN_OUT="$CRATE/target/$_bn_i"; _bn_want_target=0; } ;;
    esac
done
unset _bn_i _bn_want_target
# spec: gate-sdk/SPEC.md §build-native — the suffix is the *target*'s and never the host's: a cross
# build from Linux for a Windows triple emits `<name>.exe`, and an empty BN_TARGET is the host build
BN_ART="$BN_OUT/release/$(basename "$(gate_native_bin)")"
BN_ART="${BN_ART%.exe}$(gate_exe_suffix "$BN_TARGET")"
[[ -f "$BN_ART" ]] || {
    echo "build-native: cargo reported success but no artifact is at $BN_ART" >&2
    echo "  help: the artifact path is derived from the crate dir, any --target, and the executable" >&2
    echo "        suffix that target implies; if this build emits somewhere else, or under a suffix" >&2
    echo "        gate_exe_suffix does not derive, that derivation is what needs fixing." >&2
    exit 2
}

# spec: gate-sdk/SPEC.md §build-native — the banned set is the consumer's own resolved pattern
# roster, the same one check-tree-terms and check-commit-msg read, so the artifact is held to the
# tree's leak ban rather than to a second vocabulary this tool would have to carry
# spec: gate-sdk/SPEC.md §Fail-closed contract — the resolver's status is read from the command
# substitution and not from `mapfile`, whose own status says nothing about the producer's
BN_PATLIST="$(gate_msg_pattern_files)" || {
    echo "build-native: the banned-pattern set could not be resolved, so the artifact cannot be" >&2
    echo "  verified — treating the build as failed (not clean)." >&2
    exit 2
}
BN_PATFILES=()
while IFS= read -r _bn_f; do [[ -n "$_bn_f" ]] && BN_PATFILES+=("$_bn_f"); done <<<"$BN_PATLIST"
unset _bn_f
if [[ ${#BN_PATFILES[@]} -gt 0 ]]; then
    BN_PATS="$(grep -hEv '^[[:space:]]*(#|$)' "${BN_PATFILES[@]}")"
    BN_HITS=""
    [[ -n "$BN_PATS" ]] && BN_HITS="$(printf '%s\n' "$BN_PATS" | grep -aoEhf - "$BN_ART" | sort -u)"
    if [[ -n "$BN_HITS" ]]; then
        echo "build-native: the built artifact carries a banned pattern — it must not be shipped:" >&2
        printf '  %s\n' "$BN_ART" >&2
        while IFS= read -r _bn_h; do printf '    %s\n' "$_bn_h" >&2; done <<<"$BN_HITS"
        unset _bn_h
        echo "  help: this build path lost its --remap-path-prefix flags, or a prefix outside" >&2
        echo "        CARGO_HOME and HOME reached the binary. gate-sdk/SPEC.md §build-native owns" >&2
        echo "        the remap and why every shipped build passes through this script." >&2
        exit 2
    fi
fi
