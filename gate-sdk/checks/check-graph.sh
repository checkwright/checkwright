#!/usr/bin/env bash
# graph: couples=scripts/gates.list,scripts/*.sh,kit:*.sh,scripts/git-hooks/pre-commit,.workflow/CHECK-GRAPH.html,docs/check-graph.html,SPEC-*.md,*/SPEC-*.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-graph — manifest well-formedness, trigger parity, cycle valves, artifact drift, and amendment-body manifest validation (assertion G)
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR="$(gate_sdk_gates_dir)"
LIST="$GATES_DIR/gates.list"
WORKFLOW_DIR="${GATE_SDK_WORKFLOW_DIR:-.workflow}"
HOOK="${GATE_SDK_HOOKS_DIR:-$GATES_DIR/git-hooks}/pre-commit"
GEN="$SDK/bin/gen-pre-commit.sh"

GRAPH_VOCAB=()
GRAPH_LEADING=()
GRAPH_LAGGING=()
GRAPH_LAYERS=()
VOCAB_FILE="${GATE_SDK_GRAPH_VOCAB:-$GATES_DIR/graph-vocab.sh}"
if [[ -f "$VOCAB_FILE" ]]; then
    # shellcheck disable=SC1090  # consumer-supplied rule content, path is config
    source "$VOCAB_FILE"
fi

in_set() { local t="$1"; shift; local v; for v in "$@"; do [[ "$t" == "$v" ]] && return 0; done; return 1; }

surface_layer() {
    if declare -F graph_surface_layer >/dev/null; then
        graph_surface_layer "$1"
    else
        echo surfaces
    fi
}

layer_specs() {
    if [[ ${#GRAPH_LAYERS[@]} -gt 0 ]]; then
        printf '%s\n' "${GRAPH_LAYERS[@]}"
    else
        printf '%s\n' 'surfaces:governed surfaces'
    fi
}

mapfile -t RESOLVE_DIRS < <(gate_check_dirs)
resolve_member() {
    gate_resolve "$1" "${RESOLVE_DIRS[@]}"
}

emit_graph() {
    local checks
    local resolved_artifact="${GATE_SDK_GRAPH_ARTIFACT:-$WORKFLOW_DIR/CHECK-GRAPH.html}"
    mapfile -t checks < <(gates_list_members "$LIST" | sort -u)
    declare -A C_COUPLES C_DIR C_VALVE NODE_SEEN
    local c src man kv couples dir valve s
    for c in "${checks[@]}"; do
        src="$(resolve_member "$c" || true)"
        [[ -n "$src" ]] || continue
        man="$(grep -m1 '^# graph: ' "$src" 2>/dev/null || true)"
        [[ -n "$man" ]] || continue
        couples=""; dir=""; valve=""
        for kv in ${man#\# graph: }; do
            case "$kv" in
                couples=*) couples="${kv#couples=}" ;;
                dir=*)     dir="${kv#dir=}" ;;
                valve=*)   valve="${kv#valve=}" ;;
            esac
        done
        couples="$(gate_expand_couples "$couples")"
        C_COUPLES[$c]="$couples"; C_DIR[$c]="$dir"; C_VALVE[$c]="$valve"
        local -a csurf; IFS=',' read -ra csurf <<<"$couples"
        for s in "${csurf[@]}"; do NODE_SEEN[$s]=1; done
    done

    local nodes; mapfile -t nodes < <(printf '%s\n' "${!NODE_SEEN[@]}" | sort)
    declare -A NODE_ID; local i=0 n
    for n in "${nodes[@]}"; do NODE_ID[$n]="n$i"; i=$((i+1)); done

    local -a edge_lines=() proposed_idx=(); local idx=0 arrow src_id tgt j
    local -a surf
    for c in "${checks[@]}"; do
        [[ -n "${C_COUPLES[$c]:-}" ]] || continue
        IFS=',' read -ra surf <<<"${C_COUPLES[$c]}"
        [[ "${C_DIR[$c]}" == bi ]] && arrow="<-->" || arrow="-->"
        src_id="${NODE_ID[${surf[0]}]}"
        if [[ ${#surf[@]} -eq 1 ]]; then
            edge_lines+=("  $src_id $arrow|\"$c\"| $src_id")
            [[ "${C_VALVE[$c]}" == PROPOSED ]] && proposed_idx+=("$idx"); idx=$((idx+1))
        else
            for ((j=1; j<${#surf[@]}; j++)); do
                tgt="${NODE_ID[${surf[$j]}]}"
                edge_lines+=("  $src_id $arrow|\"$c\"| $tgt")
                [[ "${C_VALVE[$c]}" == PROPOSED ]] && proposed_idx+=("$idx"); idx=$((idx+1))
            done
        fi
    done

    local graph_text spec layer label decl joined
    graph_text="$(
        printf 'graph LR\n'
        while IFS= read -r spec; do
            layer="${spec%%:*}"; label="${spec#*:}"
            decl=""
            for n in "${nodes[@]}"; do
                [[ "$(surface_layer "$n")" == "$layer" ]] || continue
                decl+="    ${NODE_ID[$n]}[\"$n\"]"$'\n'
            done
            [[ -n "$decl" ]] || continue
            printf '  subgraph %s["%s"]\n' "$layer" "$label"
            printf '%s' "$decl"
            printf '  end\n'
        done < <(layer_specs)
        [[ ${#edge_lines[@]} -gt 0 ]] && printf '%s\n' "${edge_lines[@]}"
        if [[ ${#proposed_idx[@]} -gt 0 ]]; then
            joined="$(IFS=,; echo "${proposed_idx[*]}")"
            printf '  linkStyle %s stroke:#d97706,stroke-width:2px;\n' "$joined"
        fi
    )"

    printf '%s\n' '<!DOCTYPE html>'
    cat <<'COMMENT_HEAD'
<!--
  The check-coupling graph: which content surfaces each gates.list gate binds
  together.

  GENERATED, DO NOT EDIT. Source of truth is the per-gate `# graph:` manifests;
COMMENT_HEAD
    printf '  regenerate with:  bash gate-sdk/checks/check-graph.sh --emit > %s\n' "$resolved_artifact"
    cat <<'HEAD'
  check-graph (assertion E) fails if this file drifts from them.

  Reading it: nodes are surfaces (grouped by layer when the consumer's
  graph-vocab.sh declares layers); an edge is a gate. A bidirectional edge is a
  coupling bijection; a self-loop is a gate that guards one surface's internal
  consistency. Amber edges are cycle valves (valve=PROPOSED) - couplings where a
  leading surface may run ahead via a queue-tracked PROPOSED marker.

  Comment hygiene: arrows are named, never drawn, in this comment - an HTML
  comment-close sequence inside it would end the block and leak the rest.
-->
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Check-coupling graph</title>
  <style>
    :root { color-scheme: light dark; }
    body { margin: 0; font: 15px/1.5 system-ui, sans-serif;
           background: #fff; color: #1a1a1a; }
    header { padding: 1rem 1.5rem; border-bottom: 1px solid #ddd; }
    header h1 { margin: 0 0 .35rem; font-size: 1.25rem; }
    header p { margin: .25rem 0; max-width: 68rem; }
    .status { font-size: .85rem; opacity: .7; }
    main { padding: 1rem 1.5rem; }
    .legend { margin-bottom: .75rem; font-size: .9rem; }
    .legend .valve { display: inline-block; width: 22px; border-top: 3px solid #d97706; }
    .viewport { border: 1px solid #ddd; border-radius: 8px; padding: 16px;
                background: #fafafa; overflow: auto; height: 78vh; cursor: grab; }
    .viewport.grabbing { cursor: grabbing; }
    .viewport svg { transform-origin: 0 0; max-width: none; }
    .hint { margin: .5rem 0 0; font-size: .8rem; opacity: .6; }
    @media (prefers-color-scheme: dark) {
      body { background: #16181d; color: #e6e6e6; }
      header { border-color: #333; }
      .viewport { border-color: #333; background: #0f1115; }
    }
  </style>
</head>
<body>
  <header>
    <h1>Check-coupling graph</h1>
    <p>Which content surfaces each <code>gates.list</code> gate binds together.
       Source of truth is the per-gate <code>graph:</code> manifests; this is
       their projection, regenerated by
       <code>bash gate-sdk/checks/check-graph.sh --emit</code> and drift-checked
       by assertion E.</p>
    <p>Nodes are surfaces; an edge is a gate. A bidirectional edge is a coupling
       bijection; a self-loop guards one surface. Amber edges are cycle valves
       (<code>valve=PROPOSED</code>) - where a leading surface may run ahead via
       a queue-tracked marker.</p>
    <span class="status">Generated from the gate manifests - do not hand-edit.</span>
  </header>

  <main>
    <div class="legend">
      <span><span class="valve"></span> cycle valve (valve=PROPOSED)</span>
    </div>
    <div class="mermaid viewport" id="diagram"></div>
    <p class="hint">Ctrl/&#8984; + scroll to zoom &middot; drag to pan &middot; double-click to reset.</p>
  </main>

  <script type="module">
    import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
    const dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const graph = `%%{init:{'theme':'${dark ? 'dark' : 'base'}','themeVariables':{'fontSize':'13px'}}}%%
HEAD
    printf '%s\n' "$graph_text"
    cat <<'TAIL'
`;
    mermaid.initialize({ startOnLoad: false });
    const { svg } = await mermaid.render('graph', graph);
    const vp = document.getElementById('diagram');
    vp.innerHTML = svg;

    // Pan/zoom (self-contained; no external lib) — the platform diagram-assets
    // approach: render the SVG at its natural, readable size in an overflow:auto
    // box; Ctrl/Cmd+wheel scales the SVG, drag pans via native scroll, and
    // double-click resets the zoom. Never shrink-to-fit — that makes a dense
    // graph unreadable.
    const el = vp.querySelector('svg');
    el.style.transformOrigin = '0 0';
    let zoom = 1;
    const applyZoom = () => { el.style.transform = 'scale(' + zoom + ')'; };

    vp.addEventListener('wheel', (e) => {
      if (!(e.ctrlKey || e.metaKey)) return;
      e.preventDefault();
      zoom = Math.min(6, Math.max(0.4, zoom * (e.deltaY < 0 ? 1.1 : 1 / 1.1)));
      applyZoom();
    }, { passive: false });

    let down = false, lx = 0, ly = 0;
    vp.addEventListener('pointerdown', (e) => {
      down = true; lx = e.clientX; ly = e.clientY;
      vp.classList.add('grabbing'); vp.setPointerCapture(e.pointerId);
    });
    vp.addEventListener('pointermove', (e) => {
      if (!down) return;
      vp.scrollLeft -= e.clientX - lx; vp.scrollTop -= e.clientY - ly;
      lx = e.clientX; ly = e.clientY;
    });
    const end = () => { down = false; vp.classList.remove('grabbing'); };
    vp.addEventListener('pointerup', end);
    vp.addEventListener('pointercancel', end);
    vp.addEventListener('dblclick', () => { zoom = 1; applyZoom(); });
  </script>
</body>
</html>
TAIL
}

# spec: gate-sdk/SPEC.md §check-graph (assertion G) — validate `# graph:` manifests in SPEC-*.md amendment bodies
valid_glob_token() {
    local t="$1"
    [[ -n "$t" ]] || return 1
    t="${t#kit:}"  # the kit:<glob> couples form validates on its glob part
    [[ "$t" =~ ^[A-Za-z0-9._*?/-]+$ ]]
}

extract_amend_manifests() {
    awk '
    function emit_inline(line,   s, span) {
        s = line
        while (match(s, /`# graph: [^`]+`/)) {
            span = substr(s, RSTART, RLENGTH)
            sub(/^`# graph: /, "", span); sub(/`$/, "", span)
            print FILENAME "\t" span
            s = substr(s, RSTART + RLENGTH)
        }
    }
    FNR == 1 { infence = 0; flang = "" }
    /^[[:space:]]*```/ {
        if (infence) { infence = 0; flang = ""; next }
        infence = 1; flang = $0
        sub(/^[[:space:]]*`+/, "", flang); sub(/[^A-Za-z0-9_-].*$/, "", flang)
        next
    }
    {
        if (infence) {
            if (flang == "proto") next
            if ($0 ~ /^# graph: /) { span = $0; sub(/^# graph: /, "", span); print FILENAME "\t" span }
            next
        }
        emit_inline($0)
    }
    ' "$@"
}

validate_amend_manifest() {
    local file="$1" span="$2" tok
    [[ "$span" =~ (^|[[:space:]])(couples|dir|valve|tier|mode|trigger|gen)= ]] || return 0
    local couples="" dir="" valve="" tier="" mode="" trigger="" gen=""
    local have_couples=0 have_dir=0 have_valve=0 have_tier=0
    local -a unknown=()
    for tok in $span; do
        case "$tok" in
            couples=*) couples="${tok#couples=}"; have_couples=1 ;;
            dir=*)     dir="${tok#dir=}";         have_dir=1 ;;
            valve=*)   valve="${tok#valve=}";     have_valve=1 ;;
            tier=*)    tier="${tok#tier=}";       have_tier=1 ;;
            mode=*)    mode="${tok#mode=}" ;;
            trigger=*) trigger="${tok#trigger=}" ;;
            gen=*)     gen="${tok#gen=}" ;;
            *)         unknown+=("$tok") ;;
        esac
    done
    local where="AMEND-MANIFEST: $file"
    (( have_couples )) || g_errors+=("$where: missing required key 'couples='")
    (( have_dir ))     || g_errors+=("$where: missing required key 'dir='")
    (( have_valve ))   || g_errors+=("$where: missing required key 'valve='")
    (( have_tier ))    || g_errors+=("$where: missing required key 'tier='")
    local u
    for u in "${unknown[@]+"${unknown[@]}"}"; do
        g_errors+=("$where: unknown manifest key/token '$u'")
    done
    if (( have_dir )) && [[ "$dir" != bi && "$dir" != one ]]; then
        g_errors+=("$where: dir= must be bi|one (got '$dir')")
    fi
    if (( have_valve )) && [[ "$valve" != none && "$valve" != PROPOSED ]]; then
        g_errors+=("$where: valve= must be none|PROPOSED (got '$valve')")
    fi
    if (( have_tier )) && [[ "$tier" != precommit && "$tier" != align-only && "$tier" != commit-msg ]]; then
        g_errors+=("$where: tier= must be precommit|align-only|commit-msg (got '$tier')")
    fi
    if [[ -n "$mode" && "$mode" != staged && "$mode" != whole-tree ]]; then
        g_errors+=("$where: mode= must be staged|whole-tree (got '$mode')")
    fi
    if [[ -n "$gen" && "$gen" != manual ]]; then
        g_errors+=("$where: gen= must be manual (got '$gen')")
    fi
    if (( have_couples )) && [[ -z "$couples" ]]; then
        g_errors+=("$where: couples= is empty")
    fi
    local s
    local -a parts
    if [[ -n "$couples" ]]; then
        IFS=',' read -ra parts <<<"$couples"
        for s in "${parts[@]}"; do
            valid_glob_token "$s" || g_errors+=("$where: couples token '$s' is not a syntactically valid glob/path")
        done
    fi
    if [[ -n "$trigger" ]]; then
        IFS=',' read -ra parts <<<"$trigger"
        for s in "${parts[@]}"; do
            [[ "$s" == '*' ]] && continue
            valid_glob_token "$s" || g_errors+=("$where: trigger token '$s' is not a syntactically valid glob/path")
        done
    fi
}

amendment_findings() {
    local root="$1" file span
    local -a files
    mapfile -t files < <(find "$root" -name 'SPEC-*.md' -not -path '*/target/*' -not -path '*/.git/*' -not -path '*/gate-tests/*' 2>/dev/null | sort)
    [[ ${#files[@]} -gt 0 ]] || return 0
    while IFS=$'\t' read -r file span; do
        [[ -n "$span" ]] || continue
        validate_amend_manifest "$file" "$span"
    done < <(extract_amend_manifests "${files[@]}")
}

if [[ "${1:-}" == "--amend-only" ]]; then
    g_errors=()
    amendment_findings "${2:-.}"
    if [[ ${#g_errors[@]} -gt 0 ]]; then
        echo "CHECK-GRAPH: ${#g_errors[@]} amendment-manifest violation(s):"
        printf '  %s\n' "${g_errors[@]}"
        echo "  help: fix the malformed '# graph:' manifest in the SPEC-*.md amendment body (required keys couples/dir/valve/tier; dir=bi|one valve=none|PROPOSED tier=precommit|align-only; couples tokens must be syntactically valid globs)"
        exit 1
    fi
    echo "CHECK-GRAPH: clean (amendment-body '# graph:' manifests well-formed)"
    exit 0
fi

[[ -f "$LIST" ]] || { echo "check-graph: no registry at $LIST" >&2; exit 2; }

[[ "${1:-}" == "--emit" ]] && { emit_graph; exit 0; }

in_vocab() {
    [[ ${#GRAPH_VOCAB[@]} -eq 0 ]] && return 0
    local t="$1" v
    for v in "${GRAPH_VOCAB[@]}"; do [[ "$t" == "$v" ]] && return 0; done
    return 1
}

covered_by() {
    local s="$1"; shift
    local t
    for t in "$@"; do
        [[ "$t" == "*" ]] && return 0
        [[ "$t" == "$s" ]] && return 0
        # shellcheck disable=SC2053  # $t is deliberately unquoted: it is the glob
        [[ "$s" != *[\*\?]* && "$s" == $t ]] && return 0
        [[ "$t" == \*.* && "$s" == *"${t#\*}" ]] && return 0
    done
    return 1
}

mapfile -t CHECKS < <(gates_list_members "$LIST")
[[ ${#CHECKS[@]} -gt 0 ]] || { echo "check-graph: no members parsed from $LIST" >&2; exit 2; }

[[ -f "$GEN" ]] || { echo "check-graph: gen-pre-commit.sh not found at $GEN" >&2; exit 2; }

# assertion A: every gates.list member has a well-formed `# graph:` line
errors=()
has_msg_gate=0

for c in "${CHECKS[@]}"; do
    if ! script="$(resolve_member "$c")"; then
        errors+=("MANIFEST: $c is in gates.list but resolves in none of: ${RESOLVE_DIRS[*]}")
        continue
    fi
    man="$(grep -m1 '^# graph: ' "$script" || true)"
    if [[ -z "$man" ]]; then
        errors+=("MANIFEST: $script has no '# graph:' manifest line")
        continue
    fi
    couples=""; dir=""; valve=""; tier=""; mode=""; trigger=""; gen=""
    for kv in ${man#\# graph: }; do
        case "$kv" in
            couples=*) couples="${kv#couples=}" ;;
            dir=*)     dir="${kv#dir=}" ;;
            valve=*)   valve="${kv#valve=}" ;;
            tier=*)    tier="${kv#tier=}" ;;
            mode=*)    mode="${kv#mode=}" ;;
            trigger=*) trigger="${kv#trigger=}" ;;
            gen=*)     gen="${kv#gen=}" ;;
            *) errors+=("MANIFEST: $script unknown manifest key '$kv'") ;;
        esac
    done
    couples="$(gate_expand_couples "$couples")"
    [[ -n "$trigger" ]] && trigger="$(gate_expand_couples "$trigger")"
    [[ "$dir" == bi || "$dir" == one ]]            || errors+=("MANIFEST: $script dir= must be bi|one (got '$dir')")
    [[ "$valve" == none || "$valve" == PROPOSED ]] || errors+=("MANIFEST: $script valve= must be none|PROPOSED (got '$valve')")
    [[ "$tier" == precommit || "$tier" == align-only || "$tier" == commit-msg ]] || errors+=("MANIFEST: $script tier= must be precommit|align-only|commit-msg (got '$tier')")
    [[ "$tier" == commit-msg ]] && has_msg_gate=1
    [[ -z "$mode" || "$mode" == staged || "$mode" == whole-tree ]] || errors+=("MANIFEST: $script mode= must be staged|whole-tree (got '$mode')")
    [[ -z "$gen" || "$gen" == manual ]] || errors+=("MANIFEST: $script gen= must be manual (got '$gen')")
    [[ -n "$couples" ]] || { errors+=("MANIFEST: $script couples= is empty"); continue; }

    IFS=',' read -ra surf <<<"$couples"
    for s in "${surf[@]}"; do
        in_vocab "$s" || errors+=("MANIFEST: $script couples surface '$s' not in the declared GRAPH_VOCAB")
    done

    # assertion B: couples⊆trigger parity
    trig_set="${trigger:-$couples}"
    IFS=',' read -ra trigsurf <<<"$trig_set"
    for s in "${trigsurf[@]}"; do
        [[ "$s" == '*' ]] && continue
        in_vocab "$s" || errors+=("MANIFEST: $script trigger surface '$s' not in the declared GRAPH_VOCAB")
    done
    for s in "${surf[@]}"; do
        covered_by "$s" "${trigsurf[@]}" || \
            errors+=("PARITY: $c couples '$s' but its trigger ($trig_set) would not fire on it")
    done

    # assertion C: dir=bi cycle valve rule (PROPOSED vs none)
    if [[ "$dir" == bi ]]; then
        has_leading=0; has_lagging=0
        for s in "${surf[@]}"; do
            [[ ${#GRAPH_LEADING[@]} -gt 0 ]] && in_set "$s" "${GRAPH_LEADING[@]}" && has_leading=1
            [[ ${#GRAPH_LAGGING[@]} -gt 0 ]] && in_set "$s" "${GRAPH_LAGGING[@]}" && has_lagging=1
        done
        if [[ "$has_leading" -eq 1 && "$has_lagging" -eq 1 ]]; then
            [[ "$valve" == PROPOSED ]] || errors+=("CYCLE-VALVE: $c is a design<->code bi cycle (couples a leading AND a lagging surface) but valve=$valve; it must be valve=PROPOSED so the leading surface can run ahead via a queue-tracked marker")
        elif [[ "$has_leading" -eq 1 ]]; then
            [[ "$valve" == none || "$valve" == PROPOSED ]] || errors+=("CYCLE-VALVE: $c couples a leading design surface (dir=bi) with no lagging surface, so valve= must be none|PROPOSED (got '$valve')")
        else
            [[ "$valve" == none ]] || errors+=("CYCLE-VALVE: $c is a dir=bi bijection with no leading design surface but valve=$valve; it must agree now, so valve=none")
        fi
    fi
done

# assertion D: hook artifact freshness (pre-commit == --emit)
hook_emitted="$(bash "$GEN" --emit 2>/dev/null)"; gen_st=$?
if [[ ! -f "$HOOK" ]]; then
    errors+=("ARTIFACT: $HOOK does not exist; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write")
elif [[ "$gen_st" -ne 0 ]]; then
    errors+=("ARTIFACT: gen-pre-commit.sh --emit failed (exit $gen_st); fix the generator before trusting the hook")
elif [[ "$hook_emitted" != "$(cat "$HOOK")" ]]; then
    errors+=("ARTIFACT: $HOOK is stale vs the '# graph:' manifests; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write")
fi

# assertion D (commit-msg surface): when any gate is tier=commit-msg, the
# committed commit-msg hook equals gen-pre-commit.sh --emit-commit-msg
MSG_HOOK="${GATE_SDK_HOOKS_DIR:-$GATES_DIR/git-hooks}/commit-msg"
if [[ "$has_msg_gate" -eq 1 ]]; then
    msg_emitted="$(bash "$GEN" --emit-commit-msg 2>/dev/null)"; msg_st=$?
    if [[ ! -f "$MSG_HOOK" ]]; then
        errors+=("ARTIFACT: $MSG_HOOK does not exist but a tier=commit-msg gate is registered; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write")
    elif [[ "$msg_st" -ne 0 ]]; then
        errors+=("ARTIFACT: gen-pre-commit.sh --emit-commit-msg failed (exit $msg_st); fix the generator before trusting the hook")
    elif [[ "$msg_emitted" != "$(cat "$MSG_HOOK")" ]]; then
        errors+=("ARTIFACT: $MSG_HOOK is stale vs the '# graph:' manifests; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write")
    fi
fi

# assertion E: the coupling-graph artifact matches --emit; its path is the
# GATE_SDK_GRAPH_ARTIFACT knob (workflow-dir default), so a consumer that
# republishes the artifact elsewhere gets its own path in every remedy line
ARTIFACT="${GATE_SDK_GRAPH_ARTIFACT:-$WORKFLOW_DIR/CHECK-GRAPH.html}"
ARTIFACT_DIR="$(dirname "$ARTIFACT")"
emitted="$(emit_graph)"
if [[ ! -f "$ARTIFACT" ]]; then
    errors+=("ARTIFACT: $ARTIFACT does not exist; regenerate: bash gate-sdk/checks/check-graph.sh --emit > $ARTIFACT")
elif [[ "$emitted" != "$(cat "$ARTIFACT")" ]]; then
    errors+=("ARTIFACT: $ARTIFACT is stale vs the '# graph:' manifests; regenerate: bash gate-sdk/checks/check-graph.sh --emit > $ARTIFACT")
fi

# assertion F: every emitted asset href resolves under the artifact dir
while IFS= read -r href; do
    [[ -n "$href" ]] || continue
    [[ -f "$ARTIFACT_DIR/$href" ]] || \
        errors+=("ASSET-HREF: emitted asset '$href' does not resolve to a file under $ARTIFACT_DIR/ (artifact-relative); fix the href in emit_graph")
done < <(grep -oE '(href|src)="[^"]+"' <<<"$emitted" | sed -E 's/^(href|src)="//; s/"$//' | grep -v '://')

# assertion G: every `# graph:` manifest in a SPEC-*.md amendment body is well-formed
g_errors=()
amendment_findings "."
[[ ${#g_errors[@]} -gt 0 ]] && errors+=("${g_errors[@]}")

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "CHECK-GRAPH: ${#errors[@]} violation(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: fix the '# graph:' manifest / gates.list-membership / hook-trigger mismatch (or the malformed amendment-body manifest), then regenerate the hook and graph artifacts"
    exit 1
fi
echo "CHECK-GRAPH: clean (${#CHECKS[@]} gates; manifests well-formed, couples<->trigger parity, cycle valves, the generated pre-commit hook + CHECK-GRAPH.html artifacts fresh, emitted asset hrefs resolve, and amendment-body manifests valid)"
exit 0
