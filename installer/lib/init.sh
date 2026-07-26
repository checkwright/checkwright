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

# spec: installer/README.md §What init seeds — the consumer-layout names init writes against; they are gate-sdk's and canon-kit's own defaults, so a tree init made is the zero-config tree those kits expect
GATES_DIR="scripts"
AGENT_FILE="CLAUDE.md"
QUEUE_FILE="TASK-QUEUE.md"

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
ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
    || die "not inside a git work tree" \
       "the vendored source is meant to be committed, which is what makes it auditable. Run 'git init' first, or run init inside the repository you want governed."

# spec: installer/README.md §init — the clean-worktree precondition exists so the one commit init makes is exactly what it vendored; --no-commit is its valve, because an operator staging the vendoring themselves has taken that guarantee on
if (( ! DRY )) && (( DO_COMMIT )) && [[ -n "$(git -C "$ROOT" status --porcelain)" ]]; then
    die "the worktree is not clean" \
        "init makes one commit, and a dirty tree would fold your work into it — so a reviewer's diff would no longer be the whole of what was vendored. Commit or stash first, or pass --no-commit to stage the vendoring yourself." 1
fi

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

prior_hash() {   # $1 = repo-relative path -> the hash the manifest recorded for it, empty when unrecorded
    [[ -n "$PRIOR_FILES" ]] || return 0
    while IFS=$'\t' read -r p h; do
        [[ "$p" == "$1" ]] && { printf '%s' "$h"; return 0; }
    done <<<"$PRIOR_FILES"
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
    return 1
}

WRITTEN=()
record() { WRITTEN+=("$1"); }

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
    plan_gates() {
        printf '%s\n' "# Checkwright gate registry — written by 'checkwright init' (profile: $PROFILE)." \
            "# Each kit's starting subset; its README names the full roster to grow into."
        for kit in "${KITS[@]}"; do
            mapfile -t g < <(recipe_gates "$kit")
            [[ ${#g[@]} -gt 0 ]] || continue
            printf '# %s\n' "$kit"
            printf '%s\n' "${g[@]}"
        done
    }
    (( DRY )) || plan_gates > "$ROOT/$GATES_LIST" || die "could not write $GATES_LIST"
    record "$GATES_LIST"
fi

for kit in "${KITS[@]}"; do
    while IFS= read -r p; do
        [[ -n "$p" ]] || continue
        claim "$p" && record "$p"
    done < <(
        if (( DRY )); then
            # spec: installer/README.md §init — --dry-run resolves the same seam plan the real run would, by listing what each recipe would touch without letting it write
            recipe_config_seam_names() {
                local t
                shopt -s nullglob
                for t in "$PAYLOAD/$kit"/templates/*-config.sh; do printf '%s/%s\n' "$GATES_DIR" "${t##*/}"; done
                shopt -u nullglob
            }
            recipe_config_seam_names
            recipe_needs_queue "$kit" && [[ ! -f "$ROOT/$QUEUE_FILE" ]] && printf '%s\n' "$QUEUE_FILE"
            case "$kit" in
                gate-sdk)      printf '%s/msg-patterns.list\n' "$GATES_DIR" ;;
                evidence-kit)  printf '%s\n' .workflow/validate-baseline.txt .workflow/validate-evidence.txt ;;
                lifecycle-kit) printf '%s\n' .workflow/WORKFLOW-STATE.txt ;;
                doctrine-kit)  printf '%s\n' "$AGENT_FILE" ;;
            esac
        else
            recipe_config_seam "$PAYLOAD/$kit" "$ROOT" "$GATES_DIR" || die "could not seed $kit's config seam"
            if recipe_needs_agent_file "$kit" && [[ ! -f "$ROOT/$AGENT_FILE" ]]; then
                # spec: installer/README.md §What init seeds — the seeded agent file carries the section heading context-kit's brevity gate reads by default, so the gate init registers has the surface it was pointed at from the first commit
                printf '%s\n' "# $AGENT_FILE" "" \
                    "Resident instructions for agent sessions in this repository." "" \
                    "## Shared conventions" "" \
                    "- **Terse:** one line per rule here; the mechanism behind the pointer." \
                    > "$ROOT/$AGENT_FILE" || die "could not seed $AGENT_FILE"
                printf '%s\n' "$AGENT_FILE"
            fi
            recipe_seed "$kit" "$PAYLOAD/$kit" "$ROOT" "$QUEUE_FILE" || die "could not seed $kit"
        fi
    )
done

# spec: installer/README.md §What init seeds — a guarded seed is written once and kept thereafter, so a re-run must re-claim it: dropping it from the manifest would disown a file init created, and an uninstall that reads this roster would then leave it behind
under_kit() {   # $1 = repo-relative path -> 0 iff it lies inside one of this profile's kit directories
    local k
    for k in "${KITS[@]}"; do [[ "$1" == "$k/"* ]] && return 0; done
    return 1
}
if [[ -n "$PRIOR_FILES" ]]; then
    while IFS=$'\t' read -r p h; do
        [[ -n "$p" ]] || continue
        under_kit "$p" && continue
        printf '%s\n' "${WRITTEN[@]}" | grep -qxF "$p" && continue
        [[ -f "$ROOT/$p" ]] || continue
        [[ "$(lock_hash "$ROOT/$p")" == "$h" ]] || continue
        record "$p"
    done <<<"$PRIOR_FILES"
fi

# spec: installer/README.md §init — the generated projections are produced by the vendored tools themselves, never restated by the installer: the hook generator and the graph emitter are gate-sdk's, so a consumer's artifacts are the ones their own gate-sdk makes
GENERATED=("$GATES_DIR/git-hooks/pre-commit" ".workflow/CHECK-GRAPH.html")
if (( ! DRY )); then
    ( cd "$ROOT" && bash "$ROOT/gate-sdk/bin/gen-pre-commit.sh" --write ) >/dev/null \
        || die "gate-sdk's hook generator failed"
    ( cd "$ROOT" && bash "$ROOT/gate-sdk/checks/check-graph.sh" --emit ) > "$ROOT/.workflow/CHECK-GRAPH.html" \
        || die "gate-sdk's graph emitter failed"
fi
[[ -f "$ROOT/$GATES_DIR/git-hooks/commit-msg" ]] && GENERATED+=("$GATES_DIR/git-hooks/commit-msg")
for g in "${GENERATED[@]}"; do
    (( DRY )) || [[ -f "$ROOT/$g" ]] || continue
    record "$g"
done

manifest() {
    local f
    {
        printf '{"schema":%s,"version":%s,"commit":%s,"profile":%s,"kits":' \
            "$(jq -Rn --arg v "$CHECKWRIGHT_LOCK_SCHEMA" '$v')" \
            "$(jq -Rn --arg v "$VERSION" '$v')" \
            "$(jq -Rn --arg v "$COMMIT" '$v')" \
            "$(jq -Rn --arg v "$PROFILE" '$v')"
        printf '%s\n' "${KITS[@]}" | jq -Rn '[inputs]'
        printf ',"files":{'
        local first=1
        for f in "${WRITTEN[@]}"; do
            (( first )) || printf ','
            first=0
            printf '%s:%s' "$(jq -Rn --arg v "$f" '$v')" \
                "$(jq -Rn --arg v "$( (( DRY )) && [[ ! -f "$ROOT/$f" ]] && printf '(pending)' || lock_hash "$ROOT/$f")" '$v')"
        done
        printf '}}'
    } | jq -S .
}

if (( DRY )); then
    printf 'checkwright init --dry-run (profile: %s, version: %s)\n\n' "$PROFILE" "$VERSION"
    printf 'would vendor %d kit(s): %s\n' "${#KITS[@]}" "${KITS[*]}"
    printf 'would write %d file(s), including:\n' "$(( ${#WRITTEN[@]} + 1 ))"
    printf '  %s\n' "$GATES_LIST" "$CHECKWRIGHT_LOCK_FILE"
    for kit in "${KITS[@]}"; do printf '  %s/ (%d files)\n' "$kit" "$(find "$PAYLOAD/$kit" -type f | wc -l)"; done
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

if [[ ${#CHANGED[@]} -gt 0 ]]; then
    printf '\n%d file(s) have changed since init wrote them and were left alone:\n' "${#CHANGED[@]}"
    printf '  %s\n' "${CHANGED[@]}"
    printf '  help: review the differences; re-run with --force to take the packaged version.\n\n'
fi

git -C "$ROOT" add -- "${WRITTEN[@]}" || die "could not stage the vendored files"

# spec: installer/README.md §init — idempotence is a property of the tree, so a re-run that changed nothing reports and exits clean rather than failing on an empty commit: "nothing to do" is the success case, not an error
if git -C "$ROOT" diff --cached --quiet; then
    printf 'INIT: already at the %s profile (v%s) — %d file(s) checked, nothing to change.\n' \
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
