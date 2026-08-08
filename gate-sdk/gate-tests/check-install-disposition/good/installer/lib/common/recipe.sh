# shellcheck shell=bash
# Fixture stand-in for the installer's recipe module: the roster is derived from
# each gate's declared disposition, so no gate name is written here.
recipe_gates() {   # $1 = kit payload dir, $2 = profile
    local pay="$1" f
    for f in "$pay"/checks/check-*.sh; do
        [[ -e "$f" ]] || continue
        printf '%s\n' "$f"
    done
}
