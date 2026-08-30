# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §Consumer smoke — the shared scratch-consumer builder, and its sourcer set is wider than its builder's caller set; that section counts both
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — leg 1 of the class ruling of 2026-08-30, the config bridge. This library is sourced into its callers own shell and resolves GATE_SDK_NATIVE_BIN through lib/gate.sh accessor to place the binary, so it sits inside the bridge rather than beside it; the accounting the harness it builds then runs probes unregistered gates through gate_command, and §lib/gate.sh rules exactly one place a knob value is computed, so a crate-side form would be the second producer criterion 6 refuses. Three of its four sourcers are outside this cut and keep sourcing it (upgrade-smoke.sh, context-kit/smoke/agents-md.sh and demo/run-demo.sh, the last of which sources it for csmoke_place_binary alone rather than for the builder). Structural, not a sizing judgment.

# spec: gate-sdk/SPEC.md §Consumer smoke — csmoke_gate_descriptors: the one derivation of whether a kit set needs the binary at all, so a caller that must produce one before it can name a source tree asks the same question the placement asks
csmoke_gate_descriptors() {   # $@ = vendorable kit roots -> the number of .gate descriptors under them
    local n=0 r
    shopt -s nullglob
    for r in "$@"; do
        for _ in "$r"/checks/*.gate; do n=$((n + 1)); done
    done
    shopt -u nullglob
    printf '%s\n' "$n"
}

# spec: gate-sdk/SPEC.md §Consumer smoke — csmoke_place_binary: the vendored kit roots' descriptor set decides whether the scratch consumer needs the gate binary, and the caller names the checkout whose artifact it receives — a library resolving that from its own location hands every caller the invoking tree's binary, which is the pairing defect that made upgrade-smoke run one ref's shell against another's binary. Derived, so a kit set with no ported gate asks for nothing.
csmoke_place_binary() {   # $1 = the checkout whose native/ was built, $2.. = vendorable kit roots
    local host="$1" bin descriptors
    shift
    bin="$(gate_native_bin)"
    descriptors="$(csmoke_gate_descriptors "$@")"
    [[ "$descriptors" -gt 0 ]] || return 0

    [[ -n "$host" ]] || {
        echo "csmoke: $descriptors vendored .gate descriptor(s) need the gate binary and the caller named no checkout to take one from" >&2
        return 2
    }
    [[ -x "$host/$bin" ]] || {
        echo "csmoke: $descriptors vendored .gate descriptor(s) need the gate binary, but $host/$bin is absent or not executable" >&2
        echo "  help: build it — bash gate-sdk/bin/build-native.sh — then re-run." >&2
        return 2
    }
    mkdir -p "$SCRATCH/${bin%/*}" || return 2
    cp "$host/$bin" "$SCRATCH/$bin" || return 2
}

# spec: gate-sdk/SPEC.md §Consumer smoke — csmoke_vendor_and_install: from the binary's source checkout and the kit roots (gate-sdk first) sets SCRATCH + CSMOKE_INSTALLED; returns 2 on an environment failure; the caller owns cleanup and every post-baseline assertion
csmoke_vendor_and_install() {   # $1 = the checkout whose native/ was built, $2.. = kit roots
    local host="$1" roots r kit
    shift
    roots=("$@")
    SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/consumer-smoke.XXXXXX")" || return 2

    git -C "$SCRATCH" init -q
    # spec: gate-sdk/SPEC.md §Consumer smoke — the placed binary is ignored rather than tracked: the violation phase restores with `git clean -fd`, which spares ignored paths, so an artifact the tree needs across restores may not be untracked-and-visible
    printf '.tmp/\n%s\n' "$(gate_native_bin)" > "$SCRATCH/.gitignore"
    git -C "$SCRATCH" add -A
    git -C "$SCRATCH" -c user.email=smoke@example.invalid -c user.name=smoke \
        commit -q --allow-empty -m "seed"

    for r in "${roots[@]}"; do
        cp -R "$r" "$SCRATCH/$(basename "$r")"
    done

    csmoke_place_binary "$host" "${roots[@]}" || return 2

    CSMOKE_INSTALLED=0
    for r in "${roots[@]}"; do
        kit="$(basename "$r")"
        if ! ( cd "$SCRATCH" && SMOKE_KIT_ROOT="$SCRATCH/$kit" bash "$SCRATCH/$kit/smoke/install.sh" ); then
            echo "csmoke: $kit/smoke/install.sh failed (a broken installer is an environment failure)" >&2
            return 2
        fi
        CSMOKE_INSTALLED=$((CSMOKE_INSTALLED + 1))
    done

    git -C "$SCRATCH" add -A
    git -C "$SCRATCH" -c user.email=smoke@example.invalid -c user.name=smoke \
        commit -q --no-verify -m "installed baseline"
}
