#!/usr/bin/env bash
# spec: installer/README.md §The install boundary — the bash bootstrap, twin of the PowerShell half,
# authored against that section's five steps: every install step is on the far side of the invoke
# no-port: installer/README.md §The install boundary — this file's whole body is `bootstrap`-
# disposition steps, the one no-port cause installer/ may carry: the binary cannot select itself
#
# usage: checkwright <verb> [args...]
#   checkwright --help    list the verbs the verified artifact carries
set -uo pipefail

die() {
    printf 'checkwright: %s\n' "$1" >&2
    [[ -n "${2:-}" ]] && printf '  help: %s\n' "$2" >&2
    exit "${3:-2}"
}

# spec: installer/README.md §The install boundary — step 1: the package's own payload directory,
# resolved through the symlink chain, because npm installs the bin entry as a link in
# node_modules/.bin and the unresolved path's parent is node_modules
SELF="${BASH_SOURCE[0]}"
while [[ -L "$SELF" ]]; do
    LINK_DIR="$(cd "$(dirname "$SELF")" && pwd)"
    SELF="$(readlink "$SELF")"
    [[ "$SELF" == /* ]] || SELF="$LINK_DIR/$SELF"
done
INSTALLER="$(cd "$(dirname "$SELF")/.." && pwd)"
PAYLOAD="$INSTALLER/payload"
[[ -d "$PAYLOAD" ]] || die "this package carries no payload" \
    "the bootstrap runs the gate binary out of the package's own payload/, assembled at pack time — run it from an installed package, not from a source checkout."

# spec: installer/README.md §The gate binary — step 2: which published artifact fits this host. Two
# fields rather than `uname -a` because that is the smallest input answering the question and the
# one a PowerShell half can answer without parsing prose
target_of_host() {   # -> the Rust target triple this host is, empty when it maps to none
    case "$(uname -s 2>/dev/null)/$(uname -m 2>/dev/null)" in
        Linux/x86_64)               printf 'x86_64-unknown-linux-gnu' ;;
        Linux/aarch64|Linux/arm64)  printf 'aarch64-unknown-linux-gnu' ;;
        Darwin/x86_64)              printf 'x86_64-apple-darwin' ;;
        Darwin/arm64)               printf 'aarch64-apple-darwin' ;;
        # spec: installer/README.md §The gate binary — the map answers which *published artifact*
        # fits this host, so a MinGW/MSYS/Cygwin `uname` — which reports the shell environment and
        # not the toolchain — maps to the msvc triple a Windows build leg would publish
        MINGW*/x86_64|MSYS*/x86_64|CYGWIN*/x86_64) printf 'x86_64-pc-windows-msvc' ;;
        *) : ;;
    esac
}

# spec: installer/README.md §The gate binary — step 3: selection keeps three outcomes and only one
# of them proceeds, and they stay told apart by message and remedy rather than by exit status alone
# — an undeclared host and a broken payload remain different answers to an adopter
ARTIFACT=""
select_artifact() {
    local dir roster target src n
    local -a names
    dir="$PAYLOAD/artifact"
    target="$(target_of_host)"
    if [[ ! -d "$dir" ]]; then
        die "this host maps to no target this payload declares" \
            "the support roster is fixed at pack time and this platform is not on it, so there is nothing to verify or run here and no adopter action to take."
    fi
    roster="$dir/targets.list"
    [[ -f "$roster" ]] || die "this payload carries prebuilt gate binaries but no target roster" \
        "the roster is copied verbatim beside them at pack time; artifacts without one cannot be selected from and the payload is broken, not narrower."
    if [[ -z "$target" ]] || ! grep -Ev '^[[:space:]]*(#|$)' "$roster" | grep -qxF "$target"; then
        die "this host maps to no target this payload declares" \
            "the support roster is fixed at pack time and this platform is not on it, so there is nothing to verify or run here and no adopter action to take."
    fi
    src="$dir/$target"
    # spec: installer/README.md §The gate binary — the prefix is stripped here rather than with a
    # `-printf '%P'` primary: that primary is GNU findutils, a BSD `find` refuses it, and the refusal
    # lands in the same empty result a genuinely incomplete artifact does
    names=()
    while IFS= read -r n; do
        n="${n#"$src/"}"
        [[ -n "$n" ]] && names+=("$n")
    done < <(find "$src" -maxdepth 1 -type f ! -name '*.sha256' | sort)
    [[ ${#names[@]} -eq 1 && -f "$src/${names[0]}.sha256" ]] \
        || die "the payload declares $target but carries no complete artifact for it" \
           "a declared target whose binary or .sha256 sidecar is missing is a publisher defect you cannot act on — refusing rather than running a battery that silently shrank." 1
    ARTIFACT="$src/${names[0]}"
}

# spec: installer/README.md §The install boundary — step 4, the one step that cannot use the binary
# to verify the binary: a host carrying neither `sha256sum` nor `shasum` is refused rather than
# served an unverified artifact. Behind the invoke the crate hashes in-process
verify_digest() {
    local hasher want got
    if command -v sha256sum >/dev/null 2>&1; then
        hasher=sha256sum
    elif command -v shasum >/dev/null 2>&1; then
        hasher=shasum
    else
        die "no SHA-256 hasher on this host, and nothing unverified is ever executed" \
            "install sha256sum (GNU coreutils) or shasum, then re-run. The bootstrap verifies the published digest before it runs the artifact, and there is no path here that skips that." 1
    fi
    want="$(awk 'NR==1{print $1}' "$ARTIFACT.sha256")"
    case "$hasher" in
        sha256sum) got="$(sha256sum -- "$ARTIFACT" 2>/dev/null | cut -d' ' -f1)" ;;
        shasum)    got="$(shasum -a 256 -- "$ARTIFACT" 2>/dev/null | cut -d' ' -f1)" ;;
    esac
    [[ -n "$want" && "$want" == "$got" ]] \
        || die "the prebuilt gate binary does not match its published digest" \
           "nothing unverified is ever executed, so this refuses rather than warning. Re-download the package; a persistent mismatch means the artifact was altered after it was built." 1
}

select_artifact
verify_digest

# spec: installer/README.md §The install boundary — step 5: execute the verified artifact in place
# out of the payload, under one unconditional argv rule — a dashless leading token is prefixed with
# `--` and everything after it is forwarded verbatim
# spec: installer/README.md §The verbs — the rule introduces no verb table into the bootstrap, which
# is what the interpreter policy forbids here; the artifact owns which verbs exist
if [[ $# -gt 0 && "$1" != -* ]]; then
    VERB="--$1"
    shift
    exec "$ARTIFACT" "$VERB" "$@"
fi
exec "$ARTIFACT" "$@"
