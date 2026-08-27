// spec: gate-sdk/SPEC.md §check-action-gh-repo — the job-partitioned walk that section's rule
// describes, held here as shared gate mechanism rather than in one gate's module: the valve
// marker's spelling is the caller's parameter, everything else is the rule's and is shared

// spec: gate-sdk/SPEC.md §check-action-gh-repo — the job-partitioned walk emits these and
// the arms consume them; a typed roster is what keeps the emitter and the reader from
// disagreeing about a stream neither of them owns alone.
pub enum Ev {
    Job(String, usize),
    JobEnv,
    JobExempt,
    Step,
    StepEnv,
    StepExempt,
    Checkout(usize),
    Gh(usize, bool),
    WorkflowEnv,
    BareMarker(usize),
}

#[derive(PartialEq, Clone, Copy)]
enum EnvScope {
    None,
    Workflow,
    Job,
    Step,
}

fn is_blank(s: &str) -> bool {
    s.bytes().all(|b| b == b' ' || b == b'\t')
}

// spec: gate-sdk/SPEC.md §check-action-gh-repo — indentation is the structural key the
// job/step partition reads; a line with no non-space character has none.
fn ind(s: &str) -> i64 {
    match s.bytes().position(|b| b != b' ') {
        Some(n) => n as i64,
        None => -1,
    }
}

// spec: gate-sdk/SPEC.md §check-action-gh-repo — the mapping key is what tells `steps:`,
// `env:`, `run:` and `uses:` apart, so a token carrying an interior space is no key.
fn keyof(s: &str) -> Option<&str> {
    let t = s.trim_start_matches(' ');
    let colon = t.find(':')?;
    let name = t[..colon].trim_end_matches(' ');
    if name.is_empty() || name.contains(' ') {
        return None;
    }
    Some(name)
}

fn substr_from(s: &str, start: usize) -> &str {
    let mut i = start.min(s.len());
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    &s[i..]
}

// spec: gate-sdk/SPEC.md §check-action-gh-repo — a new step is a dash at the step-list
// column alone, so the dash prefix is measured rather than merely detected.
fn dash_prefix_len(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() && b[i] == b' ' {
        i += 1;
    }
    if i >= b.len() || b[i] != b'-' {
        return None;
    }
    i += 1;
    let mut spaces = 0;
    while i < b.len() && b[i] == b' ' {
        i += 1;
        spaces += 1;
    }
    if spaces == 0 {
        return None;
    }
    Some(i)
}

fn is_gh_repo_key(line: &str) -> bool {
    match line.trim_start_matches(' ').strip_prefix("GH_REPO") {
        Some(r) => r.trim_start_matches(' ').starts_with(':'),
        None => false,
    }
}

fn is_block_scalar(v: &str) -> bool {
    let b = v.as_bytes();
    if b.is_empty() || (b[0] != b'|' && b[0] != b'>') {
        return false;
    }
    let mut i = 1;
    if i < b.len() && (b[i] == b'-' || b[i] == b'+') {
        i += 1;
    }
    b[i..].iter().all(u8::is_ascii_digit)
}

// spec: gate-sdk/SPEC.md §check-action-gh-repo — a detected call's `--repo` is looked for
// only within that call's own extent, so a second call cannot lend the first one its flag.
fn has_repo(ext: &str) -> bool {
    let b = ext.as_bytes();
    let mut from = 0usize;
    while let Some(p) = ext[from..].find("--repo") {
        let start = from + p;
        let end = start + "--repo".len();
        let before = start == 0 || b[start - 1] == b' ' || b[start - 1] == b'\t';
        let after = end == b.len() || matches!(b[end], b' ' | b'\t' | b'=');
        if before && after {
            return true;
        }
        from = start + 1;
    }
    false
}

// spec: gate-sdk/SPEC.md §check-action-gh-repo — the reason is required and non-empty; the
// last spelling on the line owns it, so a marker naming another one still yields its own.
fn marker_reason<'a>(line: &'a str, mark: &str) -> Option<&'a str> {
    let mut best: Option<usize> = None;
    let mut from = 0usize;
    while let Some(p) = line[from..].find(mark) {
        let start = from + p;
        let after = line[start + mark.len()..].trim_start_matches([' ', '\t']);
        if after.starts_with(':') {
            best = Some(line.len() - after.len() + 1);
        }
        from = start + 1;
    }
    let idx = best?;
    Some(
        line[idx..]
            .trim_start_matches([' ', '\t'])
            .trim_end_matches([' ', '\t']),
    )
}

pub struct Walk<'a> {
    mark: &'a str,
    jobcol: i64,
    jobkeycol: i64,
    stepcol: i64,
    stepdashcol: i64,
    envscope: EnvScope,
    envcol: i64,
    curjob: String,
    pendjob: bool,
    pendstep: bool,
    inrun: bool,
    injobs: bool,
    insteps: bool,
    runkeycol: i64,
    rbi: i64,
    rbuf: Vec<(String, usize)>,
    out: Vec<Ev>,
}

impl<'a> Walk<'a> {
    // spec: gate-sdk/SPEC.md §check-action-gh-repo — the valve's marker token is the caller's,
    // so a second consumer reuses the indentation binding and the required-reason rule under
    // its own spelling rather than copying the state machine to rename one string.
    fn new(mark: &'a str) -> Self {
        Walk {
            mark,
            jobcol: -1,
            jobkeycol: -1,
            stepcol: -1,
            stepdashcol: -1,
            envscope: EnvScope::None,
            envcol: 0,
            curjob: String::new(),
            pendjob: false,
            pendstep: false,
            inrun: false,
            injobs: false,
            insteps: false,
            runkeycol: -1,
            rbi: -1,
            rbuf: Vec::new(),
            out: Vec::new(),
        }
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — a `gh` token counts only where a shell
    // would read it as a command, and whole-word matching keeps `ghost` and `gh-pages` out.
    fn scanlogical(&mut self, s: &str, ln: usize) {
        if s.trim_start_matches([' ', '\t']).starts_with('#') {
            return;
        }
        let b = s.as_bytes();
        if b.len() < 2 {
            return;
        }
        for idx in 0..b.len() - 1 {
            if b[idx] != b'g' || b[idx + 1] != b'h' {
                continue;
            }
            let joined_word = matches!(b.get(idx + 2), Some(&c)
                if c.is_ascii_alphanumeric() || c == b'_' || c == b'.' || c == b'-');
            if joined_word {
                continue;
            }
            let prefix = s[..idx].trim_end_matches([' ', '\t']);
            let ok = if prefix.is_empty() {
                true
            } else {
                let last = prefix.as_bytes()[prefix.len() - 1];
                if b"|&;(`!{".contains(&last) {
                    true
                } else {
                    let w = match prefix.rfind([' ', '\t']) {
                        Some(p) => &prefix[p + 1..],
                        None => prefix,
                    };
                    matches!(w, "then" | "else" | "do" | "elif")
                }
            };
            if !ok {
                continue;
            }
            let ext = &s[idx + 2..];
            let cut = ext.find([';', '|', '&']).unwrap_or(ext.len());
            self.out.push(Ev::Gh(ln, has_repo(&ext[..cut])));
        }
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — backslash continuations are joined
    // before matching, so a call split across lines is one unit and its `--repo` is found
    // wherever on the call it sits.
    fn endrun(&mut self) {
        self.inrun = false;
        let buf = std::mem::take(&mut self.rbuf);
        let mut acc = String::new();
        let mut accln = 0usize;
        for (s, ln) in buf {
            if acc.is_empty() {
                accln = ln;
            }
            if let Some(head) = s.strip_suffix('\\') {
                acc.push_str(head);
                acc.push(' ');
                continue;
            }
            acc.push_str(&s);
            let joined = std::mem::take(&mut acc);
            self.scanlogical(&joined, accln);
        }
        if !acc.is_empty() {
            let tail = std::mem::take(&mut acc);
            self.scanlogical(&tail, accln);
        }
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — a folded body is scanned line-wise like
    // a literal one, and a plain scalar as a single logical line: both over-detect, the
    // stated safe direction.
    fn stepkey(&mut self, rest: &str, col: i64, ln: usize) {
        let k = match keyof(rest) {
            Some(k) => k.to_string(),
            None => return,
        };
        let after = match rest.find(':') {
            Some(i) => &rest[i + 1..],
            None => rest,
        };
        let v = after
            .trim_start_matches([' ', '\t'])
            .trim_end_matches([' ', '\t']);
        if k == "uses" {
            let q = v.trim_start_matches(['"', '\'']).trim_end_matches(['"', '\'']);
            let q = match q.find([' ', '\t']) {
                Some(i) => &q[..i],
                None => q,
            };
            let u = match q.find('@') {
                Some(i) => &q[..i],
                None => q,
            };
            if u == "actions/checkout" {
                self.out.push(Ev::Checkout(ln));
            }
            return;
        }
        if k == "env" {
            self.envscope = EnvScope::Step;
            self.envcol = col;
            return;
        }
        if k != "run" {
            return;
        }
        if is_block_scalar(v) {
            self.inrun = true;
            self.runkeycol = col;
            self.rbi = -1;
            self.rbuf.clear();
            return;
        }
        if !v.is_empty() {
            self.scanlogical(v, ln);
        }
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — the marker binds by its own
    // indentation: at or left of the job-id column it precedes a job, at the step dash
    // column it precedes a step, inside a step it binds that step.
    fn marker(&mut self, line: &str, ln: usize) {
        if !line.contains(self.mark) {
            return;
        }
        match marker_reason(line, self.mark) {
            Some(r) if !r.is_empty() => {}
            _ => {
                self.out.push(Ev::BareMarker(ln));
                return;
            }
        }
        if !self.injobs {
            return;
        }
        let c = ind(line);
        if self.jobcol >= 0 && c <= self.jobcol {
            self.pendjob = true;
            return;
        }
        if self.curjob.is_empty() {
            self.pendjob = true;
            return;
        }
        if self.insteps {
            if self.stepdashcol < 0 || c <= self.stepdashcol {
                self.pendstep = true;
            } else if self.stepcol >= 0 {
                self.out.push(Ev::StepExempt);
            } else {
                self.pendstep = true;
            }
            return;
        }
        self.out.push(Ev::JobExempt);
    }

    fn line(&mut self, raw: &str, fnr: usize) {
        let line = raw.strip_suffix('\r').unwrap_or(raw);

        if self.inrun {
            if is_blank(line) {
                self.rbuf.push((String::new(), fnr));
                return;
            }
            let c = ind(line);
            if c > self.runkeycol {
                if self.rbi < 0 {
                    self.rbi = c;
                }
                let body = substr_from(line, self.rbi as usize).to_string();
                self.rbuf.push((body, fnr));
                return;
            }
            self.endrun();
        }

        if is_blank(line) {
            return;
        }
        if line.trim_start_matches(' ').starts_with('#') {
            self.marker(line, fnr);
            return;
        }

        let c = ind(line);

        if self.envscope != EnvScope::None {
            if c > self.envcol {
                if is_gh_repo_key(line) {
                    self.out.push(match self.envscope {
                        EnvScope::Workflow => Ev::WorkflowEnv,
                        EnvScope::Job => Ev::JobEnv,
                        _ => Ev::StepEnv,
                    });
                }
                return;
            }
            self.envscope = EnvScope::None;
        }

        if c == 0 {
            let topkey = keyof(line).unwrap_or("");
            self.injobs = topkey == "jobs";
            self.curjob.clear();
            self.stepcol = -1;
            self.insteps = false;
            if topkey == "env" {
                self.envscope = EnvScope::Workflow;
                self.envcol = 0;
            }
            return;
        }

        if !self.injobs {
            return;
        }
        if self.jobcol < 0 {
            self.jobcol = c;
        }

        if c == self.jobcol {
            self.curjob = keyof(line).unwrap_or("").to_string();
            if self.curjob.is_empty() {
                return;
            }
            self.out.push(Ev::Job(self.curjob.clone(), fnr));
            if self.pendjob {
                self.out.push(Ev::JobExempt);
            }
            self.pendjob = false;
            self.pendstep = false;
            self.jobkeycol = -1;
            self.stepcol = -1;
            self.stepdashcol = -1;
            self.insteps = false;
            return;
        }
        if self.curjob.is_empty() {
            return;
        }
        if self.jobkeycol < 0 {
            self.jobkeycol = c;
        }

        if c == self.jobkeycol {
            self.stepcol = -1;
            let k = keyof(line).unwrap_or("");
            self.insteps = k == "steps";
            if k == "env" {
                self.envscope = EnvScope::Job;
                self.envcol = c;
            }
            return;
        }

        if !self.insteps {
            return;
        }

        if let Some(rlen) = dash_prefix_len(line) {
            if self.stepdashcol < 0 {
                self.stepdashcol = c;
            }
            if c == self.stepdashcol {
                self.stepcol = rlen as i64;
                self.out.push(Ev::Step);
                if self.pendstep {
                    self.out.push(Ev::StepExempt);
                }
                self.pendstep = false;
                let rest = substr_from(line, rlen).to_string();
                self.stepkey(&rest, self.stepcol, fnr);
                return;
            }
        }
        if self.stepcol >= 0 && c == self.stepcol {
            let rest = substr_from(line, c as usize).to_string();
            self.stepkey(&rest, c, fnr);
        }
    }

    fn finish(mut self) -> Vec<Ev> {
        if self.inrun {
            self.endrun();
        }
        self.out
    }
}

pub fn walk_file(text: &str, mark: &str) -> Vec<Ev> {
    let mut w = Walk::new(mark);
    for (i, raw) in text.lines().enumerate() {
        w.line(raw, i + 1);
    }
    w.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GH: &str = "gh-repo-exempt";

    fn gh_hits(body: &str) -> Vec<(usize, bool)> {
        let mut w = Walk::new(GH);
        w.scanlogical(body, 1);
        w.finish()
            .into_iter()
            .filter_map(|e| match e {
                Ev::Gh(l, r) => Some((l, r)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn command_position_and_whole_word_bound_the_detector() {
        assert_eq!(gh_hits("gh release view v1").len(), 1);
        assert_eq!(gh_hits("if ! gh release view v1; then").len(), 1);
        assert_eq!(gh_hits("x=\"$(gh issue list)\"").len(), 1);
        assert_eq!(gh_hits("then gh issue list").len(), 1);
        assert_eq!(gh_hits("ghost --version").len(), 0);
        assert_eq!(gh_hits("git push origin gh-pages").len(), 0);
        assert_eq!(gh_hits("  # gh release create v9").len(), 0);
        assert_eq!(gh_hits("echo \"the gh CLI is not invoked\"").len(), 0);
    }

    #[test]
    fn the_repo_flag_is_read_within_one_call_only() {
        assert_eq!(gh_hits("gh issue list --repo owner/repo"), vec![(1, true)]);
        assert_eq!(gh_hits("gh issue list --repo=owner/repo"), vec![(1, true)]);
        assert_eq!(gh_hits("gh issue list --repository x"), vec![(1, false)]);
        assert_eq!(
            gh_hits("gh a --repo owner/repo; gh b"),
            vec![(1, true), (1, false)]
        );
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — the valve's reason is required and
    // non-empty, and a bare marker is its own failure class.
    #[test]
    fn a_marker_without_a_reason_is_distinguished_from_one_with_it() {
        assert_eq!(marker_reason("    # gh-repo-exempt: because", GH), Some("because"));
        assert_eq!(marker_reason("    # gh-repo-exempt:   ", GH), Some(""));
        assert_eq!(marker_reason("    # gh-repo-exempt", GH), None);
    }

    // spec: gate-sdk/SPEC.md §check-action-gh-repo — the marker token is the caller's, so the
    // same walk yields a second consumer's spelling and is blind to its sibling's.
    #[test]
    fn the_marker_token_is_the_callers_and_only_its_own_spelling_binds() {
        assert_eq!(
            marker_reason("    # action-permissions-exempt: why", "action-permissions-exempt"),
            Some("why")
        );
        assert_eq!(
            marker_reason("    # gh-repo-exempt: why", "action-permissions-exempt"),
            None
        );
        let evs = walk_file(
            "jobs:\n  j:\n    # action-permissions-exempt: stated\n    steps:\n      - run: gh x\n",
            "action-permissions-exempt",
        );
        assert!(evs.iter().any(|e| matches!(e, Ev::JobExempt)));
        assert!(!walk_file(
            "jobs:\n  j:\n    # action-permissions-exempt: stated\n    steps:\n      - run: gh x\n",
            GH
        )
        .iter()
        .any(|e| matches!(e, Ev::JobExempt)));
    }

    #[test]
    fn a_step_key_is_told_from_a_nested_list_entry() {
        assert_eq!(keyof("uses: a/b@v1"), Some("uses"));
        assert_eq!(keyof("  run: |"), Some("run"));
        assert_eq!(keyof("name: uses: not-a-key"), Some("name"));
        assert_eq!(keyof("- not a mapping"), None);
        assert_eq!(dash_prefix_len("      - uses: x"), Some(8));
        assert_eq!(dash_prefix_len("      -uses: x"), None);
    }

    #[test]
    fn a_continuation_makes_one_logical_line_of_a_split_call() {
        let mut w = Walk::new(GH);
        for (i, l) in [
            "jobs:",
            "  j:",
            "    steps:",
            "      - run: |",
            "          gh release upload v1 dist/a \\",
            "            --clobber --repo owner/repo",
        ]
        .iter()
        .enumerate()
        {
            w.line(l, i + 1);
        }
        let hits: Vec<(usize, bool)> = w
            .finish()
            .into_iter()
            .filter_map(|e| match e {
                Ev::Gh(l, r) => Some((l, r)),
                _ => None,
            })
            .collect();
        assert_eq!(hits, vec![(5, true)]);
    }
}
