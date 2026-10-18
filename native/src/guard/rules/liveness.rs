// spec: guard-kit/SPEC.md §The generic ruleset — the producer rules: a git write under a live
// recorded producer, a launch that records nothing, a wait no producer can end, and the two waits
// granted outright.
use super::grants::{slot_reach, Reach};
use super::reach::{interpreter_reach, refused, rm_tracked_reach};
use super::{
    background_statements, command_word, dups_onto_file, git_read, git_walk_of, has_expansion, has_substitution, inert_target, inert_value, is_banner, is_ro_segment,
    noclobber,
    loop_tokens, redirect_targets, ro_forms_clear, ro_forms_clear_statements, segment_core,
    shell_backgrounds, strip_redirect_op, unredirected_words, valued_targets, GitWalk,
};
use crate::guard::allow_match;
use crate::guard::engine::{Cmd, Ctx, Decided, Fault, Shell, Verdict};
use crate::guard::reader::View::{Sq, SqDqHd};
use crate::guard::reader::{DQ_MARK, SQ_MARK};
use crate::guard::text::{self, grep_q, head_word, trim, trim_start, unsentinel, words};

pub fn git_mutation_under_producer(ctx: &Ctx) -> Decided {
    if ctx.expands(ctx.cmd(), has_expansion)? {
        return Ok(None);
    }
    let mut writes: Vec<String> = Vec::new();
    for words in git_read(ctx, ctx.cmd())?.iter().flatten() {
        let Some(sub) = git_walk_of(words, GitWalk::Sub) else { continue };
        let sub = &sub[0];
        if matches!(
            sub.as_str(),
            "add" | "commit" | "rm" | "mv" | "restore" | "checkout" | "switch" | "reset" | "stash" | "merge"
                | "rebase" | "cherry-pick" | "revert" | "apply" | "am" | "clean"
        ) {
            writes.push(format!("git {}", sub));
        }
    }
    if writes.is_empty() {
        return Ok(None);
    }
    let runs = ctx.host().live_run_records();
    if runs.is_empty() {
        return Ok(None);
    }
    Ok(Some(Verdict::Block(format!("don't run '{}' while a producer you recorded is still running — {} names a live pid, and a tracked-tree mutation under a live producer is what the wait rule exists to prevent: the run is still writing, so a commit taken now dirties the worktree underneath it and its verdict has to be discarded and re-run. Two exits, both cheap: wait for that producer on its own artifact (loop on the recorded pid's liveness, 'until ! kill -0 <pid> 2>/dev/null; do sleep 5; done', backgrounded so its completion notifies you), or — if the producer has already exited — delete its .run file, which is not a workaround but the statement of fact becoming false and being retracted. Read-only git ('status', 'log', 'diff', 'show') is untouched. If you genuinely need this mutation now, run it yourself with !<command>.", writes[0], runs.join("; ")))))
}

// spec: guard-kit/SPEC.md §The generic ruleset — a child that writes nothing has nothing for a later
// commit to corrupt, so it owes no record.
fn is_ro_background(ctx: &Ctx, s: &str) -> Result<bool, Fault> {
    let h = ctx.host();
    if !valued_targets(ctx, ctx.cmd())?.iter().all(|t| inert_value(t)) {
        return Ok(false);
    }
    let mut reads = 0usize;
    for seg in ctx.segments(s) {
        let seg = trim_start(&seg);
        if seg.is_empty() || is_banner(seg) {
            continue;
        }
        if !is_ro_segment(h, seg) {
            return Ok(false);
        }
        reads += 1;
    }
    Ok(reads >= 1 && ro_forms_clear(ctx, ctx.cmd())?)
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `background_no_record`'s PowerShell shell form: a
// segment led by `Start-Process` or an alias, carrying no `-Wait` and no `-WhatIf` by any prefix.
fn ps_detaches(seg: &str, valued: Option<&str>) -> bool {
    let ws = words(trim_start(seg));
    let Some(head) = valued.or(ws.first().copied()) else { return false };
    if !["start-process", "saps", "start"].contains(&head.to_ascii_lowercase().as_str()) {
        return false;
    }
    !ws[1..].iter().any(|w| {
        let Some(p) = w.strip_prefix('-') else { return false };
        let (name, value) = match p.split_once(':') {
            Some((n, v)) => (n.to_ascii_lowercase(), Some(v)),
            None => (p.to_ascii_lowercase(), None),
        };
        let wait = name.len() >= 2 && "wait".starts_with(&name) && !value.is_some_and(|v| v.eq_ignore_ascii_case("$false"));
        wait || (name.len() >= 2 && "whatif".starts_with(&name))
    })
}

struct PsWrite {
    expands: bool,
    canonical: bool,
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `background_no_record`'s PowerShell exemption (1):
// `Set-Content -NoNewline`, a literal scratch `.run` path and an expandable value, and nothing else;
// `None` on a segment that is no `Set-Content`.
fn ps_record_write(ctx: &Ctx, seg: &str, raw_segs: &[String]) -> Option<PsWrite> {
    let seg = trim(seg);
    let ws = words(seg);
    if !ws.first()?.eq_ignore_ascii_case("set-content") {
        return None;
    }
    let (mut nonewline, mut path, mut value, mut positional) = (false, None, None, Vec::new());
    let mut canonical = true;
    let mut i = 1usize;
    while i < ws.len() {
        let w = ws[i];
        i += 1;
        let Some(p) = w.strip_prefix('-') else {
            positional.push(w);
            continue;
        };
        let name = p.to_ascii_lowercase();
        if !name.is_empty() && "nonewline".starts_with(&name) {
            nonewline = true;
        } else if name == "path" || name == "literalpath" {
            path = ws.get(i).copied();
            i += 1;
        } else if name == "value" {
            value = ws.get(i).copied();
            i += 1;
        } else {
            canonical = false;
        }
    }
    let mut positional = positional.into_iter();
    let path = path.or_else(|| positional.next());
    let value = value.or_else(|| positional.next());
    if positional.next().is_some() {
        canonical = false;
    }
    let path = path.map(|p| p.replace('\\', "/"));
    let expands = path.as_deref().is_some_and(|p| p.contains('$'));
    let h = ctx.host();
    let literal = path.as_deref().is_some_and(|p| {
        !p.contains(SQ_MARK) && !p.contains(DQ_MARK) && !p.contains(['$', '"', '\'', '`']) && p.ends_with(".run") && h.in_scratch(p)
    });
    canonical = canonical && nonewline && literal && value == Some(DQ_MARK) && raw_segs.iter().any(|r| ps_value_ends_newline(r, seg));
    Some(PsWrite { expands, canonical })
}

// spec: guard-kit/SPEC.md §The generic ruleset — the canonical write's value, read off the raw
// segment that masks to the skeleton's: one expandable string ending in the `` `n `` escape.
fn ps_value_ends_newline(raw_seg: &str, skel_seg: &str) -> bool {
    let r = trim(raw_seg);
    let (Some(open), Some(close)) = (r.find('"'), r.rfind('"')) else { return false };
    if close <= open || r[open + 1..close].contains('"') {
        return false;
    }
    let masked = format!("{}{}{}", &r[..open], DQ_MARK, &r[close + 1..]);
    masked == skel_seg && r[open + 1..close].ends_with("`n")
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `background_no_record`'s PowerShell exemption (2):
// the whole skeleton is one `while (<cond>) { <sleep> }`, `do { <sleep> } while (<cond>)` or
// `do { <sleep> } until (<cond>)`, its body one statement led by `Start-Sleep` or `sleep`.
fn ps_wait_loop(ctx: &Ctx, s: &str) -> bool {
    let s = trim(s);
    let b = s.as_bytes();
    let closing = |from: usize, open: u8, close: u8| -> Option<usize> {
        let mut depth = 0i32;
        for (k, &c) in b.iter().enumerate().skip(from) {
            if c == open {
                depth += 1;
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return Some(k);
                }
            }
        }
        None
    };
    let keyword = |at: usize, kw: &str| -> Option<usize> {
        let end = at + kw.len();
        (s.get(at..end)?.eq_ignore_ascii_case(kw) && b.get(end).is_some_and(|c| text::is_space(*c) || matches!(c, b'(' | b'{')))
            .then_some(end)
    };
    let skip = |mut at: usize| {
        while at < b.len() && text::is_space(b[at]) {
            at += 1;
        }
        at
    };
    let sleeps = |body: &str| {
        let segs: Vec<String> = ctx.segments(body).into_iter().filter(|g| !trim(g).is_empty()).collect();
        segs.len() == 1 && {
            let head = head_word(trim(&segs[0])).to_ascii_lowercase();
            head == "start-sleep" || head == "sleep"
        }
    };
    if let Some(at) = keyword(0, "while") {
        let open = skip(at);
        if b.get(open) != Some(&b'(') {
            return false;
        }
        let Some(cond_end) = closing(open, b'(', b')') else { return false };
        let body_open = skip(cond_end + 1);
        if b.get(body_open) != Some(&b'{') {
            return false;
        }
        return closing(body_open, b'{', b'}') == Some(b.len() - 1) && sleeps(&s[body_open + 1..b.len() - 1]);
    }
    let Some(at) = keyword(0, "do") else { return false };
    let body_open = skip(at);
    if b.get(body_open) != Some(&b'{') {
        return false;
    }
    let Some(body_close) = closing(body_open, b'{', b'}') else { return false };
    let tail = skip(body_close + 1);
    let Some(after) = keyword(tail, "while").or_else(|| keyword(tail, "until")) else { return false };
    let open = skip(after);
    b.get(open) == Some(&b'(')
        && closing(open, b'(', b')') == Some(b.len() - 1)
        && sleeps(&s[body_open + 1..body_close])
}

fn ps_background_no_record(ctx: &Ctx) -> Decided {
    let h = ctx.host();
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    let segs = ctx.segments(&s);
    let read = ctx.word_read(ctx.cmd())?;
    let head = |i: usize| read.as_ref().and_then(|r| r.get(i)).and_then(|ws| ws.first()).map(|w| w.value.as_str());
    if !(h.background || segs.iter().enumerate().any(|(i, g)| ps_detaches(g, head(i)))) {
        return Ok(None);
    }
    let raw_segs = ctx.segments(ctx.raw(ctx.cmd())?);
    let mut recorded = false;
    for g in &segs {
        if let Some(w) = ps_record_write(ctx, g, &raw_segs) {
            if w.expands {
                return Ok(None);
            }
            recorded |= w.canonical;
        }
    }
    if recorded || ps_wait_loop(ctx, &s) {
        return Ok(None);
    }
    let home = h.scratch_homes().into_iter().next().unwrap_or_default();
    Ok(Some(Verdict::Block(format!("this PowerShell call backgrounds a process and writes no liveness record — re-issue it with the record written first, in this spelling: 'Set-Content -NoNewline -Path {h}/<key>.run -Value \"pid=$PID run=<key>`n\"; try {{ <command> }} finally {{ Remove-Item {h}/<key>.run }}'. $PID is the process running the command, so the record reads held exactly while the command runs, and the -NoNewline with the trailing `n is what makes the record parse: a plain Set-Content or a '>' writes a CRLF record, which reads as corrupt. Give a Start-Process launch -Wait inside that spelling rather than leaving it detached. The record is what gives the tracked-tree-mutation rule its reach, so a commit taken while this process is still writing is refused rather than silently taken. An inline wait loop ('while (<cond>) {{ Start-Sleep 5 }}') owes no record. No guard grant applies under PowerShell, so the spelling costs one permission decision. If you genuinely need an unrecorded launch, run it yourself with !<command>.", h = home))))
}

// spec: guard-kit/SPEC.md §The rule roster — rule `wait_no_producer`'s clause (b) body test, which rule
// `background_no_record`'s exemption (2) takes: `sleep` statements and nothing else.
fn only_sleeps(ctx: &Ctx, body: &str) -> bool {
    let steps: Vec<String> = ctx.segments(body).into_iter().filter(|g| !trim(g).is_empty()).collect();
    !steps.is_empty() && steps.iter().all(|g| head_word(trim(g)) == "sleep")
}

struct LoopSpan {
    start: usize,
    cond: usize,
    open: usize,
    close: usize,
    commands: bool,
}

// spec: guard-kit/SPEC.md §The rule roster — rule `background_no_record`'s bash exemption (2): the
// call does no work its text shows, in a body, a condition or a statement outside a loop.
fn waits_only(ctx: &Ctx, s: &str, toks: &[(i32, bool, String, usize)]) -> Result<bool, Fault> {
    let mut loops: Vec<LoopSpan> = Vec::new();
    let (mut keyword, mut opened, mut nested) = (None, None, false);
    for (depth, _, tok, at) in toks.iter().filter(|t| t.1) {
        match tok.as_str() {
            "until" | "while" | "for" if *depth == 0 && keyword.is_none() => {
                keyword = Some((*at, at + tok.len(), tok != "for"))
            }
            "do" if *depth == 1 => {
                let (start, cond, commands) = keyword.take().unwrap_or((*at, *at, false));
                opened = Some(LoopSpan { start, cond, open: *at, close: *at, commands });
            }
            "do" => nested = true,
            "done" if *depth == 0 => {
                if let Some(span) = opened.take() {
                    loops.push(LoopSpan { close: *at, ..span });
                }
            }
            _ => {}
        }
    }
    if loops.is_empty() || nested {
        return Ok(false);
    }
    let text = |a: usize, b: usize| s.get(a.min(s.len())..b.min(s.len())).unwrap_or("");
    let stated = |t: &str| ctx.segments(t).into_iter().filter(|g| !trim(g).is_empty()).collect::<Vec<String>>();
    let shows_write = |g: &String| shell_backgrounds(g) || !wait_redirects_inert(ctx, g);
    for span in &loops {
        let body = text(span.open + 2, span.close);
        if !only_sleeps(ctx, body) || stated(body).iter().any(shows_write) {
            return Ok(false);
        }
        if span.commands && stated(text(span.cond, span.open)).iter().any(shows_write) {
            return Ok(false);
        }
    }
    let outside = |piece: &str| wait_statements_clear(ctx, piece);
    if !outside(text(0, loops[0].start)) {
        return Ok(false);
    }
    for (k, span) in loops.iter().enumerate() {
        let piece = text(span.close + 4, loops.get(k + 1).map_or(s.len(), |next| next.start));
        let (own, redirects) = (trim_start(piece), background_statements(piece)[0]);
        let inert = ctx.segments(redirects).len() == 1 && segment_core(redirects).is_empty() && wait_redirects_inert(ctx, redirects);
        let piece = if own.starts_with('&') && !own.starts_with("&&") {
            &own[1..]
        } else if redirects.len() < piece.len() && inert {
            &piece[redirects.len() + 1..]
        } else {
            piece
        };
        if !outside(piece) {
            return Ok(false);
        }
    }
    ro_forms_clear_statements(ctx, ctx.cmd())
}

pub fn background_no_record(ctx: &Ctx) -> Decided {
    if ctx.shell() == Shell::PowerShell {
        return ps_background_no_record(ctx);
    }
    let h = ctx.host();
    if has_expansion(ctx.raw(ctx.cmd())?) {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if !(h.background || shell_backgrounds(&s)) {
        return Ok(None);
    }
    if redirect_targets(&s).iter().any(|t| t.ends_with(".run") && h.in_scratch(t)) {
        return Ok(None);
    }
    let Some(toks) = loop_tokens(&s) else { return Ok(None) };
    if waits_only(ctx, &s, &toks)? || is_ro_background(ctx, &s)? {
        return Ok(None);
    }
    let home = h.scratch_homes().into_iter().next().unwrap_or_default();
    Ok(Some(Verdict::Block(format!("this call backgrounds a child and writes no liveness record — re-issue it with the record written at the launch, in this spelling: '<command> [<redirects>] & echo \"pid=$! run=<key>\" > {h}/<key>.run; wait; rm -f {h}/<key>.run'. The 'wait' keeps the call alive until the child exits, so a backgrounded call's completion notification means the producer finished, and the trailing 'rm -f' retracts the record at exactly that moment. That spelling of an allowlisted command is granted outright, so complying costs no permission decision. The record buys two things nothing else does: it is what gives the tracked-tree-mutation rule its reach, so a commit taken while this child is still writing is refused rather than silently taken; and it is what lets the next arrival tell whether the producer is still writing instead of guessing at a process table. An inline wait loop and a read-only pipeline owe no record; a wait behind a script name cannot be read here and takes the record like any launch. If you genuinely need an unrecorded launch, run it yourself with !<command>.", h = home))))
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `wait_no_producer`'s clause (c): a shell test,
// read past a leading `!`.
fn is_shell_test(seg: &str) -> bool {
    matches!(head_word(&command_word(&segment_core(seg))), "[" | "[[" | "test")
}

pub fn wait_no_producer(ctx: &Ctx) -> Decided {
    let h = ctx.host();
    if has_expansion(ctx.raw(ctx.cmd())?) {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if !(h.background || shell_backgrounds(&s)) {
        return Ok(None);
    }
    let Some((lead, cond, body, _)) = lone_loop(&s) else { return Ok(None) };
    if !wait_statements_clear(ctx, &lead) {
        return Ok(None);
    }
    let stated = |t: &str| ctx.segments(t).into_iter().filter(|g| !trim(g).is_empty()).collect::<Vec<String>>();
    let conds = stated(&cond);
    if conds.is_empty() || !conds.iter().all(|g| is_shell_test(g)) || !only_sleeps(ctx, &body) {
        return Ok(None);
    }
    if !h.live_run_records().is_empty() {
        return Ok(None);
    }
    let home = h.scratch_homes().into_iter().next().unwrap_or_default();
    Ok(Some(Verdict::Block(format!("this call backgrounds a wait on a file condition while no liveness record names a live producer — nothing this session is entitled to wait on can make that condition true, so the loop would hold the turn open until someone kills it. A wait launched to hold a turn open is the finding itself. Three lawful exits: (1) a shell producer is launched first, with its liveness record under {h}/ ('<command> [<redirects>] & echo \"pid=$! run=<key>\" > {h}/<key>.run; wait; rm -f {h}/<key>.run'), and the wait then passes — or wait on that recorded pid's liveness instead ('while kill -0 <pid> 2>/dev/null; do sleep 5; done'), which ends with the producer; (2) an Agent child is awaited by its completion notification, which needs no wait loop at all — where your harness delivers your report through a hand-back call, end the turn without that call and the child's hand-back arrives as your next turn; (3) a question to your caller is asked and the turn ended, never waited on. If you genuinely need this wait, run it yourself with !<command>.", h = home))))
}

fn wait_redirects_inert(ctx: &Ctx, seg: &str) -> bool {
    ctx.redirect_pairs(seg).iter().filter(|p| !p.is_empty()).all(|p| inert_target(strip_redirect_op(p)))
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `bounded_wait`'s clause (c): the loop condition
// is held to the read-only segment test plus the shell tests and `kill -0`.
fn is_wait_condition_segment(ctx: &Ctx, seg: &str, inert: bool) -> bool {
    if !inert {
        return false;
    }
    let cw = command_word(&segment_core(seg));
    match head_word(&cw) {
        "[" | "[[" | "test" => true,
        "kill" => grep_q("(^|[[:space:]])-0([[:space:]]|$)", &cw),
        _ => is_ro_segment(ctx.host(), &cw),
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `bounded_wait`'s clause (d): every segment
// after the loop, a statement or a piped reader, meets rule `ro_pipeline`'s own test.
fn is_wait_tail_segment(ctx: &Ctx, seg: &str, inert: bool) -> bool {
    if !inert {
        return false;
    }
    let core = segment_core(seg);
    let core = trim_start(&core);
    core.is_empty() || is_banner(core) || is_ro_segment(ctx.host(), core)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `background_no_record`'s exemption (2) statement
// test, which rule `wait_no_producer`'s clause (b) takes for a lead: every statement of the piece
// passes the tail's test and none ends in a bare `&`.
fn wait_statements_clear(ctx: &Ctx, piece: &str) -> bool {
    ctx.segments(piece)
        .iter()
        .filter(|g| !trim(g).is_empty())
        .all(|g| !shell_backgrounds(g) && is_wait_tail_segment(ctx, trim(g), wait_redirects_inert(ctx, g)))
}

// spec: guard-kit/SPEC.md §The generic ruleset — a skeleton carrying a `while`/`until` loop with
// exactly one balanced `do … done` span: what stands ahead of it, its condition, its body and what
// follows it, cut at the walk's own keywords.
fn lone_loop(s: &str) -> Option<(String, String, String, String)> {
    let toks = loop_tokens(s)?;
    let keyword = |w: &str| toks.iter().filter(|t| t.1 && t.2 == w).map(|t| t.3).collect::<Vec<usize>>();
    let (opens, closes) = (keyword("do"), keyword("done"));
    if opens.len() != 1 || closes.len() != 1 {
        return None;
    }
    let (open, close) = (opens[0], closes[0]);
    let (_, _, first, at) = toks.iter().find(|t| t.1 && matches!(t.2.as_str(), "until" | "while" | "for"))?;
    if first == "for" || *at > open {
        return None;
    }
    let joined = format!("{};", s.replace('\n', ";"));
    let part = |a: usize, b: usize| joined.get(a..b).unwrap_or("").to_string();
    Some((part(0, *at), part(at + first.len(), open), part(open + 2, close), part(close + 4, joined.len())))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the recorded launch's tail after its `&`:
// ` echo <quoted> > <path>; wait[; rm -f <path>]`, the quoted record either the literal
// `"pid=$! run=<key>"` or its `dq` placeholder. The key, the record path and the removed path.
fn launch_tail(t: &str, placeholder: bool) -> Option<(String, String, String)> {
    let b = t.as_bytes();
    let mut i = 0usize;
    let ws = |i: &mut usize| {
        let s = *i;
        while *i < b.len() && text::is_space(b[*i]) {
            *i += 1;
        }
        *i > s
    };
    let lit = |i: &mut usize, l: &str| {
        if t[*i..].starts_with(l) {
            *i += l.len();
            true
        } else {
            false
        }
    };
    let word = |i: &mut usize| -> Option<String> {
        let s = *i;
        while *i < b.len() && !text::is_space(b[*i]) && !b";&|<>\"'`$\\".contains(&b[*i]) {
            *i += 1;
        }
        (*i > s).then(|| t[s..*i].to_string())
    };
    let sep = |i: &mut usize| {
        ws(i);
        if *i < b.len() && (b[*i] == b';' || b[*i] == b'\n') {
            *i += 1;
            ws(i);
            true
        } else {
            false
        }
    };
    if !ws(&mut i) || !lit(&mut i, "echo") || !ws(&mut i) {
        return None;
    }
    let key = if placeholder {
        if !lit(&mut i, DQ_MARK) {
            return None;
        }
        String::new()
    } else {
        if !lit(&mut i, "\"pid=$! run=") {
            return None;
        }
        let s = i;
        while i < b.len() && (b[i].is_ascii_alphanumeric() || b"._-".contains(&b[i])) {
            i += 1;
        }
        if i == s {
            return None;
        }
        let k = t[s..i].to_string();
        if !lit(&mut i, "\"") {
            return None;
        }
        k
    };
    ws(&mut i);
    if !lit(&mut i, ">") {
        return None;
    }
    ws(&mut i);
    let path = word(&mut i)?;
    if !sep(&mut i) || !lit(&mut i, "wait") {
        return None;
    }
    ws(&mut i);
    let finish = |mut j: usize| {
        if j < b.len() && b[j] == b';' {
            j += 1;
        }
        while j < b.len() && text::is_space(b[j]) {
            j += 1;
        }
        j == b.len()
    };
    let mut j = i;
    if sep(&mut j) && lit(&mut j, "rm") && ws(&mut j) && lit(&mut j, "-f") && ws(&mut j) {
        if let Some(rm) = word(&mut j) {
            ws(&mut j);
            if finish(j) {
                return Some((key, path, rm));
            }
        }
    }
    finish(i).then(|| (key, path, String::new()))
}

// spec: guard-kit/SPEC.md §The generic ruleset — `^(.*[^&>])&<tail>`: the rightmost `&` that leaves
// a launch ending in neither `&` nor `>` and a tail that parses.
fn split_launch(s: &str, placeholder: bool) -> Option<(String, (String, String, String))> {
    let b = s.as_bytes();
    for at in (1..b.len()).rev() {
        if b[at] != b'&' || matches!(b[at - 1], b'&' | b'>') {
            continue;
        }
        if let Some(t) = launch_tail(&s[at + 1..], placeholder) {
            return Some((s[..at].to_string(), t));
        }
    }
    None
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `bounded_wait`'s arm (B), the recorded launch:
// exactly the canonical launch-record-wait spelling, its launched command granted, bounded and
// writing only gitignored or inert targets.
fn recorded_launch(ctx: &Ctx) -> Result<bool, Fault> {
    let h = ctx.host();
    let raw = ctx.raw(ctx.cmd())?;
    match raw.find("run=") {
        Some(i) if raw[i + 4..].contains(".run") => {}
        _ => return Ok(false),
    }
    let Some((launch, (key, path, rmpath))) = split_launch(raw, false) else { return Ok(false) };
    if !rmpath.is_empty() && rmpath != path {
        return Ok(false);
    }
    let want = h.lexical(&path);
    let record = format!("{}.run", key);
    if !h.scratch_homes().iter().any(|d| h.lexical(d) + "/" + &record == want) {
        return Ok(false);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if s.contains(['\'', '"']) {
        return Ok(false);
    }
    let Some((slaunch, (_, spath, _))) = split_launch(&s, true) else { return Ok(false) };
    let launch_cmd = Cmd::new(launch);
    let ls = ctx.view(&launch_cmd, SqDqHd)?;
    if slaunch != ls || spath != path {
        return Ok(false);
    }
    if ctx.view(ctx.cmd(), Sq)?.matches('$').count() != 1 {
        return Ok(false);
    }
    if ls.contains([';', '|', '\n']) || ls.contains("<<") || ls.contains("&&") || shell_backgrounds(&ls) {
        return Ok(false);
    }
    if ctx.redirect_pairs(&ls).iter().any(|p| dups_onto_file(p)) {
        return Ok(false);
    }
    let Some(lv) = ctx.dequoted(&launch_cmd)? else { return Ok(false) };
    let Ok(ws) = unredirected_words(&ls, &lv) else { return Ok(false) };
    let ws = unsentinel(&ws);
    if !h.allow_inners().iter().any(|i| allow_match(&ws, i)) {
        return Ok(false);
    }
    if !matches!(ctx.under("grant_path_slot", |g| slot_reach(g, &launch_cmd, true))?, Reach::Clean) {
        return Ok(false);
    }
    if ctx.under("rm_tracked", |r| rm_tracked_reach(r, ctx.cmd()))?.is_some() {
        return Ok(false);
    }
    if ctx.under("script_interpreter", |r| interpreter_reach(r, ctx.cmd()))?.is_some() {
        return Ok(false);
    }
    let mut targets: Vec<String> = Vec::new();
    for tgt in valued_targets(ctx, &launch_cmd)? {
        if inert_value(&tgt) {
            continue;
        }
        if tgt.contains(SQ_MARK) || tgt.contains(DQ_MARK) {
            return Ok(false);
        }
        targets.push(tgt);
    }
    targets.push(path);
    Ok(targets.iter().all(|t| h.ignored(t)))
}

pub fn bounded_wait(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    if has_substitution(raw) || noclobber(&ctx.view(ctx.cmd(), SqDqHd)?) || refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    if recorded_launch(ctx)? {
        return Ok(Some(Verdict::Allow("recorded launch of an allowlisted command (shell-guard auto-allow)".to_string())));
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if s.contains(['\'', '"']) || shell_backgrounds(&s) {
        return Ok(None);
    }
    let Some((lead, cond, body, tail)) = lone_loop(&s) else { return Ok(None) };
    if !trim(&lead).is_empty() {
        return Ok(None);
    }
    let inert = valued_targets(ctx, ctx.cmd())?.iter().all(|t| inert_value(t));
    let mut sleeps = 0usize;
    for seg in ctx.segments(&body) {
        let seg = trim(&seg);
        if seg.is_empty() {
            continue;
        }
        let bt = words(seg);
        if bt.len() != 2 || bt[0] != "sleep" || !bt[1].chars().all(|c| c.is_ascii_digit() || c == '.') {
            return Ok(None);
        }
        sleeps += 1;
    }
    if sleeps == 0 {
        return Ok(None);
    }
    let mut conds = 0usize;
    for seg in ctx.segments(&cond) {
        let seg = trim_start(&seg);
        if seg.is_empty() {
            continue;
        }
        if !is_wait_condition_segment(ctx, seg, inert) {
            return Ok(None);
        }
        conds += 1;
    }
    if conds == 0 {
        return Ok(None);
    }
    for seg in ctx.segments(&tail) {
        let seg = trim_start(&seg);
        if !seg.is_empty() && !is_wait_tail_segment(ctx, seg, inert) {
            return Ok(None);
        }
    }
    if !ro_forms_clear(ctx, ctx.cmd())? {
        return Ok(None);
    }
    Ok(Some(Verdict::Allow("bounded in-turn wait (shell-guard auto-allow)".to_string())))
}
