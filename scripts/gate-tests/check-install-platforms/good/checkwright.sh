#!/usr/bin/env bash
# Hermetic fixture stand-in for the bash bootstrap's host detector. It carries the pinned shape and
# nothing else: inside `target_of_host`'s body every mapped triple is the sole single-quoted operand
# of a `printf`, and the emitted set equals what this case's install.md declares — joined and held
# alike, which is the four-way agreement arm E passes on.
target_of_host() {
    case "$(host_shape)" in
        Linux/x86_64) printf 'x86_64-unknown-linux-gnu' ;;
        Darwin/arm64) printf 'aarch64-apple-darwin' ;;
        *) : ;;
    esac
}
