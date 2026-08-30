// spec: drift-kit/SPEC.md §The published-evidence extractor — the governed-trajectory table, a
// pure function of committed history. The stage roster and the evidence-surface pair are the
// consumer's vocabulary and cross the config bridge; not one stage name is written here.
use crate::proc;
use crate::walk;
use std::collections::{HashMap, HashSet};

const SURFACES_KNOB: &str = "DRIFT_KIT_TRAJECTORY_SURFACES";
const GATES_KNOB: &str = "DRIFT_KIT_GATES_FILE";
const STAGES_KNOB: &str = "DRIFT_KIT_STAGES";

// spec: drift-kit/SPEC.md §The published-evidence extractor — the close stamp is the row's
// existence condition, so the stage that closes an iteration is the one fixed name the table's
// shape depends on rather than a roster member it renders
const CLOSE_STAGE: &str = "close";

// spec: gate-sdk/SPEC.md §The consumer remainder cohort — the extractor anchored itself with a
// `cd` to the toplevel and fell back to the cwd outside a repository, where every git read then
// fails and the table degrades to its one n/a row. Both halves are reproduced.
fn toplevel() -> String {
    crate::walk::toplevel()
        .or_else(|_| crate::walk::cwd())
        .unwrap_or_else(|_| ".".to_string())
}

struct Git {
    top: String,
}

impl Git {
    // spec: gate-sdk/SPEC.md §Fail-closed contract — a git read that did not succeed yields no
    // stdout, and every caller here treats that as "this history is not there", which is the
    // extractor's own reading: it is advisory and exits 0 whatever git says.
    fn read(&self, args: &[&str]) -> Option<String> {
        let mut argv: Vec<&str> = vec!["-C", &self.top];
        argv.extend_from_slice(args);
        proc::run("git", &argv)
            .ok()
            .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
    }

    fn has(&self, args: &[&str]) -> bool {
        self.read(args).is_some()
    }
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — each stage's slot label is its
// shortest prefix unique among the roster; header legend and cells read this one map so they
// cannot drift, and a non-colliding roster reduces every label to its single letter.
fn abbreviations(stages: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for s in stages {
        let chars: Vec<char> = s.chars().collect();
        let mut len = 1usize;
        while len < chars.len() {
            let pfx: String = chars[..len].iter().collect();
            let collide = stages
                .iter()
                .any(|o| o != s && o.chars().take(len).collect::<String>() == pfx);
            if !collide {
                break;
            }
            len += 1;
        }
        out.push(chars[..len.min(chars.len())].iter().collect());
    }
    out
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `date -d` over git's own `--date=short` output was a
// day-difference dressed as an epoch subtraction, so it becomes a civil-date-to-day-count helper
// needing no dependency; it is also exact where the shell form truncated across a DST boundary.
fn days_from_civil(date: &str) -> Option<i64> {
    let mut it = date.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    if it.next().is_some() || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146097 + doe - 719468)
}

#[derive(Default)]
struct Iteration {
    stages: Vec<String>,
    close: Option<String>,
    feat: usize,
    debt: usize,
    amendments: usize,
    max_lag: i64,
    val_suites: usize,
    val_fail: i64,
    val_present: bool,
    gates: String,
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — the harvest's stamp grammar, the
// `^\+<iter> <stage> ` the shell form pre-filtered its diff with: one space each side of the
// stage token and a kebab iteration, which is what keeps a `+++ b/...` header out.
fn added_stamp<'a>(line: &'a str, stages: &[String]) -> Option<(&'a str, &'a str)> {
    let rest = line.strip_prefix('+')?;
    let space = rest.find(' ')?;
    let (iter, tail) = (&rest[..space], &rest[space + 1..]);
    if iter.is_empty()
        || !iter
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return None;
    }
    for s in stages {
        if let Some(after) = tail.strip_prefix(s.as_str()) {
            if after.starts_with(' ') {
                return Some((iter, &tail[..s.len()]));
            }
        }
    }
    None
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — iteration+stages harvest; the state
// file truncates at scope, so history keeps every stamp and the committed file keeps only the
// live one. Order is first-seen, which is chronological under `--reverse`.
fn harvest_stamps(
    git: &Git,
    state_file: &str,
    stages: &[String],
    order: &mut Vec<String>,
    iters: &mut HashMap<String, Iteration>,
) {
    if !git.has(&["cat-file", "-e", &format!("HEAD:{}", state_file)]) {
        return;
    }
    let log = match git.read(&[
        "log",
        "--reverse",
        "--format=COMMIT %H",
        "-p",
        "-U0",
        "--",
        state_file,
    ]) {
        Some(l) => l,
        None => return,
    };
    let mut commit = String::new();
    for line in log.lines() {
        if let Some(h) = line.strip_prefix("COMMIT ") {
            commit = h.to_string();
            continue;
        }
        let (iter, stage) = match added_stamp(line, stages) {
            Some(v) => v,
            None => continue,
        };
        let e = iters.entry(iter.to_string()).or_insert_with(|| {
            order.push(iter.to_string());
            Iteration::default()
        });
        if !e.stages.iter().any(|s| s == stage) {
            e.stages.push(stage.to_string());
        }
        if stage == CLOSE_STAGE {
            e.close = Some(commit.clone());
        }
    }
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — every range-scoped column freezes at
// (close(N-1), close(N)]; the first row's boundary is the empty one, so it takes every ancestor up
// to its own close commit.
fn harvest_ranges(
    git: &Git,
    order: &[String],
    iters: &mut HashMap<String, Iteration>,
) -> HashMap<String, String> {
    let mut owner: HashMap<String, String> = HashMap::new();
    let mut prev: Option<String> = None;
    for it in order {
        let close = match iters.get(it).and_then(|e| e.close.clone()) {
            Some(c) => c,
            None => continue,
        };
        let range = match &prev {
            Some(p) => format!("{}..{}", p, close),
            None => close.clone(),
        };
        let (mut feat, mut debt) = (0usize, 0usize);
        if let Some(log) = git.read(&["log", "--format=%H %s", &range]) {
            for line in log.lines() {
                let (h, subj) = match line.split_once(' ') {
                    Some(v) => v,
                    None => (line, ""),
                };
                owner.insert(h.to_string(), it.clone());
                if subj.starts_with("feat") {
                    feat += 1;
                } else if subj.starts_with("fix") || subj.starts_with("refactor") {
                    debt += 1;
                }
            }
        }
        if let Some(e) = iters.get_mut(it) {
            e.feat = feat;
            e.debt = debt;
        }
        prev = Some(close);
    }
    owner
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — amendment-latency harvest: the span
// from the commit that added an amendment to the one that deleted it, attributed to the closed
// iteration owning the deletion. Fixture and template paths are neither, so both are excluded.
fn harvest_amendments(
    git: &Git,
    owner: &HashMap<String, String>,
    iters: &mut HashMap<String, Iteration>,
) {
    let log = match git.read(&[
        "log",
        "--reverse",
        "--format=COMMIT %H %ad",
        "--date=short",
        "--diff-filter=AD",
        "--name-status",
        "--",
        "*/SPEC-*.md",
        "SPEC-*.md",
    ]) {
        Some(l) => l,
        None => return,
    };
    let mut added: HashMap<String, String> = HashMap::new();
    let mut commit = String::new();
    let mut date = String::new();
    for line in log.lines() {
        if let Some(rest) = line.strip_prefix("COMMIT ") {
            let mut f = rest.split_whitespace();
            commit = f.next().unwrap_or_default().to_string();
            date = f.next().unwrap_or_default().to_string();
            continue;
        }
        let (kind, path) = match line.split_once('\t') {
            Some(v) => v,
            None => continue,
        };
        if path.contains("/gate-tests/") || path.contains("/templates/") {
            continue;
        }
        match kind {
            "A" => {
                added.insert(path.to_string(), date.clone());
            }
            "D" => {
                let add_date = match added.get(path) {
                    Some(d) => d,
                    None => continue,
                };
                let it = match owner.get(&commit) {
                    Some(i) => i.clone(),
                    None => continue,
                };
                let (a, d) = match (days_from_civil(add_date), days_from_civil(&date)) {
                    (Some(a), Some(d)) => (a, d),
                    _ => continue,
                };
                if let Some(e) = iters.get_mut(&it) {
                    e.amendments += 1;
                    e.max_lag = e.max_lag.max(d - a);
                }
            }
            _ => {}
        }
    }
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — validate-attestation harvest: the
// evidence manifest truncates at scope too, so the union over every committed revision of it is
// what the whole history attested, deduplicated because a stamp survives across commits.
fn harvest_validate(git: &Git, evidence_file: &str, iters: &mut HashMap<String, Iteration>) {
    if evidence_file.is_empty()
        || !git.has(&["cat-file", "-e", &format!("HEAD:{}", evidence_file)])
    {
        return;
    }
    let revs = match git.read(&["log", "--format=%H", "--", evidence_file]) {
        Some(r) => r,
        None => return,
    };
    let mut seen: HashSet<String> = HashSet::new();
    for c in revs.split_whitespace() {
        let blob = match git.read(&["show", &format!("{}:{}", c, evidence_file)]) {
            Some(b) => b,
            None => continue,
        };
        for line in blob.lines() {
            if is_attestation(line) {
                seen.insert(line.to_string());
            }
        }
    }
    for stamp in seen {
        let mut f = stamp.splitn(3, ' ');
        let it = f.next().unwrap_or_default();
        let _suite = f.next();
        let rest = f.next().unwrap_or_default();
        if it.is_empty() {
            continue;
        }
        let e = iters.entry(it.to_string()).or_default();
        e.val_present = true;
        e.val_suites += 1;
        if rest.contains("fail=0 ") {
            continue;
        }
        if let Some(at) = rest.rfind("fail=") {
            let v = &rest[at + 5..];
            let v = v.split(' ').next().unwrap_or_default();
            if !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()) {
                e.val_fail += v.parse::<i64>().unwrap_or(0);
            }
        }
    }
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — `^[a-z0-9-]+ [a-z_]+ sha256=`: the
// attestation grammar, so a comment or a header line in the manifest is not counted as a suite
fn is_attestation(line: &str) -> bool {
    let (a, rest) = match line.split_once(' ') {
        Some(v) => v,
        None => return false,
    };
    if a.is_empty()
        || !a
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return false;
    }
    let (b, rest) = match rest.split_once(' ') {
        Some(v) => v,
        None => return false,
    };
    if b.is_empty() || !b.bytes().all(|c| c.is_ascii_lowercase() || c == b'_') {
        return false;
    }
    rest.starts_with("sha256=")
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — gate-roster growth: the registry's
// member count at the iteration's own close commit, so the column is history and not today's
fn harvest_gate_counts(git: &Git, gates_file: &str, iters: &mut HashMap<String, Iteration>) {
    for e in iters.values_mut() {
        let close = match &e.close {
            Some(c) => c.clone(),
            None => continue,
        };
        e.gates = match git.read(&["show", &format!("{}:{}", close, gates_file)]) {
            Some(roster) => roster
                .lines()
                .filter(|l| {
                    let t = l.trim_start_matches([' ', '\t']);
                    !t.is_empty() && !t.starts_with('#')
                })
                .count()
                .to_string(),
            None => "n/a".to_string(),
        };
    }
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — one slot per configured stage in
// roster order, present as its roster-unique abbreviation or absent as '·'
fn render_stages(seen: &[String], stages: &[String], abbr: &[String]) -> String {
    let mut out: Vec<&str> = Vec::new();
    for (i, s) in stages.iter().enumerate() {
        if seen.iter().any(|x| x == s) {
            out.push(&abbr[i]);
        } else {
            out.push("·");
        }
    }
    out.join(" ")
}

// spec: drift-kit/SPEC.md §The published-evidence extractor — the *bare* arm prints the committed
// projection and `--human` prepends the advisory header, inverting the shell tool's default: an
// arm returns the document, else the comparator compares against a banner.
pub fn emit(args: &[String]) -> Result<String, String> {
    let human = args.iter().any(|a| a == "--human");

    let surfaces = walk::knob_scalar(SURFACES_KNOB)?;
    let mut fields = surfaces.split_whitespace();
    let state_file = fields.next().unwrap_or_default().to_string();
    let evidence_file = fields.next().unwrap_or_default().to_string();
    let gates_file = walk::knob_scalar(GATES_KNOB)?;
    let stages = walk::knob_array(STAGES_KNOB)?;
    let abbr = abbreviations(&stages);
    let legend = abbr.join(" ");

    let git = Git { top: toplevel() };
    let mut order: Vec<String> = Vec::new();
    let mut iters: HashMap<String, Iteration> = HashMap::new();
    harvest_stamps(&git, &state_file, &stages, &mut order, &mut iters);
    let owner = harvest_ranges(&git, &order, &mut iters);
    harvest_amendments(&git, &owner, &mut iters);
    harvest_validate(&git, &evidence_file, &mut iters);
    harvest_gate_counts(&git, &gates_file, &mut iters);

    let mut out = String::new();
    if human {
        out.push_str("=== Governed trajectory (advisory — this repo's own committed history) ===\n");
        out.push_str(
            "One row per closed iteration; pure function of committed history (drift-kit/SPEC.md).\n",
        );
        out.push('\n');
    }
    out.push_str(&format!(
        "| iteration | stages ({}) | commits (feat/debt) | amendments (merged · max lag) | validate (suites) | gates.list |\n",
        legend
    ));
    out.push_str("| --- | --- | --- | --- | --- | --- |\n");

    if !git.has(&["cat-file", "-e", &format!("HEAD:{}", state_file)]) {
        out.push_str(&format!("| n/a (no {}) | · | · | · | · | · |\n", state_file));
        return Ok(out);
    }

    for it in &order {
        let e = match iters.get(it) {
            Some(e) if e.close.is_some() => e,
            _ => continue,
        };
        let amend = if e.amendments == 0 {
            "0".to_string()
        } else {
            format!("{} · ≤{}d", e.amendments, e.max_lag)
        };
        let val = if e.val_present {
            if e.val_fail == 0 {
                format!("{}s clean", e.val_suites)
            } else {
                format!("{}s {}✗", e.val_suites, e.val_fail)
            }
        } else {
            "n/a (pre-evidence-kit)".to_string()
        };
        out.push_str(&format!(
            "| {} | {} | {}f/{}d | {} | {} | {} |\n",
            it,
            render_stages(&e.stages, &stages, &abbr),
            e.feat,
            e.debt,
            amend,
            val,
            if e.gates.is_empty() { "n/a" } else { e.gates.as_str() }
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §The published-evidence extractor — a non-colliding roster reduces
    // every label to its single letter, and a colliding pair grows only until it separates
    #[test]
    fn a_slot_label_is_the_shortest_prefix_unique_among_the_roster() {
        let five: Vec<String> = ["scope", "align", "build", "validate", "close"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(abbreviations(&five), vec!["s", "a", "b", "v", "c"]);
        let six: Vec<String> = ["scope", "spec", "align", "build", "validate", "close"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(abbreviations(&six), vec!["sc", "sp", "a", "b", "v", "c"]);
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the day count the `date -d` subtraction stood
    // for, including the leap day the shell form's epoch arithmetic also crossed
    #[test]
    fn a_civil_date_converts_to_a_day_count_and_a_malformed_one_to_nothing() {
        let a = days_from_civil("2026-08-16").expect("a valid date");
        let b = days_from_civil("2026-08-18").expect("a valid date");
        assert_eq!(b - a, 2);
        assert_eq!(
            days_from_civil("2024-03-01").unwrap() - days_from_civil("2024-02-28").unwrap(),
            2,
            "2024 is a leap year"
        );
        assert_eq!(days_from_civil("1970-01-01"), Some(0));
        assert!(days_from_civil("not-a-date").is_none());
        assert!(days_from_civil("2026-13-01").is_none());
    }

    // spec: drift-kit/SPEC.md §The published-evidence extractor — the pre-filter the harvest ran
    // its diff through: a `+++ b/...` header and a stamp naming an unconfigured stage are both
    // outside the grammar, which is what keeps the harvest from reading diff furniture as history
    #[test]
    fn only_an_added_stamp_naming_a_configured_stage_is_harvested() {
        let stages: Vec<String> = ["scope", "build", "close"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            added_stamp("+my-iter build abc123 2026-01-01", &stages),
            Some(("my-iter", "build"))
        );
        assert!(added_stamp("+++ b/.workflow/WORKFLOW-STATE.txt", &stages).is_none());
        assert!(added_stamp("+my-iter deploy abc 2026-01-01", &stages).is_none());
        assert!(added_stamp("+my-iter buildx abc 2026-01-01", &stages).is_none());
        assert!(added_stamp("+My-Iter build abc 2026-01-01", &stages).is_none());
        assert!(added_stamp("-my-iter build abc 2026-01-01", &stages).is_none());
        assert!(added_stamp("+my-iter build", &stages).is_none());
    }

    // spec: drift-kit/SPEC.md §The published-evidence extractor — `^[a-z0-9-]+ [a-z_]+ sha256=`
    #[test]
    fn an_attestation_is_three_fields_the_third_a_sha256() {
        assert!(is_attestation("my-iter gate_tests sha256=abc fail=0 "));
        assert!(!is_attestation("# a comment line"));
        assert!(!is_attestation("my-iter gate_tests md5=abc"));
        assert!(!is_attestation("My-Iter gate_tests sha256=abc"));
    }
}
