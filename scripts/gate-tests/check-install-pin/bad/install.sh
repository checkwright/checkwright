#!/bin/sh
# Both invariants are violated here, deliberately: this pin trails its twin's
# (invariant A) and the version line passed in args (invariant B). expect.txt
# pins both messages, so dropping either arm fails the fixture.
checkwright_install() {
    pin='0.20.0'
    printf '%s\n' "$pin"
}

checkwright_install "$@"
