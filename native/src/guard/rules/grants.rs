// spec: guard-kit/SPEC.md §The generic ruleset — the grants and the rules sharing their tests: an
// allow is given only where every clause of its test holds, and every other failure declines.
use super::reach::{interpreter_reach, refused, rm_tracked_reach};
use super::{
    has_substitution, inert_target, is_banner, is_bare_allow, is_ro_segment, is_ro_xargs,
    redirect_targets, ro_forms_clear, ro_forms_clear_except, ro_forms_kit, segment_core,
    shell_backgrounds, strip_redirect_op, unredirected_words, xargs_command_index,
};
use crate::guard::allow_match;
use crate::guard::engine::{Cmd, Ctx, Decided, Fault, Verdict};
use crate::guard::host::{self, Host};
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
    if s.contains(['\'', '"']) || shell_backgrounds(&s) {
        return Ok(None);
    }
    for tgt in redirect_targets(&s) {
        if !tgt.is_empty() && !(tgt == "/dev/null" || is_fd_dup(&tgt)) {
            return Ok(None);
        }
    }
    let decorated = ctx.segments(&s).len() > 1;
    let mut reads = 0usize;
    for stmt in ctx.statements(&s) {
        for (i, seg) in ctx.pipes(&stmt).iter().enumerate() {
            let seg = trim_start(seg);
            if seg.is_empty() || is_banner(seg) {
                continue;
            }
            if head_word(seg) == "xargs" && !is_ro_xargs(h, seg) {
                return Ok(None);
            }
            if !is_ro_segment(h, seg) && !(i == 0 && decorated && is_bare_allow(h, seg).0) {
                return Ok(None);
            }
            reads += 1;
        }
    }
    if reads == 0 || !ro_forms_clear(ctx, ctx.cmd())? || refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    allow("read-only search pipeline (shell-guard auto-allow)")
}

fn is_fd_dup(t: &str) -> bool {
    t.strip_prefix('&').is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit()))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the bounds test's clause (a): relative, not
// home-relative, and carrying no `..` and no `.git` component.
fn contained(word: &str) -> bool {
    !word.is_empty()
        && walk::path_root(word).is_none()
        && !word.starts_with('~')
        && !word.split('/').any(|c| c == ".." || c.eq_ignore_ascii_case(".git"))
}

fn components(p: &str) -> Vec<&str> {
    p.split('/').filter(|c| !c.is_empty() && *c != ".").collect()
}

// spec: guard-kit/SPEC.md §The generic ruleset — the `bounded_write` rule's bounds test on one
// target word; a glob in the last component moves clauses (b) to (f) onto its parent directory.
pub(super) fn bounded(h: &Host, word: &str, removed: bool) -> bool {
    if !contained(word) {
        return false;
    }
    let comps = components(word);
    let glob = |c: &str| c.contains(['*', '?', '[']);
    let Some((last, parents)) = comps.split_last() else { return false };
    if parents.iter().any(|c| glob(c)) {
        return false;
    }
    let (read, strict) = if glob(last) { (parents, 0) } else { (&comps[..], 1) };
    let inside = h.scratch_dirs.iter().filter(|m| walk::path_root(m).is_none()).any(|m| {
        let mc = components(m);
        !mc.is_empty() && read.len() >= mc.len() + strict && read[..mc.len()] == mc[..]
    });
    let path = read.join("/");
    if !inside || !h.ignored(&path) || h.tracked(&path) {
        return false;
    }
    if reaches_link(std::path::Path::new("."), read) {
        return false;
    }
    let t = h.lexical(&path);
    !removed || !h.live_run_paths().iter().any(|r| host::under(&h.lexical(r), &t))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the bounds test's clause (e): an existing component
// of the path under `base`, the path included, is a symbolic link.
fn reaches_link(base: &std::path::Path, comps: &[&str]) -> bool {
    let mut p = base.to_path_buf();
    for c in comps {
        p.push(c);
        match std::fs::symlink_metadata(&p) {
            Ok(m) if m.file_type().is_symlink() => return true,
            Ok(_) => {}
            Err(_) => return false,
        }
    }
    false
}

enum Redir {
    Out,
    Dup,
    In,
}

// spec: guard-kit/SPEC.md §The generic ruleset — a word's redirect operator and its length; `Err`
// on a heredoc or herestring opener and on an operator inside a word, which the rule declines.
fn redirect_op(w: &str) -> Result<Option<(usize, Redir)>, ()> {
    let d = w.bytes().take_while(u8::is_ascii_digit).count();
    let r = &w[d..];
    Ok(Some(if d == 0 && r.starts_with("&>>") {
        (3, Redir::Out)
    } else if d == 0 && r.starts_with("&>") {
        (2, Redir::Out)
    } else if r.starts_with("<<") {
        return Err(());
    } else if r.starts_with("<&") || r.starts_with(">&") {
        (d + 2, Redir::Dup)
    } else if r.starts_with("<>") || r.starts_with(">>") {
        (d + 2, Redir::Out)
    } else if r.starts_with('>') {
        (d + 1, Redir::Out)
    } else if r.starts_with('<') {
        (d + 1, Redir::In)
    } else if w.contains(['<', '>']) {
        return Err(());
    } else {
        return Ok(None);
    }))
}

// spec: guard-kit/SPEC.md §The generic ruleset — one segment read as rule `grant_path_slot` reads
// it: its command words and its output redirect targets, dequoted with backslashes removed.
fn write_words(sw: &str, dw: &str) -> Option<(Vec<String>, Vec<String>)> {
    let (s, d) = (words(sw), words(dw));
    if s.len() != d.len() {
        return None;
    }
    let clean = |w: &str| unsentinel(w).replace('\\', "");
    let (mut cmd, mut out) = (Vec::new(), Vec::new());
    let mut k = 0usize;
    while k < s.len() {
        let Some((n, kind)) = redirect_op(s[k]).ok()? else {
            cmd.push(clean(d[k]));
            k += 1;
            continue;
        };
        if !d[k].starts_with(&s[k][..n]) {
            return None;
        }
        let tgt = if s[k].len() > n {
            &d[k][n..]
        } else {
            k += 1;
            *d.get(k)?
        };
        k += 1;
        match kind {
            Redir::In => {}
            Redir::Dup if tgt == "-" || (!tgt.is_empty() && tgt.bytes().all(|b| b.is_ascii_digit())) => {}
            Redir::Dup => return None,
            Redir::Out if inert_target(&clean(tgt)) => {}
            Redir::Out => out.push(clean(tgt)),
        }
    }
    Some((cmd, out))
}

struct Grammar {
    flags: &'static [&'static str],
    valued: &'static [&'static str],
    optional: &'static [&'static str],
}

// spec: guard-kit/SPEC.md §The generic ruleset — the `bounded_write` rule's operand grammar, one row
// per write utility; `find` is read by its own walk.
fn grammar(bin: &str) -> Option<Grammar> {
    let g = |flags, valued, optional| Some(Grammar { flags, valued, optional });
    match bin {
        "mkdir" => g(&["-p", "-v", "--parents", "--verbose"], &["-m", "--mode"], &[]),
        "touch" => g(&["-a", "-c", "-h", "-m", "--no-create"], &["-d", "-t", "-r", "--date", "--reference"], &[]),
        "rm" => g(&["-f", "-r", "-R", "-d", "-v", "--force", "--recursive", "--dir", "--verbose"], &[], &[]),
        "rmdir" => g(&["-v", "--verbose", "--ignore-fail-on-non-empty"], &[], &[]),
        "mv" => g(
            &["-f", "-n", "-v", "-T", "-u", "--force", "--no-clobber", "--verbose", "--no-target-directory", "--update"],
            &["-t", "--target-directory"],
            &[],
        ),
        "cp" => g(
            &[
                "-r", "-R", "-a", "-p", "-P", "-f", "-n", "-v", "-T", "-u", "--recursive", "--archive",
                "--no-dereference", "--force", "--no-clobber", "--verbose", "--no-target-directory", "--update",
                "--parents",
            ],
            &["-t", "--target-directory"],
            &["--preserve"],
        ),
        "tee" => g(&["-a", "-i", "--append", "--ignore-interrupts"], &[], &[]),
        _ => None,
    }
}

type Parsed = (Vec<(&'static str, Option<String>)>, Vec<String>);

// spec: guard-kit/SPEC.md §The generic ruleset — the option walk: a long option by its name or an
// unambiguous prefix, a short cluster of no-argument letters with a valued letter last, `--` ending
// the options; any other option word withholds.
fn parse_opts(g: &Grammar, ws: &[String]) -> Option<Parsed> {
    let (mut opts, mut ops) = (Vec::new(), Vec::new());
    let (mut i, mut ended) = (0usize, false);
    while i < ws.len() {
        let w = &ws[i];
        i += 1;
        if ended || w == "-" || !w.starts_with('-') {
            ops.push(w.clone());
            continue;
        }
        if w == "--" {
            ended = true;
            continue;
        }
        if let Some(long) = w.strip_prefix("--") {
            let (name, val) = match long.split_once('=') {
                Some((n, v)) => (n, Some(v.to_string())),
                None => (long, None),
            };
            let hits: Vec<&'static str> = g
                .flags
                .iter()
                .chain(g.valued)
                .chain(g.optional)
                .copied()
                .filter(|c| c.starts_with("--") && c[2..].starts_with(name))
                .collect();
            let opt = match hits.iter().find(|c| &c[2..] == name) {
                Some(e) => *e,
                None if hits.len() == 1 => hits[0],
                None => return None,
            };
            let val = if g.valued.contains(&opt) {
                match val {
                    Some(v) => Some(v),
                    None => {
                        i += 1;
                        Some(ws.get(i - 1)?.clone())
                    }
                }
            } else if g.optional.contains(&opt) || val.is_none() {
                val
            } else {
                return None;
            };
            opts.push((opt, val));
            continue;
        }
        let cluster = &w[1..];
        for (j, ch) in cluster.char_indices() {
            let f = format!("-{}", ch);
            if let Some(opt) = g.flags.iter().find(|x| **x == f) {
                opts.push((*opt, None));
                continue;
            }
            let opt = *g.valued.iter().find(|x| **x == f)?;
            let rest = &cluster[j + ch.len_utf8()..];
            let v = if rest.is_empty() {
                i += 1;
                ws.get(i - 1)?.clone()
            } else {
                rest.to_string()
            };
            opts.push((opt, Some(v)));
            break;
        }
    }
    Some((opts, ops))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the targets one write-utility segment names, each
// with whether it is removed; `None` where the member's grammar does not read the invocation.
fn write_targets(h: &Host, ws: &[String]) -> Option<Vec<(String, bool)>> {
    let (bin, args) = ws.split_first()?;
    if !h.write_bins.iter().any(|b| b == bin) {
        return None;
    }
    if bin == "find" {
        return find_targets(h, args);
    }
    let (opts, mut ops) = parse_opts(&grammar(bin)?, args)?;
    let target_dir = opts.iter().find(|(k, _)| matches!(*k, "-t" | "--target-directory"));
    let out: Vec<(String, bool)> = match bin.as_str() {
        "mkdir" | "touch" | "tee" => ops.into_iter().map(|w| (w, false)).collect(),
        "rm" | "rmdir" => ops.into_iter().map(|w| (w, true)).collect(),
        "mv" | "cp" => {
            let dest = match target_dir {
                Some((_, v)) => v.clone()?,
                None => ops.pop()?,
            };
            if ops.is_empty() {
                return None;
            }
            if bin == "cp" && opts.iter().any(|(k, _)| *k == "--parents") && !ops.iter().all(|w| contained(w)) {
                return None;
            }
            let removed = bin == "mv";
            let mut out: Vec<(String, bool)> =
                if removed { ops.into_iter().map(|w| (w, true)).collect() } else { Vec::new() };
            out.push((dest, false));
            out
        }
        _ => return None,
    };
    (!out.is_empty()).then_some(out)
}

// spec: guard-kit/SPEC.md §The generic ruleset — `find`'s row: a leading `-H` or `-P`, at least one
// start point, and an expression carrying `-delete` and no other declared write or execute form.
fn find_targets(h: &Host, args: &[String]) -> Option<Vec<(String, bool)>> {
    let lead = args.iter().take_while(|w| matches!(w.as_str(), "-H" | "-P")).count();
    let rest = &args[lead..];
    let at = rest
        .iter()
        .position(|w| w.starts_with('-') || matches!(w.as_str(), "(" | "!" | ","))
        .unwrap_or(rest.len());
    let (starts, expr) = rest.split_at(at);
    if starts.is_empty() || !expr.iter().any(|w| w == "-delete") || expr.iter().any(|w| w == "-files0-from") {
        return None;
    }
    let forms = match h.ro_forms.iter().find(|(k, _)| k == "find") {
        Some((_, v)) => v.clone(),
        None => ro_forms_kit("find")?.to_string(),
    };
    for tok in words(&forms) {
        if matches!(tok, "-delete" | "none") {
            continue;
        }
        if !(tok.len() > 2 && tok.starts_with('-') && !tok.starts_with("--")) || expr.iter().any(|w| w == tok) {
            return None;
        }
    }
    Some(starts.iter().map(|w| (w.clone(), true)).collect())
}

pub fn bounded_write(ctx: &Ctx) -> Decided {
    let h = ctx.host();
    if has_substitution(ctx.raw(ctx.cmd())?) || ctx.view(ctx.cmd(), SqHdq)?.contains('$') {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if shell_backgrounds(&s) || s.contains(['\'', '"']) || s.contains("<<") {
        return Ok(None);
    }
    let Some(v) = ctx.dequoted(ctx.cmd())? else { return Ok(None) };
    let (segs, dsegs) = (ctx.segments(&s), ctx.segments(&v));
    if segs.len() != dsegs.len() {
        return Ok(None);
    }
    let inners = h.allow_inners();
    let (mut writes, mut skip) = (0usize, Vec::new());
    for (i, (seg, dseg)) in segs.iter().zip(&dsegs).enumerate() {
        let seg = trim_start(seg);
        if seg.is_empty() {
            continue;
        }
        let Some((ws, outs)) = write_words(seg, dseg) else { return Ok(None) };
        if !outs.iter().all(|t| bounded(h, t, false)) {
            return Ok(None);
        }
        writes += outs.len();
        if ws.first().is_some_and(|w| w == "xargs")
            && (xargs_command_index(&ws.join(" ")).is_err()
                || ws[1..].iter().any(|w| h.write_bins.iter().any(|b| b == w)))
        {
            return Ok(None);
        }
        if let Some(ts) = write_targets(h, &ws) {
            if ts.iter().all(|(t, removed)| bounded(h, t, *removed)) {
                writes += 1;
                skip.push(i);
                continue;
            }
        }
        if is_banner(seg) || is_ro_segment(h, seg) {
            continue;
        }
        if ws.is_empty() || !inners.iter().any(|p| allow_match(&ws.join(" "), p)) {
            return Ok(None);
        }
    }
    if writes == 0 || !ro_forms_clear_except(ctx, ctx.cmd(), &skip)? || refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    if !matches!(ctx.under("grant_path_slot", |g| slot_reach(g, ctx.cmd(), true))?, Reach::Clean)
        || ctx.under("rm_tracked", |r| rm_tracked_reach(r, ctx.cmd()))?.is_some()
        || ctx.under("script_interpreter", |r| interpreter_reach(r, ctx.cmd()))?.is_some()
    {
        return Ok(None);
    }
    allow("bounded write to gitignored scratch (shell-guard auto-allow)")
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

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(ws: &[&str]) -> Vec<String> {
        ws.iter().map(|w| w.to_string()).collect()
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the bounds test's clause (a)
    #[test]
    fn a_contained_word_is_relative_and_never_reaches_up_or_into_git() {
        assert!(contained(".tmp/a.txt"));
        assert!(contained("./.tmp/a"));
        assert!(!contained("/tmp/x"));
        assert!(!contained("~/x"));
        assert!(!contained(".tmp/../x"));
        assert!(!contained(".tmp/.git/x"));
        assert!(!contained(".tmp/.GIT"));
        assert!(!contained(""));
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the bounds test's clause (e), on a temporary
    // directory; skipped where the host cannot create a link
    #[test]
    fn a_link_anywhere_on_the_path_is_seen() {
        let base = std::env::temp_dir().join(format!("checkwright-bounds-link.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("d/real")).unwrap();
        #[cfg(unix)]
        let linked = std::os::unix::fs::symlink(base.join("d/real"), base.join("d/link")).is_ok();
        #[cfg(windows)]
        let linked = std::os::windows::fs::symlink_dir(base.join("d/real"), base.join("d/link")).is_ok();
        assert!(!reaches_link(&base, &["d", "real", "x"]));
        assert!(!reaches_link(&base, &["d", "absent", "x"]));
        if linked {
            assert!(reaches_link(&base, &["d", "link"]));
            assert!(reaches_link(&base, &["d", "link", "x"]));
        }
        let _ = std::fs::remove_dir_all(&base);
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — a segment's words and its output targets
    #[test]
    fn the_redirects_split_from_the_words_and_an_unread_one_withholds() {
        let (w, o) = write_words("git status > .tmp/s 2>&1", "git status > .tmp/s 2>&1").unwrap();
        assert_eq!((w, o), (strings(&["git", "status"]), strings(&[".tmp/s"])));
        let (_, o) = write_words("grep x a >/dev/null <in", "grep x a >/dev/null <in").unwrap();
        assert!(o.is_empty());
        assert_eq!(write_words("echo x>.tmp/f", "echo x>.tmp/f"), None);
        assert_eq!(write_words("ls >&out", "ls >&out"), None);
        assert_eq!(write_words("cat <<EOF", "cat <<EOF"), None);
        let (_, o) = write_words("ls &>>.tmp/a", "ls &>>.tmp/a").unwrap();
        assert_eq!(o, strings(&[".tmp/a"]));
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the operand grammar's option walk
    #[test]
    fn the_option_walk_reads_clusters_prefixes_and_values() {
        let rm = grammar("rm").unwrap();
        let (o, ops) = parse_opts(&rm, &strings(&["-rf", "--verb", "--", "-x"])).unwrap();
        assert_eq!(o.len(), 3);
        assert_eq!(ops, strings(&["-x"]));
        assert!(parse_opts(&rm, &strings(&["--no-preserve-root", "x"])).is_none());
        assert!(parse_opts(&rm, &strings(&["-rI", "x"])).is_none());
        let mv = grammar("mv").unwrap();
        let (o, ops) = parse_opts(&mv, &strings(&["-ft.tmp/d", "a"])).unwrap();
        assert_eq!(o[1], ("-t", Some(".tmp/d".to_string())));
        assert_eq!(ops, strings(&["a"]));
        let cp = grammar("cp").unwrap();
        assert!(parse_opts(&cp, &strings(&["--preserve=mode", "a", "b"])).is_some());
        assert!(parse_opts(&cp, &strings(&["--p", "a", "b"])).is_none());
        assert!(parse_opts(&cp, &strings(&["--force=1", "a", "b"])).is_none());
    }
}
