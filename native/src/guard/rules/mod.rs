// spec: guard-kit/SPEC.md §The generic ruleset — the rule table, in dispatch order, and the tests the
// rules share. A rule reaches a view only through its context.
use super::engine::{Cmd, Ctx, Fault, Rule, Shell};
use super::host::Host;
use super::reader::{View, Word};
use super::text::{self, grep_q, head_word, trim, trim_blank_start, trim_start, words};

mod grants;
mod liveness;
mod reach;
mod spelling;
mod tools;

const BASH: &[Shell] = &[Shell::Bash];
const BOTH: &[Shell] = &[Shell::Bash, Shell::PowerShell];

use View::{Dequoted, Hdq, Raw, Sq, SqDqHd, SqDqHdq, SqHdq};

pub static TABLE: &[Rule] = &[
    Rule { name: "cd_compound", shells: BOTH, views: &[Raw, SqDqHd, Dequoted], test: spelling::cd_compound },
    Rule { name: "git_c_root", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: spelling::git_c_root },
    Rule { name: "scratch_redirect", shells: BOTH, views: &[Raw, SqDqHd, Dequoted], test: spelling::scratch_redirect },
    Rule { name: "abs_script", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: spelling::abs_script },
    Rule { name: "abs_prefix", shells: BASH, views: &[SqDqHd], test: spelling::abs_prefix },
    Rule { name: "expansion", shells: BASH, views: &[SqHdq, SqDqHdq], test: spelling::expansion },
    Rule { name: "brace_glyph", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: spelling::brace_glyph },
    Rule { name: "sed_file", shells: BASH, views: &[Raw, SqDqHd, Dequoted, View::Body], test: tools::sed_file },
    Rule { name: "find_glob", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: tools::find_glob },
    Rule { name: "find_exec", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: tools::find_exec },
    Rule { name: "cat_file", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: tools::cat_file },
    Rule { name: "git_grep", shells: BOTH, views: &[Raw, SqDqHd, Dequoted], test: tools::git_grep },
    Rule { name: "pgrep_self_match", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: tools::pgrep_self_match },
    Rule { name: "bare_sleep", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: tools::bare_sleep },
    Rule {
        name: "git_mutation_under_producer",
        shells: BOTH,
        views: &[Raw, SqDqHd, Dequoted],
        test: liveness::git_mutation_under_producer,
    },
    Rule {
        name: "background_no_record",
        shells: BOTH,
        views: &[Raw, SqDqHd, Dequoted],
        test: liveness::background_no_record,
    },
    Rule { name: "wait_no_producer", shells: BASH, views: &[Raw, SqDqHd], test: liveness::wait_no_producer },
    Rule {
        name: "commit_only_paths",
        shells: BOTH,
        views: &[Raw, SqHdq, SqDqHd, Dequoted],
        test: reach::commit_only_paths,
    },
    Rule { name: "git_stash", shells: BOTH, views: &[Raw, SqDqHd, Dequoted], test: reach::git_stash },
    Rule { name: "truncate_scratch", shells: BASH, views: &[SqDqHd], test: grants::truncate_scratch },
    Rule { name: "append_scratch", shells: BASH, views: &[Raw, Hdq, SqDqHd, Dequoted], test: grants::append_scratch },
    Rule { name: "ro_pipeline", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: grants::ro_pipeline },
    Rule {
        name: "bounded_wait",
        shells: BASH,
        views: &[Raw, Sq, SqDqHd, Dequoted],
        test: liveness::bounded_wait,
    },
    Rule {
        name: "bounded_write",
        shells: BASH,
        views: &[Raw, SqHdq, SqDqHd, Dequoted],
        test: grants::bounded_write,
    },
    Rule { name: "allowlist_chain", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: grants::allowlist_chain },
    Rule { name: "git_rewrite", shells: BOTH, views: &[Raw, SqDqHd, Dequoted], test: reach::git_rewrite },
    Rule { name: "rm_tracked", shells: BOTH, views: &[Raw, SqDqHd, Dequoted], test: reach::rm_tracked },
    Rule { name: "script_interpreter", shells: BOTH, views: &[Raw, SqDqHd, Dequoted], test: reach::script_interpreter },
    Rule {
        name: "grant_path_slot",
        shells: BASH,
        views: &[Raw, SqHdq, SqDqHd, Dequoted],
        test: grants::grant_path_slot,
    },
    Rule { name: "emitter_write", shells: BASH, views: &[Raw, Hdq, SqDqHd, Dequoted], test: grants::emitter_write },
    Rule { name: "shell_wrapper", shells: BASH, views: &[Raw, SqDqHd, Dequoted], test: reach::shell_wrapper },
    Rule {
        name: "worktree_confinement",
        shells: BOTH,
        views: &[Raw, SqHdq, SqDqHd, Dequoted],
        test: reach::worktree_confinement,
    },
];

// spec: guard-kit/SPEC.md §The shell guard — the raw-command carve-out every grant takes: a
// substitution in any spelling.
fn has_substitution(raw: &str) -> bool {
    grep_q(r"\$\(|<\(|>\(", raw) || raw.contains('`')
}

// spec: guard-kit/SPEC.md §The rule roster — a heredoc-bearing call read as its bare one.
struct Unheredoc {
    residue: Vec<bool>,
    bare: String,
}

// spec: guard-kit/SPEC.md §The rule roster — a line with each heredoc opener blanked.
fn unopened(ctx: &Ctx, line: &str) -> String {
    ctx.heredoc_openers(line).iter().fold(line.to_string(), |l, o| l.replacen(o.as_str(), " ", 1))
}

// spec: guard-kit/SPEC.md §The rule roster — each compound-split segment flagged where it is heredoc
// residue, and the skeleton with that residue dropped and each opener blanked; `None` where the call
// carries no heredoc, a terminator line is missing, or the line and segment walks part.
fn unheredoc(ctx: &Ctx, s: &str) -> Option<Unheredoc> {
    let lines: Vec<&str> = s.split('\n').collect();
    let (mut residue, mut kept, mut any) = (Vec::new(), Vec::new(), false);
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        i += 1;
        kept.push(unopened(ctx, line));
        residue.extend(std::iter::repeat(false).take(ctx.segments(line).len()));
        for t in ctx.heredoc_terms(line) {
            any = true;
            if lines.get(i).is_some_and(|l| trim_start(l) == super::reader::HD_MARK) {
                residue.push(true);
                i += 1;
            }
            if !lines.get(i).is_some_and(|l| trim_start(l) == t) {
                return None;
            }
            residue.push(true);
            i += 1;
        }
    }
    (any && residue.len() == ctx.segments(s).len()).then(|| Unheredoc { residue, bare: kept.join("\n") })
}

// spec: guard-kit/SPEC.md §The rule roster — a heredoc read as no composition holds only where no word
// names the stdin it feeds.
fn names_stdin(read: &[Vec<Word>]) -> bool {
    read.iter().flatten().any(|w| w.value == "-" || w.value == "/dev/stdin")
}

// spec: guard-kit/SPEC.md §The generic ruleset — the wider decline: any expansion or substitution.
fn has_expansion(raw: &str) -> bool {
    grep_q(r"\$\(|<\(|>\(|\$\{|\$[A-Za-z_]", raw) || raw.contains('`')
}

// spec: guard-kit/SPEC.md §The generic ruleset — a statement-ending bare `&`, never `&&`, `|&` or a
// redirect's fd-dup.
fn shell_backgrounds(s: &str) -> bool {
    grep_q("(^|[^&>|])&([[:space:]]|;|$)", s)
}

// spec: guard-kit/SPEC.md §The reader and its views — the noclobber operator live in a skeleton,
// which every grant withholds on.
fn noclobber(s: &str) -> bool {
    s.contains(">|")
}

// spec: guard-kit/SPEC.md §The generic ruleset — a literal echo/printf banner segment.
fn is_banner(seg: &str) -> bool {
    matches!(head_word(trim_start(seg)), "echo" | "printf")
}

// spec: guard-kit/SPEC.md §The generic ruleset — a leading shell keyword or negation does not change
// which binary a segment runs.
fn lead_keyword(tok: &str) -> bool {
    matches!(tok, "!" | "until" | "while" | "if" | "then" | "else" | "elif" | "do")
}

fn command_word(seg: &str) -> String {
    let mut s = trim_start(seg);
    while !s.is_empty() {
        let tok = head_word(s);
        if !lead_keyword(tok) {
            break;
        }
        s = trim_start(&s[tok.len()..]);
    }
    text::chomp(s).to_string()
}

#[derive(Clone, Copy)]
enum GitWalk {
    Sub,
    Globals,
    Args,
}

fn plain_word(w: &str) -> Word {
    Word::new(w, w, w)
}

// spec: guard-kit/SPEC.md §The reader and its views — the command read's words: what a segment passes
// its command, a leading keyword and every redirect with its target dropped, a word glued ahead of a
// redirect cut to what it holds ahead of the operator.
fn passed_words(ws: &[Word], escape: char) -> Vec<Word> {
    let mut out: Vec<Word> = Vec::new();
    let mut target = false;
    for w in ws {
        if std::mem::take(&mut target) || (out.is_empty() && lead_keyword(&w.live)) {
            continue;
        }
        let mut held = false;
        let at = w.live.char_indices().find(|&(_, ch)| {
            let bare = !std::mem::take(&mut held);
            held = bare && ch == escape;
            bare && matches!(ch, '>' | '<')
        });
        let Some((at, _)) = at else {
            out.push(w.clone());
            continue;
        };
        let (glued, redirect) = w.live.split_at(at);
        if !(glued.bytes().all(|b| b.is_ascii_digit()) || matches!(glued, "&" | "*")) {
            let value = w.value_before(at).unwrap_or(glued);
            out.push(Word::new(w.spelling_before(at).unwrap_or(value), value, glued));
        }
        target = matches!(redirect.trim_start_matches(['>', '<']), "" | "&");
    }
    out
}

// spec: guard-kit/SPEC.md §The reader and its views — the running reader's escape character.
fn escape_char(ctx: &Ctx) -> char {
    match ctx.shell() {
        Shell::Bash => '\\',
        Shell::PowerShell => '`',
    }
}

// spec: guard-kit/SPEC.md §The reader and its views — the command read's first half: per segment, the
// words it passes, the skeleton's where the word read answers nothing.
fn passed_read(ctx: &Ctx, c: &Cmd) -> Result<Vec<Vec<Word>>, Fault> {
    let escape = escape_char(ctx);
    let s = ctx.view(c, SqDqHd)?;
    let read = ctx.word_read(c)?;
    Ok(ctx
        .segments(&s)
        .iter()
        .enumerate()
        .map(|(i, seg)| match read.as_ref().and_then(|r| r.get(i)) {
            Some(ws) => passed_words(ws, escape),
            None => passed_words(&text::all_words(seg).into_iter().map(plain_word).collect::<Vec<Word>>(), escape),
        })
        .collect())
}

// spec: guard-kit/SPEC.md §The reader and its views — the command read: per segment, the words it
// passes its command, the wrappers the harness view strips dropped by their values.
fn command_read(ctx: &Ctx, c: &Cmd) -> Result<Vec<Vec<Word>>, Fault> {
    Ok(passed_read(ctx, c)?.into_iter().map(|passed| past_wrappers(ctx, passed)).collect())
}

// spec: guard-kit/SPEC.md §The reader and its views — the command read's second half: the wrappers
// the harness view strips dropped ahead of the command word, read off their values.
fn past_wrappers(ctx: &Ctx, passed: Vec<Word>) -> Vec<Word> {
    let values: Vec<String> = passed
        .iter()
        .map(|w| if w.value.is_empty() { "\0".to_string() } else { w.value.replace([' ', '\t', '\n'], "\x01") })
        .collect();
    let kept = text::all_words(ctx.harness_view(&values.join(" "))).len();
    passed.into_iter().skip(values.len() - kept).collect()
}

// spec: guard-kit/SPEC.md §The generic ruleset — each statement's pipe members of a skeleton, each as
// its compound-split segment index, taken in order; `None` where the two splits yield different counts.
fn pipe_members(ctx: &Ctx, s: &str) -> Option<Vec<Vec<(usize, String)>>> {
    let mut at = 0usize;
    let members: Vec<Vec<(usize, String)>> = ctx
        .statements(s)
        .iter()
        .map(|stmt| {
            ctx.pipes(stmt)
                .into_iter()
                .map(|p| {
                    at += 1;
                    (at - 1, p)
                })
                .collect()
        })
        .collect();
    (at == ctx.segments(s).len()).then_some(members)
}

// spec: guard-kit/SPEC.md §The reader and its views — a command word names `name` by its value or by
// the value's last path component.
fn names_command(w: &Word, name: &str) -> bool {
    w.value.rsplit('/').next() == Some(name)
}

// spec: guard-kit/SPEC.md §The rule roster — the git read (rule `git_mutation_under_producer`): for
// each segment of the compound split, the values the command read answers, led by `git`, or none
// where its command word does not name git.
fn git_read(ctx: &Ctx, c: &Cmd) -> Result<Vec<Option<Vec<String>>>, Fault> {
    Ok(command_read(ctx, c)?
        .into_iter()
        .map(|ws| {
            let (head, rest) = ws.split_first()?;
            names_command(head, "git")
                .then(|| std::iter::once("git".to_string()).chain(rest.iter().map(|w| w.value.clone())).collect())
        })
        .collect())
}

// spec: guard-kit/SPEC.md §The rule roster — a global option that moves the repository a git
// command reads, so the tree it names is not the session's.
fn moves_repository(global: &str) -> bool {
    matches!(global, "-C" | "--git-dir" | "--work-tree") || global.starts_with("--git-dir=") || global.starts_with("--work-tree=")
}

// spec: guard-kit/SPEC.md §The generic ruleset — git's global options consumed so the subcommand is
// reached, over the words the git read answers; an option the walk does not recognize declines.
fn git_walk_of(words: &[String], mode: GitWalk) -> Option<Vec<String>> {
    git_walk(words.iter().map(String::as_str), mode)
}

fn git_walk<'a>(words: impl Iterator<Item = &'a str>, mode: GitWalk) -> Option<Vec<String>> {
    let mut globals: Vec<String> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    let (mut first, mut expect_arg, mut found) = (true, false, false);
    for tok in words {
        if found {
            out.push(tok.to_string());
            continue;
        }
        if first {
            if tok != "git" {
                return None;
            }
            first = false;
            continue;
        }
        if expect_arg {
            expect_arg = false;
            continue;
        }
        match tok {
            "-C" | "-c" | "--git-dir" | "--work-tree" | "--namespace" | "--exec-path" | "--config-env" => {
                globals.push(tok.to_string());
                expect_arg = true;
            }
            t if ["--git-dir=", "--work-tree=", "--namespace=", "--exec-path=", "--config-env="]
                .iter()
                .any(|p| t.starts_with(p)) =>
            {
                globals.push(tok.to_string())
            }
            "-p" | "-P" | "--paginate" | "--no-pager" | "--bare" | "--no-replace-objects"
            | "--literal-pathspecs" | "--no-literal-pathspecs" | "--glob-pathspecs"
            | "--noglob-pathspecs" | "--icase-pathspecs" | "--no-optional-locks" => globals.push(tok.to_string()),
            t if t.starts_with('-') => return None,
            _ => match mode {
                GitWalk::Globals => return Some(globals),
                GitWalk::Sub => return Some(vec![tok.to_string()]),
                GitWalk::Args => {
                    out.push(tok.to_string());
                    found = true;
                }
            },
        }
    }
    found.then_some(out)
}

// spec: guard-kit/SPEC.md §The generic ruleset — every redirect target in a skeleton that names a
// file, a `>&` onto one included; an fd-dup drops out.
fn redirect_targets(s: &str) -> Vec<String> {
    text::grep_o("[0-9]*>[>&]?[[:blank:]]*[^[:blank:]|;&]+", s)
        .into_iter()
        .map(|m| strip_redirect_op(&m).to_string())
        .filter(|t| !t.starts_with('&'))
        .collect()
}

// spec: guard-kit/SPEC.md §The generic ruleset — the word a `>&` dups onto: an fd number, with or
// without the closing `-`, or a bare `-`. Any other word is a file.
fn fd_word(w: &str) -> bool {
    let fd = w.strip_suffix('-').unwrap_or(w);
    w == "-" || (!fd.is_empty() && fd.bytes().all(|c| c.is_ascii_digit()))
}

// spec: guard-kit/SPEC.md §The generic ruleset — a redirect pair's target: its fd digits and its
// operator dropped, then the blanks. An fd-dup keeps its `&`; a `>&` onto a file yields the file.
pub(super) fn strip_redirect_op(pair: &str) -> &str {
    let p = pair.trim_start_matches(|c: char| c.is_ascii_digit());
    let p = p.strip_prefix('>').unwrap_or(p);
    if let Some(word) = p.strip_prefix('&').map(trim_blank_start) {
        return if fd_word(word) { p } else { word };
    }
    let p = p.strip_prefix('>').unwrap_or(p);
    trim_blank_start(p)
}

// spec: guard-kit/SPEC.md §The reader and its views — an inert target read by value: `/dev/null`
// alone, since a live fd-dup never reaches a value and `&1` there names a file.
fn inert_value(t: &str) -> bool {
    t == "/dev/null"
}

// spec: guard-kit/SPEC.md §The generic ruleset — an inert target: `/dev/null` or an fd-dup.
fn inert_target(t: &str) -> bool {
    t == "/dev/null" || t.strip_prefix('&').is_some_and(|r| fd_word(trim_blank_start(r)))
}

// spec: guard-kit/SPEC.md §The rule roster — rule `abs_script`'s redirect test by value, taken on
// every call carrying a redirect: every target the skeleton scan found, `n` of them, is `/dev/null`
// as the shell passes it.
pub(super) fn nulled_by_value(ctx: &Ctx, c: &Cmd, n: usize) -> Result<bool, Fault> {
    let Some(read) = ctx.word_read(c)? else { return Ok(false) };
    Ok(redirect_values(&read, '\\').is_some_and(|ts| ts.len() == n && ts.iter().all(|t| t == "/dev/null")))
}

// spec: guard-kit/SPEC.md §The reader and its views — the redirect pairs' targets by value: the
// word read's where it answers and pairs one for one with the skeleton's, else the skeleton's.
pub(super) fn valued_targets(ctx: &Ctx, c: &Cmd) -> Result<Vec<String>, Fault> {
    let skel = redirect_targets(&ctx.view(c, SqDqHd)?);
    let read = ctx.word_read(c)?;
    Ok(paired_targets(skel, read.as_deref(), escape_char(ctx)))
}

fn paired_targets(skel: Vec<String>, read: Option<&[Vec<Word>]>, escape: char) -> Vec<String> {
    match read.and_then(|r| redirect_values(r, escape)) {
        Some(values) if values.len() == skel.len() => values,
        _ => skel,
    }
}

// spec: guard-kit/SPEC.md §The rule roster — each output redirect's target by its value, an fd-dup
// dropped; `None` where a target is cut inside a piece, a word carries a second operator after its
// first, or an operator has no word after it.
pub(super) fn redirect_values(read: &[Vec<Word>], escape: char) -> Option<Vec<String>> {
    let mut out = Vec::new();
    for ws in read {
        let mut k = 0usize;
        while k < ws.len() {
            let w = &ws[k];
            k += 1;
            let Some(at) = bare_operator(&w.live, 0, &['>'], escape) else { continue };
            let more = w.live[at + 1..].chars().next().filter(|c| matches!(c, '>' | '&'));
            let end = at + 1 + more.map_or(0, char::len_utf8);
            if bare_operator(&w.live, end, &['>', '<'], escape).is_some() {
                return None;
            }
            let value = if end == w.live.len() {
                k += 1;
                ws.get(k - 1)?.value.clone()
            } else {
                w.value_after(end)?.to_string()
            };
            if !(more == Some('&') && fd_word(&value)) {
                out.push(value);
            }
        }
    }
    Some(out)
}

// spec: guard-kit/SPEC.md §The reader and its views — the first character of `set` at or after `from`
// standing bare in a word's live text, not behind an escaping character.
fn bare_operator(live: &str, from: usize, set: &[char], escape: char) -> Option<usize> {
    let mut held = false;
    live.char_indices().find(|&(i, ch)| {
        let bare = !std::mem::take(&mut held);
        held = bare && ch == escape;
        bare && i >= from && set.contains(&ch)
    }).map(|(i, _)| i)
}

// spec: guard-kit/SPEC.md §The generic ruleset — a redirect pair spelled `>&` onto a named file.
fn dups_onto_file(pair: &str) -> bool {
    let p = pair.trim_start_matches(|c: char| c.is_ascii_digit());
    p.strip_prefix(">&").is_some_and(|r| !fd_word(trim_blank_start(r)))
}

// spec: guard-kit/SPEC.md §The generic ruleset — a segment with its redirects removed and trimmed.
fn segment_core(seg: &str) -> String {
    let e = text::re(r"[[:blank:]]*[0-9]*(>>?|>&|<)[[:blank:]]*(&?[0-9-]+|[^[:blank:]]+)?");
    let mut lines: Vec<String> = Vec::new();
    for line in seg.split('\n') {
        let mut kept = String::new();
        let mut pos = 0usize;
        while pos <= line.len() {
            match e.find_from(line, pos) {
                Some((s, t)) if t > s => {
                    kept.push_str(&line[pos..s]);
                    pos = t;
                }
                _ => break,
            }
        }
        kept.push_str(line.get(pos..).unwrap_or(""));
        lines.push(kept);
    }
    trim(text::chomp(&lines.join("\n"))).to_string()
}

fn on_ro_roster(h: &Host, w: &str) -> bool {
    h.ro_bins.iter().any(|b| b == w)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the xargs option walk: the index of the word naming
// the command xargs runs, none for a bare xargs, `Err` on an option the walk does not recognize.
fn xargs_command_index(seg: &str) -> Result<Option<usize>, ()> {
    let toks = words(seg);
    if toks.first() != Some(&"xargs") {
        return Err(());
    }
    let mut want_arg = false;
    for (i, tok) in toks.iter().enumerate().skip(1) {
        if want_arg {
            want_arg = false;
            continue;
        }
        match *tok {
            "-0" | "-t" | "-r" | "-x" | "-p" | "--null" | "--no-run-if-empty" | "--verbose" | "--exit"
            | "--interactive" | "--open-tty" => {}
            "-I" | "-L" | "-n" | "-P" | "-s" | "-E" | "-d" | "-a" => want_arg = true,
            t if t.len() >= 2 && t.starts_with('-') && b"0ILnPsEdae".contains(&t.as_bytes()[1]) => {}
            t if t.starts_with("--") && t[2..].contains('=') => {}
            t if t.starts_with('-') => return Err(()),
            _ => return Ok(Some(i)),
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — xargs runs a command, so the segment is read-only
// only when the command it runs is itself on the roster.
fn is_ro_xargs(h: &Host, seg: &str) -> bool {
    let seg = trim_start(seg);
    match xargs_command_index(seg) {
        Err(()) => false,
        Ok(None) => true,
        Ok(Some(i)) => {
            let t = words(seg)[i];
            match t {
                "xargs" => false,
                "echo" | "printf" => true,
                _ => on_ro_roster(h, t),
            }
        }
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — the read-only-segment test, roster membership only.
fn is_ro_segment(h: &Host, seg: &str) -> bool {
    let seg = trim_start(seg);
    let first = head_word(seg);
    if first.is_empty() {
        return false;
    }
    if first == "xargs" && !is_ro_xargs(h, seg) {
        return false;
    }
    on_ro_roster(h, first)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the kit's declaration table for the default roster.
fn ro_forms_kit(bin: &str) -> Option<&'static str> {
    Some(match bin {
        "sort" => "-o --output --compress-program",
        "uniq" => "pos:2",
        "find" => "-delete -exec -execdir -ok -okdir -fprint -fprint0 -fprintf -fls",
        "sed" => "-i --in-place",
        "rg" => "--pre",
        "grep" | "egrep" | "fgrep" | "head" | "tail" | "cat" | "wc" | "cut" | "tr" | "nl" | "rev" | "tac"
        | "paste" | "comm" | "column" | "diff" | "jq" | "ls" | "xargs" | "awk" => "none",
        _ => return None,
    })
}

// spec: guard-kit/SPEC.md §The generic ruleset — the declared-forms test on one invocation: the
// consumer's entry else the kit table; false on a matched form, an undeclared member, or `pos:N`
// under xargs.
fn ro_invocation_clear(h: &Host, bin: &str, dwords: &str, skel_core: &str, xargs: bool) -> bool {
    let forms = match h.ro_forms.iter().find(|(k, _)| k == bin) {
        Some((_, v)) => v.clone(),
        None => match ro_forms_kit(bin) {
            Some(f) => f.to_string(),
            None => return false,
        },
    };
    let decl = words(&forms);
    if decl.is_empty() {
        return false;
    }
    let unslashed = dwords.replace('\\', "");
    let ws = words(&unslashed);
    let skel = words(skel_core);
    for tok in decl {
        if tok == "none" {
            continue;
        }
        if let Some(need) = tok.strip_prefix("pos:") {
            let valid = !need.is_empty() && !need.starts_with('0') && need.bytes().all(|b| b.is_ascii_digit());
            if !valid || xargs {
                return false;
            }
            let need: usize = need.parse().unwrap_or(usize::MAX);
            let pos = skel.iter().filter(|w| !(w.len() >= 2 && w.starts_with('-'))).count();
            if pos >= need {
                return false;
            }
        } else if tok.len() >= 3 && tok.starts_with("--") {
            for w in &ws {
                let w = w.split('=').next().unwrap_or("");
                if w.len() >= 3 && w.starts_with("--") && tok[2..].starts_with(&w[2..]) {
                    return false;
                }
            }
        } else if tok.len() == 2 && tok.starts_with('-') && tok.as_bytes()[1].is_ascii_alphanumeric() {
            for w in &ws {
                if w.len() >= 2 && w.starts_with('-') && !w[1..].starts_with('-') && w.contains(&tok[1..]) {
                    return false;
                }
            }
        } else if tok.len() >= 2 && tok.starts_with('-') {
            if ws.contains(&tok) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

// spec: guard-kit/SPEC.md §The generic ruleset — the one declared-forms reader: each segment's member
// and words by their values off the command read, its program off the dequoted view aligned segment
// for segment with the skeleton; withheld where the word read answers nothing or the two do not align.
fn ro_forms_clear(ctx: &Ctx, c: &Cmd) -> Result<bool, Fault> {
    ro_forms_clear_except(ctx, c, &[])
}

// spec: guard-kit/SPEC.md §The generic ruleset — the same reader with the segments a caller has
// already read as writes, by their compound-split index, left out.
fn ro_forms_clear_except(ctx: &Ctx, c: &Cmd, skip: &[usize]) -> Result<bool, Fault> {
    ro_forms_read(ctx, c, skip, false)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `background_no_record`'s reading of the same
// reader: a segment the compound split leaves whole across a bare `&` is read statement by statement.
fn ro_forms_clear_statements(ctx: &Ctx, c: &Cmd) -> Result<bool, Fault> {
    ro_forms_read(ctx, c, &[], true)
}

// spec: guard-kit/SPEC.md §The generic ruleset — a segment cut at each statement-ending bare `&`.
fn background_statements(seg: &str) -> Vec<&str> {
    let b = seg.as_bytes();
    let mut out = Vec::new();
    let mut from = 0usize;
    for i in 0..b.len() {
        let led = i == 0 || !matches!(b[i - 1], b'&' | b'>' | b'|');
        if b[i] == b'&' && led && b.get(i + 1).map_or(true, |c| text::is_space(*c)) {
            out.push(&seg[from..i]);
            from = i + 1;
        }
    }
    out.push(&seg[from..]);
    out
}

// spec: guard-kit/SPEC.md §The rule roster — rule `ro_pipeline`'s program discriminator, its option
// test: a word the shell passes dash-led whose dequoted token is not, or a segment whose word read
// does not align with its dequoted words.
fn masked_option(read: Option<&Vec<Word>>, dseg: &str) -> bool {
    let dw = words(dseg);
    match read {
        Some(ws) if ws.len() == dw.len() => {
            ws.iter().zip(dw).any(|(w, d)| w.value.starts_with('-') && !d.starts_with('-'))
        }
        _ => true,
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — one segment's words cut where `background_statements`
// cuts its skeleton: after a word whose live text ends in a statement-ending bare `&`, the `&` dropped;
// `None` where that cut falls inside a piece.
fn background_words(ws: &[Word]) -> Option<Vec<Vec<Word>>> {
    let mut out: Vec<Vec<Word>> = vec![Vec::new()];
    for w in ws {
        let b = w.live.as_bytes();
        let n = b.len();
        let ends = n > 0 && b[n - 1] == b'&' && (n == 1 || !matches!(b[n - 2], b'&' | b'>' | b'|'));
        if !ends {
            out.last_mut()?.push(w.clone());
            continue;
        }
        if n > 1 {
            let value = w.value_before(n - 1)?;
            out.last_mut()?.push(Word::new(w.spelling_before(n - 1)?, value, &w.live[..n - 1]));
        }
        out.push(Vec::new());
    }
    Some(out)
}

fn ro_forms_read(ctx: &Ctx, c: &Cmd, skip: &[usize], statements: bool) -> Result<bool, Fault> {
    let h = ctx.host();
    let escape = escape_char(ctx);
    let Some(v) = ctx.dequoted(c)? else { return Ok(false) };
    let s = ctx.view(c, SqDqHd)?;
    let segs = ctx.segments(&s);
    let dsegs = ctx.segments(&v);
    let Some(read) = ctx.word_read(c)? else { return Ok(false) };
    if segs.len() != dsegs.len() || read.len() != segs.len() {
        return Ok(false);
    }
    let mut pairs: Vec<(&str, &str, Vec<Word>, bool)> = Vec::new();
    for (i, ((seg, dseg), ws)) in segs.iter().zip(&dsegs).zip(&read).enumerate() {
        if skip.contains(&i) {
            continue;
        }
        let masked = masked_option(Some(ws), dseg);
        if !statements {
            pairs.push((seg, dseg, passed_words(ws, escape), masked));
            continue;
        }
        let (cut, dcut) = (background_statements(seg), background_statements(dseg));
        let Some(wcut) = background_words(ws).filter(|w| w.len() == cut.len()) else { return Ok(false) };
        if cut.len() != dcut.len() {
            return Ok(false);
        }
        pairs.extend(cut.into_iter().zip(dcut).zip(wcut).map(|((s, d), w)| (s, d, passed_words(&w, escape), masked)));
    }
    for (seg, dseg, passed, masked) in pairs {
        let Some((head, args)) = passed.split_first() else { continue };
        let bin = head.value.as_str();
        if !on_ro_roster(h, bin) {
            continue;
        }
        let cw = command_word(seg);
        let dcw = command_word(dseg);
        let dhead = head_word(&dcw);
        let core = segment_core(&cw);
        let rest = |s: &str, head: &str| s.strip_prefix(head).unwrap_or(s).to_string();
        let values: Vec<&str> = args.iter().map(|w| w.value.as_str()).collect();
        if !ro_invocation_clear(h, bin, &values.join(" "), &rest(&core, head_word(&core)), false) {
            return Ok(false);
        }
        let program = format!("{}{}", bin, rest(&dcw, dhead));
        if matches!(bin, "sed" | "awk") && (masked || !tools::program_clear(bin, &program)) {
            return Ok(false);
        }
        if bin != "xargs" {
            continue;
        }
        let idx = match xargs_command_index(&dcw) {
            Err(()) => return Ok(false),
            Ok(None) => continue,
            Ok(Some(i)) => i,
        };
        let dtoks = words(&dcw);
        let xcmd = dtoks[idx].replace('\\', "");
        if xcmd == "echo" || xcmd == "printf" || !on_ro_roster(h, &xcmd) {
            continue;
        }
        if !ro_invocation_clear(h, &xcmd, &dtoks[idx + 1..].join(" "), "", true) {
            return Ok(false);
        }
        if matches!(xcmd.as_str(), "sed" | "awk") && (masked || !tools::program_clear(&xcmd, &dtoks[idx..].join(" "))) {
            return Ok(false);
        }
    }
    Ok(true)
}

// spec: guard-kit/SPEC.md §The generic ruleset — true when a segment's core is the bare form of a
// committed allow entry; the second value says a closing ` *` granted it.
fn is_bare_allow(h: &Host, seg: &str) -> (bool, bool) {
    let core = segment_core(seg);
    if core.is_empty() {
        return (false, false);
    }
    let mut star = false;
    for bl in h.allow_inners() {
        if let Some(b) = bl.strip_suffix(" *") {
            if !b.contains('*') && core == b {
                star = true;
            }
        } else if bl.contains('*') {
        } else if core == bl {
            return (true, false);
        }
    }
    (star, star)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the ruleset's one shell-keyword walk: each token
// with its loop depth and whether it stands in command position, where alone a keyword is one;
// `None` on an unbalanced do/done.
fn loop_span(s: &str) -> Option<Vec<(i32, bool, String)>> {
    Some(loop_tokens(s)?.into_iter().map(|(depth, cmdpos, tok, _)| (depth, cmdpos, tok)).collect())
}

// spec: guard-kit/SPEC.md §The generic ruleset — each compound segment's loop depth at its first word in
// command position that is no keyword; `None` where the walk and the compound split part.
fn segment_depths(s: &str, n: usize) -> Option<Vec<Option<i32>>> {
    let mut out = vec![None; n];
    let mut k = 0usize;
    for (depth, cmdpos, tok, _) in loop_tokens(s)? {
        match tok.as_str() {
            ";" | "||" | "&&" | "|" => k += 1,
            "&" | "(" | ")" | "{" | "}" => {}
            "!" | "until" | "while" | "if" | "then" | "else" | "elif" | "for" | "do" | "done" if cmdpos => {}
            _ if cmdpos => {
                if let Some(slot) = out.get_mut(k).filter(|d| d.is_none()) {
                    *slot = Some(depth);
                }
            }
            _ => {}
        }
    }
    (k == n).then_some(out)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the same walk with each token's byte offset in the
// skeleton, for a reader that needs a span's text. A redirect operator, its `&` included, ends the
// word it is glued to, and neither its target nor a `for` loop's variable stands in command position.
fn loop_tokens(s: &str) -> Option<Vec<(i32, bool, String, usize)>> {
    let joined = format!("{};", s.replace('\n', ";"));
    let b = joined.as_bytes();
    let mut cuts: Vec<(usize, usize)> = Vec::new();
    let (mut i, mut open) = (0usize, None);
    while i < b.len() {
        let glyph = match b[i] {
            b'|' | b'&' if b.get(i + 1) == Some(&b[i]) => 2,
            b';' | b'|' | b'&' | b'(' | b')' | b'{' | b'}' => 1,
            b'>' | b'<' => {
                let run = b[i..].iter().take_while(|c| matches!(c, b'>' | b'<')).count();
                run + usize::from(b.get(i + run) == Some(&b'&'))
            }
            _ => 0,
        };
        if glyph == 0 && !matches!(b[i], b' ' | b'\t') {
            open.get_or_insert(i);
            i += 1;
            continue;
        }
        if let Some(st) = open.take() {
            cuts.push((st, i));
        }
        if glyph > 0 {
            cuts.push((i, i + glyph));
        }
        i += glyph.max(1);
    }
    let mut out = Vec::new();
    let (mut depth, mut cmdpos) = (0i32, true);
    for (at, end) in cuts {
        let tok = &joined[at..end];
        match tok {
            ";" | "|" | "||" | "&&" | "&" | "(" | ")" | "{" | "}" => {
                out.push((depth, cmdpos, tok.to_string(), at));
                cmdpos = true;
            }
            "!" | "until" | "while" | "if" | "then" | "else" | "elif" | "for" if cmdpos => {
                out.push((depth, cmdpos, tok.to_string(), at));
                cmdpos = tok != "for";
            }
            "do" if cmdpos => {
                depth += 1;
                out.push((depth, cmdpos, tok.to_string(), at));
            }
            "done" if cmdpos => {
                depth -= 1;
                if depth < 0 {
                    return None;
                }
                out.push((depth, cmdpos, tok.to_string(), at));
            }
            _ => {
                out.push((depth, cmdpos && !tok.starts_with(['>', '<']), tok.to_string(), at));
                cmdpos = false;
            }
        }
    }
    (depth == 0).then_some(out)
}

// spec: guard-kit/SPEC.md §The generic ruleset — the words one segment keeps once its redirects are
// dropped: `Err(1)` when none remain, `Err(2)` when the views cannot be aligned or a heredoc opens.
fn unredirected_words(skel: &str, deq: &str) -> Result<String, u8> {
    let sw = text::held_words(skel);
    let dw = words(deq);
    if sw.len() != dw.len() {
        return Err(2);
    }
    let digit_op = |w: &str, op: &str| {
        w.len() == 1 + op.len() && w.as_bytes()[0].is_ascii_digit() && &w[1..] == op
    };
    let digit_glued = |w: &str, op: char| {
        w.len() >= 3 && w.as_bytes()[0].is_ascii_digit() && w[1..].starts_with(op)
    };
    let mut kept: Vec<&str> = Vec::new();
    let mut k = 0usize;
    while k < sw.len() {
        let w = sw[k];
        if matches!(w, "<<<" | ">" | ">>" | "<" | "&>" | "&>>" | ">&")
            || digit_op(w, ">")
            || digit_op(w, ">>")
            || digit_op(w, "<")
            || digit_op(w, ">&")
        {
            k += 2;
            continue;
        }
        if let Some(at) = w.find("<<") {
            if w.len() > at + 2 {
                if w.starts_with("<<<") && w.len() > 3 {
                    k += 1;
                    continue;
                }
                return Err(2);
            }
        }
        if (w.len() >= 2 && (w.starts_with('>') || w.starts_with('<')))
            || (w.len() >= 3 && w.starts_with("&>"))
            || digit_glued(w, '>')
            || digit_glued(w, '<')
        {
            k += 1;
            continue;
        }
        kept.push(dw[k]);
        k += 1;
    }
    if kept.is_empty() {
        return Err(1);
    }
    Ok(kept.join(" "))
}

// spec: guard-kit/SPEC.md §Layout and configuration — a program-bearing tool: the sed/awk/perl/python
// walker rows and every script interpreter.
fn program_bearing(h: &Host, b: &str) -> bool {
    let b = b.rsplit('/').next().unwrap_or(b);
    matches!(b, "sed" | "awk" | "perl" | "python" | "python3") || h.interpreters.iter().any(|i| i == b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §The generic ruleset — the table's names are distinct identifiers and
    // every declared view spells back to itself.
    #[test]
    fn the_table_rows_are_distinct_and_their_views_spell() {
        let mut names: Vec<&str> = TABLE.iter().map(|r| r.name).collect();
        assert_eq!(names.len(), 32);
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 32);
        for r in TABLE {
            let mut spelt: Vec<&str> = r.views.iter().map(|v| v.spelling()).collect();
            let n = spelt.len();
            spelt.sort();
            spelt.dedup();
            assert_eq!(spelt.len(), n, "{} declares a view twice", r.name);
        }
    }

    // spec: guard-kit/SPEC.md §The generic ruleset — the normalizer is not visible to the rules module,
    // so a rule has no path to a view except its context.
    #[test]
    fn no_rule_source_names_the_reader() {
        for src in [
            include_str!("mod.rs"),
            include_str!("spelling.rs"),
            include_str!("tools.rs"),
            include_str!("liveness.rs"),
            include_str!("grants.rs"),
            include_str!("reach.rs"),
        ] {
            let needle = ["bash", "::"].concat();
            assert!(!src.contains(&needle), "a rule source names the bash reader");
            let needle = ["skeleton", "("].concat();
            assert!(!src.contains(&needle), "a rule source calls the normalizer");
        }
    }

    // spec: guard-kit/SPEC.md §The rule roster — the git read takes each word by its value, a keyword
    // and a redirect by the live text, and a word glued ahead of a redirect by the value it holds
    #[test]
    fn the_passed_words_drop_keywords_and_redirects_by_live_text() {
        use crate::guard::reader::WordAcc;
        let word = |s: &str, v: &str, l: &str| Word::new(s, v, l);
        let got = |ws: &[Word]| passed_words(ws, '\\').into_iter().map(|w| w.value).collect::<Vec<String>>();
        assert_eq!(got(&[plain_word("if"), plain_word("git"), word("'stash'", "stash", "\0SQ")]), vec!["git", "stash"]);
        assert_eq!(got(&[plain_word("git"), plain_word("stash"), plain_word("2>&1"), plain_word(">"), plain_word("o"), plain_word("pop")]), vec!["git", "stash", "pop"]);
        assert_eq!(got(&[plain_word("git"), plain_word("stash>o"), plain_word("&>/dev/null")]), vec!["git", "stash"]);
        assert_eq!(got(&[plain_word("git"), word("'if'", "if", "\0SQ"), word("'>'", ">", "\0SQ"), word("a\\>b", "a>b", "a\\>b")]), vec!["git", "if", ">", "a>b"]);
        let pieced = |pieces: &[(&str, &str, &str)]| {
            let mut acc = WordAcc::new();
            for (s, v, l) in pieces {
                acc.piece(s.as_bytes(), v.as_bytes(), l.as_bytes());
            }
            acc.finish().remove(0).remove(0)
        };
        let quoted = pieced(&[("'stash'", "stash", "\0SQ"), (">", ">", ">"), ("/dev/null", "/dev/null", "/dev/null")]);
        let escaped = pieced(&[("st", "st", "st"), ("\\a", "a", "\\a"), ("sh", "sh", "sh"), (">", ">", ">"), ("o", "o", "o")]);
        assert_eq!(got(&[plain_word("git"), quoted.clone(), plain_word("push")]), vec!["git", "stash", "push"]);
        assert_eq!(passed_words(&[quoted], '\\')[0].spelling, "'stash'", "a glued word keeps its own spelling");
        assert_eq!(got(&[plain_word("git"), escaped]), vec!["git", "stash"]);
        assert_eq!(got(&[plain_word("git"), plain_word("stash"), plain_word("2>&"), plain_word("1"), plain_word("pop")]), vec!["git", "stash", "pop"]);
        assert_eq!(got(&[plain_word("git"), plain_word("stash"), plain_word(">&"), plain_word("out"), plain_word("pop")]), vec!["git", "stash", "pop"]);
    }

    // spec: guard-kit/SPEC.md §The reader and its views — the redirect pairs' targets by value, the
    // skeleton's standing where the word read answers nothing or the two counts part
    #[test]
    fn a_target_is_read_by_its_value_where_the_two_reads_pair() {
        use crate::guard::reader::WordAcc;
        let pieced = |pieces: &[(&str, &str, &str)]| {
            let mut acc = WordAcc::new();
            for (s, v, l) in pieces {
                acc.piece(s.as_bytes(), v.as_bytes(), l.as_bytes());
            }
            acc.finish().remove(0).remove(0)
        };
        let s = |ts: &[&str]| ts.iter().map(|t| t.to_string()).collect::<Vec<String>>();
        let paired = |skel: &[&str], ws: Vec<Word>| paired_targets(s(skel), Some(&[ws]), '\\');
        let null = s(&["/dev/null"]);
        assert_eq!(paired(&["/dev/null"], vec![plain_word("x"), plain_word("2>/dev/null")]), null, "bare");
        let dq = pieced(&[("2", "2", "2"), (">", ">", ">"), ("\"/dev/null\"", "/dev/null", "\0DQ")]);
        assert_eq!(paired(&["\0DQ"], vec![plain_word("x"), dq]), null, "quoted");
        let esc = pieced(&[(">", ">", ">"), ("\\/", "/", "\\/"), ("dev/null", "dev/null", "dev/null")]);
        assert_eq!(paired(&["\\/dev/null"], vec![plain_word("x"), esc]), null, "escaped");
        let apart = Word::new("'/dev/null'", "/dev/null", "\0SQ");
        assert_eq!(paired(&["\0SQ"], vec![plain_word("x"), plain_word(">"), apart]), null, "split apart");
        assert_eq!(paired(&[], vec![plain_word("x"), plain_word("2>&1")]), Vec::<String>::new(), "an fd-dup");
        let quoted = Word::new("\"out\"", "out", "\0DQ");
        assert_eq!(paired(&["\0DQ", "x"], vec![plain_word("y"), plain_word(">"), quoted]), s(&["\0DQ", "x"]), "a count mismatch falls back");
        assert_eq!(paired_targets(s(&["\0DQ"]), None, '\\'), s(&["\0DQ"]), "no word read falls back");
    }

    // spec: guard-kit/SPEC.md §The rule roster — rule `ro_pipeline`'s program discriminator, its
    // option test: an escaped option is masked, a quoted backslash ahead of a dash is program text
    #[test]
    fn a_word_passed_dash_led_from_behind_an_escape_is_masked() {
        let word = |s: &str, v: &str, l: &str| Word::new(s, v, l);
        let sed = |w: Word| vec![plain_word("sed"), plain_word("-n"), w];
        assert!(masked_option(Some(&sed(word("\\-x-p", "-x-p", "\\-x-p"))), "sed -n \\-x-p"));
        assert!(!masked_option(Some(&sed(word("'\\-x-p'", "\\-x-p", "\0SQ"))), "sed -n \\-x-p"));
        assert!(!masked_option(Some(&sed(word("'-E'", "-E", "\0SQ"))), "sed -n -E"));
        assert!(masked_option(Some(&sed(plain_word("p"))), "sed -n p x"));
        assert!(masked_option(None, "sed -n p"));
    }

    // spec: guard-kit/SPEC.md §The rule roster — rule `background_no_record`'s statement cut on the
    // word read: a word ending in a statement-ending bare `&` closes its statement, the `&` dropped
    #[test]
    fn the_word_read_cuts_where_the_skeleton_cuts_at_a_bare_ampersand() {
        let values = |ws: &[Word]| {
            background_words(ws).map(|st| st.into_iter().map(|w| w.into_iter().map(|w| w.value).collect::<Vec<String>>()).collect::<Vec<_>>())
        };
        let ws = [plain_word("sleep"), plain_word("1"), plain_word("&"), plain_word("sort>/dev/null"), plain_word("-o"), plain_word("x")];
        assert_eq!(values(&ws), Some(vec![vec!["sleep".to_string(), "1".to_string()], vec!["sort>/dev/null".to_string(), "-o".to_string(), "x".to_string()]]));
        let glued = [plain_word("sleep"), plain_word("1&"), plain_word("ls")];
        assert_eq!(values(&glued), Some(vec![vec!["sleep".to_string(), "1".to_string()], vec!["ls".to_string()]]));
        let kept = [plain_word("ls"), plain_word("2>&"), plain_word("1"), Word::new("'&'", "&", "\0SQ")];
        assert_eq!(values(&kept).map(|st| st.len()), Some(1), "a dup operator and a quoted '&' cut nothing");
        let held = Word::new("'a'&", "a&", "\0SQ&");
        assert_eq!(values(&[plain_word("echo"), held]), None, "a cut inside a piece");
    }

    #[test]
    fn the_shared_walks_read_as_the_library_did() {
        assert_eq!(command_word("  while ! kill -0 1"), "kill -0 1");
        assert_eq!(segment_core(" grep x f > out 2>&1 < in "), "grep x f");
        let sub = |t: &str, mode| git_walk(text::all_words(t).into_iter(), mode);
        assert_eq!(
            sub("git -C . --no-pager commit -m x", GitWalk::Sub),
            Some(vec!["commit".to_string()])
        );
        assert_eq!(sub("git --frob commit", GitWalk::Sub), None);
        assert_eq!(sub("git -c a=b log", GitWalk::Globals), Some(vec!["-c".to_string()]));
        assert_eq!(xargs_command_index("xargs -0 -n 1 grep x"), Ok(Some(4)));
        assert_eq!(xargs_command_index("xargs --bogus grep"), Err(()));
        assert!(loop_span("until x; do sleep 1; done").is_some());
        assert!(loop_span("done").is_none());
        let argued = loop_span("echo done; until grep -q do f; do sleep 1; done; echo for").unwrap();
        let keywords: Vec<&str> =
            argued.iter().filter(|(_, cmdpos, _)| *cmdpos).map(|(_, _, t)| t.as_str()).collect();
        assert_eq!(keywords, vec!["echo", "until", "grep", "do", "sleep", "done", ";", "echo"]);
        let toks = loop_tokens("until a&&b; do sleep 1|&x; done").unwrap();
        let read: Vec<(i32, &str, usize)> = toks.iter().map(|(d, _, t, at)| (*d, t.as_str(), *at)).collect();
        assert_eq!(
            read,
            vec![
                (0, "until", 0),
                (0, "a", 6),
                (0, "&&", 7),
                (0, "b", 9),
                (0, ";", 10),
                (1, "do", 12),
                (1, "sleep", 15),
                (1, "1", 21),
                (1, "|", 22),
                (1, "&", 23),
                (1, "x", 24),
                (1, ";", 25),
                (0, "done", 27),
                (0, ";", 31),
            ]
        );
        assert_eq!(
            segment_depths("echo a; until x; do sleep 1 |& cat; done", 5),
            Some(vec![Some(0), Some(0), Some(1), Some(1), None])
        );
        assert_eq!(segment_depths("a \\; b", 1), None, "an escaped separator parts the two splits");
        let glued = loop_span("until a; do sleep>o 1; done>o").unwrap();
        let keywords: Vec<&str> = glued.iter().filter(|(_, cmdpos, _)| *cmdpos).map(|(_, _, t)| t.as_str()).collect();
        assert_eq!(keywords, vec!["until", "a", "do", "sleep", "done"]);
        let argued = loop_span("for done in x; do sleep 1 >& done; done").unwrap();
        let keywords: Vec<&str> = argued.iter().filter(|(_, cmdpos, _)| *cmdpos).map(|(_, _, t)| t.as_str()).collect();
        assert_eq!(keywords, vec!["for", "do", "sleep", "done", ";"]);
        for inert in ["2>&1", ">&2", ">& 2", ">&-", "2>&3-", "> o", ">> o"] {
            assert!(!dups_onto_file(inert), "{} dups onto no file", inert);
        }
        for inert in ["2>&1", ">&2", ">& 2", ">&-", "2>&3-", "2>/dev/null", ">&/dev/null", ">& /dev/null"] {
            assert!(inert_target(strip_redirect_op(inert)), "{} is inert", inert);
        }
        for (write, file) in [(">& out", "out"), (">&out", "out"), ("2>&out", "out"), (">&2x", "2x"), (">&-x", "-x"), ("1>> o", "o")] {
            assert_eq!(strip_redirect_op(write), file);
            assert!(!inert_target(strip_redirect_op(write)), "{} writes a file", write);
            assert_eq!(dups_onto_file(write), write.contains('&'), "{}", write);
        }
        assert_eq!(redirect_targets("grep x f >& out 2>&1 >&2x 1>& 2 > o >&/dev/null"), vec!["out", "2x", "o", "/dev/null"]);
        assert_eq!(segment_core("grep x f >& out 2>& 1 >&o2"), "grep x f");
        assert_eq!(redirect_targets("grep x f >&2\rx >&\r2 >\r| head"), vec!["2\rx", "\r2", "\r"]);
        for write in [">&2\rx", ">&\r2", ">& \r2", "2>\r"] {
            assert!(!inert_target(strip_redirect_op(write)), "{:?} writes a file", write);
        }
        assert!(dups_onto_file(">&\r2"));
        assert_eq!(unredirected_words("grep x f > out", "grep x f > out"), Ok("grep x f".to_string()));
        assert_eq!(unredirected_words("> out", "> out"), Err(1));
        assert_eq!(unredirected_words("cat <<EOF", "cat <<EOF"), Err(2));
        assert_eq!(unredirected_words("rm .tmp/a\\ b > o", "rm .tmp/a\\\x01b > o"), Ok("rm .tmp/a\\\x01b".to_string()));
        assert!(shell_backgrounds("make &"));
        assert_eq!(background_statements(" done & sort -o a b 2>&1"), vec![" done ", " sort -o a b 2>&1"]);
        assert_eq!(background_statements("a && b |& c &> d"), vec!["a && b |& c &> d"]);
        assert!(!shell_backgrounds("a && b 2>&1"));
    }
}
