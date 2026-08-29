// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-gate-backlog: proposed-but-absent gates over the live gate count
use super::{na, read, Ctx};
use std::path::Path;

const LABEL: &str = "gate backlog";

fn word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

// spec: drift-kit/SPEC.md §Bundled KPIs — grep's `\b(check|scan)-[a-z0-9]+(-[a-z0-9]+)*`: a
// word-boundary lead so a hyphenated neighbour still starts a name, and a greedy hyphen run that
// stops before a trailing separator.
pub fn proposed_names(text: &str) -> Vec<String> {
    let b = text.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let lead = ["check-", "scan-"]
            .iter()
            .find(|p| b[i..].starts_with(p.as_bytes()));
        let lead = match lead {
            Some(l) => *l,
            None => {
                i += 1;
                continue;
            }
        };
        if i > 0 && word_char(b[i - 1]) {
            i += 1;
            continue;
        }
        let mut j = i + lead.len();
        let run = |k: usize| k < b.len() && (b[k].is_ascii_lowercase() || b[k].is_ascii_digit());
        if !run(j) {
            i += 1;
            continue;
        }
        while run(j) {
            j += 1;
        }
        loop {
            if j < b.len() && b[j] == b'-' && run(j + 1) {
                j += 1;
                while run(j) {
                    j += 1;
                }
            } else {
                break;
            }
        }
        out.push(text[i..j].to_string());
        i = j;
    }
    out.sort();
    out.dedup();
    out
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.queue_file) {
        Some(t) => t,
        None => return na("lead", LABEL, "no queue file", trend),
    };

    let mut dirs: Vec<String> = vec![ctx.gates_dir.clone()];
    for k in &ctx.kit_roots {
        for sub in ["checks", "bin"] {
            let d = format!("{}/{}", k, sub);
            if Path::new(&d).is_dir() {
                dirs.push(d);
            }
        }
    }

    let unbuilt = proposed_names(&text)
        .iter()
        .filter(|n| {
            !dirs
                .iter()
                .any(|d| Path::new(&format!("{}/{}.sh", d, n)).is_file())
        })
        .count();

    let live = read(&format!("{}/gates.list", ctx.gates_dir))
        .map(|t| {
            t.lines()
                .filter(|l| {
                    let s = l.trim_start_matches([' ', '\t']);
                    !(s.is_empty() || s.starts_with('#'))
                })
                .count()
        })
        .unwrap_or(0);

    if trend {
        return Some(if unbuilt > 0 {
            format!("gate-backlog {}/{}\n", unbuilt, live)
        } else {
            String::new()
        });
    }
    Some(format!(
        "lead\t{}\t{} unbuilt / {} live gates\n",
        LABEL, unbuilt, live
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the scan is `grep -o` over the whole queue, so it
    // reads a name out of prose; the boundary rule is what keeps a suffix of a longer token out
    #[test]
    fn a_proposed_name_is_read_out_of_prose_and_a_word_lead_in_is_not_one() {
        let names = proposed_names("land check-foo-bar and scan-x, not xcheck-nope; `check-a1`.");
        assert_eq!(names, vec!["check-a1", "check-foo-bar", "scan-x"]);
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — the hyphen run is greedy but stops before a trailing
    // separator, so a name ending the sentence does not swallow the punctuation
    #[test]
    fn a_trailing_separator_is_not_part_of_the_name() {
        assert_eq!(proposed_names("check-foo- and check-b--c"), vec!["check-b", "check-foo"]);
        assert!(proposed_names("check- alone").is_empty());
    }
}
