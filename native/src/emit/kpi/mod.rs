// spec: drift-kit/SPEC.md §The KPI plugin contract — the built-in members: the third resolution
// tier, reached only after the consumer's own dirs and the vendored kits' `kpis/` miss. Each
// returns the bytes its shell original printed, so the collator parses one grammar either way.
pub mod always_loaded;
pub mod amendment_age;
pub mod deferred_age;
pub mod gate_backlog;
pub mod gate_runtime;
pub mod incident_recurrence;
pub mod knowledge_friction;
pub mod overhead;
pub mod price_table_age;
pub mod prompt_friction;
pub mod queue_net_delta;
pub mod settings_local;
pub mod task_split;

use crate::proc;

// spec: drift-kit/SPEC.md §The KPI plugin contract — the resolved knob set plus the two driver
// handoffs, which is what a built-in reads where a consumer plugin reads the exported environment;
// the pair is the same value at the same transition, which is the extension point's promise.
pub struct Ctx {
    pub queue_file: String,
    pub knowledge_log: String,
    pub timings_file: String,
    pub overhead_log: String,
    pub price_table: String,
    pub gates_dir: String,
    pub guard_log: String,
    pub settings: String,
    pub settings_local: String,
    pub done_section: String,
    pub deferred_section: String,
    pub icebox_section: String,
    pub kit_roots: Vec<String>,
    pub iteration_start: String,
}

// spec: drift-kit/SPEC.md §The KPI plugin contract — a member's whole result is the bytes it would
// have printed; `None` is the shell's non-zero exit, which the collator degrades to its
// fail-visible row rather than to silence.
pub type Member = fn(&Ctx, bool) -> Option<String>;

// spec: drift-kit/SPEC.md §Bundled KPIs — the built-in roster, keyed by the registry name a
// consumer file of the same name shadows. It is also what holds the shipped registry template in
// population once the sibling directory is gone (gate-sdk/SPEC.md §check-template-registry-parity).
pub const BUILTINS: &[(&str, Member)] = &[
    ("kpi-always-loaded", always_loaded::run),
    ("kpi-amendment-age", amendment_age::run),
    ("kpi-deferred-age", deferred_age::run),
    ("kpi-gate-backlog", gate_backlog::run),
    ("kpi-gate-runtime", gate_runtime::run),
    ("kpi-incident-recurrence", incident_recurrence::run),
    ("kpi-knowledge-friction", knowledge_friction::run),
    ("kpi-overhead", overhead::run),
    ("kpi-price-table-age", price_table_age::run),
    ("kpi-prompt-friction", prompt_friction::run),
    ("kpi-queue-net-delta", queue_net_delta::run),
    ("kpi-settings-local", settings_local::run),
    ("kpi-task-split", task_split::run),
];

// spec: drift-kit/SPEC.md §The extensibility contract — the kit and sibling directory whose
// registry these members answer, declared once: the parity gate's population predicate and the
// enforcement map's attribution are two compiled readers of that one fact.
pub const REGISTRY_KIT: &str = "drift-kit";
pub const REGISTRY_DIR: &str = "kpis";

pub fn lookup(name: &str) -> Option<Member> {
    BUILTINS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

pub fn names() -> Vec<&'static str> {
    BUILTINS.iter().map(|(n, _)| *n).collect()
}

// spec: drift-kit/SPEC.md §The KPI plugin contract — a sibling kit's tool is found through the
// driver's `DRIFT_KIT_KIT_ROOTS` handoff, which a built-in reads as the resolved root set and a
// consumer plugin reads as exported environment; neither re-derives the roster.
pub fn sibling_tool(roots: &[String], rel: &str) -> Option<String> {
    roots
        .iter()
        .map(|k| format!("{}/{}", k, rel))
        .find(|p| std::path::Path::new(p).is_file())
}

pub fn read(path: &str) -> Option<String> {
    std::fs::read(path)
        .ok()
        .map(|b| String::from_utf8_lossy(&b).into_owned())
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the `n/a (<reason>)` degrade in the member's own value:
// a missing surface is a readable row, and `--trend` volunteers nothing rather than a null fragment.
pub fn na(section: &str, label: &str, reason: &str, trend: bool) -> Option<String> {
    if trend {
        return Some(String::new());
    }
    Some(format!("{}\t{}\tn/a ({})\n", section, label, reason))
}

pub fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

// spec: drift-kit/SPEC.md §Bundled KPIs — `date -d` is kept rather than replaced by an in-crate
// civil-date conversion, which would resolve UTC where every shell original resolved the operator's
// zone; the queue-index arm's cutoff is the standing precedent for that choice.
pub fn date_epoch(day: &str) -> Option<i64> {
    let c = proc::run("date", &["-d", day, "+%s"]).ok()?;
    let out = c.stdout()?;
    String::from_utf8_lossy(out).trim().parse::<i64>().ok()
}

// spec: drift-kit/SPEC.md §Bundled KPIs — `date +%F`, the operator's civil today, which is the
// anchor an expiry counts from; the same zone question `date_epoch` answers keeps it a subprocess.
pub fn today_iso() -> String {
    proc::run("date", &["+%F"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
        .unwrap_or_default()
}

pub fn is_iso_day(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && [0, 1, 2, 3, 5, 6, 8, 9]
            .iter()
            .all(|i| b[*i].is_ascii_digit())
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the awk section walk every queue-reading member shares:
// lines under `## <name>`, reset by the next `## ` heading whatever it names, so a section placed
// after the scanned one drops out of the input by construction.
pub fn section_lines<'a>(text: &'a str, name: &str) -> Vec<&'a str> {
    let head = format!("## {}", name);
    let mut inx = false;
    let mut out: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.starts_with("## ") {
            inx = line.trim_end() == head;
            continue;
        }
        if inx {
            out.push(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the roster is the registry's other half, so a
    // built-in nobody registers and a registered name nothing answers are both defects
    #[test]
    fn every_built_in_resolves_under_its_registry_name_and_nothing_else_does() {
        for n in names() {
            assert!(lookup(n).is_some(), "{} is in the roster but resolves to nothing", n);
        }
        assert!(lookup("kpi-always-loaded.sh").is_none());
        assert!(lookup("always-loaded").is_none());
        assert_eq!(BUILTINS.len(), 13, "the bundled set moved without its roster");
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — the section walk resets on any heading, which is what
    // keeps a later section's lines out; the control is a second section carrying a lookalike line
    #[test]
    fn the_section_walk_stops_at_the_next_heading_whatever_it_names() {
        let t = "## Deferred\n- alpha\n## Icebox\n- beta\n";
        assert_eq!(section_lines(t, "Deferred"), vec!["- alpha"]);
        assert_eq!(section_lines(t, "Icebox"), vec!["- beta"]);
        assert!(section_lines(t, "Done").is_empty());
    }

    #[test]
    fn an_iso_day_is_ten_characters_with_two_separators_and_eight_digits() {
        assert!(is_iso_day("2026-08-29"));
        assert!(!is_iso_day("2026-8-29"));
        assert!(!is_iso_day("2026-08-291"));
        assert!(!is_iso_day("not-a-date"));
    }
}
