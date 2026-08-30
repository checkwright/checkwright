#!/usr/bin/env bash
# spec: installer/README.md §init — vendors the selected profile's kit source out of the package payload into the consumer's repository and commits it, so what governs their tree afterwards is committed, auditable source rather than something resolved at their build time
#
# usage: checkwright init [--profile <name>] [--dry-run] [--force] [--no-commit]
#   Each flag is described by --help, which prints the selectable profiles too.
set -uo pipefail

INSTALLER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PAYLOAD="$INSTALLER/payload"
# shellcheck source=./common/lock.sh
source "$INSTALLER/lib/common/lock.sh"
# shellcheck source=./common/profile.sh
source "$INSTALLER/lib/common/profile.sh"
# shellcheck source=./common/recipe.sh
source "$INSTALLER/lib/common/recipe.sh"
# shellcheck source=./common/digest.sh
source "$INSTALLER/lib/common/digest.sh"

PROFILE=""; DRY=0; FORCE=0; DO_COMMIT=1
while [[ $# -gt 0 ]]; do
    case "$1" in
        --profile) PROFILE="${2:-}"; shift 2 ;;
        --profile=*) PROFILE="${1#*=}"; shift ;;
        --dry-run) DRY=1; shift ;;
        --force) FORCE=1; shift ;;
        --no-commit) DO_COMMIT=0; shift ;;
        -h|--help)
            printf 'usage: checkwright init [--profile <name>] [--dry-run] [--force] [--no-commit]\n\n'
            printf 'Vendors pinned kit source into this repository and commits it.\n'
            printf 'Nothing is fetched: the source comes from this package.\n\n'
            printf 'profiles: %s\n' "$(profile_names "$INSTALLER" | tr '\n' ' ')"
            exit 0 ;;
        *) printf 'checkwright init: unknown argument: %s\n' "$1" >&2; exit 2 ;;
    esac
done

die() { printf 'checkwright init: %s\n' "$1" >&2; [[ -n "${2:-}" ]] && printf '  help: %s\n' "$2" >&2; exit "${3:-2}"; }

[[ -d "$PAYLOAD" ]] || die "this package carries no payload" \
    "init copies kit source out of the package's own payload/, assembled at pack time — run it from an installed package, not from a source checkout."

# spec: installer/README.md §init — the three preconditions all refuse rather than warn, and all three are checked before any file is written: a partial install is the outcome none of them may produce
ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )" \
    || die "not inside a git work tree" \
       "the vendored source is meant to be committed, which is what makes it auditable. Run 'git init' first, or run init inside the repository you want governed."

# spec: installer/README.md §init — the clean-worktree precondition exists so the one commit init makes is exactly what it vendored; --no-commit is its valve, because an operator staging the vendoring themselves has taken that guarantee on
if (( ! DRY )) && (( DO_COMMIT )) && [[ -n "$(git -C "$ROOT" status --porcelain)" ]]; then
    die "the worktree is not clean" \
        "init makes one commit, and a dirty tree would fold your work into it — so a reviewer's diff would no longer be the whole of what was vendored. Commit or stash first, or pass --no-commit to stage the vendoring yourself." 1
fi

# spec: installer/README.md §init — the jq preflight sits here rather than at the top of the file because here is where "before its first JSON read" falls: every precondition above is about the package or the repository and is answerable without jq, and every refusal below is one a jq-less machine would otherwise meet as a misdiagnosis
lock_require_jq

PKG="$INSTALLER/package.json"
VERSION="$(jq -r '.version // ""' "$PKG" 2>/dev/null)"
COMMIT="$(jq -r '.checkwright.commit // ""' "$PKG" 2>/dev/null)"
[[ -n "$VERSION" ]] || die "this package carries no version stamp" \
    "the version is stamped at pack time from the release tag; a package without one was not assembled by the pack step."

LOCK="$(lock_path "$ROOT")"
PRIOR_FILES=""
if [[ -f "$LOCK" ]]; then
    lock_schema_ok "$LOCK" || die "$CHECKWRIGHT_LOCK_FILE carries a schema this build does not know" \
        "this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for."
    PRIOR_VERSION="$(lock_field "$LOCK" version)"
    # spec: installer/README.md §The manifest — the version field's re-run reader: a payload older than the recorded install is a silent downgrade, so it refuses rather than rolling the tree backwards, and --force is the one thing that makes the rollback deliberate rather than silent
    if (( ! FORCE )) && [[ -n "$PRIOR_VERSION" && "$PRIOR_VERSION" != "$VERSION" ]]; then
        older="$(printf '%s\n%s\n' "$PRIOR_VERSION" "$VERSION" | sort -V | head -n1)"
        if [[ "$older" == "$VERSION" ]]; then
            die "this package is $VERSION but $CHECKWRIGHT_LOCK_FILE records $PRIOR_VERSION — refusing a silent downgrade" \
                "run the release you already have, or pass --force if rolling back is what you meant." 1
        fi
    fi
    [[ -n "$PROFILE" ]] || PROFILE="$(lock_field "$LOCK" profile)"
    PRIOR_FILES="$(jq -r 'if (.files | type) == "object" then (.files | to_entries[] | "\(.key)\t\(.value)") else empty end' "$LOCK" 2>/dev/null)"
fi

PROFILE="${PROFILE:-starter}"
profile_known "$INSTALLER" "$PROFILE" || die "unknown profile: $PROFILE" \
    "selectable profiles: $(profile_names "$INSTALLER" | tr '\n' ' ')"

# spec: installer/README.md §init — doctor is the last precondition and still runs before any file is written: a below-contract toolchain must block before a partial install, and running it after the manifest and profile are resolved keeps a bad manifest from being reported as a toolchain fault
DOCTOR_OUT="$(bash "$INSTALLER/lib/doctor.sh" 2>&1)"; DOCTOR_RC=$?
if [[ "$DOCTOR_RC" -ne 0 ]]; then
    printf '%s\n' "$DOCTOR_OUT" >&2
    die "the toolchain is below contract — refusing to install" \
        "the floors above are what the gate battery needs to run, so installing first would leave you a vendored tree that cannot be checked. Fix them and re-run." "$DOCTOR_RC"
fi

KITS=()
while IFS= read -r k; do [[ -n "$k" ]] && KITS+=("$k"); done < <(profile_kits "$INSTALLER" "$PROFILE")
[[ ${#KITS[@]} -gt 0 ]] || die "profile '$PROFILE' resolves to no kit in this payload" \
    "every kit a profile names must exist in the package payload; this one names none that do."

# spec: installer/README.md §The gate binary — the host answers what platform this is, so the answer stays a local and is never persisted: a stored copy is stale the first time a vendored tree moves between machines, which is the case that matters most. Two fields rather than `uname -a` because that is the smallest input answering the question and the one a PowerShell half can answer without parsing prose
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

ARTIFACT_SRC=""; ARTIFACT_PATH=""; ARTIFACT_TARGET=""; ARTIFACT_DIGEST=""; OMIT_REASON=""

# spec: installer/README.md §The gate binary — selection has three outcomes and collapsing any two is the defect, which is why the payload's roster copy is read rather than a directory's presence inferred from: without it a platform that was never committed to and one whose artifact went missing look identical, and reading the second as the first turns a broken payload into a silently smaller green battery
select_artifact() {
    local roster target src want got names
    if [[ ! -d "$PAYLOAD/artifact" ]]; then
        OMIT_REASON="substrate-unavailable"
        return 0
    fi
    roster="$PAYLOAD/artifact/targets.list"
    [[ -f "$roster" ]] || die "this payload carries prebuilt gate binaries but no target roster" \
        "the roster is copied verbatim beside them at pack time; artifacts without one cannot be selected from and the payload is broken, not narrower."
    target="$(target_of_host)"
    if [[ -z "$target" ]] || ! grep -Ev '^[[:space:]]*(#|$)' "$roster" | grep -qxF "$target"; then
        OMIT_REASON="substrate-unavailable"
        return 0
    fi
    src="$PAYLOAD/artifact/$target"
    mapfile -t names < <(find "$src" -maxdepth 1 -type f ! -name '*.sha256' -printf '%P\n' 2>/dev/null | sort)
    [[ ${#names[@]} -eq 1 && -f "$src/${names[0]}.sha256" ]] \
        || die "the payload declares $target but carries no complete artifact for it" \
           "a declared target whose binary or .sha256 sidecar is missing is a publisher defect you cannot act on — refusing rather than installing a battery that silently shrank." 1
    if [[ -z "$(digest_hasher)" ]]; then
        OMIT_REASON="digest-unverifiable"
        return 0
    fi
    want="$(awk 'NR==1{print $1}' "$src/${names[0]}.sha256")"
    got="$(digest_of "$src/${names[0]}")"
    [[ -n "$want" && "$want" == "$got" ]] \
        || die "the prebuilt gate binary for $target does not match its published digest" \
           "nothing unverified is ever written, so this refuses rather than warning. Re-download the package; a persistent mismatch means the artifact was altered after it was built." 1
    ARTIFACT_SRC="$src/${names[0]}"
    ARTIFACT_PATH="$GATES_DIR/${names[0]}"
    ARTIFACT_TARGET="$target"
    ARTIFACT_DIGEST="$got"
}
select_artifact

# spec: installer/README.md §What init seeds — a starting-roster member whose implementation is a binary subcommand is what an omission omits; the payload's own declaration decides, so nothing here maintains a second roster of which gates are ported
dispatches_to_binary() {   # $1 = kit, $2 = gate name -> 0 iff the payload declares it as a binary subcommand
    [[ -f "$PAYLOAD/$1/checks/$2.gate" && ! -f "$PAYLOAD/$1/checks/$2.sh" ]]
}

prior_hash() {   # $1 = repo-relative path -> the hash the manifest recorded for it, empty when unrecorded
    [[ -n "$PRIOR_FILES" ]] || return 0
    while IFS=$'\t' read -r p h; do
        [[ "$p" == "$1" ]] && { printf '%s' "$h"; return 0; }
    done <<<"$PRIOR_FILES"
}

WRITTEN=()
declare -A CARRIED=()
# spec: installer/README.md §The manifest — membership in the written set is held as a key rather than re-derived by piping the list into a quiet grep: that grep exits on its first match while the writer is still writing, the writer takes SIGPIPE, and under the pipefail this file sets the pipeline's status becomes the signal instead of the match. The test then reads as "absent" for a path init wrote seconds earlier, which carries it forward at its prior hash and drops it from the staged set — and because a carried path is appended to the list, every later lookup starts further from the tail. The array is written where paths are recorded so the two cannot part company
declare -A IS_WRITTEN=()
# spec: installer/README.md §The manifest — a recorded hash is what init last wrote at that path, so an entry init did not write this run — because it refused the write, or because this payload no longer ships the path — carries its hash forward verbatim instead of letting manifest() hash the tree at emit time: hashing the tree there would file the adopter's own content as init's, and the next run would find cur == want, claim the path and overwrite it silently
record() {   # $1 = repo-relative path, $2 = the hash to emit for it; omitted, the tree is hashed at emit time
    WRITTEN+=("$1")
    IS_WRITTEN["$1"]=1
    [[ -n "${2:-}" ]] && CARRIED["$1"]="$2"
    return 0
}

# spec: installer/README.md §init — the non-destructive re-run: a file whose recorded hash still matches is init's to rewrite, one that has changed since is the adopter's and is reported rather than overwritten
CHANGED=()
claim() {   # $1 = repo-relative path -> 0 iff init may write it
    local want cur
    want="$(prior_hash "$1")"
    [[ -n "$want" ]] || return 0
    [[ -f "$ROOT/$1" ]] || return 0
    cur="$(lock_hash "$ROOT/$1")"
    [[ "$cur" == "$want" ]] && return 0
    (( FORCE )) && return 0
    CHANGED+=("$1")
    # spec: installer/README.md §The manifest — the carry-forward belongs to the refusal itself rather than to each caller, because this is the single point where the roster would otherwise lose the path and every call site refuses through it: absence of a key reads as "never installed" on the next run, which is the one reading that lets init overwrite the adopter with no report at all. Recording on the refusal is also what makes the exit condition below a strict subset — a live path is in WRITTEN either way, so only relinquished ones reach the carry-forward loop
    record "$1" "$want"
    return 1
}

copy_in() {   # $1 = source file, $2 = repo-relative destination
    claim "$2" || return 0
    (( DRY )) || { mkdir -p "$ROOT/$(dirname "$2")" && cp "$1" "$ROOT/$2"; } || die "could not write $2"
    record "$2"
}

for kit in "${KITS[@]}"; do
    [[ -d "$PAYLOAD/$kit" ]] || die "profile '$PROFILE' names $kit, which this payload does not carry" \
        "the payload's kit set is derived from the source tree at pack time; a profile naming a kit that is not there is a roster that has drifted."
    while IFS= read -r f; do
        copy_in "$PAYLOAD/$kit/$f" "$kit/$f"
    done < <(cd "$PAYLOAD/$kit" && find . -type f -printf '%P\n' | sort)
done

mkdir -p "$ROOT/$GATES_DIR" "$ROOT/.workflow" 2>/dev/null
GATES_LIST="$GATES_DIR/gates.list"
if claim "$GATES_LIST"; then
    # spec: installer/README.md §The gate binary — an omitted member rides the registry rather than a new file: `# omitted: <name> <reason>` is a comment line the runner already strips from the live set, so the record sits in the consumer's tracked history where a reviewer reads it, and a re-run on a machine that has since gained a hasher converts it back into a live member with no hand edit
    plan_gates() {
        local kit m
        local -a g
        local -A pending=()
        printf '%s\n' "# Checkwright gate registry — written by 'checkwright init' (profile: $PROFILE)." \
            "# Each kit's starting subset; its README names the full roster to grow into."
        # spec: installer/README.md §Profiles — the profile's gate set is derived once, by the same function the smoke's monotonicity assertion reads, so the registry a consumer receives and the invariant asserted over it are not two derivations of one fact. The loop below only sections that set by kit, which the registry keeps because the omission is keyed by the kit shipping the member — and a member registered by more than one kit lands once, under the first of them
        while IFS= read -r m; do [[ -n "$m" ]] && pending["$m"]=1; done < <(profile_gates "$INSTALLER" "$PROFILE")
        for kit in "${KITS[@]}"; do
            mapfile -t g < <(recipe_gates "$PAYLOAD/$kit" "$PROFILE")
            [[ ${#g[@]} -gt 0 ]] || continue
            printf '# %s\n' "$kit"
            for m in "${g[@]}"; do
                [[ -n "${pending[$m]:-}" ]] || continue
                unset "pending[$m]"
                if [[ -n "$OMIT_REASON" ]] && dispatches_to_binary "$kit" "$m"; then
                    printf '# omitted: %s %s\n' "$m" "$OMIT_REASON"
                else
                    printf '%s\n' "$m"
                fi
            done
        done
    }
    (( DRY )) || plan_gates > "$ROOT/$GATES_LIST" || die "could not write $GATES_LIST"
    record "$GATES_LIST"
fi

for kit in "${KITS[@]}"; do
    # spec: installer/README.md §What init seeds — the recipe's plan channel is performed here, in the parent shell, and that is load-bearing rather than stylistic: the producer sits inside a process substitution, where every record() and CHANGED append would be discarded, so the plan may be produced in a subshell and the writes may not
    while IFS= read -r p; do
        [[ -n "$p" ]] || continue
        if [[ "$p" == *$'\t'* ]]; then
            IFS=$'\t' read -r seam_src seam_dest <<<"$p"
            copy_in "$seam_src" "$seam_dest"
        else
            claim "$p" && record "$p"
        fi
    done < <(
        # spec: installer/README.md §init — --dry-run resolves the same seam plan the real run would, out of the same enumerator rather than a second spelling of the glob: copy_in already writes nothing under --dry-run, so the plan needs no dry variant and cannot drift from the run it predicts
        recipe_config_seam_plan "$PAYLOAD/$kit" "$GATES_DIR"
        if (( DRY )); then
            case "$kit" in
                gate-sdk)      printf '%s/msg-patterns.list\n' "$GATES_DIR" ;;
                evidence-kit)  printf '%s\n' .workflow/validate-baseline.txt .workflow/validate-evidence.txt ;;
                lifecycle-kit) printf '%s\n' .workflow/WORKFLOW-STATE.txt ;;
                doctrine-kit)  printf '%s\n' "$AGENT_FILE" ;;
            esac
        else
            if recipe_needs_agent_file "$kit" && [[ ! -f "$ROOT/$AGENT_FILE" ]]; then
                # spec: installer/README.md §What init seeds — the seeded agent file carries the section heading context-kit's brevity gate reads by default, so the gate init registers has the surface it was pointed at from the first commit
                printf '%s\n' "# $AGENT_FILE" "" \
                    "Resident instructions for agent sessions in this repository." "" \
                    "## Shared conventions" "" \
                    "- **Terse:** one line per rule here; the mechanism behind the pointer." \
                    > "$ROOT/$AGENT_FILE" || die "could not seed $AGENT_FILE"
                printf '%s\n' "$AGENT_FILE"
            fi
            recipe_seed "$kit" "$PAYLOAD/$kit" "$ROOT" || die "could not seed $kit"
        fi
    )
done

# spec: installer/README.md §What init seeds — the queue seed runs here, once, over the resolved kit set: inside the loop above the first kit reached decided the source before any kit shipping a template got a turn. It runs in the parent shell for the reason that loop's own directive gives — a record() inside the process substitution is discarded — and the resolver below is the single predicate the dry plan and the run share, so the prediction cannot drift from the write it predicts
QUEUE_SRC="$(recipe_queue_source "$PAYLOAD" "${KITS[@]}")"
if [[ -n "$QUEUE_SRC" && ! -f "$ROOT/$QUEUE_FILE" ]]; then
    (( DRY )) || recipe_write_queue "$QUEUE_SRC" "$ROOT" "$QUEUE_FILE" || die "could not seed $QUEUE_FILE"
    claim "$QUEUE_FILE" && record "$QUEUE_FILE"
fi

# spec: installer/README.md §The gate binary — the write comes after every config seam is in place and before the hook is generated, because the generator resolves each member's invocation argv and a `.gate` member resolves to this binary: the knob must name it and the file must be there, or the generator reports a dispatch it cannot make
if [[ -n "$ARTIFACT_TARGET" ]]; then
    # spec: installer/README.md §The install boundary — the placement is one call to the verified payload artifact, run in place out of the payload where step 4 verified it rather than as the installed copy, which on a first install does not exist yet. Every value the op needs arrives as argv and it reads no knob, so the PowerShell twin issues this same call
    PLACE=("$ARTIFACT_SRC" --install place-artifact
        --root "$ROOT" --src "$ARTIFACT_SRC" --dest "$ARTIFACT_PATH"
        --seam "$GATES_DIR/gate-sdk-config.sh"
        --target "$ARTIFACT_TARGET" --digest "$ARTIFACT_DIGEST")
    [[ -f "$LOCK" ]] && PLACE+=(--lock "$CHECKWRIGHT_LOCK_FILE")
    (( FORCE )) && PLACE+=(--force)
    (( DRY )) && PLACE+=(--dry-run)
    PLACED="$("${PLACE[@]}")" || die "the gate binary could not be placed" "" "$?"
    # spec: installer/README.md §The install boundary — the op's two stdout verbs, each performed here in the parent shell for the reason the seam plan above is: a record() inside a process substitution is discarded, so the plan may be produced in a subshell and the writes may not
    while IFS=$'\t' read -r verb placed_path placed_hash; do
        case "$verb" in
            own)  [[ -n "${IS_WRITTEN[$placed_path]:-}" ]] || record "$placed_path" ;;
            kept) CHANGED+=("$placed_path")
                  [[ -n "${IS_WRITTEN[$placed_path]:-}" ]] || record "$placed_path" "$placed_hash" ;;
        esac
    done <<<"$PLACED"
fi

# spec: installer/README.md §init — the generated projections are produced by the vendored tools themselves, never restated by the installer: the hook generator and the graph emitter are gate-sdk's, so a consumer's artifacts are the ones their own gate-sdk makes
GENERATED=("$GATES_DIR/git-hooks/pre-commit" "$GATES_DIR/CHECK-GRAPH.html")
if (( ! DRY )); then
    ( cd "$ROOT" && bash "$ROOT/gate-sdk/bin/gen-pre-commit.sh" --write ) >/dev/null \
        || die "gate-sdk's hook generator failed"
    # spec: installer/README.md §The gate binary — the graph emitter is a binary arm, reached through the front-end that resolves its bridged knobs (gate-sdk/SPEC.md §The non-gate arm). It runs exactly when an artifact was selected, because a payload that omitted the binary has written the graph gate into gates.list as an omission and the consumer therefore has no graph gate for this artifact to be fresh against — and because omission must never fail an install over something the adopter did not choose. The roster loop below already tolerates a generated file that was not produced.
    if [[ -n "$ARTIFACT_TARGET" ]]; then
        ( cd "$ROOT" && bash "$ROOT/gate-sdk/bin/run-gates.sh" --emit graph ) > "$ROOT/$GATES_DIR/CHECK-GRAPH.html" \
            || die "gate-sdk's graph emitter failed"
    fi
fi
[[ -f "$ROOT/$GATES_DIR/git-hooks/commit-msg" ]] && GENERATED+=("$GATES_DIR/git-hooks/commit-msg")
for g in "${GENERATED[@]}"; do
    (( DRY )) || [[ -f "$ROOT/$g" ]] || continue
    record "$g"
done

# spec: installer/README.md §The manifest — the roster's exit condition, and it is the whole rule: init owns a path because it wrote the file there, so ownership ends when the file leaves the tree and at no other moment. Dropping an entry would disown a file init created, and an uninstall reading this roster would then leave it behind. A payload that stops shipping a path is not that moment either — the file is still on disk, it may carry the adopter's edits, and disowning it is precisely what lets a later payload re-adding the same path write straight through them. So membership and existence are the only tests: every path this payload still ships has already reached claim(), which records it on the write and on the refusal alike, leaving exactly the paths no copy_in visited this run to reach here — and "this run" is why the loop is last rather than merely late: every path init writes must already be recorded when it runs, or the loop carries a path init rewrote seconds earlier at its superseded hash and the staged set loses it. The generated projections above are the ones that reached it from the wrong side
if [[ -n "$PRIOR_FILES" ]]; then
    while IFS=$'\t' read -r p h; do
        [[ -n "$p" ]] || continue
        [[ -n "${IS_WRITTEN[$p]:-}" ]] && continue
        [[ -f "$ROOT/$p" ]] || continue
        record "$p" "$h"
    done <<<"$PRIOR_FILES"
fi

# spec: installer/README.md §The manifest — what init wrote this run is a subset of the roster it records, because a path left alone for the adopter and a path this payload no longer ships are both carried forward rather than written; the two part company here so every reader downstream takes the one it means, and staging takes the written set: folding an adopter's file into the vendoring commit is what the clean-worktree precondition exists to prevent
STAGE=()
for f in "${WRITTEN[@]}"; do [[ -n "${CARRIED[$f]:-}" ]] || STAGE+=("$f"); done

files_hash() {   # $1 = repo-relative path -> the hash its files[] entry carries
    [[ -n "${CARRIED[$1]:-}" ]] && { printf '%s' "${CARRIED[$1]}"; return 0; }
    (( DRY )) && [[ ! -f "$ROOT/$1" ]] && { printf '(pending)'; return 0; }
    lock_hash "$ROOT/$1"
}

# spec: installer/README.md §The manifest — the wire shape has one writer and it is lock.sh's lock_emit; what stays here is init's own rule about which hash an entry carries, which is not the schema's business
manifest() {
    local f
    local -a args=(version="$VERSION" profile="$PROFILE" kits="${KITS[*]}")
    # spec: installer/README.md §The manifest — commit is passed on the same conditional footing as the artifact key, for the reason lock_emit already states: an identity field is present exactly when the caller supplied it, and an empty commit written as "" would be a placeholder standing in for an omission. Its emptiness was masked by statement order alone until the jq preflight landed ahead of it; with jq's absence ruled out, an empty commit means only that the package carries no commit stamp, and the existing rule settles what to do about it
    [[ -n "$COMMIT" ]] && args+=(commit="$COMMIT")
    # spec: installer/README.md §The manifest — the key's absence on a run that omitted the artifact is the omission's machine-readable form, which is why the flag is passed only when a target was selected; the binary's path needs no flag here because it already rode record() onto WRITTEN as an ordinary files[] row
    [[ -n "$ARTIFACT_TARGET" ]] && args+=(--artifact "$ARTIFACT_TARGET" "$ARTIFACT_DIGEST")
    for f in "${WRITTEN[@]}"; do
        printf '%s\t%s\n' "$f" "$(files_hash "$f")"
    done | lock_emit "${args[@]}"
}

if (( DRY )); then
    printf 'checkwright init --dry-run (profile: %s, version: %s)\n\n' "$PROFILE" "$VERSION"
    printf 'would vendor %d kit(s): %s\n' "${#KITS[@]}" "${KITS[*]}"
    printf 'would write %d file(s), including:\n' "$(( ${#STAGE[@]} + 1 ))"
    printf '  %s\n' "$GATES_LIST" "$CHECKWRIGHT_LOCK_FILE"
    for kit in "${KITS[@]}"; do printf '  %s/ (%d files)\n' "$kit" "$(find "$PAYLOAD/$kit" -type f | wc -l)"; done
    if [[ -n "$ARTIFACT_TARGET" ]]; then
        printf '\nwould place the %s gate binary at %s (digest verified against the payload sidecar)\n' \
            "$ARTIFACT_TARGET" "$ARTIFACT_PATH"
    else
        printf '\nwould omit the prebuilt gate binary (%s) and declare it in %s\n' "$OMIT_REASON" "$GATES_LIST"
    fi
    if [[ ${#CHANGED[@]} -gt 0 ]]; then
        printf '\nwould leave %d changed file(s) alone (--force to overwrite):\n' "${#CHANGED[@]}"
        printf '  %s\n' "${CHANGED[@]}"
    fi
    printf '\n%s that would be written:\n' "$CHECKWRIGHT_LOCK_FILE"
    manifest
    printf '\nDRY RUN: nothing was written.\n'
    exit 0
fi

manifest > "$LOCK" || die "could not write $CHECKWRIGHT_LOCK_FILE"
WRITTEN+=("$CHECKWRIGHT_LOCK_FILE")
STAGE+=("$CHECKWRIGHT_LOCK_FILE")

if [[ ${#CHANGED[@]} -gt 0 ]]; then
    printf '\n%d file(s) have changed since init wrote them and were left alone:\n' "${#CHANGED[@]}"
    printf '  %s\n' "${CHANGED[@]}"
    printf '  help: review the differences; re-run with --force to take the packaged version.\n\n'
fi

git -C "$ROOT" add -- "${STAGE[@]}" || die "could not stage the vendored files"

# spec: installer/README.md §init — idempotence is a property of the tree, so a re-run that changed nothing reports and exits clean rather than failing on an empty commit: "nothing to do" is the success case, not an error. The predicate reads the index rather than the worktree and it stays the guard on whether a commit is attempted, because a commit against an empty index exits non-zero and init treats a failed commit as a fatal install failure — an arm placed ahead of this branch would turn the pure idempotent path into a false hard error
if git -C "$ROOT" diff --cached --quiet; then
    # spec: installer/README.md §init — a run init considers a no-op still commits what it rewrote, so this branch asks what the predicate above cannot: whether anything on the roster init just recorded differs from the committed tree. The clean-worktree precondition is what makes the answer attributable — nothing dirty at those paths can be the adopter's — and --no-commit is exempt because it waives that precondition and hands the commit over. The expected answer is "nothing", at the cost of one status call; the arm is here so the one-commit contract survives the staged set failing to cover what init wrote rather than resting on the assumption that it never will, and the inner index check is what keeps that survival from becoming the empty commit the paragraph above rules out
    if (( DO_COMMIT )) && [[ -n "$(git -C "$ROOT" status --porcelain -- "${WRITTEN[@]}")" ]]; then
        git -C "$ROOT" add -- "${WRITTEN[@]}" \
            || die "could not stage the files init rewrote but left out of the vendoring commit"
        if ! git -C "$ROOT" diff --cached --quiet; then
            residue="$(git -C "$ROOT" diff --cached --name-only | grep -c .)"
            git -C "$ROOT" commit -q -m "chore: vendor Checkwright kits ($PROFILE profile, v$VERSION)" \
                || die "the commit failed — the files are staged; commit them yourself to finish the install" "" 1
            printf 'INIT: already at the %s profile (v%s) — %d file(s) checked; %d that init had rewritten were uncommitted and are now committed.\n' \
                "$PROFILE" "$VERSION" "${#WRITTEN[@]}" "$residue"
            exit 0
        fi
    fi
    printf 'INIT: already at the %s profile (v%s) — %d file(s) checked, nothing to change and nothing it rewrote left uncommitted.\n' \
        "$PROFILE" "$VERSION" "${#WRITTEN[@]}"
    exit 0
fi

if (( DO_COMMIT )); then
    # spec: installer/README.md §init — the commit is the distribution model, not a convenience: vendored-and-committed is what makes the tree auditable, so leaving it dirty would hand the adopter the step that does the proving
    git -C "$ROOT" commit -q -m "chore: vendor Checkwright kits ($PROFILE profile, v$VERSION)" \
        || die "the commit failed — the files are staged; commit them yourself to finish the install" "" 1
    printf 'INIT: vendored %d kit(s) at the %s profile (v%s) and committed them.\n' \
        "${#KITS[@]}" "$PROFILE" "$VERSION"
else
    printf 'INIT: vendored %d kit(s) at the %s profile (v%s) and staged them — --no-commit, so the commit is yours.\n' \
        "${#KITS[@]}" "$PROFILE" "$VERSION"
fi

printf '\nnext:\n'
printf '  bash gate-sdk/bin/install-hooks.sh   # opt this clone into the generated pre-commit hook\n'
printf '  bash gate-sdk/bin/run-gates.sh       # the battery, green on what was just vendored\n'
exit 0
