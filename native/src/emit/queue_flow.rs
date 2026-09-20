// spec: drift-kit/SPEC.md §The queue-flow arm — the trailing per-iteration inflow and drain of the
// design-pending pool, `kpi-queue-net-delta`'s definitions read over consecutive iteration-start
// commits. Advisory, so every unmeasurable input is an `n/a (<reason>)` line and exit 0.
use super::kpi::queue_net_delta::{flow, pool};
use crate::history::{Blobs, Git};
use crate::stages;

pub const KNOBS: &[&str] = &[
    "DRIFT_KIT_STATE_FILE",
    "DRIFT_KIT_QUEUE_FILE",
    "DRIFT_KIT_DEFERRED_SECTION",
    "DRIFT_KIT_ICEBOX_SECTION",
];

const DEFAULT_WINDOWS: usize = 5;

fn usage() -> String {
    "usage: --emit queue-flow [<n>]   (<n> a positive integer, the trailing windows; default 5)"
        .to_string()
}

fn windows_arg(args: &[String]) -> Result<usize, String> {
    match args {
        [] => Ok(DEFAULT_WINDOWS),
        [n] => match n.parse::<usize>() {
            Ok(v) if v > 0 && n.bytes().all(|b| b.is_ascii_digit()) => Ok(v),
            _ => Err(usage()),
        },
        _ => Err(usage()),
    }
}

// spec: drift-kit/SPEC.md §The queue-flow arm — a git path is repo-relative, so a knob spelled
// absolute under the toplevel is re-rooted rather than handed to git as a path it cannot address.
fn repo_path(top: &str, path: &str) -> String {
    crate::walk::rel_under(top, path).unwrap_or(path).to_string()
}

// spec: drift-kit/SPEC.md §The queue-flow arm — the iteration starts, newest first, keyed on the
// head and labelled by the newest committed version carrying that head.
pub fn starts(versions: &[String], want: usize) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for text in versions {
        let head = stages::first_head(text);
        if head.is_empty() || out.iter().any(|(h, _)| *h == head) {
            continue;
        }
        let label = stages::data_lines(text)
            .last()
            .and_then(|l| l.split_whitespace().next())
            .unwrap_or("")
            .to_string();
        out.push((head, label));
        if out.len() == want {
            break;
        }
    }
    out
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let n = windows_arg(args)?;
    let knob = crate::walk::knob_scalar;
    let (state_file, queue_file) = (knob("DRIFT_KIT_STATE_FILE")?, knob("DRIFT_KIT_QUEUE_FILE")?);
    let (deferred, icebox) = (
        knob("DRIFT_KIT_DEFERRED_SECTION")?,
        knob("DRIFT_KIT_ICEBOX_SECTION")?,
    );
    let top = crate::walk::toplevel().or_else(|_| crate::walk::cwd())?;
    let git = Git { top: top.clone() };
    let (state_path, queue_path) = (repo_path(&top, &state_file), repo_path(&top, &queue_file));
    let na = |reason: String| Ok(format!("n/a ({})\n", reason));

    let revs = match git.read(&["log", "--format=%H", "--", &state_path]) {
        Some(l) if !l.trim().is_empty() => l,
        _ => return na(format!("no committed state file at {}", state_path)),
    };
    let mut blobs = match Blobs::open(&top) {
        Ok(b) => b,
        Err(e) => return na(e),
    };

    // spec: drift-kit/SPEC.md §The queue-flow arm — the scan stops at the n+1th start, so a long
    // history buys only the blobs of the windows it prints.
    let mut found: Vec<(String, String)> = Vec::new();
    for rev in revs.lines() {
        let text = match blobs.at(rev, &state_path)? {
            Some(t) => t,
            None => continue,
        };
        for s in starts(&[text], 1) {
            if !found.iter().any(|(h, _)| *h == s.0) {
                found.push(s);
            }
        }
        if found.len() == n + 1 {
            break;
        }
    }
    if found.len() < 2 {
        return na("fewer than two iteration-start commits in reach".to_string());
    }
    found.reverse();

    let mut out = String::new();
    let mut filed_sum = 0i64;
    for w in found.windows(2) {
        let ((from, label), (to, _)) = (&w[0], &w[1]);
        let mut pools = Vec::new();
        for head in [from, to] {
            if !stages::commit_resolves(head) {
                return na(format!("head {} does not resolve in this clone", head));
            }
            match blobs.at(head, &queue_path)? {
                Some(t) => pools.push(pool(&t, &deferred, &icebox)),
                None => return na(format!("queue absent at {}", head)),
            }
        }
        let (filed, drained) = flow(&pools[0], &pools[1]);
        filed_sum += filed;
        out.push_str(&format!("{} filed {} drained {}\n", label, filed, drained));
    }
    let rows = (found.len() - 1) as f64;
    out.push_str(&format!("mean-filed {:.1}\n", filed_sum as f64 / rows));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §The queue-flow arm — `<n>` is a positive integer defaulting to five;
    // anything else is a usage error rather than a silently substituted default
    #[test]
    fn the_window_count_is_a_positive_integer_defaulting_to_five() {
        assert_eq!(windows_arg(&[]), Ok(5));
        assert_eq!(windows_arg(&["3".to_string()]), Ok(3));
        assert!(windows_arg(&["0".to_string()]).is_err());
        assert!(windows_arg(&["+3".to_string()]).is_err());
        assert!(windows_arg(&["x".to_string()]).is_err());
        assert!(windows_arg(&["1".to_string(), "2".to_string()]).is_err());
    }

    // spec: drift-kit/SPEC.md §The queue-flow arm — keyed on the head, labelled by the newest
    // version's last stamp, so a rename inside an iteration labels the window by its final name
    #[test]
    fn a_start_is_keyed_on_its_head_and_labelled_by_the_newest_versions_last_stamp() {
        let hdr = "# h\n---\n\n";
        let versions = vec![
            format!("{}beta scope s3 2026-01-03 bbbbbbb\n", hdr),
            format!("{}renamed scope s1 2026-01-01 aaaaaaa\nrenamed build s2 2026-01-02 ccccccc\n", hdr),
            format!("{}alpha scope s1 2026-01-01 aaaaaaa\n", hdr),
            format!("{}older scope s0 2026-01-01 none\n", hdr),
        ];
        assert_eq!(
            starts(&versions, 9),
            vec![
                ("bbbbbbb".to_string(), "beta".to_string()),
                ("aaaaaaa".to_string(), "renamed".to_string())
            ]
        );
        assert_eq!(starts(&versions, 1).len(), 1);
    }
}
