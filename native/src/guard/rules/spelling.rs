// spec: guard-kit/SPEC.md §The generic ruleset — the spelling rules: a command form the matcher cannot
// resolve, steered to the one it can.
use super::grants::rewrite_granted;
use super::{command_word, git_subcommand, GitWalk};
use crate::guard::engine::{Cmd, Ctx, Decided, Verdict};
use crate::guard::host::{knob_scalar_value, same_file};
use crate::guard::reader::View::{SqDqHd, SqDqHdq, SqHdq};
use crate::guard::reader::{DQ_MARK, SQ_MARK};
use crate::guard::text::{grep_q, head_word, trim, trim_end, trim_start, words};

fn block(m: impl Into<String>) -> Decided {
    Ok(Some(Verdict::Block(m.into())))
}

pub fn cd_compound(ctx: &Ctx) -> Decided {
    let cmd = ctx.view(ctx.cmd(), SqDqHd)?;
    if grep_q("(^|[;&|(])[[:space:]]*cd[[:space:]]", &cmd) && grep_q("[;&|]", &cmd) {
        return block("don't use 'cd' in a compound command (cwd drift, and the allowlist can't match the compound — the call costs an out-of-band permission decision). Pass absolute paths, or 'git -C <dir>' for git.");
    }
    Ok(None)
}

pub fn git_c_root(ctx: &Ctx) -> Decided {
    let pwd = &ctx.host().pwd;
    let raw = ctx.raw(ctx.cmd())?;
    let cmd = ctx.view(ctx.cmd(), SqDqHd)?;
    let needle = format!("git -C {} ", pwd);
    if cmd.split('\n').any(|l| l.contains(&needle)) {
        return block(format!("drop 'git -C {} ' — cwd is the repo root, so the bare 'git <subcommand>' form is allowlisted and resolves on the match; the absolute '-C' spelling matches nothing and costs an out-of-band permission decision. Reserve 'git -C <dir>' for a different repo.", pwd));
    }
    let git_c = cmd.find("git").is_some_and(|i| cmd[i + 3..].contains("-c"));
    if !(git_c || ["/time", "/nice", "/nohup", "/stdbuf", "="].iter().any(|n| cmd.contains(n))) {
        return Ok(None);
    }
    for seg in ctx.segments(&cmd) {
        if seg.contains('=') {
            if let Some(v) = knob_echo(raw, &seg) {
                return Ok(Some(v));
            }
        }
        let cmdseg = command_word(&seg);
        let w = head_word(&cmdseg);
        if w == "git" {
            if let Some(globals) = git_subcommand(&cmdseg, GitWalk::Globals) {
                if globals.iter().any(|g| g == "-c") {
                    return block("drop the 'git -c <key>=<value>' override and run the bare 'git <subcommand>' — a pager or color override has no effect without a terminal, and the bare form is allowlisted and resolves on the match. A '-c' form is never granted: a '-c' key can name a program the subcommand runs (core.pager, core.fsmonitor, core.sshCommand, alias.*), which is also why the harness's matcher does not see through it. If you genuinely need the config override, run it yourself with !<command>.");
                }
            }
        }
        if !w.contains('/') {
            continue;
        }
        let base = w.rsplit('/').next().unwrap_or(w);
        if matches!(base, "time" | "timeout" | "nice" | "nohup" | "stdbuf") {
            return block(format!("use the bare wrapper name '{b}' rather than '{w}' — the harness's matcher strips a bare '{b}' before it matches, so the wrapped command resolves on its own allowlist entry, while an absolute spelling is matched as itself and costs an out-of-band permission decision. A format option only the binary takes ('/usr/bin/time -f') has no stripped spelling; if you genuinely need it, run it yourself with !<command>.", b = base, w = w));
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `git_c_root`'s arm (d), one skeleton segment at
// a time: the raw command is read only to spell the corrective.
fn knob_echo(raw: &str, seg: &str) -> Option<Verdict> {
    let mut ws = words(seg);
    let mut shape = "lead";
    match ws.first().copied() {
        Some("export") => {
            shape = "export";
            ws.remove(0);
        }
        Some("env") => {
            shape = "env";
            ws.remove(0);
        }
        _ => {}
    }
    let is_asg = |w: &str| {
        let b = w.as_bytes();
        match w.find('=') {
            Some(eq) if eq > 0 => {
                (b[0].is_ascii_alphabetic() || b[0] == b'_')
                    && b[..eq].iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_')
            }
            _ => false,
        }
    };
    let asg: Vec<&str> = ws.iter().copied().take_while(|w| is_asg(w)).collect();
    if asg.is_empty() {
        return None;
    }
    let i = asg.len();
    match shape {
        "export" if i != ws.len() => return None,
        "lead" if i >= ws.len() => return None,
        _ => {}
    }
    for w0 in &asg {
        let (name, val) = w0.split_once('=').unwrap_or((w0, ""));
        if [SQ_MARK, DQ_MARK, "$", "`", "\\"].iter().any(|n| val.contains(n)) {
            continue;
        }
        let Some(resolved) = knob_scalar_value(name) else { continue };
        let mut respelt = String::new();
        if val != resolved {
            if !(std::path::Path::new(val).exists() && same_file(val, &resolved)) {
                continue;
            }
            respelt = format!(" '{}' names the same file as '{}' but is not its text, and the generated hooks bake the knob's text, so the battery it prefixes reds check-graph on a stale hook.", val, resolved);
        }
        let w: String = if shape == "export" && asg.len() == 1 { trim(seg).to_string() } else { w0.to_string() };
        let (mut pre, mut post) = match raw.find(&w) {
            Some(at) => (raw[..at].to_string(), raw[at + w.len()..].to_string()),
            None => (raw.to_string(), raw.to_string()),
        };
        post = trim_start(&post).to_string();
        if w.starts_with("export") {
            if post.starts_with("&&") || post.starts_with(';') {
                let p = post.strip_prefix("&&").unwrap_or(&post);
                let p = p.strip_prefix(';').unwrap_or(p);
                post = trim_start(p).to_string();
            }
        } else if shape == "env" && asg.len() == 1 {
            let tail = trim_end(&pre);
            if let Some(t) = tail.strip_suffix("env") {
                pre = t.to_string();
            }
        }
        if post.is_empty() {
            let p = trim_end(&pre);
            let p = p.strip_suffix("&&").unwrap_or(p);
            let p = p.strip_suffix(';').unwrap_or(p);
            pre = trim_end(p).to_string();
        }
        return Some(Verdict::Block(format!("drop the '{w}' prefix — {name} already resolves to '{resolved}', so the prefix buys nothing, and the matcher cannot see past an assignment or an export, so it costs an out-of-band permission decision.{respelt} Run it without the prefix: {pre}{post}")));
    }
    None
}

pub fn scratch_redirect(ctx: &Ctx) -> Decided {
    let cmd = ctx.view(ctx.cmd(), SqDqHd)?;
    if grep_q(
        "(^|[[:space:]])([0-9]*|&)>>?[[:space:]]*[^[:space:]/|&]+\\.(err|out|log)([[:space:]]|$)",
        &cmd,
    ) {
        return block(format!("don't redirect scratch to a bare repo-root filename (e.g. 2> op.err) — it pollutes cwd and risks a 'git add -A'. Send it to a gitignored scratch dir (e.g. {}/<name>.err).", ctx.host().first_scratch()));
    }
    Ok(None)
}

pub fn abs_script(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    let cmd = ctx.view(ctx.cmd(), SqDqHd)?;
    let pre = ctx.host().pwd_prefix();
    let bash_pre = format!("bash {}", pre);
    let rest = if let Some(r) = cmd.strip_prefix(&bash_pre) {
        r
    } else if let Some(r) = cmd.strip_prefix(&pre) {
        r
    } else {
        return Ok(None);
    };
    let rest = head_word(rest);
    if !rest.ends_with(".sh") {
        return Ok(None);
    }
    let base = rest.rsplit('/').next().unwrap_or(rest);
    let relcmd = raw.replace(&pre, "");
    for g in &ctx.host().ro_scripts {
        if crate::walk::glob_match(g, base) || crate::walk::glob_match(g, rest) {
            if rewrite_granted(ctx, &Cmd::new(relcmd.clone()))? {
                return Ok(Some(Verdict::Rewrite(
                    relcmd,
                    "abs repo read-only script normalized to relative (shell-guard)".to_string(),
                )));
            }
            break;
        }
    }
    block(format!("use the repo-relative form '{}' (cwd is the repo root) — an allowlist entry is written against the relative spelling, so it is the one that can resolve on the match; the absolute spelling matches nothing and costs an out-of-band permission decision. If you truly need the absolute path, run it yourself with !<command>.", rest))
}

pub fn abs_prefix(ctx: &Ctx) -> Decided {
    let pwd = &ctx.host().pwd;
    let cmd = ctx.view(ctx.cmd(), SqDqHd)?;
    if cmd.starts_with("git ") {
        return Ok(None);
    }
    let needle = ctx.host().pwd_prefix();
    if cmd.split('\n').any(|l| l.contains(&needle)) {
        return block(format!("drop the repo-root absolute prefix '{}/' — cwd is the repo root, so the repo-relative path is allowlisted and resolves on the match; the absolute spelling matches nothing and costs an out-of-band permission decision. If you truly need the absolute path, run it yourself with !<command>.", pwd));
    }
    Ok(None)
}

pub fn expansion(ctx: &Ctx) -> Decided {
    let sqexp = ctx.view(ctx.cmd(), SqHdq)?;
    if grep_q(r"\$\{|\$\(|<\(|\$[A-Za-z_]", &sqexp) {
        return block("avoid shell variables/expansions ($VAR, ${...}, $(...), <(...)) — the harness's matcher refuses every expansion, so no allowlist entry can match the command and it costs an out-of-band permission decision. Inline the literal path, use a relative path, or 'git -C <dir>'. If you genuinely need the expansion, run it yourself with !<command>.");
    }
    let expn = ctx.view(ctx.cmd(), SqDqHdq)?;
    if grep_q(
        "(^|[;(]|&&|\\|\\|)[[:space:]]*[A-Za-z_][A-Za-z0-9_]*=[^[:space:];|&]*[[:space:]]*($|;)",
        &expn,
    ) {
        return block("avoid shell variable assignments (NAME=value; ... $NAME) — they defeat allowlist matching, so the call costs an out-of-band permission decision no allowlist entry can pre-empt. Inline the literal value/path at each use site, or 'git -C <dir>'. If you genuinely need it, run it yourself with !<command>.");
    }
    Ok(None)
}

pub fn brace_glyph(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    let sq = ctx.view(ctx.cmd(), SqDqHd)?;
    if !sq.contains('{') {
        return Ok(None);
    }
    let resid = sq.replace("{}", "");
    if !resid.contains('{') && !resid.contains('}') {
        let rw = raw.replace("{}", "'{}'");
        if rewrite_granted(ctx, &Cmd::new(rw.clone()))? {
            return Ok(Some(Verdict::Rewrite(
                rw,
                "bare {} placeholder single-quoted so the harness matcher passes it (shell-guard)".to_string(),
            )));
        }
        return block("quote each bare {} placeholder yourself, spelling it '{}' — the harness's matcher refuses a bare '{' glyph, and this guard rewrites the placeholder silently only when the rewritten command is allowlisted and stays inside its grant's path slots, which this one does not. The quoted spelling passes the same literal {} to the command, and the call then reaches the harness's own permission decision. If you genuinely need the bare form, run it yourself with !<command>.");
    }
    if sq.split('\n').any(|l| l.contains("@{")) {
        return block("spell out the git-ref shorthand '@{...}' — the harness's matcher refuses the '{' glyph, so the call costs an out-of-band permission decision. Use 'origin/<branch>..HEAD' for '@{u}..', or the resolved ref/hash for a reflog form.");
    }
    if grep_q(r"\{[^}]*(,|\.\.)[^}]*\}", &sq) {
        return block("write out the brace expansion '{a,b}'/'{a..b}' — the harness's matcher refuses the '{' glyph and no allowlist entry can match around it, so the call costs an out-of-band permission decision. Spell the members (e.g. 'mkdir -p a/b a/c') or use a loop for a long range.");
    }
    block("quote the '{' if it's literal (an unquoted awk/sed program), or write it out if it expands — the harness's matcher refuses every bare '{' glyph before allowlist matching, so the call is decided out of band. A brace inside quotes of either kind, or in a heredoc body, is already inert and never reaches this block.")
}
