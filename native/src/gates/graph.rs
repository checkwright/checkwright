// spec: gate-sdk/SPEC.md §check-graph — the `# graph:` manifest on every gates.list member is
// well-formed and consistent, and the generated hooks and the coupling-graph artifact are the
// faithful projections of those manifests.
use crate::emit::graph as proj;
use crate::proc;
use crate::registry;
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-graph: {}", e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-graph — the retired sourced-function theme seam fails loudly. A
// consumer that quietly lost its theme emits an artifact the byte-compare cannot tell from a
// legitimate theme edit, so the failure would be invisible in a green battery.
fn retired_theme_seam(gates_dir: &str) -> Option<String> {
    if let Ok(v) = std::env::var("GATE_SDK_GRAPH_THEME") {
        if !v.is_empty() {
            return Some(format!(
                "GATE_SDK_GRAPH_THEME is set ({}), but the sourced-function theme seam is retired",
                v
            ));
        }
    }
    let legacy = format!("{}/graph-theme.sh", gates_dir.trim_end_matches('/'));
    if Path::new(&legacy).is_file() {
        return Some(format!(
            "{} exists, but the sourced-function theme seam is retired",
            legacy
        ));
    }
    None
}

const MIGRATION: &str = "  help: the theme is now a directory of verbatim part files at \
GATE_SDK_GRAPH_THEME_DIR (default <gates-dir>/graph-theme/): move graph_theme_css's body to \
theme.css, graph_theme_header's to header.html and graph_theme_footer's to footer.html, then \
delete graph-theme.sh and unset GATE_SDK_GRAPH_THEME (gate-sdk/SPEC.md §check-graph)";

// spec: gate-sdk/SPEC.md §check-graph — the external-ref allowlist: the pinned-major mermaid ESM
// import the emitter itself emits (kit-seeded, always allowed) plus the consumer-sanctioned
// GATE_SDK_GRAPH_EXTERNAL_REFS prefixes (graph-vocab seam)
const MERMAID_SEED: &str = "https://cdn.jsdelivr.net/npm/mermaid@11";

fn ext_ref_allowed(reference: &str, allowed: &[String]) -> bool {
    if reference.starts_with(MERMAID_SEED) {
        return true;
    }
    allowed
        .iter()
        .any(|p| !p.is_empty() && reference.starts_with(p.as_str()))
}

// spec: gate-sdk/SPEC.md §check-graph — every `href`/`src` attribute value in the emitted HTML; F
// takes the artifact-relative ones and H the absolute (`://`-carrying) ones
fn attr_refs(emitted: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for attr in ["href=\"", "src=\""] {
        let mut rest = emitted;
        while let Some(at) = rest.find(attr) {
            rest = &rest[at + attr.len()..];
            match rest.find('"') {
                Some(end) => {
                    out.push(rest[..end].to_string());
                    rest = &rest[end + 1..];
                }
                None => break,
            }
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §check-graph — the ESM import specifiers, the second half of the
// external-ref corpus: a quoted specifier on an `import … from '…'` or a bare `import '…'`
fn import_refs(emitted: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in emitted.lines() {
        let t = line.trim_start();
        if !t.starts_with("import ") && !t.starts_with("import'") {
            continue;
        }
        let stmt = match t.find(';') {
            Some(at) => &t[..at],
            None => t,
        };
        let mut rest = stmt;
        while let Some(a) = rest.find('\'') {
            rest = &rest[a + 1..];
            match rest.find('\'') {
                Some(b) => {
                    out.push(rest[..b].to_string());
                    rest = &rest[b + 1..];
                }
                None => break,
            }
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §check-graph — each disallowed external reference in the emitted HTML: an
// absolute href/src value or ESM import specifier matching no allowed prefix, sorted and deduped
fn disallowed_external_refs(emitted: &str, allowed: &[String]) -> Vec<String> {
    let mut refs: Vec<String> = attr_refs(emitted)
        .into_iter()
        .filter(|r| r.contains("://"))
        .collect();
    refs.extend(import_refs(emitted));
    refs.sort();
    refs.dedup();
    refs.into_iter()
        .filter(|r| !r.is_empty() && !ext_ref_allowed(r, allowed))
        .collect()
}

// spec: gate-sdk/SPEC.md §check-graph — the emitted mermaid edge count (an arrow bearing a gate
// label)
fn edge_count(emitted: &str) -> usize {
    emitted
        .lines()
        .filter(|l| {
            l.split("<-->")
                .skip(1)
                .chain(l.split("-->").skip(1))
                .any(|r| r.starts_with('|') && r[1..].starts_with('"'))
        })
        .count()
}

// spec: gate-sdk/SPEC.md §check-graph — the render cap the emitted page declares, or Mermaid's
// built-in default when the init call names none
fn render_cap(emitted: &str) -> u64 {
    for line in emitted.lines() {
        if let Some(at) = line.find("maxEdges:") {
            let digits: String = line[at + "maxEdges:".len()..]
                .trim_start()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if let Ok(n) = digits.parse::<u64>() {
                return n;
            }
        }
    }
    500
}

// spec: gate-sdk/SPEC.md §check-graph (assertion G) — a couples/trigger token's glob grammar; the
// `kit:<glob>` couples form validates on its glob part
fn valid_glob_token(tok: &str) -> bool {
    let t = tok.strip_prefix("kit:").unwrap_or(tok);
    !t.is_empty()
        && t.chars().all(|c| {
            c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '*' | '?' | '/' | '-')
        })
}

// spec: gate-sdk/SPEC.md §check-graph (assertion G) — the `# graph:` manifests in an amendment
// body: a fence's own line except under `proto`, and every inline span outside a fence
fn extract_amend_manifests(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut infence = false;
    let mut flang = String::new();
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            if infence {
                infence = false;
                flang.clear();
            } else {
                infence = true;
                let t = line.trim_start().trim_start_matches('`');
                flang = t
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                    .collect();
            }
            continue;
        }
        if infence {
            if flang == "proto" {
                continue;
            }
            if let Some(span) = line.strip_prefix("# graph: ") {
                out.push(span.to_string());
            }
            continue;
        }
        let mut rest = line;
        while let Some(at) = rest.find("`# graph: ") {
            rest = &rest[at + "`# graph: ".len()..];
            match rest.find('`') {
                Some(end) => {
                    out.push(rest[..end].to_string());
                    rest = &rest[end + 1..];
                }
                None => break,
            }
        }
    }
    out
}

fn validate_amend_manifest(file: &str, span: &str, errors: &mut Vec<String>) {
    let keyed = span.split_whitespace().any(|t| {
        matches!(
            t.split_once('='),
            Some((k, _)) if matches!(k, "couples" | "dir" | "valve" | "tier" | "mode" | "trigger" | "gen")
        )
    });
    if !keyed {
        return;
    }
    let (mut couples, mut dir, mut valve, mut tier) =
        (String::new(), String::new(), String::new(), String::new());
    let (mut mode, mut trigger, mut gen) = (String::new(), String::new(), String::new());
    let (mut have_couples, mut have_dir, mut have_valve, mut have_tier) = (false, false, false, false);
    let mut unknown: Vec<String> = Vec::new();
    for tok in span.split_whitespace() {
        match tok.split_once('=') {
            Some(("couples", v)) => {
                couples = v.to_string();
                have_couples = true;
            }
            Some(("dir", v)) => {
                dir = v.to_string();
                have_dir = true;
            }
            Some(("valve", v)) => {
                valve = v.to_string();
                have_valve = true;
            }
            Some(("tier", v)) => {
                tier = v.to_string();
                have_tier = true;
            }
            Some(("mode", v)) => mode = v.to_string(),
            Some(("trigger", v)) => trigger = v.to_string(),
            Some(("gen", v)) => gen = v.to_string(),
            _ => unknown.push(tok.to_string()),
        }
    }
    let where_ = format!("AMEND-MANIFEST: {}", file);
    if !have_couples {
        errors.push(format!("{}: missing required key 'couples='", where_));
    }
    if !have_dir {
        errors.push(format!("{}: missing required key 'dir='", where_));
    }
    if !have_valve {
        errors.push(format!("{}: missing required key 'valve='", where_));
    }
    if !have_tier {
        errors.push(format!("{}: missing required key 'tier='", where_));
    }
    for u in &unknown {
        errors.push(format!("{}: unknown manifest key/token '{}'", where_, u));
    }
    if have_dir && dir != "bi" && dir != "one" {
        errors.push(format!("{}: dir= must be bi|one (got '{}')", where_, dir));
    }
    if have_valve && valve != "none" && valve != "PROPOSED" {
        errors.push(format!(
            "{}: valve= must be none|PROPOSED (got '{}')",
            where_, valve
        ));
    }
    if have_tier && tier != "precommit" && tier != "align-only" && tier != "commit-msg" {
        errors.push(format!(
            "{}: tier= must be precommit|align-only|commit-msg (got '{}')",
            where_, tier
        ));
    }
    if !mode.is_empty() && mode != "staged" && mode != "whole-tree" {
        errors.push(format!(
            "{}: mode= must be staged|whole-tree (got '{}')",
            where_, mode
        ));
    }
    if !gen.is_empty() && gen != "manual" {
        errors.push(format!("{}: gen= must be manual (got '{}')", where_, gen));
    }
    if have_couples && couples.is_empty() {
        errors.push(format!("{}: couples= is empty", where_));
    }
    if !couples.is_empty() {
        for s in couples.split(',') {
            if !valid_glob_token(s) {
                errors.push(format!(
                    "{}: couples token '{}' is not a syntactically valid glob/path",
                    where_, s
                ));
            }
        }
    }
    if !trigger.is_empty() {
        for s in trigger.split(',') {
            if s == "*" {
                continue;
            }
            if !valid_glob_token(s) {
                errors.push(format!(
                    "{}: trigger token '{}' is not a syntactically valid glob/path",
                    where_, s
                ));
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §check-graph (assertion G) — the amendment corpus: every SPEC-*.md under
// the scan root, through the shared pruned walk rather than a second traversal of this crate's own
fn amendment_findings(root: &str, errors: &mut Vec<String>) -> Result<(), String> {
    let mut files: Vec<String> = walk::find_files(Path::new(root), &["md"])?
        .into_iter()
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("SPEC-"))
                .unwrap_or(false)
        })
        .map(|p| p.display().to_string())
        .collect();
    files.sort();
    for f in files {
        let text = std::fs::read_to_string(&f).map_err(|e| format!("cannot read {}: {}", f, e))?;
        for span in extract_amend_manifests(&text) {
            validate_amend_manifest(&f, &span, errors);
        }
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §check-graph — assertion B's four coverage branches, reproduced verbatim.
// Neither crate matcher is this predicate and substituting either flips verdicts on the live
// registry; criterion 6's globstar commitment does not reach a predicate that matches no glob.
fn covered_by(s: &str, triggers: &[&str]) -> bool {
    for t in triggers {
        if *t == "*" || *t == s {
            return true;
        }
        if !s.contains('*') && !s.contains('?') && walk::pattern_match(t, s) {
            return true;
        }
        if let Some(ext) = t.strip_prefix("*.") {
            if s.ends_with(&format!(".{}", ext)) {
                return true;
            }
        }
    }
    false
}

fn in_set(t: &str, set: &[String]) -> bool {
    set.iter().any(|v| v == t)
}

fn in_vocab(t: &str, vocab: &[String]) -> bool {
    vocab.is_empty() || in_set(t, vocab)
}

fn read_stripped(p: &str) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| format!("cannot read {}: {}", p, e))
}

// spec: gate-sdk/SPEC.md §check-graph — the generator stays shell (§gen-pre-commit), so assertion D
// spawns it; criterion 7 clears the spawn because `bash` is on the program floor
fn generator_emit(gen: &str, arm: &str) -> Result<Option<String>, String> {
    let out = proc::run("bash", &[gen, arm])?;
    Ok(out
        .stdout()
        .map(|b| String::from_utf8_lossy(b).trim_end_matches('\n').to_string()))
}

fn rule(args: &[String]) -> Result<i32, String> {
    let cfg = proj::Config::from_bridge()?;

    if let Some(what) = retired_theme_seam(&cfg.gates_dir) {
        println!("CHECK-GRAPH: 1 retired-seam violation(s):");
        println!("  RETIRED-SEAM: {}", what);
        println!("{}", MIGRATION);
        return Ok(2);
    }

    let mode = args.first().map(String::as_str).unwrap_or("");

    if mode == "--amend-only" {
        let root = args.get(1).map(String::as_str).unwrap_or(".");
        let mut errors: Vec<String> = Vec::new();
        amendment_findings(root, &mut errors)?;
        if !errors.is_empty() {
            println!(
                "CHECK-GRAPH: {} amendment-manifest violation(s):",
                errors.len()
            );
            for e in &errors {
                println!("  {}", e);
            }
            println!("  help: fix the malformed '# graph:' manifest in the SPEC-*.md amendment body (required keys couples/dir/valve/tier; dir=bi|one valve=none|PROPOSED tier=precommit|align-only; couples tokens must be syntactically valid globs)");
            return Ok(1);
        }
        println!("CHECK-GRAPH: clean (amendment-body '# graph:' manifests well-formed)");
        return Ok(0);
    }

    if mode == "--refs-only" {
        let allowed = walk::knob_array("GATE_GRAPH_EXTERNAL_REFS")?;
        let members = proj::projected_members(&cfg)?;
        let emitted = proj::render(&cfg, &members);
        let bad = disallowed_external_refs(&emitted, &allowed);
        if !bad.is_empty() {
            println!("CHECK-GRAPH: {} external-ref violation(s):", bad.len());
            for r in &bad {
                println!("  EXTERNAL-REF: emitted artifact references '{}', neither the seeded mermaid import nor a GATE_SDK_GRAPH_EXTERNAL_REFS prefix; add its prefix to the knob or drop the reference", r);
            }
            println!("  help: an emitted external reference must prefix-match the seeded mermaid import or a GATE_SDK_GRAPH_EXTERNAL_REFS prefix");
            return Ok(1);
        }
        println!("CHECK-GRAPH: clean (emitted external refs allowlisted)");
        return Ok(0);
    }

    if mode == "--cap-only" {
        let members = proj::projected_members(&cfg)?;
        let emitted = proj::render(&cfg, &members);
        let (n, cap) = (edge_count(&emitted), render_cap(&emitted));
        if n as u64 > cap {
            println!("CHECK-GRAPH: 1 render-cap violation(s):");
            println!("  RENDER-CAP: emitted graph has {} edges but the page declares maxEdges={}; Mermaid refuses a flowchart with more edges than the cap and paints an error graphic — raise GATE_SDK_GRAPH_MAX_EDGES above {}", n, cap, n);
            return Ok(1);
        }
        println!(
            "CHECK-GRAPH: clean (emitted graph within the render cap: {} edges <= maxEdges={})",
            n, cap
        );
        return Ok(0);
    }

    let list = cfg.list_path();
    if !Path::new(&list).is_file() {
        return Err(format!("no registry at {}", list));
    }
    let checks = registry::members(&read_stripped(&list)?);
    if checks.is_empty() {
        return Err(format!("no members parsed from {}", list));
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the kit root the generator lives under, bridged because
    // a compiled member has no BASH_SOURCE to find its own kit by
    let sdk_root = walk::knob_scalar("GATE_SDK_ROOT_HERE")?;
    let gen = format!("{}/bin/gen-pre-commit.sh", sdk_root.trim_end_matches('/'));
    if !Path::new(&gen).is_file() {
        return Err(format!("gen-pre-commit.sh not found at {}", gen));
    }

    let vocab = walk::knob_array("GRAPH_VOCAB")?;
    let leading = walk::knob_array("GRAPH_LEADING")?;
    let lagging = walk::knob_array("GRAPH_LAGGING")?;
    let allowed = walk::knob_array("GATE_GRAPH_EXTERNAL_REFS")?;

    let mut errors: Vec<String> = Vec::new();
    let mut has_msg_gate = false;

    for c in &checks {
        let script = match registry::resolve(c, &cfg.resolve_dirs) {
            Some(s) => s,
            None => {
                errors.push(format!(
                    "MANIFEST: {} is in gates.list but resolves in none of: {}",
                    c,
                    cfg.resolve_dirs.join(" ")
                ));
                continue;
            }
        };
        let body = read_stripped(&script)?;
        let man = match registry::manifest_line(&body) {
            Some(m) => m.to_string(),
            None => {
                errors.push(format!(
                    "MANIFEST: {} has no '# graph:' manifest line",
                    script
                ));
                continue;
            }
        };
        let (mut couples, mut dir, mut valve, mut tier) =
            (String::new(), String::new(), String::new(), String::new());
        let (mut mode_f, mut trigger, mut gen_f) = (String::new(), String::new(), String::new());
        for kv in man.trim_start_matches("# graph: ").split_whitespace() {
            match kv.split_once('=') {
                Some(("couples", v)) => couples = v.to_string(),
                Some(("dir", v)) => dir = v.to_string(),
                Some(("valve", v)) => valve = v.to_string(),
                Some(("tier", v)) => tier = v.to_string(),
                Some(("mode", v)) => mode_f = v.to_string(),
                Some(("trigger", v)) => trigger = v.to_string(),
                Some(("gen", v)) => gen_f = v.to_string(),
                _ => errors.push(format!(
                    "MANIFEST: {} unknown manifest key '{}'",
                    script, kv
                )),
            }
        }
        couples = registry::expand_couples(&couples, &cfg.kit_roots_rel);
        if !trigger.is_empty() {
            trigger = registry::expand_couples(&trigger, &cfg.kit_roots_rel);
        }
        if dir != "bi" && dir != "one" {
            errors.push(format!(
                "MANIFEST: {} dir= must be bi|one (got '{}')",
                script, dir
            ));
        }
        if valve != "none" && valve != "PROPOSED" {
            errors.push(format!(
                "MANIFEST: {} valve= must be none|PROPOSED (got '{}')",
                script, valve
            ));
        }
        if tier != "precommit" && tier != "align-only" && tier != "commit-msg" {
            errors.push(format!(
                "MANIFEST: {} tier= must be precommit|align-only|commit-msg (got '{}')",
                script, tier
            ));
        }
        if tier == "commit-msg" {
            has_msg_gate = true;
        }
        if !mode_f.is_empty() && mode_f != "staged" && mode_f != "whole-tree" {
            errors.push(format!(
                "MANIFEST: {} mode= must be staged|whole-tree (got '{}')",
                script, mode_f
            ));
        }
        if !gen_f.is_empty() && gen_f != "manual" {
            errors.push(format!(
                "MANIFEST: {} gen= must be manual (got '{}')",
                script, gen_f
            ));
        }
        if couples.is_empty() {
            errors.push(format!("MANIFEST: {} couples= is empty", script));
            continue;
        }

        let surf: Vec<&str> = couples.split(',').collect();
        for s in &surf {
            if !in_vocab(s, &vocab) {
                errors.push(format!(
                    "MANIFEST: {} couples surface '{}' not in the declared GRAPH_VOCAB",
                    script, s
                ));
            }
        }

        // assertion B: couples⊆trigger parity
        let trig_set = if trigger.is_empty() {
            couples.clone()
        } else {
            trigger.clone()
        };
        let trigsurf: Vec<&str> = trig_set.split(',').collect();
        for s in &trigsurf {
            if *s == "*" {
                continue;
            }
            if !in_vocab(s, &vocab) {
                errors.push(format!(
                    "MANIFEST: {} trigger surface '{}' not in the declared GRAPH_VOCAB",
                    script, s
                ));
            }
        }
        for s in &surf {
            if !covered_by(s, &trigsurf) {
                errors.push(format!(
                    "PARITY: {} couples '{}' but its trigger ({}) would not fire on it",
                    c, s, trig_set
                ));
            }
        }

        // assertion C: dir=bi cycle valve rule (PROPOSED vs none)
        if dir == "bi" {
            let has_leading = surf.iter().any(|s| in_set(s, &leading));
            let has_lagging = surf.iter().any(|s| in_set(s, &lagging));
            if has_leading && has_lagging {
                if valve != "PROPOSED" {
                    errors.push(format!("CYCLE-VALVE: {} is a design<->code bi cycle (couples a leading AND a lagging surface) but valve={}; it must be valve=PROPOSED so the leading surface can run ahead via a queue-tracked marker", c, valve));
                }
            } else if has_leading {
                if valve != "none" && valve != "PROPOSED" {
                    errors.push(format!("CYCLE-VALVE: {} couples a leading design surface (dir=bi) with no lagging surface, so valve= must be none|PROPOSED (got '{}')", c, valve));
                }
            } else if valve != "none" {
                errors.push(format!("CYCLE-VALVE: {} is a dir=bi bijection with no leading design surface but valve={}; it must agree now, so valve=none", c, valve));
            }
        }
    }

    // assertion D: hook artifact freshness (pre-commit == --emit)
    let hooks_dir = walk::knob_scalar("GATE_SDK_HOOKS_DIR")?;
    let hook = format!("{}/pre-commit", hooks_dir.trim_end_matches('/'));
    let hook_emitted = generator_emit(&gen, "--emit")?;
    if !Path::new(&hook).is_file() {
        errors.push(format!("ARTIFACT: {} does not exist; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write", hook));
    } else {
        match hook_emitted {
            None => errors.push("ARTIFACT: gen-pre-commit.sh --emit failed; fix the generator before trusting the hook".to_string()),
            Some(ref e) => {
                if e.as_str() != read_stripped(&hook)?.trim_end_matches('\n') {
                    errors.push(format!("ARTIFACT: {} is stale vs the '# graph:' manifests; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write", hook));
                }
            }
        }
    }

    // assertion D (commit-msg surface)
    let msg_hook = format!("{}/commit-msg", hooks_dir.trim_end_matches('/'));
    if has_msg_gate {
        let msg_emitted = generator_emit(&gen, "--emit-commit-msg")?;
        if !Path::new(&msg_hook).is_file() {
            errors.push(format!("ARTIFACT: {} does not exist but a tier=commit-msg gate is registered; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write", msg_hook));
        } else {
            match msg_emitted {
                None => errors.push("ARTIFACT: gen-pre-commit.sh --emit-commit-msg failed; fix the generator before trusting the hook".to_string()),
                Some(ref e) => {
                    if e.as_str() != read_stripped(&msg_hook)?.trim_end_matches('\n') {
                        errors.push(format!("ARTIFACT: {} is stale vs the '# graph:' manifests; regenerate: bash gate-sdk/bin/gen-pre-commit.sh --write", msg_hook));
                    }
                }
            }
        }
    }

    // assertion E: the coupling-graph artifact matches the emitter
    let members = proj::projected_members(&cfg)?;
    let emitted = proj::render(&cfg, &members);
    let artifact = cfg.artifact.clone();
    let artifact_dir = match artifact.rfind('/') {
        Some(at) => artifact[..at].to_string(),
        None => ".".to_string(),
    };
    if !Path::new(&artifact).is_file() {
        errors.push(format!(
            "ARTIFACT: {} does not exist; regenerate: bash gate-sdk/bin/run-gates.sh --emit graph > {}",
            artifact, artifact
        ));
    } else if emitted.trim_end_matches('\n') != read_stripped(&artifact)?.trim_end_matches('\n') {
        errors.push(format!(
            "ARTIFACT: {} is stale vs the '# graph:' manifests; regenerate: bash gate-sdk/bin/run-gates.sh --emit graph > {}",
            artifact, artifact
        ));
    }

    // assertion F: every emitted asset href resolves under the artifact dir
    for href in attr_refs(&emitted) {
        if href.is_empty() || href.contains("://") {
            continue;
        }
        if !Path::new(&format!("{}/{}", artifact_dir, href)).is_file() {
            errors.push(format!("ASSET-HREF: emitted asset '{}' does not resolve to a file under {}/ (artifact-relative); fix the href in the theme part or the emitter", href, artifact_dir));
        }
    }

    // assertion H: every emitted external reference is allowlisted
    for r in disallowed_external_refs(&emitted, &allowed) {
        errors.push(format!("EXTERNAL-REF: emitted artifact references '{}', neither the seeded mermaid import nor a GATE_SDK_GRAPH_EXTERNAL_REFS prefix; add its prefix to the knob or drop the reference", r));
    }

    // assertion I: the emitted edge count fits the render cap the page declares
    let (n, cap) = (edge_count(&emitted), render_cap(&emitted));
    if n as u64 > cap {
        errors.push(format!("RENDER-CAP: emitted graph has {} edges but the page declares maxEdges={}; Mermaid would paint an error graphic instead of the diagram — raise GATE_SDK_GRAPH_MAX_EDGES above {}", n, cap, n));
    }

    // assertion G: every `# graph:` manifest in a SPEC-*.md amendment body is well-formed
    amendment_findings(".", &mut errors)?;

    if !errors.is_empty() {
        println!("CHECK-GRAPH: {} violation(s):", errors.len());
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: fix the '# graph:' manifest / gates.list-membership / hook-trigger mismatch (or the malformed amendment-body manifest), then regenerate the hook and graph artifacts");
        return Ok(1);
    }
    println!("CHECK-GRAPH: clean ({} gates; manifests well-formed, couples<->trigger parity, cycle valves, the generated pre-commit hook + CHECK-GRAPH.html artifacts fresh, emitted asset hrefs resolve, external refs allowlisted, edge count within the render cap, and amendment-body manifests valid)", checks.len());
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-graph — each coverage branch on its own, with a near miss
    // beside it, so a branch that stops firing reds rather than passing vacuously
    #[test]
    fn each_coverage_branch_fires_on_its_own_case() {
        assert!(covered_by("scripts/gates.list", &["*"]));
        assert!(covered_by("scripts/gates.list", &["scripts/gates.list"]));
        assert!(covered_by("docs/site.md", &["docs/*.md"]));
        assert!(covered_by("scripts/*.sh", &["*.sh"]));
        assert!(!covered_by("scripts/*.sh", &["scripts/*"]));
        assert!(!covered_by("docs/site.md", &["scripts/*.sh"]));
        assert!(!covered_by("docs/site.md", &["*.sh"]));
    }

    // spec: gate-sdk/SPEC.md §check-graph — the cap falls back to Mermaid's own default
    #[test]
    fn an_emission_naming_no_cap_falls_back_to_mermaids_default() {
        assert_eq!(render_cap("mermaid.initialize({ startOnLoad: false });"), 500);
        assert_eq!(render_cap("  maxEdges: 4200 });"), 4200);
    }

    // spec: gate-sdk/SPEC.md §check-graph — only an arrow bearing a gate label counts as an edge
    #[test]
    fn the_edge_counter_takes_only_labelled_arrows() {
        let g = "graph LR\n  subgraph a[\"a\"]\n    n0[\"x\"]\n  end\n  n0 -->|\"check-a\"| n1\n  n0 <-->|\"check-b\"| n1\n";
        assert_eq!(edge_count(g), 2);
    }

    // spec: gate-sdk/SPEC.md §check-graph — the seeded import is allowed under an empty knob
    #[test]
    fn the_seeded_import_is_always_allowed_and_the_knob_adds() {
        let page = "<script type=\"module\">\n    import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';\n</script>\n<a href=\"https://third-party.example/w.js\">x</a>\n";
        assert_eq!(
            disallowed_external_refs(page, &[]),
            vec!["https://third-party.example/w.js".to_string()]
        );
        assert!(disallowed_external_refs(page, &["https://third-party.example".to_string()]).is_empty());
    }

    // spec: gate-sdk/SPEC.md §check-graph (assertion G) — a `proto` fence is excluded and an
    // inline span outside a fence is taken
    #[test]
    fn the_amendment_extractor_skips_proto_fences_and_takes_inline_spans() {
        let text = "```proto\n# graph: couples=x dir=one valve=none tier=precommit\n```\n\nsee `# graph: couples=y dir=one valve=none tier=precommit` above\n\n```bash\n# graph: couples=z dir=one valve=none tier=precommit\n```\n";
        let got = extract_amend_manifests(text);
        assert_eq!(got.len(), 2);
        assert!(got[0].starts_with("couples=y"));
        assert!(got[1].starts_with("couples=z"));
    }
}
