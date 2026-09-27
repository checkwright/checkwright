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

# spec: gate-sdk/SPEC.md §lib/gate.sh — the byte twin of the library's gate_path_rooted, held to it by a crate unit test: this bootstrap cannot source a payload file before it has located the payload
gate_path_rooted() {  # <path> — 0 when rooted in either dialect, walk::path_root's predicate
    case "$1" in
        /* | \\*) return 0 ;;
        [ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz]:/*) return 0 ;;
        [ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz]:\\*) return 0 ;;
    esac
    return 1
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
    gate_path_rooted "$SELF" || SELF="$LINK_DIR/$SELF"
done
INSTALLER="$(cd "$(dirname "$SELF")/.." && pwd)"
PAYLOAD="$INSTALLER/payload"
[ -d "$PAYLOAD" ] || die "this package carries no payload" \
    "the bootstrap runs the gate binary out of the package's own payload/, assembled at pack time; run it from an installed package, not from a source checkout."

# spec: installer/SPEC.md §The gate binary — the detector's whole input, factored out so a refusal
# can name what the host was detected AS rather than only that it mapped to nothing
host_shape() {   # -> `<os>/<arch>` as this host reports itself
    printf '%s/%s' "$(uname -s 2>/dev/null)" "$(host_arch)"
}

# spec: installer/SPEC.md §The gate binary — on the Windows family `uname -m` names the emulated
# POSIX runtime's architecture, so there the OS's own is read from the registry
host_arch() {   # -> this host's architecture, in `uname -m`'s vocabulary
    case "$(uname -s 2>/dev/null)" in
        MINGW*|MSYS*|CYGWIN*)
            case "$(tr -d '\000' 2>/dev/null < '/proc/registry/HKEY_LOCAL_MACHINE/SYSTEM/CurrentControlSet/Control/Session Manager/Environment/PROCESSOR_ARCHITECTURE')" in
                ARM64) printf 'aarch64'; return ;;
            esac ;;
    esac
    uname -m 2>/dev/null
}

# spec: installer/SPEC.md §Platform resolution — step 2: which published artifact fits this host. Two
# fields rather than `uname -a` because that is the smallest input answering the question and the
# one a PowerShell half can answer without parsing prose
# spec: installer/SPEC.md §Platform resolution — every mapped triple here is the sole single-quoted
# operand of a `printf` and appears nowhere else in this function, which is the shape
# check-install-platforms extracts this detector's triple set from
target_of_host() {   # -> the preferred Rust target triple for this host, empty when it maps to none
    case "$(host_shape)" in
        Linux/x86_64)               printf 'x86_64-unknown-linux-gnu' ;;
        Linux/aarch64|Linux/arm64)  printf 'aarch64-unknown-linux-gnu' ;;
        Darwin/x86_64)              printf 'x86_64-apple-darwin' ;;
        Darwin/arm64)               printf 'aarch64-apple-darwin' ;;
        # spec: installer/SPEC.md §The gate binary — the map answers which *published artifact*
        # fits this host, so a MinGW/MSYS/Cygwin `uname` — which reports the shell environment and
        # not the toolchain — maps to the msvc triple a Windows build leg would publish
        MINGW*/x86_64|MSYS*/x86_64|CYGWIN*/x86_64) printf 'x86_64-pc-windows-msvc' ;;
        MINGW*/aarch64|MSYS*/aarch64|CYGWIN*/aarch64) printf 'aarch64-pc-windows-msvc' ;;
        *) : ;;
    esac
}

# spec: installer/SPEC.md §Platform resolution — the loader answers the libc question: a preferred
# triple's fallback is taken when its verified artifact does not run. Each fallback is the sole
# single-quoted operand of a `printf`, the shape check-install-platforms extracts
fallback_of_target() {   # <triple> -> the triple selection falls back to, empty when it has none
    case "$1" in
        x86_64-unknown-linux-gnu)   printf 'x86_64-unknown-linux-musl' ;;
        aarch64-unknown-linux-gnu)  printf 'aarch64-unknown-linux-musl' ;;
        *) : ;;
    esac
}

UNROSTERED_HELP="the support roster is fixed at pack time and this platform is not on it, so there is nothing to verify or run here and no adopter action to take."

# spec: installer/SPEC.md §Selection — 0 when the payload's roster carries the triple
rostered() {   # <triple>
    [ -n "$1" ] && grep -Ev '^[[:space:]]*(#|$)' "$PAYLOAD/artifact/targets.list" | grep -qxF "$1"
}

# spec: installer/SPEC.md §Selection — step 3: the outcomes stay told apart by message and remedy
# rather than by exit status alone — an undeclared host and a broken payload remain different
# answers to an adopter
ARTIFACT=""
resolve_pair() {   # <triple> -> sets ARTIFACT to the rostered triple's one binary, or refuses
    sel_target="$1"
    sel_src="$PAYLOAD/artifact/$sel_target"
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
           "a declared target whose binary or .sha256 sidecar is missing is a publisher defect you cannot act on; refusing rather than running a battery that silently shrank." 1
    ARTIFACT="$sel_src/$sel_name"
}

# spec: installer/SPEC.md §Selection — the preferred candidate first; a candidate the roster lacks,
# or a verified one that does not run, passes to its fallback, and no refusal does
select_artifact() {
    [ -d "$PAYLOAD/artifact" ] \
        || die "this host, detected as $(host_shape), maps to no target this payload declares" "$UNROSTERED_HELP"
    [ -f "$PAYLOAD/artifact/targets.list" ] || die "this payload carries prebuilt gate binaries but no target roster" \
        "the roster is copied verbatim beside them at pack time; artifacts without one cannot be selected from and the payload is broken, not narrower."
    cand_target="$(target_of_host)"
    cand_fallback="$(fallback_of_target "$cand_target")"
    if ! rostered "$cand_target"; then
        rostered "$cand_fallback" \
            || die "this host, detected as $(host_shape), maps to no target this payload declares" "$UNROSTERED_HELP"
        cand_target="$cand_fallback"
        cand_fallback=""
    fi
    resolve_pair "$cand_target"
    verify_digest
    [ -n "$cand_fallback" ] || return 0
    # spec: installer/SPEC.md §Selection — step 4's probe, run only on a verified candidate that has
    # a fallback: a glibc below the artifact's floor or a host with no glibc loader fails it
    "$ARTIFACT" --help >/dev/null 2>&1 && return 0
    rostered "$cand_fallback" \
        || die "the $cand_target gate binary does not run on this host, and this payload carries no fallback for it" \
               "no adopter action; report the host, the triple and the release."
    resolve_pair "$cand_fallback"
    verify_digest
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
