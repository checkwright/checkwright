// spec: drift-kit/SPEC.md §The stage-economics meter — the committed-history read, given one owner.
// What must not have two producers is the git output *shape* — the `-p -U0` diff form and the
// `+`-prefix convention — because a change there breaks every reader silently and in the same way.
// spec: gate-sdk/SPEC.md §The port-candidate criteria — the field parse stays per-reader and is
// deliberately not folded in: one consumer filters against a configured stage roster and the other
// must not acquire that dependency, since a stamp outside the roster still carries real spend.
use crate::proc;

pub struct Git {
    pub top: String,
}

impl Git {
    // spec: gate-sdk/SPEC.md §Fail-closed contract — a git read that did not succeed yields no
    // stdout, and every caller here treats that as "this history is not there", which is each
    // reader's own posture: both are advisory and exit 0 whatever git says.
    pub fn read(&self, args: &[&str]) -> Option<String> {
        let mut argv: Vec<&str> = vec!["-C", &self.top];
        argv.extend_from_slice(args);
        proc::run("git", &argv)
            .ok()
            .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
    }

    pub fn has(&self, args: &[&str]) -> bool {
        self.read(args).is_some()
    }
}

// spec: drift-kit/SPEC.md §The stage-economics meter — one path's whole committed history as
// `(commit, added_line)` pairs, the added line still carrying its `+` so a reader's own grammar
// binds on it.
pub fn added_lines(git: &Git, path: &str) -> Vec<(String, String)> {
    match git.read(&["log", "--reverse", "--format=COMMIT %H", "-p", "-U0", "--", path]) {
        Some(log) => added_from_log(&log),
        None => Vec::new(),
    }
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the diff form's own reading, split out from
// the spawn so it is a pure function of the log text. The `+++ b/…` file header is excluded here
// rather than in each reader's parse: it is a property of the form this module owns.
pub fn added_from_log(log: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut commit = String::new();
    for line in log.lines() {
        if let Some(h) = line.strip_prefix("COMMIT ") {
            commit = h.to_string();
            continue;
        }
        if !line.starts_with('+') || line.starts_with("+++") {
            continue;
        }
        out.push((commit.clone(), line.to_string()));
    }
    out
}

// spec: drift-kit/SPEC.md §The stage-economics meter — the stamp grammar the committed-history arm
// filters added lines by: `<iteration> <stage> <session8> <YYYY-MM-DD>` with exactly one space
// between fields, which is what keeps a diff header and a comment line out of the union.
pub fn history_stamp(added: &str) -> Option<&str> {
    let line = added.strip_prefix('+')?;
    let f: Vec<&str> = line.splitn(5, ' ').collect();
    if f.len() < 4 {
        return None;
    }
    let kebab = |s: &str, head: fn(u8) -> bool| {
        !s.is_empty()
            && head(s.as_bytes()[0])
            && s.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    };
    if !kebab(f[0], |b| b.is_ascii_lowercase() || b.is_ascii_digit())
        || !kebab(f[1], |b| b.is_ascii_lowercase())
        || f[2].is_empty()
        || !f[2].bytes().all(|b| b.is_ascii_alphanumeric())
    {
        return None;
    }
    let d = f[3].as_bytes();
    if d.len() < 10 || d[4] != b'-' || d[7] != b'-' {
        return None;
    }
    if ![0, 1, 2, 3, 5, 6, 8, 9]
        .iter()
        .all(|i| d[*i].is_ascii_digit())
    {
        return None;
    }
    Some(line)
}

// spec: drift-kit/SPEC.md §The stage-economics meter — history ∪ live, so the boundary truncation
// of the live file destroys no economics and a stamped-but-uncommitted stage stays visible; the
// 0-exit *nothing to read* notice fires only when **both** sources yield no stamps.
// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-stage-economics-lag reads this same union in this same
// order (history in commit order, then the live file), so the meter and the KPI share one producer.
pub fn stamp_lines(state_file: &str) -> Vec<String> {
    let top = crate::walk::toplevel()
        .or_else(|_| crate::walk::cwd())
        .unwrap_or_else(|_| ".".to_string());
    let git = Git { top };
    let mut out: Vec<String> = added_lines(&git, state_file)
        .into_iter()
        .filter_map(|(_, l)| history_stamp(&l).map(str::to_string))
        .collect();
    if let Ok(b) = std::fs::read(state_file) {
        out.extend(String::from_utf8_lossy(&b).lines().map(str::to_string));
    }
    out
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — one `git cat-file --batch` child driven
// request by request, so a walk that stops at an entry's filing commit never buys the blobs of the
// commits older than it
pub struct Blobs {
    p: crate::proc::Piped,
}

impl Blobs {
    pub fn open(top: &str) -> Result<Self, String> {
        Ok(Blobs {
            p: crate::proc::piped("git", &["-C", top, "cat-file", "--batch"])?,
        })
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — a commit that does not carry the path
    // answers `None`, which is the same answer as a commit carrying no entry for the slug: both
    // mean "the walk has run past what it was measuring".
    pub fn at(&mut self, rev: &str, path: &str) -> Result<Option<String>, String> {
        self.p.ask(&format!("{}:{}", rev, path))?;
        let header = self.p.read_line()?;
        let size = match header.split_whitespace().nth(2) {
            Some(n) => n,
            None => return Ok(None),
        };
        let size: usize = size
            .parse()
            .map_err(|_| format!("git cat-file --batch: unreadable record header: {}", header.trim_end()))?;
        let body = self.p.read_exact(size)?;
        self.p.read_exact(1)?;
        Ok(Some(String::from_utf8_lossy(&body).into_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{added_from_log, history_stamp};

    // spec: drift-kit/SPEC.md §The stage-economics meter — the shared shape's own assertion, the
    // one both readers depend on: the `+++ b/…` header never enters the added-line stream, and a
    // line is attributed to the commit whose `COMMIT` marker most recently preceded it.
    #[test]
    fn the_file_header_stays_out_of_the_added_line_stream() {
        let pairs = added_from_log(
            "COMMIT aaa\n--- /dev/null\n+++ b/x.txt\n+first line\nCOMMIT bbb\n+++ b/x.txt\n\
             +second line\n-removed\n@@ -0,0 +1 @@\n",
        );
        assert_eq!(
            pairs,
            vec![
                ("aaa".to_string(), "+first line".to_string()),
                ("bbb".to_string(), "+second line".to_string()),
            ]
        );
    }

    // spec: drift-kit/SPEC.md §The stage-economics meter — the stamp grammar the history arm
    // filters by: exactly one space between fields, a kebab iteration, a lowercase-led stage, an
    // alphanumeric session8 and an ISO date, which is what keeps a diff header out of the union.
    #[test]
    fn the_history_grammar_admits_a_stamp_and_refuses_a_header() {
        assert_eq!(
            history_stamp("+alpha build s2 2025-01-01 abc123"),
            Some("alpha build s2 2025-01-01 abc123")
        );
        assert_eq!(history_stamp("+++ b/.workflow/WORKFLOW-STATE.txt"), None);
        assert_eq!(history_stamp("+# a comment line"), None);
        assert_eq!(history_stamp("+---"), None);
        assert_eq!(history_stamp("+alpha  build s2 2025-01-01"), None, "two spaces");
        assert_eq!(history_stamp("+Alpha build s2 2025-01-01"), None, "uppercase iteration");
        assert_eq!(history_stamp("+alpha build s2 2025-1-1"), None, "short date");
        assert_eq!(history_stamp("alpha build s2 2025-01-01"), None, "no + prefix");
    }
}
