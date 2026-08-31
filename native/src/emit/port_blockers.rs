// spec: gate-sdk/SPEC.md §port-blockers — the derived roster for the port's remaining work, at each
// invocation. Three exclusive arms over two corpora: the registry arms answer criteria 7 and 6 and
// speak for the battery alone, while `--tree` answers the completion predicate over the tree.
use crate::bashscan::{self, Kind};
use crate::registry;
use crate::walk;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's declared structural knobs. A hardcoded
// top-level flag receives no configuration at all, so this tool ported as one would resolve
// platform defaults and ignore every consumer override; the family is forced, not chosen.
pub const KNOBS: &[&str] = &[
    "GATE_SDK_GATES_DIR",
    "GATE_KIT_ROOTS_HERE",
    "GATE_KIT_ROOTS_REL",
    "GATE_PRUNE_DIRS",
    "GATE_SDK_PROGRAM_FLOOR",
    "GATE_SDK_TESTS_DIR",
    // spec: gate-sdk/SPEC.md §The non-gate arm — the union sentinel: this arm resolves an
    // *arbitrary* knob discovered at scan time, so a fixed roster cannot name what it must read.
    // It expands to the knobs of every member the **tree's** registry names, `--gates-dir`-scoped.
    super::EVERY_REGISTERED_KNOB,
];

const USAGE: &str = "\
usage: run-gates.sh --emit port-blockers [--gates-dir <dir>] [--group | --tree]

  (no arm)       criterion 7: every registered gate's external-program
                 requirements beyond GATE_SDK_PROGRAM_FLOOR, one
                 '<member><TAB><program><TAB><evidence>' row each — a shell
                 rule tokenized in place, a ported member read off its own
                 registry requirement declaration.
  --group        criterion 6: the corpus-derivation partition over the
                 still-shell members, largest group first, each member
                 carrying lines= (its declaration's line count), its
                 criterion 2/3/7 columns and expanded couples=.
  --tree         the port disposition of every tracked non-test shell file,
                 one '<path><TAB><disposition><TAB>lines=<n>' row each —
                 'owed', 'no-port' or 'port-until:<slug>'. The trailer's
                 owed count is the completion predicate: at zero, every
                 remaining non-test .sh carries a stated cause or is gone.
  --gates-dir    the registry the two registry arms walk, and what scopes
                 this arm's own declared-knob union. --tree needs none and
                 takes none: its corpus is the tree, not the registry.
  -h, --help     this text.

The first two arms walk the gate registry and answer for the battery; --tree
walks the tracked shell tree and answers for the project. A registry arm
reading zero owed means the battery is ported and says nothing about the tree.

The two registry arms are advisory: nothing parses either, and what cannot be
decided prints '?' and is counted rather than guessed. --tree has no such
class -- an undeclared file is 'owed', which over-counts it as work rather
than losing it -- and its trailer alone is read by a machine, a consumer's
measured-claim emitter, so that one line's grammar is an interface. The rows
are not: they are read beside a diff, like the other arms' rows.

lines= is one sizing input beside the criterion columns, a floor on a port's
size and never a ranking of it: cost concentrated in interfaces, or behind a
spawned tool, is invisible to it.
";

#[derive(PartialEq, Eq, Clone, Copy)]
enum Mode {
    Default,
    Group,
    Tree,
}

struct Args {
    mode: Mode,
    gates_dir: Option<String>,
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — `-h`/`--help` as the first argument prints usage
// at exit 0 **whatever follows it**: the help-before-arity ordering is what that contract decides,
// and a port that refused help-plus-extra would be a silent behaviour change.
fn parse(args: &[String]) -> Result<Option<Args>, String> {
    if matches!(args.first().map(String::as_str), Some("-h") | Some("--help")) {
        return Ok(None);
    }
    let mut mode: Option<Mode> = None;
    let mut gates_dir: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        let mut set = |m: Mode| match mode {
            Some(_) => Err(USAGE.to_string()),
            None => {
                mode = Some(m);
                Ok(())
            }
        };
        match args[i].as_str() {
            "--group" => set(Mode::Group)?,
            "--tree" => set(Mode::Tree)?,
            "--gates-dir" => {
                match args.get(i + 1) {
                    Some(d) if gates_dir.is_none() => gates_dir = Some(d.clone()),
                    _ => return Err(USAGE.to_string()),
                }
                i += 1;
            }
            _ => return Err(USAGE.to_string()),
        }
        i += 1;
    }
    let mode = mode.unwrap_or(Mode::Default);
    // spec: gate-sdk/SPEC.md §The non-gate arm — a documented flag that silently changes nothing is
    // worse than no flag, so the arm whose corpus is the tree refuses a registry it never reads.
    if mode == Mode::Tree && gates_dir.is_some() {
        return Err(USAGE.to_string());
    }
    Ok(Some(Args { mode, gates_dir }))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let parsed = match parse(args)? {
        Some(a) => a,
        None => return Ok(USAGE.to_string()),
    };
    match parsed.mode {
        Mode::Tree => tree(),
        m => registry_arm(m, parsed.gates_dir),
    }
}

// spec: gate-sdk/SPEC.md §port-blockers — the tree arm dispatches ahead of the registry resolution
// the other two share, so a tree carrying no gates.list still answers for its own shell: requiring
// a registry to count files the registry does not contain is the corpus confusion it exists to end.
fn tree() -> Result<String, String> {
    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the shared corpus rule degrades to an
    // empty corpus where git cannot answer, deliberately and on a monotonicity ground for its other
    // reader; this arm absorbs the divergence by probing first rather than printing `0 owed`.
    let repo = matches!(
        crate::proc::run("git", &["rev-parse", "--git-dir"]),
        Ok(ref c) if c.stdout().is_some()
    );
    if !repo {
        return Err(
            "--tree: not a git repository — this arm's corpus is the tracked shell tree".to_string(),
        );
    }
    let files = walk::tracked_shell_tree()?;
    let (mut scanned, mut noport, mut held, mut owed) = (0usize, 0usize, 0usize, 0usize);
    let mut out = String::new();
    for f in &files {
        scanned += 1;
        let (disp, lines) = match std::fs::read(Path::new(f)) {
            Ok(b) => {
                let text = String::from_utf8_lossy(&b).into_owned();
                (
                    walk::disposition(&walk::header_block(&text)),
                    line_count(&text),
                )
            }
            Err(_) => (walk::Disposition::Owed, 0),
        };
        let column = match &disp {
            walk::Disposition::NoPort => {
                noport += 1;
                "no-port".to_string()
            }
            walk::Disposition::PortUntil(s) => {
                held += 1;
                format!("port-until:{}", s)
            }
            walk::Disposition::Owed => {
                owed += 1;
                "owed".to_string()
            }
        };
        out.push_str(&format!("{}\t{}\tlines={}\n", f, column, lines));
    }
    // spec: gate-sdk/SPEC.md §port-blockers — held is separated from no-port because a temporary
    // hold is not a permanent disposition, and there is no fourth count because there is no fourth
    // disposition: absence is owed, which over-counts an undeclared file as work rather than losing it.
    out.push_str(&format!(
        "port-blockers --tree: {} file(s) scanned, {} declared no-port, {} temporarily held, {} owed\n",
        scanned, noport, held, owed
    ));
    Ok(out)
}

// spec: gate-sdk/SPEC.md §port-blockers — `wc -l` over the same resolved path the row's other
// columns are read from, so the field cannot disagree with its own row about which file it describes.
fn line_count(text: &str) -> usize {
    text.as_bytes().iter().filter(|c| **c == b'\n').count()
}

struct Row {
    member: String,
    program: String,
    evidence: String,
}

struct Registry {
    gates_dir: String,
    check_dirs: Vec<String>,
    tests_dirs: Vec<String>,
    floor: BTreeSet<String>,
    kit_funcs: BTreeSet<String>,
    kit_roots_rel: Vec<String>,
}

// spec: gate-sdk/SPEC.md §port-blockers — the resolution dirs taken through the *repo-relative*
// kit roots, so every evidence path this report prints resolves for a reader beside a diff; an
// absolute clone path resolves for nobody.
fn resolve_registry(gates_dir: Option<String>) -> Result<Registry, String> {
    let gates_dir = match gates_dir {
        Some(d) => d,
        None => walk::knob_scalar("GATE_SDK_GATES_DIR")?,
    };
    let kit_roots_rel = walk::kit_roots_rel()?;
    let mut kit_funcs = BTreeSet::new();
    for root in walk::kit_roots_abs()? {
        for lib in walk::glob_entries(&format!("{}/lib/*.sh", root)) {
            if let Ok(b) = std::fs::read(Path::new(&lib)) {
                collect_functions(&String::from_utf8_lossy(&b), &mut kit_funcs);
            }
        }
    }
    Ok(Registry {
        check_dirs: registry::resolve_dirs(&gates_dir, &kit_roots_rel),
        tests_dirs: registry::fixture_dirs(&walk::knob_scalar("GATE_SDK_TESTS_DIR")?, &kit_roots_rel),
        floor: walk::knob_array("GATE_SDK_PROGRAM_FLOOR")?.into_iter().collect(),
        kit_funcs,
        kit_roots_rel,
        gates_dir,
    })
}

// spec: gate-sdk/SPEC.md §The port-candidate criteria — a name any kit library defines as a shell
// function is not an external program, and the set is derived from the tree, so a kit that adds a
// helper never has to be listed anywhere. `declare -F <name>` is the same fact probed rather than defined.
fn collect_functions(text: &str, out: &mut BTreeSet<String>) {
    for line in text.lines() {
        let t = line.trim_start_matches([' ', '\t']);
        let t = t.strip_prefix("function ").map(str::trim_start).unwrap_or(t);
        if let Some(name) = leading_name(t) {
            let rest = t[name.len()..].trim_start_matches([' ', '\t']);
            if rest.starts_with("()") {
                out.insert(name);
            }
        }
        for at in find_all(line, "declare -F ") {
            let tail = &line[at + "declare -F ".len()..];
            if let Some(name) = leading_name(tail.trim_start_matches([' ', '\t'])) {
                out.insert(name);
            }
        }
    }
}

fn find_all(haystack: &str, needle: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(at) = haystack[from..].find(needle) {
        out.push(from + at);
        from += at + 1;
    }
    out
}

fn leading_name(s: &str) -> Option<String> {
    let b = s.as_bytes();
    if b.is_empty() || !(b[0].is_ascii_alphabetic() || b[0] == b'_') {
        return None;
    }
    let mut i = 1usize;
    while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
        i += 1;
    }
    Some(s[..i].to_string())
}

// spec: gate-sdk/SPEC.md §port-blockers — keyword and builtin status is asked of the interpreter
// rather than held as a roster: the classification is a property of bash, not a list this tool
// maintains. One batched query per run, `bash` being on the program floor already.
fn builtins(words: &BTreeSet<String>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    if words.is_empty() {
        return out;
    }
    let mut argv: Vec<&str> = vec![
        "-c",
        "for w in \"$@\"; do printf '%s\\t%s\\n' \"$w\" \"$(type -t \"$w\" 2>/dev/null)\"; done",
        "bash",
    ];
    argv.extend(words.iter().map(String::as_str));
    let out_bytes = match crate::proc::run("bash", &argv) {
        Ok(c) => match c.stdout() {
            Some(b) => b.to_vec(),
            None => return out,
        },
        Err(_) => return out,
    };
    for line in String::from_utf8_lossy(&out_bytes).lines() {
        if let Some((word, kind)) = line.split_once('\t') {
            if kind == "keyword" || kind == "builtin" {
                out.insert(word.to_string());
            }
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §port-blockers — a knob's default resolves through the config bridge the
// arm's own union sentinel had resolved before the exec, the one place a knob default is read, so
// this report cannot disagree with the value a dispatched binary is handed.
fn knob_program(knob: &str) -> Option<String> {
    if knob.chars().any(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let raw = std::env::var(format!("GATE_SDK_KNOB_{}", knob)).ok()?;
    let first = raw.split('\t').next().unwrap_or_default();
    let word = first.split(' ').next().unwrap_or_default();
    if word.is_empty() {
        None
    } else {
        Some(word.to_string())
    }
}

struct Member {
    rows: Vec<Row>,
    undecidable: bool,
    libcalls: BTreeSet<String>,
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a `.gate` member's rule is a binary subcommand
// with no text to tokenize, so its requirements are read off the registry's own declaration, onto
// the three row shapes a scanned command word already reaches and no fourth.
fn needs_rows(member: &str, decl: &str, reg: &Registry) -> Member {
    let mut m = Member {
        rows: Vec::new(),
        undecidable: false,
        libcalls: BTreeSet::new(),
    };
    // spec: gate-sdk/SPEC.md §Fail-closed contract — a registry that cannot answer for the member
    // is reported undecidable, never as an empty requirement set: a member silently reported clean
    // because the question failed is the captured-emptiness false green in report form.
    let reqs = match crate::gates::needs(member) {
        Some(r) => r,
        None => {
            m.rows.push(Row {
                member: member.to_string(),
                program: "?".to_string(),
                evidence: format!("{} (binary substrate; --needs unavailable)", decl),
            });
            m.undecidable = true;
            return m;
        }
    };
    for (program, knob) in reqs {
        if *program == "?" {
            if knob.is_empty() {
                m.rows.push(Row {
                    member: member.to_string(),
                    program: "?".to_string(),
                    evidence: format!("{} (--needs: unbounded in the registry)", decl),
                });
                m.undecidable = true;
                continue;
            }
            match knob_program(knob) {
                None => {
                    m.rows.push(Row {
                        member: member.to_string(),
                        program: "?".to_string(),
                        evidence: format!("{} (--needs ${}, default unresolvable)", decl, knob),
                    });
                    m.undecidable = true;
                }
                Some(prog) => {
                    if !reg.floor.contains(&prog) {
                        m.rows.push(Row {
                            member: member.to_string(),
                            program: prog,
                            evidence: format!("{} (--needs ${})", decl, knob),
                        });
                    }
                }
            }
            continue;
        }
        if !reg.floor.contains(*program) {
            m.rows.push(Row {
                member: member.to_string(),
                program: (*program).to_string(),
                evidence: format!("{} (--needs)", decl),
            });
        }
    }
    m
}

// spec: gate-sdk/SPEC.md §port-blockers — the three positive derivation inputs in descending
// confidence: the `command -v` guard, command position, and knob resolution for a command-position
// expansion, each filtered against the program floor. The kit-library call set is this filter inverted.
fn scan_rows(member: &str, decl: &str, text: &str, reg: &Registry) -> Member {
    let mut m = Member {
        rows: Vec::new(),
        undecidable: false,
        libcalls: BTreeSet::new(),
    };
    let mut local = BTreeSet::new();
    collect_functions(text, &mut local);

    // spec: gate-sdk/SPEC.md §port-blockers — a command-position expansion resolves to a *program*
    // before any classification of it, so the resolved word is what the interpreter is asked about;
    // asking about the knob name instead would classify the wrong word.
    enum Resolved {
        Program(String, String),
        Unresolvable(String, String),
    }
    let mut resolved: Vec<Resolved> = Vec::new();
    for t in bashscan::command_positions(text) {
        match t.kind {
            Kind::Expansion => match knob_program(&t.word) {
                Some(p) => resolved.push(Resolved::Program(
                    p,
                    format!("{}:{} (${})", decl, t.line, t.word),
                )),
                None => resolved.push(Resolved::Unresolvable(
                    t.word.clone(),
                    format!(
                        "{}:{} (command-position ${}, default unresolvable)",
                        decl, t.line, t.word
                    ),
                )),
            },
            _ => resolved.push(Resolved::Program(
                t.word.clone(),
                format!("{}:{}", decl, t.line),
            )),
        }
    }

    let candidates: BTreeSet<String> = resolved
        .iter()
        .filter_map(|r| match r {
            Resolved::Program(p, _) => Some(p.clone()),
            Resolved::Unresolvable(_, _) => None,
        })
        .collect();
    let builtin = builtins(&candidates);

    let mut seen: BTreeSet<String> = BTreeSet::new();
    for r in resolved {
        let (program, evidence) = match r {
            Resolved::Unresolvable(knob, evidence) => {
                if !seen.insert(format!("?{}", knob)) {
                    continue;
                }
                m.rows.push(Row {
                    member: member.to_string(),
                    program: "?".to_string(),
                    evidence,
                });
                m.undecidable = true;
                continue;
            }
            Resolved::Program(p, e) => (p, e),
        };
        if local.contains(&program) || reg.kit_funcs.contains(&program) {
            m.libcalls.insert(program);
            continue;
        }
        if reg.floor.contains(&program) || builtin.contains(&program) {
            continue;
        }
        if !seen.insert(program.clone()) {
            continue;
        }
        m.rows.push(Row {
            member: member.to_string(),
            program,
            evidence,
        });
    }
    m
}

// spec: gate-sdk/SPEC.md §port-blockers — criterion 2's column reads the fixture dirs
// check-gate-fixture-coverage resolves, in that order, so the report and that gate can never
// disagree about whether a member carries a pair.
fn criterion2(member: &str, text: &str, reg: &Registry) -> &'static str {
    for t in &reg.tests_dirs {
        if Path::new(&format!("{}/{}/good", t, member)).is_dir()
            && Path::new(&format!("{}/{}/bad", t, member)).is_dir()
        {
            return "pair";
        }
    }
    if text.lines().any(|l| l.starts_with("# no-fixture:")) {
        return "no-fixture";
    }
    "none"
}

// spec: gate-sdk/SPEC.md §port-blockers — the grouping key is set-equality over two derived
// factors, the kit-library call set and the content-glob set; a single shared call is not evidence
// of a shared derivation, and both single-factor candidates were measured to over-select.
fn glob_set(text: &str) -> String {
    let mut globs: BTreeSet<String> = BTreeSet::new();
    for line in text.lines() {
        if line.trim_start_matches([' ', '\t']).starts_with('#') {
            continue;
        }
        let b = line.as_bytes();
        let mut i = 0usize;
        while i + 1 < b.len() {
            if b[i] == b'*' && b[i + 1] == b'.' {
                let mut j = i + 2;
                while j < b.len() && b[j].is_ascii_alphanumeric() {
                    j += 1;
                }
                if j > i + 2 {
                    globs.insert(line[i..j].to_string());
                    i = j;
                    continue;
                }
            }
            i += 1;
        }
    }
    globs.into_iter().collect::<Vec<_>>().join(",")
}

fn registry_arm(mode: Mode, gates_dir: Option<String>) -> Result<String, String> {
    let reg = resolve_registry(gates_dir)?;
    let list = registry::list_path(&reg.gates_dir);
    if !Path::new(&list).is_file() {
        return Err(format!("registry not found: {}", list));
    }
    let listing = std::fs::read(&list)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", list, e))?;

    let mut scanned = 0usize;
    let mut undecidable = 0usize;
    let mut rows: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, (usize, String)> = BTreeMap::new();
    let mut unkeyed: Vec<String> = Vec::new();
    let (mut ported, mut permanent, mut held) = (0usize, 0usize, 0usize);

    for member in registry::members(&listing) {
        scanned += 1;
        let decl = registry::resolve(&member, &reg.check_dirs).ok_or_else(|| {
            format!("{} is registered but resolves to no declaration path", member)
        })?;
        let text = std::fs::read(Path::new(&decl))
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();

        // spec: gate-sdk/SPEC.md §port-blockers — a ported member leaves the partition entirely
        // rather than printing `?`: the grouping orders the *remaining* corpus, so there is no open
        // question to report. `--group` does not consume the requirement lookup: it has no row to fill.
        if decl.ends_with(".gate") {
            if mode == Mode::Group {
                ported += 1;
                continue;
            }
            let m = needs_rows(&member, &decl, &reg);
            undecidable += usize::from(m.undecidable);
            rows.extend(m.rows.into_iter().map(format_row));
            continue;
        }
        // spec: gate-sdk/SPEC.md §port-blockers — a declared-permanent member leaves the partition
        // on the ported member's terms and only on this arm, and a held one leaves on the
        // permanent member's terms but is counted apart from it, because it is still owed.
        if mode == Mode::Group {
            let header = walk::header_block(&text);
            if header
                .lines()
                .any(|l| field_opens(l, "no-port:"))
            {
                permanent += 1;
                continue;
            }
            if header.lines().any(|l| field_opens(l, "port-until:")) {
                held += 1;
                continue;
            }
        }

        let m = scan_rows(&member, &decl, &text, &reg);
        undecidable += usize::from(m.undecidable);
        if mode == Mode::Group {
            let lib_key = m.libcalls.iter().cloned().collect::<Vec<_>>().join(",");
            let glob_key = glob_set(&text);
            let lines = line_count(&text);
            if lib_key.is_empty() && glob_key.is_empty() {
                unkeyed.push(format!("  ?  {}\tlines={}\t{}", member, lines, decl));
                continue;
            }
            let c7 = if m.undecidable {
                "?".to_string()
            } else if m.rows.is_empty() {
                "clean".to_string()
            } else {
                m.rows
                    .iter()
                    .map(|r| r.program.clone())
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let fields = registry::manifest_fields(registry::manifest_line(&text).unwrap_or(""));
            let tier = registry::field(&fields, "tier");
            let couples =
                registry::expand_couples(&registry::field(&fields, "couples"), &reg.kit_roots_rel);
            let key = format!(
                "libs={} globs={}",
                dash_if_empty(&lib_key),
                dash_if_empty(&glob_key)
            );
            // spec: gate-sdk/SPEC.md §port-blockers — lines= sits in the fixed-width run ahead of
            // c2=, never appended: c7= is variable-width, so a field after it is the one column
            // that cannot be aligned, and a cost column read down a list is the one that must be.
            let row = format!(
                "  {:<36} lines={:<5} c2={:<10} c3={:<9} c7={}\n      couples={}\n",
                member,
                lines,
                criterion2(&member, &text, &reg),
                dash_if_empty(&tier),
                c7,
                dash_if_empty(&couples)
            );
            let entry = groups.entry(key).or_insert((0, String::new()));
            entry.0 += 1;
            entry.1.push_str(&row);
        } else {
            rows.extend(m.rows.into_iter().map(format_row));
        }
    }

    if mode == Mode::Group {
        return Ok(render_group(
            groups, unkeyed, scanned, ported, permanent, held,
        ));
    }
    rows.sort();
    let mut out: String = rows.concat();
    out.push_str(&format!(
        "port-blockers: {} member(s) scanned, {} with a requirement this report could not decide\n",
        scanned, undecidable
    ));
    Ok(out)
}

fn field_opens(line: &str, name: &str) -> bool {
    line.strip_prefix('#')
        .map(|r| r.trim_start_matches([' ', '\t']).starts_with(name))
        .unwrap_or(false)
}

fn format_row(r: Row) -> String {
    format!("{}\t{}\t{}\n", r.member, r.program, r.evidence)
}

fn dash_if_empty(s: &str) -> String {
    if s.is_empty() {
        "-".to_string()
    } else {
        s.to_string()
    }
}

// spec: gate-sdk/SPEC.md §port-blockers — groups ordered by size descending, then by key; the two
// derived totals close the trailer because four exclusion classes put them beyond a reader's own
// subtraction, and an empty-keyed member is reported, never grouped, and never grouped with another.
fn render_group(
    groups: BTreeMap<String, (usize, String)>,
    unkeyed: Vec<String>,
    scanned: usize,
    ported: usize,
    permanent: usize,
    held: usize,
) -> String {
    let mut ordered: Vec<(usize, String, String)> = groups
        .into_iter()
        .map(|(k, (n, body))| (n, k, body))
        .collect();
    ordered.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut out = String::new();
    for (idx, (n, key, body)) in ordered.iter().enumerate() {
        out.push_str(&format!("group {}: {} member(s)\n  key: {}\n", idx + 1, n, key));
        out.push_str(body);
        out.push('\n');
    }
    if !unkeyed.is_empty() {
        out.push_str("undecidable (no kit-library call and no content glob this tool can see):\n");
        out.push_str(&unkeyed.join("\n"));
        out.push_str("\n\n");
    }
    let still_owed = scanned - ported - permanent;
    let takeable = still_owed - held;
    out.push_str(&format!(
        "port-blockers --group: {} member(s) scanned, {} group(s) formed, {} undecidable, {} already ported and excluded, {} permanently shell and excluded, {} temporarily held and excluded; {} still owed, {} takeable at this cut\n",
        scanned, ordered.len(), unkeyed.len(), ported, permanent, held, still_owed, takeable
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — help before arity, and an unrecognized
    // argument refused rather than absorbed. The tool has no positionals, so a bare word is
    // unrecognized on the same footing as an unknown flag.
    #[test]
    fn help_wins_over_everything_that_follows_and_a_stray_word_refuses() {
        assert!(parse(&["--help".into(), "--group".into()])
            .expect("help refused")
            .is_none());
        assert!(parse(&["-h".into()]).expect("help refused").is_none());
        assert!(parse(&["--bogus".into()]).is_err());
        assert!(parse(&["group".into()]).is_err());
        assert!(parse(&["--group".into(), "--tree".into()]).is_err());
        assert!(parse(&["--gates-dir".into()]).is_err());
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — `--tree` needs no gates dir and takes none, and a
    // documented flag that silently changes nothing is worse than no flag.
    #[test]
    fn the_tree_arm_refuses_a_registry_it_never_reads() {
        assert!(parse(&["--gates-dir".into(), "scripts".into(), "--tree".into()]).is_err());
        let a = parse(&["--gates-dir".into(), "scripts".into(), "--group".into()])
            .expect("a scoped group arm refused")
            .expect("a scoped group arm read as help");
        assert_eq!(a.gates_dir.as_deref(), Some("scripts"));
        assert!(a.mode == Mode::Group);
        let d = parse(&[]).expect("the default arm refused").expect("help");
        assert!(d.mode == Mode::Default && d.gates_dir.is_none());
    }

    // spec: gate-sdk/SPEC.md §port-blockers — a knob outside the union reports `?` with the same
    // *default unresolvable* evidence, and a lowercase name is not a knob at all.
    #[test]
    fn an_unbridged_knob_resolves_to_nothing_rather_than_to_a_guess() {
        assert_eq!(knob_program("gate_sdk_lowercase"), None);
        assert_eq!(knob_program("PORT_BLOCKERS_ABSENT_KNOB"), None);
        let knobs = crate::knobenv::lock();
        knobs.set("GATE_SDK_KNOB_PORT_BLOCKERS_TEST", "ruby -w\tsecond");
        assert_eq!(
            knob_program("PORT_BLOCKERS_TEST"),
            Some("ruby".to_string()),
            "the requirement is the resolved value's command word: first element, first word"
        );
        knobs.remove("GATE_SDK_KNOB_PORT_BLOCKERS_TEST");
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the content-glob factor is read off the
    // declaration's non-comment lines, which is what keeps the `# graph:` manifest out of the key.
    #[test]
    fn the_glob_factor_ignores_comment_lines() {
        let text = "# graph: couples=*.md,*.gate\ngate_find . '*.sh' '*.rs'\n";
        assert_eq!(glob_set(text), "*.rs,*.sh");
    }

    // spec: gate-sdk/SPEC.md §The port-candidate criteria — a kit library's definitions and the
    // names a declaration probes with `declare -F` are both text scans, and they stay text scans.
    #[test]
    fn a_function_harvest_reads_definitions_and_probes_alike() {
        let mut got = BTreeSet::new();
        collect_functions(
            "gate_find() {\n  :\n}\nfunction gate_emit () {\n  :\n}\nif declare -F on_hook >/dev/null; then :; fi\necho not_a_function ()\n",
            &mut got,
        );
        assert!(got.contains("gate_find"));
        assert!(got.contains("gate_emit"));
        assert!(got.contains("on_hook"));
    }

    fn bare_registry() -> Registry {
        Registry {
            gates_dir: "scripts".to_string(),
            check_dirs: vec!["scripts".to_string()],
            tests_dirs: vec!["scripts/gate-tests".to_string()],
            floor: ["bash", "git"].iter().map(|s| (*s).to_string()).collect(),
            kit_funcs: BTreeSet::new(),
            kit_roots_rel: vec!["gate-sdk".to_string()],
        }
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the compiled-member path: a program takes the same
    // floor filter a scanned command word takes, so an on-floor program is suppressed on both
    // substrates by one rule, and an off-floor one keeps its `(--needs)` evidence.
    #[test]
    fn a_compiled_members_requirement_is_read_off_the_registry_and_floor_filtered() {
        let reg = bare_registry();
        let m = needs_rows("check-shellcheck", "gate-sdk/checks/check-shellcheck.gate", &reg);
        assert!(!m.undecidable);
        let progs: Vec<&str> = m.rows.iter().map(|r| r.program.as_str()).collect();
        assert!(progs.contains(&"shellcheck"), "the off-floor requirement is missing");
        assert!(
            !progs.contains(&"bash"),
            "an on-floor program survived the filter the scanned path applies"
        );
        assert!(m.rows[0].evidence.ends_with("(--needs)"));
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — a member the registry cannot answer for is
    // reported undecidable, never as an empty requirement set: a member silently reported clean
    // because the question failed is the captured-emptiness false green in report form.
    #[test]
    fn a_member_the_registry_cannot_answer_for_is_undecidable_rather_than_clean() {
        let reg = bare_registry();
        let m = needs_rows("check-not-a-registered-member", "scripts/absent.gate", &reg);
        assert!(m.undecidable, "the cannot-answer branch did not reach the counter");
        assert_eq!(m.rows.len(), 1);
        assert_eq!(m.rows[0].program, "?");
        assert!(m.rows[0].evidence.contains("--needs unavailable"));
    }

    // spec: gate-sdk/SPEC.md §port-blockers — a command-position expansion whose default cannot be
    // resolved prints `?` and reaches the undecidable counter both arms read, while a resolvable
    // one becomes an ordinary row; reporting nothing would be the false negative.
    #[test]
    fn an_unresolvable_expansion_counts_undecidable_and_a_resolvable_one_becomes_a_row() {
        let reg = bare_registry();
        let blind = scan_rows(
            "check-probe",
            "scripts/check-probe.sh",
            "\"$PORT_BLOCKERS_UNBRIDGED\" --check\n\"$PORT_BLOCKERS_UNBRIDGED\" again\n",
            &reg,
        );
        assert!(blind.undecidable);
        assert_eq!(blind.rows.len(), 1, "one row per unresolvable knob, not one per site");
        assert_eq!(blind.rows[0].program, "?");
        assert!(blind.rows[0].evidence.contains("default unresolvable"));

        let knobs = crate::knobenv::lock();
        knobs.set("GATE_SDK_KNOB_PORT_BLOCKERS_SCAN", "ruby -w");
        let seeing = scan_rows(
            "check-probe",
            "scripts/check-probe.sh",
            "\"$PORT_BLOCKERS_SCAN\" --check\n",
            &reg,
        );
        knobs.remove("GATE_SDK_KNOB_PORT_BLOCKERS_SCAN");
        assert!(!seeing.undecidable);
        assert_eq!(seeing.rows.len(), 1);
        assert_eq!(seeing.rows[0].program, "ruby");
        assert!(seeing.rows[0].evidence.ends_with("($PORT_BLOCKERS_SCAN)"));
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the kit-library call set is the default arm's own
    // filter inverted: a name a kit library defines is discarded as a requirement and is exactly
    // what the `--group` key is made of, so no roster of corpus primitives is maintained anywhere.
    #[test]
    fn a_kit_library_call_leaves_the_rows_and_joins_the_key() {
        let mut reg = bare_registry();
        reg.kit_funcs.insert("gate_find".to_string());
        let m = scan_rows(
            "check-probe",
            "scripts/check-probe.sh",
            "gate_find . '*.sh'\nshellcheck x\n",
            &reg,
        );
        assert_eq!(m.libcalls.iter().cloned().collect::<Vec<_>>(), vec!["gate_find"]);
        assert_eq!(
            m.rows.iter().map(|r| r.program.as_str()).collect::<Vec<_>>(),
            vec!["shellcheck"]
        );
    }

    // spec: gate-sdk/SPEC.md §port-blockers — keyword and builtin status is asked of the
    // interpreter, one batched query per run, so the classification is a property of bash.
    #[test]
    fn the_interpreter_answers_for_keywords_and_builtins_and_nothing_else() {
        let words: BTreeSet<String> = ["printf", "if", "checkwright_not_a_program"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        let got = builtins(&words);
        assert!(got.contains("printf"), "a builtin was not classified as one");
        assert!(got.contains("if"), "a keyword was not classified as one");
        assert!(!got.contains("checkwright_not_a_program"));
    }
}
