// spec: guard-kit/SPEC.md §The generic ruleset — the tool steers: a shell spelling of what a dedicated
// tool or a kit arm does better, and the waits and process queries that cannot end.
use super::{
    command_word, has_expansion, has_substitution, is_banner, loop_span, on_ro_roster, ro_invocation_clear,
    segment_core,
};
use crate::ere::Ere;
use crate::guard::engine::{Cmd, Ctx, Decided, Fault, Verdict};
use crate::guard::reader::View::SqDqHd;
use crate::guard::text::{self, grep_q, head_word, trim_start, words};

fn block(m: impl Into<String>) -> Decided {
    Ok(Some(Verdict::Block(m.into())))
}

#[derive(Default)]
struct Program {
    prog: String,
    inplace: bool,
    inline: bool,
    operands: Vec<String>,
    scripts: Vec<String>,
    from_file: bool,
    unmodelled: bool,
}

// spec: guard-kit/SPEC.md §The rule roster — rule `ro_pipeline`'s `sed` discriminator: the options
// that take no argument and change no command a script can reach.
fn sed_plain_option(t: &str) -> bool {
    matches!(
        t,
        "-n" | "--quiet"
            | "--silent"
            | "-E"
            | "-r"
            | "--regexp-extended"
            | "-s"
            | "--separate"
            | "-u"
            | "--unbuffered"
            | "-z"
            | "--null-data"
            | "--posix"
            | "--debug"
            | "--sandbox"
    ) || (t.len() > 2 && !t.starts_with("--") && t[1..].chars().all(|c| "nErsuz".contains(c)))
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `sed_file`'s one program-then-operands walk,
// per tool: the program word, its file operands and every program text on the command line; `None`
// on an awk, perl or python option the table does not carry.
fn program_operands(tool: &str, seg: &str) -> Option<Program> {
    let mut p = Program::default();
    let (mut skip, mut ends, mut have_prog) = ("", false, false);
    let digit_op = |t: &str, op: &str| t.len() == 1 + op.len() && t.as_bytes()[0].is_ascii_digit() && &t[1..] == op;
    for raw_tok in words(seg).into_iter().skip(1) {
        let tok = text::unsentinel(raw_tok);
        let t = tok.as_str();
        if !skip.is_empty() {
            if skip == "prog" {
                have_prog = true;
            }
            if skip == "script" {
                p.scripts.push(tok.clone());
                have_prog = true;
            }
            if skip == "text" {
                p.prog = tok.clone();
                p.inline = true;
                have_prog = true;
                ends = true;
            }
            skip = "";
            continue;
        }
        if !ends {
            let handled = match tool {
                "sed" => {
                    if t == "-i" || t.starts_with("-i") || t.starts_with("--in-place") {
                        p.inplace = true;
                    } else if t == "-e" || t == "--expression" {
                        skip = "script";
                    } else if t == "-f" || t == "--file" {
                        skip = "prog";
                        p.from_file = true;
                    } else if let Some(x) = t.strip_prefix("--expression=") {
                        p.scripts.push(x.to_string());
                        have_prog = true;
                    } else if t.starts_with("--file=") {
                        have_prog = true;
                        p.from_file = true;
                    } else if t.len() >= 2 && t.starts_with('-') && !t[1..].starts_with('-') && t.contains('i') {
                        p.inplace = true;
                    } else if t.len() > 2 && !t.starts_with("--") && t.ends_with('e') && sed_plain_option(&t[..t.len() - 1]) {
                        skip = "script";
                    } else if t.len() >= 2 && t.starts_with('-') && !sed_plain_option(t) {
                        p.unmodelled = true;
                    }
                    t.len() >= 2 && t.starts_with('-')
                }
                "awk" => match t {
                    "--" => {
                        ends = true;
                        true
                    }
                    "-F" | "-v" => {
                        skip = "arg";
                        true
                    }
                    "-f" => {
                        skip = "prog";
                        p.from_file = true;
                        true
                    }
                    _ if t.len() > 2 && (t.starts_with("-F") || t.starts_with("-v")) => true,
                    _ if t.len() > 2 && t.starts_with("-f") => {
                        have_prog = true;
                        p.from_file = true;
                        true
                    }
                    _ if t.starts_with('-') => return None,
                    _ => false,
                },
                "perl" => match t {
                    "--" => {
                        ends = true;
                        true
                    }
                    "-" => return None,
                    _ if t.starts_with("--") => return None,
                    _ if t.starts_with('-') => {
                        let mut bundle = &t[1..];
                        while let Some(letter) = bundle.chars().next() {
                            bundle = &bundle[letter.len_utf8()..];
                            match letter {
                                'a' | 'c' | 'n' | 'p' | 's' | 't' | 'T' | 'u' | 'U' | 'w' | 'W' | 'X' => {}
                                '0' | 'l' => bundle = bundle.trim_start_matches(|c: char| c.is_ascii_digit()),
                                'e' | 'E' => {
                                    if bundle.is_empty() {
                                        skip = "prog";
                                    } else {
                                        have_prog = true;
                                    }
                                    bundle = "";
                                }
                                'i' => {
                                    p.inplace = true;
                                    bundle = "";
                                }
                                _ => return None,
                            }
                        }
                        true
                    }
                    _ => false,
                },
                "python" => match t {
                    "-c" => {
                        skip = "text";
                        true
                    }
                    "-" => {
                        have_prog = true;
                        ends = true;
                        true
                    }
                    "-u" | "-B" | "-E" | "-I" | "-s" | "-S" | "-O" | "-q" => true,
                    "-W" | "-X" => {
                        skip = "arg";
                        true
                    }
                    "<<" | "<<-" | "<" | ">" | ">>" => {
                        skip = "arg";
                        true
                    }
                    _ if digit_op(t, ">") || digit_op(t, ">>") => {
                        skip = "arg";
                        true
                    }
                    _ if t.starts_with('<') || t.starts_with('>') => true,
                    _ if t.len() >= 2 && t.as_bytes()[0].is_ascii_digit() && t[1..].starts_with('>') => true,
                    _ if t.starts_with('-') => return None,
                    _ => false,
                },
                _ => false,
            };
            if handled {
                continue;
            }
        }
        if !have_prog {
            p.scripts.push(tok.clone());
            p.prog = tok;
            have_prog = true;
        } else {
            p.operands.push(tok);
        }
    }
    Some(p)
}

pub fn sed_file(ctx: &Ctx) -> Decided {
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    let door = &ctx.host().door;
    for seg in ctx.segments(&s) {
        let seg = trim_start(&seg);
        let tool = if seg == "sed" || starts_word(seg, "sed") {
            "sed"
        } else if starts_word(seg, "perl") {
            "perl"
        } else {
            continue;
        };
        let Some(p) = program_operands(tool, seg) else { continue };
        if p.inplace && (tool == "sed" || !p.operands.is_empty()) {
            return block(format!("don't rewrite a file with '{} -i' — use the rewrite arm: '{} --rewrite [--regex] [--expect <n>] [--] <find> <replace> <file>…' replaces a literal (or, with --regex, a line-scoped POSIX ERE) with fixed text across every named file and prints each changed span. For an edit a fixed replacement cannot express (a capture group, a deletion keyed on context), use the Edit tool. If you genuinely need the in-place edit, run it yourself with !<command>.", tool, door));
        }
        if tool == "sed" && !p.operands.is_empty() {
            return block("don't read a file through 'sed' — use the Read tool (offset/limit for a line range): it returns numbered lines and registers the file for a later Edit. For a markdown section, the consumer's section extractor beats a line range. If you genuinely need sed, pipe into it or run it yourself with !<command>.");
        }
    }
    if let Some(v) = awk_read(ctx, ctx.cmd(), &s)? {
        return Ok(Some(v));
    }
    if let Some(v) = stream_range(ctx, ctx.cmd())? {
        return Ok(Some(v));
    }
    python_rewrite(ctx, ctx.cmd(), &s)
}

// spec: guard-kit/SPEC.md §The generic ruleset — `<word>[[:space:]]*`, the word then a blank.
fn starts_word(seg: &str, w: &str) -> bool {
    seg.strip_prefix(w).is_some_and(|r| r.starts_with(|c: char| c.is_ascii_whitespace() || c == '\x0b'))
}

// spec: guard-kit/SPEC.md §The rule roster — rule `sed_file`'s `NR`-comparison pattern, blanks
// squeezed out, with no action or the print-all action.
fn nr_range(prog: &str) -> bool {
    let squeezed: String = prog.chars().filter(|c| !c.is_ascii_whitespace() && *c != '\x0b').collect();
    text::matches(
        "^NR(==|>=|<=|>|<)[0-9]+(&&NR(==|>=|<=|>|<)[0-9]+)*(,NR(==|>=|<=|>|<)[0-9]+(&&NR(==|>=|<=|>|<)[0-9]+)*)?(\\{print(\\$0)?\\})?$",
        &squeezed,
    )
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `sed_file`'s awk arm (i): a pipeline-head awk
// with exactly one file operand whose program is a line range.
fn awk_read(ctx: &Ctx, c: &Cmd, s: &str) -> Result<Option<Verdict>, Fault> {
    if !s.contains("awk") {
        return Ok(None);
    }
    let Some(v) = ctx.dequoted(c)? else { return Ok(None) };
    for stmt in ctx.statements(&v) {
        let pipes = ctx.pipes(&stmt);
        let seg = trim_start(pipes.first().map_or("", String::as_str));
        if !starts_word(seg, "awk") {
            continue;
        }
        let Some(p) = program_operands("awk", seg) else { continue };
        if p.prog.is_empty() || p.operands.len() != 1 {
            continue;
        }
        let operand = &p.operands[0];
        if operand == "-" || operand == "/dev/stdin" {
            continue;
        }
        if nr_range(&p.prog) {
            return Ok(Some(Verdict::Block(format!("don't read a line range through 'awk' — use the Read tool with offset/limit: it returns numbered lines and registers the file for a later Edit. For a markdown section, '{} --emit md-section <file> \"<heading>\"' prints exactly the section under that heading. An awk program carrying an action, read from a program file, or given a second operand is a transform or a filter and untouched. If you genuinely need awk, run it yourself with !<command>.", ctx.host().door))));
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `sed_file`'s stream arm (ii): a piped-into `sed -n`
// whose whole script is a range print, or a piped-into awk whose program is arm (i)'s pattern, each
// with no operand.
fn stream_range(ctx: &Ctx, c: &Cmd) -> Result<Option<Verdict>, Fault> {
    let Some(v) = ctx.dequoted(c)? else { return Ok(None) };
    for stmt in ctx.statements(&v) {
        for seg in ctx.pipes(&stmt).iter().skip(1) {
            let seg = trim_start(seg);
            let tool = if starts_word(seg, "sed") {
                "sed"
            } else if starts_word(seg, "awk") {
                "awk"
            } else {
                continue;
            };
            let Some(p) = program_operands(tool, seg) else { continue };
            if !p.operands.is_empty() || p.from_file || p.unmodelled || p.inplace || p.scripts.len() != 1 {
                continue;
            }
            let range = match tool {
                "sed" => {
                    let quiet = words(seg).iter().skip(1).map(|w| text::unsentinel(w)).any(|w| {
                        w == "--quiet" || w == "--silent" || (w.starts_with('-') && !w.starts_with("--") && w.contains('n'))
                    });
                    let squeezed: String = p.scripts[0].chars().filter(|c| !c.is_ascii_whitespace()).collect();
                    quiet && text::matches("^[0-9]+(,[0-9]+)?p$", &squeezed)
                }
                _ => nr_range(&p.scripts[0]),
            };
            if range {
                return Ok(Some(Verdict::Block(format!("don't print a stream's line range through '{t}' — pipe into 'head -n <B> | tail -n +<A>' (or 'head -n <A> | tail -n 1' for one line) instead: the read-only roster's spelling of the same lines, granted with no permission decision. A {t} program doing anything beyond the range print is a filter and untouched. If you genuinely need {t}, run it yourself with !<command>.", t = tool))));
            }
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `ro_pipeline`'s program discriminator: a `sed` or
// `awk` segment whose every program text is on the command line and clears the static check.
pub(super) fn program_clear(tool: &str, dseg: &str) -> bool {
    let Some(p) = program_operands(tool, dseg) else { return false };
    if p.from_file || p.unmodelled || p.inplace || p.scripts.is_empty() {
        return false;
    }
    p.scripts.iter().all(|s| match tool {
        "sed" => sed_script_clear(s),
        "awk" => awk_program_clear(s),
        _ => false,
    })
}

// spec: guard-kit/SPEC.md §The rule roster — the `sed` check: each command walked by its address,
// `!` and letter; `w`, `W`, `e` and an `s` flag carrying `w` or `e` withhold, as does a letter the
// walk does not model.
fn sed_script_clear(script: &str) -> bool {
    let b: Vec<char> = script.chars().collect();
    let n = b.len();
    let blank = |c: char| c == ' ' || c == '\t';
    let to_eol = |mut i: usize| {
        while i < n && b[i] != '\n' {
            i += 1;
        }
        i
    };
    let field = |mut i: usize, d: char| -> Option<usize> {
        while i < n {
            if b[i] == '\\' {
                i += 2;
                continue;
            }
            if b[i] == '\n' {
                return None;
            }
            if b[i] == d {
                return Some(i + 1);
            }
            i += 1;
        }
        None
    };
    let address = |mut i: usize| -> Option<usize> {
        if i < n && b[i].is_ascii_digit() {
            while i < n && b[i].is_ascii_digit() {
                i += 1;
            }
            if i < n && b[i] == '~' {
                i += 1;
                while i < n && b[i].is_ascii_digit() {
                    i += 1;
                }
            }
        } else if i < n && b[i] == '$' {
            i += 1;
        } else if i < n && (b[i] == '/' || b[i] == '\\') {
            let d = if b[i] == '\\' {
                i += 1;
                *b.get(i)?
            } else {
                '/'
            };
            i = field(i + 1, d)?;
            while i < n && (b[i] == 'I' || b[i] == 'M') {
                i += 1;
            }
        }
        Some(i)
    };
    let mut i = 0usize;
    let mut depth = 0i32;
    loop {
        while i < n && (blank(b[i]) || b[i] == '\n' || b[i] == ';') {
            i += 1;
        }
        if i >= n {
            return depth == 0;
        }
        if b[i] == '#' {
            i = to_eol(i);
            continue;
        }
        let start = i;
        let Some(j) = address(i) else { return false };
        i = j;
        if i > start && i < n && b[i] == ',' {
            i += 1;
            if i < n && (b[i] == '+' || b[i] == '~') {
                i += 1;
                while i < n && b[i].is_ascii_digit() {
                    i += 1;
                }
            } else {
                let Some(k) = address(i) else { return false };
                i = k;
            }
        }
        while i < n && blank(b[i]) {
            i += 1;
        }
        if i < n && b[i] == '!' {
            i += 1;
            while i < n && blank(b[i]) {
                i += 1;
            }
        }
        let Some(&cmd) = b.get(i) else { return false };
        i += 1;
        match cmd {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            's' | 'y' => {
                let Some(&d) = b.get(i) else { return false };
                if d == '\n' || d == '\\' {
                    return false;
                }
                let Some(k) = field(i + 1, d) else { return false };
                let Some(k) = field(k, d) else { return false };
                i = k;
                while cmd == 's' && i < n && !matches!(b[i], ';' | '\n' | '}') {
                    let flag = b[i].is_ascii_digit() || matches!(b[i], 'g' | 'p' | 'i' | 'I' | 'm' | 'M') || blank(b[i]);
                    if !flag {
                        return false;
                    }
                    i += 1;
                }
            }
            'a' | 'i' | 'c' | 'r' | 'R' => i = to_eol(i),
            'b' | 't' | 'T' | ':' => {
                while i < n && !matches!(b[i], ';' | '\n') {
                    i += 1;
                }
            }
            'q' | 'Q' | 'l' => {
                while i < n && (blank(b[i]) || b[i].is_ascii_digit()) {
                    i += 1;
                }
            }
            'p' | 'P' | 'd' | 'D' | 'n' | 'N' | 'g' | 'G' | 'h' | 'H' | 'x' | 'z' | 'F' | '=' => {}
            _ => return false,
        }
    }
}

// spec: guard-kit/SPEC.md §The rule roster — the `awk` check: the program tokenized with strings,
// regex literals and comments skipped; `system`, a lone `|`, `@`, an output redirect in a print
// statement and an unterminated literal withhold.
fn awk_program_clear(prog: &str) -> bool {
    let b: Vec<char> = prog.chars().collect();
    let n = b.len();
    let mut i = 0usize;
    let mut depth = 0i32;
    let mut print_depth: Option<i32> = None;
    let mut operand = false;
    while i < n {
        let c = b[i];
        if c == '"' {
            i += 1;
            while i < n && b[i] != '"' {
                if b[i] == '\\' {
                    i += 1;
                }
                if i < n && b[i] == '\n' {
                    return false;
                }
                i += 1;
            }
            if i >= n {
                return false;
            }
            i += 1;
            operand = true;
            continue;
        }
        if c == '/' && !operand {
            i += 1;
            let mut class = false;
            while i < n && (b[i] != '/' || class) {
                match b[i] {
                    '\\' => i += 1,
                    '[' => class = true,
                    ']' => class = false,
                    '\n' => return false,
                    _ => {}
                }
                i += 1;
            }
            if i >= n {
                return false;
            }
            i += 1;
            operand = true;
            continue;
        }
        if c == '#' {
            while i < n && b[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < n && (b[i].is_ascii_alphanumeric() || b[i] == '_') {
                i += 1;
            }
            let word: String = b[start..i].iter().collect();
            match word.as_str() {
                "system" => return false,
                "print" | "printf" => {
                    print_depth = Some(depth);
                    operand = false;
                }
                "return" | "in" | "case" | "do" | "else" => operand = false,
                _ => operand = true,
            }
            continue;
        }
        if c.is_ascii_digit() || (c == '.' && b.get(i + 1).is_some_and(char::is_ascii_digit)) {
            while i < n && (b[i].is_ascii_alphanumeric() || b[i] == '.') {
                i += 1;
            }
            operand = true;
            continue;
        }
        match c {
            '@' => return false,
            '|' if b.get(i + 1) == Some(&'|') => {
                i += 2;
                operand = false;
                continue;
            }
            '|' => return false,
            '>' if print_depth == Some(depth) => return false,
            '(' | '[' => {
                depth += 1;
                operand = false;
            }
            ')' | ']' => {
                depth -= 1;
                operand = true;
            }
            ';' | '\n' | '{' | '}' => {
                if print_depth.is_some_and(|d| depth <= d) {
                    print_depth = None;
                }
                operand = false;
            }
            '+' | '-' if b.get(i + 1) == Some(&c) => {
                i += 2;
                continue;
            }
            ' ' | '\t' => {}
            _ => operand = false,
        }
        i += 1;
    }
    true
}

// spec: guard-kit/SPEC.md §The generic ruleset — the literal-rewrite test on a python body: it writes
// a file, calls `.replace(` and carries no computed-text construct; `live` is an unquoted-delimiter
// body, where a `$` is computed text.
fn is_literal_rewrite(b: &str, live: bool) -> bool {
    if !b.contains(".replace(") {
        return false;
    }
    let open = text::matches(
        "open\\(([^)]*,)?[[:space:]]*(mode[[:space:]]*=[[:space:]]*)?[\"'][rbtx+]*[wa][rbtx+]*[\"'][[:space:]]*[,)]",
        b,
    );
    if !open && !b.contains(".write_text(") {
        return false;
    }
    if text::matches(
        "(^|[^A-Za-z0-9_])[rRbB]?[fF][rR]?[\"']|[\"'][[:space:]]*%|(^|[^A-Za-z0-9_])re\\.|(^|[^A-Za-z0-9_])import([^A-Za-z0-9_]|$)|\\.(format|index|find|join|split)\\(|\\[[^]]*:[^]]*\\]|(^|[^A-Za-z0-9_])input\\(|(^|[^A-Za-z0-9_])sys\\.",
        b,
    ) {
        return false;
    }
    !(live && b.contains('$'))
}

fn python_block(door: &str) -> Decided {
    block(format!("don't rewrite a file with an inline python body — use the rewrite arm: '{} --rewrite [--expect <n>] [--] <find> <replace> <file>…' replaces a literal across every named file and prints each changed span; --expect <n> is the count assertion, one call per find/replace pair, and '\\n' escapes carry a multi-line literal on one line. For a multi-line literal you would rather not escape, use the Edit tool. If the program is intended as written, run it yourself with !<command>.", door))
}

fn is_python(seg: &str) -> bool {
    starts_word(seg, "python") || starts_word(seg, "python3")
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `sed_file`'s python arm: a python segment whose
// -c argument or stdin heredoc is a literal rewrite.
fn python_rewrite(ctx: &Ctx, c: &Cmd, s: &str) -> Decided {
    if !s.contains("python") {
        return Ok(None);
    }
    let door = &ctx.host().door;
    if let Some(v) = ctx.dequoted(c)? {
        for seg in ctx.segments(&v) {
            let seg = trim_start(&seg);
            if !is_python(seg) {
                continue;
            }
            let Some(p) = program_operands("python", seg) else { continue };
            if p.inline && is_literal_rewrite(&p.prog, false) {
                return python_block(door);
            }
        }
    }
    let mut k = 0usize;
    for seg in ctx.segments(s) {
        let seg = trim_start(&seg);
        let openers = text::grep_o(
            "(^|[^<])<<-?[[:space:]]*(\"[^\"]*\"|'[^']*'|[A-Za-z_][A-Za-z0-9_]*)",
            seg,
        );
        if is_python(seg) && !openers.is_empty() && !openers[0].starts_with(|c: char| c.is_ascii_digit()) {
            if let Some(p) = program_operands("python", seg) {
                if !p.inline && p.prog.is_empty() {
                    let body = ctx.body(c, k + 1)?;
                    let live = !openers[0].contains(['"', '\'']);
                    if is_literal_rewrite(&body, live) {
                        return python_block(door);
                    }
                }
            }
        }
        k += openers.len();
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — a `;`-compound skeleton is a batched read when every
// segment is a bare read or a literal banner, and at least one is a read.
fn is_read_batch(ctx: &Ctx, s: &str, pred: impl Fn(&str) -> bool) -> bool {
    let mut reads = 0usize;
    for seg in ctx.segments(s) {
        let seg = trim_start(&seg);
        if seg.is_empty() {
            continue;
        }
        if pred(seg) {
            reads += 1;
        } else if !is_banner(seg) {
            return false;
        }
    }
    reads >= 1
}

fn has_search_tool(ctx: &Ctx, t: &str) -> bool {
    ctx.host().search_tools.iter().any(|x| x == t)
}

pub fn find_glob(ctx: &Ctx) -> Decided {
    if !has_search_tool(ctx, "Glob") || has_substitution(ctx.raw(ctx.cmd())?) {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if grep_q(r"(&&|\|\||\||&|<|>)", &s) {
        return Ok(None);
    }
    let h = ctx.host();
    let listing = |seg: &str| {
        head_word(seg) == "find" && {
            let rest = seg.strip_prefix("find").unwrap_or(seg);
            ro_invocation_clear(h, "find", rest, &segment_core(rest), false)
        }
    };
    if !is_read_batch(ctx, &s, listing) {
        return Ok(None);
    }
    block("don't list files with a bare 'find' — use the Glob tool: it returns matching paths (registered for a later Read) and needs no permission decision at all. If your toolset carries no Glob tool, keep 'find' and pipe the listing into a read-only consumer ('find <dir> -type f | sort'), which the read-only pipeline grant allows. This fires on a lone listing and on a ';'-sequence of them (a literal echo/printf banner between them is fine); a 'find' carrying an action predicate (-exec/-delete/…), piped into a consumer, or redirected is untouched. If you genuinely need find, run it yourself with !<command>.")
}

// spec: guard-kit/SPEC.md §The rule roster — a word as a steer prints it: bare where the shell reads
// it as written, single-quoted otherwise.
fn shell_word(w: &str) -> String {
    if !w.is_empty() && w.bytes().all(|b| b.is_ascii_alphanumeric() || b"_./=:,+%@^-".contains(&b)) {
        w.to_string()
    } else {
        format!("'{}'", w.replace('\'', "'\\''"))
    }
}

pub fn find_exec(ctx: &Ctx) -> Decided {
    let c = ctx.cmd();
    let s = ctx.view(c, SqDqHd)?;
    if !s.contains("-exec") {
        return Ok(None);
    }
    let Some(v) = ctx.dequoted(c)? else { return Ok(None) };
    let (segs, dsegs) = (ctx.segments(&s), ctx.segments(&v));
    if segs.len() != dsegs.len() {
        return Ok(None);
    }
    let h = ctx.host();
    for (k, (seg, dseg)) in segs.iter().zip(&dsegs).enumerate() {
        let seg = trim_start(seg);
        if head_word(seg) != "find" || seg.contains("{}") {
            continue;
        }
        let mut dr: Vec<&str> = words(trim_start(dseg));
        if dr.last() == Some(&"\\") {
            if matches!(dsegs.get(k + 1), Some(n) if !trim_start(n).trim_end().is_empty()) {
                continue;
            }
            dr.pop();
            dr.push(";");
        }
        let dw: Vec<String> = dr.iter().map(|w| text::unsentinel(w)).collect();
        let execs: Vec<usize> = dw.iter().enumerate().filter(|(_, w)| *w == "-exec").map(|(i, _)| i).collect();
        if execs.len() != 1 {
            continue;
        }
        let e = execs[0];
        let Some(t) = (e + 1..dw.len()).find(|&k| matches!(dw[k].replace('\\', "").as_str(), ";" | "+")) else {
            continue;
        };
        let per_file = dw[t].replace('\\', "") == ";";
        if t + 1 != dw.len() || t < e + 3 || dw[t - 1] != "{}" || dw[e + 1..t - 1].iter().any(|w| w.contains("{}")) {
            continue;
        }
        let preds = dr[1..e].join(" ");
        if !ro_invocation_clear(h, "find", &preds, &preds, false) {
            continue;
        }
        let cmd = &dw[e + 1..t - 1];
        let bin = cmd[0].as_str();
        if bin == "xargs" || !on_ro_roster(h, bin) || !ro_invocation_clear(h, bin, &dr[e + 2..t - 1].join(" "), "", true) {
            continue;
        }
        if matches!(bin, "sed" | "awk") && !program_clear(bin, &dr[e + 1..t - 1].join(" ")) {
            continue;
        }
        let quoted = |ws: &[String]| ws.iter().map(|w| shell_word(w)).collect::<Vec<_>>().join(" ");
        let steer = format!(
            "find {} -print0 | xargs -0 {}{}",
            quoted(&dw[1..e]),
            if per_file { "-n 1 " } else { "" },
            quoted(cmd)
        );
        return block(format!("don't run a read-only command through 'find -exec' — pipe the listing into xargs instead: '{}' runs the same command over the same files, and the read-only pipeline grant allows it with no permission decision, while -exec is one of find's execute forms and costs one. A find carrying any other action (-execdir, -ok, -delete, -fprint), running a command outside the read-only roster, or carrying more than one -exec is untouched. If you genuinely need -exec, run it yourself with !<command>.", steer));
    }
    Ok(None)
}

pub fn cat_file(ctx: &Ctx) -> Decided {
    if has_substitution(ctx.raw(ctx.cmd())?) {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if grep_q(r"(&&|\|\||\||&|<|>)", &s) {
        return Ok(None);
    }
    let cat_read = |seg: &str| {
        head_word(seg) == "cat"
            && words(seg.strip_prefix("cat").unwrap_or(seg)).iter().filter(|t| !t.starts_with('-')).count() == 1
    };
    if !is_read_batch(ctx, &s, cat_read) {
        return Ok(None);
    }
    block("don't read files with a bare 'cat' — use the Read tool: it returns numbered lines registered for a later Edit, and needs no permission decision at all. This fires on a lone 'cat <file>' and on a ';'-sequence of them (a literal echo/printf banner between reads is fine — batch them into one Read); a 'cat' feeding a pipe or heredoc, redirecting, or concatenating multiple files in one command is composition and untouched. If you genuinely need cat, run it yourself with !<command>.")
}

pub fn git_grep(ctx: &Ctx) -> Decided {
    if !has_search_tool(ctx, "Grep") || ctx.expands(ctx.cmd(), has_substitution)? {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    if grep_q(r"(&&|\|\||;|\||&|<|>)", &s) {
        return Ok(None);
    }
    let toks = words(&s);
    if toks.first() != Some(&"git") || toks.get(1) != Some(&"grep") {
        return Ok(None);
    }
    let (mut positionals, mut want_arg, mut pat_opt) = (0usize, false, false);
    for tok in &toks[2..] {
        if want_arg {
            want_arg = false;
            continue;
        }
        match *tok {
            "--" => break,
            "--cached" | "--staged" | "--no-index" | "--untracked" => return Ok(None),
            "-e" | "-f" => {
                pat_opt = true;
                want_arg = true;
            }
            "-m" | "-A" | "-B" | "-C" | "--max-depth" | "--max-count" | "--threads" | "--context"
            | "--after-context" | "--before-context" => want_arg = true,
            t if t.starts_with('-') => {}
            _ => positionals += 1,
        }
    }
    let working_tree = if pat_opt { positionals == 0 } else { positionals == 1 };
    if !working_tree {
        return Ok(None);
    }
    block("don't search with 'git grep' over the working tree — use the Grep tool: it returns matching lines (files registered for a later Read) and needs no permission decision at all. If your toolset carries no Grep tool, bare 'grep -rn <pattern> <path>' searches the same working tree. A 'git grep' naming a revision, searching the index (--cached), or piped into a consumer is untouched — those reach beyond the working tree the Grep tool sees. If you genuinely need git grep, run it yourself with !<command>.")
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `pgrep_self_match`'s pattern operand: the ERE
// `-f` scans argv for, read whole across its blanks, with its case flag; `None` where the options
// cannot be walked without guessing or `-x`/`-v` void the self-match argument.
fn pgrep_pattern(seg: &str) -> Option<(bool, String)> {
    let (mut pat, mut have_f, mut skip, mut icase) = (String::new(), false, false, false);
    let mut open: Option<char> = None;
    for tok in words(seg).into_iter().skip(1) {
        if let Some(q) = open {
            pat.push(' ');
            pat.push_str(tok);
            if tok.ends_with(q) {
                open = None;
            }
            continue;
        }
        if skip {
            skip = false;
            continue;
        }
        match tok {
            "--full" => have_f = true,
            "--exact" | "--inverse" => return None,
            "--ignore-case" => icase = true,
            "--count" | "--newest" | "--oldest" | "--list-name" | "--list-full" | "--lightweight" => {}
            "--signal" | "--parent" | "--pgroup" | "--group" | "--session" | "--terminal" | "--euid" | "--uid"
            | "--delimiter" => skip = true,
            t if t.starts_with("--") => return None,
            t if t.len() >= 2 && t.starts_with('-') && t.as_bytes()[1].is_ascii_digit() => {}
            "-d" | "-P" | "-g" | "-G" | "-s" | "-t" | "-u" | "-U" => skip = true,
            t if t.starts_with('-') => {
                for ch in t[1..].chars() {
                    match ch {
                        'f' => have_f = true,
                        'i' => icase = true,
                        'v' | 'x' => return None,
                        'a' | 'c' | 'l' | 'n' | 'o' | 'w' => {}
                        _ => return None,
                    }
                }
            }
            t => {
                if !pat.is_empty() {
                    return None;
                }
                pat = t.to_string();
                for q in ['\'', '"'] {
                    if t.starts_with(q) && !(t.len() > 1 && t.ends_with(q)) {
                        open = Some(q);
                    }
                }
            }
        }
    }
    if !have_f || pat.is_empty() || open.is_some() {
        return None;
    }
    for q in ['\'', '"'] {
        if pat.len() >= 2 && pat.starts_with(q) && pat.ends_with(q) {
            pat = pat[1..pat.len() - 1].to_string();
            break;
        }
    }
    if pat.is_empty() || pat.contains(['\'', '"']) {
        return None;
    }
    Some((icase, pat))
}

pub fn pgrep_self_match(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    if has_expansion(raw) {
        return Ok(None);
    }
    for seg in ctx.segments(raw).iter().flat_map(|s| s.split('&').map(String::from).collect::<Vec<_>>()) {
        let cmdseg = command_word(&seg);
        if !matches!(head_word(&cmdseg), "pgrep" | "pkill") {
            continue;
        }
        let Some((icase, pat)) = pgrep_pattern(&cmdseg) else { continue };
        let (re_src, hay) = if icase { (pat.to_ascii_lowercase(), raw.to_ascii_lowercase()) } else { (pat.clone(), raw.to_string()) };
        let Ok(re) = Ere::compile(&re_src) else { continue };
        if !hay.split('\n').any(|l| re.is_match(l)) {
            continue;
        }
        return block(format!("don't query process liveness with 'pgrep -f {}' — '-f' matches full argv, and the harness runs this command through a wrapper whose argv carries the whole command text, pattern included, so the pattern always finds at least that wrapper: a wait loop never exits, a one-shot query always answers running, and a pkill signals its own wrapper. Nothing reds: the work finishes and the only symptom is the foreground cap absorbing an unbounded wait. Wait on the work's own artifact instead (an evidence file, a lock, an exit marker the work itself writes), or — where liveness genuinely is the condition — 'kill -0 <pid>' against a PID you recorded, whoever started that producer: a child you backgrounded yourself counts, and its PID is the one you recorded at launch, one line 'pid=<n> run=<key>' in a '<key>.run' file under your gitignored scratch dir. A pattern is a guess about a process table that includes the guesser; a PID is an identity. If you genuinely need pgrep, run it yourself with !<command>.", pat));
    }
    Ok(None)
}

pub fn bare_sleep(ctx: &Ctx) -> Decided {
    if has_expansion(ctx.raw(ctx.cmd())?) {
        return Ok(None);
    }
    let s = ctx.view(ctx.cmd(), SqDqHd)?;
    let Some(span) = loop_span(&s) else { return Ok(None) };
    if !span.iter().any(|(depth, cmdpos, tok)| tok == "sleep" && *cmdpos && *depth == 0) {
        return Ok(None);
    }
    block("don't wait by sleeping in the foreground — a wait must end when its condition goes true, not when a duration expires, and a foreground sleep spends a full-price turn doing nothing. Background a command that *exits* on the condition ('run_in_background' wrapping 'until <cond>; do sleep N; done') and take its completion notification: it fires the moment the condition holds and then ends. A dispatched agent is awaited by its own completion notification and never by a path on disk. The harness's event-stream form stays armed to its deadline after its event fires when the command it was armed with is unbounded, so it is the second choice for a single completion. Mind the polarity: 'until' takes a done predicate ('until [ -f marker ]'), while a PID's liveness is a still-running one and takes 'while' ('while kill -0 <pid> 2>/dev/null; do sleep N; done') — inverted, the loop exits at once with the producer still running. Spell that PID as the literal number you read out of the .run record: a '\"$var\"' expansion in the condition is refused by the expansion rule before this steer can be followed, and the literal form is the one the bounded-wait grant recognizes. A sleep inside a condition loop is untouched — that is the sanctioned form. If you genuinely need the settle, run it yourself with !<command>.")
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §The rule roster — the sed check clears a read and withholds every write
    // or execute spelling, and every command letter it does not model
    #[test]
    fn the_sed_check_withholds_every_write_and_execute_spelling() {
        for ok in ["5,10p", "s/a/b/g", "/x/,/y/p", "$!N;P;D", "1~2{p;d}", "y/ab/cd/", "s|a|b|2", "0,/re/I p", "r notes.md"] {
            assert!(sed_script_clear(ok), "{} should clear", ok);
        }
        for no in ["w out", "s/a/b/w out", "s/a/b/e", "1e ls", "W out", "v", "s/a/b", "{p", "L"] {
            assert!(!sed_script_clear(no), "{} should withhold", no);
        }
    }

    // spec: guard-kit/SPEC.md §The rule roster — the awk check skips strings, regexes and comments, and
    // withholds `system`, a pipe, `@`, a print redirect and an unterminated literal
    #[test]
    fn the_awk_check_withholds_every_write_and_execute_spelling() {
        for ok in [
            "{s+=$1} END {print s}",
            "{print \"a > b | c\"}",
            "/^## A/,/^## B/",
            "NR==3||NR==13",
            "{print ($1 > 2)}",
            "$1 > 5 {print}",
            "{x = y / 2; print x} # > out",
            "{getline line < \"f\"; print line}",
        ] {
            assert!(awk_program_clear(ok), "{} should clear", ok);
        }
        for no in [
            "BEGIN {system(\"ls\")}",
            "{print > \"out\"}",
            "{printf(\"%s\", $1) >> \"out\"}",
            "{print | \"sort\"}",
            "{\"date\" | getline d}",
            "{print |& \"cmd\"}",
            "@include \"x\"",
            "{print \"open",
            "/open",
        ] {
            assert!(!awk_program_clear(no), "{} should withhold", no);
        }
    }
}
