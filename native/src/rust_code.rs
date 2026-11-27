// spec: gate-sdk/SPEC.md §The Rust code reader — a Rust file's code view, every comment and
// literal blanked, and each call in it with its callee path and depth-zero arguments
use crate::literals::{self, Lang};

// spec: gate-sdk/SPEC.md §The Rust code reader — an argument's source text, trimmed, and its span
#[derive(Debug)]
pub struct Arg {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Call {
    pub path: String,
    pub start: usize,
    pub line: usize,
    pub args: Vec<Arg>,
}

impl Call {
    pub fn method(&self) -> bool {
        self.path.starts_with('.')
    }

    pub fn segments(&self) -> Vec<&str> {
        self.path.trim_start_matches('.').trim_start_matches("::").split("::").collect()
    }

    pub fn name(&self) -> &str {
        self.segments().last().copied().unwrap_or("")
    }
}

pub struct View {
    pub code: String,
    pub calls: Vec<Call>,
    starts: Vec<usize>,
}

impl View {
    pub fn line_of(&self, at: usize) -> usize {
        self.starts.partition_point(|&s| s <= at)
    }
}

fn ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn back_over_space(code: &[u8], mut k: usize) -> usize {
    while k > 0 && code[k - 1].is_ascii_whitespace() {
        k -= 1;
    }
    k
}

// spec: gate-sdk/SPEC.md §The Rust code reader — a `<…>` group read backwards from its `>`, the `>`
// of a `->` closing nothing; None where no `<` balances it
fn back_over_angles(code: &[u8], close: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut k = close;
    while k > 0 {
        k -= 1;
        match code[k] {
            b'>' if k > 0 && code[k - 1] == b'-' => {}
            b'>' => depth += 1,
            b'<' if depth == 0 => return None,
            b'<' => {
                depth -= 1;
                if depth == 0 {
                    return Some(k);
                }
            }
            b';' | b'{' | b'}' => return None,
            _ => {}
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §The Rust code reader — a path with the whitespace around each `::` dropped
fn tight(path: &str) -> String {
    path.split("::").map(str::trim).collect::<Vec<_>>().join("::")
}

// spec: gate-sdk/SPEC.md §The Rust code reader — a keyword ending at `end` opens no path segment, so
// `return ::f()` reads `::f`
fn keyword_before(code: &[u8], end: usize) -> bool {
    let mut w = end;
    while w > 0 && ident(code[w - 1]) {
        w -= 1;
    }
    matches!(
        &code[w..end],
        b"as" | b"break" | b"const" | b"else" | b"for" | b"if" | b"impl" | b"in" | b"let" | b"match" | b"move"
            | b"mut" | b"ref" | b"return" | b"static" | b"unsafe" | b"use" | b"where" | b"while" | b"dyn" | b"yield"
    )
}

// spec: gate-sdk/SPEC.md §The Rust code reader — the callee a `(` closes over, read backwards: a
// `::` path whose head may be a qualified `<…>`, a turbofish dropped, `.` marking a method. A macro,
// a `fn` definition, a `fn` pointer type and a bare group are no call
fn callee(code: &[u8], open: usize) -> Option<(usize, String)> {
    let mut end = back_over_space(code, open);
    if end > 0 && code[end - 1] == b'>' {
        let sep = back_over_space(code, back_over_angles(code, end)?);
        if sep < 2 || &code[sep - 2..sep] != b"::" {
            return None;
        }
        end = back_over_space(code, sep - 2);
    }
    let mut k = end;
    loop {
        let seg_end = k;
        while k > 0 && ident(code[k - 1]) {
            k -= 1;
        }
        if k == seg_end {
            return None;
        }
        let sep = back_over_space(code, k);
        if sep >= 2 && &code[sep - 2..sep] == b"::" {
            let head = back_over_space(code, sep - 2);
            if head > 0 && code[head - 1] == b'>' {
                k = back_over_angles(code, head)?;
                break;
            }
            if head == 0 || !ident(code[head - 1]) || keyword_before(code, head) {
                k = sep - 2;
                break;
            }
            k = head;
            continue;
        }
        break;
    }
    let start = k;
    let path = tight(std::str::from_utf8(&code[start..end]).ok()?);
    let path = path.as_str();
    let before = back_over_space(code, start);
    if before > 0 && code[before - 1] == b'.' && !(before > 1 && code[before - 2] == b'.') {
        return Some((before - 1, format!(".{}", path)));
    }
    let mut w = before;
    while w > 0 && ident(code[w - 1]) {
        w -= 1;
    }
    if &code[w..before] == b"fn" || path == "fn" {
        return None;
    }
    Some((start, path.to_string()))
}

struct Frame {
    open: u8,
    at: usize,
    commas: Vec<usize>,
}

fn closes(open: u8, close: u8) -> bool {
    matches!((open, close), (b'(', b')') | (b'[', b']') | (b'{', b'}'))
}

// spec: gate-sdk/SPEC.md §The Rust code reader — the view, or the line of the first unparseable
// span: an unterminated literal or block comment, or a bracket nothing balances
pub fn read(text: &str) -> Result<View, usize> {
    let lexed = literals::lex(text, Lang::Rust);
    if let Some(line) = lexed.unterminated {
        return Err(line);
    }
    let mut bytes = text.as_bytes().to_vec();
    for s in &lexed.spans {
        bytes[s.start..s.end].iter_mut().filter(|b| **b != b'\n').for_each(|b| *b = b' ');
    }
    let starts: Vec<usize> =
        std::iter::once(0).chain(bytes.iter().enumerate().filter(|(_, b)| **b == b'\n').map(|(i, _)| i + 1)).collect();
    let line_of = |at: usize| starts.partition_point(|&s| s <= at);

    let mut stack: Vec<Frame> = Vec::new();
    let mut calls = Vec::new();
    for (i, &c) in bytes.iter().enumerate() {
        match c {
            b'(' | b'[' | b'{' => stack.push(Frame { open: c, at: i, commas: Vec::new() }),
            b',' => {
                if let Some(top) = stack.last_mut() {
                    top.commas.push(i);
                }
            }
            b')' | b']' | b'}' => {
                let frame = match stack.pop() {
                    Some(f) if closes(f.open, c) => f,
                    _ => return Err(line_of(i)),
                };
                if c != b')' {
                    continue;
                }
                let Some((start, path)) = callee(&bytes, frame.at) else { continue };
                let bounds: Vec<usize> =
                    std::iter::once(frame.at).chain(frame.commas.iter().copied()).chain(std::iter::once(i)).collect();
                let mut args: Vec<Arg> = bounds
                    .windows(2)
                    .map(|w| {
                        let (a, b) = (w[0] + 1, w[1]);
                        Arg { text: text[a..b].trim().to_string(), start: a, end: b }
                    })
                    .collect();
                if args.last().is_some_and(|a| a.text.is_empty()) && (args.len() == 1 || !frame.commas.is_empty()) {
                    args.pop();
                }
                calls.push(Call { line: line_of(start), path, start, args });
            }
            _ => {}
        }
    }
    if let Some(f) = stack.first() {
        return Err(line_of(f.at));
    }
    calls.sort_by_key(|c| c.start);
    let code = String::from_utf8_lossy(&bytes).into_owned();
    Ok(View { code, calls, starts })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(text: &str) -> Vec<String> {
        read(text)
            .expect("reads")
            .calls
            .iter()
            .map(|c| format!("{}@{}({})", c.path, c.line, c.args.iter().map(|a| a.text.as_str()).collect::<Vec<_>>().join("|")))
            .collect()
    }

    // spec: gate-sdk/SPEC.md §The Rust code reader — comments and every literal form blank to
    // spaces, line breaks and integer literals kept, so a view offset is a file offset
    #[test]
    fn the_view_blanks_comments_and_literals() {
        let text = "let a = \"x(\"; // y(\nlet b = r#\"z\"(\"#; /* c /* d */ ( */ 'q' 0o755 b\"(\" '\\''\nfn f<'a>(x: &'a str) {}\n";
        let v = read(text).expect("reads");
        assert_eq!(v.code.len(), text.len());
        assert_eq!(v.code.lines().count(), text.lines().count());
        assert!(!v.code.contains('"') && !v.code.contains("//") && !v.code.contains("/*"));
        assert!(v.code.contains("0o755") && v.code.contains("'a"));
        assert_eq!(v.line_of(text.find("fn").expect("fn")), 3);
    }

    // spec: gate-sdk/SPEC.md §The Rust code reader — each call's path as written and its
    // depth-zero arguments; a macro, a definition and a bare group are none
    #[test]
    fn a_call_carries_its_path_and_arguments() {
        let text = "fs::set_permissions(p, Permissions::from_mode(0o7_55))?;\nf.set_mode(m,);\n<X as Y>::mode(&mut o, 0o600);\nlet v = a.iter().collect::<Vec<_>>();\nprintln!(\"{}\", g(1, (2, 3)));\nfn mode(x: u32) {}\nlet t = (1, 2);\n";
        assert_eq!(
            calls(text),
            [
                "fs::set_permissions@1(p|Permissions::from_mode(0o7_55))",
                "Permissions::from_mode@1(0o7_55)",
                ".set_mode@2(m)",
                "<X as Y>::mode@3(&mut o|0o600)",
                ".iter@4()",
                ".collect@4()",
                "g@5(1|(2, 3))",
            ]
        );
        let v = read("std::fs::copy(a, b);\nx\n  .mode(0o755);\n").expect("reads");
        assert_eq!(v.calls[0].segments(), ["std", "fs", "copy"]);
        assert!(v.calls[1].method() && v.calls[1].name() == "mode" && v.calls[1].line == 3);
        let v = read("std::fs ::copy(a, b);\nstd :: fs\n  :: copy(a);\n:: libc :: chmod(p, m);\n<X as Y> :: mode(o, 0o600);\nreturn ::std::fs::copy(a, b);\n").expect("reads");
        let paths: Vec<_> = v.calls.iter().map(|c| (c.path.as_str(), c.line)).collect();
        assert_eq!(paths, [("std::fs::copy", 1), ("std::fs::copy", 2), ("::libc::chmod", 4), ("<X as Y>::mode", 5), ("::std::fs::copy", 6)]);
        let v = read("std::fs::copy:: <&str, &str>(a, b);\nstd::fs::copy ::<&str, &str>(a, b);\nstd::fs::copy :: <&str>\n  (a);\n").expect("reads");
        let paths: Vec<_> = v.calls.iter().map(|c| (c.path.as_str(), c.line)).collect();
        assert_eq!(paths, [("std::fs::copy", 1), ("std::fs::copy", 2), ("std::fs::copy", 3)]);
        let v = read("f::<fn() -> ()>(a);\nstd::fs::copy::<fn(u8) -> Vec<u8>, &str>(a, b);\n").expect("reads");
        let paths: Vec<_> = v.calls.iter().map(|c| (c.path.as_str(), c.line)).collect();
        assert_eq!(paths, [("f", 1), ("std::fs::copy", 2)]);
    }

    // spec: gate-sdk/SPEC.md §The Rust code reader — an unterminated literal or block comment, or
    // an unbalanced bracket, names its line
    #[test]
    fn an_unparseable_span_names_its_line() {
        assert_eq!(read("let a = 1;\nlet s = r#\"open\";\n").err(), Some(2));
        assert_eq!(read("/* a\n").err(), Some(1));
        assert_eq!(read("f(a,\n  b;\n").err(), Some(1));
        assert_eq!(read("a\n)\n").err(), Some(2));
        assert_eq!(read("f(a]\n").err(), Some(1));
    }
}
