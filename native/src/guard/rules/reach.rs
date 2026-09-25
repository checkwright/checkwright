// spec: guard-kit/SPEC.md §The generic ruleset — the rules that judge what a command reaches: a
// history rewrite, a tracked deletion, a scratch body, a wrapped payload, the main checkout.
use super::{
    command_word, git_subcommand, has_expansion, inert_target, is_banner, is_ro_segment,
    program_bearing, ro_forms_clear, segment_core, shell_backgrounds, GitWalk,
};
use crate::guard::engine::{Cmd, Ctx, Decided, Fault, Shell, Verdict};
use crate::guard::host::under;
use crate::guard::reader::View::{SqDqHd, SqHdq};
use crate::guard::text::{self, grep_q, head_word, trim_start, unsentinel, words};
use crate::walk;

fn block(m: impl Into<String>) -> Decided {
    Ok(Some(Verdict::Block(m.into())))
}

pub fn git_rewrite(ctx: &Ctx) -> Decided {
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    let commit = grep_q("(^|[[:space:]])git[[:space:]]+commit([[:space:]]|$)", &s)
        && grep_q("(^|[[:space:]])(-F|--file|--amend)([^A-Za-z0-9_]|$)", &s);
    let reset = grep_q("(^|[[:space:]])git[[:space:]]+reset([[:space:]]|$)", &s)
        && grep_q("(^|[[:space:]])--soft([^A-Za-z0-9_]|$)", &s);
    if !(commit || reset) || refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    Ok(Some(Verdict::Advise("re-verify volatile git state before this history rewrite (DOCTRINE.md: Re-verify volatile state before a git history rewrite): confirm HEAD with 'git log --oneline -3' before an amend or squash; after a 'git reset --soft' re-stage and verify staged content with 'git show :<path>' before committing (the soft reset keeps the old index snapshot); carry the message in the command ('-m', or '-F -' from a heredoc) — a scratch message file may be another session's, and a leftover lands the wrong message with exit 0; if you must use a file, write it in this same command and read the result back with 'git log -1 --format=%B'; and rewrite the message when amending so it states the combined change.".to_string())))
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `rm_tracked`'s test, a block for it and a
// predicate for rule `bounded_wait`'s arm (B): the first refusal, or none.
pub fn rm_tracked_reach(ctx: &Ctx, c: &Cmd) -> Result<Option<String>, Fault> {
    if ctx.expands(c, has_expansion)? {
        return Ok(None);
    }
    let s = ctx.view(c, SqDqHd)?;
    let dsegs = ctx.dequoted(c)?.map(|v| ctx.segments(&v)).unwrap_or_default();
    for (i, seg) in ctx.segments(&s).iter().enumerate() {
        let seg = trim_start(seg);
        let src = dsegs.get(i).filter(|d| !d.is_empty()).map_or(seg, String::as_str);
        let cmdseg = command_word(src);
        if head_word(&cmdseg) == "git" {
            let ws = git_subcommand(&cmdseg, GitWalk::Args).unwrap_or_default();
            if ws.first().map(String::as_str) != Some("rm") {
                continue;
            }
            for w in &ws[1..] {
                let w = w.replace('\\', "");
                if w == "--" {
                    break;
                }
                let force = matches!(w.as_str(), "--f" | "--fo" | "--for" | "--forc" | "--force")
                    || (w.len() >= 2 && w.starts_with('-') && !w[1..].starts_with('-') && w[1..].contains('f'));
                if force {
                    return Ok(Some(format!("don't force a 'git rm' with '{}' — the force flag is the one spelling of 'git rm' that destroys uncommitted work, silently when a committed grant matches it. Three exits: drop the flag, since 'git rm' refuses a file with local modifications and says so; use 'git rm --cached <path>' to untrack the file and keep it; or, where the loss is intended, run it yourself with !<command>.", w)));
                }
            }
            continue;
        }
        let word = head_word(seg);
        let paths = match ctx.shell() {
            Shell::Bash if word == "rm" => {
                text::all_words(&seg[word.len()..]).into_iter().filter(|a| !a.starts_with('-')).map(String::from).collect()
            }
            Shell::PowerShell if PS_DELETE.contains(&word.to_ascii_lowercase().as_str()) => {
                match ps_delete_paths(&seg[word.len()..]) {
                    Some(p) => p,
                    None => continue,
                }
            }
            _ => continue,
        };
        for arg in paths {
            if ctx.host().tracked(&arg) {
                return Ok(Some(format!("don't delete the git-tracked path '{a}' with a bare '{w}' — use 'git rm -q {a}': it removes the file and stages exactly that deletion in one motion, so no later 'git add -A' is needed to pick it up (which risks staging a concurrent session's foreign path). An '{w}' of an untracked or gitignored path is untouched. If you genuinely need {w}, run it yourself with !<command>.", a = arg, w = word)));
            }
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `rm_tracked`'s PowerShell deletion commands,
// matched without regard to case.
const PS_DELETE: &[&str] = &["rm", "del", "erase", "ri", "rd", "rmdir", "remove-item"];

// spec: guard-kit/SPEC.md §The generic ruleset — the value-taking parameters whose value is no path.
const PS_VALUE_PARAMS: &[&str] = &[
    "filter",
    "include",
    "exclude",
    "stream",
    "credential",
    "erroraction",
    "errorvariable",
    "warningaction",
    "warningvariable",
    "informationaction",
    "informationvariable",
    "outvariable",
    "outbuffer",
    "pipelinevariable",
    "progressaction",
];

// spec: guard-kit/SPEC.md §The generic ruleset — the paths a PowerShell deletion names, each
// parameter read by its prefix; `None` where a `-WhatIf` prefix makes the segment delete nothing.
fn ps_delete_paths(args: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut skip = false;
    for w in text::all_words(args) {
        if std::mem::take(&mut skip) {
            continue;
        }
        let Some(param) = w.strip_prefix('-') else {
            out.extend(w.split(',').filter(|p| !p.is_empty()).map(String::from));
            continue;
        };
        let (name, value) = match param.split_once(':') {
            Some((n, v)) => (n.to_ascii_lowercase(), Some(v)),
            None => (param.to_ascii_lowercase(), None),
        };
        if name.len() >= 2 && "whatif".starts_with(&name) {
            return None;
        }
        let valued = !name.is_empty() && PS_VALUE_PARAMS.iter().any(|p| p.starts_with(&name));
        match value {
            None => skip = valued,
            Some(v) if !valued => out.extend(v.split(',').filter(|p| !p.is_empty()).map(String::from)),
            Some(_) => {}
        }
    }
    Some(out)
}

pub fn rm_tracked(ctx: &Ctx) -> Decided {
    match rm_tracked_reach(ctx, ctx.cmd())? {
        Some(m) => block(m),
        None => Ok(None),
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — a path word as the rule compares it: under
// PowerShell `\` folds to `/`, rule `worktree_confinement`'s precedent.
fn path_word(ctx: &Ctx, w: &str) -> String {
    match ctx.shell() {
        Shell::PowerShell => w.replace('\\', "/"),
        Shell::Bash => w.to_string(),
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — the interpreter classification: arm (a) the bash/sh
// pair the runner serves, arm (b) the interpreter roster.
fn interpreter_arm(ctx: &Ctx, w: &str) -> Option<char> {
    let w = w.rsplit('/').next().unwrap_or(w);
    if matches!(w, "bash" | "sh") {
        return Some('a');
    }
    ctx.host().interpreters.iter().any(|i| i == w).then_some('b')
}

// spec: guard-kit/SPEC.md §The rule roster — arm (c)'s host word: the final path component less a
// trailing `.exe`, compared without regard to case under PowerShell.
fn powershell_host(ctx: &Ctx, w: &str) -> bool {
    let base = w.rsplit('/').next().unwrap_or(w);
    match ctx.shell() {
        Shell::PowerShell => {
            let b = base.to_ascii_lowercase();
            matches!(b.strip_suffix(".exe").unwrap_or(&b), "pwsh" | "powershell")
        }
        Shell::Bash => matches!(base.strip_suffix(".exe").unwrap_or(base), "pwsh" | "powershell"),
    }
}

enum Body {
    Inline,
    File(String),
    Stdin,
}

// spec: guard-kit/SPEC.md §The generic ruleset — a redirect word in either reader's grammar: `Some`
// with whether it stands alone and so takes the next word as its target.
fn redirect_word(tok: &str) -> Option<bool> {
    let t = tok.trim_start_matches(|c: char| c.is_ascii_digit() || c == '*' || c == '&');
    let op = t.strip_prefix(">>").or_else(|| t.strip_prefix('>')).or_else(|| t.strip_prefix('<'));
    let lead = tok.len() - t.len();
    match op {
        Some(rest) if lead <= 1 || tok.starts_with("&>") => Some(rest.is_empty()),
        _ => None,
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — an interpreter takes its program body from a -c/-e
// argument, its first bare operand, or stdin; `None` on an option the walk cannot size.
fn interpreter_body(seg: &str, arm: char) -> Option<Body> {
    let mut skip = false;
    for tok in words(seg).into_iter().skip(1) {
        if skip {
            skip = false;
            continue;
        }
        match tok {
            "-c" | "--command" | "-m" | "--module" => return Some(Body::Inline),
            "-e" | "--eval" => {
                if arm != 'a' {
                    return Some(Body::Inline);
                }
            }
            "-" | "/dev/stdin" | "/dev/fd/0" => return Some(Body::Stdin),
            "--" => {}
            _ if redirect_word(tok).is_some() => skip = redirect_word(tok) == Some(true),
            _ if tok.starts_with('-') => {
                let rest = &tok[1..];
                if rest.starts_with('-') || !rest.chars().all(|c| "BEIOilnstuvx".contains(c)) {
                    return None;
                }
            }
            _ => return Some(Body::File(tok.to_string())),
        }
    }
    Some(Body::Stdin)
}

// spec: guard-kit/SPEC.md §The rule roster — a PowerShell host's options, each matched by a prefix
// of its name without regard to case; the first three carry the body, the valued ones skip their
// value, and the order settles a prefix two names share.
const PS_BODY_OPTS: &[&str] = &["file", "command", "encodedcommand"];
const PS_VALUED_OPTS: &[&str] = &[
    "executionpolicy",
    "workingdirectory",
    "windowstyle",
    "outputformat",
    "inputformat",
    "configurationname",
    "configurationfile",
    "custompipename",
    "settingsfile",
    "psconsolefile",
    "version",
];
const PS_SWITCHES: &[&str] = &[
    "noprofile",
    "nologo",
    "noninteractive",
    "noexit",
    "interactive",
    "login",
    "sta",
    "mta",
    "noprofileloadtime",
];

// spec: guard-kit/SPEC.md §The rule roster — the host's own CLI grammar: `-File`'s operand or the
// first bare word is the body, `-Command` and `-EncodedCommand` carry it inline, a `-` after `-File`
// or `-Command` or no body word at all is stdin, and any other option declines.
fn powershell_body(seg: &str) -> Option<Body> {
    let toks = words(seg);
    let mut i = 1usize;
    while i < toks.len() {
        let tok = toks[i];
        i += 1;
        if let Some(alone) = redirect_word(tok) {
            i += usize::from(alone);
            continue;
        }
        if tok == "-" {
            return Some(Body::Stdin);
        }
        let Some(name) = tok.strip_prefix('-').map(str::to_ascii_lowercase) else {
            return Some(Body::File(tok.to_string()));
        };
        let is = |full: &str| !name.is_empty() && full.starts_with(name.as_str());
        match PS_BODY_OPTS.iter().find(|o| is(o)) {
            Some(&"file") => {
                return match toks.get(i) {
                    Some(&"-") => Some(Body::Stdin),
                    Some(w) => Some(Body::File(w.to_string())),
                    None => None,
                }
            }
            Some(&"command") if toks.get(i) == Some(&"-") => return Some(Body::Stdin),
            Some(_) => return Some(Body::Inline),
            None => {}
        }
        if matches!(name.as_str(), "ep" | "wd") || PS_VALUED_OPTS.iter().any(|o| is(o)) {
            i += 1;
        } else if !PS_SWITCHES.iter().any(|o| is(o)) {
            return None;
        }
    }
    Some(Body::Stdin)
}

// spec: guard-kit/SPEC.md §The generic ruleset — where a body source sits: under the main checkout's
// scratch dir seen from a linked worktree, or under a scratch dir the runner here serves.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Home {
    Here,
    Main,
}

// spec: guard-kit/SPEC.md §The generic ruleset — the scratch-source test on one token: a member as
// written, and from a linked worktree a member of the main checkout or of the session's own
// worktree, each compared lexically with `..` folded.
fn scratch_home(ctx: &Ctx, p: &str) -> Option<Home> {
    let p = path_word(ctx, p);
    let h = ctx.host();
    let own = h.own_scratch_homes();
    if !own.is_empty() {
        let t = h.lexical(&p);
        let main_homes = h.scratch_homes();
        if main_homes.iter().zip(&own).any(|(m, o)| m != o && under(&t, &h.lexical(m))) {
            return Some(Home::Main);
        }
        if own.iter().any(|o| under(&t, o)) {
            return Some(Home::Here);
        }
    }
    h.scratch_prefixes()
        .iter()
        .any(|(bare, dotted)| p.starts_with(bare.as_str()) || p.starts_with(dotted.as_str()))
        .then_some(Home::Here)
}

fn names_scratch(ctx: &Ctx, s: &str) -> bool {
    let s = path_word(ctx, s);
    ctx.host().scratch_prefixes().iter().any(|(bare, _)| s.contains(bare.as_str()))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the bash expansion decline: a substitution is a
// body source this rule reads, so only the parameter and process forms decline.
fn declines_expansion(raw: &str) -> bool {
    grep_q(r"\$\{|<\(|>\(|\$[A-Za-z_]", raw)
}

pub type Reach = (char, String, String, Home);

// spec: guard-kit/SPEC.md §The rule roster — arm (c) case 2, the PowerShell invocation forms: the
// call or dot-source operator before a scratch path, a scratch path in command position, and
// `Invoke-Expression` in a segment or behind a producer naming one.
fn powershell_form(ctx: &Ctx, seg: &str, producers: &[String]) -> Option<Reach> {
    let toks = words(seg);
    let word = *toks.first()?;
    let hit = |src: &str, home: Home| Some(('c', word.to_string(), src.to_string(), home));
    if matches!(word, "&" | ".") {
        let next = toks.get(1)?;
        return scratch_home(ctx, next).and_then(|home| hit(next, home));
    }
    if let Some(home) = scratch_home(ctx, word) {
        return hit(word, home);
    }
    if !matches!(word.to_ascii_lowercase().as_str(), "invoke-expression" | "iex") {
        return None;
    }
    let sources = toks[1..].iter().copied().chain(producers.iter().flat_map(|p| words(p)));
    for tok in sources {
        if let Some(home) = scratch_home(ctx, tok) {
            return hit(tok, home);
        }
    }
    None
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `script_interpreter`'s test, a block for it and
// a predicate for rule `bounded_wait`'s arm (B): the first interpreter taking its body from a scratch
// path, as its arm, its word and the body source.
pub fn interpreter_reach(ctx: &Ctx, c: &Cmd) -> Result<Option<Reach>, Fault> {
    let raw = ctx.raw(c)?;
    if !names_scratch(ctx, raw) || ctx.expands(c, declines_expansion)? {
        return Ok(None);
    }
    let ps = ctx.shell() == Shell::PowerShell;
    let s = ctx.view(c, SqDqHd)?;
    for stmt in ctx.statements(&s) {
        let pipes = ctx.pipes(&stmt);
        for (i, pipe) in pipes.iter().enumerate() {
            let seg = command_word(pipe);
            let word = path_word(ctx, head_word(&seg));
            if word.is_empty() {
                continue;
            }
            if ps {
                if let Some(r) = powershell_form(ctx, &seg, &pipes[..i]) {
                    return Ok(Some(r));
                }
            }
            let (arm, body) = if powershell_host(ctx, &word) {
                ('c', powershell_body(&seg))
            } else {
                let Some(arm) = interpreter_arm(ctx, &word) else { continue };
                (arm, interpreter_body(&seg, arm))
            };
            let hit = |src: &str, home: Home| Some((arm, word.clone(), src.to_string(), home));
            match body {
                None => continue,
                Some(Body::File(p)) => {
                    if let Some(home) = scratch_home(ctx, &p) {
                        return Ok(hit(&p, home));
                    }
                }
                Some(Body::Stdin) => {
                    for m in text::grep_o("(^|[^<])<[[:space:]]*[^[:space:]<>|;&]+", pipe) {
                        let src = m.strip_prefix(|ch: char| ch != '<').unwrap_or(&m);
                        let src = trim_start(src.strip_prefix('<').unwrap_or(src));
                        if let Some(home) = scratch_home(ctx, src) {
                            return Ok(hit(src, home));
                        }
                    }
                    for prev in &pipes[..i] {
                        for tok in words(prev) {
                            if let Some(home) = scratch_home(ctx, tok) {
                                return Ok(hit(tok, home));
                            }
                        }
                    }
                }
                Some(Body::Inline) if !ps => {
                    let subst = text::grep_o(r"`[^`]*`|\$\([^)]*\)", raw);
                    if subst.iter().any(|span| names_scratch(ctx, span)) {
                        return Ok(hit("a command substitution", Home::Here));
                    }
                }
                Some(Body::Inline) => {}
            }
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The shell guard — the steer door as the deciding reader's shell runs it:
// under PowerShell the call operator before the door single-quoted, any `'` doubled.
fn runner(ctx: &Ctx) -> String {
    let door = &ctx.host().door;
    match ctx.shell() {
        Shell::PowerShell => format!("& '{}' --scratch-run", door.replace('\'', "''")),
        Shell::Bash => format!("{} --scratch-run", door),
    }
}

pub fn script_interpreter(ctx: &Ctx) -> Decided {
    let Some((arm, word, src, home)) = interpreter_reach(ctx, ctx.cmd())? else { return Ok(None) };
    let runner = runner(ctx);
    let host = &ctx.host().scratch_powershell;
    let ps_on = arm == 'c' && !host.is_empty();
    let inline = if arm == 'c' {
        "A body carried in the command string — a '-Command' argument, a here-string — is untouched"
    } else {
        "A body carried in the command string — a '-c' argument, a heredoc, a herestring — is untouched"
    };
    let off = if arm == 'c' && host.is_empty() {
        "the PowerShell scratch path is off in this project (GUARD_KIT_SCRATCH_POWERSHELL is empty; guard-kit/SPEC.md §scratch-run), and "
    } else {
        ""
    };
    if home == Home::Main {
        let own = ctx.host().own_scratch_homes().into_iter().next().unwrap_or_default();
        let lead = if arm == 'a' || ps_on {
            let script = if ps_on { "<script>.ps1" } else { "<script>" };
            format!("run a scratch script through the runner from this linked worktree: write it under the worktree's own scratch dir ({own}) and run '{runner} {script} [args…]' there, since the runner refuses a script under the main checkout's scratch dir from here (guard-kit/SPEC.md §scratch-run). This call takes the program body for '{word}' from '{src}'")
        } else if arm == 'c' {
            format!("{off}this call takes the program body for '{word}' from '{src}'. Write the body as a bash script under this linked worktree's own scratch dir ({own}) and run it through '{runner} <script> [args…]' there, since the runner refuses a script under the main checkout's scratch dir from here. The body sits")
        } else {
            format!("scratch execution runs in bash, or in PowerShell where the project names a host (guard-kit/SPEC.md §scratch-run), and '{word}' is neither: this call takes its program body from '{src}'. Write the body as a shell script under this linked worktree's own scratch dir ({own}) and run it through '{runner} <script> [args…]' there, since the runner refuses a script under the main checkout's scratch dir from here. The body sits")
        };
        return block(format!("{lead} in a scratch dir any session can rewrite, so the body reviewed at the permission decision need not be the body that runs; the runner echoes the body as it executes, which is the compensating control a direct run has none of. {inline}. If you genuinely need the direct form, run it yourself with !<command>."));
    }
    if arm == 'a' || ps_on {
        let (script, runs) = if ps_on {
            ("<script>.ps1", format!(", which echoes the body and runs it under the project's PowerShell host ({host})"))
        } else {
            ("<script>", String::new())
        };
        return block(format!("run a scratch script through the runner: '{runner} {script} [args…]'{runs} (guard-kit/SPEC.md §scratch-run). This call takes the program body for '{word}' from '{src}', which sits in a scratch dir any session can rewrite, so the body reviewed at the permission decision need not be the body that runs. The runner is allowlistable and echoes the body as it executes, which is the compensating control a direct run has none of. {inline}. If you genuinely need the direct form, run it yourself with !<command>."));
    }
    if arm == 'c' {
        return block(format!("{off}this call takes the program body for '{word}' from '{src}' under a scratch dir, where no compensating control reaches it. Write the body as a bash script under the scratch dir and run it through '{runner} <script> [args…]', which echoes the body as it executes. {inline}, because the approver and the friction log both see it verbatim. If you genuinely need the direct run, run it yourself with !<command>."));
    }
    block(format!("scratch execution runs in bash, or in PowerShell where the project names a host (guard-kit/SPEC.md §scratch-run), and '{word}' is neither: this call takes its program body from '{src}' under a scratch dir, where no compensating control reaches it. Write the body as a shell script and run it through '{runner} <script> [args…]', which echoes the body as it executes; a script whose shebang names a non-bash interpreter is refused there too. {inline}, because the approver and the friction log both see it verbatim. If you genuinely need the direct run, run it yourself with !<command>."))
}

pub fn shell_wrapper(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    let wrapped = raw.find("sh").is_some_and(|i| raw[i + 2..].contains("-c"));
    if !wrapped {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    for seg in ctx.segments(&s) {
        let seg = trim_start(&seg);
        let word = head_word(seg);
        if !matches!(word, "bash" | "sh") {
            continue;
        }
        if head_word(trim_start(&seg[word.len()..])) != "-c" {
            continue;
        }
        return block(format!("don't wrap a command in '{} -c': the payload sits inside one quoted argument, so neither the allowlist nor any guard rule can read what it runs. Run the payload as the command itself, or, for a body that needs a shell of its own, write it to a scratch script and run it through '{} --scratch-run <script>'. If you genuinely need the wrapper, run it yourself with !<command>.", word, ctx.host().door));
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `worktree_confinement`'s test as a predicate,
// taken under that rule's own declared views.
pub fn refused(ctx: &Ctx, c: &Cmd) -> Result<bool, Fault> {
    Ok(ctx.under("worktree_confinement", |w| worktree_refuses(w, c))?.is_some())
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `worktree_confinement`'s test: from a linked
// worktree, the first path word resolving into the main checkout outside its scratch dirs, unless
// the command is the admitted read.
pub fn worktree_refuses(ctx: &Ctx, c: &Cmd) -> Result<Option<(String, String)>, Fault> {
    let h = ctx.host();
    if h.roots().main.is_empty() {
        return Ok(None);
    }
    let ps = ctx.shell() == Shell::PowerShell;
    let live = ctx.view(c, SqHdq)?;
    let declines = if ps {
        live.contains('$')
    } else {
        grep_q(r"\$\{|\$\(|<\(|>\(|\$[A-Za-z_]", &live) || ctx.raw(c)?.contains('`')
    };
    if declines {
        return Ok(None);
    }
    let s = ctx.view(c, SqDqHd)?;
    let v = ctx.dequoted(c)?.unwrap_or_else(|| s.clone());
    let mut found: Option<(String, String)> = None;
    'lines: for line in v.split('\n') {
        for w in words(line) {
            let mut w = unsentinel(w);
            if ps {
                w = w.replace('\\', "/");
                if w.starts_with("*>") {
                    w.remove(0);
                }
            }
            let digits = w.bytes().take_while(u8::is_ascii_digit).count();
            let after = &w[digits..];
            let op = [">>", ">&", "&>", ">", "<"].iter().find(|o| after.starts_with(*o));
            if let Some(op) = op {
                w = after[op.len()..].to_string();
            }
            if w.starts_with("--") && w.len() > 3 && w.as_bytes()[3..].contains(&b'=') {
                w = w.split_once('=').map_or(w.clone(), |(_, r)| r.to_string());
            }
            if walk::path_root(&w).is_none() {
                let dotdot = w == ".." || w.starts_with("../") || w.ends_with("/..") || w.contains("/../");
                if !dotdot {
                    continue;
                }
            }
            let t = h.lexical(&w);
            if !h.in_main(&t) || h.in_scratch(&t) {
                continue;
            }
            found = Some((w, t));
            break 'lines;
        }
    }
    let Some(hit) = found else { return Ok(None) };
    if !ps && worktree_admitted(ctx, c, &s)? {
        return Ok(None);
    }
    Ok(Some(hit))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the admitted read: every segment a roster read in
// none of its declared write and execute forms, none led by a program-bearing tool, and every
// redirect target inert, in the own worktree or under a main-checkout scratch dir.
fn worktree_admitted(ctx: &Ctx, c: &Cmd, s: &str) -> Result<bool, Fault> {
    let h = ctx.host();
    if h.worktree_reads != "read-only" || shell_backgrounds(s) {
        return Ok(false);
    }
    let mut reads = 0usize;
    for seg in ctx.segments(s) {
        let seg = trim_start(&seg);
        if seg.is_empty() || is_banner(seg) {
            continue;
        }
        let cw = command_word(&segment_core(seg));
        if program_bearing(h, head_word(&cw)) || !is_ro_segment(h, &cw) {
            return Ok(false);
        }
        reads += 1;
    }
    if reads == 0 {
        return Ok(false);
    }
    for pair in ctx.redirect_pairs(s) {
        if pair.is_empty() {
            continue;
        }
        let tgt = super::strip_redirect_op(&pair);
        if inert_target(tgt) {
            continue;
        }
        let t = h.lexical(tgt);
        if under(&t, &h.roots().own) || (h.in_main(&t) && h.in_scratch(&t)) {
            continue;
        }
        return Ok(false);
    }
    ro_forms_clear(ctx, c)
}

pub fn worktree_confinement(ctx: &Ctx) -> Decided {
    let h = ctx.host();
    let Some((word, path)) = worktree_refuses(ctx, ctx.cmd())? else { return Ok(None) };
    let reads = if h.worktree_reads == "read-only" && ctx.shell() == Shell::Bash {
        format!(" a search or read of it as a read-only pipeline, every segment led by one of the read-only roster ({}) in none of its write or execute forms and no segment led by sed, awk or an interpreter, redirecting only to /dev/null, your own worktree or the main checkout's scratch dir;", h.ro_bins.join(" "))
    } else {
        String::new()
    };
    block(format!("'{}' reaches the main checkout ({}) from this linked worktree, and a shell command naming the main checkout outside its scratch dir is refused here: an isolated session never alters the work tree it was isolated from. Lawful routes: the Read tool for a named file;{} a history read (git log, git show) in your own worktree, which shares the main checkout's refs and objects; and your journal, appended by shell under the main checkout's {} dir. If the command is genuinely needed, return and have your dispatcher run it in the main checkout.", word, path, reads, h.first_scratch()))
}
