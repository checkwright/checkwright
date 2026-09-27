#!/usr/bin/env bash
# spec: companion/SPEC.md §Packing the extension — the one body that turns the tracked extension directory into its Release asset, run by the release pack job and by the toolkit leg
# usage: ci-pack-extension.sh <version> <out-dir>
#   Run from anywhere inside the checkout. Reads no CI variable, so a developer
#   machine runs it unchanged.
set -euo pipefail

usage() {
    printf 'usage: %s <version> <out-dir>\n' "${0##*/}"
    printf '  <version>  written into the packed extension.yml and the asset name\n'
    printf '  <out-dir>  receives checkwright-companion-<version>.zip and its .sha256\n'
}

case "${1:-}" in
    -h|--help) usage; exit 0 ;;
    --) shift ;;
esac
if [ "$#" -ne 2 ]; then
    usage >&2
    exit 2
fi
version="$1"
out="$2"
case "$version" in
    ''|-*|*/*|*[[:space:]]*) usage >&2; exit 2 ;;
esac

refuse() {
    printf 'ci-pack-extension: %s\n' "$1" >&2
    exit 2
}

# spec: companion/SPEC.md §Packing the extension — every external program is probed by name before the first step
for tool in git python3 tar; do
    command -v "$tool" >/dev/null 2>&1 || refuse "$tool not found on PATH"
done
if command -v sha256sum >/dev/null 2>&1; then
    hasher=(sha256sum)
elif command -v shasum >/dev/null 2>&1; then
    hasher=(shasum -a 256)
else
    refuse "neither sha256sum nor shasum found on PATH"
fi

mkdir -p "$out"
out="$(cd "$out" && pwd -P)"
cd "$(git rev-parse --show-toplevel)" || refuse "not inside a git checkout"
name="checkwright-companion-$version"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
stage="$scratch/$name"
mkdir "$stage"

# spec: companion/SPEC.md §Packing the extension — the tracked files at HEAD, never the worktree, plus the repository's LICENSE
git archive --format=tar HEAD:companion/speckit | tar -xf - -C "$stage"
git show HEAD:LICENSE > "$stage/LICENSE"

manifest="$stage/extension.yml"
placeholder='  version: "0.0.0"'
count="$(grep -cxF "$placeholder" "$manifest" || true)"
[ "$count" = 1 ] || refuse "extension.yml carries the version placeholder line $count time(s), where the stamp needs exactly one"
awk -v want="$placeholder" -v line="  version: \"$version\"" '$0 == want { print line; next } { print }' \
    "$manifest" > "$manifest.new"
mv "$manifest.new" "$manifest"

( cd "$scratch" && python3 -m zipfile -c "$name.zip" "$name" )
mv "$scratch/$name.zip" "$out/$name.zip"
# spec: gate-sdk/SPEC.md §Consumer payload — the sidecar in the `<hex>  <name>` form every other asset's takes
( cd "$out" && "${hasher[@]}" "$name.zip" > "$name.zip.sha256" )
printf 'packed %s/%s.zip at version %s\n' "$out" "$name" "$version"
