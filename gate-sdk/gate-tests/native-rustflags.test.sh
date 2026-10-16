#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §build-native — caller flags precede remaps, empty and root prefixes add nothing
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
source "$GATE_SDK_TEST_LIB_DIR/gate.sh"

check_flags() {
    local got
    got="$(gate_native_rustflags)"
    [[ "$got" == "$1" ]] || { printf 'FAIL: expected <%s>, got <%s>\n' "$1" "$got"; return 1; }
}
RUSTFLAGS='-C opt-level=1' CARGO_HOME=/cache/ HOME=/builder-home/ \
    check_flags '-C opt-level=1 --remap-path-prefix=/cache=/builder --remap-path-prefix=/builder-home=/builder' || exit 1
(
    unset CARGO_HOME RUSTFLAGS
    HOME=/builder-home check_flags '--remap-path-prefix=/builder-home/.cargo=/builder --remap-path-prefix=/builder-home=/builder'
) || exit 1
RUSTFLAGS='' CARGO_HOME=/ HOME='' check_flags '' || exit 1
RUSTFLAGS='caller' CARGO_HOME=/ HOME=/ check_flags 'caller ' || exit 1
echo 'NATIVE-RUSTFLAGS: clean (caller flags, default cargo home, trailing slash, empty and root prefixes)'
