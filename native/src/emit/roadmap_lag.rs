// spec: queue-kit/SPEC.md §The roadmap-lag arm — the entries outside the first horizon, or untagged,
// under which a split slice landed: a report with no verdict and no exit-1 path
use crate::programs;
use crate::queue::{self, Sections};
use crate::walk;

pub const USAGE: &str = "\
usage: --emit roadmap-lag [queue-file]
  one row per top-level entry outside the icebox, in queue order, whose
  [roadmap:] horizon is not the first QUEUE_KIT_HORIZONS member or which carries
  no [roadmap:] tag, and which has a slice in the done section:
  \"<slug>\\t<horizon or ->\\t<slice>[,<slice>...]\". No row means nothing lags.
";

pub const KNOBS: &[&str] = &[
    "QUEUE_KIT_QUEUE_FILE",
    "QUEUE_KIT_ACTIVE_SECTIONS",
    "QUEUE_KIT_DEFERRED_SECTION",
    "QUEUE_KIT_ICEBOX_SECTION",
    "QUEUE_KIT_DONE_SECTION",
    "QUEUE_KIT_HORIZONS",
];

// spec: queue-kit/SPEC.md §The roadmap-lag arm — a row-eligible entry and the done slugs its body
// cites, the parent side of the citation pair, read before any history is walked
#[derive(Debug, PartialEq)]
struct Candidate {
    slug: String,
    horizon: String,
    cited: Vec<String>,
}

fn horizon_of(tags: &str) -> Option<String> {
    let t = queue::field_tags(tags, "roadmap");
    let raw = t.first()?.raw.trim();
    Some(raw.split_once('/').map_or(raw, |(h, _)| h).to_string())
}

fn cites(line: &str, slug: &str) -> bool {
    queue::backtick_slugs(line)
        .into_iter()
        .chain(queue::link_slugs(line))
        .any(|(a, b)| &line[a..b] == slug)
}

fn candidates(text: &str, sec: &Sections, first: &str) -> Vec<Candidate> {
    let lines: Vec<&str> = text.lines().collect();
    let done = queue::done_slugs(text, sec);
    queue::entries(&lines, sec)
        .into_iter()
        .filter(|e| e.level == 3 && !sec.is_icebox(&e.section))
        .filter_map(|e| {
            let horizon = horizon_of(e.tags(&lines));
            if horizon.as_deref() == Some(first) {
                return None;
            }
            let cited: Vec<String> = done
                .iter()
                .filter(|x| e.body_lines().any(|i| cites(lines[i], x)))
                .cloned()
                .collect();
            (!cited.is_empty()).then(|| Candidate {
                slug: e.slug.clone(),
                horizon: horizon.unwrap_or_else(|| "-".to_string()),
                cited,
            })
        })
        .collect()
}

// spec: queue-kit/SPEC.md §The roadmap-lag arm — the child side: the slugs a revision's entry headed
// `slug` links, or `None` where no entry in that revision is headed by it
fn links_of(rev_text: &str, sec: &Sections, slug: &str) -> Option<Vec<String>> {
    let lines: Vec<&str> = rev_text.lines().collect();
    let e = queue::entries(&lines, sec).into_iter().find(|e| e.slug == slug)?;
    let mut out = Vec::new();
    for line in &lines[e.start + 1..e.end] {
        out.extend(queue::link_slugs(line).into_iter().map(|(a, b)| line[a..b].to_string()));
    }
    Some(out)
}

fn rows(cands: &[Candidate], last_live: &[(String, Vec<String>)]) -> String {
    let mut out = String::new();
    for c in cands {
        let slices: Vec<&str> = c
            .cited
            .iter()
            .filter(|x| last_live.iter().any(|(s, links)| s == *x && links.contains(&c.slug)))
            .map(String::as_str)
            .collect();
        if !slices.is_empty() {
            out.push_str(&format!("{}\t{}\t{}\n", c.slug, c.horizon, slices.join(",")));
        }
    }
    out
}

// spec: queue-kit/SPEC.md §The roadmap-lag arm — the degradations: the work tree is the queue file's
// own, and where none answers no slice is confirmed, said once on stderr
fn locate(file: &str) -> Result<(String, String), String> {
    if !crate::proc::on_path(&programs::GIT) {
        return Err("git is not on PATH".to_string());
    }
    let abs = walk::abs_against(&walk::cwd()?, &walk::cross_arg(file));
    let dir = std::path::Path::new(&abs).parent().map(|d| d.to_string_lossy().into_owned()).unwrap_or_default();
    let top = walk::toplevel_in(&dir).map_err(|_| format!("{} is in no git work tree", file))?;
    let rel = walk::rel_under(&top, &abs)
        .ok_or_else(|| format!("{} is outside its work tree {}", file, top))?
        .to_string();
    Ok((top, rel))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let mut file = String::new();
    for a in args {
        match a.as_str() {
            other if other.starts_with('-') => return Err(format!("unknown option: {}\n{}", other, USAGE)),
            other if file.is_empty() => file = other.to_string(),
            other => return Err(format!("unexpected argument: {}\n{}", other, USAGE)),
        }
    }
    let file = if file.is_empty() { queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")? } else { file };
    let text = std::fs::read_to_string(&file).map_err(|_| format!("queue file not found: {}", file))?;
    let horizons = queue::knob_array("QUEUE_KIT_HORIZONS")?;
    let Some(first) = horizons.first() else {
        return Err("QUEUE_KIT_HORIZONS is empty — no roadmap vocabulary is configured".to_string());
    };
    let sec = Sections::with_done()?;
    let cands = candidates(&text, &sec, first);
    if cands.is_empty() {
        return Ok(String::new());
    }
    let (top, rel) = match locate(&file) {
        Ok(v) => v,
        Err(why) => {
            eprintln!("roadmap-lag: no slice can be confirmed: {}", why);
            return Ok(String::new());
        }
    };
    let mut wanted: Vec<String> = cands.iter().flat_map(|c| c.cited.iter().cloned()).collect();
    wanted.sort();
    wanted.dedup();
    let mut last_live: Vec<(String, Vec<String>)> = Vec::new();
    queue::walk_history(&top, &rel, &sec, &mut |rev| {
        if let Some(t) = rev.text.as_deref() {
            wanted.retain(|x| match links_of(t, &sec, x) {
                Some(links) => {
                    last_live.push((x.clone(), links));
                    false
                }
                None => true,
            });
        }
        !wanted.is_empty()
    })?;
    Ok(rows(&cands, &last_live))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sec() -> Sections {
        Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: "Icebox".into(),
            done: "Done".into(),
        }
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — `--help` is the member's shape refusal carrying the usage
    #[test]
    fn help_and_a_second_operand_are_refusals_carrying_the_usage() {
        for argv in [vec!["--help".to_string()], vec!["a".into(), "b".into()]] {
            let err = emit(&argv).expect_err("a usage error must refuse");
            assert!(err.contains(USAGE), "{}", err);
        }
    }

    const NOW: &str = "\
## New Features

### parent-later

[roadmap: later/core]

Body citing `child-a` and `child-b`.

### parent-now

[roadmap: now/core]

Body citing `child-c`.

#### sub-cites

Cites `child-a` too.

### parent-untagged

Cites `child-d`.

## Deferred

## Icebox

### iced

Cites `child-a`.

## Done

- child-a
- child-b
- child-c
- child-d
";

    // spec: queue-kit/SPEC.md §The roadmap-lag arm — the row rule: a top-level entry outside the icebox
    // whose horizon is not the first, or untagged, carrying the done slugs its body cites
    #[test]
    fn candidates_are_top_level_non_icebox_entries_off_the_first_horizon() {
        let c = candidates(NOW, &sec(), "now");
        let got: Vec<(&str, &str, Vec<&str>)> = c
            .iter()
            .map(|c| (c.slug.as_str(), c.horizon.as_str(), c.cited.iter().map(String::as_str).collect()))
            .collect();
        assert_eq!(
            got,
            vec![
                ("parent-later", "later", vec!["child-a", "child-b"]),
                ("parent-untagged", "-", vec!["child-d"]),
            ]
        );
    }

    // spec: queue-kit/SPEC.md §The roadmap-lag arm — the slice rule: the child's last live entry
    // links the parent, and a revision where the child heads nothing yields no link set
    #[test]
    fn a_slice_is_a_cited_done_slug_whose_last_live_entry_linked_the_parent() {
        let then = "## New Features\n\n### child-a\n\nSplit from [parent-later](#parent-later).\n\n\
                    ### child-b\n\nUnrelated, cites [other](#other).\n";
        assert_eq!(links_of(then, &sec(), "child-a"), Some(vec!["parent-later".to_string()]));
        assert_eq!(links_of(then, &sec(), "child-z"), None);
        let cands = candidates(NOW, &sec(), "now");
        let last = vec![
            ("child-a".to_string(), links_of(then, &sec(), "child-a").unwrap()),
            ("child-b".to_string(), links_of(then, &sec(), "child-b").unwrap()),
            ("child-d".to_string(), vec!["parent-untagged".to_string()]),
        ];
        assert_eq!(rows(&cands, &last), "parent-later\tlater\tchild-a\nparent-untagged\t-\tchild-d\n");
        assert_eq!(rows(&cands, &[]), "");
    }
}
