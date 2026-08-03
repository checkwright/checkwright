#!/usr/bin/env bash
# spec: installer/README.md §The consumer smoke — packs the package, installs it from the resulting tarball with no registry access, and drives init through a scratch consumer once per profile; exit 0 asserts the whole activation path (install → green battery → manifest agrees with the tree → idempotent re-run → doctor clean) plus the profile invariant, the evidence-kit 'installer_smoke' validate suite each validate stage re-runs.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO="$(dirname "$REPO")"

# spec: installer/README.md §The consumer smoke — INSTALLER_SMOKE_TMP_DIR is the only knob; everything the smoke writes lands under it, so a run leaves the worktree untouched
BASE="${INSTALLER_SMOKE_TMP_DIR:-${TMPDIR:-/tmp}}"
[[ -d "$BASE" ]] || { echo "INSTALLER-SMOKE: scratch base not a directory: $BASE" >&2; exit 2; }
SCRATCH="$(mktemp -d "$BASE/installer-smoke.XXXXXX")" || exit 2
cleanup() { rm -rf "$SCRATCH"; }
trap cleanup EXIT

say() { printf '  %s\n' "$*"; }
fail() { printf 'INSTALLER-SMOKE: FAIL — %s\n' "$*"; exit 1; }
blocked() { printf 'INSTALLER-SMOKE: %s\n' "$*" >&2; exit 2; }

for tool in npm node jq git tar sha256sum; do
    command -v "$tool" >/dev/null 2>&1 || blocked "$tool not found on PATH — the smoke cannot run."
done
[[ -z "$(git -C "$REPO" status --porcelain)" ]] \
    || blocked "the worktree is dirty — the pack step refuses to stamp a commit the payload does not match. Commit or stash first."

printf 'pack\n'
VERSION="$(git -C "$REPO" describe --tags --abbrev=0 2>/dev/null)"; VERSION="${VERSION#v}"
[[ -n "$VERSION" ]] || VERSION="0.0.0-smoke"
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --version "$VERSION" --out "$SCRATCH" 2>&1)" \
    || { printf '%s\n' "$PACK_OUT" >&2; blocked "the pack step failed."; }
say "$(grep -m1 '^PACK:' <<<"$PACK_OUT")"
shopt -s nullglob
tarballs=("$SCRATCH"/*.tgz)
shopt -u nullglob
[[ ${#tarballs[@]} -eq 1 ]] || fail "expected exactly one tarball, found ${#tarballs[@]}"
TARBALL="${tarballs[0]}"

# spec: installer/README.md §The consumer smoke — the install is from the packed tarball with --offline, which is what proves the claim the install page makes: a one-shot vendoring installer resolves nothing from a registry, so the payload must already be inside the tarball
printf 'install (from the tarball, --offline)\n'
NODE_HOME="$SCRATCH/node-home"
mkdir -p "$NODE_HOME"
printf '{"name":"smoke-host","version":"1.0.0","private":true}\n' > "$NODE_HOME/package.json"
npm_out="$( cd "$NODE_HOME" && npm install --offline --no-audit --no-fund --loglevel=error "$TARBALL" 2>&1 )" \
    || { printf '%s\n' "$npm_out" >&2; fail "npm could not install the packed tarball offline — the package resolves something from a registry"; }
CW="$NODE_HOME/node_modules/.bin/checkwright"
[[ -x "$CW" ]] || fail "the installed package exposes no executable checkwright bin entry"
PKG_ROOT="$NODE_HOME/node_modules/checkwright"
say "installed $(jq -r '.version' "$PKG_ROOT/package.json") from $(basename "$TARBALL")"

# spec: installer/README.md §Profiles — the invariant is asserted against the installed payload rather than the source tree, so it holds for what an adopter actually receives
printf 'profile invariant\n'
# shellcheck source=../lib/common/profile.sh
source "$PKG_ROOT/lib/common/profile.sh"
mapfile -t PAYLOAD_KITS < <(profile_payload_kits "$PKG_ROOT")
[[ ${#PAYLOAD_KITS[@]} -gt 0 ]] || fail "the installed payload carries no kit"
mapfile -t PROFILES < <(profile_names "$PKG_ROOT")
[[ ${#PROFILES[@]} -le 3 ]] || fail "at most three profiles, found ${#PROFILES[@]}: ${PROFILES[*]}"
say "profiles: ${PROFILES[*]} (${#PAYLOAD_KITS[@]} kits in the payload)"

resolves() { local k; for k in "${PAYLOAD_KITS[@]}"; do [[ "$k" == "$1" ]] && return 0; done; return 1; }
contains() {   # $1 = superset (newline list), $2 = subset name, $3 = subset (newline list)
    local m
    while IFS= read -r m; do
        [[ -n "$m" ]] || continue
        grep -qxF "$m" <<<"$1" || fail "$2 is not contained in its successor: $m is missing"
    done <<<"$3"
}
for p in "${PROFILES[@]}"; do
    mapfile -t members < <(profile_kits "$PKG_ROOT" "$p")
    [[ ${#members[@]} -gt 0 ]] || fail "profile '$p' resolves to no kit in the payload"
    for k in "${members[@]}"; do
        resolves "$k" || fail "profile '$p' names $k, which the payload does not carry"
    done
done
# spec: installer/README.md §Profiles — the containment chain is what makes "progressive" a contract instead of a word: moving up a profile only ever adds
STARTER="$(profile_kits "$PKG_ROOT" starter)"
DELEGATION="$(profile_kits "$PKG_ROOT" delegation)"
FULL="$(profile_kits "$PKG_ROOT" "$PROFILE_DERIVED")"
contains "$DELEGATION" "starter" "$STARTER"
contains "$FULL" "delegation" "$DELEGATION"
say "every named kit resolves; starter ⊆ delegation ⊆ ${PROFILE_DERIVED}"

consumer() {   # $1 = profile -> a fresh scratch consumer repo, echoed
    local c
    c="$(mktemp -d "$SCRATCH/consumer-$1.XXXXXX")" || return 1
    git -C "$c" init -q
    git -C "$c" config user.email smoke@example.invalid
    git -C "$c" config user.name smoke
    printf '.tmp/\n' > "$c/.gitignore"
    git -C "$c" add -A
    git -C "$c" commit -q -m "seed"
    printf '%s' "$c"
}

# spec: installer/README.md §The consumer smoke — one encoding of the post-conditions, read by both transports, so the two arms cannot drift into asserting different things about the same install; ENTRY is the invocation of the installed entry point and RUN_PATH the PATH every step runs under, which is what lets the download arm mask node/npm without a second copy of the assertions
assert_install() {   # $1 = profile, $2 = scratch consumer dir
    local profile="$1" C="$2" out rc before after LOCK mismatch checked path want got target seam bin list

    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" init --profile "$profile" 2>&1 )" \
        || { printf '%s\n' "$out" >&2; fail "init failed for the $profile profile"; }
    say "init: $(grep -m1 '^INIT:' <<<"$out")"

    out="$( cd "$C" && PATH="$RUN_PATH" bash gate-sdk/bin/run-gates.sh 2>&1 )"; rc=$?
    if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
        printf '%s\n' "$out"
        fail "the battery is not green on the $profile consumer init just made"
    fi
    say "battery: $(grep -E 'All [0-9]+ gates passed' <<<"$out")"

    # spec: installer/README.md §The manifest — the files[] hash is what init's changed-file detection reads, so a manifest that disagrees with the tree it describes would make the non-destructive re-run report on noise
    LOCK="$C/checkwright.lock"
    [[ -f "$LOCK" ]] || fail "$profile: init wrote no checkwright.lock"
    [[ "$(jq -r '.schema' "$LOCK")" == "checkwright-lock v1" ]] || fail "$profile: manifest carries an unexpected schema"
    [[ "$(jq -r '.version' "$LOCK")" == "$VERSION" ]] \
        || fail "$profile: manifest records version $(jq -r '.version' "$LOCK"), packed $VERSION"
    [[ "$(jq -r '.commit' "$LOCK")" =~ ^[0-9a-f]{40}$ ]] || fail "$profile: manifest records no 40-hex commit"
    [[ "$(jq -r '.profile' "$LOCK")" == "$profile" ]] || fail "$profile: manifest records the wrong profile"
    mismatch=0; checked=0
    while IFS=$'\t' read -r path want; do
        checked=$((checked + 1))
        [[ -f "$C/$path" ]] || { echo "  manifest names a file that is not there: $path"; mismatch=$((mismatch + 1)); continue; }
        got="$(git hash-object -- "$C/$path")"
        [[ "$got" == "$want" ]] || { echo "  manifest hash disagrees with the tree: $path"; mismatch=$((mismatch + 1)); }
    done < <(jq -r '.files | to_entries[] | "\(.key)\t\(.value)"' "$LOCK")
    [[ "$mismatch" -eq 0 ]] || fail "$profile: $mismatch of $checked manifest entries disagree with the tree"
    [[ "$checked" -gt 0 ]] || fail "$profile: the manifest records no file"
    mapfile -t lock_kits < <(jq -r '.kits[]' "$LOCK")
    mapfile -t want_kits < <(profile_kits "$PKG_ROOT" "$profile")
    [[ "${lock_kits[*]}" == "${want_kits[*]}" ]] \
        || fail "$profile: manifest kits (${lock_kits[*]}) differ from the profile roster (${want_kits[*]})"
    say "manifest: $checked file(s) agree with the tree, ${#lock_kits[@]} kit(s) recorded"

    # spec: installer/README.md §init — idempotence is a property of the tree, so the assertion is on the tree object and not on what the re-run printed
    before="$(git -C "$C" rev-parse 'HEAD^{tree}')"
    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" init 2>&1 )" \
        || { printf '%s\n' "$out" >&2; fail "$profile: the idempotent re-run of init failed"; }
    after="$(git -C "$C" rev-parse 'HEAD^{tree}')"
    [[ "$before" == "$after" ]] || fail "$profile: re-running init changed the tree — the install is not idempotent"
    [[ -z "$(git -C "$C" status --porcelain)" ]] || fail "$profile: the re-run left the worktree dirty"
    say "re-run: tree unchanged"

    # spec: installer/README.md §The consumer smoke — the artifact arm takes whichever outcome the payload and host produce: with a packed artifact the target, the digest and an executable binary at the seam's path; with none, the omission arm below, which is what reds if init ever records an artifact the payload did not carry
    target="$(jq -r '.artifact.target // ""' "$LOCK")"
    seam="$(jq -r '.files | keys[] | select(endswith("/gate-sdk-config.sh")) // ""' "$LOCK" | head -n1)"
    if [[ -n "$target" ]]; then
        [[ -n "$seam" && -f "$C/$seam" ]] || fail "$profile: an artifact is recorded but no gate-sdk config seam names its path"
        bin="$(sed -n 's/^GATE_SDK_NATIVE_BIN=//p' "$C/$seam" | head -n1)"
        [[ -n "$bin" && -x "$C/$bin" ]] || fail "$profile: no executable gate binary at '${bin:-<unset>}'"
        [[ "$(sha256sum "$C/$bin" | cut -d' ' -f1)" == "$(jq -r '.artifact.digest' "$LOCK")" ]] \
            || fail "$profile: the installed gate binary does not match the digest the manifest recorded"
        say "artifact: $target verified in place at $bin"
    else
        list="$(jq -r '.files | keys[] | select(endswith("/gates.list"))' "$LOCK" | head -n1)"
        [[ -n "$list" ]] || fail "$profile: the manifest records no gates.list"
        ! grep -q '^# omitted:' "$C/$list" \
            || fail "$profile: the registry declares omitted members while the manifest records no artifact"
        [[ -z "$seam" ]] || ! grep -q '^GATE_SDK_NATIVE_BIN=' "$C/$seam" \
            || fail "$profile: no artifact was installed, yet the config seam points at a gate binary"
        say "artifact: none packed — omitted, and nothing claims one"
    fi

    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" doctor 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "$profile: doctor exited $rc inside the installed consumer"; }
    grep -q "^  profile      $profile\$" <<<"$out" || fail "$profile: doctor did not report the installed profile"
    say "doctor: clean, reports the installed profile"
}

ENTRY=("$CW")
RUN_PATH="$PATH"
for profile in "${PROFILES[@]}"; do
    printf '%s\n' "$profile"
    C="$(consumer "$profile")" || fail "could not build a scratch consumer for $profile"
    assert_install "$profile" "$C"
done

# spec: installer/README.md §The consumer smoke — the download transport, asserted rather than documented: verify the digest, extract with tar rather than npm, and drive the same post-conditions with node/npm masked, so a latent Node dependency reds here instead of passing on a host that happens to carry Node
printf 'download arm (%s, node/npm masked)\n' "$PROFILE_DERIVED"
DL="$SCRATCH/download"
mkdir -p "$DL"
cp "$TARBALL" "$DL/"
DL_NAME="$(basename "$TARBALL")"
( cd "$DL" && sha256sum "$DL_NAME" > "$DL_NAME.sha256" && sha256sum -c --status "$DL_NAME.sha256" ) \
    || fail "the packed tarball does not verify against its own sha256 digest — the checksum step the install page documents would not work"
( cd "$DL" && tar -xzf "$DL_NAME" ) || fail "tar could not extract the packed tarball"
DL_ENTRY="$DL/package/bin/checkwright.sh"
[[ -f "$DL_ENTRY" ]] || fail "the extracted tarball carries no package/bin/checkwright.sh — the Node-free entry point is not in the payload"
say "verified $DL_NAME against its digest and extracted package/ with tar"

# spec: installer/README.md §The consumer smoke — the mask is the whole value of this arm: a shim that reds and names itself turns a latent Node reach into a loud failure rather than a silent pass, and shims are what keeps the mask portable — dropping every PATH entry that carries node would take /usr/bin with it on the hosts where node lives there
MASK="$SCRATCH/mask"
mkdir -p "$MASK"
for masked in node npm npx; do
    printf '#!/usr/bin/env bash\necho "download arm: %s was reached — the tarball path is not Node-free" >&2\nexit 127\n' \
        "$masked" > "$MASK/$masked"
    chmod +x "$MASK/$masked"
done

ENTRY=(bash "$DL_ENTRY")
RUN_PATH="$MASK:$PATH"
# spec: installer/README.md §The consumer smoke — the mask is proved rather than assumed: an arm whose PATH silently failed to shadow the real interpreter would assert nothing while passing
for masked in node npm npx; do
    resolved="$( PATH="$RUN_PATH" bash -c "command -v $masked" 2>/dev/null )"
    [[ "$resolved" == "$MASK/$masked" ]] \
        || fail "the mask did not take: $masked resolves to '${resolved:-nothing}', not the shim at $MASK/$masked"
done
say "mask: node, npm and npx resolve to failing shims"
C="$(consumer "download")" || fail "could not build a scratch consumer for the download arm"
assert_install "$PROFILE_DERIVED" "$C"

printf 'INSTALLER-SMOKE: clean (%d profile(s) installed from the packed tarball with no registry access, plus the extracted-tarball arm with node/npm masked)\n' "${#PROFILES[@]}"
exit 0
