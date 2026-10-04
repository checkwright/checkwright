#!/bin/sh
# spec: installer/SPEC.md §The dependency boundary — the one-line install: docs/install.md's recipe for one pinned release, and nothing else
# spec: installer/SPEC.md §The hosted install pin — `pin` is the only version this script installs and `attest_from` its attestation floor, both held to docs/install.ps1 by check-install-pin, the pin to the newest tag too
# no-port: installer/SPEC.md §The install boundary — this file's whole body is `bootstrap`-disposition steps: it fetches, verifies and unpacks the first artifact and hands off to its bootstrap, which the binary cannot do for itself
#
# usage: curl -fsSL https://checkwright.dev/install.sh | sh -s -- [verb] [args...]
#   CHECKWRIGHT_VERSION       the release to install (default: the pin below)
#   CHECKWRIGHT_RELEASE_BASE  the download base (default: this project's GitHub Releases)

# spec: installer/SPEC.md §The dependency boundary — the whole body is one function called on the last line, so a truncated fetch defines nothing and runs nothing
checkwright_install() {
    pin='0.31.0'
    attest_from='0.31.1'
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

    # spec: installer/SPEC.md §The dependency boundary — the attestation check: the floor over dotted digit runs, a version that does not parse compared as attested, then the three-part presence test and the four outcomes
    cw_attested=1
    case "$cw_version" in
        '' | .* | *. | *..* | *[!0-9.]*) ;;
        *)
            cw_v="$cw_version." cw_f="$attest_from."
            while [ -n "$cw_v$cw_f" ]; do
                cw_a="0${cw_v%%.*}" cw_b="0${cw_f%%.*}"
                cw_v="${cw_v#*.}" cw_f="${cw_f#*.}"
                if [ "$cw_a" -lt "$cw_b" ]; then cw_attested=0; break; fi
                if [ "$cw_a" -gt "$cw_b" ]; then break; fi
            done
            ;;
    esac
    if [ "$cw_attested" = 0 ]; then
        printf 'checkwright install: v%s predates build attestation; checked against its digest only\n' "$cw_version" >&2
    elif command -v gh >/dev/null 2>&1 \
        && gh attestation verify --help >/dev/null 2>&1 \
        && gh auth status --hostname github.com >/dev/null 2>&1; then
        cw_gh="$(gh attestation verify "$cw_dir/$cw_tgz" --repo checkwright/checkwright \
            --signer-workflow checkwright/checkwright/.github/workflows/publish.yml 2>&1)" || {
            printf '%s\n' "$cw_gh" >&2
            printf "checkwright install: verify failed: %s carries no build attestation from checkwright/checkwright's publish workflow\n" "$cw_tgz" >&2
            exit 2
        }
        printf 'checkwright install: build attestation verified\n' >&2
    else
        printf 'checkwright install: build attestation not checked (gh is not installed or not signed in); to check it: gh attestation verify %s --repo checkwright/checkwright\n' "$cw_tgz" >&2
    fi

    (cd "$cw_dir" && tar -xzf "$cw_tgz") || {
        printf 'checkwright install: extract failed: %s\n' "$cw_tgz" >&2
        exit 2
    }

    [ "$#" -gt 0 ] || set -- init
    sh "$cw_dir/package/bin/checkwright.sh" "$@"
    exit "$?"
}

checkwright_install "$@"
