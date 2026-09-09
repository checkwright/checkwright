#!/usr/bin/env bash
# spec: installer/README.md §The consumer smoke — builds the host gate binary, packs the package around it, installs it from the resulting tarball with no registry access, and drives init through a scratch consumer once per profile; exit 0 asserts the whole activation path (install → every command init printed in its follow-up block resolves in the payload just written with every flag it names accepted → green battery → manifest agrees with the tree, a disagreement whose own operands are hashes failing at exit 1 as a verdict about the consumer while one that reached the comparison malformed refuses at exit 2 as a precondition of this harness → the seeded queue satisfies queue-kit's section contract, or none is seeded where none is owed — which of the two is owed read from the package through the --install queue-source op rather than derived a second time here → idempotent re-run → doctor clean → a planted prose defect caught and cleared → diff clean → uninstall back to the pre-init tree object) plus the four profile-lattice assertions and the value assertion over the loop (some profile below the maximum catches that defect) (every named kit resolves, exactly one minimum and one maximum, the maximum is the payload-derived profile, and gate rosters are monotone across every comparable pair of the registries the installs wrote), an artifact-less refusal leg driving the packer's own artifact-free output and asserting that init, doctor, diff and a bare invocation all meet one bootstrap refusal that names the platform, carries a remedy and writes nothing, a two-hop cross-version upgrade that also relinquishes a payload path on one hop and re-adds it on the next, whose first hop asserts a non-zero live-member count and a placed artifact in the consumer's registry before asserting the worktree is clean — so cleanliness is evidence over a hop that rewrote something rather than over one that rewrote nothing, a cross-version reversal arm carrying an unedited consumer across those same three versions and back to its pre-init tree object, so removability is asserted after a payload changed shape and the roster is asserted to cover an upgrade hop's write set rather than a first init's alone, a toolchain-free arm driving doctor and a full init with cargo and rustc masked off PATH, a jq-less arm asserting that diff and uninstall run clean with no jq on PATH while init is blocked by doctor's floor verdict and doctor still reaches its whole report, a same-version seam arm over the two surfaces init rewrites every run and the protection branch chained onto it, a narrowing arm re-running init at a smaller profile so files[] outlives kits, and an artifact arm driving the selection outcomes a single install cannot show — the unrostered host's refusal, the tampered artifact's and the declared-but-absent target's, asserted to differ in message and remedy rather than only in exit status; the evidence-kit 'installer_smoke' validate suite each validate stage re-runs.
# no-port: installer/README.md §The consumer smoke, The port disposition — ruled 2026-08-31 by the operator in consult. This is the repo's own acceptance harness for the installer and rides no payload: scripts/pack-installer.sh assembles the tarball and the npm package out of the kit roots and never out of installer/consumer-smoke/, so no adopter receives or runs it, and its only callers are the evidence-kit installer_smoke validate suite and the gates workflow. It is the same shape gate-sdk/SPEC.md §Consumer smoke, The port disposition declares on its leg 3 — a smoke executed by no adopter path — reached one step further, for a harness the payload does not even carry; and it drives cargo, the packer and init as black boxes across every profile, so a crate-side form would test the binary from inside the binary. Structural, not a sizing judgment: its size was measured at the ruling and is not the ground.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO="$(dirname "$REPO")"

# spec: installer/README.md §The consumer smoke — INSTALLER_SMOKE_TMP_DIR is the scratch knob; everything the smoke writes lands under it, so a run leaves the worktree untouched
BASE="${INSTALLER_SMOKE_TMP_DIR:-${TMPDIR:-/tmp}}"
[[ -d "$BASE" ]] || { echo "INSTALLER-SMOKE: scratch base not a directory: $BASE" >&2; exit 2; }
SCRATCH="$(mktemp -d "$BASE/installer-smoke.XXXXXX")" || exit 2
cleanup() { rm -rf "$SCRATCH"; }
trap cleanup EXIT

# spec: evidence-kit/SPEC.md §Layout and configuration — the unindented arm headers below are a PARSED contract, not narration: the installer_smoke validate parser derives its scenario roster from these `printf` lines and names each scenario by the literal before the parenthetical, so rewording one renames a baseline scenario and changing the header's shape empties the roster
say() { printf '  %s\n' "$*"; }
fail() { printf 'INSTALLER-SMOKE: FAIL — %s\n' "$*"; exit 1; }
blocked() { printf 'INSTALLER-SMOKE: %s\n' "$*" >&2; exit 2; }

# spec: installer/README.md §The consumer smoke — the terminator handling the .files read owns, factored into the ONE owner every other multi-line reader of the manifest stream goes through: a `mapfile -t` splits on newline alone, so on a host whose stream ends its lines with CRLF every element keeps the carriage return and reaches an assertion as part of the value. Exactly one trailing CR is dropped per line so a doubled one still shows, the strips are counted, and the count is DECLARED rather than swallowed — which is what keeps a value that genuinely ended in a CR visible as a count instead of vanishing into an array element. The .files read keeps its own copy of the strip rather than calling here because it holds each line UNSTRIPPED as an evidence operand, which an array of stripped values cannot carry
# spec: installer/README.md §The consumer smoke — the byte gets a NAME and the strip gets a single spelling, because rounds 22 to 24 measured `${x%$'\r'}` behaving inconsistently across contexts on the Windows host — stripping where the count is computed and not where the value is stored, in adjacent lines of one function. No mechanism for that is established and none is asserted; what is chosen here is the construct that removes the context-dependence outright, since a `printf`-built variable in a quoted suffix pattern is one expansion with one reading everywhere. The strip is taken ONCE into a named scalar and both the count and the stored value read that scalar, so the two can no longer disagree whatever the cause was
CR="$(printf '\r')"
[[ "${#CR}" -eq 1 ]] \
    || blocked "the carriage return this harness strips by came out ${#CR} byte(s) long — every terminator assertion below would be measuring the wrong byte."
CRLF_DECLARED=""
read_stream() {   # $1 = destination array name, $2 = what a declaration calls this stream; stdin = the stream
    local -n _rs_dst="$1"
    local _rs_line _rs_field _rs_crlf=0
    _rs_dst=(); CRLF_DECLARED=""
    while IFS= read -r _rs_line || [[ -n "$_rs_line" ]]; do
        _rs_field="${_rs_line%"$CR"}"
        [[ "$_rs_field" == "$_rs_line" ]] || _rs_crlf=$((_rs_crlf + 1))
        _rs_dst+=("$_rs_field")
    done
    [[ "$_rs_crlf" -eq 0 ]] || {
        CRLF_DECLARED="$2: the stream delivered $_rs_crlf of ${#_rs_dst[@]} line(s) ending in a carriage return, each dropped as a line terminator"
        say "$CRLF_DECLARED"
    }
}
# spec: installer/README.md §The consumer smoke — no host this repository can reach emits the byte, so the reader above is exercised against a SYNTHETIC stream here rather than left to the one platform that produces it: a repair witnessed only on the host that motivated it is covered by nothing on every host that runs this smoke, and this arm reds on the LF hosts too. Silent and header-less by construction — the say is swallowed and nothing prints on the green path, so this buys no scenario in the parsed roster below
# spec: installer/README.md §The consumer smoke — the expectation is DERIVED from a `mapfile -t` over the identical channel and never spelled as a literal, because a literal makes this arm assert what the CHANNEL delivered rather than what the reader did: the round-21 host was measured putting a carriage return on a stream the LF hosts deliver clean, so a hard-coded `a b c` reds there on the harness's own synthetic data while the reader under test is behaving. The contract asserted instead is the reader's own — `mapfile -t` minus exactly one trailing carriage return per element, counted and declared — which is host-independent by construction. The vacuity guard is the strip having fired at all: a channel delivering no carriage return anywhere covers nothing and says so
_rs_raw=(); _rs_probe=(); _rs_n=0
mapfile -t _rs_raw < <(printf 'a\r\nb\nc\r\n')
read_stream _rs_probe "self-test" < <(printf 'a\r\nb\nc\r\n') > /dev/null
[[ "${#_rs_probe[@]}" -eq "${#_rs_raw[@]}" ]] \
    || blocked "the stream reader returned ${#_rs_probe[@]} element(s) where the same channel delivered ${#_rs_raw[@]} line(s) — it is losing or splitting records, not just terminators."
for _rs_i in "${!_rs_raw[@]}"; do
    _rs_cut="${_rs_raw[_rs_i]%"$CR"}"
    [[ "${_rs_raw[_rs_i]}" == "${_rs_probe[_rs_i]}" ]] || _rs_n=$((_rs_n + 1))
    [[ "${_rs_probe[_rs_i]}" == "$_rs_cut" ]] \
        || blocked "the stream reader did not take exactly one trailing carriage return off element $_rs_i. The channel handed it $(printf '%q' "${_rs_raw[_rs_i]}") and it returned $(printf '%q' "${_rs_probe[_rs_i]}"); one strip off what it was handed is $(printf '%q' "$_rs_cut"). Read those three: a returned value equal to the handed one is a strip that did not fire, and every multi-line manifest read below would inherit the byte."
done
# spec: installer/README.md §The consumer smoke — an uncovered strip is DECLARED here and never blocked on, and the two are different verdicts about different subjects. A reader that mishandles a terminator is this harness's precondition and refuses above; a channel that will not carry the byte the synthetic stream was written with is a fact about the HOST, and refusing on it would stop the one platform whose manifest reads this arm exists to protect before it reached a single one of them. The declaration prints the bytes rather than the conclusion, because the two readings it cannot separate — the channel consumed the carriage return, or the strip pattern never matched it — differ in which surface is at fault and agree on every count a verdict could print
if [[ "$_rs_n" -eq 0 ]]; then
    say "self-test: this host's channel delivered the synthetic CRLF stream with no carriage return for the reader to take — element 0 arrived as $(printf '%q' "${_rs_raw[0]}") and came back as $(printf '%q' "${_rs_probe[0]}"), and the reader's own declaration was [$CRLF_DECLARED]. The strip is UNCOVERED here and said so rather than greened over; every host whose channel carries the byte covers it. A NON-EMPTY declaration beside two identical operands is not an uncovered strip at all — it is a reader whose count and whose stored value disagree, and it is what rounds 22 to 24 were reading."
else
    [[ "$CRLF_DECLARED" == "self-test: the stream delivered $_rs_n of ${#_rs_raw[@]} line(s)"* ]] \
        || blocked "the stream reader took $_rs_n strip(s) and declared [$CRLF_DECLARED] — a strip that stops declaring is the normalization the manifest arm below stays closed to."
fi
unset _rs_raw _rs_probe _rs_n _rs_i _rs_cut

# spec: installer/README.md §The consumer smoke — INSTALLER_SMOKE_ARTIFACTS_DIR is the hand-off knob: a caller that already holds a producer's artifact directory points this at it and the smoke installs those bytes instead of building its own, which is the whole difference between a run that exercises a release-shaped artifact and one that exercises a harness stand-in
PREBUILT_DIR="${INSTALLER_SMOKE_ARTIFACTS_DIR:-}"
[[ -z "$PREBUILT_DIR" || -d "$PREBUILT_DIR" ]] \
    || { echo "INSTALLER-SMOKE: artifact hand-off not a directory: $PREBUILT_DIR" >&2; exit 2; }

# spec: installer/README.md §The consumer smoke — cargo and rustc join the preflight because the smoke builds the binary the main payload carries; they refuse here with every other missing tool, since a machine that cannot compile the crate has not falsified the install path. The relaxation is on the hand-off path ALONE: a host handed a prebuilt artifact was never asked to compile anything, so refusing it for a missing compiler would refuse the exact case the knob exists to serve
SMOKE_TOOLS=(npm node jq git tar sha256sum)
[[ -n "$PREBUILT_DIR" ]] || SMOKE_TOOLS+=(cargo rustc)
for tool in "${SMOKE_TOOLS[@]}"; do
    command -v "$tool" >/dev/null 2>&1 || blocked "$tool not found on PATH — the smoke cannot run."
done
[[ -z "$(git -C "$REPO" status --porcelain)" ]] \
    || blocked "the worktree is dirty — the pack step refuses to stamp a commit the payload does not match. Commit or stash first."

# spec: installer/README.md §The consumer smoke — the binary is built before the pack rather than by a late arm, because the main payload carries it: what every profile installs has to be the battery an adopter on a covered platform actually receives, and a payload with no artifact makes each profile an uncovered-platform install by accident of the harness
printf 'build (the host gate binary the main payload carries)\n'
# spec: gate-sdk/SPEC.md §Layout and configuration — the native knobs are read through gate-sdk's own accessors, from the tree under test, so the arm cannot drift from the roster or from a knob override; in a subshell because the library auto-sources a consumer config seam off the current directory and this script is not one of its consumers
native() {   # $@ = a gate-sdk accessor and its arguments, resolved against the tree under test
    # shellcheck source=../../gate-sdk/lib/gate.sh
    ( cd "$REPO" && source gate-sdk/lib/gate.sh && "$@" )
}
NATIVE_BIN="$(native gate_native_bin)"; NATIVE_BIN="${NATIVE_BIN##*/}"
NATIVE_CRATE="$(native gate_native_crate)"
# spec: installer/README.md §The consumer smoke — rustc answers the host triple wherever rustc is present, and on the hand-off path where it is not, the sole target directory in the artifact directory answers it: those bytes were produced FOR a host, so the hand-off carries the fact the toolchain would otherwise have been asked for. Two directories and no rustc is refused rather than guessed, because picking one would exercise an artifact for a platform this machine is not
HOST_TARGET="$(rustc -vV 2>/dev/null | awk '/^host:/{print $2}')"
if [[ -z "$HOST_TARGET" && -n "$PREBUILT_DIR" ]]; then
    handed=()
    for d in "$PREBUILT_DIR"/*/; do
        [[ -d "$d" ]] || continue
        d="${d%/}"; handed+=("${d##*/}")
    done
    [[ ${#handed[@]} -eq 1 ]] \
        || blocked "no rustc on PATH and $PREBUILT_DIR carries ${#handed[@]} target directory/ies — which platform this run exercises cannot be told from that."
    HOST_TARGET="${handed[0]}"
fi
[[ -n "$HOST_TARGET" ]] || blocked "rustc reported no host target — the arm cannot tell which roster line this machine satisfies."
# spec: installer/README.md §The consumer smoke — the smoke steers its own roster at this host by default, so pack's all-targets demand is satisfied by construction rather than by every caller knowing to narrow it; a caller that already set the knob keeps it, which is what leaves the override branch a live path rather than a fixture-only one
if [[ -z "${GATE_SDK_NATIVE_TARGETS_FILE:-}" ]]; then
    GATE_SDK_NATIVE_TARGETS_FILE="$SCRATCH/host-targets.list"
    export GATE_SDK_NATIVE_TARGETS_FILE
    printf '%s\n' "$HOST_TARGET" > "$GATE_SDK_NATIVE_TARGETS_FILE" \
        || blocked "could not write the one-line host roster this smoke steers itself at."
    say "roster: no caller knob set, so this run is steered at $HOST_TARGET alone"
fi
ROSTER_FILE="$(native gate_native_targets_file)"
[[ "$ROSTER_FILE" == /* ]] || ROSTER_FILE="$REPO/$ROSTER_FILE"
mapfile -t ROSTER < <(native gate_native_targets)
[[ ${#ROSTER[@]} -gt 0 ]] \
    || blocked "no declared target at $ROSTER_FILE — there is no platform set to build for."
# spec: installer/README.md §The consumer smoke — pack refuses a roster target no leg built, so a host build satisfies --artifacts only while the roster is this host alone; the moment it declares a second target the smoke blocks here naming its own remedy rather than packing a payload with a hole in it
[[ ${#ROSTER[@]} -eq 1 && "${ROSTER[0]}" == "$HOST_TARGET" ]] \
    || blocked "the roster declares ${ROSTER[*]} and this host is $HOST_TARGET — a host build no longer satisfies pack's all-targets demand. Steer this smoke's pack at the host alone with GATE_SDK_NATIVE_TARGETS_FILE, or give the build leg a cross-compiling build."

if [[ -n "$PREBUILT_DIR" ]]; then
    # spec: gate-sdk/SPEC.md §Consumer payload — on the hand-off path the bytes and their sidecar are MOVED and never re-derived: a second sha256sum on this side is exactly what lets a published digest and an installed digest diverge while both look computed, so the artifact directory is adopted as it arrived
    ART="$PREBUILT_DIR/$HOST_TARGET"
    PACK_ARTIFACTS="$PREBUILT_DIR"
    [[ -f "$ART/$NATIVE_BIN" && -f "$ART/$NATIVE_BIN.sha256" ]] \
        || blocked "$PREBUILT_DIR carries no $NATIVE_BIN and .sha256 sidecar for $HOST_TARGET — the hand-off this run was pointed at is not a producer's output for this host."
    say "adopted $NATIVE_BIN for $HOST_TARGET from the hand-off, sidecar and all — nothing rebuilt, nothing rehashed"
else
    # spec: installer/README.md §The consumer smoke — the binary is built rather than fabricated with a matching digest: a stand-in would drive the same placement code while leaving the one thing most likely to break — the real build's digest agreeing with what init verifies before writing — covered by nothing
    ART="$SCRATCH/artifacts/$HOST_TARGET"
    PACK_ARTIFACTS="$SCRATCH/artifacts"
    mkdir -p "$ART"
    build_out="$(cd "$REPO" && bash gate-sdk/bin/build-native.sh 2>&1)" \
        || { printf '%s\n' "$build_out" >&2; blocked "the crate would not compile for $HOST_TARGET."; }
    BUILT="$REPO/$NATIVE_CRATE/target/release/$NATIVE_BIN"
    [[ -x "$BUILT" ]] || blocked "cargo reported success but there is no executable at $BUILT."
    cp "$BUILT" "$ART/$NATIVE_BIN" || fail "could not stage the built binary for packing"
    # spec: gate-sdk/SPEC.md §Consumer payload — the digest is emitted once, here, where the bytes are produced: pack re-verifies this sidecar and init verifies it again before writing, so both readers check a value neither of them computed
    ( cd "$ART" && sha256sum "$NATIVE_BIN" > "$NATIVE_BIN.sha256" ) \
        || fail "could not emit the digest sidecar beside the built binary"
    # spec: gate-sdk/SPEC.md §Consumer payload — the mode loss is PLANTED, because this path is the one that cannot produce it: a local build stages an already-executable binary, so without this line pack is never asked to restore a mode and the restoration is witnessed only on a host whose artifact crossed the real wire. After the sidecar deliberately: a mode is not a content write, and emitting the digest first is what says so. A red here is init failing to execute the payload artifact, exactly as an adopter would
    chmod 644 "$ART/$NATIVE_BIN" \
        || fail "could not plant the artifact transport's mode loss on the staged binary"
    [[ -z "$(git -C "$REPO" status --porcelain)" ]] \
        || fail "the build leg left the worktree dirty — the crate's output must land in gitignored build space and the artifact directory in the smoke's own scratch"
    say "built $NATIVE_BIN for $HOST_TARGET with the sidecar this leg emitted, staged non-executable so pack must restore the mode the transport drops"
fi

printf 'pack\n'
VERSION="$(git -C "$REPO" describe --tags --abbrev=0 2>/dev/null)"; VERSION="${VERSION#v}"
[[ -n "$VERSION" ]] || VERSION="0.0.0-smoke"
# spec: installer/README.md §The consumer smoke — --root "$REPO" is what makes the packed tree and the asserted tree the same tree by construction: $REPO is script-path-derived, so without it the current directory selects what gets packed and a run from a second checkout greens while asserting nothing about the tree under test
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --root "$REPO" \
    --version "$VERSION" --out "$SCRATCH" --artifacts "$PACK_ARTIFACTS" 2>&1)" \
    || { printf '%s\n' "$PACK_OUT" >&2; blocked "the pack step failed."; }
say "$(grep -m1 '^PACK:' <<<"$PACK_OUT")"
shopt -s nullglob
tarballs=("$SCRATCH"/*.tgz)
shopt -u nullglob
[[ ${#tarballs[@]} -eq 1 ]] || fail "expected exactly one tarball, found ${#tarballs[@]}"
TARBALL="${tarballs[0]}"

# spec: installer/README.md §The consumer smoke — steering the roster at this host alone removes pack's declared-target-with-no-artifact refusal from every ordinary path in this smoke, so the case is PLANTED rather than left with no witness: that refusal is the one reader here whose verdict reds on FINDING a target instead of on finding none, so a narrowing that removes its subject cannot be cleared by inspection the way the others can
PLANT_OUT="$SCRATCH/planted-pack"
mkdir -p "$PLANT_OUT" || fail "could not make the planted pack's output directory"
printf '%s\nother-%s\n' "$HOST_TARGET" "${HOST_TARGET#*-}" > "$SCRATCH/planted-targets.list" \
    || fail "could not write the planted roster"
plant_out="$(GATE_SDK_NATIVE_TARGETS_FILE="$SCRATCH/planted-targets.list" \
    INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --root "$REPO" \
    --version "$VERSION" --out "$PLANT_OUT" --artifacts "$PACK_ARTIFACTS" 2>&1)"; plant_rc=$?
[[ "$plant_rc" -ne 0 ]] \
    || fail "pack accepted a roster declaring a target the artifact directory has nothing for — a broken payload packed as a narrower one"
grep -q 'has no artifact directory' <<<"$plant_out" \
    || { printf '%s\n' "$plant_out" >&2; fail "pack refused the planted roster, but for something other than the declared target it had no artifact for"; }
say "pack: a declared target with no artifact directory refused, not packed narrower"

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

# spec: installer/README.md §The consumer smoke — this smoke sources nothing out of the package it is testing, which is its own `# no-port:` header's declared ground rather than a consequence of the relocation: it drives the packer and init as BLACK BOXES, and sourcing the implementation under test was always in tension with that. Each fact it needs is re-hosted by one rule — a fact that is DATA is read as data, and a fact that is a DERIVATION is read from the package through the binary's own wire. So profiles.list and payload/*/ are read below as the data they are, checkwright.lock with the jq this script's preflight already requires, an artifact's identity with the sha256sum it already requires, and the one derivation left with a reader here — which template a kit set's queue is seeded from — through the --install queue-source read op
# spec: installer/README.md §Profiles — reading the rows here rather than asking the installer for its kit set is what keeps every assertion below non-vacuous: each compares the installer's own answer against this independent second reading of the same data, where a smoke that asked the binary would be comparing a derivation against itself
# spec: installer/README.md §The manifest — the two consumer-layout constants are spelled once here as the OPERANDS of assertions that red on a wrong value, never as a silent second copy: a wrong GATES_DIR makes the manifest arm's "records no gates.list" refusal fire on the first install and the narrowing arm's doctor assertion name a registry doctor never reports, and a wrong QUEUE_FILE makes the queue arm refuse with "its kit set reads the queue file and init seeded none" on the first profile that is owed one, and a wrong PROFILE_DERIVED puts a name in PROFILES that init refuses as unknown before the lattice assertion that compares it against the computed maximum is ever reached
GATES_DIR=scripts
QUEUE_FILE=TASK-QUEUE.md
PROFILE_DERIVED=full
SEAM_FILES=("$GATES_DIR/gates.list" "$GATES_DIR/gate-sdk-config.sh")
# spec: installer/README.md §Profiles — every install's own registry, keyed by profile, so the monotonicity assertion deferred out of the profile-invariant arm has the gate-set derivation's own output to run over
declare -A REGISTRY=()

lock_own_file() {   # $1 = manifest path, $2 = the repo-relative path init writes -> that path when the manifest records it, empty when it does not
    jq -r --arg p "$2" '(.files // {}) | if has($p) then $p else "" end' "$1" 2>/dev/null
}
digest_of() {   # $1 = file -> its SHA-256 in hex; sha256sum is a preflight tool of this harness, so there is no hasher to resolve between
    sha256sum -- "$1" 2>/dev/null | cut -d' ' -f1
}
payload_kits() {   # -> every kit root the installed payload carries, in directory order
    local d
    shopt -s nullglob
    for d in "$PKG_ROOT"/payload/*/; do d="${d%/}"; printf '%s\n' "${d##*/}"; done
    shopt -u nullglob
}
profile_rows() {   # -> the '<profile><TAB><kit>' rows of profiles.list, comments and blanks dropped; a '#' anywhere ends a line, which is what lets a row carry a trailing note
    local line
    [[ -f "$PKG_ROOT/profiles.list" ]] || return 0
    while IFS= read -r line; do
        line="${line%%#*}"
        [[ -n "${line//[[:space:]]/}" ]] || continue
        printf '%s\n' "$line"
    done < "$PKG_ROOT/profiles.list"
}
profile_names() {   # -> every selectable profile, the payload-derived one last
    local seen="" p
    while IFS=$'\t' read -r p _; do
        [[ -n "$p" && "$seen" != *"|$p|"* ]] || continue
        seen="$seen|$p|"
        printf '%s\n' "$p"
    done < <(profile_rows)
    printf '%s\n' "$PROFILE_DERIVED"
}
profile_kits() {   # $1 = profile -> its kit set in payload order, empty when the profile names none
    local want="$1" p k members="" kit
    if [[ "$want" == "$PROFILE_DERIVED" ]]; then payload_kits; return 0; fi
    while IFS=$'\t' read -r p k; do
        [[ "$p" == "$want" ]] && members="$members|$k|"
    done < <(profile_rows)
    [[ -n "$members" ]] || return 0
    # spec: installer/README.md §Profiles — emit in payload order so a roster's line order never decides install order
    while IFS= read -r kit; do
        [[ "$members" == *"|$kit|"* ]] && printf '%s\n' "$kit"
    done < <(payload_kits)
}
# spec: installer/README.md §Profiles — the order is derived from set inclusion over the kit rosters and never declared beside them: a declared parent and the kit sets could disagree, and then two surfaces would assert the containment again. The profiles form a lattice rather than a chain, so a pair comparing in neither direction is legitimate and simply absent from this output
profile_order() {   # -> one '<a><TAB><b>' line per ordered pair of distinct profiles whose kit set is contained in the other's
    local a b sub sup k ok
    local -a names
    mapfile -t names < <(profile_names)
    for a in "${names[@]}"; do
        sub="$(profile_kits "$a")"
        for b in "${names[@]}"; do
            [[ "$a" == "$b" ]] && continue
            sup="$(profile_kits "$b")"
            ok=1
            while IFS= read -r k; do
                [[ -n "$k" ]] || continue
                grep -qxF "$k" <<<"$sup" || { ok=0; break; }
            done <<<"$sub"
            (( ok )) && printf '%s\t%s\n' "$a" "$b"
        done
    done
}

# spec: installer/README.md §Profiles — the invariant is asserted against the installed payload rather than the source tree, so it holds for what an adopter actually receives
printf 'profile invariant\n'
mapfile -t PAYLOAD_KITS < <(payload_kits)
[[ ${#PAYLOAD_KITS[@]} -gt 0 ]] || fail "the installed payload carries no kit"
mapfile -t PROFILES < <(profile_names)
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
    mapfile -t members < <(profile_kits "$p")
    [[ ${#members[@]} -gt 0 ]] || fail "profile '$p' resolves to no kit in the payload"
    for k in "${members[@]}"; do
        resolves "$k" || fail "profile '$p' names $k, which the payload does not carry"
    done
done

# spec: installer/README.md §Profiles — assertions 2 and 3: the lattice is bounded, so it has exactly one profile below every other and exactly one above every other, and the one above is the payload-derived profile by construction. A profile comparable to nothing, a second incomparable maximum, and two profiles resolving to the same kit set are all reds here — which is the contract the deleted "at most three profiles" bound was standing in for, stated as a shape instead of a count
mapfile -t ORDER < <(profile_order)
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

# spec: installer/README.md §Profiles — assertion 4 is the one assertion this arm cannot make, and it is DEFERRED to after the per-profile loop rather than dropped. The promise is not that a bigger profile vendors more directories but that moving up only ever adds to the battery, and kit-set containment stops implying gate-set containment the moment a roster varies by profile. Unlike the three above, the gate set is a DERIVATION the installer owns rather than data the package carries, so the smoke reads it where that derivation's own output lands — the gates.list each install writes — instead of unioning the per-kit recipes a second time here

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

# spec: installer/README.md §The consumer smoke — the shape test's independently observable parts, computed once here because the report line and the refusal line both read them and a second producer could disagree with the first; the class half is a GLOB-BRACKET parameter expansion and never the ERE the composite uses, so the two matchers stay independent implementations of "is this character acceptable" and a disagreement between them on one variable is observable instead of absorbed, and the composite is printed as the observation it is rather than consulted as a decider
shape_verdict() {   # $1 = the value -> 'len40=<yes|no> class=<clean|dirty[...]> shape=<pass|fail>'
    local v="$1" leftover before len40 class shape
    leftover="${v//[0-9a-f]/}"
    before="${v%%[!0-9a-f]*}"
    [[ "${#v}" -eq 40 ]] && len40=yes || len40=no
    [[ -z "$leftover" ]] && class=clean \
        || class="dirty[residue=$(printf '%q' "$leftover") first=${#before}]"
    [[ "$v" =~ ^[0-9a-f]{40}$ ]] && shape=pass || shape=fail
    printf 'len40=%s class=%s shape=%s' "$len40" "$class" "$shape"
}

# spec: installer/README.md §The consumer smoke — the two renderings of one value printed together, because their DISAGREEMENT is itself the finding and one of the two cannot lie about bytes: %q is a shell-quoting renderer whose output coincides with byte-exactness only for the class that motivated it, so the octet dump carries the byte claim and is produced through no construct that could normalize — no echo, whose escape handling is shell-dependent, and no re-quoting. A dump that comes back empty for a value that is not is SAID rather than printed blank, since a blank byte line reads as an empty value and this is the one line that must not mislead about bytes
value_probe() {   # $1 = the value to render and judge
    local v="$1" dump
    dump="$(printf '%s' "$v" | od -An -tx1 | tr '\n' ' ' | tr -s ' ')"
    dump="${dump# }"; dump="${dump% }"
    [[ -n "$v" && -z "$dump" ]] && dump='<no octet dump: od produced nothing on this host>'
    printf '            bytes  %s\n' "$(printf '%q' "$v")"
    printf '            len    %s\n' "${#v}"
    printf '            octets %s\n' "$dump"
    printf '            tests  %s\n' "$(shape_verdict "$v")"
}

# spec: installer/README.md §The consumer smoke — one RE-READ value of the manifest report's six-value block: the plain value, the two renderings and the decomposed shape verdict value_probe owns, the call that produced it rather than a hand-written description of the call, and that call's standard error so a refusal is named instead of showing as an empty value
hash_probe() {   # $1 = label, $2.. = the command whose stdout is the value
    local label="$1"; shift
    local val err e="$SCRATCH/manifest-report.err"
    val="$("$@" 2>"$e")"
    err="$(<"$e")"
    printf '    %-7s %s\n' "$label" "${val:-<empty>}"
    value_probe "$val"
    printf '            call   %s\n' "$*"
    [[ -z "$err" ]] || printf '            stderr %s\n' "$err"
}

# spec: installer/README.md §The consumer smoke — one HELD value of that block, printed out of the variable the loop assigned instead of by re-running the call that produced it, which is what lets a value that reaches the comparison mangled and re-reads clean still be seen; its call line states WHERE THE VALUE WAS READ FROM and asserts nothing further, since a command a reader could re-run is exactly what this print refuses to be and an identity between this value and the one the verdict computed on is a claim this print cannot check — the coincidence outcome above checks it and states it there
held_probe() {   # $1 = label, $2 = the value the loop assigned, $3 = the variable and the read it came off
    printf '    %-7s %s\n' "$1" "${2:-<empty>}"
    value_probe "$2"
    printf '            call   %s\n' "$3"
}

# spec: installer/README.md §The consumer smoke — one attribute lookup of the manifest report, printed as three distinguishable outcomes rather than one blob: the attributes a repository reports, git's refusal where the path is outside the repository asked and no attribute chain is reachable from it, and a clean silence where the path simply carries none
attr_probe() {   # $1 = the repository to ask, $2 = which repository that is, $3 = the path as that repository spells it
    local out err e="$SCRATCH/manifest-report.err"
    out="$(git -C "$1" check-attr -a -- "$3" 2>"$e")"
    err="$(<"$e")"
    printf '    %s, %s:\n' "$3" "$2"
    [[ -z "$out" ]] || printf '%s\n' "$out"
    [[ -z "$err" ]] || printf '      refused: %s\n' "$err"
    [[ -n "$out" || -n "$err" ]] || printf '      <no attribute reported>\n'
}

# spec: installer/README.md §The consumer smoke — the refusal names its operand AND what the decomposition saw of it, because "the operand is not 40 lowercase hex" is a claim about the value while what the arm observed is that its shape test refused the value, and where len40 and class both read clean those are different statements and only the second is true; it reads the witness tuple with the same three-field split manifest_report uses rather than minting a second grammar, and both operands failing is a distinct reading from either alone, so it is spelled out rather than collapsed to the first
malformed_operands() {   # $1 = the '<path><TAB><want><TAB><got>' witness the shape test recorded -> the refused operand(s), each with its decomposed verdict
    local rest="${1#*$'\t'}" w g which=""
    w="${rest%%$'\t'*}"; g="${rest#*$'\t'}"
    [[ "$w" =~ ^[0-9a-f]{40}$ ]] || which="want ($(shape_verdict "$w"))"
    [[ "$g" =~ ^[0-9a-f]{40}$ ]] || which="${which:+$which and }got ($(shape_verdict "$g"))"
    printf '%s' "$which"
}

# spec: installer/README.md §The consumer smoke — the manifest arm's failure report, in the arm because this script mktemps its scratch under a cleanup trap and nothing after the run can open the disagreeing consumer; it is a straight-line sequence of prints with no branch that can change the verdict the caller goes on to fail with, every value it prints has a named reader in that section's truth table, and the entry that earns the exit-2 verdict is carried in as an operand so the sample set is required to contain it BY ITS BYTES and not only by its path
manifest_report() {   # $1 = profile, $2 = consumer dir, $3 = its manifest, $4 = mismatch count, $5 = checked count, $6 = the malformed witness tuple, empty where no disagreement failed the shape test, $7 = how many disagreements failed it, $8 = the first raw line of the .files stream, $9 = the raw line of the first disagreeing entry, empty where no hash disagreement was found, $10.. = one '<path><TAB><the want the loop held><TAB><the got the loop held>' per hash disagreement, in the order the loop found them
    local profile="$1" C="$2" LOCK="$3" mismatch="$4" checked="$5" mal_first="$6" mal_n="$7" raw_first="$8" raw_bad="$9"; shift 9
    local -a bad=("$@") samples=() roles=()
    local target digest_want art seam entry p w g rest r out found pathmate i role

    printf '  == manifest report: %s, %s of %s entries disagree ==\n' "$profile" "$mismatch" "$checked"
    printf '  read the values below against the truth table in installer/README.md §The consumer smoke\n'
    # spec: installer/README.md §The consumer smoke — the two RAW STREAM LINES, carried in as named operands rather than re-read, since a value re-read is a second observation and cannot testify about the first; the first line is unconditional on any comparison, which is what makes it the cheapest possible statement about the channel, and the disagreeing entry's line is the one want's own decomposition is read against. Their shape verdict is not a finding — a raw line is a path, a tab and a hash, so it fails the hash shape test by construction — and the octet dump is what they are printed for
    printf '  -- the raw manifest stream, as the loop read it and before any split\n'
    held_probe stream1 "$raw_first" 'the first line the .files stream delivered, whatever it is, held before the split'
    [[ -z "$raw_bad" ]] \
        || held_probe badline "$raw_bad" 'the raw line of the first disagreeing entry, held before the split that produced its want'
    if [[ ${#bad[@]} -eq 0 ]]; then
        printf '  every disagreement is a path the manifest names and the tree does not hold, so there is no hash to compare\n'
        return 0
    fi
    # spec: installer/README.md §The consumer smoke — the count is what says whether the one printed witness row is representative: one malformed entry out of hundreds is a statement about that path, all of them a statement about the capture step every entry runs through, and those two readings send a reader to different places
    printf '  %s of those %s carry an operand that is not 40 lowercase hex; the samples below are the first disagreeing path, the artifact row and the first such entry, deduplicated\n' "$mal_n" "$mismatch"
    samples=("${bad[0]}"); roles=("")

    target="$(jq -r '.artifact.target // ""' "$LOCK")"
    art=""
    if [[ -n "$target" ]]; then
        seam="$(lock_own_file "$LOCK" "$GATES_DIR/gate-sdk-config.sh")"
        [[ -n "$seam" && -f "$C/$seam" ]] \
            && art="$(sed -n 's/^GATE_SDK_NATIVE_BIN=//p' "$C/$seam" | head -n1)"
    fi
    if [[ -z "$target" ]]; then
        printf '  the manifest records no artifact key, so the discriminating binary sample is absent from this payload\n'
    elif [[ -z "$art" ]]; then
        printf '  the manifest records artifact %s but no config seam names its path, so the binary sample is unresolved\n' "$target"
    else
        found=""
        for entry in "${bad[@]}"; do [[ "${entry%%$'\t'*}" == "$art" ]] && { found="$entry"; break; }; done
        if [[ -z "$found" ]]; then
            printf '  the artifact row %s is not in the disagreeing set, so the sample is the first path alone\n' "$art"
        elif [[ "$found" == "${bad[0]}" ]]; then
            printf '  the artifact row %s is also the first disagreeing path, so the two samples coincide\n' "$art"
        else
            samples+=("$found"); roles+=("")
        fi
    fi

    # spec: installer/README.md §The consumer smoke — the verdict's own row joins the sample set last, deduplicated ON THE WHOLE TUPLE, so no run can exit 2 on an entry whose BYTES this report did not print; the coincidence claim is a check this report performs and prints in three outcomes, because a path-equal pair whose bytes differ is two decompositions of one recorded entry disagreeing — a finding about this harness — and collapsing it into the coincidence sentence asserts an identity nothing checked
    if [[ -z "$mal_first" ]]; then
        printf '  no disagreeing entry failed the operand shape test, so the verdict below is the manifest one and the samples are the %s path(s) above\n' "${#samples[@]}"
    else
        found=""; pathmate=""
        for entry in "${samples[@]}"; do
            [[ "$entry" == "$mal_first" ]] && { found="$entry"; break; }
            [[ -z "$pathmate" && "${entry%%$'\t'*}" == "${mal_first%%$'\t'*}" ]] && pathmate="$entry"
        done
        if [[ -n "$found" ]]; then
            printf '  the witness row %s is one of the samples already chosen, byte for byte, so the verdict row and that sample coincide\n' "${mal_first%%$'\t'*}"
        else
            samples+=("$mal_first"); roles+=(" — the witness row the exit-2 verdict below is computed from")
            [[ -z "$pathmate" ]] \
                || printf '  the witness row %s is path-equal to a sample already chosen and its bytes DIFFER, so both blocks are printed below: two decompositions of one recorded entry disagree, which is a statement about this harness and not about the consumer tree, and no reading of the consumer tree may be taken from this run manifest arm\n' "${mal_first%%$'\t'*}"
        fi
    fi

    for i in "${!samples[@]}"; do
        entry="${samples[$i]}"; role="${roles[$i]}"
        p="${entry%%$'\t'*}"; rest="${entry#*$'\t'}"; w="${rest%%$'\t'*}"; g="${rest#*$'\t'}"
        printf '  -- %s%s\n' "$p" "$role"
        held_probe want "$w" 'the want variable, as the manifest loop read left it and carried it out on the failure branch'
        held_probe got "$g" 'the got variable, as the git hash-object command substitution left it in that same loop'
        hash_probe reread git hash-object -- "$C/$p"
        hash_probe own git -C "$C" hash-object -- "$p"
        hash_probe raw git hash-object --no-filters -- "$C/$p"
        # spec: installer/README.md §The consumer smoke — want is the one value in the block with no independent producer, so its control reads the same key out of the same lock through a channel that shares nothing with the manifest pipeline: no jq line render, no tab, no read splitting, which is what makes want != wantalt a statement about that pipeline and want == wantalt an exoneration of it
        hash_probe wantalt jq -r --arg p "$p" '.files[$p]' "$LOCK"
    done

    printf '  -- the consumer worktree, the witness for a tree that has diverged from what init committed\n'
    git -C "$C" status --porcelain 2>&1 | head -n 40
    git -C "$C" log -1 --stat 2>&1 | head -n 40
    printf '  -- core.autocrlf, core.eol and core.safecrlf with their origins, in both repositories\n'
    for r in "$C" "$REPO"; do
        printf '    in %s\n' "$r"
        out="$(git -C "$r" config --list --show-origin 2>&1 | grep -iE 'core\.(autocrlf|eol|safecrlf)=')"
        printf '%s\n' "${out:-      <none of the three is set>}"
    done
    printf '  -- git check-attr -a for each sampled path, in both repositories, since an attribute reaches a path the config does not\n'
    for entry in "${samples[@]}"; do
        p="${entry%%$'\t'*}"
        attr_probe "$C" "in the consumer" "$p"
        attr_probe "$REPO" "in the smoke repository" "$C/$p"
    done

    if [[ -n "$target" && -n "$art" && -f "$C/$art" ]]; then
        digest_want="$(jq -r '.artifact.digest // ""' "$LOCK")"
        printf '  -- the artifact digest, a control on the content question alone: SHA-256 is taken by no git filter and in no repository context\n'
        printf '    subject    %s (%s)\n' "$art" "$target"
        printf '    recorded   %s\n' "$digest_want"
        printf '    recomputed %s\n' "$(digest_of "$C/$art")"
        printf '    equal means the artifact bytes are exactly the bytes init published, which speaks for this path and no other\n'
    fi
    return 0
}

# spec: installer/README.md §init — the follow-up block's grammar is the operand, so this arm parses the pair out of what init PRINTED rather than comparing it against a second copy a rename would have to be remembered to move; the cheap form of this assertion is exactly the defect it exists to close
# spec: installer/README.md §The consumer smoke — the flag probe runs in a throwaway copy of the consumer because the probe RUNS the printed command, and the live flag wires the clone's hooksPath: executing it in the consumer itself would put a pre-commit hook in front of every later arm's commit, including the value arm's planted defect, so the arm would decide what it is supposed to observe
# spec: installer/README.md §The gate binary — the flag probe is unconditional here, where it was once skipped on a leg whose install had no binary for the front-end to dispatch a flag to. Selection has one success path, so there is no such install left to make: every consumer this arm is handed was written by a verb that ran, and a verb runs only past a verified artifact
assert_followups() {   # $1 = profile, $2 = the consumer init just wrote, $3 = init's captured output
    local profile="$1" C="$2" out="$3" line target flag sentinel ctl expect probe sandbox i ti
    local -a cmds=() toks=() ctl_argv=()

    mapfile -t cmds < <(awk '$0 == "next:" { b = 1; next } b && /^[[:space:]]+[^[:space:]]/ { sub(/#.*$/, ""); sub(/^[[:space:]]+/, ""); sub(/[[:space:]]+$/, ""); print; next } b { exit }' <<<"$out")
    # spec: installer/README.md §init — zero commands extracted is a red and never a skip: an assertion over an empty set passes vacuously, so a reflowed banner would otherwise turn full coverage into silent zero coverage, and a block this arm cannot read at all is a failure of its own construction rather than a finding about the payload
    [[ ${#cmds[@]} -gt 0 ]] \
        || { printf '%s\n' "$out" >&2; blocked "$profile: init printed no follow-up block matching the grammar installer/README.md §init states, so this arm has nothing to assert over."; }

    sandbox="$SCRATCH/followup-$profile"
    rm -rf "$sandbox"
    cp -Rp "$C" "$sandbox" || fail "$profile: could not copy the consumer for the follow-up probe"

    for line in "${cmds[@]}"; do
        read -r -a toks <<<"$line"
        ti=-1
        for i in "${!toks[@]}"; do [[ "${toks[$i]}" == */* ]] && { ti=$i; break; }; done
        [[ "$ti" -ge 0 ]] \
            || blocked "$profile: the follow-up command '$line' names no repo-relative script path, so this arm cannot tell which file init is telling the adopter to run."
        target="${toks[$ti]}"
        [[ -f "$C/$target" && -x "$C/$target" ]] \
            || fail "$profile: init told the adopter to run '$line', and $target is not an executable file in the tree init just wrote"
        for (( i = ti + 1; i < ${#toks[@]}; i++ )); do
            flag="${toks[$i]}"
            [[ "$flag" == -* ]] || continue
            # spec: installer/README.md §The consumer smoke — the negative control is what keeps the flag assertion from passing vacuously: it establishes that this target refuses an unknown flag BY NAME, and the refusal it prints is what the positive probe is then measured against, so no refusal string is spelled here
            sentinel="$flag--checkwright-smoke-unknown"
            ctl_argv=("${toks[@]}"); ctl_argv[$i]="$sentinel"
            ctl="$( cd "$sandbox" && PATH="$RUN_PATH" "${ctl_argv[@]}" 2>&1 | grep -m1 -F -- "$sentinel" )"
            [[ -n "$ctl" ]] \
                || blocked "$profile: $target does not refuse '$sentinel' by name, so this arm cannot tell a flag it accepts from one it rejects."
            expect="${ctl//"$sentinel"/"$flag"}"
            probe="$( cd "$sandbox" && PATH="$RUN_PATH" "${toks[@]}" 2>&1 )"
            [[ "$probe" != *"$expect"* ]] \
                || fail "$profile: init told the adopter to run '$line', and $target refuses that flag — $expect"
        done
    done

    rm -rf "$sandbox"
    say "follow-up: ${#cmds[@]} printed command(s), each resolving in the install with every flag it names accepted"
}

# spec: installer/README.md §The consumer smoke — one encoding of the post-conditions, read by both transports, so the two arms cannot drift into asserting different things about the same install; ENTRY is the invocation of the installed entry point and RUN_PATH the PATH every step runs under, which is what lets the download arm mask node/npm without a second copy of the assertions
# spec: installer/README.md §The gate binary — the battery expectation is no longer a parameter of this helper and the alternative it once carried is no longer a branch: selection has one success path, so an install that ran at all placed a verified artifact and a green battery is the only post-condition an install can earn. The refusals are asserted where they now occur — at the bootstrap, before any verb — by the artifact-less leg and the artifact arm
assert_install() {   # $1 = profile, $2 = scratch consumer dir
    local profile="$1" C="$2" out rc before after LOCK mismatch checked malformed_first malformed_n raw_first raw_bad path want got target seam bin list k m line field crlf n_omitted q_seam q_bin queue_src files_raw
    local -a bad_hash=() lock_kits=() want_kits=()

    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" init --profile "$profile" 2>&1 )" \
        || { printf '%s\n' "$out" >&2; fail "init failed for the $profile profile"; }
    say "init: $(grep -m1 '^INIT:' <<<"$out")"
    # spec: installer/README.md §init — the arm rides THIS init invocation and never the idempotent re-run below, whose no-op branch prints no follow-up block at all: an arm placed there would assert over an empty block on every profile and pass by vacuity, which is the hole the emptiness assertion exists to close arriving through the back door
    assert_followups "$profile" "$C" "$out"

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
    mismatch=0; checked=0; malformed_first=""; malformed_n=0; raw_first=""; raw_bad=""; crlf=0
    # spec: installer/README.md §The consumer smoke — the line is read WHOLE and split by parameter expansion, so the bytes the stream delivered exist in a variable the report can print: reading with IFS=$'\t' would make the split the same operation that consumes the evidence, and tab is IFS WHITESPACE, so that read also collapses tab runs and strips trailing ones — normalizing an anomalous line out of existence before anything can observe it. On the two-field, single-tab line the producer emits the two agree byte for byte, CR included, so this buys the witness and moves no value
    # spec: installer/README.md §The consumer smoke — the capture stays a command substitution and the line TERMINATOR is the read's to own, because this is the harness's only multi-line jq read and a terminator is host-dependent: a single-value control that comes back clean carries only the one terminator its own capture already consumes, so it discriminates nothing about the producer and reading it as a channel witness is what sent the previous repair at the channel
    files_raw="$(jq -r '.files | to_entries[] | "\(.key)\t\(.value)"' "$LOCK")"
    if [[ -n "$files_raw" ]]; then
        while IFS= read -r line; do
            checked=$((checked + 1))
            [[ -n "$raw_first" ]] || raw_first="$line"
            # spec: installer/README.md §The consumer smoke — a trailing CR is dropped as the second byte of a host's line terminator, and never silently: the raw line stays unstripped for the two evidence operands, exactly one strip is taken per line so a doubled one still shows, and the count is DECLARED below, which is what keeps a value that genuinely ended in a CR visible as a count instead of vanishing at the split
            field="${line%$'\r'}"; [[ "$field" == "$line" ]] || crlf=$((crlf + 1))
            path="${field%%$'\t'*}"; want="${field#*$'\t'}"
            [[ -f "$C/$path" ]] || { echo "  manifest names a file that is not there: $path"; mismatch=$((mismatch + 1)); continue; }
            got="$(git hash-object -- "$C/$path")"
            # spec: installer/README.md §The consumer smoke — the shape test sits on the failure branch beside the tuple it diagnoses, over BOTH operands, because a value that is not a hash makes the disagreement a statement about this harness rather than about the consumer's tree, and the verdict below has to be able to say which; it records the FIRST offending entry as a whole tuple in bad_hash's own spelling and counts every one, since a run-wide flag recording only THAT something tripped it hands the reader a verdict and withholds its subject, and it keeps the RAW LINE of the first disagreeing entry beside that tuple because a value re-read is a second observation and cannot testify about the first
            [[ "$got" == "$want" ]] \
                || { echo "  manifest hash disagrees with the tree: $path"; mismatch=$((mismatch + 1))
                     [[ ${#bad_hash[@]} -gt 0 ]] || raw_bad="$line"
                     bad_hash+=("$path"$'\t'"$want"$'\t'"$got")
                     [[ "$want" =~ ^[0-9a-f]{40}$ && "$got" =~ ^[0-9a-f]{40}$ ]] \
                         || { malformed_n=$((malformed_n + 1)); [[ -n "$malformed_first" ]] || malformed_first="$path"$'\t'"$want"$'\t'"$got"; }; }
        done <<< "$files_raw"
    fi
    # spec: installer/README.md §The consumer smoke — the strip's declaration, printed on the green path as well as the red because that is the only path a working strip ever takes: a count equal to the entries the capture did not consume a terminator for reads as the host's line ending, and any other count is the anomaly this arm refuses to normalize away
    [[ "$crlf" -eq 0 ]] \
        || say "manifest: the .files stream delivered $crlf of $checked line(s) ending in a carriage return, each dropped as a line terminator before the split; the raw operands any report below prints are held unstripped"
    # spec: installer/README.md §The consumer smoke — the report runs while the disagreeing consumer is still on disk and immediately before the verdict, so a leg that reds here says what it found rather than only how many, and it takes the witness and the two raw stream lines as named operands so the sample set contains the row the verdict below is about and the report can state what the channel delivered
    [[ "$mismatch" -eq 0 ]] \
        || manifest_report "$profile" "$C" "$LOCK" "$mismatch" "$checked" "$malformed_first" "$malformed_n" "$raw_first" "$raw_bad" ${bad_hash[@]+"${bad_hash[@]}"}
    # spec: installer/README.md §The consumer smoke — the refused operand is reported AFTER the report, not at the first bad value, because the report is exactly what diagnoses it and the report is now required to contain the entry this refusal names, so pointing at it is a direction rather than a hope; the line states the shape test's VERDICT and the decomposition beside it rather than declaring the operand malformed, since a reader who never scrolls up takes the last line away and a matcher refusing a well-formed hash would make that claim false; and it refuses at the harness-precondition code on either ground, since neither a mangled operand nor a matcher that will not accept a hash says anything about whether the tree matches what init recorded
    [[ "$mismatch" -eq 0 || -z "$malformed_first" ]] \
        || blocked "$profile: on ${malformed_first%%$'\t'*} the shape test refused $(malformed_operands "$malformed_first"), and it refused $malformed_n of $mismatch disagreeing entries. The report above samples that entry and prints each value's octet dump beside its shell-quoted rendering. Read the decomposition in the parentheses: len40=no or class=dirty says the operand is not a hash, while len40=yes with class=clean says this harness's matcher refused one. That is this harness's own precondition either way, not a finding about the consumer."
    [[ "$mismatch" -eq 0 ]] || fail "$profile: $mismatch of $checked manifest entries disagree with the tree"
    [[ "$checked" -gt 0 ]] || fail "$profile: the manifest records no file"
    read_stream lock_kits "manifest kits" < <(jq -r '.kits[]' "$LOCK")
    mapfile -t want_kits < <(profile_kits "$profile")
    [[ "${lock_kits[*]}" == "${want_kits[*]}" ]] \
        || fail "$profile: manifest kits (${lock_kits[*]}) differ from the profile roster (${want_kits[*]})"
    say "manifest: $checked file(s) agree with the tree, ${#lock_kits[@]} kit(s) recorded"

    # spec: installer/README.md §What init seeds — the queue file is the one surface init seeds whose *content* has a format contract, and whether a profile gets one at all is a property of its kit set: the arm asks the same resolver init used rather than naming a profile, and asserts the same contract for both of its outcomes, since which arm wrote the file is exactly what the selection rule makes irrelevant. The section floor is on-surface, so it is absent from the battery just asserted above and is run here out of the installed payload — the only spelling that reaches a profile whose kit set reads the queue and carries no queue-kit
    # spec: installer/README.md §The install boundary — the resolver is reached through the --install queue-source read op rather than by sourcing the module that owns it: one derivation with two readers, and the smoke now reads it ACROSS the package boundary instead of inside it. The wire is the family's — nonempty stdout is the whole answer — and the op is invoked through $CW, which is the bootstrap of the package $PKG_ROOT names, so the arm reads the package it resolved its kit set from whichever transport's entry point this leg is exercising
    queue_src="$( "$CW" --install queue-source --payload "$PKG_ROOT/payload" \
        --kits "$(IFS=,; printf '%s' "${want_kits[*]}")" 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] \
        || { printf '%s\n' "$queue_src" >&2; fail "$profile: the --install queue-source read op exited $rc, so the queue post-condition has no resolver to assert against"; }
    if [[ -n "$queue_src" ]]; then
        [[ -f "$C/$QUEUE_FILE" ]] \
            || fail "$profile: its kit set reads the queue file and init seeded none"
        # spec: installer/README.md §The gate binary — the section floor ported to the binary substrate, so this arm resolves it through gate_command out of the payload exactly as a battery would, rather than naming a script path that no longer exists; the knobs resolve against the consumer's own config because the dispatch runs with the consumer as cwd. There is no omitted-and-declared branch to fall into: every install that reaches here placed a verified artifact, because the bootstrap refuses rather than proceeding without one
        q_seam="$(lock_own_file "$LOCK" "$GATES_DIR/gate-sdk-config.sh")"
        q_bin=""
        [[ -n "$q_seam" && -f "$C/$q_seam" ]] \
            && q_bin="$(sed -n 's/^GATE_SDK_NATIVE_BIN=//p' "$C/$q_seam" | head -n1)"
        [[ -n "$q_bin" && -x "$C/$q_bin" ]] \
            || fail "$profile: the seam names no executable gate binary at '${q_bin:-<unset>}', so the section floor has nothing to dispatch to on an install the bootstrap let proceed"
        out="$( cd "$C" && PATH="$RUN_PATH" GATE_SDK_NATIVE_BIN="$C/$q_bin" bash -c '
            source "$1/payload/gate-sdk/lib/gate.sh"
            mapfile -t argv < <(gate_command check-queue-sections "$1/payload/queue-kit/checks") || exit 2
            [[ ${#argv[@]} -gt 0 ]] || exit 2
            exec "${argv[@]}" "$2"' _ "$PKG_ROOT" "$QUEUE_FILE" 2>&1 )"; rc=$?
        [[ "$rc" -eq 0 ]] \
            || { printf '%s\n' "$out" >&2; fail "$profile: the queue file init seeded does not satisfy the section contract queue-kit's own gate reads"; }
        say "queue: $(grep -m1 '^QUEUE-SECTIONS:' <<<"$out")"
    else
        [[ ! -f "$C/$QUEUE_FILE" ]] \
            || fail "$profile: no kit in its set reads the queue file, yet init seeded one"
        say "queue: none seeded, and none is owed to this kit set"
    fi

    # spec: installer/README.md §init — idempotence is a property of the tree, so the assertion is on the tree object and not on what the re-run printed
    before="$(git -C "$C" rev-parse 'HEAD^{tree}')"
    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" init 2>&1 )" \
        || { printf '%s\n' "$out" >&2; fail "$profile: the idempotent re-run of init failed"; }
    after="$(git -C "$C" rev-parse 'HEAD^{tree}')"
    [[ "$before" == "$after" ]] || fail "$profile: re-running init changed the tree — the install is not idempotent"
    [[ -z "$(git -C "$C" status --porcelain)" ]] || fail "$profile: the re-run left the worktree dirty"
    say "re-run: tree unchanged"

    # spec: installer/README.md §The gate binary — selection keeps three outcomes and only ONE of them proceeds, so an install that reached this line placed a verified artifact by construction: the branch that recorded no artifact and declared its omissions is not narrowed here, it is unreachable, because the bootstrap refuses before any verb runs. The three outcomes are still asserted, in the artifact arm, where the two refusals are driven directly and told apart by message and remedy
    target="$(jq -r '.artifact.target // ""' "$LOCK")"
    seam="$(lock_own_file "$LOCK" "$GATES_DIR/gate-sdk-config.sh")"
    list="$(lock_own_file "$LOCK" "$GATES_DIR/gates.list")"
    [[ -n "$list" ]] || fail "$profile: the manifest records no gates.list"
    # spec: installer/README.md §Profiles — the registry this install wrote, kept for the monotonicity assertion the profile-invariant arm deferred: comment and blank lines dropped, so what is compared is the live membership an adopter of this profile actually receives
    REGISTRY["$profile"]="$(grep -Ev '^[[:space:]]*(#|$)' "$C/$list" | LC_ALL=C sort -u)"
    [[ -n "${REGISTRY[$profile]}" ]] \
        || fail "$profile: the registry init wrote declares no member at all — every gate this profile's kits register went missing rather than being installed"
    n_omitted="$(grep -c '^# omitted:' "$C/$list")"
    [[ "$n_omitted" -eq 0 ]] \
        || fail "$profile: the registry declares $n_omitted omitted member(s) on an install that placed a verified artifact — the installer writes no omission record at all now, so this is a producer that outlived the outcome it recorded"
    [[ -n "$target" ]] \
        || fail "$profile: the manifest records no artifact, yet the bootstrap ran a verb — the only outcome that proceeds is a verified artifact, so an install with none is a refusal that did not refuse"
    [[ -n "$seam" && -f "$C/$seam" ]] || fail "$profile: an artifact is recorded but no gate-sdk config seam names its path"
    bin="$(sed -n 's/^GATE_SDK_NATIVE_BIN=//p' "$C/$seam" | head -n1)"
    [[ -n "$bin" && -x "$C/$bin" ]] || fail "$profile: no executable gate binary at '${bin:-<unset>}'"
    [[ "$(digest_of "$C/$bin")" == "$(jq -r '.artifact.digest' "$LOCK")" ]] \
        || fail "$profile: the installed gate binary does not match the digest the manifest recorded"
    # spec: installer/README.md §The manifest — the binary and the seam are files init wrote, so both are on the roster it records: a path init created and did not record reads as "never installed" next run, which is the reading that lets the following install write straight through it
    for path in "$bin" "$seam"; do
        [[ "$(jq -r --arg f "$path" '.files | has($f)' "$LOCK")" == "true" ]] \
            || fail "$profile: init wrote $path on the placement path but the manifest roster does not record it"
    done
    say "artifact: $target verified in place at $bin, recorded with the seam, $(grep -c . <<<"${REGISTRY[$profile]}") live member(s) and nothing omitted"

    out="$( cd "$C" && PATH="$RUN_PATH" "${ENTRY[@]}" doctor 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "$profile: doctor exited $rc inside the installed consumer"; }
    grep -q "^  profile      $profile\$" <<<"$out" || fail "$profile: doctor did not report the installed profile"
    say "doctor: clean, reports the installed profile"
}

# spec: installer/README.md §The consumer smoke — the value post-condition: an install that is green, idempotent and reversible is still worth nothing if it never catches anything, so each profile's battery is put in front of one real defect in adopter-authored prose — a mistyped relative link in a README, on a consumer whose own content is markdown and nothing else. Which gate delivers the red is deliberately not asserted: naming one would be a second roster to maintain beside the registry init writes, and the claim is about the battery rather than about a member of it. The arm restores the consumer to the commit it found, so the reversal that follows still asserts against the tree init wrote
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

# spec: installer/README.md §Profiles — assertion 4, deferred out of the profile-invariant arm and made here over the registries the installs above actually wrote. The gate set is the installer's own derivation, so this reads its OUTPUT rather than unioning the per-kit recipes a second time, and the claim strengthens with the move: monotonicity now holds over what an adopter of each profile receives rather than over what a recipe said they would
for pair in "${ORDER[@]}"; do
    IFS=$'\t' read -r a b <<<"$pair"
    [[ -n "${REGISTRY[$a]:-}" && -n "${REGISTRY[$b]:-}" ]] \
        || fail "gate-roster monotonicity, $a ⊆ $b: the loop above recorded no installed registry for one of them, so the containment would hold by vacuity"
    contains "${REGISTRY[$b]}" "gate-roster monotonicity, $a ⊆ $b" "${REGISTRY[$a]}"
done
say "gate rosters are monotone across every comparable pair of installed registries"

# spec: installer/README.md §The consumer smoke — the artifact-less refusal leg, which was a named INSTALL until selection was given one success path. It drives the packer's own artifact-free output, and that is what no other leg reaches: the artifact arm's two refusals are driven against a payload this smoke mutated by hand, so without this leg nothing asserts that a payload the publishing path actually produces without artifacts refuses rather than proceeding. It asserts a refusal that wrote NOTHING — the shape the artifact arm's tampered leg already uses — and it asserts it on the FIRST verb an adopter would reach for, because the refusal is the bootstrap's and precedes every verb rather than belonging to one. Naming a profile here is a scoping choice about which invocation to make, and the refusal is reached before the profile is ever read, which is itself part of what the leg says
resolves_profile() { local p; for p in "${PROFILES[@]}"; do [[ "$p" == "$1" ]] && return 0; done; return 1; }
BARE_PROFILE=prose
resolves_profile "$BARE_PROFILE" \
    || fail "the artifact-less leg is scoped to '$BARE_PROFILE' and the payload declares [${PROFILES[*]}] — re-scope it on a declared profile, never drop the leg"
printf 'artifact-less refusal leg (%s, payload packed with no artifact)\n' "$BARE_PROFILE"
BARE="$SCRATCH/bare"
mkdir -p "$BARE"
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --root "$REPO" --version "$VERSION" --out "$BARE" 2>&1)" \
    || { printf '%s\n' "$PACK_OUT" >&2; blocked "the artifact-less pack step failed."; }
say "$(grep -m1 '^PACK:' <<<"$PACK_OUT")"
shopt -s nullglob
bare_tarballs=("$BARE"/*.tgz)
shopt -u nullglob
[[ ${#bare_tarballs[@]} -eq 1 ]] || fail "expected exactly one artifact-less tarball, found ${#bare_tarballs[@]}"
( cd "$BARE" && tar -xzf "${bare_tarballs[0]##*/}" ) || fail "tar could not extract the artifact-less tarball"
# spec: installer/README.md §The consumer smoke — the leg proves its own premise, because a payload that quietly gained an artifact would be selected and verified and the refusal assertions would never run
[[ ! -e "$BARE/package/payload/artifact" ]] \
    || fail "the artifact-less payload carries an artifact directory — the leg would assert a refusal against a payload with something to run"

C="$(consumer artifact-less)" || fail "could not build a scratch consumer for the artifact-less refusal leg"
BARE_SEED="$(git -C "$C" rev-parse 'HEAD^{tree}')"
out="$( cd "$C" && bash "$BARE/package/bin/checkwright.sh" init --profile "$BARE_PROFILE" 2>&1 )"; rc=$?
[[ "$rc" -ne 0 ]] \
    || { printf '%s\n' "$out" >&2; fail "a payload the packer produced with no artifact directory installed anyway — selection has one success path, and this is not it"; }
# spec: installer/README.md §The gate binary — the outcome is asserted by its MESSAGE and its remedy rather than by the exit status alone, which is delta 1's own bound: an unrostered host and a broken payload are different answers to an adopter and the status is not what tells them apart
grep -q 'maps to no target this payload declares' <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "the artifact-less payload refused without naming the platform as the thing it carries nothing for"; }
grep -q 'no adopter action to take' <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "the artifact-less refusal carries no remedy line, so an adopter cannot tell a platform they can do nothing about from a payload they should re-download"; }
# spec: installer/README.md §The consumer smoke — the refusal is asserted on the CONSUMER and not only on the exit code: a refusal that wrote first and refused after would exit non-zero too, and only an untouched tree tells the two apart
[[ "$(git -C "$C" rev-parse 'HEAD^{tree}')" == "$BARE_SEED" && -z "$(git -C "$C" status --porcelain)" && ! -f "$C/checkwright.lock" ]] \
    || fail "the artifact-less refusal left the consumer changed — it refused after writing something, not before"
say "artifact-less payload: refused naming the platform, with a remedy, and nothing written"

# spec: installer/README.md §The install boundary — the refusal precedes the invoke, so it cannot be a property of one verb: the same package is driven with a second verb and with a bare invocation, and all three answer alike. A leg asserting only init would pass on a bootstrap that had grown a per-verb branch, which is exactly what the branchless argv rule forbids
bare_probe() {   # $@ = the argv to hand the artifact-less package's bootstrap
    local out rc
    out="$( cd "$C" && bash "$BARE/package/bin/checkwright.sh" "$@" 2>&1 )"; rc=$?
    [[ "$rc" -ne 0 ]] && grep -q 'maps to no target this payload declares' <<<"$out" \
        || { printf '%s\n' "$out" >&2; fail "'checkwright $*' on the artifact-less payload answered differently from init — the refusal is the bootstrap's and precedes every verb"; }
}
bare_probe doctor
bare_probe diff
bare_probe
say "the same refusal answers doctor, diff and a bare invocation — it is the bootstrap's, not a verb's"

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

# spec: installer/README.md §The consumer smoke — the toolchain-free arm, and the reason it uses the mask the Node-free arm already proved rather than a knob: the preflight requires cargo and rustc off the artifact hand-off path, because there the smoke builds the binary the payload carries, so every arm above drives doctor and init on a machine that has them and none could observe an install path demanding them. INSTALLER_SMOKE_ARTIFACTS_DIR does relax that preflight, and it is still not this arm's instrument: it relaxes the whole run's build, where this arm needs a toolchain-free host underneath a payload the run built for itself. The payload it installs carries that prebuilt artifact, so this arm asserts the pre-compiled path end to end on a host that could not have compiled it. A masked PATH is what a machine with no Rust toolchain actually is, where a knob suppressing a roster member would be a second, test-only audience axis no adopter ever exercises
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
# spec: installer/README.md §The consumer smoke — masking is per-arm, which is what lets this arm exist without weakening the preflight the build step depends on; the mask is proved rather than assumed for the same reason the Node-free one is
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

# spec: installer/README.md §The consumer smoke — the jq-less arm, and what it asserts INVERTED when the verbs moved behind the invoke: nothing there reads JSON with jq any more, because the crate reads it with serde_json, so the claim is no longer that the verbs refuse naming jq but that they no longer need it at all. That is the stronger reading of the same arm and it is why the arm survives the relocation rather than retiring with the preflight it was built around. Masking is still per-arm, and per-arm is still load-bearing for a reason the other two masks do not have — this harness reads every manifest assertion with jq itself, so a mask on the harness's own PATH would disarm the assertions rather than the installer. The mask rides the verb's PATH alone
# spec: installer/README.md §The consumer smoke — the mask is by ABSENCE, not by the failing shim the node and cargo arms use, and the difference is the question each arm asks rather than a style choice. Those arms ask whether the payload ever *reaches* a program, so a shim that fails loudly is exactly right. This arm asks what a machine *without* jq is told, and a shim is a jq that is present: `command -v jq` — the preflight's own predicate — resolves it and the preflight never fires, so a shim-masked arm would drive the verbs straight into the misdiagnosis it exists to catch. The farm is derived from the live PATH rather than from a maintained list of the programs the verbs use, so it cannot drift out of date the way such a list would
printf 'jq-less arm (%s, jq absent from the verbs'\'' PATH)\n' "$PROFILE_MIN"
JQFARM="$SCRATCH/jqfarm"
mkdir -p "$JQFARM"
IFS=: read -ra jq_path_dirs <<<"$PATH"
# spec: installer/README.md §The consumer smoke — a directory is FARMED only when it actually carries a jq, and every other one is kept on the arm's PATH verbatim. Farming the whole of PATH was correct where a link is a link and free, and it is neither on a host whose `ln -s` deep-copies: it copies every executable on the system PATH into scratch, and the copies are then the only thing on PATH, so a relocated binary looks for the runtime library beside it and does not find it. Keeping the untouched directories removes both at once and takes nothing away from the mask, because a directory with no jq in it cannot put jq back
# spec: installer/README.md §The consumer smoke — one rule decides both the skip and the farm's exclusion, spelled once here, because a detector that disagreed with the excluder would farm a directory and then link its jq straight back in. The rule is the STEM, case-folded, and never the bare name: on a host carrying an executable suffix a literal `jq` matches no file, and the suffix is not one value to strip — the shell resolves `.exe` where the crate's own PATH search reads `PATHEXT` and would find a `.cmd` this arm had left behind. Stripping at the first dot covers every member of that set without this surface naming any of them, which is also why no suffix accessor is read here: the question is not what THIS host appends to an artifact
jq_is_jq() {   # $1 = a file name -> 0 when it names the jq program under any extension this or any host resolves
    local n="${1##*/}"
    n="${n%%.*}"
    [[ "${n,,}" == jq ]]
}
JQ_KEEP=()
for jq_d in "${jq_path_dirs[@]}"; do
    [[ -d "$jq_d" ]] || continue
    jq_here=0
    for jq_f in "$jq_d"/*; do
        jq_is_jq "$jq_f" || continue
        jq_here=1; break
    done
    if [[ "$jq_here" -eq 0 ]]; then JQ_KEEP+=("$jq_d"); continue; fi
    for jq_f in "$jq_d"/*; do
        jq_b="${jq_f##*/}"
        jq_is_jq "$jq_f" && continue
        [[ -x "$jq_f" && ! -d "$jq_f" ]] || continue
        [[ -e "$JQFARM/$jq_b" ]] && continue
        ln -s "$jq_f" "$JQFARM/$jq_b" 2>/dev/null
    done
done
# spec: installer/README.md §The consumer smoke — the kept directories go AFTER the farm rather than before, so a program present in both resolves to the original the host installed and not to the farm's stand-in for it. That ordering is free here because the mask does not depend on it: jq is absent from the farm by construction and from every kept directory by the test above, so no order can put it back
JQ_PATH="$JQFARM"
for jq_d in ${JQ_KEEP[@]+"${JQ_KEEP[@]}"}; do JQ_PATH="$JQ_PATH:$jq_d"; done
# spec: installer/README.md §The consumer smoke — the mask is proved in both directions, for the reason the other two masks are proved in one: a PATH that failed to drop jq would assert nothing while passing, and a farm that failed to populate would make every verb fail for the wrong reason and pass this arm on a refusal that has nothing to do with jq. So jq must be gone and a control program must still resolve
[[ -z "$( PATH="$JQ_PATH" bash -c 'command -v jq' 2>/dev/null )" ]] \
    || fail "the mask did not take: jq still resolves under the arm's PATH"
# spec: installer/README.md §The consumer smoke — the control RUNS its program rather than resolving it, because `command -v` is a stat and the failure it has to catch is a PATH whose entries resolve and will not execute. A farm of relocated binaries satisfies a stat and dies on exec, and the arm would then red four assertions later at a verb, naming jq for a fault that has nothing to do with jq — which is the exact misattribution this control exists to prevent
PATH="$JQ_PATH" git --version >/dev/null 2>&1 \
    || fail "the jq-less farm's git will not run — the arm's PATH resolves entries this host cannot execute, so every verb below would refuse for a reason that is not jq"
say "mask: jq resolves to nothing, and the farm's git still runs"

# spec: installer/README.md §The consumer smoke — the JSON-reading verbs are asserted to SUCCEED here, which is the whole inversion: a verb that still shelled out to jq would fail on this PATH, and one that reads the manifest in-process cannot tell the difference. The label is carried so a red names which verb reached for a program that is not there
assert_jq_free() {   # $1 = a label for the message, $2 = consumer dir, $3.. = the verb and its argv
    local label="$1" dir="$2"; shift 2
    local out rc
    out="$( cd "$dir" && PATH="$JQ_PATH" "${ENTRY[@]}" "$@" 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] \
        || { printf '%s\n' "$out" >&2; fail "$label exited $rc on a jq-less machine — nothing behind the invoke reads JSON with jq, so a verb that needs it is reaching for a program the relocation removed the dependency on"; }
    grep -q 'jq' <<<"$out" \
        && { printf '%s\n' "$out" >&2; fail "$label ran on a jq-less machine but mentioned jq — the verb still has an opinion about a program it no longer uses"; }
    say "$label: runs clean with no jq on PATH, exit 0"
}

ENTRY=("$CW")
C="$(consumer jq-less)" || fail "could not build a scratch consumer for the jq-less arm"
# spec: installer/README.md §init — init is the one verb here that DOES refuse, and not for a reason of its own: jq is a consumer-audience member of the toolchain floor, so a jq-less machine is below contract and init's last precondition is doctor's verdict. The refusal an adopter meets on this machine is therefore the floor's, delivered before anything is written, rather than a JSON reader's
assert_jq_blocked() {   # $1 = a label for the message, $2 = consumer dir, $3.. = the verb and its argv
    local label="$1" dir="$2"; shift 2
    local out rc before status lock_before lock_after
    # spec: installer/README.md §init — "nothing written" is asserted as a DIFFERENCE across the refusal and never as the absence of a manifest, because this arm asserts the same refusal against a tree that already carries one: an absence test passes by accident on the first call and reds on the second for a manifest an earlier install wrote, which is a fact about the arm's ordering rather than about the refusal
    before="$(git -C "$dir" rev-parse 'HEAD^{tree}')"
    status="$(git -C "$dir" status --porcelain)"
    lock_before=""; [[ ! -f "$dir/checkwright.lock" ]] || lock_before="$(digest_of "$dir/checkwright.lock")"
    out="$( cd "$dir" && PATH="$JQ_PATH" "${ENTRY[@]}" "$@" 2>&1 )"; rc=$?
    [[ "$rc" -eq 1 ]] \
        || { printf '%s\n' "$out" >&2; fail "$label exited $rc on a jq-less machine, not the 1 doctor's below-contract verdict carries into it"; }
    grep -q 'the toolchain is below contract — refusing to install' <<<"$out" \
        || { printf '%s\n' "$out" >&2; fail "$label refused on a jq-less machine without naming the toolchain floor as the reason — it found some other objection ahead of the precondition an adopter must actually fix"; }
    grep -qE '^  jq +NOT FOUND' <<<"$out" \
        || { printf '%s\n' "$out" >&2; fail "$label refused on the floor without rendering the report that names jq as the missing member, so an adopter is told to fix a floor and not which part of it"; }
    lock_after=""; [[ ! -f "$dir/checkwright.lock" ]] || lock_after="$(digest_of "$dir/checkwright.lock")"
    [[ "$(git -C "$dir" rev-parse 'HEAD^{tree}')" == "$before" && "$(git -C "$dir" status --porcelain)" == "$status" && "$lock_before" == "$lock_after" ]] \
        || fail "$label refused on the toolchain floor and changed the consumer — the verdict is a precondition, so the refusal writes nothing"
    say "$label: blocked by doctor's floor verdict naming jq, consumer unchanged, exit 1"
}

# spec: installer/README.md §init — on a tree with no manifest there is nothing yet to read, so this is the case that isolates the floor refusal from every manifest question
assert_jq_blocked "init (no manifest yet)" "$C" init --profile "$PROFILE_MIN"

# spec: installer/README.md §The consumer smoke — this arm sets no RUN_PATH: its masked calls carry JQ_PATH explicitly and this one ordinary install runs under the ambient PATH, so nothing here reads RUN_PATH and the arms below reach their own assignments untouched. Left as a note rather than a defensive assignment because a dead assignment that looks load-bearing is what the next arm inserted here would copy
out="$( cd "$C" && "${ENTRY[@]}" init --profile "$PROFILE_MIN" 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the jq-less arm could not make an ordinary install to run its manifest-reading verbs against"; }
# spec: installer/README.md §init — with a manifest present the floor is still what init meets first, so the refusal is the same one and is asserted a second time against a tree that HAS a manifest: a verb that had grown a jq-shaped manifest read would answer differently here than it did above
assert_jq_blocked "init (manifest present)" "$C" init --profile "$PROFILE_MIN"
# spec: installer/README.md §The verbs — diff and uninstall run no doctor precondition, so on this machine they are the two verbs that both read the manifest and reach their answer: they are the arm's positive evidence that the JSON read itself no longer needs jq
assert_jq_free "diff" "$C" diff
assert_jq_free "uninstall --dry-run" "$C" uninstall --dry-run

# spec: installer/README.md §doctor — doctor is asserted DIRECTLY as well as through init, and the difference is what each shows: init's refusal proves the floor is a precondition, and this proves doctor reaches its whole report rather than refusing somewhere ahead of it. jq is a consumer-audience member of the floor, so a machine without it is genuinely below contract and doctor saying so is correct rather than a defect; asserting exit 0 here would have been asserting the opposite of the contract
out="$( cd "$C" && PATH="$JQ_PATH" "${ENTRY[@]}" doctor 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] \
    || { printf '%s\n' "$out" >&2; fail "doctor exited $rc with jq absent, not the 1 that means below contract — jq is a floor member the vendored battery needs, so a jq-less machine is below contract and doctor is the verb that says so"; }
grep -q '^DOCTOR: below contract' <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "doctor exited 1 on a jq-less machine without rendering its below-contract verdict — it refused somewhere ahead of the report instead of reaching it"; }
grep -qE '^  jq +NOT FOUND' <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "doctor's toolchain block does not name jq as missing — the report an adopter reads to find out what to install is silent about the program that stopped every other verb"; }
say "doctor: reaches its full diagnosis, names jq missing, verdict below contract"

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
# spec: installer/README.md §The gate binary — every cross-version pack carries the artifact directory the main pack used, because selection has one success path: a payload packed without one refuses at the bootstrap, and these hops assert manifest behavior that only a completed install reaches. The bytes are the same ones the build leg staged, so the hops differ in version and in the relinquish this arm performs, and in nothing else
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --root "$REPO" --version "$UP_VERSION" --out "$UP" --artifacts "$PACK_ARTIFACTS" 2>&1)" \
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
# spec: installer/README.md §The consumer smoke — the clean-worktree assertion below is only evidence over a hop that rewrote something, so the tripwire that keeps it non-vacuous is asserted first. Its SUBJECT moved with the relocation and the assertion did not: it used to count the omissions this hop's artifact-free payload declared, and every payload now carries a verified artifact, so what it counts is the live membership the hop installed and the artifact it placed. That is the same claim over the class delta 1 left standing rather than the one it emptied — and it is a stronger reading, because the members are the ones that ran rather than the ones that could not. Its scope is this hop; the second hop below carries no tripwire, and widening it is a separate judgment
UP_LIVE=0
[[ ! -f "$C/$GATES_DIR/gates.list" ]] || UP_LIVE="$(grep -Evc '^[[:space:]]*(#|$)' "$C/$GATES_DIR/gates.list")"
[[ "$UP_LIVE" -gt 0 && "$(jq -r '.artifact.target // ""' "$LOCK")" != "" ]] \
    || fail "the upgrade hop's $PROFILE_MIN install left $UP_LIVE live registry member(s) and $(jq -r 'if has("artifact") then "an" else "no" end' "$LOCK") artifact, so the clean-worktree assertion below holds over a hop that rewrote nothing — repair the hop, never drop the assertion"
say "tripwire: $UP_LIVE live member(s) and a placed artifact on this hop, so the clean-worktree assertion has something to be about"
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
PACK_OUT="$(INSTALLER_PACK_TMP_DIR="$SCRATCH" bash "$REPO/scripts/pack-installer.sh" --root "$REPO" --version "$UP2_VERSION" --out "$UP2" --artifacts "$PACK_ARTIFACTS" 2>&1)" \
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

# spec: installer/README.md §The consumer smoke — the cross-version reversal arm, and what it reaches is that every other reversal in this suite runs on a consumer that has only ever met one version's payload — each reverses a first install — so nothing asserts that the roster covers an upgrade hop's write set, and nothing asserts that a tree whose payload changed shape between hops is still wholly removable. It is its own consumer with no adopter edit, because the arm above carries two committed edits and an edit is exactly the case tree-object equality cannot host — that case is the protection branch's, chained onto the seam arm below. It reuses the two packages that arm already extracted and buys no pack of its own
printf 'cross-version reversal arm (three versions, no adopter edit, %s)\n' "$PROFILE_MIN"
C="$(consumer cross-version-reversal)" || fail "could not build a scratch consumer for the cross-version reversal arm"
SEED="$(git -C "$C" rev-parse 'HEAD^{tree}')"
out="$( cd "$C" && "$CW" init --profile "$PROFILE_MIN" 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the cross-version reversal arm's starting install failed"; }
# spec: installer/README.md §The consumer smoke — the arm proves its own premise, in the idiom the binary-less leg's disclosure count and the upgrade hop's omission count already use: the relinquish is a mutation of a package this arm only reads, so an arm re-ordered above it would drive three identical payloads and reverse an ordinary install under a cross-version name
[[ ! -f "$UP/package/payload/$RELINQUISHED" ]] \
    || fail "the upgrade package still ships $RELINQUISHED, so this arm's three hops carry identical payloads and its reversal is an ordinary install wearing a cross-version name — keep the arm below the relinquish that the upgrade arm performs, never drop the assertion"
out="$( cd "$C" && bash "$UP/package/bin/checkwright.sh" init 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the cross-version reversal arm's relinquishing hop failed"; }
out="$( cd "$C" && bash "$UP2/package/bin/checkwright.sh" init 2>&1 )" \
    || { printf '%s\n' "$out" >&2; fail "the cross-version reversal arm's re-adding hop failed"; }
CROSS_LOCK="$C/checkwright.lock"
[[ "$(jq -r '.version' "$CROSS_LOCK")" == "$UP2_VERSION" ]] \
    || fail "the cross-version reversal consumer records $(jq -r '.version' "$CROSS_LOCK") rather than $UP2_VERSION, so it fell through a hop and the reversal runs on a shorter history than the arm claims — repair the hop, never drop the assertion"
[[ "$(jq -r --arg f "$RELINQUISHED" '.files | has($f)' "$CROSS_LOCK")" == "true" ]] \
    || fail "the cross-version reversal consumer's roster lost $RELINQUISHED across the payload hole, so uninstall has nothing cross-version to clear and this arm reverses an ordinary install — re-scope the relinquish subject onto a path the $PROFILE_MIN profile records, never drop the assertion"
say "premise: three hops landed at $UP2_VERSION, the relinquished path crossed the payload hole and is on the roster"
# spec: installer/README.md §The consumer smoke — both globals are set here rather than inherited, and it is load-bearing: RUN_PATH still carries the toolchain-free arm's mask and ENTRY still names the first version's entry point, so an arm that inherited them would reverse with cargo and rustc masked off and drive the oldest verb, asserting something other than what it says. ENTRY is the latest package because that is what an adopter holds after an upgrade, and reversing with the newest verb against a roster three versions old is the case under test
ENTRY=(bash "$UP2/package/bin/checkwright.sh")
RUN_PATH="$PATH"
assert_reversal "$PROFILE_MIN" "$C" "$SEED"
say "cross-version reversal: $VERSION -> $UP_VERSION -> $UP2_VERSION reversed to the pre-init tree object"

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
read_stream SEAM_ROSTER "the seam manifest's roster" < <(jq -r '.files | keys[]' "$SEAM_LOCK")
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
read_stream SEAM_RESIDUAL "the residual roster" < <(jq -r '.files | keys[]' "$SEAM_LOCK")
mapfile -t SEAM_SURVIVORS < <(printf '%s\n' "${SEAM_EDITED[@]}" | LC_ALL=C sort)
[[ "${SEAM_RESIDUAL[*]}" == "${SEAM_SURVIVORS[*]}" ]] \
    || fail "the residual roster is [${SEAM_RESIDUAL[*]}] where the survivors are [${SEAM_SURVIVORS[*]}]"
for f in "${SEAM_EDITED[@]}"; do
    [[ "$(jq -r --arg f "$f" '.files[$f]' "$SEAM_LOCK")" == "${SEAM_INIT_HASH[$f]}" ]] \
        || fail "the residual manifest records $f at a hash other than the one init wrote there — the next init would find it unchanged and claim it"
done

# spec: installer/README.md §The manifest — the residual shape is asserted on the object itself and not through an accessor, because that is the class of drift an accessor cannot catch: a missing key and a present-but-null key both read back as the empty string, so only has() tells an omitted artifact apart from a null one, and only re-sorting the captured text proves the sort reached every nesting level rather than the top one
jq -e 'has("artifact") | not' "$SEAM_LOCK" >/dev/null \
    || fail "the residual manifest carries an artifact key — an omitted field leaves the key absent, never null"
# spec: installer/README.md §The consumer smoke — the two sides are compared through the terminator owner rather than with `cmp` over jq's raw stdout, and the reason is measured on this arm's own two declarations rather than assumed: `jq` delivered 1210 of 1210 and 2 of 2 lines carrying a carriage return here, where the manifest it is re-rendering was written by the crate and carries none. A `cmp` between those two is a comparison of CHANNELS, and it fails on a host where they differ however canonical the writer is. Reading it line-wise after the one declared strip keeps every axis this assertion is about — the key order at every nesting level, which is what `-S` tests, the indentation, the line count, and any trailing byte that is not exactly one carriage return — and gives up only the axis the channel owns, which is the terminator exception this harness already rules on
_seam_sorted=(); _seam_stored=()
read_stream _seam_sorted "the residual manifest's recursive sort" < <(jq -S . "$SEAM_LOCK")
read_stream _seam_stored "the residual manifest as the crate wrote it" < "$SEAM_LOCK"
[[ "${_seam_sorted[*]}" == "${_seam_stored[*]}" ]] \
    || fail "the residual manifest does not match its own recursive sort — the one writer of the wire shape emitted an order its second writer could not reproduce"
unset _seam_sorted _seam_stored
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
    # spec: installer/README.md §The consumer smoke — the separator is concatenated INSIDE jq and never passed in as the head of an argument, because an argument that looks like an absolute POSIX path is rewritten into a host path on its way to a native Windows program: `/gates.list` arrives as something under the interpreter's own install root, `endswith` then matches nothing, and the arm reports a manifest carrying no shadow rather than a query that was never asked. Both the value and the defect are invisible in the verdict, which is why the rule is to hand a native program a RELATIVE operand and let the filter build the rest
    shadow="$(jq -r --arg b "${f##*/}" --arg own "$f" \
        '.files | keys | map(select(endswith("/" + $b) and . != $own)) | length' "$NARROW_LOCK")"
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
# spec: installer/README.md §The consumer smoke — the vacuity refusal carries the roster it searched and not just its verdict, because the two things that produce this zero are indistinguishable from a count: a manifest that legitimately carries no shadowing fixture, and a search that looked for the wrong string. The first is a fact about the payload and the second is a defect in this arm, and a reader handed `0` alone has to buy a round to tell them apart — which is exactly what the round that first hit this on Windows had to do
[[ "$narrow_shadowed" -gt 0 ]] || {
    printf '  narrowed manifest: %s key(s), kits %s -> %s\n' \
        "$(jq -r '.files | length' "$NARROW_LOCK")" "$wide_kits" "$narrow_kits"
    for f in "${SEAM_FILES[@]}"; do
        printf '  recorded %-28s %s\n' "$f" "$(jq -r --arg f "$f" '.files | has($f)' "$NARROW_LOCK")"
        printf '  keys sharing its basename: %s\n' \
            "$(jq -r --arg b "${f##*/}" '[.files | keys[] | select(endswith("/" + $b))] | join(" ")' "$NARROW_LOCK")"
        # spec: installer/README.md §The consumer smoke — the operand is echoed back THROUGH the same program the query goes to, because that is the only way to see an argument a host rewrote in transit: a value printed by the shell is the value the shell holds, not the one the native program was handed, and the two came apart here once already
        printf '  that basename as jq received it: %s\n' \
            "$(jq -rn --arg b "${f##*/}" '$b')"
    done
    printf '  a sample of the roster, to show the separator and depth it actually carries:\n'
    # spec: installer/README.md §The consumer smoke — the sample is bounded inside jq rather than by closing a pipe under it, so the diagnostic does not print a write error of its own beside the failure it is explaining
    jq -r '[limit(5; .files | keys[])] | .[]' "$NARROW_LOCK" | sed 's/^/    /'
    fail "no vendored fixture shadows any checked seam basename — the arm would pass without asserting"
}

out="$( cd "$NC2" && "$CW" doctor 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || { printf '%s\n' "$out" >&2; fail "doctor exited $rc on the narrowed consumer"; }
grep -q "^  registry     $GATES_DIR/gates.list\$" <<<"$out" \
    || { printf '%s\n' "$out" >&2; fail "doctor did not name the consumer's own $GATES_DIR/gates.list as the registry it inspected"; }
say "narrowing: $wide_kits kit(s) -> $narrow_kits, $narrow_checked recorded seam path(s) ($narrow_shadowed shadowed) still resolve to the consumer's own on the narrowed and the residual shape, doctor names the registry"

# spec: installer/README.md §The consumer smoke — the artifact arm drives the selection outcomes a single install cannot show, against its own extraction of the main payload: the binary that payload carries is the one this run built, so the arm mutates an extracted copy and never a second pack, and pack's own --artifacts path is already exercised by the pack every profile above installed from
printf 'artifact arm (selection outcomes, on a mutated copy of the packed payload)\n'
ARTP="$SCRATCH/artifact-pack"
mkdir -p "$ARTP"
tar -xzf "$TARBALL" -C "$ARTP" || fail "tar could not extract the packed tarball for the artifact arm"
PAY_ART="$ARTP/package/payload/artifact"
[[ -f "$PAY_ART/targets.list" && -f "$PAY_ART/$HOST_TARGET/$NATIVE_BIN" && -f "$PAY_ART/$HOST_TARGET/$NATIVE_BIN.sha256" ]] \
    || fail "the packed payload carries no complete $HOST_TARGET artifact beside a verbatim roster copy"
say "the packed payload carries $NATIVE_BIN for $HOST_TARGET with the sidecar the build leg emitted"

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

# spec: installer/README.md §The gate binary — the first selection outcome, INVERTED: a host the payload never committed to used to omit and declare and proceed, and it is now refused, because once every step of an install sits behind the invoke that branch has nothing to run into and its outcome would be a silent non-install rather than a smaller battery. What it must NOT collapse into is the declared-but-absent refusal below — the two are different answers to an adopter and the assertion pair is what holds them apart
printf '%s\n' "other-${HOST_TARGET#*-}" > "$PAY_ART/targets.list" \
    || fail "could not narrow the payload roster off this host"
NC="$(consumer artifact-undeclared)" || fail "could not build a scratch consumer for the undeclared-host leg"
NC_SEED="$(git -C "$NC" rev-parse 'HEAD^{tree}')"
undeclared_out="$( cd "$NC" && "${ENTRY[@]}" init --profile "$PROFILE_MIN" 2>&1 )"; rc=$?
[[ "$rc" -ne 0 ]] \
    || { printf '%s\n' "$undeclared_out" >&2; fail "init installed on a host the payload never committed to — omit-and-declare retired with the relocation, so this platform is refused rather than served an install whose battery cannot run"; }
grep -q 'maps to no target this payload declares' <<<"$undeclared_out" \
    || { printf '%s\n' "$undeclared_out" >&2; fail "the unrostered host was refused without being told that this platform is the thing the payload carries nothing for"; }
grep -q 'no adopter action to take' <<<"$undeclared_out" \
    || { printf '%s\n' "$undeclared_out" >&2; fail "the unrostered refusal names no remedy, so an adopter cannot tell it from the broken-payload one they are supposed to act on"; }
[[ "$(git -C "$NC" rev-parse 'HEAD^{tree}')" == "$NC_SEED" && -z "$(git -C "$NC" status --porcelain)" && ! -f "$NC/checkwright.lock" ]] \
    || fail "the unrostered refusal wrote into the consumer — a refusal that wrote first is a partial install, not a refusal"
say "host off the payload roster: refused naming the platform, no adopter action, nothing written"
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
absent_out="$( cd "$AC" && "${ENTRY[@]}" init --profile "$PROFILE_MIN" 2>&1 )"; rc=$?
[[ "$rc" -ne 0 ]] || { printf '%s\n' "$absent_out" >&2; fail "the payload declares $HOST_TARGET and carries no artifact for it, and init installed anyway — a broken payload read as a narrower one"; }
grep -q 'carries no complete artifact for it' <<<"$absent_out" \
    || { printf '%s\n' "$absent_out" >&2; fail "the declared-but-absent target was refused without naming the incomplete artifact as the cause"; }
[[ ! -f "$AC/checkwright.lock" && -z "$(git -C "$AC" status --porcelain)" ]] \
    || fail "the broken-payload refusal still wrote into the consumer"
say "declared target with no artifact: refused, and not with the unrostered host's answer"
# spec: installer/README.md §The gate binary — delta 1's bound, asserted rather than assumed: the table keeps three outcomes and they stay told apart by MESSAGE AND REMEDY, never by exit status alone. Two of them now refuse, so an arm that checked only the status would read them as one answer — this compares the two refusals directly and reds if they ever converge on the same words
[[ "$undeclared_out" != "$absent_out" ]] \
    || { printf '%s\n' "$absent_out" >&2; fail "the unrostered host and the broken payload printed the same thing — two of the three selection outcomes refuse, and collapsing them tells an adopter with no action to take to go and act"; }
grep -q 'no adopter action to take' <<<"$absent_out" \
    && { printf '%s\n' "$absent_out" >&2; fail "the broken-payload refusal wears the unrostered host's remedy — it is the one refusal an adopter CAN act on, by re-downloading"; }
say "the two refusals differ in message and remedy, which is what the exit status does not carry"

# spec: installer/README.md §The consumer smoke — the hand-off arm adopts a producer's bytes and sidecar unchanged, so the marker states the binary's provenance rather than asserting this run built one
PROVENANCE="the gate binary this run built"
[[ -z "$PREBUILT_DIR" ]] || PROVENANCE="the gate binary adopted from the hand-off, unrebuilt"

# spec: evidence-kit/SPEC.md §Layout and configuration — this line is the run's COMPLETION MARKER, derived positionally. A header printed after this line would silently become the marker and demote this one to an arm — the one hazard of that rule, and no gate catches it
printf 'INSTALLER-SMOKE: clean (%d profile(s) installed from the packed tarball with no registry access, each carrying %s, each put in front of a real prose defect (caught by %s) and each reversed back to its pre-init tree object, with gate rosters monotone across every comparable pair of the registries those installs wrote, plus the artifact-less %s leg driving a payload the packer itself produced with no artifact and asserting one refusal for init, doctor, diff and a bare invocation alike, naming the platform and writing nothing, the extracted-tarball arm with node/npm masked and reversed the same way, the toolchain-free arm driving doctor and a full init with cargo/rustc masked, the jq-less arm asserting diff and uninstall run clean with no jq on PATH while init is blocked by the toolchain floor verdict doctor renders, the two-hop cross-version upgrade arm carrying the relinquish and re-add, the cross-version reversal arm reversing an unedited consumer back to its pre-init tree object after those same three hops, the same-version seam arm and the protection branch chained onto it, the narrowing arm re-running init at a smaller profile so files[] outlives kits, and the artifact arm driving the three selection outcomes on a mutated copy of that payload, with its two refusals asserted to differ in message and remedy)\n' "${#PROFILES[@]}" "$PROVENANCE" "${VALUE_RED[*]}" "$BARE_PROFILE"
exit 0
