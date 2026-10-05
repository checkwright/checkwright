// spec: drift-kit/SPEC.md §The manual-operation meter — the operations an iteration's sessions
// repeated by hand, ranked, over the transcripts the stage-economics meter attributes to it.
// spec: gate-sdk/SPEC.md §The non-gate arm — an arm table member because it reads consumer knobs,
// and `Arm::Emit` because exit is always 0 bar a usage error, so no `1` is load-bearing.
use std::collections::{BTreeSet, HashMap, HashSet};

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_STATE_FILE",
    "DRIFT_KIT_SESSIONS_DIR",
    "DRIFT_KIT_SUPERVISION_LABEL",
    "DRIFT_KIT_FANOUT_SUFFIX",
    "DRIFT_KIT_METRIC_DIR",
    "DRIFT_KIT_MANUAL_OPS_LOG",
    "DRIFT_KIT_MANUAL_OPS_IGNORE",
    "DRIFT_KIT_MANUAL_OPS_TOP",
];

pub const USAGE: &str = "usage: --emit manual-ops [iteration]\n  bare: the iteration the last stamp \
                         in DRIFT_KIT_STATE_FILE names";

// spec: drift-kit/SPEC.md §The manual-operation meter — the harness's transcript vocabulary: its
// shell tools and its file-writing tools, each with the input field that keys the call
const SHELL_TOOLS: &[&str] = &["Bash", "PowerShell"];
const EDIT_TOOLS: &[(&str, &str)] = &[
    ("Edit", "file_path"),
    ("MultiEdit", "file_path"),
    ("Write", "file_path"),
    ("NotebookEdit", "notebook_path"),
];

const OUTSIDE: &str = "(outside)";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Kind {
    Shell,
    Edit,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Shell => "shell",
            Kind::Edit => "edit",
        }
    }
}

// spec: drift-kit/SPEC.md §The manual-operation meter — an edit's key: its path relative to the
// line's cwd, else to the toplevel; a path under neither is never printed
fn edit_key(path: &str, cwd: Option<&str>, top: Option<&str>) -> String {
    let base = cwd.or(top).unwrap_or("");
    let abs = crate::walk::abs_against(base, path);
    for root in [cwd, top].into_iter().flatten() {
        if let Some(rel) = crate::walk::rel_under(&crate::walk::normalize_abs(root), &abs) {
            return rel.replace('\\', "/");
        }
    }
    OUTSIDE.to_string()
}

// spec: drift-kit/SPEC.md §The manual-operation meter — the operations: one `tool_use` block on an
// assistant line is one call, a block id repeated across a streaming transcript counted once
fn calls(body: &str, top: Option<&str>) -> Vec<(Kind, String)> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for line in body.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if v.get("type").and_then(|t| t.as_str()) != Some("assistant") {
            continue;
        }
        let cwd = v.get("cwd").and_then(|c| c.as_str()).filter(|c| !c.is_empty());
        let Some(blocks) = v.pointer("/message/content").and_then(|c| c.as_array()) else {
            continue;
        };
        for b in blocks {
            if b.get("type").and_then(|t| t.as_str()) != Some("tool_use") {
                continue;
            }
            if let Some(id) = b.get("id").and_then(|i| i.as_str()) {
                if !seen.insert(id.to_string()) {
                    continue;
                }
            }
            let name = b.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let field = |k: &str| b.pointer(&format!("/input/{}", k)).and_then(|x| x.as_str());
            let call = if SHELL_TOOLS.contains(&name) {
                let cmd = field("command").unwrap_or("");
                let key = if name == "PowerShell" {
                    super::scan_prompts::powershell_key(cmd)
                } else {
                    super::scan_prompts::ranking_key(cmd)
                };
                (!key.is_empty()).then_some((Kind::Shell, key))
            } else if let Some((_, f)) = EDIT_TOOLS.iter().find(|(t, _)| *t == name) {
                field(f)
                    .filter(|p| !p.is_empty())
                    .map(|p| (Kind::Edit, edit_key(p, cwd, top)))
            } else {
                None
            };
            out.extend(call);
        }
    }
    out
}

#[derive(Default)]
struct Tally {
    calls: u64,
    sessions: BTreeSet<String>,
    rows: BTreeSet<String>,
}

// spec: drift-kit/SPEC.md §The manual-operation meter — the ranking: ignored keys dropped, then
// calls descending, sessions descending, key ascending, the first `top` kept
fn ranked(
    tally: &HashMap<(Kind, String), Tally>,
    ignore: &[String],
    top: usize,
) -> Vec<(Kind, String)> {
    let mut keys: Vec<&(Kind, String)> =
        tally.keys().filter(|(_, k)| !ignore.contains(k)).collect();
    keys.sort_by(|a, b| {
        let (ta, tb) = (&tally[*a], &tally[*b]);
        tb.calls
            .cmp(&ta.calls)
            .then(tb.sessions.len().cmp(&ta.sessions.len()))
            .then(a.1.cmp(&b.1))
            .then(a.0.cmp(&b.0))
    });
    keys.into_iter().take(top).cloned().collect()
}

// spec: drift-kit/SPEC.md §The manual-operation meter — the log: the iteration's lines replaced
// where they stand, a new iteration's appended
fn rewritten(kept: &[String], iteration: &str, fresh: &[String]) -> Vec<String> {
    let mine = |l: &str| l.split(' ').next() == Some(iteration);
    let kept: Vec<&String> = kept.iter().filter(|l| !l.is_empty()).collect();
    let mut out: Vec<String> = Vec::new();
    let mut placed = false;
    for l in kept {
        if mine(l) {
            if !placed {
                out.extend(fresh.iter().cloned());
                placed = true;
            }
        } else {
            out.push(l.clone());
        }
    }
    if !placed {
        out.extend(fresh.iter().cloned());
    }
    out
}

fn iterations_ranked(log: &[String], kind: Kind, key: &str) -> usize {
    log.iter()
        .filter_map(|l| {
            let mut f = l.splitn(3, ' ');
            let (it, k, rest) = (f.next()?, f.next()?, f.next()?);
            (k == kind.name() && rest == key).then_some(it)
        })
        .collect::<HashSet<&str>>()
        .len()
}

// spec: drift-kit/SPEC.md §The manual-operation meter — the bare operand's iteration: the last data
// line of the live state file
fn last_stamped(state_file: &str) -> Option<String> {
    let text = std::fs::read_to_string(state_file).ok()?;
    text.lines()
        .map(str::trim)
        .rfind(|l| !l.is_empty() && !l.starts_with('#') && *l != "---")
        .and_then(|l| l.split_whitespace().next())
        .map(str::to_string)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let given = super::file_survey::positionals(args, "iteration")
        .map_err(|e| format!("{}\n{}", e, USAGE))?;
    if given.len() > 1 {
        return Err(USAGE.to_string());
    }
    let state_file = crate::walk::knob_scalar("DRIFT_KIT_STATE_FILE")?;
    let log = crate::walk::knob_scalar("DRIFT_KIT_MANUAL_OPS_LOG")?;
    let ignore = crate::walk::knob_array("DRIFT_KIT_MANUAL_OPS_IGNORE")?;
    let top_raw = crate::walk::knob_scalar("DRIFT_KIT_MANUAL_OPS_TOP")?;
    let top: usize = top_raw
        .parse()
        .map_err(|_| format!("DRIFT_KIT_MANUAL_OPS_TOP '{}' is not a positive integer", top_raw))?;

    let Some(iteration) = given.first().cloned().or_else(|| last_stamped(&state_file)) else {
        return Ok(format!(
            "manual-ops: no stamp in {} names an iteration — nothing to read\n  help: pass an \
             iteration, or set DRIFT_KIT_STATE_FILE to the WORKFLOW-STATE path carrying the stage \
             stamps.\n",
            state_file
        ));
    };
    let dir = crate::sessions::sessions_dir(&super::stage_economics::inputs()?);
    if !std::path::Path::new(&dir).is_dir() {
        return Ok(format!(
            "manual-ops: no sessions dir {} — set DRIFT_KIT_SESSIONS_DIR\n",
            dir
        ));
    }

    let top_dir = crate::walk::toplevel_opt().ok().flatten();
    let mut tally: HashMap<(Kind, String), Tally> = HashMap::new();
    let (mut transcripts, mut counted) = (0usize, 0u64);
    for a in super::stage_economics::attribution()? {
        if !a.iterations.contains(&iteration) {
            continue;
        }
        transcripts += 1;
        let Ok(body) = std::fs::read(&a.transcript) else {
            continue;
        };
        for (kind, key) in calls(&String::from_utf8_lossy(&body), top_dir.as_deref()) {
            let t = tally.entry((kind, key)).or_default();
            t.calls += 1;
            t.sessions.insert(a.transcript.clone());
            t.rows.insert(a.row.clone());
            counted += 1;
        }
    }
    let mut out = format!(
        "manual-ops: {} — {} call(s) in {} transcript(s)\n",
        iteration, counted, transcripts
    );
    let printed = ranked(&tally, &ignore, top);
    if printed.is_empty() {
        out.push_str("  no candidate — nothing logged\n");
        return Ok(out);
    }

    let at = crate::walk::capture_path(&log);
    let kept: Vec<String> = std::fs::read(&at)
        .map(|b| String::from_utf8_lossy(&b).lines().map(str::to_string).collect())
        .unwrap_or_default();
    let fresh: Vec<String> = printed
        .iter()
        .map(|(kind, key)| format!("{} {} {}", iteration, kind.name(), key))
        .collect();
    let body = rewritten(&kept, &iteration, &fresh);
    for (kind, key) in &printed {
        let t = &tally[&(*kind, key.clone())];
        let rows: Vec<&str> = t.rows.iter().map(String::as_str).collect();
        out.push_str(&format!(
            "  {} {} {} {} {} [{}]\n",
            t.calls,
            t.sessions.len(),
            iterations_ranked(&body, *kind, key),
            kind.name(),
            key,
            rows.join(",")
        ));
    }
    out.push_str(
        "  (calls sessions iterations kind key [rows] — a candidate, never a verdict: a high count \
         can be the work itself)\n",
    );
    if let Some(parent) = std::path::Path::new(&at).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut text = body.join("\n");
    text.push('\n');
    std::fs::write(&at, text).map_err(|e| format!("cannot write {}: {}", log, e))?;
    out.push_str(&format!("  logged: {} ({} key(s))\n", log, printed.len()));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(cwd: &str, blocks: &str) -> String {
        format!(
            "{{\"type\":\"assistant\",\"cwd\":\"{}\",\"message\":{{\"content\":[{}]}}}}",
            cwd, blocks
        )
    }

    fn tool(id: &str, name: &str, input: &str) -> String {
        format!(
            "{{\"type\":\"tool_use\",\"id\":\"{}\",\"name\":\"{}\",\"input\":{}}}",
            id, name, input
        )
    }

    // spec: drift-kit/SPEC.md §The manual-operation meter — a shell call keys through the
    // prompt-friction ranking key, so the two channels name a shape alike
    #[test]
    fn a_shell_call_keys_by_the_ranking_key() {
        let body = line(
            "/r",
            &format!(
                "{},{}",
                tool("a", "Bash", "{\"command\":\"git status --short\"}"),
                tool("b", "Bash", "{\"command\":\"bash gate-sdk/bin/run-gates.sh --run\"}")
            ),
        );
        assert_eq!(
            calls(&body, None),
            vec![
                (Kind::Shell, "git status".to_string()),
                (Kind::Shell, "bash gate-sdk/bin/run-gates.sh".to_string())
            ]
        );
    }

    // spec: drift-kit/SPEC.md §The manual-operation meter — an edit keys under its cwd, else under
    // the toplevel, else as `(outside)`, so no path off the repository is printed
    #[test]
    fn an_edit_keys_under_its_cwd_then_the_toplevel_else_outside() {
        assert_eq!(edit_key("/r/sub/a.md", Some("/r/sub"), Some("/r")), "a.md");
        assert_eq!(edit_key("/r/b.md", Some("/r/sub"), Some("/r")), "b.md");
        assert_eq!(edit_key("/r/b.md", None, Some("/r")), "b.md");
        assert_eq!(edit_key("/etc/passwd", Some("/r/sub"), Some("/r")), OUTSIDE);
        assert_eq!(edit_key("../../x", Some("/r/sub"), Some("/r")), OUTSIDE);
        assert_eq!(edit_key("c.md", Some("/r"), None), "c.md");
    }

    // spec: drift-kit/SPEC.md §The manual-operation meter — any other tool and an unparseable line
    // count nothing, and a repeated block id counts once
    #[test]
    fn other_tools_unparseable_lines_and_repeated_ids_count_nothing() {
        let w = tool("w", "Write", "{\"file_path\":\"/r/q.md\"}");
        let body = format!(
            "not json\n{}\n{}\n",
            line("/r", &format!("{},{}", w, tool("x", "Read", "{\"file_path\":\"/r/q.md\"}"))),
            line("/r", &w)
        );
        assert_eq!(calls(&body, None), vec![(Kind::Edit, "q.md".to_string())]);
    }

    fn tally_of(rows: &[(Kind, &str, u64, &[&str])]) -> HashMap<(Kind, String), Tally> {
        rows.iter()
            .map(|(k, key, n, sessions)| {
                (
                    (*k, key.to_string()),
                    Tally {
                        calls: *n,
                        sessions: sessions.iter().map(|s| s.to_string()).collect(),
                        rows: BTreeSet::new(),
                    },
                )
            })
            .collect()
    }

    // spec: drift-kit/SPEC.md §The manual-operation meter — the ignore set drops a key whole, the
    // order is calls, then sessions, then key, and `top` bounds it
    #[test]
    fn the_ranking_ignores_orders_and_bounds() {
        let t = tally_of(&[
            (Kind::Shell, "grep", 9, &["s1"]),
            (Kind::Edit, "b.md", 4, &["s1"]),
            (Kind::Edit, "a.md", 4, &["s1", "s2"]),
            (Kind::Shell, "cat", 4, &["s1"]),
        ]);
        let order = |ignore: &[&str], top: usize| -> Vec<String> {
            let ig: Vec<String> = ignore.iter().map(|s| s.to_string()).collect();
            ranked(&t, &ig, top).into_iter().map(|(_, k)| k).collect()
        };
        assert_eq!(order(&["grep"], 10), vec!["a.md", "b.md", "cat"]);
        assert_eq!(order(&[], 2), vec!["grep", "a.md"]);
    }

    // spec: drift-kit/SPEC.md §The manual-operation meter — a re-run replaces the iteration's lines
    // where they stand and a new iteration appends; the iterations column counts distinct ones
    #[test]
    fn the_log_replaces_in_place_and_counts_iterations() {
        let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let kept = s(&["a shell git status", "b edit q.md", "b shell ls", "c shell ls"]);
        let fresh = s(&["b shell ls"]);
        assert_eq!(
            rewritten(&kept, "b", &fresh),
            s(&["a shell git status", "b shell ls", "c shell ls"])
        );
        assert_eq!(rewritten(&kept, "d", &fresh).last().map(String::as_str), Some("b shell ls"));
        assert_eq!(iterations_ranked(&kept, Kind::Shell, "ls"), 2);
        assert_eq!(iterations_ranked(&kept, Kind::Edit, "ls"), 0);
    }
}
