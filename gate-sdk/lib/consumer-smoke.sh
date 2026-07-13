# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §Consumer smoke — the shared scratch-consumer builder both smoke harnesses vendor through (run-consumer-smoke.sh and context-kit/smoke/agents-md.sh)

# spec: gate-sdk/SPEC.md §Consumer smoke — csmoke_vendor_and_install: from the kit roots (gate-sdk first) sets SCRATCH + CSMOKE_INSTALLED; returns 2 on an environment failure; the caller owns cleanup and every post-baseline assertion
csmoke_vendor_and_install() {
    local roots=("$@") r kit
    SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/consumer-smoke.XXXXXX")" || return 2

    git -C "$SCRATCH" init -q
    printf '.tmp/\n' > "$SCRATCH/.gitignore"
    git -C "$SCRATCH" add -A
    git -C "$SCRATCH" -c user.email=smoke@example.invalid -c user.name=smoke \
        commit -q --allow-empty -m "seed"

    for r in "${roots[@]}"; do
        cp -R "$r" "$SCRATCH/$(basename "$r")"
    done

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
