#!/bin/sh
checkwright_install() {
    pin='0.21.0'
    cw_tgz="checkwright-$pin.tgz"
    for cw_file in "$cw_tgz" "$cw_tgz.sha256"; do
        printf '%s\n' "$cw_file"
    done
}

checkwright_install "$@"
