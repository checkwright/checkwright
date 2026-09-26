#!/bin/sh
# spec: installer/SPEC.md §The dependency boundary — the one-line install: docs/install.md's recipe for one pinned release, and nothing else
# spec: installer/SPEC.md §The hosted install pin — `pin` is the only version this script names, held to docs/install.ps1 and to the newest tag by check-install-pin
#
# usage: curl -fsSL https://checkwright.dev/install.sh | sh -s -- [verb] [args...]
#   CHECKWRIGHT_VERSION       the release to install (default: the pin below)
#   CHECKWRIGHT_RELEASE_BASE  the download base (default: this project's GitHub Releases)

# spec: installer/SPEC.md §The dependency boundary — the whole body is one function called on the last line, so a truncated fetch defines nothing and runs nothing
checkwright_install() {
    pin='0.26.0'
    set -u
    cw_version="${CHECKWRIGHT_VERSION:-$pin}"
    cw_base="${CHECKWRIGHT_RELEASE_BASE:-https://github.com/checkwright/checkwright/releases/download}"
    cw_tgz="checkwright-$cw_version.tgz"

    cw_dir="$(mktemp -d)" || {
        printf 'checkwright install: could not make a temporary directory\n' >&2
        exit 2
    }
    trap 'rm -rf "$cw_dir"' EXIT
    trap 'exit 130' HUP INT TERM

    printf 'checkwright install: fetching %s from %s\n' "$cw_tgz" "$cw_base" >&2
    for cw_file in "$cw_tgz" "$cw_tgz.sha256"; do
        curl -fsSLo "$cw_dir/$cw_file" "$cw_base/v$cw_version/$cw_file" || {
            printf 'checkwright install: download failed: %s/v%s/%s\n' "$cw_base" "$cw_version" "$cw_file" >&2
            exit 2
        }
    done

    if command -v sha256sum >/dev/null 2>&1; then
        (cd "$cw_dir" && sha256sum -c "$cw_tgz.sha256")
    elif command -v shasum >/dev/null 2>&1; then
        (cd "$cw_dir" && shasum -a 256 -c "$cw_tgz.sha256")
    else
        printf 'checkwright install: verify failed: neither sha256sum nor shasum is on PATH\n' >&2
        exit 2
    fi || {
        printf 'checkwright install: verify failed: %s does not match its published digest\n' "$cw_tgz" >&2
        exit 2
    }

    (cd "$cw_dir" && tar -xzf "$cw_tgz") || {
        printf 'checkwright install: extract failed: %s\n' "$cw_tgz" >&2
        exit 2
    }

    [ "$#" -gt 0 ] || set -- init
    sh "$cw_dir/package/bin/checkwright.sh" "$@"
    exit "$?"
}

checkwright_install "$@"
