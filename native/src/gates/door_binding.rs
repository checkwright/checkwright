// spec: guard-kit/SPEC.md §check-door-binding — no kit-shipped surface outside the fail-open set
// names a front-end stub as a command to run, and no kit template names a path to the gate binary
use crate::emit::FAIL_OPEN_ARMS;
use crate::gates::smoke_entry_guard::{kit_abs, kit_name, scan_root};
use crate::walk;
use std::path::Path;

const NAME: &str = "check-door-binding";

// spec: guard-kit/SPEC.md §check-door-binding — the two front-end spellings, by basename: a door is
// named by either half, and the twin is reached under a `pwsh -File` word rather than `bash`
const DOORS: &[&str] = &["run-gates.sh", "run-gates.ps1"];

// spec: guard-kit/SPEC.md §check-door-binding — the corpus: each kit root's README and the three
// shipped directories, never `gate-tests/` or `smoke/`, which are fixture and harness corpora a
// sweep of adopter surfaces does not reach
const DIRS: &[&str] = &["templates", "lib", "bin"];

// spec: guard-kit/SPEC.md §check-door-binding — assertion C's corpus knob, consumer-configured and
// empty by default, so an upgrade moves no consumer's verdict
const ROOTS: &str = "GUARD_KIT_DOOR_ROOTS";

// spec: guard-kit/SPEC.md §check-door-binding — assertion C's site declaration; the reason is
// mandatory on the `comment-tier-exempt:` convention and an empty one is malformed, not exempting
const DECL: &str = "door-contributor:";

// spec: guard-kit/SPEC.md §check-door-binding — a path token's characters, so an occurrence found by
// basename expands to the whole spelling around it before either question is asked of it
fn is_path_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '/' | '-' | '\\')
}

// spec: guard-kit/SPEC.md §check-door-binding — one delimiter a markup or JSON context closes with,
// stripped so the discriminator reads the command and not the quoting around it
fn strip_close(s: &str) -> &str {
    s.strip_prefix(['`', '\'', '"', ')']).unwrap_or(s)
}

// spec: guard-kit/SPEC.md §check-door-binding — the token holding an occurrence, as (start, end)
fn token_span(line: &str, at: usize, len: usize) -> (usize, usize) {
    let b = line.as_bytes();
    let mut s = at;
    while s > 0 && is_path_char(b[s - 1] as char) {
        s -= 1;
    }
    let mut e = at + len;
    while e < b.len() && is_path_char(b[e] as char) {
        e += 1;
    }
    (s, e)
}

// spec: guard-kit/SPEC.md §check-door-binding — the door/file discriminator: a mention is a door
// when it is spawned, carried by an interpreter word, or when an arm follows it
fn is_door(line: &str, span: (usize, usize)) -> bool {
    let (s, e) = span;
    // spec: guard-kit/SPEC.md §check-door-binding — a `Bash(…)` permission pattern is not an
    // invocation, and the wrapper shelters only what it encloses
    if let Some(open) = line[..s].rfind("Bash(") {
        if !line[open + "Bash(".len()..s].contains(')') {
            return false;
        }
    }
    let before = line[..s].trim_end_matches(['`', '\'', '"', '(']);
    // spec: guard-kit/SPEC.md §check-door-binding — the interpreter word, with the markup or JSON
    // quoting that may open in front of it stripped: `` `bash <door>` `` carries the same spawn a
    // bare `bash <door>` does, and reading the delimiter as part of the word would miss it
    let lead = before
        .split(|c: char| c.is_ascii_whitespace() || matches!(c, '`' | '\'' | '"' | '('))
        .rfind(|w| !w.is_empty())
        .unwrap_or("");
    if matches!(lead, "bash" | "sh" | "-File") {
        return true;
    }
    let after = strip_close(&line[e..]);
    let mut rest = after.trim_start_matches([' ', '\t']);
    if rest.len() == after.len() {
        return false;
    }
    rest = strip_close(rest);
    rest.starts_with('-')
}

// spec: guard-kit/SPEC.md §check-door-binding — the exemption keys on the ARM, not on the file, and
// reads the set from the crate's declaration rather than re-listing it
fn names_fail_open_arm(line: &str, end: usize) -> bool {
    line[end..]
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .any(|t| FAIL_OPEN_ARMS.contains(&t))
}

// spec: guard-kit/SPEC.md §check-door-binding — a declaration's reason, or `None` on a line that
// carries no declaration at all. An unterminated comment reads to end of line rather than matching
// nothing, so a truncated `-->` cannot turn a malformed reason into a silent non-declaration.
fn decl_reason(line: &str) -> Option<&str> {
    let at = line.find(DECL)?;
    let rest = &line[at + DECL.len()..];
    let end = rest.find("-->").unwrap_or(rest.len());
    Some(rest[..end].trim_matches([' ', '\t']))
}

// spec: guard-kit/SPEC.md §check-door-binding — a declaration standing alone on its line, which is
// what opens a span; one sharing its line with anything else is site-scoped
fn decl_alone(line: &str) -> bool {
    let t = line.trim_matches([' ', '\t']);
    t.starts_with("<!--") && t.ends_with("-->") && decl_reason(t).is_some()
}

fn is_fence(line: &str) -> bool {
    let t = line.trim_start_matches([' ', '\t']);
    t.starts_with("```") || t.starts_with("~~~")
}

// spec: guard-kit/SPEC.md §check-door-binding — the two declaration scopes resolved over one file:
// whole-file when a standalone declaration stands before the first `#` heading, and the fenced
// block a standalone declaration immediately precedes
struct Scopes {
    file: bool,
    spans: Vec<(usize, usize)>,
    declared: Vec<bool>,
    malformed: Vec<usize>,
}

fn scopes(lines: &[&str]) -> Scopes {
    let heading = lines.iter().position(|l| l.starts_with('#'));
    let mut s = Scopes {
        file: false,
        spans: Vec::new(),
        declared: vec![false; lines.len()],
        malformed: Vec::new(),
    };
    for (i, line) in lines.iter().enumerate() {
        let Some(reason) = decl_reason(line) else { continue };
        if reason.is_empty() {
            s.malformed.push(i);
            continue;
        }
        s.declared[i] = true;
        if !decl_alone(line) {
            continue;
        }
        if heading.map_or(true, |h| i < h) {
            s.file = true;
            continue;
        }
        let Some(off) = lines[i + 1..]
            .iter()
            .position(|n| !n.trim_matches([' ', '\t']).is_empty())
        else {
            continue;
        };
        let open = i + 1 + off;
        if !is_fence(lines[open]) {
            continue;
        }
        let close = lines[open + 1..]
            .iter()
            .position(|n| is_fence(n))
            .map(|k| open + 1 + k)
            .unwrap_or(lines.len().saturating_sub(1));
        s.spans.push((open, close));
    }
    s
}

impl Scopes {
    fn exempts(&self, i: usize) -> bool {
        self.file
            || self.spans.iter().any(|(s, e)| i >= *s && i <= *e)
            || self.declared[i]
            || (i > 0 && self.declared[i - 1])
    }
}

// spec: guard-kit/SPEC.md §check-door-binding — every door occurrence on one line, span-expanded,
// with the corpus-wide fail-open arm exemption already applied
fn doors_on(line: &str, base: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if DOORS.contains(&base) {
        return out;
    }
    for needle in DOORS {
        for at in occurrences(line, needle) {
            let span = token_span(line, at, needle.len());
            if is_door(line, span) && !names_fail_open_arm(line, span.1) {
                out.push(span);
            }
        }
    }
    out
}

// spec: guard-kit/SPEC.md §check-door-binding — a configured entry's tracked members, the corpus
// being the tracked tree: a filesystem walk would reach a local build artifact, which is no
// governed surface
fn tracked_under(root: &str, entry: &str) -> Result<Vec<String>, String> {
    let listed = crate::proc::run(&crate::programs::GIT, &["-C", root, "ls-files", "--", entry])?;
    let text = listed
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .unwrap_or_default();
    Ok(text.lines().map(str::to_string).collect())
}

// spec: guard-kit/SPEC.md §check-door-binding — assertion C's corpus: each configured entry as
// (shown, path), a directory yielding its tracked members under A's own prune so a page added
// under it is swept unedited, and anything else the misconfiguration answer rather than a clean
fn configured(root: &str) -> Result<Vec<(String, String)>, String> {
    let mut prune = walk::prune_dirs()?;
    prune.push("gate-tests".to_string());
    prune.push("smoke".to_string());
    let base = root.trim_end_matches('/');
    let mut out: Vec<(String, String)> = Vec::new();
    for entry in walk::knob_array(ROOTS)?.iter().filter(|e| !e.is_empty()) {
        let e = entry.trim_end_matches('/');
        let mut members = tracked_under(base, e)?;
        members.sort();
        if Path::new(&format!("{}/{}", base, e)).is_dir() {
            for m in members {
                if walk::path_pruned(&m, &prune) {
                    continue;
                }
                out.push((m.clone(), format!("{}/{}", base, m)));
            }
        } else if members.len() == 1 && members[0] == e {
            out.push((e.to_string(), format!("{}/{}", base, e)));
        } else {
            return Err(format!(
                "{} entry '{}' resolves to neither a tracked file nor a directory",
                ROOTS, e
            ));
        }
    }
    Ok(out)
}

// spec: guard-kit/SPEC.md §check-door-binding — one walk from the root, the corpus a filter over it,
// so the walked-root set is bounded at one whatever the kit roster holds; the prune composes the
// configured set with this gate's own
fn walk_tree(root: &str) -> Result<Vec<String>, String> {
    let configured = walk::prune_dirs()?;
    let prune = |n: &str| n == "gate-tests" || n == "smoke" || configured.iter().any(|d| d == n);
    Ok(walk::find_with_prune(Path::new(root), &prune)?
        .iter()
        .filter_map(|p| p.to_str().map(String::from))
        .collect())
}

// spec: guard-kit/SPEC.md §check-door-binding — the corpus predicate over one kit root: its README,
// and anything under the three shipped directories
fn in_corpus(rel: &str) -> bool {
    rel == "README.md" || DIRS.iter().any(|d| walk::under(d, rel))
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|_| format!("unreadable corpus file: {}", path))
}

// spec: guard-kit/SPEC.md §check-door-binding — every occurrence of a needle in a line, by byte offset
fn occurrences(line: &str, needle: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(i) = line[from..].find(needle) {
        out.push(from + i);
        from += i + needle.len();
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {} — the check could not run; treating as failure (not clean)", NAME, e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = scan_root(args, NAME).ok_or_else(|| "no root".to_string())?;
    if !Path::new(&root).is_dir() {
        return Err(format!("root not found: {}", root));
    }
    let kit_roots = walk::kit_roots()?;
    if kit_roots.is_empty() {
        return Err("no kit roots enumerated".to_string());
    }

    // spec: guard-kit/SPEC.md §check-door-binding — the artifact's own basename, the crate's package
    // name, so the seam assertion and the binary it is about cannot drift apart
    let artifact = env!("CARGO_PKG_NAME");

    let mut doors: Vec<String> = Vec::new();
    let mut seam: Vec<String> = Vec::new();
    let mut undeclared: Vec<String> = Vec::new();
    let mut malformed: Vec<String> = Vec::new();
    let mut swept = 0usize;
    let mut templates = 0usize;
    let mut configured_swept = 0usize;

    let mut tree = walk_tree(&root)?;
    tree.sort();

    for raw in &kit_roots {
        let r = raw.trim_end_matches('/');
        let abs = kit_abs(&root, r);
        let kit = kit_name(r);
        let files: Vec<&String> = tree
            .iter()
            .filter(|f| walk::rel_under(&abs, f).is_some_and(in_corpus))
            .collect();
        for f in files {
            let rel = walk::rel_under(&abs, f).unwrap_or(f);
            let shown = format!("{}/{}", kit, rel);
            let base = Path::new(f).file_name().and_then(|n| n.to_str()).unwrap_or("");
            let text = read(f)?;
            swept += 1;
            let in_templates = rel.starts_with("templates/");
            if in_templates {
                templates += 1;
            }
            for (n, line) in text.lines().enumerate() {
                // spec: guard-kit/SPEC.md §check-door-binding — a stub names itself; the retired
                // spelling is retired as an ADOPTER-FACING door, and the file's own name is not one
                for _ in doors_on(line, base) {
                    doors.push(format!("{}:{}: {}", shown, n + 1, line.trim()));
                }
                if in_templates {
                    for at in occurrences(line, artifact) {
                        let (s, e) = token_span(line, at, artifact.len());
                        if line[s..e].contains('/') {
                            seam.push(format!("{}:{}: {}", shown, n + 1, &line[s..e]));
                        }
                    }
                }
            }
        }
    }

    // spec: guard-kit/SPEC.md §check-door-binding — assertion C over the configured extra corpus,
    // default-deny: every door reds unless its site declares itself contributor-facing
    for (shown, path) in configured(&root)? {
        let text = read(&path)?;
        let base = Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or("");
        let lines: Vec<&str> = text.lines().collect();
        let scope = scopes(&lines);
        configured_swept += 1;
        for i in &scope.malformed {
            malformed.push(format!("{}:{}: {}", shown, i + 1, lines[*i].trim()));
        }
        for (n, line) in lines.iter().enumerate() {
            if scope.exempts(n) {
                continue;
            }
            for _ in doors_on(line, base) {
                undeclared.push(format!("{}:{}: {}", shown, n + 1, line.trim()));
            }
        }
    }

    if !doors.is_empty() || !seam.is_empty() || !undeclared.is_empty() || !malformed.is_empty() {
        if !doors.is_empty() {
            println!("{}: kit-shipped surface(s) name a front-end stub as a command to run:", NAME);
            for d in &doors {
                println!("  {}", d);
            }
            println!("  help: name the gate binary through GATE_SDK_NATIVE_BIN and the arm as the arm,");
            println!("        with no 'bash' and no path literal. A mention that names the program as");
            println!("        its subject rather than invoking it is a file, not a door, and is left");
            println!("        alone; the arms {} keep the front end by the", FAIL_OPEN_ARMS.join(" and "));
            println!("        fail-open limit and are exempt wherever they appear.");
        }
        if !seam.is_empty() {
            if !doors.is_empty() {
                println!();
            }
            println!("{}: kit template(s) name a path to the gate binary:", NAME);
            for s in &seam {
                println!("  {}", s);
            }
            println!("  help: the install location has exactly one owner, GATE_SDK_NATIVE_BIN, and a");
            println!("        kit file carrying an adopter's path publishes it. Name the knob.");
        }
        if !undeclared.is_empty() {
            if !doors.is_empty() || !seam.is_empty() {
                println!();
            }
            println!("{}: configured surface(s) carry an undeclared door:", NAME);
            for d in &undeclared {
                println!("  {}", d);
            }
            println!("  help: a door on a consumer's own surface is adopter-facing unless its site says");
            println!("        otherwise — there is no 'door-adopter:' twin, so the unmarked case defaults");
            println!("        to deny. A contributor-facing door takes '<!-- door-contributor: <reason> -->'");
            println!("        on its own line or the line above; standing alone, that line covers the fenced");
            println!("        block it precedes, or the whole file before the first '#' heading. An");
            println!("        adopter-facing door takes no declaration: name the gate binary through");
            println!("        GATE_SDK_NATIVE_BIN instead, and where the binary does not exist yet carry");
            println!("        the precondition at the site (gate-sdk/SPEC.md §The adopter constraints). A");
            println!("        door inside a generated region takes its declaration from the emitter, never");
            println!("        from the page, which the next regeneration would erase.");
        }
        if !malformed.is_empty() {
            if !doors.is_empty() || !seam.is_empty() || !undeclared.is_empty() {
                println!();
            }
            println!(
                "{}: '{}' with an empty reason is malformed (the reason is the audit trail):",
                NAME, DECL
            );
            for m in &malformed {
                println!("  {}", m);
            }
            println!("  help: write the reason that says which contributor reads the door, on the");
            println!("        'comment-tier-exempt:' convention. An empty reason reds rather than exempting.");
        }
        return Ok(1);
    }

    // spec: guard-kit/SPEC.md §check-door-binding — all three counts on the clean line, so a corpus
    // that silently shrank to nothing is visible on green rather than passing vacuously
    println!(
        "DOOR-BINDING: clean ({} kit-shipped surface(s) name no front-end door, {} of them template file(s) naming no path to the binary, {} configured surface(s) swept)",
        swept, templates, configured_swept
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn door(line: &str, needle: &str) -> bool {
        let at = line.find(needle).unwrap();
        is_door(line, token_span(line, at, needle.len()))
    }

    // spec: guard-kit/SPEC.md §check-door-binding — the discriminator's two door shapes, and the
    // subject-position mention the amendment leaves standing
    #[test]
    fn a_spawned_or_armed_mention_is_a_door_and_a_subject_mention_is_not() {
        assert!(door("bash gate-sdk/bin/run-gates.sh --emit graph", "run-gates.sh"));
        assert!(door("`bash gate-sdk/bin/run-gates.sh`", "run-gates.sh"));
        assert!(!door("\"Bash(bash gate-sdk/bin/run-gates.sh *)\"", "run-gates.sh"));
        assert!(!door("      \"Bash(bash gate-sdk/bin/run-gates.sh)\",", "run-gates.sh"));
        assert!(door("\"Bash(ls)\" then bash gate-sdk/bin/run-gates.sh --emit graph", "run-gates.sh"));
        assert!(door("`pwsh -File gate-sdk/bin/run-gates.ps1`", "run-gates.ps1"));
        assert!(door("its twin: `pwsh -File gate-sdk/bin/run-gates.ps1`.", "run-gates.ps1"));
        assert!(door("`run-gates.sh --emit file-gap` mid-iteration", "run-gates.sh"));
        assert!(!door("`run-gates.sh` runs gates, not tests", "run-gates.sh"));
        assert!(!door("- `bin/run-gates.sh` — the aggregate battery", "run-gates.sh"));
        assert!(!door("`run-gates.sh`'s pre-push full battery", "run-gates.sh"));
        assert!(!door("rather than through bin/run-gates.sh. That front-end", "run-gates.sh"));
    }

    // spec: guard-kit/SPEC.md §check-door-binding — the declaration's mandatory reason: an empty one
    // is malformed and reds, and a line carrying no declaration is not one
    #[test]
    fn the_declaration_reason_is_mandatory_and_an_empty_one_is_malformed() {
        assert_eq!(decl_reason("<!-- door-contributor: a battery register -->"), Some("a battery register"));
        assert_eq!(decl_reason("<!-- door-contributor: -->"), Some(""));
        assert_eq!(decl_reason("<!-- door-contributor:"), Some(""));
        assert_eq!(decl_reason("plain prose about a door"), None);
        assert!(decl_alone("  <!-- door-contributor: r -->  "));
        assert!(!decl_alone("text <!-- door-contributor: r --> more"));
    }

    // spec: guard-kit/SPEC.md §check-door-binding — the two scopes: a standalone declaration before
    // the first heading covers the file, and one immediately preceding a fence covers that block
    #[test]
    fn a_standalone_declaration_covers_its_fence_or_the_whole_file() {
        let fenced = [
            "# Title",
            "",
            "<!-- door-contributor: the battery register -->",
            "```bash",
            "bash gate-sdk/bin/run-gates.sh --run-gate-tests x",
            "```",
            "bash gate-sdk/bin/run-gates.sh --run-demo",
        ];
        let s = scopes(&fenced);
        assert!(!s.file, "a declaration after the first heading is not file-scoped");
        assert!(s.exempts(4), "the fenced door rides the span");
        assert!(!s.exempts(6), "a door past the fence classifies on its own");

        let whole = [
            "<!-- door-contributor: a generated mirror -->",
            "# Title",
            "bash gate-sdk/bin/run-gates.sh --emit graph",
        ];
        let s = scopes(&whole);
        assert!(s.file && s.exempts(2), "before the first heading, the file is covered");

        let site = ["# Title", "<!-- door-contributor: the banner -->", "bash gate-sdk/bin/run-gates.sh --emit footprint"];
        let s = scopes(&site);
        assert!(!s.file && s.exempts(1) && s.exempts(2), "site scope is the line and the line above");

        let bad = ["# Title", "<!-- door-contributor: -->", "bash gate-sdk/bin/run-gates.sh --emit graph"];
        let s = scopes(&bad);
        assert_eq!(s.malformed, vec![1]);
        assert!(!s.exempts(2), "a malformed declaration reds rather than exempting");
    }

    // spec: guard-kit/SPEC.md §check-door-binding — the exemption keys on the arm, so a hook command
    // is exempt in a JSON `command` value exactly as it is on a shell line
    #[test]
    fn a_fail_open_arm_exempts_the_door_that_carries_it() {
        let hook = "{ \"command\": \"bash gate-sdk/bin/run-gates.sh --hook wakeup-guard\" }";
        let at = hook.find("run-gates.sh").unwrap();
        let span = token_span(hook, at, "run-gates.sh".len());
        assert!(is_door(hook, span));
        assert!(names_fail_open_arm(hook, span.1));
        let poll = "bash gate-sdk/bin/run-gates.sh --usage-poll";
        let at = poll.find("run-gates.sh").unwrap();
        assert!(!names_fail_open_arm(poll, token_span(poll, at, 12).1));
    }

    // spec: guard-kit/SPEC.md §check-door-binding — the seam reds on a path to the artifact and not
    // on the bare program name, which is the kit's own vocabulary rather than an adopter's location
    #[test]
    fn the_seam_reads_a_path_to_the_artifact_and_not_its_bare_name() {
        let a = env!("CARGO_PKG_NAME");
        for (line, want) in [
            ("GATE_SDK_NATIVE_BIN = scripts/checkwright-gates", true),
            ("run ./native/target/release/checkwright-gates --run", true),
            ("the binary named by GATE_SDK_NATIVE_BIN, not checkwright-gates", false),
        ] {
            let at = line.find(a).unwrap();
            let (s, e) = token_span(line, at, a.len());
            assert_eq!(line[s..e].contains('/'), want, "{}", line);
        }
    }
}
