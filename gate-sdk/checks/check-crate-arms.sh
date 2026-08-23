#!/usr/bin/env bash
# graph: couples=native/Cargo.toml,native/build.rs,native/src/*.rs,native/src/gates/*.rs dir=one valve=none tier=precommit
# install: never
# spec: gate-sdk/SPEC.md §check-crate-arms — the crate's lint and test arms run at commit time, so a battery that passes cannot coexist with a CI that fails on them
#
# usage: check-crate-arms.sh
#   The crate comes from GATE_SDK_NATIVE_CRATE and the build scratch from
#   GATE_SDK_CARGO_TARGET_DIR; there are no positional arguments.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

CRATE="$(gate_native_crate)"
TARGET_DIR="${GATE_SDK_CARGO_TARGET_DIR:-$CRATE/target}"

# spec: gate-sdk/SPEC.md §check-crate-arms — the predicate is the crate's presence, never
# cargo's: a consumer tree is missing the corpus, not the toolchain, and a gate with no
# corpus reports clean rather than declaring a skip
[[ -f "$CRATE/Cargo.toml" ]] || {
    echo "CRATE-ARMS: clean (no crate at $CRATE — $CRATE/Cargo.toml is absent, so there is no corpus to lint or test)"
    exit 0
}

# spec: gate-sdk/SPEC.md §Fail-closed contract — a crate with no toolchain to check it is
# "cannot verify", which must not share an exit code with "verified clean"
command -v cargo >/dev/null 2>&1 || {
    echo "check-crate-arms: cargo is not on PATH but a crate is present at $CRATE — the check could not run; treating as failure (not clean)" >&2
    echo "  help: cargo is the contributor-side toolchain floor for a tree carrying the crate" >&2
    echo "        (context-kit/SPEC.md §bin/env-probe). Install a Rust toolchain, then re-run." >&2
    exit 2
}

# spec: gate-sdk/SPEC.md §check-crate-arms — the source-stamp cache: both arms re-run only when the
# crate's tracked-source stamp or the toolchain differs from the last green run recorded under
# GATE_SDK_TMP_DIR; an untracked file under the crate, or a crate outside git, never hits the cache
CACHE="${GATE_SDK_TMP_DIR:-.tmp}/crate-arms-$(printf '%s' "$CRATE" | git hash-object --stdin 2>/dev/null || echo crate).green"
key=""
if stamp="$(gate_native_source_stamp 2>/dev/null)" \
    && [[ -z "$(git -C "$CRATE" ls-files --others --exclude-standard -- . 2>/dev/null)" ]]; then
    key="$stamp $(rustc --version 2>/dev/null) $(cargo --version 2>/dev/null)"
    if [[ -n "$stamp" && -f "$CACHE" && "$(cat "$CACHE")" == "$key" ]]; then
        echo "CRATE-ARMS: clean (cached — source stamp ${stamp:0:12} and toolchain unchanged since the last green run recorded at $CACHE; cargo clippy --all-targets at -D warnings and cargo test, both --release over $CRATE)"
        exit 0
    fi
fi

fail=0

# spec: gate-sdk/SPEC.md §check-crate-arms — both arms run even when the first fails, so one
# commit-time report carries what CI would have said in two
lint="$(cargo clippy --release --manifest-path "$CRATE/Cargo.toml" --target-dir "$TARGET_DIR" --all-targets -- -D warnings 2>&1)"; st=$?
if [[ "$st" -ne 0 ]]; then
    echo "check-crate-arms: cargo clippy failed (exit $st) on $CRATE:"
    echo "$lint"
    fail=1
fi

unit="$(cargo test --release --manifest-path "$CRATE/Cargo.toml" --target-dir "$TARGET_DIR" 2>&1)"; st=$?
if [[ "$st" -ne 0 ]]; then
    echo "check-crate-arms: cargo test failed (exit $st) on $CRATE:"
    echo "$unit"
    fail=1
fi

if [[ "$fail" -ne 0 ]]; then
    echo "  help: fix the finding above. These are the arms CI runs, and this gate is now their"
    echo "        only spelling — the battery plus bash gate-sdk/bin/build-native.sh is the whole"
    echo "        commit-time obligation, and neither discharges the other."
    exit 1
fi

if [[ -n "$key" ]] && mkdir -p "$(dirname "$CACHE")" 2>/dev/null; then
    printf '%s' "$key" >"$CACHE" 2>/dev/null || true
fi
echo "CRATE-ARMS: clean (cargo clippy --all-targets at -D warnings and cargo test, both --release over $CRATE, build scratch $TARGET_DIR)"
exit 0
