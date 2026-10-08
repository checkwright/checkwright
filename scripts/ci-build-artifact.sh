#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer payload — one artifact body for the release build leg and the CI producer, so what CI exercises is what a release publishes by construction rather than by inspection
# no-port: ruled 2026-09-08 by the operator, asked and answered in a `/lead` session and lead-relayed. This body IS the build of the gate binary, so a ported arm would be a subcommand of the artifact it produces and would have to exist before it was built. That is the contributor-side irreducible gate-sdk/bin/build-native.sh's own disposition records, met here on the release path instead of on a fresh clone; this script is that script's caller and inherits the constraint rather than restating it. The cause is this file's and never a class: nothing else may cite it, and a second builder is a new file with its own disposition.
# usage: ci-build-artifact.sh <target-triple> <output-dir>
#   Run from the repo root. Reads no CI variable, so a developer machine runs it
#   unchanged — which is how this body is verified before it reaches a tag.
set -euo pipefail

usage() {
    printf 'usage: %s <target-triple> <output-dir>\n' "${0##*/}"
    printf '  <target-triple>  the rustc target to add and build for\n'
    printf '  <output-dir>     receives <output-dir>/<target-triple>/ with the binary and its .sha256\n'
}

case "${1:-}" in
    -h|--help) usage; exit 0 ;;
    --) shift ;;
esac
if [ "$#" -ne 2 ]; then
    usage >&2
    exit 2
fi
case "$1" in
    -*) usage >&2; exit 2 ;;
esac
target="$1"
outdir="$2"

# shellcheck source=../gate-sdk/lib/gate.sh
source gate-sdk/lib/gate.sh

crate="$(gate_native_crate)"
# spec: gate-sdk/SPEC.md §Consumer payload — the executable suffix is the TARGET's and never the host's, the same derivation gate-sdk/bin/build-native.sh takes from the one owner, while the --pack-installer arm discovers the name it wrote: this job's runner is that target's own platform today, so a host-derived suffix agrees by coincidence rather than by construction, and the day one caller passes a triple the host is not, the copy below looks for a name cargo never emitted
binary="$(gate_native_bin)"
binary="${binary##*/}"
binary="${binary%.exe}$(gate_exe_suffix "$target")"

# spec: gate-sdk/SPEC.md §Consumer payload — a glibc artifact's floor is a version need the loader must refuse below, and the linker decides whether that need is hard: one whose every reference is weak is flagged weak by a newer linker, and the loader then warns and runs. The runner label pins the C library and not the Rust toolchain the image carries (native/runners.list), so the glibc targets build on a named toolchain; move it as one deliberate edit, whose push is its witness.
case "$target" in
    *-unknown-linux-gnu)
        export RUSTUP_TOOLCHAIN=1.98.1
        rustup toolchain install "$RUSTUP_TOOLCHAIN" --profile minimal --no-self-update
        ;;
esac
rustup target add "$target"
bash gate-sdk/bin/build-native.sh --target "$target"

# spec: gate-sdk/SPEC.md §Consumer payload — the one digest computation for this artifact anywhere in the run or the release; every later hop moves the file, so a published digest and an installed digest cannot diverge
# spec: gate-sdk/SPEC.md §Consumer payload — the hasher is resolved rather than named: stock macOS ships `shasum` and no `sha256sum`, and both spell the `<hex>  <bare name>` sidecar `sha256sum -c` reads wherever a later reader puts the two files
out="$outdir/$target"
mkdir -p "$out"
cp "$crate/target/$target/release/$binary" "$out/$binary"

# spec: gate-sdk/SPEC.md §Consumer payload — the artifact's OS floor is measured and held equal to the floor the platforms table declares for its target, before any digest exists, so a refused artifact never gets a sidecar
platforms_page=docs/requirements.md

floor_refuse() {
    printf 'floor %s: %s\n' "$target" "$1" >&2
    exit 1
}

# spec: docs/install-parity-contracts.md §The install-platforms parity contract — the row whose first backticked run is the target, and the declared floor token in it
row="$(awk -v t="$target" '
    /^<!-- platforms:begin -->$/ { inb = 1; next }
    /^<!-- platforms:end -->$/   { inb = 0; next }
    inb && /^[ \t]*\|/ && !done && match($0, /`[^`]+`/) {
        if (substr($0, RSTART + 1, RLENGTH - 2) == t) { print; done = 1 }
    }
' "$platforms_page")"
[ -n "$row" ] || floor_refuse "$platforms_page's platforms table has no row for this target, so a built target exceeds what the page declares"

floor_token() {
    printf '%s\n' "$row" | awk -v lead="$1 " '{
        if (match($0, lead "[0-9][0-9.]*")) print substr($0, RSTART + length(lead), RLENGTH - length(lead))
    }'
}

# spec: gate-sdk/SPEC.md §Consumer payload — compared as numbers with trailing zero components dropped, so 11 equals 11.0
floor_cmp_awk='function cmp(a, b,   x, y, n, m, k, i) {
    n = split(a, x, "."); m = split(b, y, "."); k = n > m ? n : m
    for (i = 1; i <= k; i++) if (x[i] + 0 != y[i] + 0) return (x[i] + 0 < y[i] + 0) ? -1 : 1
    return 0
}'

case "$target" in
    *-unknown-linux-gnu)  floor_lead=glibc floor_tool=readelf ;;
    *-unknown-linux-musl) floor_lead=Linux floor_tool=static ;;
    *-apple-darwin)       floor_lead=macOS floor_tool=otool ;;
    *)                    floor_lead='' floor_tool='' ;;
esac
if [ -z "$floor_tool" ]; then
    echo "floor $target: declared only, measured nothing"
elif [ "$floor_tool" = static ]; then
    declared="$(floor_token "$floor_lead")"
    [ -n "$declared" ] || floor_refuse "its $platforms_page row states no '$floor_lead <version>' Minimum, so there is nothing to hold the artifact to"
    command -v readelf >/dev/null 2>&1 || floor_refuse "'readelf' is not on PATH, so the artifact's linkage cannot be measured"
    # spec: installer/SPEC.md §Requirements — a Linux artifact that links anything dynamically is refused
    dynamic="$(readelf -d "$out/$binary")" || floor_refuse "'readelf -d' could not read the artifact"
    headers="$(readelf -l "$out/$binary")" || floor_refuse "'readelf -l' could not read the artifact"
    case "$dynamic" in
        *'(NEEDED)'*) floor_refuse "the artifact lists a NEEDED shared library, so it is not statically linked and would need that library on every Linux host" ;;
    esac
    case "$headers" in
        *INTERP*) floor_refuse "the artifact requests a program interpreter, so it is not statically linked and would need that loader on every Linux host" ;;
    esac
    echo "floor $target: static, no loader or shared library needed; $floor_lead $declared declared, not measured"
else
    declared="$(floor_token "$floor_lead")"
    [ -n "$declared" ] || floor_refuse "its $platforms_page row states no '$floor_lead <version>' Minimum, so there is nothing to hold the artifact to"
    command -v "$floor_tool" >/dev/null 2>&1 || floor_refuse "'$floor_tool' is not on PATH, so the artifact's floor cannot be measured"
    if [ "$floor_tool" = readelf ]; then
        # spec: gate-sdk/SPEC.md §Consumer payload — a need flagged weak is no floor, since the loader runs the artifact without it, so only the hard needs are measured
        measured="$(readelf -V --wide "$out/$binary" | awk "$floor_cmp_awk"'
            /^Version needs section/ { inb = 1; next }
            /^Version .* section/    { inb = 0 }
            inb && !/Flags: WEAK/ {
                for (i = 1; i <= NF; i++) if ($i ~ /^GLIBC_[0-9]/) {
                    v = substr($i, 7)
                    if (best == "" || cmp(v, best) > 0) best = v
                }
            }
            END { print best }
        ')"
    else
        measured="$(otool -l "$out/$binary" | awk '
            $1 == "cmd" { mode = $2 }
            found == "" && mode == "LC_BUILD_VERSION" && $1 == "minos" { found = $2 }
            found == "" && mode == "LC_VERSION_MIN_MACOSX" && $1 == "version" { found = $2 }
            END { print found }
        ')"
    fi
    [ -n "$measured" ] || floor_refuse "'$floor_tool' found no $floor_lead floor in the artifact, and the step never passes unmeasured"
    if ! awk -v a="$measured" -v b="$declared" "$floor_cmp_awk"' BEGIN { exit cmp(a, b) != 0 }'; then
        floor_refuse "the artifact needs $floor_lead $measured while $platforms_page declares $floor_lead $declared.
  Pin this target's runner in native/runners.list to an image with the older library, or
  raise the row's Minimum cell to $floor_lead $measured, a support narrowing the release
  declaration surface declares under Behavior changes. A glibc measure below the
  declared floor is a need the linker flagged weak: hold the toolchain this body names."
    fi
    echo "floor $target: $floor_lead $measured measured, equal to the declared $floor_lead $declared"
fi

if command -v sha256sum >/dev/null 2>&1; then
    ( cd "$out" && sha256sum "$binary" > "$binary.sha256" )
else
    ( cd "$out" && shasum -a 256 "$binary" > "$binary.sha256" )
fi
echo "built $target: $(cat "$out/$binary.sha256")"
