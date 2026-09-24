// spec: guard-kit/SPEC.md §The generic ruleset — the grants and the rules sharing their tests: an
// allow is given only where every clause of its test holds, and every other failure declines.
use super::reach::refused;
use super::{
    has_substitution, inert_target, is_banner, is_bare_allow, is_ro_segment, is_ro_xargs,
    redirect_targets, ro_forms_clear, segment_core, shell_backgrounds, strip_redirect_op,
    unredirected_words,
};
use crate::guard::allow_match;
use crate::guard::engine::{Cmd, Ctx, Decided, Fault, Verdict};
use crate::guard::reader::View::{Hdq, SqDqHd, SqHdq};
use crate::guard::reader::HD_MARK;
use crate::guard::text::{self, grep_q, head_word, trim, trim_start, unsentinel, words};
use crate::walk;

fn allow(m: &str) -> Decided {
    Ok(Some(Verdict::Allow(m.to_string())))
}

fn block(m: impl Into<String>) -> Decided {
    Ok(Some(Verdict::Block(m.into())))
}

pub fn truncate_scratch(ctx: &Ctx) -> Decided {
    let cmd = ctx.view(ctx.cmd(), SqDqHd)?;
    if !text::matches(
        "^[[:space:]]*:([[:space:]]+[0-9]*>>?[[:space:]]*[^[:space:]&|;<]+)+[[:space:]]*$",
        &cmd,
    ) {
        return Ok(None);
    }
    let mut all_ignored = true;
    for m in text::grep_o("[0-9]*>>?[[:space:]]*[^[:space:]&|;<]+", &cmd) {
        let tgt = strip_redirect_op(&m);
        if tgt.is_empty() {
            continue;
        }
        if !ctx.host().ignored(tgt) {
            all_ignored = false;
            break;
        }
    }
    if refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    if all_ignored {
        return allow("truncate gitignored scratch (shell-guard auto-allow)");
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `emitter_write`'s clause (d), read by it and by
// rule `append_scratch`: the substitution declines run on the `hdq` view, since a quoted-delimiter
// heredoc body cannot substitute while every other region can.
fn emitter_unmodelled(ctx: &Ctx, c: &Cmd) -> Result<bool, Fault> {
    let live = ctx.view(c, Hdq)?;
    Ok(grep_q(r"\$\(|<\(|>\(", &live) || live.contains('`'))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the single-statement test: one segment plus exactly
// the residue its own openers produce.
fn only_heredoc_residue(ctx: &Ctx, s: &str) -> bool {
    let segs = ctx.segments(s);
    if segs.is_empty() {
        return false;
    }
    let at = |i: usize| segs.get(i).map_or("", |s| trim_start(s));
    let mut i = 1usize;
    for t in ctx.heredoc_terms(&segs[0]) {
        let mut seg = at(i);
        if seg == HD_MARK {
            i += 1;
            seg = at(i);
        }
        if seg != t {
            return false;
        }
        i += 1;
    }
    i == segs.len()
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `emitter_write`'s clauses (0), (a) and (c) and
// its quoted-target decline on one statement: every target that is neither `/dev/null` nor an
// fd-dup, or `None` when the statement is no emitter write the rule can test.
fn emitter_targets(ctx: &Ctx, s: &str) -> Option<Vec<String>> {
    if shell_backgrounds(s) || !only_heredoc_residue(ctx, s) {
        return None;
    }
    let lead = head_word(trim_start(s.split('\n').next().unwrap_or("")));
    if !ctx.host().append_bins.iter().any(|b| b == lead) {
        return None;
    }
    let mut out = Vec::new();
    for pair in ctx.redirect_pairs(s) {
        if pair.is_empty() {
            continue;
        }
        let p = pair.trim_start_matches(|c: char| c.is_ascii_digit());
        let tgt = p.strip_prefix(">>").or_else(|| p.strip_prefix('>')).unwrap_or(p);
        let tgt = trim_start(tgt);
        if inert_target(tgt) {
            continue;
        }
        if tgt.contains(['"', '\'']) {
            return None;
        }
        out.push(tgt.to_string());
    }
    (!out.is_empty()).then_some(out)
}

pub fn append_scratch(ctx: &Ctx) -> Decided {
    if emitter_unmodelled(ctx, ctx.cmd())? {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    let Some(targets) = emitter_targets(ctx, &s) else { return Ok(None) };
    if !targets.iter().all(|t| ctx.host().ignored(t)) {
        return Ok(None);
    }
    if refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    allow("write to gitignored scratch (shell-guard auto-allow)")
}

pub fn ro_pipeline(ctx: &Ctx) -> Decided {
    let h = ctx.host();
    let raw = ctx.raw(ctx.cmd())?;
    if has_substitution(raw) {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if s.contains(['\'', '"']) || grep_q(r"(&&|\|\||;|&)", &s) {
        return Ok(None);
    }
    for tgt in redirect_targets(&s) {
        if !tgt.is_empty() && !(tgt == "/dev/null" || is_fd_dup(&tgt)) {
            return Ok(None);
        }
    }
    let segs = ctx.segments(&s);
    let mut reads = 0usize;
    for (i, seg) in segs.iter().enumerate() {
        let seg = trim_start(seg);
        if seg.is_empty() || is_banner(seg) {
            continue;
        }
        if head_word(seg) == "xargs" && !is_ro_xargs(h, seg) {
            return Ok(None);
        }
        if !is_ro_segment(h, seg) && !(i == 0 && segs.len() > 1 && is_bare_allow(h, seg).0) {
            return Ok(None);
        }
        reads += 1;
    }
    if reads == 0 || !ro_forms_clear(ctx, ctx.cmd())? || refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    allow("read-only search pipeline (shell-guard auto-allow)")
}

fn is_fd_dup(t: &str) -> bool {
    t.strip_prefix('&').is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit()))
}

pub fn allowlist_chain(ctx: &Ctx) -> Decided {
    let h = ctx.host();
    let inners = h.allow_inners();
    if inners.is_empty() {
        return Ok(None);
    }
    let skel = ctx.view(ctx.cmd(), SqDqHd)?;
    let segs = ctx.segments(&skel);
    let lead = trim(&segs[0]).to_string();
    let lead_core = segment_core(&lead);
    let (bare, via_star) = is_bare_allow(h, &lead);
    if !bare {
        return Ok(None);
    }
    if via_star && lead != lead_core && h.append_bins.iter().any(|e| head_word(&lead_core) == e) {
        return Ok(None);
    }
    let steer = format!("run '{}' bare — it's a statically allowlisted command, but the decoration (chaining or a redirect) leaves a segment nothing grants, so the whole call falls off the match path and costs an out-of-band permission decision. Run the allowlisted command on its own; issue the rest as separate calls.", lead_core);
    if lead != lead_core {
        return block(steer);
    }
    for seg in segs.iter().skip(1) {
        let seg = trim(seg);
        if seg.is_empty() {
            continue;
        }
        if shell_backgrounds(seg) || !inners.iter().any(|p| allow_match(seg, p)) {
            return block(steer);
        }
    }
    Ok(None)
}

pub enum Reach {
    Hit(String),
    Clean,
    Undecided,
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `grant_path_slot`'s test, a block for it and a
// predicate for rules `abs_script`, `brace_glyph` and `bounded_wait`; `predicate` reads a segment
// carrying a quoted statement separator whole rather than skipping it.
pub fn slot_reach(ctx: &Ctx, c: &Cmd, predicate: bool) -> Result<Reach, Fault> {
    let live = ctx.view(c, SqHdq)?;
    if live.contains('$') || live.contains("<(") || live.contains(">(") || ctx.raw(c)?.contains('`') {
        return Ok(Reach::Undecided);
    }
    let inners = ctx.host().allow_inners();
    if inners.is_empty() {
        return Ok(Reach::Clean);
    }
    let s = ctx.view(c, SqDqHd)?;
    let Some(v) = ctx.dequoted(c)? else { return Ok(Reach::Undecided) };
    let segs = ctx.segments(&s);
    let dsegs = ctx.segments(&v);
    if segs.len() != dsegs.len() {
        return Ok(Reach::Undecided);
    }
    let mut undecided = false;
    for (seg, dseg) in segs.iter().zip(&dsegs) {
        let mut dseg = dseg.clone();
        if dseg.contains(['\x03', '\x04', '\x05']) {
            if !predicate {
                undecided = true;
                continue;
            }
            dseg = dseg.replace('\x03', ";").replace('\x04', "|").replace('\x05', "&");
        }
        match slot_segment(ctx, seg, &dseg, &inners) {
            Reach::Hit(m) => return Ok(Reach::Hit(m)),
            Reach::Undecided => undecided = true,
            Reach::Clean => {}
        }
    }
    Ok(if undecided { Reach::Undecided } else { Reach::Clean })
}

fn slot_segment(ctx: &Ctx, sw: &str, dw: &str, inners: &[String]) -> Reach {
    let ws = match unredirected_words(sw, dw) {
        Ok(w) => w,
        Err(1) => return Reach::Clean,
        Err(_) => return Reach::Undecided,
    };
    let sv = ctx.harness_view(&ws).replace('\\', "");
    let rv = sv.replace('\x01', " ").replace('\x02', "\t");
    for inner in inners {
        let pat = inner.replace(":*", "*");
        if !pat.contains('*') || pat.contains(['?', '[']) || !allow_match(&rv, &pat) {
            continue;
        }
        let pieces: Vec<&str> = pat.split('*').collect();
        let mut stars: Vec<(String, bool, bool, String)> = Vec::new();
        let mut j = 0usize;
        for piece in &pieces[..pieces.len() - 1] {
            j += piece.len();
            let left = pat[..j].rsplit([' ', '\t', '\n']).next().unwrap_or("");
            let right = head_word(&pat[j + 1..]);
            let token = format!("{}*{}", left, right);
            let slot = token.contains('/');
            let rlit = right.split('*').next().unwrap_or("").to_string();
            stars.push((token, slot, left.is_empty(), rlit));
            j += 1;
        }
        let Some(caps) = capture(&rv, &pieces) else { continue };
        let mut off = 0usize;
        for (g, (token, slot, start, rlit)) in stars.iter().enumerate() {
            off += pieces[g].len();
            let len = caps[g];
            if *slot {
                if let Some(reach) = slot_capture(&sv, off, len, token, *start, rlit) {
                    return Reach::Hit(format!(
                        "'Bash({})' matches this command only through a path slot that reaches {}",
                        inner, reach
                    ));
                }
            }
            off += len;
        }
    }
    Reach::Clean
}

// spec: guard-kit/SPEC.md §The generic ruleset — `^lit0(.*)lit1(.*)…$`, each group the longest it can
// be, left to right; the capture lengths, or `None` when the subject does not match.
fn capture(s: &str, lits: &[&str]) -> Option<Vec<usize>> {
    fn go(s: &[u8], at: usize, lits: &[&[u8]], out: &mut Vec<usize>) -> bool {
        let Some((next, more)) = lits.split_first() else {
            return at == s.len();
        };
        for end in (at..=s.len()).rev() {
            if s[end..].starts_with(next) {
                out.push(end - at);
                if go(s, end + next.len(), more, out) {
                    return true;
                }
                out.pop();
            }
        }
        false
    }
    let b = s.as_bytes();
    let lits: Vec<&[u8]> = lits.iter().map(|l| l.as_bytes()).collect();
    let (first, rest) = lits.split_first()?;
    if !b.starts_with(first) {
        return None;
    }
    let mut out = Vec::new();
    go(b, first.len(), rest, &mut out).then_some(out)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the cleanness test on one word: no `..` component,
// and the text opening the path is neither rooted nor home-relative.
fn unclean_word(full: &str, lead: &str) -> Option<String> {
    if format!("/{}/", full).contains("/../") {
        return Some(format!("'{}', which carries a '..' component", full));
    }
    if walk::path_root(lead).is_some() {
        return Some(format!("'{}', an absolute path", full));
    }
    if lead.starts_with('~') {
        return Some(format!("'{}', a home-relative path", full));
    }
    None
}

// spec: guard-kit/SPEC.md §The generic ruleset — one path-slot capture: the first word tested in the
// full shell word carrying it, and every later non-option word re-matching the slot's own token.
fn slot_capture(sv: &str, off: usize, len: usize, token: &str, start: bool, right: &str) -> Option<String> {
    let b = sv.as_bytes();
    let cap = String::from_utf8_lossy(&b[off..off + len]).into_owned();
    let firstw = head_word(&cap).to_string();
    let pre_all = String::from_utf8_lossy(&b[..off]).into_owned();
    let pre = pre_all.rsplit([' ', '\t', '\n', '\x0b', '\x0c', '\r']).next().unwrap_or("").to_string();
    let post_all = String::from_utf8_lossy(&b[off..]).into_owned();
    let post = head_word(&post_all).to_string();
    let word = format!("{}{}", pre, post).replace('\x01', " ").replace('\x02', "\t");
    let lead = if firstw.is_empty() && start { right.to_string() } else { firstw.clone() };
    if let Some(r) = unclean_word(&word, &lead) {
        return Some(r);
    }
    let mut rest = cap[firstw.len()..].to_string();
    if rest.bytes().any(|c| !text::is_space(c)) && !rest.ends_with(|c: char| c.is_ascii_whitespace() || c == '\x0b') {
        let ext = String::from_utf8_lossy(&b[off + len..]).into_owned();
        rest.push_str(head_word(&ext));
    }
    for w in words(&rest) {
        if w.starts_with('-') {
            continue;
        }
        let w = w.replace('\x01', " ").replace('\x02', "\t");
        if !allow_match(&w, token) {
            return Some(format!("'{}', a second operand outside the slot", w));
        }
        if let Some(r) = unclean_word(&w, &w) {
            return Some(r);
        }
    }
    None
}

// spec: guard-kit/SPEC.md §The generic ruleset — the grant test on a rewritten command, rule
// `worktree_confinement`'s predicate first: every segment matches a committed allow pattern and rule
// `grant_path_slot`'s test finds no reach and can decide.
pub fn rewrite_granted(ctx: &Ctx, c: &Cmd) -> Result<bool, Fault> {
    if refused(ctx, c)? {
        return Ok(false);
    }
    let inners = ctx.host().allow_inners();
    if inners.is_empty() {
        return Ok(false);
    }
    ctx.view(c, SqDqHd)?;
    let Some(v) = ctx.dequoted(c)? else { return Ok(false) };
    for seg in ctx.segments(&v) {
        if shell_backgrounds(&seg) {
            return Ok(false);
        }
        let seg = unsentinel(&seg);
        let seg = trim(&seg);
        if seg.is_empty() {
            continue;
        }
        if !inners.iter().any(|i| allow_match(seg, i)) {
            return Ok(false);
        }
    }
    Ok(matches!(ctx.under("grant_path_slot", |g| slot_reach(g, c, true))?, Reach::Clean))
}

pub fn grant_path_slot(ctx: &Ctx) -> Decided {
    if !ctx.raw(ctx.cmd())?.contains('/') {
        return Ok(None);
    }
    match slot_reach(ctx, ctx.cmd(), false)? {
        Reach::Hit(reach) => block(format!("don't reach past a committed grant's path slot — {}. A Bash rule's '*' spans '/', '..' and whole words, so a grant written for one directory reaches paths its author never named. Spell the path inside the pattern's reach, or, if you genuinely need this command, run it yourself with !<command>.", reach)),
        _ => Ok(None),
    }
}

pub fn emitter_write(ctx: &Ctx) -> Decided {
    if !ctx.raw(ctx.cmd())?.contains('>') || emitter_unmodelled(ctx, ctx.cmd())? {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if shell_backgrounds(&s) {
        return Ok(None);
    }
    let h = ctx.host();
    let stmts = ctx.residue_statements(&s);
    if stmts.len() > 1 {
        for stmt in &stmts {
            let Some(targets) = emitter_targets(ctx, stmt) else { continue };
            let lead = head_word(trim_start(stmt));
            let unignored: Vec<&String> = targets.iter().filter(|t| !h.ignored(t)).collect();
            if unignored.is_empty() {
                return block(format!("issue the '{}' write to '{}' as its own call, and the rest of this command as a separate one: alone, that write is granted with no permission decision, while compounded it makes the whole call one the harness decides out of band. If you genuinely need the compound, run it yourself with !<command>.", lead, targets.join(" ")));
            }
            if targets.iter().any(|t| t.starts_with("/dev/")) {
                continue;
            }
            let un: Vec<&str> = unignored.iter().map(|s| s.as_str()).collect();
            return block(format!("write '{}' with the Write or Edit tool as its own call instead of the '{}' redirect, and issue the rest of this command as a separate one: git does not ignore the target, so no rule grants the write, and compounding it takes the whole call off the match path. If you genuinely need the compound, run it yourself with !<command>.", un.join(" "), lead));
        }
        return Ok(None);
    }
    let Some(targets) = emitter_targets(ctx, &s) else { return Ok(None) };
    if targets.iter().any(|t| t.starts_with("/dev/")) {
        return Ok(None);
    }
    for tgt in &targets {
        if h.ignored(tgt) {
            continue;
        }
        return block(format!("don't write '{}' through a redirect: git does not ignore it, so no rule grants the write. Use the Write or Edit tool for a file, or the capture arm that owns the surface when the target is one — the tool is reviewable where a redirect is not, and the arm keeps the surface's grammar. If you genuinely need the redirect, run it yourself with !<command>.", tgt));
    }
    Ok(None)
}
