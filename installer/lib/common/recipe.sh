# shellcheck shell=bash
# spec: installer/README.md §What init seeds — sourceable owner of the per-kit install recipe: the starting gate roster a kit registers in a fresh consumer, and the seam surfaces it needs before those gates can run. Every function here prints one repo-relative path per file it wrote, which is what the manifest's files[] is built from

# spec: installer/README.md §What init seeds — the config seam is derived, never listed: a kit's consumer config is whatever `templates/*-config.sh` it ships, and the destination is always the gates dir under the file's own name
recipe_config_seam() {   # $1 = kit payload dir, $2 = consumer root, $3 = gates dir
    local t base dest
    shopt -s nullglob
    for t in "$1"/templates/*-config.sh; do
        base="${t##*/}"
        dest="$3/$base"
        cp "$t" "$2/$dest" || return 1
        printf '%s\n' "$dest"
    done
    shopt -u nullglob
}

# spec: installer/README.md §What init seeds — the starting roster is the subset a fresh consumer begins with, not the kit's full roster: a gate whose subject the adopter has not authored yet (a glossary, a skills directory, a docs host) would exit 2 on their tree, so it is registered when that surface exists rather than at install
recipe_gates() {   # $1 = kit name -> the gates it registers in a fresh consumer, none for a kit that ships no zero-config gate
    case "$1" in
        gate-sdk)
            printf '%s\n' check-shellcheck check-gate-output check-gate-fail-closed \
                check-gate-fixture-coverage check-gate-exemption-tasks \
                check-gate-assertions check-graph check-commit-msg check-tree-terms ;;
        canon-kit)
            printf '%s\n' check-amendment-queue check-spec-dod-singleton \
                check-spec-derivable-section check-spec-embedded-source ;;
        context-kit)    printf '%s\n' check-brevity ;;
        delegation-kit) printf '%s\n' check-gate-tamper ;;
        doctrine-kit)   printf '%s\n' check-doctrine-registration ;;
        evidence-kit)   printf '%s\n' check-evidence-baseline check-evidence-manifest ;;
        # spec: installer/README.md §What init seeds — lifecycle-kit registers nothing at install: its gates read an attestation only a stage session can write, so they arm at the adopter's first stamp
        lifecycle-kit)  : ;;
        queue-kit)
            printf '%s\n' check-queue-hygiene check-queue-wrap check-tag-lead-line \
                check-task-names check-task-conservation check-queue-prose-precondition ;;
        *) : ;;
    esac
}

recipe_needs_queue() {   # $1 = kit name -> 0 iff one of its starting gates reads the queue file
    case "$1" in canon-kit|lifecycle-kit|queue-kit) return 0 ;; *) return 1 ;; esac
}

recipe_needs_agent_file() {   # $1 = kit name -> 0 iff one of its starting gates reads the always-loaded agent file
    case "$1" in context-kit|doctrine-kit) return 0 ;; *) return 1 ;; esac
}

# spec: installer/README.md §What init seeds — a seam surface is written only when it is absent: init owns what it created, never what the adopter authored, which is what keeps a re-run non-destructive on a tree that has grown since
recipe_seed() {   # $1 = kit name, $2 = kit payload dir, $3 = consumer root, $4 = queue file
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
            cp "$pay/templates/msg-patterns.list" "$root/$GATES_DIR/msg-patterns.list" || return 1
            printf '%s\n' "$GATES_DIR/msg-patterns.list" ;;
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
