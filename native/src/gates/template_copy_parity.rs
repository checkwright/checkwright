// spec: gate-sdk/SPEC.md §check-template-copy-parity — a kit template and its vendored consumer
// copy agree on their declared contract surface, with copy-side additions declared
use crate::ere::Ere;
use crate::proc;
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-template-copy-parity: {}", e);
            2
        }
    }
}

fn read(path: &str) -> Option<String> {
    std::fs::read(Path::new(path))
        .ok()
        .map(|b| String::from_utf8_lossy(&b).into_owned())
}

fn lstrip(s: &str) -> &str {
    s.trim_start_matches(|c: char| c.is_ascii_whitespace())
}

// spec: gate-sdk/SPEC.md §check-template-copy-parity — assertion A reads the resolved
// `<file> §<section>` target, never the trailing prose: a pair may gloss one target two ways
fn spec_target(text: &str) -> String {
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("# spec:") {
            let mut t = lstrip(rest);
            if let Some(i) = find_em_dash_start(t) {
                t = &t[..i];
            }
            return t.trim_end_matches(|c: char| c.is_ascii_whitespace()).to_string();
        }
    }
    String::new()
}

// spec: gate-sdk/SPEC.md §check-template-copy-parity — the cut is at the whitespace run in front
// of the first em dash, so the target keeps no trailing space of its own
fn find_em_dash_start(s: &str) -> Option<usize> {
    let at = s.find('—')?;
    let b = s.as_bytes();
    let mut i = at;
    while i > 0 && b[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    Some(i)
}

struct Grammar {
    func: Ere,
    lib_outer: Ere,
    lib_inner: Ere,
    knob: Ere,
    case_open: Ere,
    esac: Ere,
    ident: Ere,
}

impl Grammar {
    fn new() -> Result<Grammar, String> {
        let c = |p: &str| Ere::compile(p).map_err(|e| format!("{} failed to compile: {}", p, e));
        Ok(Grammar {
            func: c("^[[:space:]]*(function[[:space:]]+)?([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*\\(\\)[[:space:]]*\\{?.*$")?,
            lib_outer: c("(^|[;&|)]|\\$\\(|&&|\\|\\|)[[:space:]]*[a-z_][a-z0-9]*_[a-z0-9_]+([[:space:]]|\\))")?,
            lib_inner: c("[a-z_][a-z0-9]*_[a-z0-9_]+")?,
            knob: c("\\$\\{[A-Z][A-Z0-9]*_[A-Z0-9_]+:[-=]")?,
            case_open: c("(^|[[:space:];])case[[:space:]].*[[:space:]]in[[:space:]]*$")?,
            esac: c("^[[:space:]]*esac([[:space:]]|$)")?,
            ident: c("[A-Za-z_][A-Za-z0-9_]*")?,
        })
    }
}

// spec: gate-sdk/SPEC.md §check-template-copy-parity — `grep -o`'s per-line scan: every
// non-overlapping leftmost-longest match, each resumed at the end of the last
fn matches_in<'a>(re: &Ere, line: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos <= line.len() {
        let Some(rest) = line.get(pos..) else {
            pos += 1;
            continue;
        };
        match re.find(rest) {
            Some((s, e)) => {
                out.push(&rest[s..e]);
                pos += if e > s { e } else { s + 1 };
            }
            None => break,
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §check-template-copy-parity — the declared surface: four classes of
// *declaration*, never content; the `case` arm's pattern is discarded rather than captured,
// which is a privacy boundary and not a parsing convenience
fn declared_surface(g: &Grammar, text: &str) -> (Vec<String>, bool) {
    let mut out: Vec<String> = Vec::new();
    let mut knob_hit = false;

    for line in text.lines() {
        if g.func.is_match(line) {
            if let Some(name) = func_name(g, line) {
                out.push(format!("func:{}", name));
            }
        }
    }

    let mut in_case = false;
    let mut expect = false;
    let mut want = false;
    for line in text.lines() {
        if g.case_open.is_match(line) {
            in_case = true;
            expect = true;
            continue;
        }
        if g.esac.is_match(line) {
            in_case = false;
            expect = false;
            want = false;
            continue;
        }
        if !in_case {
            continue;
        }
        let mut rest = line;
        if expect {
            if let Some(i) = rest.find(')') {
                rest = &rest[i + 1..];
                expect = false;
                want = true;
            }
        }
        if want {
            if let Some((s, e)) = g.ident.find(rest) {
                out.push(format!("case:{}", &rest[s..e]));
                want = false;
            }
        }
        if line.contains(";;") {
            expect = true;
            want = false;
        }
    }

    for line in text.lines() {
        for frag in matches_in(&g.lib_outer, line) {
            for id in matches_in(&g.lib_inner, frag) {
                out.push(format!("lib:{}", id));
            }
        }
    }

    for line in text.lines() {
        for m in matches_in(&g.knob, line) {
            knob_hit = true;
            let name = m
                .trim_start_matches("${")
                .trim_end_matches(['-', '='])
                .trim_end_matches(':');
            out.push(format!("knob:{}", name));
        }
    }

    out.sort();
    out.dedup();
    (out, knob_hit)
}

// spec: gate-sdk/SPEC.md §check-template-copy-parity — the captured group is the name, so the
// optional `function` keyword never becomes the declaration; POSIX prefers the longer group, so
// the keyword-stripped reading is tried first
fn func_name(g: &Grammar, line: &str) -> Option<String> {
    let t = lstrip(line);
    let mut cands: Vec<&str> = Vec::new();
    if let Some(r) = t.strip_prefix("function") {
        if r.starts_with([' ', '\t']) {
            cands.push(lstrip(r));
        }
    }
    cands.push(t);
    for c in cands {
        let Some((s, e)) = g.ident.find(c) else {
            continue;
        };
        if s != 0 {
            continue;
        }
        if lstrip(&c[e..]).starts_with("()") {
            return Some(c[s..e].to_string());
        }
    }
    None
}

fn divergence_reasons(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let t = lstrip(line);
        let Some(rest) = t.strip_prefix('#') else {
            continue;
        };
        let rest = lstrip(rest);
        let Some(r) = rest.strip_prefix("copy-divergence:") else {
            continue;
        };
        out.push_str(lstrip(r));
        out.push('\n');
    }
    out
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = match args.first().filter(|a| !a.is_empty()) {
        Some(r) => r.clone(),
        None => {
            // spec: gate-sdk/SPEC.md §check-template-copy-parity — the root defaults to the git
            // toplevel, a derivation no injected fixture case can reach, so a sibling harness
            // drives it against the live tree instead
            let out = proc::run("git", &["rev-parse", "--show-toplevel"])
                .ok()
                .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
                .filter(|s| !s.is_empty());
            match out {
                Some(r) => r,
                None => return Err("not a git repository and no root given".to_string()),
            }
        }
    };
    if !Path::new(&root).is_dir() {
        return Err(format!("root not a directory: {}", root));
    }
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let g = Grammar::new()?;

    let mut pairs = 0usize;
    let mut findings: Vec<String> = Vec::new();

    for tpl in walk::glob_entries(&format!("{}/*/templates/*.sh", root)) {
        let name = tpl.rsplit('/').next().unwrap_or(&tpl).to_string();
        if name.ends_with("-config.sh") {
            continue;
        }
        let copy = format!("{}/{}/{}", root, gates_dir, name);
        if !Path::new(&copy).is_file() {
            continue;
        }
        pairs += 1;
        let rel_copy = copy
            .strip_prefix(&format!("{}/", root))
            .unwrap_or(&copy)
            .to_string();

        let tpl_text = read(&tpl).unwrap_or_default();
        let copy_text = read(&copy).unwrap_or_default();

        let t_target = spec_target(&tpl_text);
        let c_target = spec_target(&copy_text);
        if t_target != c_target {
            findings.push(format!("{}: spec: target differs from its template", rel_copy));
            findings.push(format!(
                "    template: {}",
                if t_target.is_empty() { "<none>" } else { &t_target }
            ));
            findings.push(format!(
                "    copy:     {}",
                if c_target.is_empty() { "<none>" } else { &c_target }
            ));
        }

        // spec: gate-sdk/SPEC.md §check-template-copy-parity — the fail-closed refusal a file
        // carrying no knob-with-default idiom triggers is reproduced rather than repaired: it is a
        // verdict the shell form makes, and a port that stopped making it would change the verdict
        let (t_surface, t_knob) = declared_surface(&g, &tpl_text);
        if !t_knob {
            return Err(
                "declared_surface exited 1 — the check could not run; treating as failure (not clean)"
                    .to_string(),
            );
        }
        let (c_surface, c_knob) = declared_surface(&g, &copy_text);
        if !c_knob {
            return Err(
                "declared_surface exited 1 — the check could not run; treating as failure (not clean)"
                    .to_string(),
            );
        }

        let missing: Vec<&String> = t_surface.iter().filter(|t| !c_surface.contains(t)).collect();
        if !missing.is_empty() {
            findings.push(format!(
                "{}: drops surface the template declares (a template-side change never propagated, or a copy-side removal):",
                rel_copy
            ));
            for tok in &missing {
                findings.push(format!("    {}", tok));
            }
        }

        let extra: Vec<&String> = c_surface.iter().filter(|t| !t_surface.contains(t)).collect();
        if !extra.is_empty() {
            let reasons = divergence_reasons(&copy_text);
            let undeclared: Vec<&&String> = extra
                .iter()
                .filter(|tok| {
                    let bare = match tok.find(':') {
                        Some(i) => &tok[i + 1..],
                        None => tok.as_str(),
                    };
                    !reasons.lines().any(|r| r.contains(bare))
                })
                .collect();
            if !undeclared.is_empty() {
                findings.push(format!(
                    "{}: adds surface no '# copy-divergence:' marker names:",
                    rel_copy
                ));
                for tok in &undeclared {
                    findings.push(format!("    {}", tok));
                }
            }
        }
    }

    if !findings.is_empty() {
        println!("check-template-copy-parity: template <-> consumer-copy contract surface diverged:");
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: assertion A — point both copies' 'spec:' line at the same '<file> §<section>'");
        println!("        (trailing prose may differ). Assertion B — propagate the template's");
        println!("        declaration to the copy, or retire it from the template. Assertion C —");
        println!("        add a '# copy-divergence: <reason>' line to the copy whose reason names");
        println!("        the added token and says why the copy needs it.");
        return Ok(1);
    }
    println!(
        "TEMPLATE-COPY-PARITY: clean ({} template<->copy pair(s) agree on spec: target, template-declared surface present, copy additions declared)",
        pairs
    );
    Ok(0)
}
