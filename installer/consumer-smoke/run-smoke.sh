#!/usr/bin/env bash
# spec: installer/README.md §The consumer smoke — packs the package, installs it from the resulting tarball with no registry access, and drives init through a scratch consumer once per profile; exit 0 asserts the whole activation path (install → green battery → manifest agrees with the tree → idempotent re-run → doctor clean → a planted prose defect caught and cleared → diff clean → uninstall back to the pre-init tree object) plus the four profile-lattice assertions and the value assertion over the loop (some profile below the maximum catches that defect) (every named kit resolves, exactly one minimum and one maximum, the maximum is the payload-derived profile, and gate rosters are monotone across every comparable pair), a two-hop cross-version upgrade that also relinquishes a payload path on one hop and re-adds it on the next, a toolchain-free arm driving doctor and a full init with cargo and rustc masked off PATH, a same-version seam arm over the two surfaces init rewrites every run and the protection branch chained onto it, a narrowing arm re-running init at a smaller profile so files[] outlives kits, and an artifact arm that builds the gate binary, packs it, and drives both selection branches — placement, omit-and-declare, and the two refusals between them; the evidence-kit 'installer_smoke' validate suite each validate stage re-runs.
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

# spec: installer/README.md §The consumer smoke — cargo and rustc join the preflight because the artifact arm builds the binary it packs; they refuse here with every other missing tool, since a machine that cannot compile the crate has not falsified the install path
for tool in npm node jq git tar sha256sum cargo rustc; do
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
# spec: installer/README.md §The gate binary — which members an omission covers is derived from the payload, so the arm asserting the record reads that derivation out of the installed package rather than carrying a second copy of it
# shellcheck source=../lib/common/recipe.sh
source "$PKG_ROOT/lib/common/recipe.sh"
# spec: installer/README.md §The manifest — the two seam paths every verb asks the resolver for, spelled once from the installer's own GATES_DIR constant rather than at each call site: a literal here would be a second copy of the consumer layout the module already owns
SEAM_FILES=("$GATES_DIR/gates.list" "$GATES_DIR/gate-sdk-config.sh")
mapfile -t PAYLOAD_KITS < <(profile_payload_kits "$PKG_ROOT")
[[ ${#PAYLOAD_KITS[@]} -gt 0 ]] || fail "the installed payload carries no kit"
mapfile -t PROFILES < <(profile_names "$PKG_ROOT")
say "profiles: ${PROFILES[*]} (${#PAYLOAD_KITS[@]} kits in the payload)"

resolves() { local k; for k in "${PAYLOAD_KITS[@]}"; do [[ "$k" == "$1" ]] && return 0; done; return 1; }
contains() {   # $1 = superset (newline list), $2 = the containment claim being asserted, $3 = subset (newline list)
    local m
    while IFS= read -r m; do
        [[ -n "$m" ]] || continue
        grep -qxF "$m" <<<"$1" || fail "$2: $m is in the smaller set and missing from the larger"
    done <<<"$3"
}
# spec: installer/README.md §Profiles — assertion 1: every named kit resolves in the payload, which is what makes a roster a roster rather than a wish
for p in "${PROFILES[@]}"; do
    mapfile -t members < <(profile_kits "$PKG_ROOT" "$p")
    [[ ${#members[@]} -gt 0 ]] || fail "profile '$p' resolves to no kit in the payload"
    for k in "${members[@]}"; do
        resolves "$k" || fail "profile '$p' names $k, which the payload does not carry"
    done
done

# spec: installer/README.md §Profiles — assertions 2 and 3: the lattice is bounded, so it has exactly one profile below every other and exactly one above every other, and the one above is the payload-derived profile by construction. A profile comparable to nothing, a second incomparable maximum, and two profiles resolving to the same kit set are all reds here — which is the contract the deleted "at most three profiles" bound was standing in for, stated as a shape instead of a count
mapfile -t ORDER < <(profile_order "$PKG_ROOT")
MINIMA=(); MAXIMA=()
for p in "${PROFILES[@]}"; do
    below=0; above=0
    for pair in "${ORDER[@]}"; do
        IFS=$'\t' read -r a b <<<"$pair"
        [[ "$a" == "$p" ]] && below=$((below + 1))
        [[ "$b" == "$p" ]] && above=$((above + 1))
    done
    [[ "$below" -eq $(( ${#PROFILES[@]} - 1 )) ]] && MINIMA+=("$p")
    [[ "$above" -eq $(( ${#PROFILES[@]} - 1 )) ]] && MAXIMA+=("$p")
done
[[ ${#MINIMA[@]} -eq 1 ]] \
    || fail "the profile order has ${#MINIMA[@]} minima [${MINIMA[*]}] where a bounded lattice has exactly one"
[[ ${#MAXIMA[@]} -eq 1 ]] \
    || fail "the profile order has ${#MAXIMA[@]} maxima [${MAXIMA[*]}] where a bounded lattice has exactly one"
[[ "${MAXIMA[0]}" == "$PROFILE_DERIVED" ]] \
    || fail "the maximum profile is ${MAXIMA[0]} where the payload-derived profile $PROFILE_DERIVED is the top by construction"
PROFILE_MIN="${MINIMA[0]}"
say "order: ${#ORDER[@]} comparable pair(s), minimum $PROFILE_MIN, maximum $PROFILE_DERIVED"

# spec: installer/README.md §Profiles — assertion 4, and it is what the profile argument on recipe_gates is for: the promise is not that a bigger profile vendors more directories but that moving up only ever adds to the battery, and kit-set containment stops implying gate-set containment the moment a roster varies by profile
for pair in "${ORDER[@]}"; do
    IFS=$'\t' read -r a b <<<"$pair"
    contains "$(profile_gates "$PKG_ROOT" "$b")" "gate-roster monotonicity, $a ⊆ $b" "$(profile_gates "$PKG_ROOT" "$a")"
done
say "gate rosters are monotone across every comparable pair"

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
    local profile="$1" C="$2" out rc before after LOCK mismatch checked path want got target seam bin list k m line omitted want_omitted n_omitted

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

    # spec: installer/README.md §The gate binary — the arm takes whichever of the selection outcomes the payload and host produce, and the set init must omit is derived from the consumer's own vendored tree rather than spelled here: a literal would read as green on today's tree, where nothing dispatches to a binary, and stop asserting on the first tree where something does
    target="$(jq -r '.artifact.target // ""' "$LOCK")"
    seam="$(lock_own_file "$LOCK" "$GATES_DIR/gate-sdk-config.sh")"
    list="$(lock_own_file "$LOCK" "$GATES_DIR/gates.list")"
    [[ -n "$list" ]] || fail "$profile: the manifest records no gates.list"
    omitted="$(sed -n 's/^# omitted: \([^[:space:]]*\).*$/\1/p' "$C/$list" | sort)"
    want_omitted="$(
        for k in "${lock_kits[@]}"; do
            while IFS= read -r m; do
                [[ -n "$m" ]] || continue
                [[ -f "$C/$k/checks/$m.gate" && ! -f "$C/$k/checks/$m.sh" ]] && printf '%s\n' "$m"
            done < <(recipe_gates "$PKG_ROOT/payload/$k" "$profile")
        done | sort
    )"
    n_omitted="$(grep -c . <<<"$omitted")"
    if [[ -n "$target" ]]; then
        [[ -n "$seam" && -f "$C/$seam" ]] || fail "$profile: an artifact is recorded but no gate-sdk config seam names its path"
        bin="$(sed -n 's/^GATE_SDK_NATIVE_BIN=//p' "$C/$seam" | head -n1)"
        [[ -n "$bin" && -x "$C/$bin" ]] || fail "$profile: no executable gate binary at '${bin:-<unset>}'"
        [[ "$(sha256sum "$C/$bin" | cut -d' ' -f1)" == "$(jq -r '.artifact.digest' "$LOCK")" ]] \
            || fail "$profile: the installed gate binary does not match the digest the manifest recorded"
        # spec: installer/README.md §The manifest — the binary and the seam are files init wrote, so both are on the roster it records: a path init created and did not record reads as "never installed" next run, which is the reading that lets the following install write straight through it
        for path in "$bin" "$seam"; do
            [[ "$(jq -r --arg f "$path" '.files | has($f)' "$LOCK")" == "true" ]] \
                || fail "$profile: init wrote $path on the placement path but the manifest roster does not record it"
        done
        [[ "$n_omitted" -eq 0 ]] \
            || fail "$profile: the artifact was placed, so nothing is omitted, yet the registry declares $n_omitted: $omitted"
        say "artifact: $target verified in place at $bin, recorded with the seam, nothing omitted"
    else
        [[ -z "$seam" ]] || ! grep -q '^GATE_SDK_NATIVE_BIN=' "$C/$seam" \
            || fail "$profile: no artifact was installed, yet the config seam points at a gate binary"
        [[ "$omitted" == "$want_omitted" ]] \
            || fail "$profile: the registry omits [${omitted//$'\n'/ }] where this payload dispatches [${want_omitted//$'\n'/ }] to the binary"
        while IFS= read -r line; do
            [[ -z "$line" || "$line" =~ ^#\ omitted:\ [^[:space:]]+\ [^[:space:]]+$ ]] \
                || fail "$profile: malformed omission record '$line' — the record carries a member and the reason token naming its remedy"
        done < <(grep '^# omitted:' "$C/$list")
        say "artifact: none packed — $n_omitted member(s) omitted and declared, exactly the ones this payload dispatches to a binary"
    fi

    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" doctor 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "$profile: doctor exited $rc inside the installed consumer"; }
    grep -q "^  profile      $profile\$" <<<"$out" || fail "$profile: doctor did not report the installed profile"
    say "doctor: clean, reports the installed profile"
}

# spec: installer/README.md §The consumer smoke — the value post-condition: an install that is green, idempotent and reversible is still worth nothing if it never catches anything, so each profile's battery is put in front of one real defect in adopter-authored prose — a mistyped relative link in a README, on a consumer whose own content is markdown and nothing else. Which gate delivers the red is deliberately not asserted: naming one would be a second roster to maintain beside recipe_gates, and the claim is about the battery rather than about a member of it. The arm restores the consumer to the commit it found, so the reversal that follows still asserts against the tree init wrote
assert_value() {   # $1 = profile, $2 = scratch consumer dir -> sets VALUE_VERDICT to 'red' or 'green' for the defect
    local profile="$1" C="$2" out rc head
    VALUE_VERDICT=
    head="$(git -C "$C" rev-parse HEAD)"
    mkdir -p "$C/docs"
    printf '# Handbook\n\nStart with [the style guide](style-guid.md).\n' > "$C/docs/README.md"
    printf '# Style guide\n\nWrite plainly, and link what you cite.\n' > "$C/docs/style-guide.md"
    git -C "$C" add -A && git -C "$C" commit -q -m "handbook" \
        || fail "$profile: could not commit the prose consumer's own content"

    out="$( cd "$C" && PATH="$RUN_PATH" bash gate-sdk/bin/run-gates.sh 2>&1 )"; rc=$?
    if [[ "$rc" -eq 0 ]]; then VALUE_VERDICT=green; else VALUE_VERDICT=red; fi

    # spec: installer/README.md §The consumer smoke — the fix is the link and never the corpus
    printf '# Handbook\n\nStart with [the style guide](style-guide.md).\n' > "$C/docs/README.md"
    git -C "$C" commit -qam "fix the link" || fail "$profile: could not commit the fix"
    out="$( cd "$C" && PATH="$RUN_PATH" bash gate-sdk/bin/run-gates.sh 2>&1 )"; rc=$?
    if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
        printf '%s\n' "$out"
        fail "$profile: the battery is still not green on prose whose only defect was fixed"
    fi
    say "value: the planted prose defect is $VALUE_VERDICT on this profile, green once fixed"

    git -C "$C" reset -q --hard "$head" && git -C "$C" clean -qfd \
        || fail "$profile: could not restore the consumer after the value arm"
}

# spec: installer/README.md §The consumer smoke — one encoding of the reversal, run on the same consumer assert_install just finished with, so both transports prove it and the masked arm proves diff and uninstall are Node-free at no extra pack cost. The tree-object equality against the pre-init seed is the load-bearing assertion and it proves more than uninstall: no other arm asserts that the roster covers everything init wrote — the per-profile check runs the other direction, entry against tree — so a file init wrote and failed to record survives the removal and reds here
assert_reversal() {   # $1 = profile, $2 = scratch consumer dir, $3 = the consumer's tree object before init ran
    local profile="$1" C="$2" seed="$3" out rc before status planned

    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" diff 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] \
        || { printf '%s\n' "$out" >&2; fail "$profile: diff exited $rc against the tree init just wrote — a freshly installed tree is the definition of no drift"; }
    grep -q '^DIFF: clean' <<<"$out" \
        || { printf '%s\n' "$out" >&2; fail "$profile: diff exited 0 without reporting the tree clean"; }
    say "diff: $(grep -m1 '^DIFF:' <<<"$out")"

    # spec: installer/README.md §The consumer smoke — the --dry-run rule is asserted behaviorally rather than through a syntactic proxy: the plan must name a non-zero removal count while the tree object and the worktree are both exactly what they were, which is what a flag that parsed and then wrote anyway would fail and a flag that merely existed would pass
    before="$(git -C "$C" rev-parse 'HEAD^{tree}')"
    status="$(git -C "$C" status --porcelain)"
    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" uninstall --dry-run 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "$profile: uninstall --dry-run exited $rc"; }
    planned="$(sed -n 's/^would remove \([0-9][0-9]*\) file(s):$/\1/p' <<<"$out" | head -n1)"
    [[ -n "$planned" && "$planned" -gt 0 ]] \
        || { printf '%s\n' "$out" >&2; fail "$profile: uninstall --dry-run planned no removal against an install it is about to reverse"; }
    [[ "$(git -C "$C" rev-parse 'HEAD^{tree}')" == "$before" ]] \
        || fail "$profile: uninstall --dry-run changed the tree object"
    [[ "$(git -C "$C" status --porcelain)" == "$status" ]] \
        || fail "$profile: uninstall --dry-run left the worktree changed"
    say "uninstall --dry-run: $planned file(s) planned, tree object and worktree unchanged"

    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" uninstall 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "$profile: uninstall exited $rc"; }
    [[ "$(git -C "$C" rev-parse 'HEAD^{tree}')" == "$seed" ]] \
        || { printf '%s\n' "$out" >&2; fail "$profile: the tree after uninstall is not the tree from before init — either init wrote something it did not record, or the removal reached past the roster"; }
    [[ -z "$(git -C "$C" status --porcelain)" ]] || fail "$profile: uninstall left the worktree dirty"
    [[ ! -f "$C/checkwright.lock" ]] \
        || fail "$profile: every recorded file was removed, yet a manifest survives asserting an install that is gone"
    say "uninstall: $(sed -n 's/^UNINSTALL: //p' <<<"$out" | head -n1) tree object is back to its pre-init state"
}

ENTRY=("$CW")
RUN_PATH="$PATH"
VALUE_RED=()
for profile in "${PROFILES[@]}"; do
    printf '%s\n' "$profile"
    C="$(consumer "$profile")" || fail "could not build a scratch consumer for $profile"
    SEED="$(git -C "$C" rev-parse 'HEAD^{tree}')"
    assert_install "$profile" "$C"
    assert_value "$profile" "$C"
    [[ "$VALUE_VERDICT" == red ]] && VALUE_RED+=("$profile")
    assert_reversal "$profile" "$C" "$SEED"
done

# spec: installer/README.md §The consumer smoke — the value assertion is over the whole loop rather than inside it, because which profiles catch a prose defect is derived from the rosters and asserting it per profile would be that derivation copied out. Two claims: some profile catches it at all, and some profile *below the maximum* does — the second is the one that matters, since a defect only the payload-derived profile catches is not value an adopter can choose, it is value they have to take everything for
[[ ${#VALUE_RED[@]} -gt 0 ]] \
    || fail "no profile's battery caught the planted prose defect — the install is green, idempotent and reversible, and worth nothing on a document"
value_below_max=0
for p in "${VALUE_RED[@]}"; do [[ "$p" != "$PROFILE_DERIVED" ]] && value_below_max=1; done
[[ "$value_below_max" -eq 1 ]] \
    || fail "only $PROFILE_DERIVED caught the planted prose defect — no profile short of everything delivers value on prose"
say "value: caught by ${VALUE_RED[*]}, at least one of them below $PROFILE_DERIVED"

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
SEED="$(git -C "$C" rev-parse 'HEAD^{tree}')"
assert_install "$PROFILE_DERIVED" "$C"
assert_reversal "$PROFILE_DERIVED" "$C" "$SEED"

# spec: installer/README.md §The consumer smoke — the toolchain-free arm, and the reason it uses the mask the Node-free arm already proved rather than a knob: the preflight requires cargo and rustc because the artifact arm builds the crate it packs, so every arm above drives doctor and init on a machine that has them and none could observe an install path demanding them. A masked PATH is what a machine with no Rust toolchain actually is, where a knob suppressing a roster member would be a second, test-only audience axis no adopter ever exercises
printf 'toolchain-free arm (%s, cargo/rustc masked)\n' "$PROFILE_DERIVED"
TOOLMASK="$SCRATCH/toolmask"
mkdir -p "$TOOLMASK"
for masked in cargo rustc; do
    printf '#!/usr/bin/env bash\necho "toolchain-free arm: %s was reached — the install path is not free of the Rust toolchain" >&2\nexit 127\n' \
        "$masked" > "$TOOLMASK/$masked"
    chmod +x "$TOOLMASK/$masked"
done

ENTRY=("$CW")
RUN_PATH="$TOOLMASK:$PATH"
# spec: installer/README.md §The consumer smoke — masking is per-arm, which is what lets this arm exist without weakening the preflight the artifact arm depends on; the mask is proved rather than assumed for the same reason the Node-free one is
for masked in cargo rustc; do
    resolved="$( PATH="$RUN_PATH" bash -c "command -v $masked" 2>/dev/null )"
    [[ "$resolved" == "$TOOLMASK/$masked" ]] \
        || fail "the mask did not take: $masked resolves to '${resolved:-nothing}', not the shim at $TOOLMASK/$masked"
done
say "mask: cargo and rustc resolve to failing shims"
C="$(consumer toolchain-free)" || fail "could not build a scratch consumer for the toolchain-free arm"
# spec: installer/README.md §doctor — doctor is asserted before init as well as inside it, because init reads only its exit status: an adopter meets this verdict first, and it is the precondition every later refusal is downstream of
out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" doctor 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] \
    || { printf '%s\n' "$out" >&2; fail "doctor is below contract on a machine carrying no Rust toolchain — a contributor-audience roster member is reaching the adopter's verdict"; }
grep -q '^DOCTOR: clean' <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "doctor exited 0 on a toolchain-free machine without reporting clean"; }
if grep -qE '^  (cargo|rustc) ' <<<"$out"; then
    printf '%s\n' "$out" >&2
    fail "doctor rendered a contributor-audience member to an adopter — such a member is omitted from the consumer verdict, not reported as informational"
fi
say "doctor: clean with no Rust toolchain on PATH, and silent about the members that need one"
assert_install "$PROFILE_DERIVED" "$C"

# spec: installer/README.md §The consumer smoke — the upgrade arm packs a second, higher version and drives the same installed tree across it, because everything above installs at one version: what only a cross-version run reaches is the manifest's version comparison falling through in the upgrade direction, the profile re-read from the lock with no flag, and claim() re-applying around a file the adopter has since edited
printf 'upgrade arm (two cross-version hops, %s profile — the lattice minimum, so the arm is the smallest install that carries the manifest behavior it asserts)\n' "$PROFILE_MIN"
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
out="$( cd "$C" && "$CW" init --profile "$PROFILE_MIN" 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the upgrade arm's starting install failed"; }
LOCK="$C/checkwright.lock"
[[ "$(jq -r '.version' "$LOCK")" == "$VERSION" ]] || fail "the upgrade arm did not start at $VERSION"
was_kits="$(jq -r '.kits | join(" ")' "$LOCK")"
say "installed $VERSION at the $PROFILE_MIN profile ($was_kits)"

# spec: installer/README.md §init — the adopter's edit is committed, because init refuses a dirty worktree: the case under test is a file changed since init wrote it, not an uncommitted one
EDITED="gate-sdk/README.md"
[[ "$(jq -r --arg f "$EDITED" '.files | has($f)' "$LOCK")" == "true" ]] \
    || fail "the $PROFILE_MIN manifest does not record $EDITED — the arm has nothing whose adopter edit it can assert"
# spec: installer/README.md §The manifest — the relinquish subject is chosen against a criterion, not by taste: a payload file the minimum profile records in files[] that no init step and neither generated projection reads, so dropping it from one hop's payload exercises the roster's exit condition and nothing else
RELINQUISHED="gate-sdk/templates/check-skeleton.sh"
[[ "$(jq -r --arg f "$RELINQUISHED" '.files | has($f)' "$LOCK")" == "true" ]] \
    || fail "the $PROFILE_MIN manifest does not record $RELINQUISHED — the relinquish arm has no subject"
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
[[ "$(jq -r '.profile' "$LOCK")" == "$PROFILE_MIN" ]] \
    || fail "the upgrade was run with no --profile and did not re-read $PROFILE_MIN from the manifest"
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
printf 'seam arm (same-version re-run, %s profile)\n' "$PROFILE_DERIVED"
SC="$(consumer seam)" || fail "could not build a scratch consumer for the seam arm"
out="$( cd "$SC" && "$CW" init --profile "$PROFILE_DERIVED" 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the seam arm's install failed"; }
say "init: $(grep -m1 '^INIT:' <<<"$out")"
SEAM_LOCK="$SC/checkwright.lock"
# spec: installer/README.md §What init seeds — the two surfaces init rewrites on every run: a templates/*-config.sh destination and gate-sdk's msg-patterns.list. The arm runs at the maximum profile because that is the only profile whose kit set is fixed by the payload rather than by a roster judgment, so it is where both surfaces are present by construction — a smaller profile would tie the arm to a membership row that is a judgment and may be revised
SEAM_EDITED=(scripts/queue-config.sh scripts/msg-patterns.list)
declare -A SEAM_INIT_HASH=() SEAM_WANT=()
for f in "${SEAM_EDITED[@]}"; do
    [[ "$(jq -r --arg f "$f" '.files | has($f)' "$SEAM_LOCK")" == "true" ]] \
        || fail "the $PROFILE_DERIVED manifest does not record $f — the seam arm has nothing whose adopter edit it can assert"
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

# spec: installer/README.md §The consumer smoke — the protection branch chains onto this arm rather than the reversal arm, because an adopter edit is exactly the case tree-object equality cannot host: this consumer already carries two edited, committed vendored files, which is the case that reaches uninstall's keep branch and the residual manifest behind it
declare -A SEAM_KEPT=()
for f in "${SEAM_EDITED[@]}"; do SEAM_KEPT["$f"]=1; done
mapfile -t SEAM_ROSTER < <(jq -r '.files | keys[]' "$SEAM_LOCK")
[[ ${#SEAM_ROSTER[@]} -gt ${#SEAM_EDITED[@]} ]] \
    || fail "the seam manifest records ${#SEAM_ROSTER[@]} file(s), so the protection chain has nothing whose removal it can assert beside the two it keeps"

out="$( cd "$SC" && "$CW" diff 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] \
    || { printf '%s\n' "$out" >&2; fail "diff exited $rc on a consumer carrying two adopter-edited vendored files — the drift verdict is the exit status, and 1 is what a CI step gating on a pristine vendored tree reads"; }
for f in "${SEAM_EDITED[@]}"; do
    grep -qF "$f" <<<"$out" || { printf '%s\n' "$out" >&2; fail "diff reported drift without naming $f"; }
done
say "diff: $(grep -m1 '^DIFF:' <<<"$out"), naming ${SEAM_EDITED[*]}"

out="$( cd "$SC" && "$CW" uninstall 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "uninstall exited $rc on the seam arm's consumer"; }
for f in "${SEAM_EDITED[@]}"; do
    [[ "$(git hash-object -- "$SC/$f")" == "${SEAM_WANT[$f]}" ]] \
        || fail "uninstall removed or rewrote $f, which the adopter had changed since init wrote it"
    grep -qF "$f" <<<"$out" || { printf '%s\n' "$out" >&2; fail "uninstall kept $f but did not report it"; }
done
for f in "${SEAM_ROSTER[@]}"; do
    [[ -n "${SEAM_KEPT[$f]:-}" ]] && continue
    [[ ! -e "$SC/$f" ]] \
        || fail "uninstall left $f on the tree, which the adopter never touched — the removal stopped short of the roster it was given"
done

# spec: installer/README.md §The manifest — the survivors are still on disk, so their ownership has not ended and the roster must retain them: a manifest deleted here would disown exactly the paths the hash rule just protected, and the next init would read them as never installed and write straight through the adopter. The recorded hash is init's rather than the adopter's for the same reason the upgrade arm names the two apart
[[ -f "$SEAM_LOCK" ]] \
    || fail "uninstall kept ${#SEAM_EDITED[@]} file(s) and deleted the manifest — the next init would read them as never installed and write straight through them"
got="$(jq -r 'keys | join(" ")' "$SEAM_LOCK")"
[[ "$got" == "files schema" ]] \
    || fail "the residual manifest carries [$got] where an install that no longer exists may assert only its schema and the files it still owns"
got="$(jq -r '.files | keys[]' "$SEAM_LOCK")"
want="$(printf '%s\n' "${SEAM_EDITED[@]}" | LC_ALL=C sort)"
[[ "$got" == "$want" ]] \
    || fail "the residual roster is [${got//$'\n'/ }] where the survivors are [${want//$'\n'/ }]"
for f in "${SEAM_EDITED[@]}"; do
    [[ "$(jq -r --arg f "$f" '.files[$f]' "$SEAM_LOCK")" == "${SEAM_INIT_HASH[$f]}" ]] \
        || fail "the residual manifest records $f at a hash other than the one init wrote there — the next init would find it unchanged and claim it"
done

# spec: installer/README.md §The manifest — the residual shape is asserted on the object itself and not through an accessor, because that is the class of drift an accessor cannot catch: a missing key and a present-but-null key both read back as the empty string, so only has() tells an omitted artifact apart from a null one, and only re-sorting the captured text proves the sort reached every nesting level rather than the top one
jq -e 'has("artifact") | not' "$SEAM_LOCK" >/dev/null \
    || fail "the residual manifest carries an artifact key — an omitted field leaves the key absent, never null"
jq -S . "$SEAM_LOCK" | cmp -s - "$SEAM_LOCK" \
    || fail "the residual manifest is not byte-identical to its own recursive sort — the one writer of the wire shape emitted an order its second writer could not reproduce"
say "protection: ${SEAM_EDITED[*]} kept and reported, $(( ${#SEAM_ROSTER[@]} - ${#SEAM_EDITED[@]} )) recorded file(s) removed, manifest narrowed to schema + the survivors at init's hashes"

# spec: installer/README.md §The manifest — the narrowing arm, and it is the only arm that moves a consumer *down* the lattice: every other re-run holds the profile fixed, so none reaches the state where files[] outlives kits. That state is not exotic — it is the ordinary consequence of the carry-forward rule, which keeps every once-vendored path on the roster while the recorded kit set shrinks
printf 'narrowing arm (%s installed, re-run at %s)\n' "$PROFILE_DERIVED" "$PROFILE_MIN"
NC2="$(consumer narrowing)" || fail "could not build a scratch consumer for the narrowing arm"
out="$( cd "$NC2" && "$CW" init --profile "$PROFILE_DERIVED" 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the narrowing arm's wide install failed"; }
NARROW_LOCK="$NC2/checkwright.lock"
wide_kits="$(jq -r '.kits | length' "$NARROW_LOCK")"
out="$( cd "$NC2" && "$CW" init --profile "$PROFILE_MIN" 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the narrowing re-run failed"; }
narrow_kits="$(jq -r '.kits | length' "$NARROW_LOCK")"
[[ "$narrow_kits" -lt "$wide_kits" ]] \
    || fail "the re-run recorded $narrow_kits kit(s) where the wide install recorded $wide_kits — the arm did not narrow anything"

# spec: installer/README.md §The manifest — the residual shape is the same defect with the kits key absent rather than shrunk, so it is asserted on a kits-stripped copy of this manifest rather than bought a second consumer: uninstall's residual carries schema and files only, and a resolver leaning on the recorded kit set excludes nothing at all there
jq 'del(.kits)' "$NARROW_LOCK" > "$NC2/residual-shape.json" \
    || fail "could not derive the residual manifest shape"

# spec: installer/README.md §The manifest — only the seam paths this install actually recorded are asserted on, because the config seam is written on the artifact placement path alone: a payload carrying no prebuilt binary records none, and demanding it here would fail the arm on the payload rather than on the resolver. The arm proves its own premise instead — at least one checked path, and at least one of them genuinely shadowed by a vendored fixture tree, or a green result would mean only that the payload changed shape
narrow_checked=0
narrow_shadowed=0
for f in "${SEAM_FILES[@]}"; do
    [[ "$(jq -r --arg f "$f" '.files | has($f)' "$NARROW_LOCK")" == "true" ]] || continue
    narrow_checked=$((narrow_checked + 1))
    shadow="$(jq -r --arg b "/${f##*/}" --arg own "$f" \
        '.files | keys | map(select(endswith($b) and . != $own)) | length' "$NARROW_LOCK")"
    [[ "$shadow" -gt 0 ]] && narrow_shadowed=$((narrow_shadowed + 1))
    got="$(lock_own_file "$NARROW_LOCK" "$f")"
    [[ "$got" == "$f" ]] \
        || fail "after narrowing, the consumer's own $f resolves to '${got:-<nothing>}' — files[] outlives kits, so a recorded-kit predicate stops excluding the dropped kits' fixture trees"
    got="$(lock_own_file "$NC2/residual-shape.json" "$f")"
    [[ "$got" == "$f" ]] \
        || fail "on a manifest carrying no kits, the consumer's own $f resolves to '${got:-<nothing>}' — the residual shape has no kit set to exclude anything with"
done
[[ "$narrow_checked" -gt 0 ]] \
    || fail "the narrowed manifest records none of [${SEAM_FILES[*]}] — the arm has no seam path to resolve"
[[ "$narrow_shadowed" -gt 0 ]] \
    || fail "no vendored fixture shadows any checked seam basename — the arm would pass without asserting"

out="$( cd "$NC2" && "$CW" doctor 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "doctor exited $rc on the narrowed consumer"; }
grep -q "^  registry     $GATES_DIR/gates.list\$" <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "doctor did not name the consumer's own $GATES_DIR/gates.list as the registry it inspected"; }
say "narrowing: $wide_kits kit(s) -> $narrow_kits, $narrow_checked recorded seam path(s) ($narrow_shadowed shadowed) still resolve to the consumer's own on the narrowed and the residual shape, doctor names the registry"

# spec: installer/README.md §The consumer smoke — the artifact arm builds the binary it packs rather than fabricating a stand-in with a matching digest: a stand-in would drive the placement path while leaving the one thing most likely to break — the real build's digest agreeing with what init verifies before writing — covered by nothing
printf 'artifact arm (host build, placement branch and the two refusals)\n'
# spec: gate-sdk/SPEC.md §Layout and configuration — the native knobs are read through gate-sdk's own accessors, from the tree under test, so the arm cannot drift from the roster or from a knob override; in a subshell because the library auto-sources a consumer config seam off the current directory and this script is not one of its consumers
native() {   # $@ = a gate-sdk accessor and its arguments, resolved against the tree under test
    # shellcheck source=../../gate-sdk/lib/gate.sh
    ( cd "$REPO" && source gate-sdk/lib/gate.sh && "$@" )
}
NATIVE_BIN="$(native gate_native_bin)"; NATIVE_BIN="${NATIVE_BIN##*/}"
NATIVE_CRATE="$(native gate_native_crate)"
ROSTER_FILE="$(native gate_native_targets_file)"
[[ "$ROSTER_FILE" == /* ]] || ROSTER_FILE="$REPO/$ROSTER_FILE"
mapfile -t ROSTER < <(native gate_native_targets)
[[ ${#ROSTER[@]} -gt 0 ]] \
    || blocked "no declared target at $ROSTER_FILE — there is no platform set to build for."
HOST_TARGET="$(rustc -vV 2>/dev/null | awk '/^host:/{print $2}')"
[[ -n "$HOST_TARGET" ]] || blocked "rustc reported no host target — the arm cannot tell which roster line this machine satisfies."
# spec: installer/README.md §The consumer smoke — pack refuses a roster target no leg built, so a host build satisfies --artifacts only while the roster is this host alone; the moment it declares a second target the arm blocks here naming its own remedy rather than packing a payload with a hole in it
[[ ${#ROSTER[@]} -eq 1 && "${ROSTER[0]}" == "$HOST_TARGET" ]] \
    || blocked "the roster declares ${ROSTER[*]} and this host is $HOST_TARGET — a host build no longer satisfies pack's all-targets demand. Steer this arm's pack at the host alone with GATE_SDK_NATIVE_TARGETS_FILE, or give the leg a cross-compiling build."

ART="$SCRATCH/artifacts/$HOST_TARGET"
mkdir -p "$ART"
build_out="$(cargo build --release --manifest-path "$REPO/$NATIVE_CRATE/Cargo.toml" 2>&1)" \
    || { printf '%s\n' "$build_out" >&2; blocked "the crate would not compile for $HOST_TARGET."; }
BUILT="$REPO/$NATIVE_CRATE/target/release/$NATIVE_BIN"
[[ -x "$BUILT" ]] || blocked "cargo reported success but there is no executable at $BUILT."
cp "$BUILT" "$ART/$NATIVE_BIN" || fail "could not stage the built binary for packing"
# spec: gate-sdk/SPEC.md §Consumer payload — the digest is emitted once, here, where the bytes are produced: pack re-verifies this sidecar and init verifies it again before writing, so both readers check a value neither of them computed
( cd "$ART" && sha256sum "$NATIVE_BIN" > "$NATIVE_BIN.sha256" ) \
    || fail "could not emit the digest sidecar beside the built binary"
[[ -z "$(git -C "$REPO" status --porcelain)" ]] \
    || fail "the build leg left the worktree dirty — the crate's output must land in gitignored build space and the artifact directory in the smoke's own scratch"

ARTP="$SCRATCH/artifact-pack"
mkdir -p "$ARTP"
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" \
    --version "$VERSION" --out "$ARTP" --artifacts "$SCRATCH/artifacts" 2>&1)" \
    || { printf '%s\n' "$PACK_OUT" >&2; blocked "the artifact pack step failed."; }
say "$(grep -m1 '^PACK:' <<<"$PACK_OUT")"
shopt -s nullglob
art_tarballs=("$ARTP"/*.tgz)
shopt -u nullglob
[[ ${#art_tarballs[@]} -eq 1 ]] || fail "expected exactly one artifact tarball, found ${#art_tarballs[@]}"
( cd "$ARTP" && tar -xzf "${art_tarballs[0]##*/}" ) || fail "tar could not extract the artifact tarball"
PAY_ART="$ARTP/package/payload/artifact"
[[ -f "$PAY_ART/targets.list" && -f "$PAY_ART/$HOST_TARGET/$NATIVE_BIN" && -f "$PAY_ART/$HOST_TARGET/$NATIVE_BIN.sha256" ]] \
    || fail "the artifact payload carries no complete $HOST_TARGET artifact beside a verbatim roster copy"
say "built $NATIVE_BIN for $HOST_TARGET and packed it with the sidecar this leg emitted"

ENTRY=(bash "$ARTP/package/bin/checkwright.sh")
RUN_PATH="$PATH"
C="$(consumer artifact)" || fail "could not build a scratch consumer for the artifact arm"
assert_install "$PROFILE_MIN" "$C"
LOCK="$C/checkwright.lock"
# spec: installer/README.md §The gate binary — target resolution is asserted against what the toolchain says this host is, not against whatever init selected: the two derivations are independent (uname pair versus rustc's own triple) and only comparing them catches a mapping that resolves confidently to the wrong roster line
[[ "$(jq -r '.artifact.target' "$LOCK")" == "$HOST_TARGET" ]] \
    || fail "init selected '$(jq -r '.artifact.target' "$LOCK")' where rustc reports this host as $HOST_TARGET"
[[ "$(jq -r '.artifact.digest' "$LOCK")" == "$(awk 'NR==1{print $1}' "$ART/$NATIVE_BIN.sha256")" ]] \
    || fail "the manifest records a digest other than the one this arm's build leg emitted"

# spec: installer/README.md §The gate binary — the third selection outcome, and the one the other two are told apart from: a host the payload never committed to omits and declares, so it must exit clean and write a registry rather than refuse the way a declared-but-absent target does
printf '%s\n' "other-${HOST_TARGET#*-}" > "$PAY_ART/targets.list" \
    || fail "could not narrow the payload roster off this host"
NC="$(consumer artifact-undeclared)" || fail "could not build a scratch consumer for the undeclared-host leg"
out="$( cd "$NC" && "${ENTRY[@]}" init --profile "$PROFILE_MIN" 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "init refused a payload that simply does not commit to this platform — 'never declared' omits and declares, it does not fail the install"; }
[[ "$(jq -r 'has("artifact")' "$NC/checkwright.lock")" == "false" ]] \
    || fail "the payload declares no artifact for this host, yet the manifest records one"
say "host off the payload roster: omitted and declared, install clean"
cp "$ROSTER_FILE" "$PAY_ART/targets.list" || fail "could not restore the payload roster"

# spec: installer/README.md §The gate binary — the verification is pre-write, so the assertion is on the consumer's tree and not only on the exit code: a warn-then-install would exit non-zero too, and only an untouched tree tells the two apart
printf 'tampered\n' >> "$PAY_ART/$HOST_TARGET/$NATIVE_BIN"
TC="$(consumer artifact-tampered)" || fail "could not build a scratch consumer for the tampered-artifact leg"
before="$(git -C "$TC" rev-parse 'HEAD^{tree}')"
out="$( cd "$TC" && "${ENTRY[@]}" init --profile "$PROFILE_MIN" 2>&1 )"; rc=$?
[[ "$rc" -ne 0 ]] || { printf '%s\n' "$out" >&2; fail "init installed a gate binary whose bytes do not match the digest published beside it"; }
[[ "$(git -C "$TC" rev-parse 'HEAD^{tree}')" == "$before" && -z "$(git -C "$TC" status --porcelain)" && ! -f "$TC/checkwright.lock" ]] \
    || fail "the digest refusal left the consumer changed — it was checked after something was written, not before"
say "tampered artifact: refused with nothing written"

# spec: installer/README.md §The gate binary — a declared target whose artifact went missing is the outcome that must not collapse into the omission above: same host, same roster, and the only difference is the missing pair, so a run that omitted here would be reading a broken payload as a narrower one
rm -f "$PAY_ART/$HOST_TARGET/$NATIVE_BIN" || fail "could not remove the declared target's binary"
AC="$(consumer artifact-absent)" || fail "could not build a scratch consumer for the declared-but-absent leg"
out="$( cd "$AC" && "${ENTRY[@]}" init --profile "$PROFILE_MIN" 2>&1 )"; rc=$?
[[ "$rc" -ne 0 ]] || { printf '%s\n' "$out" >&2; fail "the payload declares $HOST_TARGET and carries no artifact for it, and init installed anyway — a broken payload read as a narrower one"; }
[[ ! -f "$AC/checkwright.lock" && -z "$(git -C "$AC" status --porcelain)" ]] \
    || fail "the broken-payload refusal still wrote into the consumer"
say "declared target with no artifact: refused, not omitted"

printf 'INSTALLER-SMOKE: clean (%d profile(s) installed from the packed tarball with no registry access and each reversed back to its pre-init tree object, plus the extracted-tarball arm with node/npm masked and reversed the same way, the toolchain-free arm driving doctor and a full init with cargo/rustc masked, the two-hop cross-version upgrade arm carrying the relinquish and re-add, the same-version seam arm and the protection branch chained onto it, the narrowing arm re-running init at a smaller profile so files[] outlives kits, and the artifact arm packing a binary this run built)\n' "${#PROFILES[@]}"
exit 0
