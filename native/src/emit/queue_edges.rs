// spec: queue-kit/SPEC.md §The queue-edges arm — the inbound-citation aggregator. It reads the
// queue and its own history, writes stdout only, and mutates nothing. The `--inbound` mode rides
// the arm's own argv tail, the mechanism §The queue-index arm already uses for its three modes.
use crate::proc;
use crate::queue::{self, Sections};

struct Args {
    target: String,
    file: String,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut a = Args {
        target: String::new(),
        file: String::new(),
    };
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--inbound" => {
                a.target = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            other if other.starts_with('-') => {
                return Err(format!("unknown option: {}", other));
            }
            other => {
                a.file = other.to_string();
                i += 1;
            }
        }
    }
    Ok(a)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let a = parse(args)?;
    let file = if a.file.is_empty() {
        queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?
    } else {
        a.file.clone()
    };
    let text =
        std::fs::read_to_string(&file).map_err(|e| format!("file not found: {}: {}", file, e))?;
    let sec = Sections::active_and_deferred()?;
    let live = queue::live_slugs(&text, &sec);
    let retired = queue::retired_set(&file, &live);

    // spec: queue-kit/SPEC.md §The queue-edges arm — a slug that is neither live nor retired is a
    // caller error, not an empty set: silence has to mean "no inbound edges" and nothing else.
    if !a.target.is_empty() && !live.contains(&a.target) && !retired.contains(&a.target) {
        return Err(format!("not a live or retired slug: {}", a.target));
    }
    let stems = if retired.is_empty() {
        Vec::new()
    } else {
        tracked_stems(&file)
    };
    Ok(aggregate(&text, &sec, &live, &retired, &a.target, &stems))
}

// spec: queue-kit/SPEC.md §The queue-edges arm — the name-live test's input is the tracked tree
// listing and not a curated roster; an absent `git` or a queue file outside a work tree marks
// nothing, the same direction the retired set's own degradations take.
fn tracked_stems(file: &str) -> Vec<(String, String)> {
    if !proc::on_path("git") {
        return Vec::new();
    }
    let path = std::path::Path::new(file);
    let dir = match path.parent().map(|p| p.to_string_lossy().into_owned()) {
        Some(d) if !d.is_empty() => d,
        _ => ".".to_string(),
    };
    let listing = match proc::run("git", &["-C", &dir, "ls-files", "--full-name", "--", ":/"]) {
        Ok(c) => match c.stdout() {
            Some(o) => String::from_utf8_lossy(o).into_owned(),
            None => return Vec::new(),
        },
        Err(_) => return Vec::new(),
    };
    listing
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            std::path::Path::new(l)
                .file_stem()
                .map(|s| (s.to_string_lossy().into_owned(), l.to_string()))
        })
        .collect()
}

// spec: queue-kit/SPEC.md §The queue-edges arm — the first match in tracked order, with a further
// match noted as a count; a name that is no file's own stem is unmarked and its row reads as it did
fn name_live(stems: &[(String, String)], slug: &str) -> Option<String> {
    let hits: Vec<&String> = stems
        .iter()
        .filter(|(stem, _)| stem == slug)
        .map(|(_, path)| path)
        .collect();
    let first = hits.first()?;
    Some(match hits.len() {
        1 => format!(" — name live at {}", first),
        n => format!(" — name live at {} (+{})", first, n - 1),
    })
}

struct Agg<'a> {
    live: &'a [String],
    retired: &'a [String],
    want: &'a str,
    cur: String,
    order: Vec<String>,
    rorder: Vec<String>,
    recs: Vec<(String, Vec<String>)>,
}

impl Agg<'_> {
    // spec: queue-kit/SPEC.md §The tag algebra — resolution against the live slug set and the
    // retired one, self-citation dropped; a token in neither is silently not an edge, never a
    // complaint.
    fn edge(&mut self, tgt: &str, line: &str) {
        if self.cur.is_empty() || tgt == self.cur {
            return;
        }
        let is_retired = if self.live.iter().any(|s| s == tgt) {
            false
        } else if self.retired.iter().any(|s| s == tgt) {
            true
        } else {
            return;
        };
        if !self.want.is_empty() && tgt != self.want {
            return;
        }
        let rec = format!("  {:<46} {}", self.cur, line.trim());
        match self.recs.iter_mut().find(|(k, _)| k == tgt) {
            Some((_, v)) => v.push(rec),
            None => {
                if is_retired {
                    self.rorder.push(tgt.to_string());
                } else {
                    self.order.push(tgt.to_string());
                }
                self.recs.push((tgt.to_string(), vec![rec]));
            }
        }
    }

    fn scan_blocked(&mut self, line: &str) {
        for tgt in queue::blocked_by(line) {
            self.edge(tgt, line);
        }
    }

    // spec: queue-kit/SPEC.md §The queue-edges arm — the body-citation scan lives in this arm
    // rather than in the shared adapter: it has exactly one reader, and the library's rule is
    // shared adapters. A backticked kebab token is the candidate; resolution decides the rest.
    fn scan_body(&mut self, line: &str) {
        let b = line.as_bytes();
        let mut i = 0usize;
        while i < b.len() {
            if b[i] != b'`' {
                i += 1;
                continue;
            }
            let start = i + 1;
            if start >= b.len() || !(b[start].is_ascii_lowercase() || b[start].is_ascii_digit()) {
                i += 1;
                continue;
            }
            let mut j = start + 1;
            while j < b.len()
                && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-')
            {
                j += 1;
            }
            if j < b.len() && b[j] == b'`' {
                let tgt = line[start..j].to_string();
                self.edge(&tgt, line);
                i = j + 1;
            } else {
                i += 1;
            }
        }
    }
}

// spec: queue-kit/SPEC.md §The queue-edges arm — a citation is attributed to the nearest
// preceding slug bullet, so a sub-task cites in its own name; the lead line yields its
// `[blocked-by:]` tag alone, never its prose.
fn aggregate(
    text: &str,
    sec: &Sections,
    live: &[String],
    retired: &[String],
    want: &str,
    stems: &[(String, String)],
) -> String {
    let mut agg = Agg {
        live,
        retired,
        want,
        cur: String::new(),
        order: Vec::new(),
        rorder: Vec::new(),
        recs: Vec::new(),
    };
    let mut in_task = false;
    for line in text.lines() {
        if sec.is_task(line) {
            in_task = true;
            agg.cur.clear();
            continue;
        }
        if queue::is_section_line(line) {
            in_task = false;
            agg.cur.clear();
            continue;
        }
        if !in_task {
            continue;
        }
        if let Some(slug) = queue::bullet_slug(line) {
            agg.cur = slug.to_string();
            agg.scan_blocked(line);
            continue;
        }
        if !agg.cur.is_empty() {
            agg.scan_body(line);
        }
    }

    let mut out = String::new();
    for tgt in &agg.order {
        block(tgt, "", &agg.recs, &mut out);
    }
    // spec: queue-kit/SPEC.md §The queue-edges arm — retired targets sort alphabetically because
    // a retired slug has no queue position to order by.
    let mut rorder = agg.rorder.clone();
    rorder.sort();
    for tgt in &rorder {
        // spec: queue-kit/SPEC.md §The queue-edges arm — a retired slug and a retired name are
        // different questions: the mark adds a column of evidence, filters nothing, and rules
        // nothing, so the block's no-red posture is untouched.
        let suffix = format!(", retired{}", name_live(stems, tgt).unwrap_or_default());
        block(tgt, &suffix, &agg.recs, &mut out);
    }
    out
}

fn block(tgt: &str, suffix: &str, recs: &[(String, Vec<String>)], out: &mut String) {
    let Some((_, lines)) = recs.iter().find(|(k, _)| k == tgt) else {
        return;
    };
    out.push_str(&format!("{} ({} inbound{})\n", tgt, lines.len(), suffix));
    for l in lines {
        out.push_str(l);
        out.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const Q: &str = "\
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** — a feature citing `feat-b` on its lead line.
  **Relation to `feat-b`:** this one subsumes it entirely.
  It also mentions `landed-thing`, which is not a live slug, and `feat-a`.
  - **feat-a-sub** — a sub-task.
    It cites `def-a` in its own name.

- **feat-b** [blocked-by: def-a] — blocked, so it cites its blocker.

## Technical Debt

- **debt-a** — an entry nobody cites.
  It mentions `feat-b` once more.

## Deferred

- **def-a** — a deferred entry is a live target.

## Done

- done-slug
  It mentions `feat-b`, but Done is not a task section.

## Lessons Learned
";

    fn sec() -> Sections {
        Sections {
            active: vec!["New Features".into(), "Technical Debt".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
            done: String::new(),
        }
    }

    fn run(want: &str, retired: &[&str]) -> String {
        marked(want, retired, &[])
    }

    fn marked(want: &str, retired: &[&str], paths: &[&str]) -> String {
        let s = sec();
        let live = queue::live_slugs(Q, &s);
        let ret: Vec<String> = retired.iter().map(|r| r.to_string()).collect();
        let stems: Vec<(String, String)> = paths
            .iter()
            .filter_map(|p| {
                std::path::Path::new(p)
                    .file_stem()
                    .map(|st| (st.to_string_lossy().into_owned(), p.to_string()))
            })
            .collect();
        aggregate(Q, &s, &live, &ret, want, &stems)
    }

    // spec: queue-kit/SPEC.md §The queue-edges arm — the citation rules that section enumerates,
    // and the verbatim citing line beside them
    #[test]
    fn the_citation_grammar_resolves_counts_and_stays_silent_on_a_non_slug() {
        let out = run("", &[]);
        assert!(out.contains("feat-b (2 inbound)"), "{}", out);
        assert!(out.contains("def-a (2 inbound)"), "{}", out);
        assert!(!out.contains("landed-thing ("), "{}", out);
        assert!(!out.contains("feat-a ("), "{}", out);
        assert!(!out.contains("done-slug ("), "{}", out);
        assert!(
            out.contains("**Relation to `feat-b`:** this one subsumes it entirely."),
            "the citing line is not carried verbatim: {}",
            out
        );
    }

    // spec: queue-kit/SPEC.md §The queue-edges arm — attribution is to the nearest preceding slug
    // bullet, and a lead line yields its `[blocked-by:]` tag alone rather than its prose.
    #[test]
    fn a_sub_task_cites_in_its_own_name_and_a_lead_line_yields_only_its_tag() {
        let out = run("def-a", &[]);
        assert!(out.contains("feat-a-sub"), "{}", out);
        assert!(out.contains("[blocked-by: def-a]"), "{}", out);
        assert!(!out.contains("  feat-a  "), "the parent was credited: {}", out);
        let out = run("feat-b", &[]);
        assert!(out.contains("subsumes it entirely"), "{}", out);
        assert!(out.contains("debt-a"), "{}", out);
        assert!(
            !out.contains("a feature citing"),
            "the lead line's prose became an edge: {}",
            out
        );
        assert_eq!(run("debt-a", &[]), "", "a live slug with no inbound edges");
    }

    // spec: queue-kit/SPEC.md §The queue-edges arm — a retired target is an edge like any other,
    // marked as such, printed after the live block and sorted alphabetically within it.
    #[test]
    fn retired_targets_trail_the_live_block_alphabetically() {
        let out = run("", &["landed-thing", "a-retired-one"]);
        assert!(out.contains("feat-b (2 inbound)"), "{}", out);
        assert!(out.contains("landed-thing (1 inbound, retired)"), "{}", out);
        let live_at = out.find("feat-b (2 inbound)").expect("no live block");
        let ret_at = out.find("landed-thing (1 inbound").expect("no retired block");
        assert!(ret_at > live_at, "the retired block does not trail: {}", out);
        // comment-tier-exempt: names this fixture's second retired slug, which is cited by
        // nothing and so never reaches the ordering — a property of the fixture, not the rule
        assert!(!out.contains("a-retired-one ("), "{}", out);
    }

    // spec: queue-kit/SPEC.md §The queue-edges arm — a retired target whose slug is a tracked
    // file's own stem is marked with that path, a second match noted as a count; a name that is no
    // file's stem is unmarked and its row reads byte-for-byte as it did.
    #[test]
    fn a_retired_target_whose_name_is_a_tracked_file_stem_is_marked() {
        let out = marked("", &["landed-thing"], &["a/landed-thing.sh"]);
        assert!(
            out.contains("landed-thing (1 inbound, retired — name live at a/landed-thing.sh)"),
            "{}",
            out
        );
        let out = marked(
            "",
            &["landed-thing"],
            &["a/landed-thing.sh", "b/landed-thing", "c/landed-thing.md"],
        );
        assert!(
            out.contains("landed-thing (1 inbound, retired — name live at a/landed-thing.sh (+2))"),
            "{}",
            out
        );
        let out = marked("", &["landed-thing"], &["a/other.sh", "b/landed-thing.d/x"]);
        assert!(out.contains("landed-thing (1 inbound, retired)"), "{}", out);
    }

    // spec: queue-kit/SPEC.md §The queue-edges arm — the argv tail is `[--inbound <slug>]
    // [queue-file]` and nothing else, so an unknown option is a refusal rather than a path.
    #[test]
    fn the_argv_tail_is_the_mode_flag_and_the_file() {
        let a = parse(&["--inbound".into(), "x".into(), "q.md".into()]).expect("parse failed");
        assert_eq!((a.target.as_str(), a.file.as_str()), ("x", "q.md"));
        let a = parse(&["--inbound".into()]).expect("parse failed");
        assert_eq!((a.target.as_str(), a.file.as_str()), ("", ""));
        assert!(parse(&["-h".into()]).is_err());
    }
}
