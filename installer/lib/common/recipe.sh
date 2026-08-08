# shellcheck shell=bash
# spec: installer/README.md §What init seeds — sourceable owner of the per-kit install recipe: the starting gate roster a kit registers in a fresh consumer, and the seam surfaces it needs before those gates can run. Two output channels, and the difference is who writes the file: a bare repo-relative path is one this module seeded itself and init need only claim for the roster, while a '<src>\t<dest>' pair is a plan init performs through copy_in. Both feed the manifest's files[]

# shellcheck disable=SC2034  # QUEUE_FILE is read by this module's callers, not by the module
# spec: installer/README.md §What init seeds — the consumer-layout names init writes against; they are gate-sdk's and canon-kit's own defaults, so a tree init made is the zero-config tree those kits expect. They live here rather than in one verb because uninstall must trim the doctrine span out of the same agent file init wrote it into: doctrine-kit's own DOCTRINE_KIT_AGENT_FILE knob cannot answer that, since an adopter override would not change the path init actually used, so the name needs one writer both verbs read
GATES_DIR="scripts"
AGENT_FILE="CLAUDE.md"
QUEUE_FILE="TASK-QUEUE.md"

# spec: installer/README.md §What init seeds — the config seam is derived, never listed: a kit's consumer config is whatever `templates/*-config.sh` it ships, and the destination is always the gates dir under the file's own name. It plans and writes nothing, because the seam is rewritten on every run and is the file class whose whole purpose is to be edited: a copy landing before claim() hashes the tree destroys the very evidence the refusal is computed from, so the write belongs to copy_in
recipe_config_seam_plan() {   # $1 = kit payload dir, $2 = gates dir -> one '<src><TAB><dest>' line per config template
    local t base
    shopt -s nullglob
    for t in "$1"/templates/*-config.sh; do
        base="${t##*/}"
        printf '%s\t%s/%s\n' "$t" "$2" "$base"
    done
    shopt -u nullglob
}

# spec: gate-sdk/SPEC.md §The install disposition — the gate is the one place that knows whether it can run on a tree init just made, so this reads the declaration off each shipped gate instead of carrying a second copy of it. A kit that adds a zero-config gate is picked up with no edit here, which is the exposure this replaces: a roster maintained here could never learn about it
recipe_install_disposition() {   # $1 = gate file -> the disposition its header declares, empty when it declares none
    awk 'sub(/^# install:[[:space:]]+/, "") { print $1; exit }' "$1"
}

# spec: installer/README.md §What init seeds — the starting roster is the subset a fresh consumer begins with, not the kit's full roster: a gate whose subject the adopter has not authored yet (a glossary, a skills directory, a docs host) would exit 2 on their tree, so it is registered when that surface exists rather than at install
# spec: installer/README.md §Profiles — the roster is keyed by profile as well as by kit, so a roster that varies by profile becomes a change to one arm rather than a change to this signature and to every caller reading it. No disposition varies on it today; the parameter is the seam, and its reader is profile_gates
# spec: gate-sdk/SPEC.md §Consumer payload — both declaration spellings are scanned, because a ported gate is still a gate a kit ships and its descriptor rides the payload on the same terms as a shell implementation
recipe_gates() {   # $1 = kit payload dir, $2 = profile -> the gates it registers in a fresh consumer, none for a kit that ships no zero-config gate
    local pay="$1" f name
    [[ -d "$pay/checks" ]] || return 0
    shopt -s nullglob
    for f in "$pay"/checks/check-*.sh "$pay"/checks/check-*.gate; do
        name="${f##*/}"
        [[ "$(recipe_install_disposition "$f")" == zero-config ]] && printf '%s\n' "${name%.*}"
    done | LC_ALL=C sort -u
    shopt -u nullglob
}

recipe_needs_queue() {   # $1 = kit name -> 0 iff one of its starting gates reads the queue file
    case "$1" in canon-kit|lifecycle-kit|queue-kit) return 0 ;; *) return 1 ;; esac
}

recipe_needs_agent_file() {   # $1 = kit name -> 0 iff one of its starting gates reads the always-loaded agent file
    case "$1" in context-kit|doctrine-kit) return 0 ;; *) return 1 ;; esac
}

# spec: installer/README.md §What init seeds — seed what is absent, plan what must be claimed. A surface init creates once and then leaves alone is written here only when it is absent, which is what keeps a re-run non-destructive on a tree that has grown since; a surface init rewrites on every run is never written here at all but planned for copy_in, because only claim() can compare the adopter's content before the overwrite lands. The two disciplines are not interchangeable and each arm below takes the one its surface needs
recipe_seed() {   # $1 = kit name, $2 = kit payload dir, $3 = consumer root, $4 = queue file; prints seeded paths and '<src><TAB><dest>' plans
    local kit="$1" pay="$2" root="$3" queue="$4"

    if recipe_needs_queue "$kit" && [[ ! -f "$root/$queue" ]]; then
        if [[ -f "$pay/templates/TASK-QUEUE.md" ]]; then
            cp "$pay/templates/TASK-QUEUE.md" "$root/$queue" || return 1
        else
            printf '%s\n' "# $queue" "" "## Iteration: —" "" "---" "" \
                "## New Features" "" "## Technical Debt" "" "## Deferred" "" "## Done" \
                > "$root/$queue" || return 1
        fi
        printf '%s\n' "$queue"
    fi

    case "$kit" in
        gate-sdk)
            [[ -f "$pay/templates/msg-patterns.list" ]] || return 0
            printf '%s\t%s/msg-patterns.list\n' "$pay/templates/msg-patterns.list" "$GATES_DIR" ;;
        evidence-kit)
            mkdir -p "$root/.workflow" || return 1
            if [[ ! -f "$root/.workflow/validate-baseline.txt" ]]; then
                printf '%s\n' "# contract: evidence-kit/SPEC.md §Baseline manifest — held-constant validate baseline: <suite> <scenario> <status> [<slug>]" \
                    > "$root/.workflow/validate-baseline.txt" || return 1
                printf '%s\n' ".workflow/validate-baseline.txt"
            fi
            if [[ ! -f "$root/.workflow/validate-evidence.txt" ]]; then
                printf '%s\n' "# contract: evidence-manifest v1" \
                    > "$root/.workflow/validate-evidence.txt" || return 1
                printf '%s\n' ".workflow/validate-evidence.txt"
            fi ;;
        lifecycle-kit)
            mkdir -p "$root/.workflow" || return 1
            [[ -f "$root/.workflow/WORKFLOW-STATE.txt" ]] && return 0
            printf '%s\n' "# contract: lifecycle-kit/SPEC.md §check-stage-evidence" \
                "# One data line per stage-skill invocation: <iteration> <stage> <session-id> <date>." \
                "" "---" "" > "$root/.workflow/WORKFLOW-STATE.txt" || return 1
            printf '%s\n' ".workflow/WORKFLOW-STATE.txt" ;;
        doctrine-kit)
            # spec: doctrine-kit/SPEC.md §install-doctrine — the reference block is the kit's own installer's to write, so init calls it rather than restating the block it emits
            ( cd "$root" && bash "$pay/bin/install-doctrine.sh" "$AGENT_FILE" "doctrine-kit/DOCTRINE.md" ) >/dev/null || return 1
            printf '%s\n' "$AGENT_FILE" ;;
        *) : ;;
    esac
}
