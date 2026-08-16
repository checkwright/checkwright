// spec: gate-sdk/SPEC.md §enforcement-map — the kit→surface→class map emitted from the class
// registries. Two levels: `measure` produces the sections and their per-row kit attribution,
// `emit` renders them, so the value-rollup join consumes data rather than re-parsing the page.
use crate::fresh;
use crate::proc;
use crate::walk;
use serde_json::Value;
use std::path::Path;

const GATES_DIR_KNOB: &str = "GATE_SDK_GATES_DIR";
const KPIS_KNOB: &str = "DRIFT_KIT_KPIS_FILE";
const SETTINGS_KNOB: &str = "CONTEXT_KIT_SETTINGS_FILE";
const SCAN_KNOB: &str = "GATE_SDK_ENFORCE_SCAN_DIR";
const BLOB_REF_KNOB: &str = "CANON_KIT_DOCS_BLOB_REF";
const SUITES_KNOB: &str = "EVIDENCE_KIT_SUITES";
const RUN_PREFIX: &str = "EVIDENCE_KIT_RUN_";
const MONITOR_MARKER: &str = "enforce:";

pub struct Row {
    pub kit: String,
    pub cells: Vec<String>,
}

pub struct Section {
    pub title: String,
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
}

pub struct EnforcementMap {
    pub sections: Vec<Section>,
    classes: String,
}

// spec: gate-sdk/SPEC.md §enforcement-map — attribute a path to its owning kit label by the first
// kit segment; a surface under no kit groups as (consumer)
fn attribute_kit(path: &str) -> String {
    for seg in path.trim_start_matches("./").split('/') {
        if seg == "gate-sdk" || seg.ends_with("-kit") {
            return seg.to_string();
        }
    }
    "(consumer)".to_string()
}

// spec: gate-sdk/SPEC.md §enforcement-map — the first slash-bearing token of a hook/suite command
// is the enforcing script, whose dir attributes the row
fn command_path(cmd: &str) -> String {
    for tok in cmd.split_whitespace() {
        if tok.contains('/') {
            return tok.to_string();
        }
    }
    cmd.to_string()
}

// spec: gate-sdk/SPEC.md §enforcement-map — the kit column links each kit's docs page relative to
// the emitted page; the (consumer) group owns no kit page and stays plain text
fn kit_cell(kit: &str) -> String {
    if kit == "(consumer)" {
        return kit.to_string();
    }
    format!("[{}]({}/index.md)", kit, kit)
}

fn tracked(path: &str) -> bool {
    proc::run("git", &["ls-files", "--error-unmatch", "--", path])
        .map(|c| c.stdout().is_some())
        .unwrap_or(false)
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — gate_self_repo_prefix: the origin remote rewritten to a
// blob prefix, degrading to nothing when there is no origin or it is not an http(s)/scp form
fn self_repo_prefix(reference: &str) -> String {
    let origin = match proc::run("git", &["remote", "get-url", "origin"]) {
        Ok(c) => match c.stdout() {
            Some(o) => String::from_utf8_lossy(o).trim().to_string(),
            None => return String::new(),
        },
        Err(_) => return String::new(),
    };
    if origin.is_empty() {
        return String::new();
    }
    let id = origin
        .strip_suffix(".git")
        .unwrap_or(&origin)
        .trim_end_matches('/')
        .to_string();
    let id = if let Some(rest) = id.strip_prefix("git@") {
        match rest.split_once(':') {
            Some((host, path)) => format!("https://{}/{}", host, path),
            None => return String::new(),
        }
    } else if id.starts_with("https://") || id.starts_with("http://") {
        id
    } else {
        return String::new();
    };
    format!("{}/blob/{}/", id, reference)
}

// spec: gate-sdk/SPEC.md §enforcement-map — cite a class's owner section through the
// reference-link grammar, degrading to the bare `<path> §<title>` when the identity is unknown or
// the SPEC is untracked
fn owner_ref(prefix: &str, path: &str, anchor: &str, title: &str) -> String {
    if !prefix.is_empty() && tracked(path) {
        return format!("[`{}` §{}]({}{}#{})", path, title, prefix, path, anchor);
    }
    format!("`{}` §{}", path, title)
}

fn members(text: &str) -> Vec<String> {
    fresh::file_lines(text)
        .iter()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect()
}

fn read_file(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — gate_resolve's declaration path: dirs consumer-first, and
// `.sh` beats `.gate` within a dir
fn resolve(name: &str, dirs: &[String]) -> Option<String> {
    for d in dirs {
        for ext in ["sh", "gate"] {
            let p = format!("{}/{}.{}", d, name, ext);
            if Path::new(&p).is_file() {
                return Some(p);
            }
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §enforcement-map — the tier is read from the declaration's `# graph:`
// line; the shell form's sed requires whitespace before `tier=` and takes `[a-z-]*` after it
fn tier_of(src: &str) -> String {
    let text = match read_file(src) {
        Ok(t) => t,
        Err(_) => return "?".to_string(),
    };
    for line in fresh::file_lines(&text) {
        if !line.starts_with("# graph:") {
            continue;
        }
        if let Some(at) = line.rfind(" tier=").or_else(|| line.rfind('\t')) {
            let rest = &line[at..];
            if let Some(v) = rest.strip_prefix(" tier=") {
                let val: String = v
                    .chars()
                    .take_while(|c| c.is_ascii_lowercase() || *c == '-')
                    .collect();
                if !val.is_empty() {
                    return val;
                }
            }
        }
    }
    "?".to_string()
}

fn check_dirs(gates_dir: &str) -> Result<Vec<String>, String> {
    let mut dirs = vec![gates_dir.to_string()];
    for k in walk::kit_roots()? {
        dirs.push(format!("{}/checks", k.trim_end_matches('/')));
    }
    Ok(dirs)
}

// spec: gate-sdk/SPEC.md §enforcement-map — Blocking gates: gates.list membership, tier from each
// gate's `# graph:` line, grouped by tier and in gates.list order within a tier
fn gate_section(gates_dir: &str) -> Result<Option<Section>, String> {
    let list = format!("{}/gates.list", gates_dir);
    if !Path::new(&list).is_file() {
        return Ok(None);
    }
    let dirs = check_dirs(gates_dir)?;
    let (mut pre, mut msg, mut align): (Vec<Row>, Vec<Row>, Vec<Row>) =
        (Vec::new(), Vec::new(), Vec::new());
    for name in members(&read_file(&list)?) {
        let src = match resolve(&name, &dirs) {
            Some(s) => s,
            None => continue,
        };
        let tier = tier_of(&src);
        let row = Row {
            kit: attribute_kit(&src),
            cells: vec![name, tier.clone()],
        };
        match tier.as_str() {
            "precommit" => pre.push(row),
            "commit-msg" => msg.push(row),
            _ => align.push(row),
        }
    }
    if pre.is_empty() && msg.is_empty() && align.is_empty() {
        return Ok(None);
    }
    let mut rows = pre;
    rows.extend(msg);
    rows.extend(align);
    Ok(Some(Section {
        title: "Blocking gates".into(),
        columns: vec!["gate".into(), "tier".into()],
        rows,
    }))
}

// spec: gate-sdk/SPEC.md §enforcement-map — Advisory KPIs: the drift-kit KPI registry, kit from
// where each plugin resolves (the gates dir, then each kit's kpis/)
fn kpi_section(gates_dir: &str) -> Result<Option<Section>, String> {
    let file = walk::knob_scalar(KPIS_KNOB)?;
    if file.is_empty() || !Path::new(&file).is_file() {
        return Ok(None);
    }
    let roots = walk::kit_roots()?;
    let mut rows: Vec<Row> = Vec::new();
    for name in members(&read_file(&file)?) {
        let direct = format!("{}/{}.sh", gates_dir, name);
        let mut src = if Path::new(&direct).is_file() {
            direct.clone()
        } else {
            String::new()
        };
        if src.is_empty() {
            for d in &roots {
                let p = format!("{}/kpis/{}.sh", d.trim_end_matches('/'), name);
                if Path::new(&p).is_file() {
                    src = p;
                    break;
                }
            }
        }
        if src.is_empty() {
            src = direct;
        }
        rows.push(Row {
            kit: attribute_kit(&src),
            cells: vec![name],
        });
    }
    if rows.is_empty() {
        return Ok(None);
    }
    Ok(Some(Section {
        title: "Advisory KPIs".into(),
        columns: vec!["KPI".into()],
        rows,
    }))
}

// spec: gate-sdk/SPEC.md §enforcement-map — Guards and Session warnings: PreToolUse / SessionStart
// command hooks in the tracked harness settings file. This is the `jq` dependency's retirement:
// the rows come from serde_json, so the battery's own path needs no non-floor program.
fn hook_sections(settings: &str) -> Result<Vec<Section>, String> {
    if settings.is_empty() || !Path::new(settings).is_file() {
        return Ok(Vec::new());
    }
    let doc: Value = serde_json::from_str(&read_file(settings)?).map_err(|e| {
        format!(
            "CONTEXT_KIT_SETTINGS_FILE unparseable: {}: {}",
            settings, e
        )
    })?;
    let mut out: Vec<Section> = Vec::new();

    let mut guards: Vec<Row> = Vec::new();
    if let Some(entries) = doc.pointer("/hooks/PreToolUse").and_then(Value::as_array) {
        for e in entries {
            let matcher = e
                .get("matcher")
                .and_then(Value::as_str)
                .unwrap_or("*")
                .replace('|', "\\|");
            for h in e.get("hooks").and_then(Value::as_array).unwrap_or(&vec![]) {
                if h.get("type").and_then(Value::as_str) != Some("command") {
                    continue;
                }
                let cmd = h.get("command").and_then(Value::as_str).unwrap_or("");
                let path = command_path(cmd);
                guards.push(Row {
                    kit: attribute_kit(&path),
                    cells: vec![path, matcher.clone()],
                });
            }
        }
    }
    if !guards.is_empty() {
        out.push(Section {
            title: "Guards".into(),
            columns: vec!["surface".into(), "intercepts".into()],
            rows: guards,
        });
    }

    let mut warnings: Vec<Row> = Vec::new();
    if let Some(entries) = doc.pointer("/hooks/SessionStart").and_then(Value::as_array) {
        for e in entries {
            for h in e.get("hooks").and_then(Value::as_array).unwrap_or(&vec![]) {
                if h.get("type").and_then(Value::as_str) != Some("command") {
                    continue;
                }
                let cmd = h.get("command").and_then(Value::as_str).unwrap_or("");
                let path = command_path(cmd);
                warnings.push(Row {
                    kit: attribute_kit(&path),
                    cells: vec![path],
                });
            }
        }
    }
    if !warnings.is_empty() {
        out.push(Section {
            title: "Session warnings".into(),
            columns: vec!["surface".into()],
            rows: warnings,
        });
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §enforcement-map — Validate suites: evidence-kit's suite registry. The
// roster is EVIDENCE_KIT_SUITES and each run command is looked up *by name* in the bridged
// EVIDENCE_KIT_RUN_ family, because a prefix is a resolution set and never a roster.
fn suite_section() -> Result<Option<Section>, String> {
    let suites = walk::knob_array(SUITES_KNOB)?;
    if suites.is_empty() {
        return Ok(None);
    }
    let family = walk::knob_prefix(RUN_PREFIX);
    // spec: gate-sdk/SPEC.md §lib/gate.sh — the reader holds the roster, so the reader is what
    // fail-closes: a suite the roster named with no `RUN_` entry is adopted-but-broken and refuses
    // naming it. An *empty* roster never reaches here, which is the not-adopted case degrading.
    let mut rows: Vec<Row> = Vec::new();
    for s in &suites {
        let cmd = walk::knob_in_family(&family, s).ok_or_else(|| {
            format!(
                "EVIDENCE_KIT_SUITES names suite '{}' but no {}{} resolved — the suite registry \
                 is adopted and incomplete; treating as failure (not clean)",
                s, RUN_PREFIX, s
            )
        })?;
        rows.push(Row {
            kit: attribute_kit(&command_path(&cmd)),
            cells: vec![s.clone()],
        });
    }
    Ok(Some(Section {
        title: "Validate suites".into(),
        columns: vec!["suite".into()],
        rows,
    }))
}

// spec: gate-sdk/SPEC.md §enforcement-map — Monitors: the one class with no parseable registry, so
// a non-gate surface declares itself with a line-start `# enforce: class=monitor` marker. A marker
// under templates/ is an inert copy-source and is pruned, atop the gate-tests exclusion.
fn monitor_section(scan: &str) -> Result<Option<Section>, String> {
    // spec: gate-sdk/SPEC.md §enforcement-map — a scan root that is not a directory refuses
    // *naming the knob* rather than the path, and the default `.` is always a directory, so the
    // arm is reachable only for an explicitly-set value.
    if !fresh::is_dir(scan) {
        return Err(format!(
            "GATE_SDK_ENFORCE_SCAN_DIR not found: {} — the monitor registry could not be \
             scanned; treating as failure (not clean)",
            scan
        ));
    }
    let prune = walk::prune_dirs()?;
    let files = walk::find_with_prune(Path::new(scan), &|n| prune.iter().any(|d| d == n))?;
    let mut hits: Vec<(String, String)> = Vec::new();
    for f in files {
        let rel = f.display().to_string();
        if rel.contains("/templates/") {
            continue;
        }
        let bytes = match std::fs::read(&f) {
            Ok(b) => b,
            Err(_) => continue,
        };
        if bytes.contains(&0u8) {
            continue;
        }
        let text = String::from_utf8_lossy(&bytes).into_owned();
        for (i, line) in fresh::file_lines(&text).iter().enumerate() {
            if let Some(surface) = monitor_surface(line) {
                hits.push((format!("{}:{}", rel, i + 1), surface));
            }
        }
    }
    hits.sort();
    if hits.is_empty() {
        return Ok(None);
    }
    Ok(Some(Section {
        title: "Monitors".into(),
        columns: vec!["surface".into()],
        rows: hits
            .into_iter()
            .map(|(loc, surface)| Row {
                kit: attribute_kit(loc.split(':').next().unwrap_or("")),
                cells: vec![surface],
            })
            .collect(),
    }))
}

// spec: gate-sdk/SPEC.md §enforcement-map — the marker grammar: optional leading whitespace, `#`,
// `enforce:`, `class=monitor`, then whitespace and the free-text surface, trailing space trimmed
fn monitor_surface(line: &str) -> Option<String> {
    let t = line.trim_start();
    let t = t.strip_prefix('#')?;
    let t = t.trim_start();
    let t = t.strip_prefix(MONITOR_MARKER)?;
    if !t.starts_with(char::is_whitespace) {
        return None;
    }
    let t = t.trim_start();
    let t = t.strip_prefix("class=monitor")?;
    if !t.starts_with(char::is_whitespace) {
        return None;
    }
    Some(t.trim().to_string())
}

// spec: gate-sdk/SPEC.md §enforcement-map — the enforcement-class taxonomy, ordered hardest to
// softest; this page owns the prose and each class cites its mechanism owner
fn class_roster(prefix: &str) -> String {
    let r = |p: &str, a: &str, t: &str| owner_ref(prefix, p, a, t);
    format!(
        "- A **blocking gate** fails the commit (or, at the `align-only` tier, the\n  \
         consistency audit) — the pre-commit hook is its local reach, the CI workflow\n  \
         its server-side backstop. Owner: {}.\n\
         - An **advisory KPI** never blocks; it reports a drift trend into the\n  \
         session-context line. Owner: {}.\n\
         - A **guard** intercepts a tool call before it runs. Owner: {}.\n\
         - A **session warning** surfaces context when a session opens. Owner: {}.\n\
         - A **validate suite** holds a test baseline that a per-run evidence manifest\n  \
         attests. Owner: {}.\n\
         - A **monitor** watches deployment truth rather than tree truth, so it reds a\n  \
         scheduled run, never a merge. Owner: {}.\n",
        r("gate-sdk/SPEC.md", "enforcement-tiers", "Enforcement tiers"),
        r(
            "drift-kit/SPEC.md",
            "the-kpi-plugin-contract",
            "The KPI plugin contract"
        ),
        r(
            "guard-kit/SPEC.md",
            "the-guard-framework-libguardsh",
            "The guard framework"
        ),
        r(
            "context-kit/SPEC.md",
            "the-session-context-hook-template",
            "The session-context hook"
        ),
        r(
            "evidence-kit/SPEC.md",
            "baseline-manifest",
            "Baseline manifest"
        ),
        r("site-kit/SPEC.md", "the-monitor-boundary", "The monitor boundary"),
    )
}

pub fn measure() -> Result<EnforcementMap, String> {
    let gates_dir = walk::knob_scalar(GATES_DIR_KNOB)?;
    let settings = walk::knob_scalar(SETTINGS_KNOB)?;
    let scan = walk::knob_scalar(SCAN_KNOB)?;
    let blob_ref = walk::knob_scalar(BLOB_REF_KNOB)?;

    let mut sections: Vec<Section> = Vec::new();
    if let Some(s) = gate_section(&gates_dir)? {
        sections.push(s);
    }
    if let Some(s) = kpi_section(&gates_dir)? {
        sections.push(s);
    }
    sections.extend(hook_sections(&settings)?);
    if let Some(s) = suite_section()? {
        sections.push(s);
    }
    if let Some(s) = monitor_section(&scan)? {
        sections.push(s);
    }
    Ok(EnforcementMap {
        sections,
        classes: class_roster(&self_repo_prefix(&blob_ref)),
    })
}

const HEAD: &str = r#"---
title: Enforcement map
nav_parent: value
nav_child_order: 1
---
# Enforcement map

_Generated by `bash gate-sdk/bin/run-gates.sh --emit enforcement-map`; do not
hand-edit — `check-enforcement-fresh` byte-compares this page against the
emitter._

Every governed surface in this repo is held by one enforcement class, ordered
here from hardest to softest:

"#;

const MID: &str = r#"
The rows below derive from the class registries — the gate registry, the KPI
registry, the harness settings hooks, the evidence-suite config, and the
`# enforce:` markers a non-gate surface declares itself with — so this map
cannot drift from what actually runs. A registry a consumer has not adopted
leaves its section absent.

"#;

pub fn render(m: &EnforcementMap) -> String {
    let mut out = String::from(HEAD);
    out.push_str(&m.classes);
    out.push_str(MID);
    for s in &m.sections {
        out.push_str(&format!("## {}\n\n", s.title));
        out.push_str(&format!("| kit | {} |\n", s.columns.join(" | ")));
        out.push_str(&format!(
            "| --- |{}\n",
            " --- |".repeat(s.columns.len())
        ));
        for r in &s.rows {
            out.push_str(&format!(
                "| {} | {} |\n",
                kit_cell(&r.kit),
                r.cells.join(" | ")
            ));
        }
        out.push('\n');
    }
    out
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    Ok(render(&measure()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §enforcement-map — a path under no kit groups as (consumer), and the
    // first kit segment wins wherever it sits in the path
    #[test]
    fn a_surface_is_attributed_by_its_first_kit_segment() {
        assert_eq!(attribute_kit("./gate-sdk/checks/x.sh"), "gate-sdk");
        assert_eq!(attribute_kit("canon-kit/kpis/y.sh"), "canon-kit");
        assert_eq!(attribute_kit("scripts/z.sh"), "(consumer)");
        assert_eq!(command_path("env A=1 bash kit/bin/x.sh --flag"), "kit/bin/x.sh");
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the two arms P1 separates, which a test covering only
    // the refusal would miss: an empty roster is *not adopted* and drops its section, while a
    // roster naming a member the family does not carry is *adopted but broken* and refuses.
    #[test]
    fn an_empty_roster_drops_the_section_and_a_named_missing_member_refuses() {
        for k in ["EVIDENCE_KIT_SUITES", "EVIDENCE_KIT_RUN_alpha"] {
            std::env::remove_var(format!("GATE_SDK_KNOB_{}", k));
        }
        std::env::set_var("GATE_SDK_KNOB_EVIDENCE_KIT_SUITES", "");
        assert!(
            suite_section().expect("an empty roster must not refuse").is_none(),
            "an empty roster is the not-adopted case and drops its section"
        );

        std::env::set_var("GATE_SDK_KNOB_EVIDENCE_KIT_SUITES", "alpha");
        let err = match suite_section() {
            Err(e) => e,
            Ok(_) => panic!("a named-but-absent member must refuse"),
        };
        assert!(
            err.contains("alpha") && err.contains(RUN_PREFIX),
            "the refusal names the suite and the knob it looked for: {}",
            err
        );

        std::env::set_var("GATE_SDK_KNOB_EVIDENCE_KIT_RUN_alpha", "bash guard-kit/bin/x.sh");
        let s = suite_section().expect("a whole roster resolves").expect("section present");
        assert_eq!(s.rows.len(), 1);
        assert_eq!(s.rows[0].kit, "guard-kit");

        for k in ["EVIDENCE_KIT_SUITES", "EVIDENCE_KIT_RUN_alpha"] {
            std::env::remove_var(format!("GATE_SDK_KNOB_{}", k));
        }
    }

    // spec: gate-sdk/SPEC.md §enforcement-map — the settings registry's two arms, held here
    // because the front-end anchors at the repo root: the shell suite used to reach the
    // not-adopted case by running from a foreign cwd, which is no longer a reachable state.
    #[test]
    fn absent_settings_drop_the_hook_sections_and_unparseable_settings_refuse() {
        let dir = std::env::temp_dir().join(format!("enfmap-hooks-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let missing = dir.join("no-such-settings.json");
        assert!(
            hook_sections(&missing.display().to_string())
                .expect("an absent settings file must not refuse")
                .is_empty(),
            "not adopted: an absent settings file drops both hook sections"
        );
        assert!(
            hook_sections("").expect("an empty knob must not refuse").is_empty(),
            "an empty settings knob is the same not-adopted case"
        );
        let bad = dir.join("bad-settings.json");
        std::fs::write(&bad, "{ not json\n").expect("write");
        let err = match hook_sections(&bad.display().to_string()) {
            Err(e) => e,
            Ok(_) => panic!("adopted but broken: unparseable settings must refuse"),
        };
        assert!(err.contains("unparseable"), "the refusal says what is wrong: {}", err);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: gate-sdk/SPEC.md §enforcement-map — the marker needs whitespace after `enforce:` and
    // after `class=monitor`, so a longer class name cannot match by prefix
    #[test]
    fn the_monitor_marker_binds_on_whole_tokens() {
        assert_eq!(
            monitor_surface("# enforce: class=monitor the deployed site").as_deref(),
            Some("the deployed site")
        );
        assert_eq!(
            monitor_surface("   #   enforce:  class=monitor  spaced  ").as_deref(),
            Some("spaced")
        );
        assert!(monitor_surface("# enforce: class=monitoring x").is_none());
        assert!(monitor_surface("# enforce: class=gate x").is_none());
        assert!(monitor_surface("not a comment").is_none());
    }
}
