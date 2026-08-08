# shellcheck shell=bash
# Fixture stand-in carrying the second copy the derivation replaced: a literal
# per-kit roster in the installer, which a kit adding a gate can never update.
recipe_gates() {   # $1 = kit payload dir, $2 = profile
    case "${1##*/}" in
        alpha-kit) printf '%s\n' check-alpha check-delta ;;
        *) : ;;
    esac
}
