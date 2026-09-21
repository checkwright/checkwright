// spec: canon-kit/SPEC.md §check-fence-command-head — every command in a shell fence of the governed
// doc set starts with a word that can run, and a word beginning with `-` never can
use crate::bashscan::{self, Kind, Token};
use crate::{proc, programs, spec, walk};
use std::collections::HashSet;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-fence-command-head: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-fence-command-head — the fence languages whose body is a command
// sequence a reader pastes into a shell
const SHELL_LANGS: [&str; 3] = ["bash", "sh", "shell"];

// spec: canon-kit/SPEC.md §check-fence-command-head — the shell's own grammar: bash's builtins and
// reserved words, which run with nothing on PATH
const SHELL_WORDS: &[&str] = &[
    ".", ":", "[", "[[", "!", "{", "}", "alias", "bg", "bind", "break", "builtin", "caller",
    "case", "cd", "command", "compgen", "complete", "continue", "declare", "dirs", "disown", "do",
    "done", "echo", "elif", "else", "enable", "esac", "eval", "exec", "exit", "export", "false",
    "fc", "fg", "fi", "for", "function", "getopts", "hash", "help", "history", "if", "in", "jobs",
    "kill", "let", "local", "logout", "mapfile", "popd", "printf", "pushd", "pwd", "read",
    "readarray", "readonly", "return", "select", "set", "shift", "shopt", "source", "suspend",
    "test", "then", "time", "times", "trap", "true", "type", "typeset", "ulimit", "umask",
    "unalias", "unset", "until", "wait", "while",
];

struct Ctx {
    top: String,
    cwd: String,
    tracked: HashSet<String>,
    programs: HashSet<String>,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let top = walk::toplevel()
        .map_err(|_| "not a git repository — cannot resolve tracked paths".to_string())?;
    let exclude = spec::knob_array_pub("CANON_KIT_MDREF_EXCLUDE")?;
    let files: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        spec::manifest_files(".")?
            .into_iter()
            .map(|p| p.display().to_string())
            .filter(|f| {
                let rel = spec::strip_dot_slash(f);
                !exclude.iter().any(|g| walk::pattern_match(g, &rel))
            })
            .collect()
    };
    let ctx = Ctx {
        tracked: tracked_files(&top)?,
        cwd: walk::cwd()?,
        top,
        programs: spec::vocabulary("CANON_KIT_FENCE_PROGRAMS", "CANON_KIT_FENCE_PROGRAMS_EXTRA")?
            .into_iter()
            .collect(),
    };

    let mut bad: Vec<String> = Vec::new();
    let mut nfence = 0usize;
    let mut nhead = 0usize;
    for f in &files {
        if !Path::new(f).is_file() {
            continue;
        }
        let docdir = dirname(f);
        let text = spec::read_text(Path::new(f))?;
        for (open_ln, body) in shell_fences(&text) {
            nfence += 1;
            for (ln, word, why) in judge_fence(&ctx, &docdir, &body, &mut nhead)? {
                bad.push(format!("{}:{}: '{}' {}", f, open_ln + ln, word, why));
            }
        }
    }

    if !bad.is_empty() {
        println!("check-fence-command-head: shell-fence command(s) whose first word cannot run:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: start the command with what runs it — a tool reached through a sourced library");
        println!("        function or a variable the fence assigns, a tracked script path, or a program");
        println!("        named in CANON_KIT_FENCE_PROGRAMS_EXTRA. A fence that is not meant to be pasted");
        println!("        into a shell takes another info string (text, console).");
        return Ok(1);
    }
    println!(
        "FENCE-COMMAND-HEAD: clean ({} doc(s); {} shell fence(s); {} command head(s) can run)",
        files.len(),
        nfence,
        nhead
    );
    Ok(0)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the tracked set is read from the repository
// toplevel, and a failed listing is the check that could not run
fn tracked_files(top: &str) -> Result<HashSet<String>, String> {
    let out = proc::run(&programs::GIT, &["-C", top, "ls-files", "-z"])?;
    match out.code() {
        Some(0) => Ok(String::from_utf8_lossy(out.stdout().unwrap_or(&[]))
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect()),
        Some(c) => Err(format!("git ls-files failed (exit {}) listing tracked files", c)),
        None => Err("git ls-files failed (killed by a signal) listing tracked files".to_string()),
    }
}

// spec: canon-kit/SPEC.md §check-fence-command-head — a fence toggles on the shared fence shape, and
// its info string's first word, case-folded, is its language
fn shell_fences(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut open: Option<(usize, bool, String)> = None;
    for (idx, raw) in text.lines().enumerate() {
        if spec::is_fence_line(raw) {
            match open.take() {
                Some((ln, true, body)) => out.push((ln, body)),
                Some(_) => {}
                None => {
                    let info = raw.trim_start().trim_start_matches('`').trim();
                    let lang = info.split_whitespace().next().unwrap_or("").to_ascii_lowercase();
                    open = Some((idx + 1, SHELL_LANGS.contains(&lang.as_str()), String::new()));
                }
            }
            continue;
        }
        if let Some((_, _, body)) = open.as_mut() {
            body.push_str(raw);
            body.push('\n');
        }
    }
    out
}

fn judge_fence(
    ctx: &Ctx,
    docdir: &str,
    body: &str,
    nhead: &mut usize,
) -> Result<Vec<(usize, String, &'static str)>, String> {
    let mut funcs = defined_functions(body);
    let mut bad = Vec::new();
    for t in bashscan::command_heads(body) {
        if matches!(t.kind, Kind::Guard) {
            continue;
        }
        *nhead += 1;
        if matches!(t.kind, Kind::Keyword | Kind::Expansion) {
            continue;
        }
        if t.word == "." || t.word == "source" {
            if let Some(text) = sourced_text(ctx, docdir, &t)? {
                funcs.extend(defined_functions(&text));
            }
        }
        if let Some(why) = verdict(ctx, docdir, &funcs, &t.word) {
            bad.push((t.line, t.word.clone(), why));
        }
    }
    Ok(bad)
}

// spec: canon-kit/SPEC.md §check-fence-command-head — the accepted heads, in the section's order;
// `None` is a head that runs, `Some` the reason one does not
fn verdict(ctx: &Ctx, docdir: &str, funcs: &HashSet<String>, raw: &str) -> Option<&'static str> {
    if raw.starts_with('-') {
        return Some("is a flag, and a flag names no command");
    }
    let lead = raw.trim_start_matches('"');
    if lead.starts_with('`') || (lead.starts_with('$') && lead.len() > 1) {
        return None;
    }
    if raw == "$" {
        return Some("is a copied shell prompt, which runs nothing");
    }
    let w: String = raw.chars().filter(|c| *c != '"' && *c != '\'').collect();
    if w == "..." || w == "…" || (w.starts_with('<') && w.ends_with('>')) {
        return None;
    }
    if SHELL_WORDS.contains(&w.as_str()) || funcs.contains(&w) || ctx.programs.contains(&w) {
        return None;
    }
    if let Some(rest) = w.strip_prefix('/').or_else(|| w.strip_prefix("~/")) {
        let base = rest.rsplit('/').next().unwrap_or(rest);
        if ctx.programs.contains(base) || SHELL_WORDS.contains(&base) {
            return None;
        }
        return Some("is a machine path naming no configured program");
    }
    if w.contains('/') {
        if tracked_path(ctx, docdir, &w).is_some() {
            return None;
        }
        return Some("is a path the repository does not track");
    }
    Some("names no builtin, configured program, fence-defined or sourced function, or tracked path")
}

// spec: canon-kit/SPEC.md §check-fence-command-head — a relative path resolves against the doc's own
// directory first and the working directory second, the `check-docs-cmd` order
fn tracked_path(ctx: &Ctx, docdir: &str, tok: &str) -> Option<String> {
    let tok = tok.strip_prefix("./").unwrap_or(tok);
    let docabs = if walk::path_root(docdir).is_some() {
        docdir.to_string()
    } else {
        format!("{}/{}", ctx.cwd, docdir)
    };
    for base in [docabs, ctx.cwd.clone()] {
        let abs = walk::normalize_abs(&format!("{}/{}", base, tok));
        if let Some(rel) = walk::rel_under(&ctx.top, &abs) {
            if ctx.tracked.contains(rel) {
                return Some(abs.clone());
            }
        }
    }
    None
}

// spec: canon-kit/SPEC.md §check-fence-command-head — a sourced operand's `${NAME:-default}` reads as
// its default, the value a tree that sets no locator runs with; an operand that stays an expansion
// after that, or names no tracked file, contributes no function
fn sourced_text(ctx: &Ctx, docdir: &str, t: &Token) -> Result<Option<String>, String> {
    let op = match t.operands.iter().find(|o| !o.starts_with('-')) {
        Some(o) => o,
        None => return Ok(None),
    };
    let path = defaulted(&op.replace(['"', '\''], ""));
    if path.contains('$') {
        return Ok(None);
    }
    match tracked_path(ctx, docdir, &path) {
        Some(abs) => spec::read_text(Path::new(&abs)).map(Some),
        None => Ok(None),
    }
}

fn defaulted(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("${") {
        out.push_str(&rest[..i]);
        let tail = &rest[i + 2..];
        let end = match tail.find('}') {
            Some(e) => e,
            None => {
                out.push_str(&rest[i..]);
                return out;
            }
        };
        let inner = &tail[..end];
        match inner.split_once(":-").or_else(|| inner.split_once('-')) {
            Some((_, d)) => out.push_str(d),
            None => {
                out.push_str("${");
                out.push_str(inner);
                out.push('}');
            }
        }
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}

// spec: canon-kit/SPEC.md §check-fence-command-head — a function is defined by `name()` or
// `function name` at the start of a line
fn defined_functions(text: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    for line in text.lines() {
        let l = line.trim_start();
        let (l, kw) = match l.strip_prefix("function ") {
            Some(r) => (r.trim_start(), true),
            None => (l, false),
        };
        let n = l
            .bytes()
            .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-'))
            .count();
        if n == 0 || l.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        if kw || l[n..].trim_start().starts_with("()") {
            out.insert(l[..n].to_string());
        }
    }
    out
}

fn dirname(p: &str) -> String {
    match p.rfind('/') {
        Some(0) => "/".to_string(),
        Some(i) => p[..i].to_string(),
        None => ".".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(programs: &[&str]) -> Ctx {
        Ctx {
            top: "/r".into(),
            cwd: "/r".into(),
            tracked: ["lib/tool.sh".to_string(), "bin/run.sh".to_string()].into_iter().collect(),
            programs: programs.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn heads_red(c: &Ctx, body: &str) -> Vec<String> {
        let mut funcs = defined_functions(body);
        let mut out = Vec::new();
        for t in bashscan::command_heads(body) {
            if matches!(t.kind, Kind::Guard | Kind::Keyword | Kind::Expansion) {
                continue;
            }
            if t.word == "." {
                funcs.insert("tool_fn".to_string());
            }
            if verdict(c, ".", &funcs, &t.word).is_some() {
                out.push(t.word);
            }
        }
        out
    }

    #[test]
    fn a_flag_head_is_red_wherever_the_command_starts() {
        let c = ctx(&["git"]);
        assert_eq!(heads_red(&c, "--run-demo\n"), vec!["--run-demo"]);
        assert_eq!(heads_red(&c, "git status && --emit x\n"), vec!["--emit"]);
        assert_eq!(heads_red(&c, "git log \\\n  --oneline\n"), Vec::<String>::new());
    }

    #[test]
    fn expansions_builtins_functions_and_tracked_paths_run() {
        let c = ctx(&["git"]);
        let body = ". lib/tool.sh && door=\"$(tool_fn)\"\n\"$door\" --emit x\nbin/run.sh\nf() { :; }\nf\n";
        assert_eq!(heads_red(&c, body), Vec::<String>::new());
        assert_eq!(heads_red(&c, "bin/ghost.sh\n"), vec!["bin/ghost.sh"]);
        assert_eq!(heads_red(&c, "widgetctl up\n"), vec!["widgetctl"]);
        assert_eq!(heads_red(&c, "/usr/bin/git x\n"), Vec::<String>::new());
    }

    #[test]
    fn a_heredoc_body_and_a_case_pattern_are_not_heads() {
        let c = ctx(&["cat"]);
        let body = "cat > f <<'EOF'\n--not-a-command\nEOF\ncase \"$x\" in\n  a) cat ;;\nesac\n";
        assert_eq!(heads_red(&c, body), Vec::<String>::new());
    }

    #[test]
    fn a_locator_default_is_the_path_a_sourced_operand_reads() {
        assert_eq!(defaulted("${GATE_SDK_ROOT:-gate-sdk}/lib/gate.sh"), "gate-sdk/lib/gate.sh");
        assert_eq!(defaulted("${ROOT}/lib/gate.sh"), "${ROOT}/lib/gate.sh");
    }

    #[test]
    fn only_a_shell_fence_is_read() {
        let text = "```bash\na\n```\n```text\nb\n```\n```sh\nc\n```\n";
        let f = shell_fences(text);
        assert_eq!(f, vec![(1, "a\n".to_string()), (7, "c\n".to_string())]);
    }
}
