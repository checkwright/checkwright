// spec: guard-kit/SPEC.md §The generic ruleset — the producer rules: a git write under a live
// recorded producer, a launch that records nothing, and the two waits granted outright.
use super::grants::{slot_reach, Reach};
use super::reach::{interpreter_reach, refused, rm_tracked_reach};
use super::{
    command_word, git_subcommand, has_expansion, has_substitution, inert_target, is_banner, is_ro_segment,
    loop_span, redirect_targets, ro_forms_clear, segment_core, shell_backgrounds, strip_redirect_op,
    unredirected_words, GitWalk,
};
use crate::guard::allow_match;
use crate::guard::engine::{Cmd, Ctx, Decided, Fault, Verdict};
use crate::guard::reader::View::{Sq, SqDqHd};
use crate::guard::text::{self, grep_q, head_word, trim, trim_start, unsentinel, words};

pub fn git_mutation_under_producer(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    if !raw.contains("git") || has_expansion(raw) {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    let mut writes: Vec<String> = Vec::new();
    for seg in ctx.segments(&s) {
        let Some(sub) = git_subcommand(&command_word(&seg), GitWalk::Sub) else { continue };
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
    if !redirect_targets(s).iter().all(|t| t == "/dev/null" || t.strip_prefix('&').is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit()))) {
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

pub fn background_no_record(ctx: &Ctx) -> Decided {
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
    let Some(span) = loop_span(&s) else { return Ok(None) };
    if span.iter().any(|(depth, _, _)| *depth >= 1) || is_ro_background(ctx, &s)? {
        return Ok(None);
    }
    let home = h.scratch_homes().into_iter().next().unwrap_or_default();
    Ok(Some(Verdict::Block(format!("this call backgrounds a child and writes no liveness record — re-issue it with the record written at the launch, in this spelling: '<command> [<redirects>] & echo \"pid=$! run=<key>\" > {h}/<key>.run; wait; rm -f {h}/<key>.run'. The 'wait' keeps the call alive until the child exits, so a backgrounded call's completion notification means the producer finished, and the trailing 'rm -f' retracts the record at exactly that moment. That spelling of an allowlisted command is granted outright, so complying costs no permission decision. The record buys two things nothing else does: it is what gives the tracked-tree-mutation rule its reach, so a commit taken while this child is still writing is refused rather than silently taken; and it is what lets the next arrival tell whether the producer is still writing instead of guessing at a process table. An inline wait loop and a read-only pipeline owe no record; a wait behind a script name cannot be read here and takes the record like any launch. If you genuinely need an unrecorded launch, run it yourself with !<command>.", h = home))))
}

fn wait_redirects_inert(ctx: &Ctx, seg: &str) -> bool {
    ctx.redirect_pairs(seg).iter().filter(|p| !p.is_empty()).all(|p| inert_target(strip_redirect_op(p)))
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `bounded_wait`'s clause (c): the loop condition
// is held to the read-only segment test plus the shell tests and `kill -0`.
fn is_wait_condition_segment(ctx: &Ctx, seg: &str) -> bool {
    if !wait_redirects_inert(ctx, seg) {
        return false;
    }
    let cw = command_word(&segment_core(seg));
    match head_word(&cw) {
        "[" | "[[" | "test" => true,
        "kill" => grep_q("(^|[[:space:]])-0([[:space:]]|$)", &cw),
        _ => is_ro_segment(ctx.host(), &cw),
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `bounded_wait`'s clause (d): every statement
// after the loop meets rule `ro_pipeline`'s own test.
fn is_wait_tail_segment(ctx: &Ctx, seg: &str) -> bool {
    if !wait_redirects_inert(ctx, seg) {
        return false;
    }
    let core = segment_core(seg);
    let core = trim_start(&core);
    core.is_empty() || is_banner(core) || is_ro_segment(ctx.host(), core)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the loop view split as
// `^[[:space:]]*(until|while)[[:space:]]+(.+)[[:space:]]+do[[:space:]]+(.+)[[:space:]]+done([[:space:];].*)?$`,
// each group the longest it can be.
fn loop_parts(view: &str) -> Option<(String, String, String)> {
    let sp = |c: u8| text::is_space(c);
    let b = view.as_bytes();
    let mut i = 0usize;
    while i < b.len() && sp(b[i]) {
        i += 1;
    }
    if !(view[i..].starts_with("until") || view[i..].starts_with("while")) {
        return None;
    }
    i += 5;
    let ws_start = i;
    while i < b.len() && sp(b[i]) {
        i += 1;
    }
    if i == ws_start {
        return None;
    }
    let rest = &view[i..];
    let rb = rest.as_bytes();
    let keyword_after = |at: usize, kw: &str| -> Option<usize> {
        let mut j = at;
        if j >= rb.len() || !sp(rb[j]) {
            return None;
        }
        while j < rb.len() && sp(rb[j]) {
            j += 1;
        }
        rest[j..].starts_with(kw).then_some(j + kw.len())
    };
    for p in (1..rb.len()).rev() {
        let Some(after_do) = keyword_after(p, "do") else { continue };
        if after_do >= rb.len() || !sp(rb[after_do]) {
            continue;
        }
        let mut q0 = after_do;
        while q0 < rb.len() && sp(rb[q0]) {
            q0 += 1;
        }
        for q in (q0 + 1..rb.len()).rev() {
            let Some(after_done) = keyword_after(q, "done") else { continue };
            let tail = &rest[after_done..];
            if !tail.is_empty() && !tail.starts_with(|c: char| c.is_ascii_whitespace() || c == ';' || c == '\x0b') {
                continue;
            }
            return Some((rest[..p].to_string(), rest[q0..q].to_string(), tail.to_string()));
        }
    }
    None
}

// spec: guard-kit/SPEC.md §The generic ruleset — the recorded launch's tail after its `&`:
// ` echo <quoted> > <path>; wait[; rm -f <path>]`, the quoted record either the literal
// `"pid=$! run=<key>"` or its `DQ` placeholder. The key, the record path and the removed path.
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
        if !lit(&mut i, "DQ") {
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
    if grep_q(">&[^0-9-]", &ls) {
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
    for pair in ctx.redirect_pairs(&ls) {
        if pair.is_empty() {
            continue;
        }
        let tgt = strip_redirect_op(&pair);
        if inert_target(tgt) {
            continue;
        }
        if tgt.contains("SQ") || tgt.contains("DQ") {
            return Ok(false);
        }
        targets.push(tgt.to_string());
    }
    targets.push(path);
    Ok(targets.iter().all(|t| h.ignored(t)))
}

pub fn bounded_wait(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    if has_substitution(raw) || refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    if recorded_launch(ctx)? {
        return Ok(Some(Verdict::Allow("recorded launch of an allowlisted command (shell-guard auto-allow)".to_string())));
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if s.contains(['\'', '"']) || shell_backgrounds(&s) {
        return Ok(None);
    }
    let Some(span) = loop_span(&s) else { return Ok(None) };
    let first = span.first().map_or("", |(_, _, t)| t.as_str());
    if !matches!(first, "until" | "while") {
        return Ok(None);
    }
    let count = |w: &str| span.iter().filter(|(_, _, t)| t == w).count();
    if count("do") != 1 || count("done") != 1 {
        return Ok(None);
    }
    let view = format!("{};", s.replace('\n', ";"));
    let Some((cond, body, tail)) = loop_parts(&view) else { return Ok(None) };
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
        if !is_wait_condition_segment(ctx, seg) {
            return Ok(None);
        }
        conds += 1;
    }
    if conds == 0 {
        return Ok(None);
    }
    for seg in ctx.segments(&tail) {
        let seg = trim_start(&seg);
        if !seg.is_empty() && !is_wait_tail_segment(ctx, seg) {
            return Ok(None);
        }
    }
    if !ro_forms_clear(ctx, ctx.cmd())? {
        return Ok(None);
    }
    Ok(Some(Verdict::Allow("bounded in-turn wait (shell-guard auto-allow)".to_string())))
}
