# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §Consumer smoke — the shared scratch-consumer builder both smoke harnesses vendor through (run-consumer-smoke.sh and context-kit/smoke/agents-md.sh)

# spec: gate-sdk/SPEC.md §Consumer smoke — csmoke_place_binary: the vendored kit roots' descriptor set decides whether the scratch consumer needs the gate binary, and the invoking repo's already-built artifact is what it receives. Derived, so a kit set with no ported gate asks for nothing.
csmoke_place_binary() {
    local host bin descriptors=0 r
    host="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
    bin="$(gate_native_bin)"

    shopt -s nullglob
    for r in "$@"; do
        for _ in "$r"/checks/*.gate; do descriptors=$((descriptors + 1)); done
    done
    shopt -u nullglob
    [[ "$descriptors" -gt 0 ]] || return 0

    [[ -x "$host/$bin" ]] || {
        echo "csmoke: $descriptors vendored .gate descriptor(s) need the gate binary, but $host/$bin is absent or not executable" >&2
        echo "  help: build it — cargo build --release --manifest-path native/Cargo.toml — then re-run." >&2
        return 2
    }
    mkdir -p "$SCRATCH/${bin%/*}" || return 2
    cp "$host/$bin" "$SCRATCH/$bin" || return 2
}

# spec: gate-sdk/SPEC.md §Consumer smoke — csmoke_vendor_and_install: from the kit roots (gate-sdk first) sets SCRATCH + CSMOKE_INSTALLED; returns 2 on an environment failure; the caller owns cleanup and every post-baseline assertion
csmoke_vendor_and_install() {
    local roots=("$@") r kit
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

    csmoke_place_binary "${roots[@]}" || return 2

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
