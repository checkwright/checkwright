#!/bin/sh
# spec: installer/SPEC.md §The install boundary — the POSIX sh bootstrap, twin of the PowerShell half,
# authored against that section's five steps: every install step is on the far side of the invoke
# no-port: installer/SPEC.md §The install boundary — this file's whole body is `bootstrap`-
# disposition steps, the one no-port cause installer/ may carry: the binary cannot select itself
#
# usage: checkwright <verb> [args...]
#   checkwright --help    list the verbs the verified artifact carries
set -u

# spec: installer/SPEC.md §Implementation — POSIX has no `local`, so every working name in this
# file is unique to the one function that uses it

die() {
    printf 'checkwright: %s\n' "$1" >&2
    [ -n "${2:-}" ] && printf '  help: %s\n' "$2" >&2
    exit "${3:-2}"
}

# spec: installer/SPEC.md §The install boundary — step 1: the package's own payload directory,
# resolved through the symlink chain, because npm installs the bin entry as a link in
# node_modules/.bin and the unresolved path's parent is node_modules
# spec: installer/SPEC.md §Implementation — a POSIX `cd` into a relative path consults CDPATH and
# prints the directory it picks, which would corrupt every `$(cd … && pwd)` below
unset CDPATH
SELF="$0"
while [ -L "$SELF" ]; do
    LINK_DIR="$(cd "$(dirname "$SELF")" && pwd)"
    SELF="$(readlink "$SELF")"
    case "$SELF" in
        /*) : ;;
        *) SELF="$LINK_DIR/$SELF" ;;
    esac
done
INSTALLER="$(cd "$(dirname "$SELF")/.." && pwd)"
PAYLOAD="$INSTALLER/payload"
[ -d "$PAYLOAD" ] || die "this package carries no payload" \
    "the bootstrap runs the gate binary out of the package's own payload/, assembled at pack time — run it from an installed package, not from a source checkout."

# spec: installer/SPEC.md §The gate binary — the detector's whole input, factored out so a refusal
# can name what the host was detected AS rather than only that it mapped to nothing
host_shape() {   # -> `<os>/<arch>` exactly as this host reports itself
    printf '%s/%s' "$(uname -s 2>/dev/null)" "$(uname -m 2>/dev/null)"
}

# spec: installer/SPEC.md §The gate binary — step 2: which published artifact fits this host. Two
# fields rather than `uname -a` because that is the smallest input answering the question and the
# one a PowerShell half can answer without parsing prose
# spec: installer/SPEC.md §The gate binary — every mapped triple here is the sole single-quoted
# operand of a `printf` and appears nowhere else in this function, which is the shape
# check-install-platforms extracts this detector's triple set from
target_of_host() {   # -> the Rust target triple this host is, empty when it maps to none
    case "$(host_shape)" in
        Linux/x86_64)               printf 'x86_64-unknown-linux-gnu' ;;
        Linux/aarch64|Linux/arm64)  printf 'aarch64-unknown-linux-gnu' ;;
        Darwin/x86_64)              printf 'x86_64-apple-darwin' ;;
        Darwin/arm64)               printf 'aarch64-apple-darwin' ;;
        # spec: installer/SPEC.md §The gate binary — the map answers which *published artifact*
        # fits this host, so a MinGW/MSYS/Cygwin `uname` — which reports the shell environment and
        # not the toolchain — maps to the msvc triple a Windows build leg would publish
        MINGW*/x86_64|MSYS*/x86_64|CYGWIN*/x86_64) printf 'x86_64-pc-windows-msvc' ;;
        *) : ;;
    esac
}

# spec: installer/SPEC.md §The gate binary — the libc question, answered beside the other refusals
# rather than inside the detector: two questions, two places, and each verdict rests on a POSITIVE
# signal, so an unidentifiable libc refuses rather than being read as glibc
libc_flavour() {   # -> musl | gnu | unknown
    for libc_ld in /lib/ld-musl-*; do
        [ -e "$libc_ld" ] && { printf 'musl'; return; }
    done
    if getconf GNU_LIBC_VERSION >/dev/null 2>&1 \
        || ldd --version 2>&1 | grep -qiE 'gnu libc|glibc'; then
        printf 'gnu'
        return
    fi
    printf 'unknown'
}

# spec: installer/SPEC.md §The gate binary — step 3: selection keeps three outcomes and only one
# of them proceeds, and they stay told apart by message and remedy rather than by exit status alone
# — an undeclared host and a broken payload remain different answers to an adopter
ARTIFACT=""
select_artifact() {
    sel_dir="$PAYLOAD/artifact"
    sel_target="$(target_of_host)"
    if [ ! -d "$sel_dir" ]; then
        die "this host, detected as $(host_shape), maps to no target this payload declares" \
            "the support roster is fixed at pack time and this platform is not on it, so there is nothing to verify or run here and no adopter action to take."
    fi
    sel_roster="$sel_dir/targets.list"
    [ -f "$sel_roster" ] || die "this payload carries prebuilt gate binaries but no target roster" \
        "the roster is copied verbatim beside them at pack time; artifacts without one cannot be selected from and the payload is broken, not narrower."
    # spec: installer/SPEC.md §The gate binary — the one case where the roster grep cannot refuse
    # for us: `uname` cannot tell glibc from musl, so a musl host resolves to a triple that IS on
    # the roster and would be handed a binary that dies in the dynamic loader
    case "$sel_target" in
        *-linux-gnu)
            case "$(libc_flavour)" in
                gnu) : ;;
                musl)
                    die "this host, detected as $(host_shape), runs a musl C library, and every Linux artifact this payload carries is linked against glibc" \
                        "musl and glibc are not interchangeable at the dynamic loader, so a glibc build would die there rather than run. This payload carries no musl artifact, so there is no adopter action to take."
                    ;;
                *)
                    die "this host, detected as $(host_shape), did not identify its C library, and every Linux artifact this payload carries is linked against glibc" \
                        "neither 'getconf GNU_LIBC_VERSION' nor 'ldd --version' identified a GNU libc here, and no musl loader was found under /lib — so nothing establishes that a glibc build would run, and this refuses rather than handing you one that may die in the loader. Install GNU libc's getconf or ldd so the probe can answer."
                    ;;
            esac
            ;;
    esac
    if [ -z "$sel_target" ] || ! grep -Ev '^[[:space:]]*(#|$)' "$sel_roster" | grep -qxF "$sel_target"; then
        die "this host, detected as $(host_shape), maps to no target this payload declares" \
            "the support roster is fixed at pack time and this platform is not on it, so there is nothing to verify or run here and no adopter action to take."
    fi
    sel_src="$sel_dir/$sel_target"
    # spec: installer/SPEC.md §The gate binary — a glob rather than `find`, so no primary can
    # disagree between GNU and BSD; it skips dotfiles, so a stray one is not a second artifact
    sel_count=0
    sel_name=""
    for sel_path in "$sel_src"/*; do
        [ -f "$sel_path" ] || continue
        case "$sel_path" in *.sha256) continue ;; esac
        sel_count=$((sel_count + 1))
        sel_name="${sel_path##*/}"
    done
    [ "$sel_count" -eq 1 ] && [ -f "$sel_src/$sel_name.sha256" ] \
        || die "the payload declares $sel_target but carries no complete artifact for it" \
           "a declared target whose binary or .sha256 sidecar is missing is a publisher defect you cannot act on — refusing rather than running a battery that silently shrank." 1
    ARTIFACT="$sel_src/$sel_name"
}

# spec: installer/SPEC.md §The install boundary — step 4, the one step that cannot use the binary
# to verify the binary: a host carrying neither `sha256sum` nor `shasum` is refused rather than
# served an unverified artifact. Behind the invoke the crate hashes in-process
verify_digest() {
    if command -v sha256sum >/dev/null 2>&1; then
        dig_hasher=sha256sum
    elif command -v shasum >/dev/null 2>&1; then
        dig_hasher=shasum
    else
        die "no SHA-256 hasher on this host, and nothing unverified is ever executed" \
            "install sha256sum (GNU coreutils) or shasum, then re-run. The bootstrap verifies the published digest before it runs the artifact, and there is no path here that skips that." 1
    fi
    dig_want=""
    dig_got=""
    read -r dig_want _ < "$ARTIFACT.sha256" || true
    case "$dig_hasher" in
        sha256sum) dig_got="$(sha256sum -- "$ARTIFACT" 2>/dev/null | cut -d' ' -f1)" ;;
        shasum)    dig_got="$(shasum -a 256 -- "$ARTIFACT" 2>/dev/null | cut -d' ' -f1)" ;;
    esac
    [ -n "$dig_want" ] && [ "$dig_want" = "$dig_got" ] \
        || die "the prebuilt gate binary does not match its published digest" \
           "nothing unverified is ever executed, so this refuses rather than warning. Re-download the package; a persistent mismatch means the artifact was altered after it was built." 1
}

select_artifact
verify_digest

# spec: installer/SPEC.md §The install boundary — step 5: execute the verified artifact in place
# out of the payload, under one unconditional argv rule — a dashless leading token is prefixed with
# `--` and everything after it is forwarded verbatim
# spec: installer/SPEC.md §The verbs — the rule introduces no verb table into the bootstrap, which
# is what the interpreter policy forbids here; the artifact owns which verbs exist
if [ $# -gt 0 ]; then
    case "$1" in
        -*) : ;;
        *)
            VERB="--$1"
            shift
            exec "$ARTIFACT" "$VERB" "$@"
            ;;
    esac
fi
exec "$ARTIFACT" "$@"
