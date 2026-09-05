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

#[cfg(test)]
mod tests {
    use super::added_from_log;

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
}
