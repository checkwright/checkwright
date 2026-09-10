#!/usr/bin/env bash
# Hermetic fixture stand-in for the bash bootstrap's host detector, carrying arm E's forward defect
# and it is the attested one rather than a synthetic case: `aarch64-unknown-linux-gnu` is DETECTED
# WHILE THE PAGE DECLARES IT NOWHERE, which is how a triple came to be reachable by the installer
# with nothing going red. The converse — a declared triple no detector emits — is carried by the
# page rather than by this file.
target_of_host() {
    case "$(host_shape)" in
        Linux/x86_64)  printf 'x86_64-unknown-linux-gnu' ;;
        Linux/aarch64) printf 'aarch64-unknown-linux-gnu' ;;
        Darwin/arm64)  printf 'aarch64-apple-darwin' ;;
        Darwin/x86_64) printf 'x86_64-apple-darwin' ;;
        *) : ;;
    esac
}
