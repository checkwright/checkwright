// spec: queue-kit/SPEC.md §The queue verbs — `--queue <verb> <operand>…`: one move or stamp, written
// only when its postcondition holds, then the queue's own gates as the post-check
use crate::emit::rewrite;
use crate::proc;
use crate::programs;
use crate::queue::{self, Entry, Sections};
use std::io::{Read, Write};

pub const USAGE: &str = "\
usage: --queue <verb> <operand>...
  promote <slug> <section> [--spec <file>]      a deferred entry into an active section
  done <slug>                                   an entry or sub-task to the done section
  clear-done                                    every bullet under the done section removed
  icebox <slug> <sentence>                      a deferred entry into the icebox
  recur <slug> [<date>]                         a date appended to the [recurrence:] array
  demote <slug> [--cost <class>] [--surface <entry>]
                                                an active entry back to the deferred section
  thaw <slug> [--cost <class>] [--surface <entry>] [--date <date>]
                                                an icebox entry back, its body restored
  split <parent> <child> <parent-sentence>      the child's tag line and body on stdin
  exit 0 written and the queue's gates green, 1 written and a gate red, 2 nothing written
";

const QUEUE_READS: &[&str] = &[
    "QUEUE_KIT_QUEUE_FILE",
    "QUEUE_KIT_ACTIVE_SECTIONS",
    "QUEUE_KIT_DEFERRED_SECTION",
    "QUEUE_KIT_ICEBOX_SECTION",
    "QUEUE_KIT_DONE_SECTION",
];

const fn joined<const N: usize>(a: &[&'static str], b: &[&'static str]) -> [&'static str; N] {
    let mut out = [""; N];
    let mut i = 0;
    while i < a.len() {
        out[i] = a[i];
        i += 1;
    }
    let mut j = 0;
    while j < b.len() {
        out[a.len() + j] = b[j];
        j += 1;
    }
    out
}

const ROSTER_LEN: usize = QUEUE_READS.len() + crate::runner::KNOBS.len();
const ROSTER: [&str; ROSTER_LEN] = joined(QUEUE_READS, crate::runner::KNOBS);

// spec: gate-sdk/SPEC.md §The non-gate arm — a dispatching arm declares its callee's reads beside its own
pub const KNOBS: &[&str] = &ROSTER;

// spec: queue-kit/SPEC.md §The queue verbs — the inputs a verb reads besides the file
struct Ctx<'a> {
    today: String,
    stdin: String,
    history: &'a dyn History,
}

// spec: queue-kit/SPEC.md §The queue verbs — the queue at the revision before the newest commit
// moving `slug` out of `from` into one of `into`
trait History {
    fn before_move(&self, slug: &str, from: &str, into: &[&str]) -> Result<Option<String>, String>;
}

struct GitHistory<'a> {
    file: &'a str,
    sec: &'a Sections,
}

impl History for GitHistory<'_> {
    fn before_move(&self, slug: &str, from: &str, into: &[&str]) -> Result<Option<String>, String> {
        let rows = queue::transitions(self.file, slug, self.sec)?;
        match rows.iter().rev().find(|t| t.from == from && into.contains(&t.to.as_str())) {
            Some(queue::Transition { prior: Some(p), .. }) => queue::revision_text(self.file, p, self.sec),
            _ => Ok(None),
        }
    }
}

// spec: queue-kit/SPEC.md §The queue verbs — the new file and one line per act, or `None` for a
// stamp already present, which writes nothing
struct Outcome {
    text: Option<String>,
    acts: Vec<String>,
}

pub fn run(args: &[String]) -> i32 {
    match execute(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("checkwright-gates: --queue: {}", e);
            2
        }
    }
}

fn execute(args: &[String]) -> Result<i32, String> {
    let sec = Sections::with_done()?;
    let file = queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?;
    let text = crate::emit::read_text(&file)?;
    let mut stdin = String::new();
    if args.first().is_some_and(|v| v == "split") {
        std::io::stdin()
            .read_to_string(&mut stdin)
            .map_err(|e| format!("cannot read the child's tag line and body on stdin: {}", e))?;
    }
    let history = GitHistory { file: &file, sec: &sec };
    let ctx = Ctx { today: crate::emit::kpi::today_iso(), stdin, history: &history };
    let out = apply(args, &text, &sec, &ctx)?;
    for a in &out.acts {
        println!("{}", a);
    }
    let Some(new) = out.text else {
        return Ok(0);
    };
    rewrite::write_replacing(&file, new.as_bytes())?;
    let _ = std::io::stdout().flush();
    Ok(match post_check(&file) {
        Ok(0) => 0,
        Ok(_) => 1,
        Err(e) => {
            println!("post-check: {}", e);
            1
        }
    })
}

// spec: queue-kit/SPEC.md §The queue verbs — the post-check spawns this executable on the argv the
// front end composes for `--for`, rather than calling the battery in process
fn post_check(file: &str) -> Result<i32, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("cannot locate the running binary: {}", e))?
        .display()
        .to_string();
    let gates_dir = crate::walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let top = crate::walk::toplevel_opt()?;
    let rel = top.as_deref().and_then(|t| crate::walk::rel_under(t, file)).unwrap_or(file);
    proc::run_to(
        &programs::CHECKWRIGHT_GATES.at(exe),
        &["--run", "--gates-dir", &gates_dir, "--for", rel],
        &proc::Sink::Inherit,
    )
}

// spec: queue-kit/SPEC.md §The queue verbs — the argv split into positionals and the verb's own
// flags, each flag taking one value; `--` ends option processing for a sentence led by a dash
fn operands(verb: &str, args: &[String], flags: &[&str]) -> Result<(Vec<String>, Opts), String> {
    let mut pos = Vec::new();
    let mut opts: Opts = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if a == "--" {
            pos.extend(args[i + 1..].iter().cloned());
            break;
        }
        if a.starts_with('-') && a.len() > 1 {
            if !flags.contains(&a.as_str()) {
                return Err(format!("{}: unknown option: {}\n{}", verb, a, USAGE));
            }
            let v = args.get(i + 1).ok_or_else(|| format!("{}: {} needs a value", verb, a))?;
            opts.push((a.clone(), v.clone()));
            i += 2;
            continue;
        }
        pos.push(a.clone());
        i += 1;
    }
    Ok((pos, opts))
}

type Opts = Vec<(String, String)>;

fn opt<'a>(opts: &'a [(String, String)], name: &str) -> Option<&'a str> {
    opts.iter().rev().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

fn arity(verb: &str, pos: &[String], min: usize, max: usize) -> Result<(), String> {
    if pos.len() < min || pos.len() > max {
        return Err(format!("{}: wrong number of operands\n{}", verb, USAGE));
    }
    Ok(())
}

type Sets = (Vec<String>, Vec<String>);

fn apply(args: &[String], text: &str, sec: &Sections, ctx: &Ctx) -> Result<Outcome, String> {
    let Some(verb) = args.first() else {
        return Err(format!("needs a <verb>\n{}", USAGE));
    };
    let rest = &args[1..];
    let mut doc = Doc::parse(text);
    let before: Sets = (live(&doc, sec), done(&doc, sec));
    let (acts, expect): (Option<Vec<String>>, Sets) = match verb.as_str() {
        "promote" => {
            let (pos, opts) = operands(verb, rest, &["--spec"])?;
            arity(verb, &pos, 2, 2)?;
            (promote(&mut doc, sec, &pos[0], &pos[1], opt(&opts, "--spec"))?, before)
        }
        "done" => {
            let (pos, _) = operands(verb, rest, &[])?;
            arity(verb, &pos, 1, 1)?;
            let a = done_move(&mut doc, sec, &pos[0])?;
            let mut exp = before;
            exp.0.retain(|s| s != &pos[0]);
            exp.1.push(pos[0].clone());
            (a, exp)
        }
        "clear-done" => {
            let (pos, _) = operands(verb, rest, &[])?;
            arity(verb, &pos, 0, 0)?;
            (clear_done(&mut doc, sec)?, (before.0, Vec::new()))
        }
        "icebox" => {
            let (pos, _) = operands(verb, rest, &[])?;
            arity(verb, &pos, 2, 2)?;
            (icebox(&mut doc, sec, &pos[0], &pos[1])?, before)
        }
        "recur" => {
            let (pos, _) = operands(verb, rest, &[])?;
            arity(verb, &pos, 1, 2)?;
            let date = pos.get(1).cloned().unwrap_or_else(|| ctx.today.clone());
            (recur(&mut doc, sec, &pos[0], &date)?, before)
        }
        "demote" => {
            let (pos, opts) = operands(verb, rest, &["--cost", "--surface"])?;
            arity(verb, &pos, 1, 1)?;
            let board = Board::from_opts(&opts)?;
            (demote(&mut doc, sec, &pos[0], &board, ctx)?, before)
        }
        "thaw" => {
            let (pos, opts) = operands(verb, rest, &["--cost", "--surface", "--date"])?;
            arity(verb, &pos, 1, 1)?;
            let board = Board::from_opts(&opts)?;
            let date = opt(&opts, "--date").map_or_else(|| ctx.today.clone(), str::to_string);
            (thaw(&mut doc, sec, &pos[0], &board, &date, ctx)?, before)
        }
        "split" => {
            let (pos, _) = operands(verb, rest, &[])?;
            arity(verb, &pos, 3, 3)?;
            let a = split(&mut doc, sec, &pos[0], &pos[1], &pos[2], &ctx.stdin)?;
            let mut exp = before;
            exp.0.push(pos[1].clone());
            (a, exp)
        }
        other => return Err(format!("unknown verb: {}\n{}", other, USAGE)),
    };
    let Some(acts) = acts else {
        let what = rest.first().map_or(String::new(), |s| format!(" {}", s));
        return Ok(Outcome { text: None, acts: vec![format!("{}{}: nothing to change; nothing written", verb, what)] });
    };
    postcondition(&doc, sec, &expect)?;
    Ok(Outcome { text: Some(doc.text()), acts })
}

// spec: queue-kit/SPEC.md §The queue verbs — the live and done sets after the transform against the
// set each verb states, compared as multisets; the refusal names the difference
fn postcondition(doc: &Doc, sec: &Sections, expect: &Sets) -> Result<(), String> {
    for (name, got, want) in [("live", live(doc, sec), &expect.0), ("done", done(doc, sec), &expect.1)] {
        let (mut g, mut w) = (got.clone(), want.clone());
        g.sort();
        w.sort();
        if g != w {
            let gained: Vec<&String> = g.iter().filter(|s| !w.contains(s)).collect();
            let lost: Vec<&String> = w.iter().filter(|s| !g.contains(s)).collect();
            return Err(format!(
                "the move would change the {} set beyond its own act (gained {:?}, lost {:?}); nothing written",
                name, gained, lost
            ));
        }
    }
    Ok(())
}

fn live(doc: &Doc, sec: &Sections) -> Vec<String> {
    queue::live_slugs(&doc.text(), sec)
}

fn done(doc: &Doc, sec: &Sections) -> Vec<String> {
    queue::done_slugs(&doc.text(), sec)
}

// spec: queue-kit/SPEC.md §The queue verbs — the file as lines, its final newline kept
struct Doc {
    lines: Vec<String>,
    nl: bool,
}

impl Doc {
    fn parse(text: &str) -> Doc {
        Doc { lines: text.lines().map(str::to_string).collect(), nl: text.ends_with('\n') }
    }

    fn text(&self) -> String {
        let mut t = self.lines.join("\n");
        if self.nl && !self.lines.is_empty() {
            t.push('\n');
        }
        t
    }

    fn entries(&self, sec: &Sections) -> Vec<Entry> {
        let refs: Vec<&str> = self.lines.iter().map(String::as_str).collect();
        queue::entries(&refs, sec)
    }

    fn find(&self, sec: &Sections, slug: &str) -> Option<Entry> {
        self.entries(sec).into_iter().find(|e| e.slug == slug)
    }

    // spec: queue-kit/SPEC.md §The queue verbs — a `## <name>` heading and the index its section
    // ends at, the next `## ` line or the end of the file
    fn section(&self, name: &str) -> Result<(usize, usize), String> {
        let h = self
            .lines
            .iter()
            .position(|l| queue::heading_name(l) == Some(name))
            .ok_or_else(|| format!("the queue has no '## {}' section", name))?;
        let end = (h + 1..self.lines.len())
            .find(|&i| queue::is_section_line(&self.lines[i]))
            .unwrap_or(self.lines.len());
        Ok((h, end))
    }

    // spec: queue-kit/SPEC.md §The queue verbs — an entry's extent lifted out, its trailing blanks
    // dropped, and the seam it leaves closed to one blank line
    fn take(&mut self, e: &Entry) -> Vec<String> {
        let mut block: Vec<String> = self.lines.drain(e.start..e.end).collect();
        while block.last().is_some_and(|l| blank(l)) {
            block.pop();
        }
        let at = e.start;
        while at > 0 && at < self.lines.len() && blank(&self.lines[at - 1]) && blank(&self.lines[at]) {
            self.lines.remove(at);
        }
        if at >= self.lines.len() {
            while self.lines.last().is_some_and(|l| blank(l)) {
                self.lines.pop();
            }
        }
        block
    }

    // spec: queue-kit/SPEC.md §The queue verbs — a block placed after line `last`, a blank line on
    // each side
    fn insert_after(&mut self, last: usize, block: Vec<String>) {
        let mut ins = vec![String::new()];
        ins.extend(block);
        if last + 1 < self.lines.len() && !blank(&self.lines[last + 1]) {
            ins.push(String::new());
        }
        self.lines.splice(last + 1..last + 1, ins);
    }

    fn last_content(&self, from: usize, end: usize) -> usize {
        (from..end).rev().find(|&i| !blank(&self.lines[i])).unwrap_or(from)
    }

    fn append_entry(&mut self, section: &str, block: Vec<String>) -> Result<(), String> {
        let (h, end) = self.section(section)?;
        let last = self.last_content(h, end);
        self.insert_after(last, block);
        Ok(())
    }

    // spec: queue-kit/SPEC.md §The queue verbs — an in-place edit of one entry's head, its trailing
    // blank lines kept as they stood
    fn edit(&mut self, e: &Entry, f: impl FnOnce(&mut Vec<String>)) {
        let mut block: Vec<String> = self.lines[e.start..e.end].to_vec();
        let mut tail = 0;
        while block.last().is_some_and(|l| blank(l)) {
            block.pop();
            tail += 1;
        }
        f(&mut block);
        block.resize(block.len() + tail, String::new());
        self.lines.splice(e.start..e.end, block);
    }
}

fn blank(l: &str) -> bool {
    l.trim().is_empty()
}

// spec: queue-kit/SPEC.md §The queue format — the tag line's bracketed tokens, in order
fn tokens(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find('[') {
        let Some(close) = rest[open..].find(']') else { break };
        out.push(rest[open..open + close + 1].to_string());
        rest = &rest[open + close + 1..];
    }
    out
}

fn token_name(tok: &str) -> &str {
    let body = tok.trim_start_matches('[').trim_end_matches(']');
    body.split(':').next().unwrap_or(body)
}

fn tag_index(block: &[String]) -> Option<usize> {
    (1..block.len())
        .find(|&i| !blank(&block[i]))
        .filter(|&i| queue::heading_level(&block[i]).is_none() && queue::is_tag_line(&block[i]))
}

fn block_tags(block: &[String]) -> Vec<String> {
    tag_index(block).map(|i| tokens(&block[i])).unwrap_or_default()
}

// spec: queue-kit/SPEC.md §The queue format — the tag line rewritten, created under the heading
// where the entry had none, and removed with its blank line where no tag is left
fn set_tags(block: &mut Vec<String>, toks: &[String]) {
    match (tag_index(block), toks.is_empty()) {
        (Some(i), true) => {
            block.remove(i);
            if i < block.len() && blank(&block[i]) {
                block.remove(i);
            }
            while block.len() > 1 && block.last().is_some_and(|l| blank(l)) {
                block.pop();
            }
        }
        (Some(i), false) => block[i] = toks.join(" "),
        (None, true) => {}
        (None, false) => {
            block.insert(1, toks.join(" "));
            block.insert(1, String::new());
            if block.len() > 3 && !blank(&block[3]) {
                block.insert(3, String::new());
            }
        }
    }
}

fn split_tags(toks: Vec<String>, names: &[&str]) -> (Vec<String>, Vec<String>) {
    toks.into_iter().partition(|t| !names.contains(&token_name(t)))
}

fn act(verb: &str, slug: &str, from: &str, to: &str, dropped: &[String], added: &[String]) -> String {
    let mut s = format!("{} {}: {} -> {}", verb, slug, from, to);
    if !dropped.is_empty() {
        s.push_str(&format!("; dropped {}", dropped.join(" ")));
    }
    if !added.is_empty() {
        s.push_str(&format!("; added {}", added.join(" ")));
    }
    s
}

// spec: queue-kit/SPEC.md §The queue verbs — every verb's operand is a top-level entry, `done` and
// `recur` a sub-task too
fn top_level(verb: &str, e: &Entry) -> Result<(), String> {
    if e.level != 3 {
        return Err(format!(
            "{} takes a top-level entry, and {} is a sub-task: make it an entry of its own with `split` first",
            verb, e.slug
        ));
    }
    Ok(())
}

fn live_entry(doc: &Doc, sec: &Sections, slug: &str) -> Result<Entry, String> {
    doc.find(sec, slug).ok_or_else(|| {
        if queue::done_slugs(&doc.text(), sec).iter().any(|s| s == slug) {
            format!("{} is in the done section, not a live entry", slug)
        } else {
            format!("no live entry: {}", slug)
        }
    })
}

fn in_section(e: &Entry, want: &str, verb: &str) -> Result<(), String> {
    if e.section != want {
        return Err(format!("{} moves an entry out of {}, and {} is in {}", verb, want, e.slug, e.section));
    }
    Ok(())
}

type Acts = Result<Option<Vec<String>>, String>;

fn promote(doc: &mut Doc, sec: &Sections, slug: &str, section: &str, spec: Option<&str>) -> Acts {
    if !sec.active.iter().any(|a| a == section) {
        return Err(format!("not an active section: {} (active: {})", section, sec.active.join(", ")));
    }
    let e = live_entry(doc, sec, slug)?;
    top_level("promote", &e)?;
    in_section(&e, &sec.deferred, "promote")?;
    let mut block = doc.take(&e);
    let drop: &[&str] = if spec.is_some() { &["cost", "surface", "spec"] } else { &["cost", "surface"] };
    let (mut keep, dropped) = split_tags(block_tags(&block), drop);
    let mut added = Vec::new();
    if let Some(f) = spec {
        let t = format!("[spec: {}]", f);
        keep.insert(0, t.clone());
        added.push(t);
    }
    set_tags(&mut block, &keep);
    doc.append_entry(section, block)?;
    Ok(Some(vec![act("promote", slug, &e.section, section, &dropped, &added)]))
}

fn done_move(doc: &mut Doc, sec: &Sections, slug: &str) -> Acts {
    let e = live_entry(doc, sec, slug)?;
    let dropped = block_tags(&doc.lines[e.start..e.end]);
    let roadmap = dropped.iter().any(|t| token_name(t) == "roadmap");
    if roadmap && (sec.is_deferred(&e.section) || sec.is_icebox(&e.section)) {
        return Err(format!(
            "{} carries [roadmap:] in {}: no drain may retire it, and reversing it is the ruling authority's",
            slug, e.section
        ));
    }
    doc.take(&e);
    let (h, end) = doc.section(&sec.done)?;
    let last = doc.last_content(h, end);
    let mut ins = Vec::new();
    if last == h {
        ins.push(String::new());
    }
    ins.push(format!("- {}", slug));
    if last + 1 < doc.lines.len() && !blank(&doc.lines[last + 1]) {
        ins.push(String::new());
    }
    doc.lines.splice(last + 1..last + 1, ins);
    let link = format!("[{0}](#{0})", slug);
    let cite = format!("`{}`", slug);
    let mut rewritten = 0;
    for l in doc.lines.iter_mut() {
        let n = l.matches(&link).count();
        if n > 0 {
            *l = l.replace(&link, &cite);
            rewritten += n;
        }
    }
    let mut a = act("done", slug, &e.section, &sec.done, &dropped, &[]);
    if rewritten > 0 {
        a.push_str(&format!("; retired {} link(s) to {}", rewritten, cite));
    }
    Ok(Some(vec![a]))
}

fn clear_done(doc: &mut Doc, sec: &Sections) -> Acts {
    let (h, end) = doc.section(&sec.done)?;
    let cleared: Vec<String> = doc.lines[h + 1..end]
        .iter()
        .filter(|l| queue::is_bullet(l))
        .map(|l| queue::bare_bullet_slug(l).unwrap_or(l.trim()).to_string())
        .collect();
    if cleared.is_empty() {
        return Ok(None);
    }
    let mut body: Vec<String> = doc.lines[h + 1..end].iter().filter(|l| !queue::is_bullet(l)).cloned().collect();
    body.dedup_by(|a, b| blank(a) && blank(b));
    if body.is_empty() && end < doc.lines.len() {
        body.push(String::new());
    }
    doc.lines.splice(h + 1..end, body);
    Ok(Some(cleared.iter().map(|s| format!("clear-done {}: {} -> {}", s, sec.done, queue::ABSENT)).collect()))
}

fn icebox(doc: &mut Doc, sec: &Sections, slug: &str, sentence: &str) -> Acts {
    if sec.icebox.is_empty() {
        return Err("no icebox is configured (QUEUE_KIT_ICEBOX_SECTION is empty)".to_string());
    }
    if blank(sentence) || sentence.contains(['\n', '\r']) {
        return Err("the icebox sentence must be one non-empty line".to_string());
    }
    let e = live_entry(doc, sec, slug)?;
    top_level("icebox", &e)?;
    in_section(&e, &sec.deferred, "icebox")?;
    let tags = block_tags(&doc.lines[e.start..e.end]);
    if let Some(t) = tags.iter().find(|t| ["roadmap", "not-icebox-eligible"].contains(&token_name(t))) {
        return Err(format!("{} carries {}, which keeps it out of the icebox", slug, t));
    }
    doc.take(&e);
    let block = vec![format!("### {}", slug), String::new(), sentence.trim().to_string()];
    doc.append_entry(&sec.icebox, block)?;
    Ok(Some(vec![act("icebox", slug, &e.section, &sec.icebox, &tags, &[])]))
}

// spec: queue-kit/SPEC.md §The queue verbs — `recur`'s rules on one tag line: the array and the date
// parse, the date is not older than the newest, and a date equal to it is already stamped (`None`)
fn stamp(tag_line: &str, slug: &str, date: &str) -> Result<Option<String>, String> {
    if !queue::is_calendar_date(date) {
        return Err(format!("not a calendar date: '{}'", date));
    }
    let dates = queue::recurrence_array(tag_line)
        .map_err(|tok| format!("{}'s [recurrence:] array holds a malformed token '{}'", slug, tok))?;
    if let Some(last) = dates.last() {
        if *last == date {
            return Ok(None);
        }
        if date < *last {
            return Err(format!(
                "{} is older than {}'s newest recurrence {}: dates are appended in order",
                date, slug, last
            ));
        }
    }
    let mut all: Vec<&str> = dates.clone();
    all.push(date);
    let tag = format!("[recurrence: {}]", all.join(", "));
    Ok(Some(match queue::field_tags(tag_line, "recurrence").first() {
        Some(t) => format!("{}{}{}", &tag_line[..t.start], tag, &tag_line[t.end..]),
        None if tag_line.is_empty() => tag,
        None => format!("{} {}", tag_line, tag),
    }))
}

fn recur(doc: &mut Doc, sec: &Sections, slug: &str, date: &str) -> Acts {
    let Some(e) = doc.find(sec, slug) else {
        if queue::done_slugs(&doc.text(), sec).iter().any(|s| s == slug) {
            return Err(format!(
                "{} is found only in the done section: a finding there is a new defect, filed as a gap, not a recurrence",
                slug
            ));
        }
        return Err(format!("no live entry: {}", slug));
    };
    let block = doc.lines[e.start..e.end].to_vec();
    let line = tag_index(&block).map(|i| block[i].clone()).unwrap_or_default();
    let Some(new) = stamp(&line, slug, date)? else {
        return Ok(None);
    };
    let toks = tokens(&new);
    doc.edit(&e, |b| set_tags(b, &toks));
    Ok(Some(vec![format!("recur {}: {}; stamped {}", slug, e.section, date)]))
}

// spec: queue-kit/SPEC.md §The queue verbs — the board tags a demotion or a thaw writes: a flag's
// value, validated in the tag's grammar
struct Board {
    cost: Option<String>,
    surface: Option<String>,
}

impl Board {
    fn from_opts(opts: &[(String, String)]) -> Result<Board, String> {
        let cost = opt(opts, "--cost").map(str::to_string);
        let surface = opt(opts, "--surface").map(str::to_string);
        if let Some(c) = &cost {
            if !queue::cost_class_valid(c) {
                return Err(format!("--cost '{}' is not <recurrence>/<magnitude>", c));
            }
        }
        if let Some(s) = &surface {
            if !queue::surface_value_shaped(s) {
                return Err(format!("--surface '{}' is not one top-level root entry", s));
            }
        }
        Ok(Board { cost, surface })
    }

    // spec: queue-kit/SPEC.md §The queue verbs — a flag overrides the restored tag, and the pair is
    // written first on the tag line
    fn over(&self, toks: Vec<String>, restored: &[String]) -> Vec<String> {
        let find = |name: &str| restored.iter().find(|t| token_name(t) == name).cloned();
        let cost = self.cost.as_ref().map(|c| format!("[cost: {}]", c)).or_else(|| find("cost"));
        let surface = self.surface.as_ref().map(|s| format!("[surface: {}]", s)).or_else(|| find("surface"));
        let (rest, _) = split_tags(toks, &["cost", "surface"]);
        cost.into_iter().chain(surface).chain(rest).collect()
    }
}

fn section_slugs(text: &str, sec: &Sections, section: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    queue::entries(&lines, sec)
        .into_iter()
        .filter(|e| e.level == 3 && e.section == section)
        .map(|e| e.slug)
        .collect()
}

// spec: queue-kit/SPEC.md §The queue verbs — the predecessor rule: after the nearest entry that
// preceded the slug in the earlier revision and still stands in the deferred section, else leading
// the section
fn place_restored(doc: &mut Doc, sec: &Sections, slug: &str, prior: &str, block: Vec<String>) -> Result<(), String> {
    let was = section_slugs(prior, sec, &sec.deferred);
    let idx = was.iter().position(|s| s == slug).unwrap_or(0);
    let here: Vec<Entry> = doc
        .entries(sec)
        .into_iter()
        .filter(|e| e.level == 3 && e.section == sec.deferred)
        .collect();
    for p in was[..idx].iter().rev() {
        if let Some(e) = here.iter().find(|e| &e.slug == p) {
            let last = doc.last_content(e.start, e.end);
            doc.insert_after(last, block);
            return Ok(());
        }
    }
    match here.first() {
        Some(first) => {
            let mut ins = block;
            ins.push(String::new());
            doc.lines.splice(first.start..first.start, ins);
            Ok(())
        }
        None => doc.append_entry(&sec.deferred, block),
    }
}

fn prior_entry(prior: &str, sec: &Sections, slug: &str) -> Option<Vec<String>> {
    let p = Doc::parse(prior);
    let e = p.find(sec, slug).filter(|e| e.section == sec.deferred)?;
    let mut block = p.lines[e.start..e.end].to_vec();
    while block.last().is_some_and(|l| blank(l)) {
        block.pop();
    }
    Some(block)
}

fn demote(doc: &mut Doc, sec: &Sections, slug: &str, board: &Board, ctx: &Ctx) -> Acts {
    let e = live_entry(doc, sec, slug)?;
    top_level("demote", &e)?;
    if !sec.active.contains(&e.section) {
        return Err(format!("demote moves an entry out of an active section, and {} is in {}", slug, e.section));
    }
    let into: Vec<&str> = sec.active.iter().map(String::as_str).collect();
    let prior = ctx.history.before_move(slug, &sec.deferred, &into)?;
    if prior.is_none() && (board.cost.is_none() || board.surface.is_none()) {
        return Err(format!(
            "{} has no move from {} into an active section in the file's history, so --cost and --surface are both required",
            slug, sec.deferred
        ));
    }
    let restored = prior
        .as_deref()
        .and_then(|p| prior_entry(p, sec, slug))
        .map(|b| block_tags(&b))
        .unwrap_or_default();
    let mut block = doc.take(&e);
    let (keep, mut dropped) = split_tags(block_tags(&block), &["spec", "drain-exempt"]);
    let toks = board.over(keep.clone(), &restored);
    let added: Vec<String> = toks.iter().filter(|t| !keep.contains(t)).cloned().collect();
    dropped.extend(keep.iter().filter(|t| !toks.contains(t)).cloned());
    set_tags(&mut block, &toks);
    match &prior {
        Some(p) => place_restored(doc, sec, slug, p, block)?,
        None => doc.append_entry(&sec.deferred, block)?,
    }
    Ok(Some(vec![act("demote", slug, &e.section, &sec.deferred, &dropped, &added)]))
}

fn thaw(doc: &mut Doc, sec: &Sections, slug: &str, board: &Board, date: &str, ctx: &Ctx) -> Acts {
    if sec.icebox.is_empty() {
        return Err("no icebox is configured (QUEUE_KIT_ICEBOX_SECTION is empty)".to_string());
    }
    let e = live_entry(doc, sec, slug)?;
    top_level("thaw", &e)?;
    in_section(&e, &sec.icebox, "thaw")?;
    let prior = ctx.history.before_move(slug, &sec.deferred, &[sec.icebox.as_str()])?.ok_or_else(|| {
        format!(
            "{} has no move from {} into {} in the file's history, so there is no body to restore",
            slug, sec.deferred, sec.icebox
        )
    })?;
    let mut block = prior_entry(&prior, sec, slug)
        .ok_or_else(|| format!("the revision before {}'s eviction carries no {} entry for it", slug, sec.deferred))?;
    let restored = block_tags(&block);
    let toks = board.over(restored.clone(), &restored);
    let toks = match stamp(&toks.join(" "), slug, date)? {
        Some(l) => tokens(&l),
        None => toks,
    };
    set_tags(&mut block, &toks);
    doc.take(&e);
    place_restored(doc, sec, slug, &prior, block)?;
    let added: Vec<String> = toks.iter().filter(|t| !restored.contains(t)).cloned().collect();
    let mut a = act("thaw", slug, &e.section, &sec.deferred, &[], &added);
    a.push_str("; body restored");
    Ok(Some(vec![a]))
}

fn split(doc: &mut Doc, sec: &Sections, parent: &str, child: &str, sentence: &str, stdin: &str) -> Acts {
    if !queue::is_slug(child) {
        return Err(format!("not a valid slug: {}", child));
    }
    let text = doc.text();
    let known = queue::live_slugs(&text, sec).into_iter().chain(queue::done_slugs(&text, sec));
    if known.into_iter().any(|s| s == child) {
        return Err(format!("{} is already live or done", child));
    }
    let up = format!("[{0}](#{0})", parent);
    let down = format!("[{0}](#{0})", child);
    if !stdin.contains(&up) {
        return Err(format!("the child's text on stdin carries no {} link to its parent", up));
    }
    if !sentence.contains(&down) || sentence.contains(['\n', '\r']) {
        return Err(format!("the parent sentence must be one line carrying the {} link to the child", down));
    }
    let e = live_entry(doc, sec, parent)?;
    top_level("split", &e)?;
    let own_end = (e.start + 1..e.end)
        .find(|&i| queue::heading_level(&doc.lines[i]).is_some())
        .unwrap_or(e.end);
    let last = doc.last_content(e.start, own_end);
    doc.insert_after(last, vec![sentence.trim().to_string()]);
    let e = doc.find(sec, parent).ok_or_else(|| format!("{} vanished mid-split", parent))?;
    let mut body: Vec<String> = stdin.lines().map(str::to_string).collect();
    while body.first().is_some_and(|l| blank(l)) {
        body.remove(0);
    }
    while body.last().is_some_and(|l| blank(l)) {
        body.pop();
    }
    let mut block = vec![format!("### {}", child), String::new()];
    block.extend(body);
    let last = doc.last_content(e.start, e.end);
    doc.insert_after(last, block);
    Ok(Some(vec![format!("split {}: added {} after it in {}", parent, child, e.section)]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sec() -> Sections {
        Sections {
            active: vec!["New Features".into(), "Technical Debt".into()],
            deferred: "Deferred".into(),
            icebox: "Icebox".into(),
            done: "Done".into(),
        }
    }

    struct NoHistory;
    impl History for NoHistory {
        fn before_move(&self, _: &str, _: &str, _: &[&str]) -> Result<Option<String>, String> {
            Ok(None)
        }
    }

    struct Prior(String);
    impl History for Prior {
        fn before_move(&self, _: &str, _: &str, _: &[&str]) -> Result<Option<String>, String> {
            Ok(Some(self.0.clone()))
        }
    }

    const Q: &str = "\
# TASK-QUEUE.md

## New Features

### live-a

[spec: SPEC-a.md] [drain-exempt: tail]

the active entry, cited by [def-b](#def-b).

## Technical Debt

## Deferred

### def-b

[cost: event/low] [surface: gate-sdk] [recurrence: 2026-01-01]

deferred b, which cites [live-a](#live-a).

#### def-b-sub

a sub-task.

### def-c

[roadmap: next/x] [cost: once/low] [surface: canon-kit]

deferred c.

### def-d

[cost: event/low] [surface: gate-sdk]

deferred d.

## Icebox

### ice-e

one sentence.

## Done

- old-f

## Lessons Learned
";

    fn run_with(args: &[&str], text: &str, history: &dyn History, stdin: &str) -> Result<Outcome, String> {
        let argv: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let ctx = Ctx { today: "2026-09-28".into(), stdin: stdin.into(), history };
        apply(&argv, text, &sec(), &ctx)
    }

    fn ok(args: &[&str]) -> String {
        run_with(args, Q, &NoHistory, "").expect("the verb succeeds").text.expect("the verb writes")
    }

    fn refused(args: &[&str]) -> String {
        match run_with(args, Q, &NoHistory, "") {
            Err(e) => e,
            Ok(_) => panic!("{:?} was not refused", args),
        }
    }

    fn place(text: &str, slug: &str) -> String {
        queue::place_of(text, &sec(), slug)
    }

    // spec: queue-kit/SPEC.md §The queue verbs — promote drops the board tags, writes --spec first,
    // lands last in the named active section, and keeps the live set
    #[test]
    fn promote_moves_a_deferred_entry_and_drops_the_board_tags() {
        let t = ok(&["promote", "def-d", "Technical Debt", "--spec", "SPEC-d.md"]);
        assert_eq!(place(&t, "def-d"), "Technical Debt");
        assert!(t.contains("## Technical Debt\n\n### def-d\n\n[spec: SPEC-d.md]\n\ndeferred d.\n\n## Deferred"), "{}", t);
        let t = ok(&["promote", "def-b", "New Features"]);
        assert!(t.contains("### def-b\n\n[recurrence: 2026-01-01]\n"), "{}", t);
        assert_eq!(place(&t, "def-b-sub"), "New Features");
        assert!(refused(&["promote", "live-a", "New Features"]).contains("out of Deferred"));
        assert!(refused(&["promote", "def-d", "Deferred"]).contains("not an active section"));
        assert!(refused(&["promote", "def-b-sub", "New Features"]).contains("split"));
    }

    // spec: queue-kit/SPEC.md §The queue verbs — done lands a bare slug last in the done section and
    // rewrites every same-file link to it into the retired citation
    #[test]
    fn done_retires_the_entry_and_every_link_to_it() {
        let t = ok(&["done", "live-a"]);
        assert!(t.contains("## Done\n\n- old-f\n- live-a\n\n## Lessons"), "{}", t);
        assert!(t.contains("which cites `live-a`."), "{}", t);
        assert!(!t.contains("[live-a](#live-a)"));
        let t = ok(&["done", "def-b-sub"]);
        assert_eq!(queue::done_slugs(&t, &sec()), vec!["old-f".to_string(), "def-b-sub".to_string()]);
        assert!(refused(&["done", "def-c"]).contains("[roadmap:]"));
        assert!(refused(&["done", "def-b"]).contains("lost [\"def-b-sub\"]"));
        assert!(refused(&["done", "old-f"]).contains("done section"));
    }

    // spec: queue-kit/SPEC.md §The queue verbs — the done section emptied with its heading kept, and
    // an already empty section writes nothing
    #[test]
    fn clear_done_empties_the_section_and_is_idempotent() {
        let t = ok(&["clear-done"]);
        assert!(t.contains("## Done\n\n## Lessons Learned\n"), "{}", t);
        let again = run_with(&["clear-done"], &t, &NoHistory, "").expect("no-op");
        assert!(again.text.is_none());
    }

    // spec: queue-kit/SPEC.md §The queue verbs — icebox writes the one-sentence form, and the two
    // declared exclusions refuse
    #[test]
    fn icebox_compresses_the_entry_to_one_sentence() {
        let t = ok(&["icebox", "def-d", "dormant d."]);
        assert!(t.contains("### ice-e\n\none sentence.\n\n### def-d\n\ndormant d.\n\n## Done"), "{}", t);
        assert!(refused(&["icebox", "def-c", "x."]).contains("[roadmap:"));
        assert!(refused(&["icebox", "def-d", ""]).contains("one non-empty line"));
        assert!(refused(&["icebox", "live-a", "x."]).contains("out of Deferred"));
    }

    // spec: queue-kit/SPEC.md §The queue verbs — recur appends in order, creates the tag, stamps a
    // sub-task, is idempotent on the newest date and refuses an older or malformed one
    #[test]
    fn recur_appends_a_date_in_order() {
        let t = ok(&["recur", "def-b", "2026-02-01"]);
        assert!(t.contains("[recurrence: 2026-01-01, 2026-02-01]"), "{}", t);
        let t = ok(&["recur", "def-d"]);
        assert!(t.contains("[cost: event/low] [surface: gate-sdk] [recurrence: 2026-09-28]"), "{}", t);
        let t = ok(&["recur", "def-b-sub", "2026-03-01"]);
        assert!(t.contains("#### def-b-sub\n\n[recurrence: 2026-03-01]\n\na sub-task."), "{}", t);
        assert!(run_with(&["recur", "def-b", "2026-01-01"], Q, &NoHistory, "").expect("no-op").text.is_none());
        assert!(refused(&["recur", "def-b", "2025-12-31"]).contains("appended in order"));
        assert!(refused(&["recur", "def-b", "2026-02-30"]).contains("not a calendar date"));
        assert!(refused(&["recur", "old-f"]).contains("only in the done section"));
        let bad = Q.replace("[recurrence: 2026-01-01]", "[recurrence: 2026-01-01 2026-01-02]");
        let e = run_with(&["recur", "def-b", "2026-02-01"], &bad, &NoHistory, "").err().expect("refused");
        assert!(e.contains("malformed token '2026-01-01 2026-01-02'"), "{}", e);
    }

    // spec: queue-kit/SPEC.md §The queue verbs — demote without history needs both flags and lands
    // last; with history it restores the board tags and the place after its old predecessor
    #[test]
    fn demote_restores_position_and_board_tags() {
        assert!(refused(&["demote", "live-a"]).contains("both required"));
        let t = ok(&["demote", "live-a", "--cost", "once/low", "--surface", "queue-kit"]);
        assert!(
            t.contains("deferred d.\n\n### live-a\n\n[cost: once/low] [surface: queue-kit]\n\nthe active"),
            "{}",
            t
        );
        let prior = Q
            .replace("### live-a\n\n[spec: SPEC-a.md] [drain-exempt: tail]\n\nthe active entry, cited by [def-b](#def-b).\n\n", "")
            .replace("### def-d\n", "### live-a\n\n[cost: iteration/high] [surface: installer]\n\nold body.\n\n### def-d\n");
        let o = run_with(&["demote", "live-a", "--surface", "canon-kit"], Q, &Prior(prior), "").expect("demote");
        let t = o.text.expect("written");
        assert!(
            t.contains("deferred c.\n\n### live-a\n\n[cost: iteration/high] [surface: canon-kit]\n\nthe active entry"),
            "{}",
            t
        );
        assert!(o.acts[0].contains("dropped [spec: SPEC-a.md] [drain-exempt: tail]"), "{:?}", o.acts);
    }

    // spec: queue-kit/SPEC.md §The queue verbs — thaw restores the whole prior extent at its old place
    // and stamps the recurrence
    #[test]
    fn thaw_restores_the_body_and_stamps_a_recurrence() {
        assert!(refused(&["thaw", "ice-e"]).contains("no body to restore"));
        let prior = Q.replace(
            "### def-d\n",
            "### ice-e\n\n[cost: event/low] [surface: drift-kit]\n\nthe full body.\n\n### def-d\n",
        );
        let o = run_with(&["thaw", "ice-e", "--date", "2026-05-05"], Q, &Prior(prior), "").expect("thaw");
        let t = o.text.expect("written");
        assert!(
            t.contains("deferred c.\n\n### ice-e\n\n[cost: event/low] [surface: drift-kit] [recurrence: 2026-05-05]\n\nthe full body.\n\n### def-d"),
            "{}",
            t
        );
        assert!(t.contains("## Icebox\n\n## Done"), "{}", t);
    }

    // spec: queue-kit/SPEC.md §The queue verbs — split inserts the child after the parent and adds the
    // parent's sentence; the bidirectional links are required
    #[test]
    fn split_writes_the_child_and_the_parent_sentence() {
        let stdin = "[cost: event/low] [surface: gate-sdk]\n\nthe child of [def-d](#def-d).\n";
        let args = ["split", "def-d", "def-g", "Split out: [def-g](#def-g)."];
        let t = run_with(&args, Q, &NoHistory, stdin).expect("split").text.expect("written");
        assert!(
            t.contains("deferred d.\n\nSplit out: [def-g](#def-g).\n\n### def-g\n\n[cost: event/low] [surface: gate-sdk]\n\nthe child of [def-d](#def-d).\n\n## Icebox"),
            "{}",
            t
        );
        let e = run_with(&["split", "def-d", "def-g", "no link"], Q, &NoHistory, stdin).err().expect("refused");
        assert!(e.contains("[def-g](#def-g)"), "{}", e);
        let e = run_with(&["split", "def-d", "def-g", "[def-g](#def-g)"], Q, &NoHistory, "no link\n").err().expect("refused");
        assert!(e.contains("[def-d](#def-d)"), "{}", e);
        let e = run_with(&["split", "def-d", "def-b", "[def-b](#def-b)"], Q, &NoHistory, stdin).err().expect("refused");
        assert!(e.contains("already live"), "{}", e);
        let args = ["split", "def-b", "def-h", "[def-h](#def-h)."];
        let t = run_with(&args, Q, &NoHistory, "x [def-b](#def-b)\n").expect("split").text.expect("written");
        assert!(t.contains("which cites [live-a](#live-a).\n\n[def-h](#def-h).\n\n#### def-b-sub"), "{}", t);
    }

    // spec: queue-kit/SPEC.md §The queue verbs — an unknown verb, a bad arity and an unknown flag are
    // usage refusals carrying the usage
    #[test]
    fn usage_errors_refuse_with_the_usage() {
        for args in [vec![], vec!["nope"], vec!["done"], vec!["done", "a", "--x", "y"]] {
            let e = refused(&args);
            assert!(e.contains(USAGE), "{:?}: {}", args, e);
        }
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the declared roster is the queue reads and the
    // runner's, sentinel included
    #[test]
    fn the_roster_carries_the_runners_reads() {
        for k in QUEUE_READS.iter().chain(crate::runner::KNOBS) {
            assert!(KNOBS.contains(k), "{}", k);
        }
        assert!(KNOBS.contains(&crate::emit::EVERY_REGISTERED_KNOB));
    }
}
