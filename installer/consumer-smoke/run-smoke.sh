#!/usr/bin/env bash
# spec: installer/README.md §The consumer smoke — packs the package, installs it from the resulting tarball with no registry access, and drives init through a scratch consumer once per profile; exit 0 asserts the whole activation path (install → green battery → manifest agrees with the tree → idempotent re-run → doctor clean) plus the profile invariant, a two-hop cross-version upgrade that also relinquishes a payload path on one hop and re-adds it on the next, and a same-version seam arm over the two surfaces init rewrites every run, the evidence-kit 'installer_smoke' validate suite each validate stage re-runs.
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
# shellcheck source=../lib/common/lock.sh
source "$PKG_ROOT/lib/common/lock.sh"
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
    seam="$(lock_own_file "$LOCK" /gate-sdk-config.sh)"
    if [[ -n "$target" ]]; then
        [[ -n "$seam" && -f "$C/$seam" ]] || fail "$profile: an artifact is recorded but no gate-sdk config seam names its path"
        bin="$(sed -n 's/^GATE_SDK_NATIVE_BIN=//p' "$C/$seam" | head -n1)"
        [[ -n "$bin" && -x "$C/$bin" ]] || fail "$profile: no executable gate binary at '${bin:-<unset>}'"
        [[ "$(sha256sum "$C/$bin" | cut -d' ' -f1)" == "$(jq -r '.artifact.digest' "$LOCK")" ]] \
            || fail "$profile: the installed gate binary does not match the digest the manifest recorded"
        say "artifact: $target verified in place at $bin"
    else
        list="$(lock_own_file "$LOCK" /gates.list)"
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

# spec: installer/README.md §The consumer smoke — the upgrade arm packs a second, higher version and drives the same installed tree across it, because everything above installs at one version: what only a cross-version run reaches is the manifest's version comparison falling through in the upgrade direction, the profile re-read from the lock with no flag, and claim() re-applying around a file the adopter has since edited
printf 'upgrade arm (two cross-version hops, starter profile)\n'
next_patch() { awk -F. '{ printf "%d.%d.%d", $1, $2, $3 + 1 }' <<<"${1%%[-+]*}"; }
upgrade_direction() {   # $1 = from, $2 = to -> 0 iff $2 sorts strictly above $1
    [[ "$1" != "$2" && "$(printf '%s\n%s\n' "$1" "$2" | sort -V | head -n1)" == "$1" ]]
}
UP_VERSION="$(next_patch "$VERSION")"
upgrade_direction "$VERSION" "$UP_VERSION" \
    || fail "the arm derived $UP_VERSION from $VERSION, which is not the upgrade direction — it would assert the downgrade refusal instead"
UP="$SCRATCH/upgrade"
mkdir -p "$UP"
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --version "$UP_VERSION" --out "$UP" 2>&1)" \
    || { printf '%s\n' "$PACK_OUT" >&2; blocked "the upgrade pack step failed."; }
say "$(grep -m1 '^PACK:' <<<"$PACK_OUT")"
shopt -s nullglob
up_tarballs=("$UP"/*.tgz)
shopt -u nullglob
[[ ${#up_tarballs[@]} -eq 1 ]] || fail "expected exactly one upgrade tarball, found ${#up_tarballs[@]}"
( cd "$UP" && tar -xzf "${up_tarballs[0]##*/}" ) || fail "tar could not extract the upgrade tarball"

C="$(consumer upgrade)" || fail "could not build a scratch consumer for the upgrade arm"
out="$( cd "$C" && "$CW" init --profile starter 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the upgrade arm's starting install failed"; }
LOCK="$C/checkwright.lock"
[[ "$(jq -r '.version' "$LOCK")" == "$VERSION" ]] || fail "the upgrade arm did not start at $VERSION"
was_kits="$(jq -r '.kits | join(" ")' "$LOCK")"
say "installed $VERSION at the starter profile ($was_kits)"

# spec: installer/README.md §init — the adopter's edit is committed, because init refuses a dirty worktree: the case under test is a file changed since init wrote it, not an uncommitted one
EDITED="gate-sdk/README.md"
[[ "$(jq -r --arg f "$EDITED" '.files | has($f)' "$LOCK")" == "true" ]] \
    || fail "the starter manifest does not record $EDITED — the arm has nothing whose adopter edit it can assert"
# spec: installer/README.md §The manifest — the relinquish subject is chosen against a criterion, not by taste: a starter-kit payload file init records in files[] that no init step and neither generated projection reads, so dropping it from one hop's payload exercises the roster's exit condition and nothing else
RELINQUISHED="gate-sdk/templates/check-skeleton.sh"
[[ "$(jq -r --arg f "$RELINQUISHED" '.files | has($f)' "$LOCK")" == "true" ]] \
    || fail "the starter manifest does not record $RELINQUISHED — the relinquish arm has no subject"
R_INIT_HASH="$(jq -r --arg f "$RELINQUISHED" '.files[$f]' "$LOCK")"
printf '\nAn adopter edited this line.\n' >> "$C/$EDITED"
printf '\n# An adopter edited this line.\n' >> "$C/$RELINQUISHED"
EDITED_WANT="$(git hash-object -- "$C/$EDITED")"
R_WANT="$(git hash-object -- "$C/$RELINQUISHED")"
git -C "$C" add -- "$EDITED" "$RELINQUISHED" && git -C "$C" commit -q -m "edit two vendored files" \
    || fail "could not commit the adopter edits in the scratch consumer"

# spec: installer/README.md §The consumer smoke — the relinquish is performed on the extracted package's own payload rather than through a pack flag: pack-installer assembles every version from one worktree, so without this the two hops carry byte-identical payloads and no path ever leaves a kit's shipped set. Mutating the test's own extracted copy keeps the publishing path with no way to ship a payload with a hole in it
rm -f "$UP/package/payload/$RELINQUISHED" \
    || fail "could not drop $RELINQUISHED from the upgrade payload"
[[ ! -f "$UP/package/payload/$RELINQUISHED" ]] \
    || fail "the upgrade payload still ships $RELINQUISHED — the relinquish hop would assert nothing"

out="$( cd "$C" && bash "$UP/package/bin/checkwright.sh" init 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the cross-version re-run of init failed — the version check did not fall through in the upgrade direction"; }
[[ "$(jq -r '.version' "$LOCK")" == "$UP_VERSION" ]] \
    || fail "the manifest records $(jq -r '.version' "$LOCK") after upgrading to $UP_VERSION"
[[ "$(jq -r '.profile' "$LOCK")" == "starter" ]] \
    || fail "the upgrade was run with no --profile and did not re-read starter from the manifest"
[[ "$(jq -r '.kits | join(" ")' "$LOCK")" == "$was_kits" ]] \
    || fail "the upgrade changed the recorded kit set from '$was_kits' to '$(jq -r '.kits | join(" ")' "$LOCK")'"
[[ "$(git hash-object -- "$C/$EDITED")" == "$EDITED_WANT" ]] \
    || fail "the upgrade overwrote $EDITED, which the adopter had changed since init wrote it"
grep -qF "$EDITED" <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "the upgrade left $EDITED alone but did not report it as changed"; }
[[ -z "$(git -C "$C" status --porcelain)" ]] || fail "the upgrade left the worktree dirty"
# spec: installer/README.md §The manifest — the roster is what carries the protection to the next hop, so it is asserted directly and not only through its effect: a dropped entry reads as "never installed" next run, and an entry recorded at the adopter's own hash reads as unchanged — both let the following init claim the path, so both are named apart
[[ "$(jq -r --arg f "$EDITED" '.files | has($f)' "$LOCK")" == "true" ]] \
    || fail "the upgrade dropped $EDITED from the manifest roster — the next run would read its absence as 'never installed'"
[[ "$(jq -r --arg f "$EDITED" '.files[$f]' "$LOCK")" != "$EDITED_WANT" ]] \
    || fail "the upgrade recorded the adopter's own hash for $EDITED — the next run would find it unchanged and claim it"
# spec: installer/README.md §The manifest — a path leaves the roster when the file leaves the tree and at no other moment, so the hop whose payload stopped shipping it must still carry it at the hash init wrote there: dropping it here is what makes the re-adding hop below read the path as never installed
[[ "$(git hash-object -- "$C/$RELINQUISHED")" == "$R_WANT" ]] \
    || fail "the upgrade touched $RELINQUISHED, which its payload no longer ships"
[[ "$(jq -r --arg f "$RELINQUISHED" '.files | has($f)' "$LOCK")" == "true" ]] \
    || fail "the upgrade disowned $RELINQUISHED because its payload stopped shipping it — the next release to re-add the path would write straight through the adopter's edits"
[[ "$(jq -r --arg f "$RELINQUISHED" '.files[$f]' "$LOCK")" == "$R_INIT_HASH" ]] \
    || fail "the upgrade kept $RELINQUISHED on the roster at a hash other than the one init wrote there"
say "upgrade: $VERSION -> $UP_VERSION, profile re-read, $EDITED preserved and reported, $RELINQUISHED relinquished and still owned"

# spec: installer/README.md §The consumer smoke — the second hop is the one the first cannot stand in for: one upgrade shows the protection starting, and only the next shows whether it persists or inverts, so the same consumer is carried across a third version with no fresh adopter edit
UP2_VERSION="$(next_patch "$UP_VERSION")"
upgrade_direction "$UP_VERSION" "$UP2_VERSION" \
    || fail "the arm derived $UP2_VERSION from $UP_VERSION, which is not the upgrade direction — it would assert the downgrade refusal instead"
UP2="$SCRATCH/upgrade2"
mkdir -p "$UP2"
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --version "$UP2_VERSION" --out "$UP2" 2>&1)" \
    || { printf '%s\n' "$PACK_OUT" >&2; blocked "the second upgrade pack step failed."; }
say "$(grep -m1 '^PACK:' <<<"$PACK_OUT")"
shopt -s nullglob
up2_tarballs=("$UP2"/*.tgz)
shopt -u nullglob
[[ ${#up2_tarballs[@]} -eq 1 ]] || fail "expected exactly one second-upgrade tarball, found ${#up2_tarballs[@]}"
( cd "$UP2" && tar -xzf "${up2_tarballs[0]##*/}" ) || fail "tar could not extract the second-upgrade tarball"

out="$( cd "$C" && bash "$UP2/package/bin/checkwright.sh" init 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the second cross-version re-run of init failed"; }
[[ "$(jq -r '.version' "$LOCK")" == "$UP2_VERSION" ]] \
    || fail "the manifest records $(jq -r '.version' "$LOCK") after upgrading to $UP2_VERSION"
[[ "$(git hash-object -- "$C/$EDITED")" == "$EDITED_WANT" ]] \
    || fail "the second upgrade overwrote $EDITED — the protection lasted one upgrade and then inverted"
grep -qF "$EDITED" <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "the second upgrade left $EDITED alone but did not report it as changed"; }
# spec: installer/README.md §The manifest — the re-adding hop is where the ownership rule pays: this payload ships $RELINQUISHED again, so it must meet the carried claim and refuse, which is the whole defect reproduced end to end rather than argued about
[[ "$(git hash-object -- "$C/$RELINQUISHED")" == "$R_WANT" ]] \
    || fail "the re-adding payload overwrote $RELINQUISHED — the roster did not carry the ownership across the relinquish"
grep -qF "$RELINQUISHED" <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "the re-adding payload left $RELINQUISHED alone but did not report it as changed"; }
[[ "$(jq -r --arg f "$RELINQUISHED" '.files[$f]' "$LOCK")" == "$R_INIT_HASH" ]] \
    || fail "the re-adding payload recorded $RELINQUISHED at a hash other than the one init wrote there"
[[ -z "$(git -C "$C" status --porcelain)" ]] || fail "the second upgrade left the worktree dirty"
say "second upgrade: $UP_VERSION -> $UP2_VERSION, $EDITED still the adopter's and still reported, $RELINQUISHED re-added and refused"

# spec: installer/README.md §What init seeds — the seam arm is its own consumer, because the per-profile loop asserts the manifest agrees with the tree file by file against a freshly initialized consumer and an adopter edit inside it would break the assertion it is there to make. It reuses the already-installed package with no extra pack, and it re-runs at the same version with no flags: this class needs no upgrade and no --force, so an arm that only ran across versions would attribute it to a path it does not live on
printf 'seam arm (same-version re-run, delegation profile)\n'
SC="$(consumer seam)" || fail "could not build a scratch consumer for the seam arm"
out="$( cd "$SC" && "$CW" init --profile delegation 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the seam arm's install failed"; }
say "init: $(grep -m1 '^INIT:' <<<"$out")"
SEAM_LOCK="$SC/checkwright.lock"
# spec: installer/README.md §What init seeds — the two surfaces init rewrites on every run: a templates/*-config.sh destination (starter is gate-sdk alone and gate-sdk ships no config template, so these are reachable from delegation up) and gate-sdk's msg-patterns.list, which reaches the starter profile and so the smallest install
SEAM_EDITED=(scripts/queue-config.sh scripts/msg-patterns.list)
declare -A SEAM_INIT_HASH=() SEAM_WANT=()
for f in "${SEAM_EDITED[@]}"; do
    [[ "$(jq -r --arg f "$f" '.files | has($f)' "$SEAM_LOCK")" == "true" ]] \
        || fail "the delegation manifest does not record $f — the seam arm has nothing whose adopter edit it can assert"
    SEAM_INIT_HASH["$f"]="$(jq -r --arg f "$f" '.files[$f]' "$SEAM_LOCK")"
    printf '\n# An adopter edited this line.\n' >> "$SC/$f"
    SEAM_WANT["$f"]="$(git hash-object -- "$SC/$f")"
done
git -C "$SC" add -- "${SEAM_EDITED[@]}" && git -C "$SC" commit -q -m "edit the seam surfaces" \
    || fail "could not commit the adopter's seam edits in the scratch consumer"

out="$( cd "$SC" && "$CW" init 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the seam arm's same-version re-run of init failed"; }
for f in "${SEAM_EDITED[@]}"; do
    [[ "$(git hash-object -- "$SC/$f")" == "${SEAM_WANT[$f]}" ]] \
        || fail "the re-run overwrote $f — it is copied into the consumer outside claim(), so the comparison ran against the copy rather than the adopter's content"
    grep -qF "$f" <<<"$out" \
        || { printf '%s\n' "$out" >&2; fail "the re-run left $f alone but did not report it as changed"; }
    [[ "$(jq -r --arg f "$f" '.files[$f]' "$SEAM_LOCK")" == "${SEAM_INIT_HASH[$f]}" ]] \
        || fail "the re-run recorded $f at a hash other than the one init wrote there"
done
[[ -z "$(git -C "$SC" status --porcelain)" ]] || fail "the seam arm's re-run left the worktree dirty"
say "seam: ${SEAM_EDITED[*]} preserved, reported and still recorded at init's hash"

printf 'INSTALLER-SMOKE: clean (%d profile(s) installed from the packed tarball with no registry access, plus the extracted-tarball arm with node/npm masked, the two-hop cross-version upgrade arm carrying the relinquish and re-add, and the same-version seam arm)\n' "${#PROFILES[@]}"
exit 0
