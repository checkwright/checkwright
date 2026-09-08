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
binary="$(gate_native_bin)"
binary="${binary##*/}"

rustup target add "$target"
bash gate-sdk/bin/build-native.sh --target "$target"

# spec: gate-sdk/SPEC.md §Consumer payload — the one digest computation for this artifact anywhere in the run or the release; every later hop moves the file, so a published digest and an installed digest cannot diverge
# spec: gate-sdk/SPEC.md §Consumer payload — the hasher is resolved rather than named: stock macOS ships `shasum` and no `sha256sum`, and both spell the `<hex>  <bare name>` sidecar `sha256sum -c` reads wherever a later reader puts the two files
out="$outdir/$target"
mkdir -p "$out"
cp "$crate/target/$target/release/$binary" "$out/$binary"
if command -v sha256sum >/dev/null 2>&1; then
    ( cd "$out" && sha256sum "$binary" > "$binary.sha256" )
else
    ( cd "$out" && shasum -a 256 "$binary" > "$binary.sha256" )
fi
echo "built $target: $(cat "$out/$binary.sha256")"
