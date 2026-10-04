#!/bin/sh
# Every invariant is violated here, deliberately: this pin and this floor trail
# their twin's (invariant A), the pin trails the version line passed in args
# (invariant B), and it fetches a name the pinned declaration does not carry
# (invariant C). The page spells no asset name, the pinned doc declares the
# tarball without its sidecar, and its commit-cost figure names another version
# (invariant D). expect.txt pins each message, so dropping any arm fails the
# fixture.
checkwright_install() {
    pin='0.20.0'
    attest_from='0.20.1'
    cw_tgz="checkwright-$v.tar.gz"
    printf '%s\n' "$cw_tgz"
}

checkwright_install "$@"
