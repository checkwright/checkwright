// spec: drift-kit/SPEC.md §The report skeleton — the collator: it owns the frame, every
// measurement lives in a member. Advisory by construction, so the arm's exit is always 0 and a
// member that fails yields a visible row rather than a missing one.
use super::kpi::{self, Ctx};
use crate::proc;
use crate::stages;
use crate::walk;
use std::path::Path;

// spec: drift-kit/SPEC.md §The KPI plugin contract — the declared reads: `DRIFT_KIT_*` is the
// **prefix family**, the kit's family read rather than a transcribed roster, so a consumer knob file's
// own scalar resolves and reaches a plugin unchanged.
pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_*",
    "GATE_SDK_GATES_DIR",
    "GATE_KIT_ROOTS_HERE",
    "GUARD_KIT_LOG",
    "GUARD_KIT_SETTINGS",
    "GUARD_KIT_SETTINGS_LOCAL",
    // spec: drift-kit/SPEC.md §Bundled KPIs — what `kpi-always-loaded` reads in process, which a
    // spawned child used to resolve for itself; `CONTEXT_KIT_GROWTH_PATHS` is deliberately absent,
    // the row reading the bare mode only.
    "CONTEXT_KIT_SURFACES",
    "CONTEXT_KIT_HOOK_CMD",
    "CONTEXT_KIT_BASELINE_FILE",
];

const HEADER: &str = "=== Drift KPIs (advisory — trend, not level) ===";
const LEAD_BANNER: &str = "--- Lead (weighted high — act before drift compounds) ---";
const LAG_BANNER: &str = "--- Lag (weighted low — undercounts by construction) ---";
const FOOTER: &str = "Read trend across sessions; lag KPIs lower-bound only.";

// spec: drift-kit/SPEC.md §The KPI plugin contract — every scalar the family read resolves, keyed by
// the suffix the prefix leaves
fn family() -> Result<Vec<(String, String)>, String> {
    Ok(crate::knobs::family("DRIFT_KIT_")?
        .into_iter()
        .filter_map(|(n, v)| n.strip_prefix("DRIFT_KIT_").map(|s| (s.to_string(), v)))
        .collect())
}

fn scalar(fam: &[(String, String)], suffix: &str) -> String {
    walk::knob_in_family(fam, suffix).unwrap_or_default()
}

// spec: drift-kit/SPEC.md §The report skeleton — the iteration-start commit, derived **before** the
// member loop so the same value reaches every member and the header, off drift-kit's own
// state-file knob and never off the iteration's name.
fn iteration_start(fam: &[(String, String)]) -> String {
    stages::iteration_start(&scalar(fam, "STATE_FILE"))
}


// spec: drift-kit/SPEC.md §The extensibility contract — the three resolution tiers, consumer-first:
// the adopter's own dirs, then each vendored kit's `kpis/`, then the crate's built-in members. The
// first two answer with a path to execute directly; the third with a compiled member.
enum Resolved {
    Plugin(String),
    Builtin(kpi::Member),
}

fn resolve(name: &str, kpi_dirs: &[String], kit_roots: &[String]) -> Option<Resolved> {
    for d in kpi_dirs {
        let p = format!("{}/{}.sh", d, name);
        if Path::new(&p).is_file() {
            return Some(Resolved::Plugin(p));
        }
    }
    for k in kit_roots {
        let p = format!("{}/kpis/{}.sh", k, name);
        if Path::new(&p).is_file() {
            return Some(Resolved::Plugin(p));
        }
    }
    kpi::lookup(name).map(Resolved::Builtin)
}

// spec: drift-kit/SPEC.md §The KPI plugin contract — the exported environment a consumer plugin
// reads: every scalar the family read resolves, plus the two driver handoffs, which are
// recomputed every run and are not consumer knobs.
fn child_env(fam: &[(String, String)], ctx: &Ctx) -> Vec<(String, String)> {
    let mut env: Vec<(String, String)> = fam
        .iter()
        .map(|(suffix, value)| (format!("DRIFT_KIT_{}", suffix), value.clone()))
        .collect();
    env.push((
        "DRIFT_KIT_KIT_ROOTS".to_string(),
        ctx.kit_roots.join("\n"),
    ));
    env.push((
        "DRIFT_KIT_ITERATION_START".to_string(),
        ctx.iteration_start.clone(),
    ));
    env
}

fn members(kpis_file: &str) -> Vec<String> {
    std::fs::read(kpis_file)
        .map(|b| {
            String::from_utf8_lossy(&b)
                .lines()
                .filter(|l| {
                    let s = l.trim_start_matches([' ', '\t']);
                    !(s.is_empty() || s.starts_with('#'))
                })
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

// spec: drift-kit/SPEC.md §The report skeleton — degrade discipline: a member that exits non-zero
// or prints nothing is the fail-visible row, because a silently vanishing KPI is itself drift.
fn invoke(r: &Resolved, ctx: &Ctx, trend: bool, env: &[(String, String)]) -> Option<String> {
    match r {
        Resolved::Builtin(f) => f(ctx, trend),
        Resolved::Plugin(p) => {
            let args: Vec<&str> = if trend { vec!["--trend"] } else { vec![] };
            proc::run_with_env(p, &args, env)
                .ok()
                .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
        }
    }
}

fn render_rows(rows: &[(String, String)]) -> String {
    let mut out = String::new();
    for (label, value) in rows {
        out.push_str(&format!("  {:<22}  {}\n", label, value));
    }
    out
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let trend = args.iter().any(|a| a == "--trend");
    let fam = family()?;
    let kit_roots = walk::kit_roots_abs()?;

    let ctx = Ctx {
        iteration_start: iteration_start(&fam),
        queue_file: scalar(&fam, "QUEUE_FILE"),
        knowledge_log: scalar(&fam, "KNOWLEDGE_LOG"),
        timings_file: scalar(&fam, "TIMINGS_FILE"),
        overhead_log: scalar(&fam, "OVERHEAD_LOG"),
        price_table: scalar(&fam, "PRICE_TABLE"),
        gates_dir: walk::knob_scalar("GATE_SDK_GATES_DIR")?,
        guard_log: walk::knob_scalar("GUARD_KIT_LOG")?,
        settings: walk::knob_scalar("GUARD_KIT_SETTINGS")?,
        settings_local: walk::knob_scalar("GUARD_KIT_SETTINGS_LOCAL")?,
        done_section: scalar(&fam, "DONE_SECTION"),
        deferred_section: scalar(&fam, "DEFERRED_SECTION"),
        icebox_section: scalar(&fam, "ICEBOX_SECTION"),
        kit_roots,
        };

    let env = child_env(&fam, &ctx);
    let kpi_dirs = walk::knob_array("DRIFT_KIT_KPI_DIRS").unwrap_or_default();
    let registry = members(&scalar(&fam, "KPIS_FILE"));

    if trend {
        let mut frags: Vec<String> = Vec::new();
        for name in &registry {
            let r = match resolve(name, &kpi_dirs, &ctx.kit_roots) {
                Some(r) => r,
                None => continue,
            };
            let frag = invoke(&r, &ctx, true, &env).unwrap_or_default();
            let frag = frag.lines().next().unwrap_or("");
            if !frag.is_empty() {
                frags.push(frag.to_string());
            }
        }
        if frags.is_empty() {
            return Ok(String::new());
        }
        return Ok(format!("drift: {}\n", frags.join(" · ")));
    }

    let mut lead: Vec<(String, String)> = Vec::new();
    let mut lag: Vec<(String, String)> = Vec::new();
    for name in &registry {
        let r = match resolve(name, &kpi_dirs, &ctx.kit_roots) {
            Some(r) => r,
            None => {
                lead.push((
                    name.clone(),
                    "n/a (unresolved — not in any KPI dir)".to_string(),
                ));
                continue;
            }
        };
        let out = invoke(&r, &ctx, false, &env).unwrap_or_default();
        let out = out.trim_end_matches('\n');
        if out.is_empty() {
            lead.push((name.clone(), "n/a (plugin failed)".to_string()));
            continue;
        }
        for row in out.lines() {
            let mut f = row.splitn(3, '\t');
            let section = f.next().unwrap_or("");
            if section.is_empty() {
                continue;
            }
            let label = f.next().unwrap_or("").to_string();
            let value = f.next().unwrap_or("").to_string();
            if section == "lag" {
                lag.push((label, value));
            } else {
                lead.push((label, value));
            }
        }
    }

    let mut out = String::from(HEADER);
    if !ctx.iteration_start.is_empty() {
        out.push_str(&format!("  [iteration start {}]", ctx.iteration_start));
    }
    out.push_str("\n\n");
    out.push_str(LEAD_BANNER);
    out.push('\n');
    out.push_str(&if lead.is_empty() {
        "  (none registered)\n".to_string()
    } else {
        render_rows(&lead)
    });
    out.push('\n');
    out.push_str(LAG_BANNER);
    out.push('\n');
    out.push_str(&if lag.is_empty() {
        "  (none registered)\n".to_string()
    } else {
        render_rows(&lag)
    });
    out.push('\n');
    out.push_str(FOOTER);
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §The extensibility contract — the consumer tier resolves before the
    // built-in one, which is the shadowing promise; a name nothing answers resolves to nothing
    #[test]
    fn a_consumer_plugin_shadows_the_built_in_of_the_same_name() {
        let dir = std::env::temp_dir().join(format!("cw-kpi-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot make the fixture dir");
        let shadow = dir.join("kpi-task-split.sh");
        std::fs::write(&shadow, "#!/bin/sh\n").expect("cannot write the fixture plugin");
        let dirs = vec![dir.display().to_string()];

        match resolve("kpi-task-split", &dirs, &[]) {
            Some(Resolved::Plugin(p)) => assert_eq!(p, shadow.display().to_string()),
            _ => panic!("the consumer dir did not shadow the built-in"),
        }
        assert!(
            matches!(resolve("kpi-task-split", &[], &[]), Some(Resolved::Builtin(_))),
            "the built-in tier did not answer once no file did"
        );
        assert!(resolve("kpi-nobody-ships-this", &dirs, &[]).is_none());
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: drift-kit/SPEC.md §The KPI plugin contract — the exported environment: every family
    // scalar plus the two handoffs
    #[test]
    fn the_child_environment_carries_the_family_and_both_handoffs() {
        let fam = vec![
            ("QUEUE_FILE".to_string(), "TASK-QUEUE.md".to_string()),
            ("SESSIONS_DIR".to_string(), "/tmp/sessions".to_string()),
            ("SMOKE_CUSTOM".to_string(), "reached".to_string()),
        ];
        let ctx = ctx_for_test();
        let env = child_env(&fam, &ctx);
        let has = |k: &str| env.iter().any(|(n, _)| n == k);
        assert!(has("DRIFT_KIT_QUEUE_FILE") && has("DRIFT_KIT_SESSIONS_DIR") && has("DRIFT_KIT_SMOKE_CUSTOM"));
        assert!(has("DRIFT_KIT_KIT_ROOTS") && has("DRIFT_KIT_ITERATION_START"));
    }

    // spec: drift-kit/SPEC.md §The report skeleton — the value reaching `Ctx` is the first stamp's
    // head read through `DRIFT_KIT_STATE_FILE`, whatever the queue header names, and an unset or
    // absent state file hands every member the empty string
    #[test]
    fn the_iteration_start_reaching_ctx_is_the_state_files_first_head() {
        let head = proc::run("git", &["rev-parse", "--short", "HEAD"])
            .ok()
            .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
            .expect("the crate's tests run inside a work tree");
        let dir = std::env::temp_dir().join(format!("cw-iter-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot make the fixture dir");
        let state = dir.join("WORKFLOW-STATE.txt");
        std::fs::write(
            &state,
            format!("# hdr\n---\n\nit scope s1 2026-09-14 {}\nit build s2 2026-09-14 0000000\n", head),
        )
        .expect("cannot write the fixture state file");
        let fam = vec![("STATE_FILE".to_string(), state.display().to_string())];
        assert_eq!(iteration_start(&fam), head);
        let absent = vec![(
            "STATE_FILE".to_string(),
            dir.join("absent.txt").display().to_string(),
        )];
        assert_eq!(iteration_start(&absent), "");
        assert_eq!(iteration_start(&[]), "");
        std::fs::remove_dir_all(&dir).ok();
    }

    fn ctx_for_test() -> Ctx {
        Ctx {
            queue_file: String::new(),
            knowledge_log: String::new(),
            timings_file: String::new(),
            overhead_log: String::new(),
            price_table: String::new(),
            gates_dir: String::new(),
            guard_log: String::new(),
            settings: String::new(),
            settings_local: String::new(),
            done_section: String::new(),
            deferred_section: String::new(),
            icebox_section: String::new(),
            kit_roots: vec!["/tmp/kit".to_string()],
            iteration_start: "abc1234".to_string(),
        }
    }
}
