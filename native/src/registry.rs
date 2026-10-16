// spec: gate-sdk/SPEC.md §lib/gate.sh — the crate-side registry layer: the `gates.list` member
// grammar, `gate_resolve`'s declaration path, the `# graph:` field read and the couples/trigger
// expansion, in one module rather than a private copy per reader
// spec: gate-sdk/SPEC.md §The non-gate arm — a universal layer, so no `.gate` descriptor couples
// it: an edit here can change every member's verdict, which is what §check-crate-arms and the
// binary's source stamp hold, exactly as they hold `walk.rs` and `proc.rs`
use crate::fresh;
use std::path::Path;

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gates_list_members`: every line that is neither blank
// nor a comment, in file order
pub fn members(text: &str) -> Vec<String> {
    member_lines(text).into_iter().map(|(_, l)| l.to_string()).collect()
}

// spec: gate-sdk/SPEC.md §run-gates — each member with its 1-based registry line, the location a
// red member with no finding line of its own takes in the SARIF log
pub fn member_lines(text: &str) -> Vec<(usize, &str)> {
    fresh::file_lines(text)
        .into_iter()
        .enumerate()
        .filter(|(_, l)| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .map(|(i, l)| (i + 1, l))
        .collect()
}

// spec: gate-sdk/SPEC.md §Layout and configuration — the registration file's one declaration form,
// `# unregistered: <name> — <reason>`: §Consumer smoke's grammar on a second roster, both fields
// read, and invisible to `members` above because every reader of that one drops comment lines.
pub fn unregistered_declarations(text: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for l in fresh::file_lines(text) {
        let Some(rest) = l.trim_start().strip_prefix('#') else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix("unregistered:") else {
            continue;
        };
        let rest = rest.trim();
        let (name, reason) = match rest.find(char::is_whitespace) {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, ""),
        };
        let reason = reason
            .trim_start()
            .trim_start_matches('—')
            .trim_start_matches("--")
            .trim_start_matches('-')
            .trim();
        out.push((name.to_string(), reason.to_string()));
    }
    out
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion J's heredoc reader: the body of
// every column-0 `cat` whose `>`/`>>` target's basename is `gates.list` and which opens a `<<`
// heredoc, in file order, each body running to the line equal to its delimiter
pub fn smoke_registry_bodies(text: &str) -> Vec<String> {
    let lines = fresh::file_lines(text);
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let Some((delim, strip_tabs)) = registry_heredoc_opener(lines[i]) else {
            i += 1;
            continue;
        };
        let mut body: Vec<&str> = Vec::new();
        i += 1;
        while i < lines.len() {
            let l = if strip_tabs { lines[i].trim_start_matches('\t') } else { lines[i] };
            if l == delim {
                break;
            }
            body.push(lines[i]);
            i += 1;
        }
        out.push(body.join("\n"));
        i += 1;
    }
    out
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the opener's delimiter, quotes removed, and
// whether `<<-` strips the terminator's leading tabs; the redirect and the heredoc in either order
fn registry_heredoc_opener(line: &str) -> Option<(String, bool)> {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.first() != Some(&"cat") || line.starts_with(char::is_whitespace) {
        return None;
    }
    let (mut target, mut heredoc): (Option<&str>, Option<(String, bool)>) = (None, None);
    let mut k = 1;
    while k < words.len() {
        let w = words[k];
        if let Some(rest) = w.strip_prefix("<<") {
            let (rest, strip) = match rest.strip_prefix('-') {
                Some(r) => (r, true),
                None => (rest, false),
            };
            let raw = if rest.is_empty() {
                k += 1;
                words.get(k).copied().unwrap_or("")
            } else {
                rest
            };
            let d = raw.trim_matches(|c| c == '\'' || c == '"');
            if !d.is_empty() {
                heredoc = Some((d.to_string(), strip));
            }
        } else if let Some(rest) = w.strip_prefix(">>").or_else(|| w.strip_prefix('>')) {
            let t = if rest.is_empty() {
                k += 1;
                words.get(k).copied().unwrap_or("")
            } else {
                rest
            };
            target = Some(t.trim_matches(|c| c == '\'' || c == '"'));
        }
        k += 1;
    }
    let target = target?;
    (target.rsplit('/').next() == Some("gates.list")).then_some(())?;
    heredoc
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_resolve`: dirs consumer-first, `.sh` beating `.gate`
// *within* a dir, so a consumer shadowing a ported member with its own shell script still wins
pub fn resolve(name: &str, dirs: &[String]) -> Option<String> {
    for d in dirs {
        for ext in ["sh", "gate"] {
            let p = format!("{}/{}.{}", d, name, ext);
            if Path::new(&p).is_file() {
                return Some(p);
            }
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — what implements a resolved declaration, the
// one answer every site asking takes; an exec member carries its field's value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Substrate {
    Shell,
    Compiled,
    Exec(String),
}

pub const EXEC: &str = "# exec:";

// spec: gate-sdk/SPEC.md §The gate-authoring surface — a `.gate` descriptor's `# exec:` lines, held
// to one, naming a file, relative and with no `..` segment; a `.sh` is its own implementation
pub fn substrate(decl: &str) -> Result<Substrate, String> {
    if !decl.ends_with(".gate") {
        return Ok(Substrate::Shell);
    }
    let text = std::fs::read(decl)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", decl, e))?;
    exec_field(&text).map_err(|e| format!("{}: {} (gate-sdk/SPEC.md §The gate-authoring surface)", decl, e))
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — the descriptor-text half, so the placement
// assertion reads the same refusals the classifier raises
pub fn exec_field(text: &str) -> Result<Substrate, String> {
    let values: Vec<&str> = fresh::file_lines(text)
        .into_iter()
        .filter_map(|l| l.strip_prefix(EXEC))
        .map(str::trim)
        .collect();
    let value = match values.as_slice() {
        [] => return Ok(Substrate::Compiled),
        [v] => *v,
        _ => return Err(format!("carries {} `{}` lines, and a descriptor names one implementation", values.len(), EXEC)),
    };
    if value.is_empty() || value.ends_with(['/', '\\']) {
        return Err(format!("its `{}` value '{}' names no file", EXEC, value));
    }
    let rooted = crate::walk::path_root(value).is_some() || value.as_bytes().get(1) == Some(&b':');
    if rooted || value.split(['/', '\\']).any(|s| s == "..") {
        return Err(format!(
            "its `{}` value '{}' is rooted or climbs with `..` — it names a file relative to the descriptor's own directory",
            EXEC, value
        ));
    }
    Ok(Substrate::Exec(value.to_string()))
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — an exec value joined to its descriptor's
// directory, the path the host starts and every target reader reads
pub fn exec_target(decl: &str, value: &str) -> String {
    match decl.rsplit_once('/') {
        Some((dir, _)) => format!("{}/{}", dir, value),
        None => value.to_string(),
    }
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — one member's start: the program and its
// leading arguments, the member's own arguments following
pub struct Invocation {
    pub substrate: Substrate,
    pub program: crate::programs::Program,
    pub args: Vec<String>,
}

pub fn invocation(name: &str, decl: &str, self_exe: &str) -> Result<Invocation, String> {
    use crate::programs::{self, Program};
    use crate::substrate::TargetClass;
    let substrate = substrate(decl)?;
    let (program, args) = match &substrate {
        Substrate::Shell => shell_start(decl)?,
        Substrate::Compiled => {
            if crate::gates::declared(name).is_none() {
                return Err(format!(
                    "checkwright-gates: no such gate subcommand: {} — the check could not run; treating as failure (not clean)",
                    name
                ));
            }
            (programs::CHECKWRIGHT_GATES.at(self_exe), vec![name.to_string()])
        }
        Substrate::Exec(value) => {
            let target = exec_target(decl, value);
            let Some(class) = crate::substrate::target_class(&target) else {
                return Err(format!(
                    "{}: its `{}` target {} is of no admitted target class (a PowerShell script, .ps1, or an executable, whose name carries no `.`) — gate-sdk/SPEC.md §The gate-authoring surface",
                    decl, EXEC, target
                ));
            };
            if class == TargetClass::Executable {
                let path = crate::substrate::executable_path(&target);
                if !crate::proc::is_executable(Path::new(&path)) {
                    return Err(format!(
                        "{}: its `{}` target {} is not an executable file — your own build places it there, so build the gate before the battery runs; a registered gate that cannot start is not clean",
                        decl, EXEC, path
                    ));
                }
                return Ok(Invocation {
                    program: Program::consumer(programs::GATE_DECLARATION, path),
                    args: Vec::new(),
                    substrate,
                });
            }
            if !Path::new(&target).is_file() {
                return Err(format!(
                    "{}: its `{}` target {} is not a file — the check could not run; treating as failure (not clean)",
                    decl, EXEC, target
                ));
            }
            let host = crate::walk::knob_scalar(crate::substrate::KNOB)?;
            if host.is_empty() {
                return Err(format!(
                    "{} starts the PowerShell script {}, and {} is empty — set it to pwsh or powershell; a registered gate that cannot start is not clean",
                    name,
                    target,
                    crate::substrate::KNOB
                ));
            }
            let program = crate::substrate::resolve_host("GATE_SDK_POWERSHELL", &host)?;
            (program, crate::substrate::host_args(&target))
        }
    };
    Ok(Invocation { substrate, program, args })
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — a shell member's start, the one function the
// battery's dispatch and the fixture runner share: the declaration itself, or on native Windows bash
// with the declaration as its first argument
pub fn shell_start(decl: &str) -> Result<(crate::programs::Program, Vec<String>), String> {
    #[cfg(windows)]
    {
        windows_shell_start(decl, crate::proc::resolve_for_dispatch(crate::programs::BASH.invocation()))
    }
    #[cfg(not(windows))]
    {
        Ok((crate::programs::Program::consumer(crate::programs::GATE_DECLARATION, decl), Vec::new()))
    }
}

// spec: gate-sdk/SPEC.md §The gate-authoring surface — bash resolved outside the system directory
// before the spawn, or the dispatch refused
#[cfg_attr(not(windows), allow(dead_code))]
fn windows_shell_start(
    decl: &str,
    bash: Result<String, String>,
) -> Result<(crate::programs::Program, Vec<String>), String> {
    match bash {
        Ok(path) => Ok((crate::programs::BASH.at(path), vec![decl.to_string()])),
        Err(e) => Err(format!(
            "{} is a shell gate, which native Windows starts through Git for Windows' bash: {} (docs/requirements.md §Writing your own shell gates)",
            decl, e
        )),
    }
}

// spec: gate-sdk/SPEC.md §Layout and configuration — a registered name resolving nowhere is the
// runner's one red; a gate reading the registry skips it and names the skip on its own line
pub fn unresolved_skipped(n: usize) -> String {
    if n == 0 {
        return String::new();
    }
    format!(", {} unresolved member(s) skipped, reported by the runner", n)
}

// spec: gate-sdk/SPEC.md §run-gates — the resolve-dir order a battery reads a member's declaration
// through: the gates dir first, then each kit's `checks/`
pub fn resolve_dirs(gates_dir: &str, kit_roots: &[String]) -> Vec<String> {
    let mut dirs = vec![gates_dir.to_string()];
    for k in kit_roots {
        dirs.push(format!("{}/checks", k.trim_end_matches('/')));
    }
    dirs
}

// spec: gate-sdk/SPEC.md §port-blockers — the fixture dirs check-gate-fixture-coverage resolves,
// shared rather than duplicated, and taking its roots as a *parameter* because the gate resolves
// absolute ones where this report's evidence column must stay repo-relative.
pub fn fixture_dirs(tests_dir: &str, kit_roots: &[String]) -> Vec<String> {
    let mut dirs = vec![tests_dir.to_string()];
    for k in kit_roots {
        dirs.push(format!("{}/gate-tests", k.trim_end_matches('/')));
    }
    dirs
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the fixture-suite derivation: one `(suite, tests, checks)`
// triple per directory carrying a `gate-tests/` tree, the kit roots in order then the gates dir; the
// suite is the basename with `-` turned to `_`, and `checks` is empty where no sibling `checks/` is
// spec: gate-sdk/SPEC.md §Layout and configuration — both joins are statted and a row's dirs are
// the operand `--run-gate-tests` receives, so a caller hands this the working-directory spelling
pub fn fixture_suites_in(kit_roots: &[String], gates_dir: &str) -> Vec<(String, String, String)> {
    let mut out: Vec<(String, String, String)> = Vec::new();
    for base in kit_roots.iter().map(String::as_str).chain(std::iter::once(gates_dir)) {
        let base = base.trim_end_matches('/');
        let tests = format!("{}/gate-tests", base);
        if !Path::new(&tests).is_dir() {
            continue;
        }
        let suite = base.rsplit('/').next().unwrap_or(base).replace('-', "_");
        let checks = format!("{}/checks", base);
        let checks = if Path::new(&checks).is_dir() { checks } else { String::new() };
        out.push((suite, tests, checks));
    }
    out
}

pub fn fixture_suites() -> Result<Vec<(String, String, String)>, String> {
    Ok(fixture_suites_in(
        &crate::walk::kit_roots()?,
        &crate::walk::knob_scalar("GATE_SDK_GATES_DIR")?,
    ))
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `--emit fixture-suites`: one tab-separated line per triple
pub fn emit_fixture_suites(_args: &[String]) -> Result<String, String> {
    Ok(fixture_suites()?
        .into_iter()
        .map(|(s, t, c)| format!("{}\t{}\t{}\n", s, t, c))
        .collect())
}

pub fn list_path(gates_dir: &str) -> String {
    format!("{}/gates.list", gates_dir)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the first `# graph: ` line's whitespace-split
// `<key>=<value>` tokens; an unknown token is the caller's to report, so this keeps them all
pub fn manifest_line(text: &str) -> Option<&str> {
    fresh::file_lines(text)
        .into_iter()
        .find(|l| l.starts_with("# graph: "))
}

pub fn manifest_fields(man: &str) -> Vec<(String, String)> {
    man.trim_start_matches("# graph: ")
        .split_whitespace()
        .map(|kv| match kv.split_once('=') {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (kv.to_string(), String::new()),
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §The install disposition — the arming declaration's one reader: the value
// of every `# armed-by:` header line, in file order, so a count check and a lone read share it
pub const ARMED_BY: &str = "# armed-by:";

pub fn armed_by(text: &str) -> Vec<String> {
    fresh::file_lines(text)
        .iter()
        .filter_map(|l| l.strip_prefix(ARMED_BY))
        .map(|v| v.trim().to_string())
        .collect()
}

// spec: gate-sdk/SPEC.md §The install disposition — the leading token of the content form; no knob
// is named `content`, so the bare form stays unambiguous
pub const ARMED_BY_CONTENT: &str = "content";

#[derive(Debug, PartialEq, Eq)]
pub enum Arming<'a> {
    Knob(&'a str),
    Content(Vec<&'a str>),
}

// spec: gate-sdk/SPEC.md §The install disposition — one declaration value's form: `content` and
// its knob list, else the whole value as a bare knob
pub fn arming_form(value: &str) -> Arming<'_> {
    let mut words = value.split_whitespace();
    if words.next() == Some(ARMED_BY_CONTENT) {
        return Arming::Content(words.collect());
    }
    Arming::Knob(value)
}

// spec: gate-sdk/SPEC.md §The install disposition — the projection declaration's one reader: the
// value of every `# projection:` header line, in file order, so a count check and a lone read share it
pub const PROJECTION: &str = "# projection:";

pub fn projection(text: &str) -> Vec<String> {
    fresh::file_lines(text)
        .iter()
        .filter_map(|l| l.strip_prefix(PROJECTION))
        .map(|v| v.trim().to_string())
        .collect()
}

pub fn is_slug_byte(c: u8) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-'
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — a slug opens on a lowercase letter or a digit
pub fn slug_at(b: &[u8], at: usize) -> Option<usize> {
    if at >= b.len() || !(b[at].is_ascii_lowercase() || b[at].is_ascii_digit()) {
        return None;
    }
    let mut j = at + 1;
    while j < b.len() && is_slug_byte(b[j]) {
        j += 1;
    }
    Some(j)
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — one exemption element's disposition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    Until(String),
    Permanent,
    Missing,
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the one disposition reader both grammars
// share: the first `#`-opened `until: <slug>` annotation, else a `permanent:` one
pub fn disposition(line: &str) -> Disposition {
    let b = line.as_bytes();
    let opened = |kw: &str| -> Vec<usize> {
        let mut at = Vec::new();
        for i in 0..b.len() {
            if b[i] != b'#' {
                continue;
            }
            let mut j = i + 1;
            while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
                j += 1;
            }
            if line[j..].starts_with(kw) {
                at.push(j + kw.len());
            }
        }
        at
    };
    for k in opened("until:") {
        let mut k = k;
        while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
            k += 1;
        }
        if let Some(end) = slug_at(b, k) {
            return Disposition::Until(String::from_utf8_lossy(&b[k..end]).into_owned());
        }
    }
    if opened("permanent:").is_empty() {
        Disposition::Missing
    } else {
        Disposition::Permanent
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the descriptor's exemption field
pub const EXCEPTION: &str = "# exception:";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exemption {
    pub line: usize,
    pub element: String,
    pub disposition: Disposition,
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the field's one parser: each `# exception:`
// line's first token, and the disposition read off the rest of that line; an empty element is no
// element
pub fn exemptions(text: &str) -> Vec<Exemption> {
    let mut out = Vec::new();
    for (idx, l) in fresh::file_lines(text).into_iter().enumerate() {
        let Some(rest) = l.strip_prefix(EXCEPTION) else {
            continue;
        };
        let rest = rest.trim_start();
        let (element, tail) = match rest.find(char::is_whitespace) {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, ""),
        };
        if element.is_empty() || element.starts_with('#') {
            continue;
        }
        out.push(Exemption { line: idx + 1, element: element.to_string(), disposition: disposition(tail) });
    }
    out
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArrayElement {
    pub line: usize,
    pub raw: String,
    pub value: String,
    pub opening: bool,
}

#[derive(Debug)]
pub struct ExceptionArray {
    pub header: usize,
    pub elements: Vec<ArrayElement>,
}

fn assignment_opens(line: &str) -> bool {
    let t = line.trim_start_matches([' ', '\t']).as_bytes();
    if t.is_empty() || !(t[0].is_ascii_alphabetic() || t[0] == b'_') { return false; }
    let mut i = 1;
    while i < t.len() && (t[i].is_ascii_alphanumeric() || t[i] == b'_') { i += 1; }
    t[i..].starts_with(b"=(")
}

fn is_exception_tag(line: &str) -> bool {
    line.trim_start_matches([' ', '\t'])
        .strip_prefix('#')
        .is_some_and(|s| s.trim_start_matches([' ', '\t']).starts_with("exception-list:"))
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — one reader owns tagged shell arrays for both gates
pub fn exception_arrays(text: &str) -> Vec<ExceptionArray> {
    let lines: Vec<&str> = text.lines().collect();
    let mut arrays = Vec::new();
    let mut pending = None;
    let mut current: Option<ExceptionArray> = None;
    let mut opening_line = 0;
    let mut balance = 0i64;
    let mut quote: Option<char> = None;
    let mut continued = false;
    let mut word: Option<(usize, String)> = None;
    let mut yielded = 0;
    let finish = |word: &mut Option<(usize, String)>, current: &mut Option<ExceptionArray>, yielded: &mut usize, opening_line: usize, end: usize| {
        if let (Some((line, value)), Some(array)) = (word.take(), current.as_mut()) {
            if line != *yielded {
                *yielded = line;
                array.elements.push(ArrayElement { line, raw: lines[end - 1].to_string(), value, opening: line == opening_line });
            }
        }
    };
    for (idx, raw) in lines.iter().enumerate() {
        let line = idx + 1;
        let mid_word = quote.is_some() || continued;
        if !mid_word && is_exception_tag(raw) {
            pending = Some(line);
            continue;
        }
        let body = if current.is_none() {
            if !assignment_opens(raw) { continue; }
            let Some(header) = pending.take() else { continue; };
            current = Some(ExceptionArray { header, elements: Vec::new() });
            opening_line = line;
            balance = 0;
            &raw[raw.find('=').map_or(0, |i| i + 1)..]
        } else { raw };
        continued = false;
        let chars: Vec<char> = body.chars().collect();
        let mut i = 0;
        let mut closed = false;
        while i < chars.len() {
            let ch = chars[i];
            i += 1;
            match quote {
                Some('\'') => {
                    if ch == '\'' { quote = None; } else { word.as_mut().unwrap().1.push(ch); }
                    continue;
                }
                Some(_) => {
                    if ch == '"' {
                        quote = None;
                    } else if ch == '\\' && i == chars.len() {
                        continued = true;
                    } else if ch == '\\' && matches!(chars[i], '"' | '\\' | '$' | '`') {
                        word.as_mut().unwrap().1.push(chars[i]);
                        i += 1;
                    } else {
                        word.as_mut().unwrap().1.push(ch);
                    }
                    continue;
                }
                None => {}
            }
            match ch {
                ' ' | '\t' => finish(&mut word, &mut current, &mut yielded, opening_line, line),
                '#' if word.is_none() => break,
                '(' | ')' => {
                    finish(&mut word, &mut current, &mut yielded, opening_line, line);
                    balance += if ch == '(' { 1 } else { -1 };
                    if balance <= 0 { closed = true; break; }
                }
                '\\' if i == chars.len() => continued = true,
                _ => {
                    let w = word.get_or_insert_with(|| (line, String::new()));
                    match ch {
                        '\'' | '"' => quote = Some(ch),
                        '\\' => { w.1.push(chars[i]); i += 1; }
                        _ => w.1.push(ch),
                    }
                }
            }
        }
        if closed {
            arrays.push(current.take().unwrap());
            quote = None;
            continued = false;
            continue;
        }
        if quote.is_some() && !continued {
            word.as_mut().unwrap().1.push('\n');
        } else if quote.is_none() && !continued {
            finish(&mut word, &mut current, &mut yielded, opening_line, line);
        }
    }
    finish(&mut word, &mut current, &mut yielded, opening_line, lines.len().max(1));
    if let Some(array) = current { arrays.push(array); }
    arrays
}

// spec: gate-sdk/SPEC.md §The declaration cohort — the descriptor line declaring a knob no static
// kit's prefix owns, and the prefix its name carries
pub const KNOB_LINE: &str = "# knob:";
pub const LOCAL_PREFIX: &str = "GATE_LOCAL_";

// spec: gate-sdk/SPEC.md §The declaration cohort — one declared knob: its declared value, a scalar or
// an indexed knob's elements in line order, and the descriptor declaring it
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredKnob {
    pub name: String,
    pub value: crate::knobs::Value,
    pub file: String,
}

// spec: gate-sdk/SPEC.md §The declaration cohort — every `# knob:` line of one gates directory's
// descriptors, each read by the knob-file line reader and held to one producer per name
// spec: gate-sdk/SPEC.md §check-reads-couples — a single-level listing, so a knob read walks no root
pub fn descriptor_knobs(gates_dir: &str) -> Result<Vec<DeclaredKnob>, String> {
    use crate::knobfile::Form;
    use crate::knobs::Value;
    let dir = Path::new(gates_dir);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = crate::walk::list_dir(dir)?
        .into_iter()
        .filter(|(n, is_dir)| !is_dir && n.ends_with(".gate"))
        .map(|(n, _)| n)
        .collect();
    names.sort();
    let mut out: Vec<DeclaredKnob> = Vec::new();
    for n in names {
        let path = format!("{}/{}", gates_dir.trim_end_matches('/'), n);
        let text = std::fs::read(&path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read {}: {}", path, e))?;
        for (idx, line) in text.lines().enumerate() {
            let Some(rest) = line.strip_prefix(KNOB_LINE) else { continue };
            let at = format!("{}:{}", path, idx + 1);
            let entries = crate::knobfile::parse(rest, &path)
                .map_err(|e| e.replacen(&format!("{}:1:", path), &format!("{}:", at), 1))?;
            let Some(e) = entries.into_iter().next() else {
                return Err(format!("{}: a `{}` line carries no knob-file line", at, KNOB_LINE));
            };
            if let Some(kit) = crate::knobs::owner(&e.name) {
                return Err(format!(
                    "{}: {} is {}'s knob, and a descriptor declares only a name no static kit's prefix owns",
                    at, e.name, kit.root
                ));
            }
            if !e.name.starts_with(LOCAL_PREFIX) {
                return Err(format!("{}: {} carries no {} prefix", at, e.name, LOCAL_PREFIX));
            }
            let element = match e.form {
                Form::Scalar => None,
                Form::Indexed => Some(e.value.clone()),
                Form::Keyed(_) | Form::Reference(_) => {
                    return Err(format!(
                        "{}: {} — a descriptor declares `NAME = value` or one `NAME[] = element` line per element",
                        at, e.name
                    ))
                }
            };
            match (out.iter_mut().find(|d| d.name == e.name), element) {
                (None, None) => out.push(DeclaredKnob { name: e.name, value: Value::Scalar(e.value), file: path.clone() }),
                (None, Some(el)) => out.push(DeclaredKnob { name: e.name, value: Value::Indexed(vec![el]), file: path.clone() }),
                (Some(d), Some(el)) if d.file == path && matches!(d.value, Value::Indexed(_)) => {
                    if let Value::Indexed(v) = &mut d.value {
                        v.push(el);
                    }
                }
                (Some(d), _) => {
                    return Err(format!(
                        "{}: {} is declared again — {} declares it, and a name has one declaring line set",
                        at, e.name, d.file
                    ))
                }
            }
        }
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one field's value, empty when the field is
// absent; never an error on a missing field
pub fn field(fields: &[(String, String)], key: &str) -> String {
    fields
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .unwrap_or_default()
}

// spec: gate-sdk/SPEC.md §run-gates — a member's effective trigger field: `trigger=`, else `couples=`
pub fn effective_trigger(fields: &[(String, String)]) -> String {
    let trigger = field(fields, "trigger");
    if trigger.is_empty() {
        field(fields, "couples")
    } else {
        trigger
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the closed set of special `couples=` prefixes,
// read from here by every reader that recognises one, because a prefix one reader knows and another
// does not falls through as an inert literal glob
pub const COUPLES_PREFIXES: &[&str] = &["knob:", "kit:"];

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the character set a literal glob, and a prefixed
// token's remainder, is held to
pub fn literal_glob(t: &str) -> bool {
    !t.is_empty() && t.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '*' | '?' | '/' | '-'))
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the knob a `knob:` token's remainder names,
// rooted or projected
pub fn knob_token_name(r: &str) -> &str {
    match crate::knobs::rooted(r) {
        Some((name, _)) => name,
        None => crate::knobs::reference(r).0,
    }
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the couples-knob sentinel: the name a member declares in
// place of knobs written on the descriptor corpus rather than in its own entry
pub const EVERY_COUPLES_KNOB: &str = "@every-couples-knob";

// spec: gate-sdk/SPEC.md §lib/gate.sh — the sentinel's expansion for a caller already holding the
// resolve dirs: every `knob:` name the descriptor corpus carries, sorted once each
pub fn couples_knob_names(resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let mut out = corpus_knob_tokens(resolve_dirs)?;
    out.sort();
    out.dedup();
    Ok(out)
}

fn corpus_knob_tokens(resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    for d in resolve_dirs {
        let decls = crate::walk::glob_files(
            Path::new(d),
            &["*.gate".to_string(), "*.sh".to_string()],
        )?;
        for p in decls {
            let text = match std::fs::read(&p) {
                Ok(b) => String::from_utf8_lossy(&b).into_owned(),
                Err(_) => continue,
            };
            // spec: gate-sdk/SPEC.md §The install disposition — a `# projection:` knob-path token is a
            // `knob:` token the descriptor corpus carries
            tokens.extend(
                projection(&text)
                    .iter()
                    .flat_map(|l| l.split(','))
                    .filter_map(|t| t.trim().strip_prefix("knob:"))
                    .map(|r| knob_token_name(r).to_string()),
            );
            let Some(man) = manifest_line(&text) else {
                continue;
            };
            for (k, v) in manifest_fields(man) {
                if k != "couples" && k != "trigger" {
                    continue;
                }
                tokens.extend(
                    v.split(',')
                        .filter_map(|t| t.strip_prefix("knob:"))
                        .map(|r| knob_token_name(r).to_string()),
                );
            }
        }
    }
    Ok(tokens)
}

fn reach_kits(name: &str, kits: &mut Vec<&'static str>, seen: &mut Vec<&'static str>) {
    let bare = name.trim_end_matches('*');
    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — an environment-only name is no knob file's
    let Some(kit) = crate::knobs::owner(bare).filter(|k| !k.is_env_only(bare)) else {
        return;
    };
    if !kits.contains(&kit.root) {
        kits.push(kit.root);
    }
    let family = name.ends_with('*');
    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a declared family reaches the kits its
    // derivation reads, as a row reaches its inputs
    if family {
        for f in kit.families.iter().filter(|f| f.prefix == bare) {
            for input in f.inputs {
                reach_kits(input, kits, seen);
            }
        }
    }
    for row in kit.rows {
        let hit = if family { row.name.starts_with(bare) } else { row.name == bare };
        if !hit || seen.contains(&row.name) {
            continue;
        }
        seen.push(row.name);
        for input in row.inputs {
            reach_kits(input, kits, seen);
        }
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a static kit's knob file is a derived couple:
// the tracked file at its default location for every static kit the member's declaration reaches,
// in the static-kit table's order; a staged member reaching one is refused, never handed the file
pub fn knob_files(member: &str, resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let Some(declared) = crate::gates::declared(member) else {
        return Ok(Vec::new());
    };
    let mut kits: Vec<&'static str> = Vec::new();
    let mut seen: Vec<&'static str> = Vec::new();
    for name in declared {
        if *name == EVERY_COUPLES_KNOB {
            for t in couples_knob_names(resolve_dirs)? {
                reach_kits(&t, &mut kits, &mut seen);
            }
        } else {
            reach_kits(name, &mut kits, &mut seen);
        }
    }
    if kits.is_empty() {
        return Ok(Vec::new());
    }
    let dir = crate::knobs::gates_dir();
    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a reference line in a reached kit's tracked
    // file reaches its referent's kit, read from the file now; the local overlay is never read
    for root in kits.clone() {
        let Some(kit) = crate::knobs::STATIC_KITS.iter().find(|k| k.root == root) else {
            continue;
        };
        let path = format!("{}/{}-config.knobs", dir, kit.stem());
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for e in crate::knobfile::parse(&text, &path)? {
            if let crate::knobfile::Form::Reference(other) = &e.form {
                reach_kits(other, &mut kits, &mut seen);
            }
        }
    }
    if let Some(src) = resolve(member, resolve_dirs) {
        let text = std::fs::read(&src)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let mode = manifest_line(&text).map(|m| field(&manifest_fields(m), "mode")).unwrap_or_default();
        if mode == "staged" {
            return Err(format!(
                "{} is mode=staged and reads the knob file of {} — the staged hook passes matched \
                 paths to the gate as arguments, so a derived knob-file couple would reach it as a \
                 file to scan; treating as failure (not clean).\n  help: read no static knob from a \
                 mode=staged member, or drop mode=staged",
                member,
                kits.join(", ")
            ));
        }
    }
    Ok(crate::knobs::STATIC_KITS
        .iter()
        .filter(|k| kits.contains(&k.root))
        .map(|k| format!("{}/{}-config.knobs", dir, k.stem()))
        .collect())
}

include!(concat!(env!("OUT_DIR"), "/gate_modules.rs"));

// spec: gate-sdk/SPEC.md §The non-gate arm — the layers every member reaches, held by
// §check-crate-arms and the source stamp rather than by any one descriptor
pub const UNIVERSAL_MODULES: &[&str] =
    &["walk.rs", "proc.rs", "registry.rs", "gates/mod.rs", "emit/mod.rs", "knobs/mod.rs"];

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a member's derived couples: its knob files, then
// its crate modules
pub fn derived_couples(member: &str, resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let mut out = knob_files(member, resolve_dirs)?;
    out.extend(module_files(member, resolve_dirs)?);
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the member's cut: its own module, its direct
// first-party imports, and the direct imports of each `emit/` module among them, less the universal
// layers; empty where the crate source is absent or the member is `mode=staged`
pub fn module_files(member: &str, resolve_dirs: &[String]) -> Result<Vec<String>, String> {
    let Some((_, module)) = GATE_MODULES.iter().find(|(n, _)| *n == member) else {
        return Ok(Vec::new());
    };
    let src = crate::walk::knob_scalar("GATE_SDK_NATIVE_SRC")?;
    let src = src.trim_end_matches('/');
    if src.is_empty() || !Path::new(src).is_dir() {
        return Ok(Vec::new());
    }
    if let Some(desc) = resolve(member, resolve_dirs) {
        let text = std::fs::read(&desc).map(|b| String::from_utf8_lossy(&b).into_owned()).unwrap_or_default();
        if manifest_line(&text).map(|m| field(&manifest_fields(m), "mode") == "staged").unwrap_or(false) {
            return Ok(Vec::new());
        }
    }
    Ok(cut(src, module).into_iter().map(|m| format!("{}/{}", src, m)).collect())
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the cut over a crate source, as module paths relative to it
fn cut(src: &str, module: &str) -> Vec<String> {
    let Some(own) = module_file(src, &format!("gates/{}", module)) else {
        return Vec::new();
    };
    let mut cut: Vec<String> = vec![own.clone()];
    for m in module_imports(src, &own) {
        if !cut.contains(&m) {
            cut.push(m.clone());
        }
        if m.starts_with("emit/") && !UNIVERSAL_MODULES.contains(&m.as_str()) {
            for e in module_imports(src, &m) {
                if !cut.contains(&e) {
                    cut.push(e);
                }
            }
        }
    }
    cut.retain(|m| !UNIVERSAL_MODULES.contains(&m.as_str()));
    cut
}

// spec: gate-sdk/SPEC.md §The non-gate arm — a module path's file, relative to the crate source
fn module_file(src: &str, module: &str) -> Option<String> {
    [format!("{}.rs", module), format!("{}/mod.rs", module)]
        .into_iter()
        .find(|f| Path::new(&format!("{}/{}", src, f)).is_file())
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the deepest module a path names from `base`, with the
// module path it reached and whether every segment was a module
fn deepest(src: &str, base: &str, segs: &[&str]) -> Option<(String, String, bool)> {
    let mut best: Option<(String, String)> = None;
    let mut at = base.to_string();
    let mut consumed = 0;
    for seg in segs {
        let next = if at.is_empty() { seg.to_string() } else { format!("{}/{}", at, seg) };
        match module_file(src, &next) {
            Some(f) => {
                best = Some((f, next.clone()));
                at = next;
                consumed += 1;
            }
            None => break,
        }
    }
    match best {
        Some((f, m)) => Some((f, m, consumed == segs.len())),
        None if base.is_empty() => None,
        None => module_file(src, base).map(|f| (f, base.to_string(), segs.is_empty())),
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — a module file's direct first-party imports: every
// `crate::` and `super::` path outside comments and the test module, braced `use` groups expanded,
// and every path through a name a path imported as a module
fn module_imports(src: &str, file: &str) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(format!("{}/{}", src, file)) else {
        return Vec::new();
    };
    let own = file.strip_suffix("/mod.rs").or_else(|| file.strip_suffix(".rs")).unwrap_or(file);
    let parent = own.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
    let mut code = String::new();
    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("#[cfg(test)]") {
            break;
        }
        if !t.starts_with("//") {
            code.push_str(line);
            code.push('\n');
        }
    }
    let mut out: Vec<String> = Vec::new();
    let mut aliases: Vec<(String, String)> = Vec::new();
    let push = |f: String, out: &mut Vec<String>| {
        if f != file && !out.contains(&f) {
            out.push(f);
        }
    };
    for (path, alias) in use_paths(&code, &["crate", "super"]) {
        let segs: Vec<&str> = path.split("::").filter(|s| !s.is_empty() && *s != "self").collect();
        let found = match segs.split_first() {
            Some((&"crate", rest)) => deepest(src, "", rest),
            Some((&"super", rest)) => deepest(src, parent, rest),
            _ => None,
        };
        if let Some((f, m, whole)) = found {
            if whole {
                let name = alias.unwrap_or_else(|| m.rsplit('/').next().unwrap_or(&m).to_string());
                if !aliases.iter().any(|(n, _)| *n == name) {
                    aliases.push((name, m));
                }
            }
            push(f, &mut out);
        }
    }
    let names: Vec<&str> = aliases.iter().map(|(n, _)| n.as_str()).collect();
    for (path, _) in use_paths(&code, &names) {
        let segs: Vec<&str> = path.split("::").filter(|s| !s.is_empty() && *s != "self").collect();
        let Some((head, rest)) = segs.split_first() else {
            continue;
        };
        let Some((_, m)) = aliases.iter().find(|(n, _)| n == head) else {
            continue;
        };
        if let Some((f, _, _)) = deepest(src, m, rest) {
            push(f, &mut out);
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §The non-gate arm — each path in the code whose first segment is one of
// `heads`, a braced group spelling one path per leaf, with the name an `as` binds it to
fn use_paths(code: &str, heads: &[&str]) -> Vec<(String, Option<String>)> {
    let b = code.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let rest = &code[i..];
        let boundary = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_' || b[i - 1] == b':');
        let head = heads.iter().find(|h| rest.starts_with(&format!("{}::", h)));
        match head.filter(|_| boundary) {
            Some(_) => {
                let len = path_len(rest);
                let alias = rest[len..]
                    .strip_prefix(" as ")
                    .map(|a| a.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect::<String>())
                    .filter(|a| !a.is_empty());
                let start = out.len();
                expand_group(&rest[..len], &mut out);
                if out.len() == start + 1 && out[start].1.is_none() {
                    out[start].1 = alias;
                }
                i += len.max(1);
            }
            None => i += rest.chars().next().map(char::len_utf8).unwrap_or(1),
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §The non-gate arm — a path's extent: identifiers, `::` and balanced braces
fn path_len(s: &str) -> usize {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' if depth == 0 => return i,
            '}' => depth -= 1,
            c if c.is_alphanumeric() || c == '_' || c == ':' || c == '*' => {}
            c if depth > 0 && (c == ',' || c.is_whitespace()) => {}
            _ => return i,
        }
    }
    s.len()
}

fn expand_group(p: &str, out: &mut Vec<(String, Option<String>)>) {
    let Some(open) = p.find('{') else {
        let mut words = p.split_whitespace();
        let leaf = words.next().unwrap_or("").trim_end_matches("::").to_string();
        let alias = match (words.next(), words.next()) {
            (Some("as"), Some(a)) => Some(a.to_string()),
            _ => None,
        };
        out.push((leaf, alias));
        return;
    };
    let prefix = &p[..open];
    let inner = &p[open + 1..p.rfind('}').unwrap_or(p.len())];
    let mut depth = 0usize;
    let mut start = 0;
    let mut items: Vec<&str> = Vec::new();
    for (i, c) in inner.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                items.push(&inner[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    items.push(&inner[start..]);
    for item in items.into_iter().map(str::trim).filter(|s| !s.is_empty()) {
        expand_group(&format!("{}{}", prefix, item), out);
    }
}

// spec: gate-sdk/SPEC.md §Reading a `couples=` field's reach — the field's one matcher, which every
// reader asking what an expanded token reaches calls by this name
pub fn couple_matches(path: &str, token: &str) -> bool {
    crate::walk::pattern_match(token, path)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a `knob:` member's covering string pattern; a
// `kit:<glob>` member passes unconverted to the `kit:` pass
fn covering_pattern(member: &str) -> String {
    if member.starts_with("kit:") {
        return member.to_string();
    }
    let collapsed = member.replace("**/", "*");
    if collapsed.starts_with('*') {
        collapsed
    } else {
        format!("*{}", collapsed)
    }
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a `knob:<NAME>/<glob>` token's covering patterns:
// the glob under each directory the knob resolves to
fn rooted_patterns(token: &str) -> Result<Vec<String>, String> {
    rooted_paths(token, "couples").map(|ps| ps.iter().map(|p| covering_pattern(p)).collect())
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a rooted token's paths before any conversion:
// a scalar root is one path and an empty one a refusal, since it would re-root the glob at the
// repository root; a word list roots the glob at each word, and an empty list names no root at all
fn rooted_paths(token: &str, kind: &str) -> Result<Vec<String>, String> {
    let Some((name, glob)) = crate::knobs::rooted(token) else {
        return Err(format!("{} token 'knob:{}' is not rooted", kind, token));
    };
    let refuse = |why: String| {
        format!(
            "{} token 'knob:{}' cannot be expanded: {} — a rooted token is one glob under each \
             directory its knob resolves to; treating as failure (not clean)",
            kind, token, why
        )
    };
    if let Some(why) = crate::knobs::root_refusal(name) {
        return Err(refuse(why));
    }
    if !literal_glob(glob) {
        return Err(refuse(format!("'{}' is not a literal glob", glob)));
    }
    let roots = if crate::knobs::is_words(name) {
        crate::walk::knob_words(name).map_err(refuse)?
    } else {
        vec![crate::walk::knob_scalar(name).map_err(refuse)?]
    };
    roots.iter().map(|r| rooted_join(name, r, glob).map_err(refuse)).collect()
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one root's terms: a trailing `/` trimmed, a
// leading `./` dropped, `.` read as no prefix, and an empty root or one carrying a comma refused
fn rooted_join(name: &str, value: &str, glob: &str) -> Result<String, String> {
    let root = value.trim_end_matches('/');
    if root.is_empty() {
        return Err(format!("{} resolves to no directory", name));
    }
    if root.contains(',') || root.chars().any(char::is_whitespace) {
        return Err(format!("{}'s value '{}' carries a comma or whitespace", name, root));
    }
    let root = root.strip_prefix("./").unwrap_or(root);
    if root == "." {
        return Ok(glob.to_string());
    }
    Ok(format!("{}/{}", root, glob))
}

// spec: gate-sdk/SPEC.md §The install disposition — the knob-path token's one resolver: a literal is
// itself, `knob:<NAME>` the knob's members and `knob:<NAME>/<glob>` its rooted path, each verbatim,
// since a declaration naming a place is resolved to the place and never to a covering pattern
pub fn knob_paths(token: &str) -> Result<Vec<String>, String> {
    if token.starts_with("kit:") {
        return Err(format!(
            "knob-path token '{}' carries 'kit:' — a place a kit gate writes or prunes is the \
             consumer's tree, never a kit root; treating as failure (not clean)",
            token
        ));
    }
    let Some(name) = token.strip_prefix("knob:") else {
        return Ok(vec![token.to_string()]);
    };
    if crate::knobs::rooted(name).is_some() {
        return rooted_paths(name, "knob-path");
    }
    let members = match crate::knobs::reference(name) {
        (knob, Some(field)) => crate::knobs::project(knob, field),
        (knob, None) => crate::knobs::refuse_whole_packed(knob).and_then(|_| crate::walk::knob_array(knob)),
    };
    members.map_err(|e| {
        format!(
            "knob-path token 'knob:{}' could not be resolved: {} — a knob-path token resolves to \
             the knob's value, and an unresolvable knob would be a lost declaration; treating as \
             failure (not clean)",
            name, e
        )
    })
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the couples/trigger expansion every reader
// shares: two passes in a fixed order, `knob:` then `kit:` over the result, one pass each
pub fn expand_couples(field: &str, kit_roots_rel: &[String]) -> Result<String, String> {
    let mut once: Vec<String> = Vec::new();
    for tok in field.split(',') {
        match tok.strip_prefix("knob:") {
            Some(name) if crate::knobs::rooted(name).is_some() => once.extend(rooted_patterns(name)?),
            Some(name) => {
                // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a `<NAME>.<field>` token expands to
                // the field's members across the elements, and a bare token on a packed knob is refused
                let members = match crate::knobs::reference(name) {
                    (knob, Some(field)) => crate::knobs::project(knob, field),
                    (knob, None) => {
                        crate::knobs::refuse_whole_packed(knob).and_then(|_| crate::walk::knob_array(knob))
                    }
                };
                for m in members.map_err(|e| {
                    format!(
                        "couples token 'knob:{}' could not be resolved: {} — a knob token expands \
                         to the knob's members, and an unresolvable knob would be a lost trigger; \
                         treating as failure (not clean).\n  help: name a knob a static kit declares, \
                         and declare the '{}' sentinel on the expanding member",
                        name, e, EVERY_COUPLES_KNOB
                    )
                })? {
                    if m.starts_with("knob:") {
                        return Err(format!(
                            "couples token 'knob:{}' has the member '{}', itself a knob token — \
                             expansion is one pass each, so a nested token is a refusal rather \
                             than a second pass",
                            name, m
                        ));
                    }
                    if m.contains(',') || m.chars().any(char::is_whitespace) {
                        return Err(format!(
                            "couples token 'knob:{}' has the member '{}', which carries a comma or \
                             whitespace — `couples=` is comma-separated and its manifest line \
                             splits on unquoted whitespace, so such a member is unrepresentable \
                             after expansion and would truncate the trigger silently",
                            name, m
                        ));
                    }
                    once.push(covering_pattern(&m));
                }
            }
            None => once.push(tok.to_string()),
        }
    }
    let mut out: Vec<String> = Vec::new();
    for tok in &once {
        match tok.strip_prefix("kit:") {
            Some(glob) => {
                for r in kit_roots_rel {
                    out.push(format!("{}/{}", r.trim_end_matches('/'), glob));
                }
            }
            None => out.push(tok.clone()),
        }
    }
    Ok(out.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The gate-authoring surface — a value ending in a separator names no
    // file, beside the empty, rooted and climbing refusals
    #[test]
    fn an_exec_value_must_name_a_file() {
        for bad in ["# exec: bin/\n", "# exec: bin\\\n", "# exec:\n", "# exec: /abs\n", "# exec: ../up\n"] {
            assert!(exec_field(bad).is_err(), "{:?}", bad);
        }
        assert!(exec_field("# exec: bin/\n").unwrap_err().contains("names no file"));
        assert_eq!(exec_field("# exec: bin/check-x\n"), Ok(Substrate::Exec("bin/check-x".to_string())));
    }

    // spec: gate-sdk/SPEC.md §The gate-authoring surface — the Windows shell start: bash resolved
    // first and handed the declaration, or a refusal naming Git for Windows' bash and the page
    #[test]
    fn the_windows_shell_start_runs_bash_or_refuses() {
        let (program, args) = windows_shell_start("scripts/check-x.sh", Ok("C:/Git/bin/bash.exe".to_string())).expect("start");
        assert_eq!(program.invocation(), "C:/Git/bin/bash.exe");
        assert_eq!(args, vec!["scripts/check-x.sh".to_string()]);
        let e = windows_shell_start("scripts/check-x.sh", Err("cannot resolve bash".to_string())).err().unwrap_or_default();
        assert!(e.contains("Git for Windows' bash") && e.contains("§Writing your own shell gates"), "{}", e);
    }

    #[test]
    fn the_exemption_field_reads_each_element_with_its_disposition() {
        let text = "# graph: dir=one\n\
# exception: docs/*.md # until: fix-docs\n\
# exception: legacy-token # permanent: structural\n\
# exception: src/bare.rs\n\
# exception:\n\
# exception: # until: no-element\n\
# exec: check-x.ps1\n";
        let got: Vec<(usize, String, Disposition)> =
            exemptions(text).into_iter().map(|e| (e.line, e.element, e.disposition)).collect();
        assert_eq!(
            got,
            vec![
                (2, "docs/*.md".to_string(), Disposition::Until("fix-docs".to_string())),
                (3, "legacy-token".to_string(), Disposition::Permanent),
                (4, "src/bare.rs".to_string(), Disposition::Missing),
            ]
        );
        assert!(exemptions("# graph: dir=one\n# exception-list: arrays are not this field\n").is_empty());
    }

    #[test]
    fn a_smoke_registry_body_is_the_gates_list_heredoc_and_nothing_else() {
        let text = "#!/usr/bin/env bash\nmkdir -p scripts\ncat >> scripts/gates.list <<'EOF'\n# kit\ncheck-a\n\
                    # unregistered: check-b — why\nEOF\ncat > notes.txt <<'EOF'\ncheck-z\nEOF\n\
                    printf 'check-y\\n' > \"$q/gates.list\"\n  cat > scripts/gates.list <<EOF\ncheck-x\nEOF\n";
        let got = smoke_registry_bodies(text);
        assert_eq!(got, vec!["# kit\ncheck-a\n# unregistered: check-b — why".to_string()]);
        let other_order = "cat <<\"END\" > gates.list\ncheck-a\nEND\ncat >>scripts/gates.list <<-EOF\ncheck-b\n\tEOF\n";
        assert_eq!(smoke_registry_bodies(other_order), vec!["check-a".to_string(), "check-b".to_string()]);
        assert!(smoke_registry_bodies("cat scripts/gates.list\n").is_empty());
    }

    #[test]
    fn an_arming_value_is_a_bare_knob_or_content_and_its_knobs() {
        assert_eq!(arming_form("GATE_SDK_PORTABILITY_PATHS"), Arming::Knob("GATE_SDK_PORTABILITY_PATHS"));
        assert_eq!(arming_form("content A_KNOB B_KNOB"), Arming::Content(vec!["A_KNOB", "B_KNOB"]));
        assert_eq!(arming_form("content"), Arming::Content(vec![]));
        assert_eq!(arming_form("contents"), Arming::Knob("contents"));
        assert_eq!(arming_form(""), Arming::Knob(""));
    }

    #[test]
    fn a_member_line_is_neither_blank_nor_a_comment() {
        let got = members("check-a\n\n  # a comment\ncheck-b\n");
        assert_eq!(got, vec!["check-a".to_string(), "check-b".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §check-graph — a `kit:` token expands once per repo-relative kit
    // root and a plain token passes through, which is the expansion both substrates must agree on
    #[test]
    fn kit_tokens_expand_and_plain_tokens_pass_through() {
        let roots = vec!["gate-sdk".to_string(), "queue-kit/".to_string()];
        assert_eq!(
            expand_couples("scripts/gates.list,kit:*.sh", &roots).expect("no knob token"),
            "scripts/gates.list,gate-sdk/*.sh,queue-kit/*.sh"
        );
    }

    // spec: gate-sdk/SPEC.md §Reading a `couples=` field's reach — the one semantics: `*` and `?`
    // cross `/`, and `**` is no more than `*`
    #[test]
    fn the_couples_matcher_is_a_slash_spanning_string_match() {
        assert!(couple_matches("native/src/emit/mod.rs", "native/src/*.rs"));
        assert!(!couple_matches("a/b.rs", "a/**/b.rs"));
        assert!(couple_matches("a/b", "a?b"));
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — each knob member expands to its covering
    // pattern, and a member already leading with `*` expands unchanged
    #[test]
    fn a_knob_member_expands_to_the_pattern_that_covers_it() {
        let knobs = crate::knobenv::lock();
        let d = scratch_corpus(
            &knobs,
            "cover",
            "CANON_KIT_MANIFEST_FILES[] = **/*.sh\nCANON_KIT_MANIFEST_FILES[] = SPEC.md\n\
             CANON_KIT_MANIFEST_FILES[] = templates/*.md\nCANON_KIT_MANIFEST_FILES[] = */README.md\n",
        );
        let roots = vec!["gate-sdk".to_string()];
        let got = expand_couples("knob:CANON_KIT_MANIFEST_FILES", &roots).expect("resolvable");
        assert_eq!(got, "**.sh,*SPEC.md,*templates/*.md,*/README.md");
        for (path, tok) in [
            ("x.sh", "**.sh"),
            ("a/b/x.sh", "**.sh"),
            ("docs/gate-sdk/SPEC.md", "*SPEC.md"),
            ("lifecycle-kit/templates/scope.md", "*templates/*.md"),
        ] {
            assert!(couple_matches(path, tok), "{} must cover {}", tok, path);
        }
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a scratch gates dir holding one canon-kit
    // knob file, so a `knob:` token names a static knob with a value this test controls
    fn scratch_corpus(knobs: &crate::knobenv::KnobEnv, tag: &str, body: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("checkwright-couples-{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        std::fs::write(d.join("canon-config.knobs"), body).expect("write");
        knobs.set("GATE_SDK_GATES_DIR", &d.display().to_string());
        knobs.remove("CANON_KIT_KNOB_FILE");
        knobs.remove("CANON_KIT_LOCAL_KNOB_FILE");
        crate::knobs::reset(knobs);
        d
    }

    fn unscratch(knobs: &crate::knobenv::KnobEnv, d: &Path) {
        knobs.remove("GATE_SDK_GATES_DIR");
        crate::knobs::reset(knobs);
        let _ = std::fs::remove_dir_all(d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — the resolution order: `knob:` first, then
    // `kit:` over the result, so a knob member spelled `kit:<glob>` composes; and the union is a
    // union, so a descriptor's own literals survive beside the knob's members.
    #[test]
    fn a_knob_token_expands_first_and_a_kit_member_then_composes() {
        let knobs = crate::knobenv::lock();
        let d = scratch_corpus(
            &knobs,
            "compose",
            "CANON_KIT_MANIFEST_FILES[] = CLAUDE.md\nCANON_KIT_MANIFEST_FILES[] = kit:SPEC.md\n",
        );
        let roots = vec!["gate-sdk".to_string()];
        assert_eq!(
            expand_couples("*SPEC*.md,knob:CANON_KIT_MANIFEST_FILES", &roots).expect("resolvable"),
            "*SPEC*.md,*CLAUDE.md,gate-sdk/SPEC.md"
        );
        std::fs::write(d.join("canon-config.knobs"), "CANON_KIT_MANIFEST_FILES =\n").expect("write");
        crate::knobs::reset(&knobs);
        assert_eq!(
            expand_couples("CLAUDE.md,knob:CANON_KIT_MANIFEST_FILES", &roots).expect("resolved-empty"),
            "CLAUDE.md",
            "a consumer's empty knob expands to nothing, because the gate then scans nothing either"
        );
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a packed knob is addressed by a declared field,
    // read through its one parser with a short element's missing fields empty, and never whole
    #[test]
    fn a_packed_knob_expands_by_its_declared_field_and_is_refused_whole() {
        let knobs = crate::knobenv::lock();
        let d = scratch_corpus(
            &knobs,
            "packed",
            "CANON_KIT_EMBED_LANGS[] = rs|rust,rs|*.rs\n\
             CANON_KIT_EMBED_LANGS[] = make|make|Makefile,**/build/*.mk\n\
             CANON_KIT_EMBED_LANGS[] = bare\n",
        );
        let roots = vec!["gate-sdk".to_string()];
        assert_eq!(
            expand_couples("*SPEC*.md,knob:CANON_KIT_EMBED_LANGS.file-globs", &roots).expect("a declared field"),
            "*SPEC*.md,*.rs,*Makefile,*build/*.mk"
        );
        for refused in [
            "knob:CANON_KIT_EMBED_LANGS",
            "knob:CANON_KIT_EMBED_LANGS.no-such-field",
            "knob:CANON_KIT_SPEC_NAME.file-globs",
        ] {
            assert!(expand_couples(refused, &roots).is_err(), "'{}' expanded rather than refusing", refused);
        }
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the sentinel is spelled in the crate alone, and its
    // expansion is every `knob:` name the descriptor corpus carries, each a static kit's
    #[test]
    fn the_couples_knob_sentinel_expands_to_the_static_names_the_corpus_carries() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let dirs = repo_check_dirs(&repo);
        let names = couples_knob_names(&dirs).expect("the descriptor corpus is readable");
        assert!(
            !names.is_empty(),
            "the descriptor corpus names no knob token, so the expansion held over nothing — read it \
             as unverified rather than as clean"
        );
        for n in &names {
            assert!(crate::knobs::owner(n).is_some(), "the knob token {} names no static kit's knob", n);
        }
        let mut sorted = names.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(names, sorted, "the expansion is not deterministic");
    }

    // spec: gate-sdk/SPEC.md §run-gates — this repo's resolve dirs, derived in the crate: the gates
    // dir, then each kit root's `checks/`, absolute
    fn repo_check_dirs(repo: &Path) -> Vec<String> {
        let sdk = crate::walk::normalize_abs(&repo.join("gate-sdk").display().to_string());
        let roots: Vec<String> = crate::walk::kit_roots_rel_from(&sdk, "")
            .expect("the kit roots derive")
            .into_iter()
            .map(|r| repo.join(r).display().to_string())
            .collect();
        resolve_dirs(&repo.join(crate::knobs::GATES_DIR_DEFAULT).display().to_string(), &roots)
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the universal layers and every member's own module
    // name existing files, and the attested member's cut is the set its fixing session found
    #[test]
    fn the_module_cut_reads_the_crate_source() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").display().to_string();
        for m in UNIVERSAL_MODULES {
            assert!(Path::new(&format!("{}/{}", src, m)).is_file(), "universal layer {} is no file", m);
        }
        for (name, _, _, _, _, _) in crate::gates::REGISTRY {
            let (_, module) = GATE_MODULES
                .iter()
                .find(|(n, _)| n == name)
                .unwrap_or_else(|| panic!("{} has no module in the build-time map", name));
            assert!(module_file(&src, &format!("gates/{}", module)).is_some(), "{} maps to no module file", name);
        }
        assert_eq!(GATE_MODULES.len(), crate::gates::REGISTRY.len(), "the map carries a row the registry does not");
        let (_, module) = GATE_MODULES.iter().find(|(n, _)| *n == "check-value-rollup-fresh").expect("mapped");
        let mut got = cut(&src, module);
        got.sort();
        let mut want = vec![
            format!("gates/{}.rs", module),
            "emit/value_rollup.rs".to_string(),
            "emit/enforcement_map.rs".to_string(),
            "emit/footprint.rs".to_string(),
            "fresh.rs".to_string(),
            "marker.rs".to_string(),
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn a_braced_use_group_spells_one_path_per_leaf() {
        let got = use_paths(
            "use crate::{walk, emit::{graph as g, self}};\nuse crate::emit::footprint as fp;\nlet x = super::field(crate::fresh::y);\n",
            &["crate", "super"],
        );
        let want: Vec<(String, Option<String>)> = [
            ("crate::walk", None),
            ("crate::emit::graph", Some("g")),
            ("crate::emit::self", None),
            ("crate::emit::footprint", Some("fp")),
            ("super::field", None),
            ("crate::fresh::y", None),
        ]
        .into_iter()
        .map(|(p, a)| (p.to_string(), a.map(String::from)))
        .collect();
        assert_eq!(got, want);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — the derivation over every registry member:
    // a declared static name yields its kit's file, and the sentinel yields exactly the kits the
    // corpus's tokens name beside the member's own
    #[test]
    fn every_member_derives_the_knob_files_its_declaration_reaches() {
        let knobs = crate::knobenv::lock();
        knobs.remove("GATE_SDK_GATES_DIR");
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let dirs = repo_check_dirs(&repo);
        let file = |k: &crate::knobs::Kit| {
            format!("{}/{}-config.knobs", crate::knobs::GATES_DIR_DEFAULT, k.stem())
        };
        let corpus: Vec<&str> = couples_knob_names(&dirs)
            .expect("the descriptor corpus is readable")
            .iter()
            .filter_map(|t| crate::knobs::owner(t).map(|k| k.root))
            .collect();
        assert!(
            !corpus.is_empty(),
            "the descriptor corpus names no static knob token, so the sentinel half held over nothing"
        );
        let (mut declaring, mut sentinel) = (0, 0);
        for (member, _) in crate::gates::names_with_owners() {
            let got = knob_files(member, &dirs).expect("no registry member is staged");
            let declared = crate::gates::declared(member).unwrap_or(&[]);
            let own: Vec<&str> = declared
                .iter()
                .map(|n| n.trim_end_matches('*'))
                .filter_map(|n| crate::knobs::owner(n).filter(|k| !k.is_env_only(n)).map(|k| k.root))
                .collect();
            for kit in crate::knobs::STATIC_KITS.iter().filter(|k| own.contains(&k.root)) {
                declaring += 1;
                assert!(
                    got.contains(&file(kit)),
                    "{} declares a {} knob but derives no couple on {}",
                    member,
                    kit.root,
                    file(kit)
                );
            }
            if declared.contains(&EVERY_COUPLES_KNOB) {
                sentinel += 1;
                let reached: Vec<&str> = crate::knobs::STATIC_KITS
                    .iter()
                    .filter(|k| corpus.contains(&k.root) || own.contains(&k.root))
                    .map(|k| k.root)
                    .collect();
                let referents = referent_kits(&repo, &reached);
                let want: Vec<String> = crate::knobs::STATIC_KITS
                    .iter()
                    .filter(|k| reached.contains(&k.root) || referents.contains(&k.root))
                    .map(|k| file(k))
                    .collect();
                assert_eq!(got, want, "{}'s sentinel derivation", member);
            }
        }
        assert!(declaring > 0 && sentinel > 0, "one half of the derivation held over no member");
    }

    fn referent_kits(repo: &Path, reached: &[&str]) -> Vec<&'static str> {
        let mut out: Vec<&'static str> = Vec::new();
        for kit in crate::knobs::STATIC_KITS.iter().filter(|k| reached.contains(&k.root)) {
            let path = repo.join(crate::knobs::GATES_DIR_DEFAULT).join(format!("{}-config.knobs", kit.stem()));
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for e in crate::knobfile::parse(&text, "tracked").expect("a tracked knob file parses") {
                if let crate::knobfile::Form::Reference(other) = e.form {
                    out.extend(crate::knobs::owner(&other).map(|k| k.root));
                }
            }
        }
        out
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a reference line in a reached kit's tracked
    // file reaches its referent's kit; the same line in the local overlay reaches nothing
    #[test]
    fn a_tracked_reference_reaches_its_referents_kit_and_the_overlay_does_not() {
        let knobs = crate::knobenv::lock();
        let d = std::env::temp_dir().join(format!("checkwright-knob-refs.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        knobs.set("GATE_SDK_GATES_DIR", &d.display().to_string());
        let dirs: Vec<String> = Vec::new();
        let member = "check-trajectory-fresh";
        let lifecycle = format!("{}/lifecycle-config.knobs", d.display());
        std::fs::write(d.join("drift-config.local.knobs"), "DRIFT_KIT_STAGES[] <- LIFECYCLE_KIT_STAGES\n").expect("write");
        assert!(!knob_files(member, &dirs).expect("derives").contains(&lifecycle));
        std::fs::write(d.join("drift-config.knobs"), "DRIFT_KIT_STAGES[] <- LIFECYCLE_KIT_STAGES\n").expect("write");
        assert!(knob_files(member, &dirs).expect("derives").contains(&lifecycle));
        knobs.remove("GATE_SDK_GATES_DIR");
        let _ = std::fs::remove_dir_all(&d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a staged member reaching a static kit is a
    // refusal, because the staged hook would hand it the derived file as a path to scan
    #[test]
    fn a_staged_member_reaching_a_static_kit_is_refused() {
        let _knobs = crate::knobenv::lock();
        let d = std::env::temp_dir().join(format!("checkwright-knob-files.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let dirs = vec![d.display().to_string()];
        let member = "check-queue-sections";
        let write = |mode: &str| {
            std::fs::write(
                d.join(format!("{}.gate", member)),
                format!("# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit{}\n", mode),
            )
            .expect("write");
        };
        write("");
        let whole = knob_files(member, &dirs);
        write(" mode=staged");
        let staged = knob_files(member, &dirs);
        let _ = std::fs::remove_dir_all(&d);
        assert!(whole.is_ok_and(|f| !f.is_empty()), "the whole-tree member derives its kit's file");
        assert!(staged.is_err(), "the staged member must refuse rather than derive");
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — each refusal the knob token carries: a name no
    // static kit owns, a nested token, and a member unrepresentable after expansion. None may
    // degrade to an empty expansion, because an empty expansion is a lost trigger.
    #[test]
    fn every_knob_token_refusal_is_an_error_and_never_an_empty_expansion() {
        let knobs = crate::knobenv::lock();
        let roots = vec!["gate-sdk".to_string()];
        assert!(expand_couples("knob:PROBE_ABSENT", &roots).is_err());
        for bad in ["knob:PROBE_NEST", "a,b", "has space"] {
            let d = scratch_corpus(&knobs, "bad", &format!("CANON_KIT_MANIFEST_FILES[] = {}\n", bad));
            assert!(
                expand_couples("knob:CANON_KIT_MANIFEST_FILES", &roots).is_err(),
                "member {:?} must refuse",
                bad
            );
            unscratch(&knobs, &d);
        }
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a rooted token is its knob's one directory
    // with any trailing `/` trimmed, then its glob, covering-converted; an empty root, a row holding
    // more than one directory and an undeclared name each refuse
    #[test]
    fn a_rooted_token_expands_under_its_directory_and_refuses_what_is_no_directory() {
        let knobs = crate::knobenv::lock();
        let roots = vec!["gate-sdk".to_string()];
        let d = scratch_corpus(&knobs, "rooted", "");
        knobs.remove("GATE_SDK_WORKFLOW_DIR");
        std::fs::write(d.join("gate-sdk-config.knobs"), "GATE_SDK_WORKFLOW_DIR = wf/\n").expect("write");
        crate::knobs::reset(&knobs);
        assert_eq!(
            expand_couples("knob:GATE_SDK_GATES_DIR/gates.list,knob:GATE_SDK_WORKFLOW_DIR/*.md", &roots).unwrap(),
            format!("*{}/gates.list,*wf/*.md", d.display())
        );
        for bad in ["knob:GATE_SDK_PROGRAM_FLOOR/x", "knob:PROBE_ABSENT_DIR/x", "knob:GATE_SDK_WORKFLOW_DIR/a b"] {
            assert!(expand_couples(bad, &roots).is_err(), "{} must refuse", bad);
        }
        std::fs::write(d.join("gate-sdk-config.knobs"), "GATE_SDK_WORKFLOW_DIR =\n").expect("write");
        crate::knobs::reset(&knobs);
        let e = expand_couples("knob:GATE_SDK_WORKFLOW_DIR/*.md", &roots).unwrap_err();
        assert!(e.contains("resolves to no directory"), "{}", e);
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a rooted token over a word list roots its glob
    // at each word on the scalar root's terms, and an empty list expands to no pattern
    #[test]
    fn a_rooted_token_over_a_word_list_expands_once_per_word() {
        let knobs = crate::knobenv::lock();
        let roots = vec!["gate-sdk".to_string()];
        let d = scratch_corpus(&knobs, "rootedwords", "");
        knobs.remove("GATE_SDK_LINT_EXTRA_DIRS");
        std::fs::write(d.join("gate-sdk-config.knobs"), "GATE_SDK_LINT_EXTRA_DIRS = ops/ ./tools\n").expect("write");
        crate::knobs::reset(&knobs);
        assert_eq!(
            expand_couples("knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh", &roots).unwrap(),
            "*ops/*.sh,*tools/*.sh"
        );
        assert_eq!(
            knob_paths("knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh").unwrap(),
            vec!["ops/*.sh".to_string(), "tools/*.sh".to_string()]
        );
        std::fs::write(d.join("gate-sdk-config.knobs"), "GATE_SDK_LINT_EXTRA_DIRS = ops a,b\n").expect("write");
        crate::knobs::reset(&knobs);
        assert!(expand_couples("knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh", &roots).is_err());
        std::fs::write(d.join("gate-sdk-config.knobs"), "GATE_SDK_LINT_EXTRA_DIRS =\n").expect("write");
        crate::knobs::reset(&knobs);
        assert_eq!(expand_couples("knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh,a/b", &roots).unwrap(), "a/b");
        assert!(knob_paths("knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh").unwrap().is_empty());
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — a `.` root contributes no prefix and a
    // leading `./` is dropped, so a rooted token over the repository root covers what a walk returns
    #[test]
    fn a_dot_root_contributes_no_prefix_to_a_rooted_token() {
        let knobs = crate::knobenv::lock();
        let roots = vec!["gate-sdk".to_string()];
        let d = scratch_corpus(&knobs, "dotroot", "");
        knobs.remove("GATE_SDK_ENFORCE_SCAN_DIR");
        for (value, want) in [(".", "*"), ("./", "*"), ("./ops", "*ops/*"), ("./ops/", "*ops/*")] {
            std::fs::write(d.join("gate-sdk-config.knobs"), format!("GATE_SDK_ENFORCE_SCAN_DIR = {}\n", value))
                .expect("write");
            crate::knobs::reset(&knobs);
            assert_eq!(expand_couples("knob:GATE_SDK_ENFORCE_SCAN_DIR/*", &roots).unwrap(), want, "root {:?}", value);
        }
        assert!(couple_matches(".github/workflows/ci.yml", "*"));
        unscratch(&knobs, &d);
    }

    // spec: gate-sdk/SPEC.md §The install disposition — the knob-path token resolves verbatim: a
    // literal to itself, a file knob to its value, a rooted token to its path, and `kit:` refuses
    #[test]
    fn a_knob_path_token_resolves_to_the_place_verbatim() {
        let knobs = crate::knobenv::lock();
        let d = scratch_corpus(&knobs, "knobpaths", "");
        knobs.remove("GATE_SDK_ENFORCEMENT_FILE");
        knobs.remove("GATE_SDK_GRAPH_THEME_DIR");
        std::fs::write(
            d.join("gate-sdk-config.knobs"),
            "GATE_SDK_ENFORCEMENT_FILE = site/enforcement.md\nGATE_SDK_GRAPH_THEME_DIR = ./theme/\n",
        )
        .expect("write");
        crate::knobs::reset(&knobs);
        assert_eq!(knob_paths("docs/x.md").unwrap(), vec!["docs/x.md".to_string()]);
        assert_eq!(knob_paths("knob:GATE_SDK_ENFORCEMENT_FILE").unwrap(), vec!["site/enforcement.md".to_string()]);
        assert_eq!(knob_paths("knob:GATE_SDK_GRAPH_THEME_DIR/base.css").unwrap(), vec!["theme/base.css".to_string()]);
        assert!(knob_paths("kit:templates/*.md").is_err());
        assert!(knob_paths("knob:PROBE_ABSENT").is_err());
        knobs.remove("QUEUE_KIT_ROADMAP_FILE");
        crate::knobs::reset(&knobs);
        assert!(knob_paths("knob:QUEUE_KIT_ROADMAP_FILE").unwrap().is_empty(), "an empty value is no place");
        unscratch(&knobs, &d);
    }
}

#[cfg(test)]
mod exception_array_tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — both tag spacings arm the next assignment
    #[test]
    fn tags_arm_only_the_next_assignment_shape() {
        let a = exception_arrays("#exception-list:\nignored text\n NAME=(\n a/b\n)\nOTHER=(ignored)\n");
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].header, 1);
        assert_eq!(a[0].elements[0].line, 4);
        assert_eq!(a[0].elements[0].value, "a/b");
        assert_eq!(exception_arrays("# exception-list:\nA=(a/b)\n")[0].elements[0].value, "a/b");
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — comments and quoted parentheses do not close the balance
    #[test]
    fn comments_and_quotes_protect_balance_and_raw_dispositions() {
        let a = exception_arrays("# exception-list:\nA=( # )\n 'a/).rs' # permanent: ) reason\n b/c # until: live-task\n)\n");
        assert_eq!(a[0].elements.len(), 2);
        assert_eq!(a[0].elements[0].value, "a/).rs");
        assert_eq!(disposition(&a[0].elements[0].raw), Disposition::Permanent);
        assert_eq!(disposition(&a[0].elements[1].raw), Disposition::Until("live-task".into()));
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — each line yields its first token, with quotes removed
    #[test]
    fn one_line_is_one_element_and_opening_elements_are_flagged() {
        let a = exception_arrays("#exception-list:\nA=(\"a/b\" c/d)\n# exception-list:\nB=(\n 'e/f' g/h # permanent: reason\n)\n");
        assert_eq!(a[0].elements.len(), 1);
        assert_eq!(a[0].elements[0].value, "a/b");
        assert!(a[0].elements[0].opening);
        assert_eq!(a[1].elements[0].value, "e/f");
        assert!(!a[1].elements[0].opening);
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — a tag inside an open array arms the next assignment
    #[test]
    fn a_tag_inside_an_open_array_arms_the_next_assignment() {
        let a = exception_arrays("# exception-list:\nA=(\n a/b\n# exception-list:\n)\nB=(\n c/d\n)\n");
        assert_eq!(a.len(), 2);
        assert_eq!(a[1].header, 4);
        assert_eq!(a[1].elements[0].value, "c/d");
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — quote state carries across lines as bash reads it, and a word's raw line is its last
    #[test]
    fn a_quote_spanning_lines_holds_the_balance_open() {
        let a = exception_arrays("# exception-list:\nA=(\n 'a/b\n)' # permanent: x\n c/d\n# exception-list: quoted, so no tag\n)\n");
        assert_eq!(a.len(), 1);
        let values: Vec<&str> = a[0].elements.iter().map(|e| e.value.as_str()).collect();
        assert_eq!(values, ["a/b\n)", "c/d"]);
        assert_eq!(a[0].elements[0].line, 3);
        assert_eq!(disposition(&a[0].elements[0].raw), Disposition::Permanent, "a word's disposition sits on the line it ends on");
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — a backslash escapes outside single quotes only
    #[test]
    fn a_backslash_escapes_outside_single_quotes_only() {
        let a = exception_arrays("# exception-list:\nA=(\n \"a/\\\").rs\" # x\n 'b/\\' # x\n c/d\\ e # x\n f\\'g\n h#i # x\n j/k\n)\n");
        let values: Vec<&str> = a[0].elements.iter().map(|e| e.value.as_str()).collect();
        assert_eq!(values, ["a/\").rs", "b/\\", "c/d e", "f'g", "h#i", "j/k"]);
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — a trailing backslash continues the word onto the next line
    #[test]
    fn a_trailing_backslash_continues_the_word() {
        let a = exception_arrays("# exception-list:\nA=(\n a/\\\nb # x\n c\n)\n");
        let values: Vec<&str> = a[0].elements.iter().map(|e| e.value.as_str()).collect();
        assert_eq!(values, ["a/b", "c"]);
    }
}
