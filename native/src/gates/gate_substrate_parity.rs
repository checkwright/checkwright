// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — one declaration per member,
// descriptor/subcommand parity both ways, a disposition for every substrate-sensitive member,
// no implementation source in the vendoring set, and one owner for the target roster
use crate::fresh;
use crate::gates;
use crate::proc;
use crate::registry;
use crate::section;
use crate::walk;
use std::path::Path;

const SECTION: &str = "## Meta-gate conservation for the binary substrate";

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion B's owner column: a kit
// directory basename, or this sentinel for a member the consumer's own gates directory declares
const CONSUMER_OWNER: &str = "-";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-gate-substrate-parity: {}", e);
            2
        }
    }
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

fn ltrim(s: &str) -> &str {
    s.trim_start_matches([' ', '\t', '\n', '\x0b', '\x0c', '\r'])
}

fn trim(s: &str) -> &str {
    s.trim_matches([' ', '\t', '\n', '\x0b', '\x0c', '\r'])
}

// spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — the one disposition
// surface, opened by exact heading equality and closed at the next level-2 heading, so a nested
// subsection stays inside the body assertion B and assertion C both read
fn conservation_body(text: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    let mut inb = false;
    for line in section::split_lines(text) {
        if line == SECTION {
            inb = true;
            continue;
        }
        if inb && line.starts_with("## ") {
            inb = false;
        }
        if inb {
            out.push(line);
        }
    }
    out.join("\n")
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion H opens the section the
// declaration's own pointer names, so the depth is the matched heading's own and the body runs to
// the next heading at that depth or shallower: the two live heading levels need no special case
fn spec_section_body(text: &str, want: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    let mut inb = false;
    let mut lvl = 0usize;
    for line in section::split_lines(text) {
        let level = section::heading_level(line);
        if level > 0 {
            if inb && level <= lvl {
                inb = false;
            }
            let t = trim(line.trim_start_matches('#'));
            if !inb && t == want {
                inb = true;
                lvl = level;
                continue;
            }
        }
        if inb {
            out.push(line);
        }
    }
    out.join("\n")
}

fn count_field(text: &str, field: &str) -> usize {
    section::split_lines(text)
        .into_iter()
        .filter(|l| field_payload(l, field).is_some())
        .count()
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a header field is a comment leader, optional
// space, the field name and its colon; everything after the colon is the payload
fn field_payload<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    let rest = line.strip_prefix('#')?;
    let rest = rest.trim_start_matches([' ', '\t']);
    rest.strip_prefix(field)?.strip_prefix(':')
}

fn first_payload<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    section::split_lines(text)
        .into_iter()
        .find_map(|l| field_payload(l, field))
}

struct Ctx {
    findings: Vec<String>,
}

fn main_rule_dirs(gates_dir: &str, kit_roots: &[String]) -> (Vec<String>, Vec<String>) {
    let mut dirs = vec![gates_dir.to_string()];
    let mut names: Vec<String> = Vec::new();
    for k in kit_roots {
        if k.is_empty() {
            continue;
        }
        let k = k.trim_end_matches('/');
        dirs.push(format!("{}/checks", k));
        names.push(k.rsplit('/').next().unwrap_or(k).to_string());
    }
    (dirs, names)
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion B's two-clause scope rule: a
// kit-owned subcommand is in scope iff this tree vendored that kit, and a consumer-declared one
// iff this is the tree that declared it, which is the tree carrying the crate's tracked source
fn subcommand_in_scope(owner: &str, kit_names: &[String], publishing: bool) -> bool {
    if owner == CONSUMER_OWNER {
        return publishing;
    }
    kit_names.iter().any(|k| k == owner)
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion B's roster half, as a value
// rather than as a walk: the descriptor set, the roster and the two scope inputs are everything
// it reads, so the configurations no committed fixture can manufacture are unit-testable
struct RosterVerdict {
    in_scope: usize,
    out_of_scope: usize,
    refonly: usize,
    findings: Vec<String>,
}

fn roster_parity(
    descriptors: &[String],
    roster: &[(String, String)],
    kit_names: &[String],
    publishing: bool,
    section_body: &str,
) -> RosterVerdict {
    let mut v = RosterVerdict {
        in_scope: 0,
        out_of_scope: 0,
        refonly: 0,
        findings: Vec::new(),
    };
    let names: Vec<&str> = roster.iter().map(|(s, _)| s.as_str()).collect();
    for g in descriptors {
        if !names.contains(&g.as_str()) {
            v.findings.push(format!(
                "descriptor names no subcommand: {}.gate declares a gate the binary does not carry",
                g
            ));
        }
    }
    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the roster half speaks only for what
    // this tree declared: a subcommand from an unvendored kit, or consumer-declared elsewhere, is
    // out of scope rather than stranded, and it is counted rather than dropped silently
    for (s, owner) in roster {
        if !subcommand_in_scope(owner, kit_names, publishing) {
            v.out_of_scope += 1;
            continue;
        }
        v.in_scope += 1;
        if descriptors.iter().any(|d| d == s) {
            continue;
        }
        let quoted = format!("`{}`", s);
        if section_body
            .lines()
            .any(|l| l.contains(&quoted) && l.to_lowercase().contains("reference-only"))
        {
            v.refonly += 1;
            continue;
        }
        v.findings.push(format!(
            "subcommand nothing declares: the binary carries '{}' with no {}.gate descriptor and no reference-only disposition in {}",
            s, s, SECTION
        ));
    }
    v
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F's publishing test: this tree
// carries the crate's *tracked source*, which is what makes it the tree that declared the kits it
// carries. Source, so build output under the crate root cannot read as authorship.
fn authoring_tree(crate_dir: &str) -> bool {
    if !fresh::is_dir(crate_dir) {
        return false;
    }
    match proc::run("git", &["-C", crate_dir, "ls-files"]) {
        Ok(c) => c
            .stdout()
            .map(|o| !String::from_utf8_lossy(o).trim().is_empty())
            .unwrap_or(false),
        Err(_) => false,
    }
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion G's report, one finding per
// shape fault, in the gate's existing per-finding shape. One formatter for both corpora, so a
// malformed declaration reads the same wherever it sits.
fn shape_finding(path: &str, fault: &walk::ShapeFault) -> String {
    match fault {
        walk::ShapeFault::DoubledCause(n) => format!("more than one port declaration: {} carries {} '# no-port:' lines — a declaration carries at most one", path, n),
        walk::ShapeFault::EmptyCause => format!("port declaration with no cause: {} carries a bare '# no-port:' line — the cause names the ruling that makes the member permanent, and is the field's whole payload", path),
        walk::ShapeFault::DoubledHold(n) => format!("more than one hold declaration: {} carries {} '# port-until:' lines — a declaration carries at most one", path, n),
        walk::ShapeFault::EmptySlug => format!("hold declaration with no slug: {} carries a bare '# port-until:' line — the slug names the live queue entry that owns the blocker, and is the field's whole payload", path),
        walk::ShapeFault::BothFields => format!("contradictory port declarations: {} carries both '# no-port:' and '# port-until:' — permanent and temporarily-held are opposite verdicts about the same member", path),
    }
}

fn ind(line: &str) -> i64 {
    match line.bytes().position(|b| b != b' ') {
        Some(n) => n as i64,
        None => -1,
    }
}

fn matrix_key(line: &str) -> Option<bool> {
    let t = ltrim(line);
    let rest = t.strip_prefix("matrix:")?;
    Some(!ltrim(rest).is_empty())
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F, roster-derived matrix: every
// value in a matrix declaration is a GitHub expression, never a literal
fn matrix_literals(text: &str) -> (usize, Vec<(usize, String)>) {
    let mut declarations = 0usize;
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut inb = false;
    let mut keycol: i64 = -1;
    for (i, line) in section::split_lines(text).iter().enumerate() {
        let fnr = i + 1;
        if inb {
            if section::blank(line) {
                continue;
            }
            if ind(line) > keycol {
                if !ltrim(line).starts_with('#') && !line.contains("${{") {
                    out.push((fnr, (*line).to_string()));
                }
                continue;
            }
            inb = false;
        }
        match matrix_key(line) {
            Some(false) => {
                inb = true;
                keycol = ind(line);
                declarations += 1;
            }
            Some(true) => {
                declarations += 1;
                if !line.contains("${{") {
                    out.push((fnr, (*line).to_string()));
                }
            }
            None => {}
        }
    }
    (declarations, out)
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F, one producer per digest: a
// job computes at most one, and a job that downloads a run artifact and uploads none computes none
struct JobDigests {
    job: String,
    computed: usize,
    downloads: bool,
    uploads: bool,
}

fn uses_action(line: &str, action: &str) -> bool {
    let mut from = 0usize;
    while let Some(at) = line[from..].find("uses:") {
        let after = ltrim(&line[from + at + "uses:".len()..]);
        let token = after.split([' ', '\t']).next().unwrap_or("");
        if token.contains(action) {
            return true;
        }
        from += at + "uses:".len();
    }
    false
}

fn computes_digest(line: &str) -> bool {
    if ltrim(line).starts_with('#') || !line.contains("sha256sum") {
        return false;
    }
    let mut from = 0usize;
    while let Some(at) = line[from..].find("sha256sum") {
        let after = &line[from + at + "sha256sum".len()..];
        let spaced = after.trim_start_matches([' ', '\t']);
        if spaced.len() < after.len() && spaced.starts_with("-c") {
            return false;
        }
        from += at + "sha256sum".len();
    }
    true
}

fn job_name(line: &str) -> Option<String> {
    let rest = line.strip_prefix("  ")?;
    if rest.starts_with(' ') {
        return None;
    }
    let name: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if name.is_empty() {
        return None;
    }
    let tail = &rest[name.len()..];
    let tail = tail.strip_prefix(':')?;
    if trim(tail).is_empty() {
        Some(name)
    } else {
        None
    }
}

fn workflow_digests(text: &str) -> Vec<JobDigests> {
    let mut out: Vec<JobDigests> = Vec::new();
    let mut injobs = false;
    let mut cur: Option<JobDigests> = None;
    for line in section::split_lines(text) {
        if trim(line) == "jobs:" && line.starts_with("jobs:") {
            injobs = true;
            continue;
        }
        if injobs {
            if let Some(name) = job_name(line) {
                if let Some(j) = cur.take() {
                    out.push(j);
                }
                cur = Some(JobDigests {
                    job: name,
                    computed: 0,
                    downloads: false,
                    uploads: false,
                });
                continue;
            }
            let first = line.as_bytes().first().copied();
            if matches!(first, Some(b) if b != b' ' && b != b'\t' && b != b'#') {
                if let Some(j) = cur.take() {
                    out.push(j);
                }
                injobs = false;
            }
        }
        if let Some(j) = cur.as_mut() {
            if computes_digest(line) {
                j.computed += 1;
            }
            if uses_action(line, "download-artifact") {
                j.downloads = true;
            }
            if uses_action(line, "upload-artifact") {
                j.uploads = true;
            }
        }
    }
    if let Some(j) = cur.take() {
        out.push(j);
    }
    out
}

fn rule(args: &[String]) -> Result<i32, String> {
    let gates_dir = match args.first() {
        Some(a) => a.clone(),
        None => walk::knob_scalar("GATE_SDK_GATES_DIR")?,
    };
    let doc = match args.get(1) {
        Some(a) => a.clone(),
        None => format!(
            "{}/SPEC.md",
            walk::knob_scalar("GATE_SDK_ROOT_HERE")?.trim_end_matches('/')
        ),
    };
    let list = registry::list_path(&gates_dir);

    if !Path::new(&list).is_file() {
        return Err(format!("no registry at {}", list));
    }
    if !Path::new(&doc).is_file() {
        return Err(format!("conservation doc not found: {}", doc));
    }
    let members = registry::members(&read(&list)?);
    if members.is_empty() {
        return Err(format!("{} names no gates", list));
    }

    let kit_roots = walk::kit_roots_rel()?;
    let (resolve_dirs, kit_names) = main_rule_dirs(&gates_dir, &kit_roots);

    let section_body = conservation_body(&read(&doc)?);
    if section_body.is_empty() {
        return Err(format!("no '{}' section in {}", SECTION, doc));
    }

    let crate_dir = walk::knob_scalar("GATE_SDK_NATIVE_CRATE")?;
    let crate_dir = crate_dir.trim_end_matches('/').to_string();
    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the publishing test is computed once
    // and read twice: by assertion B's consumer-declared scope clause and assertion F's
    // missing-roster arm. One holder, shared with §check-gate-exemption-tasks' scope rule.
    let publishing = authoring_tree(&crate_dir);

    let mut ctx = Ctx {
        findings: Vec::new(),
    };

    // assertion A: each member resolves to exactly one declaration — a dir carrying
    // both <name>.sh and <name>.gate is ambiguous dispatch, never resolved by order
    let mut declared = 0usize;
    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — a descriptor on disk is a declaration;
    // a registered member resolving to one is a dispatch, and only a dispatch makes the binary
    // load-bearing. Derived here because assertion A already resolves every member.
    let mut dispatching = 0usize;
    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion G's two counters, derived in
    // assertion A's own loop because the declaration set and its spelling are exactly its output
    let mut declpaths_shell = 0usize;
    let mut noport_declared = 0usize;
    let mut portuntil_declared = 0usize;
    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion H's one new datum: the count
    // of held declarations whose ground was verified, read by the session choosing the next cohort
    // cut, for which a zero in a tree that declares holds is the vacuous-pass tell
    let mut portuntil_grounded = 0usize;
    let mut declpaths: Vec<String> = Vec::new();
    for m in &members {
        for d in &resolve_dirs {
            if Path::new(&format!("{}/{}.sh", d, m)).is_file()
                && Path::new(&format!("{}/{}.gate", d, m)).is_file()
            {
                ctx.findings.push(format!(
                    "ambiguous dispatch: {} carries both {}.sh and {}.gate",
                    d, m, m
                ));
            }
        }
        let Some(src) = registry::resolve(m, &resolve_dirs) else {
            ctx.findings.push(format!(
                "unresolvable member: {} declares in none of: {}",
                m,
                resolve_dirs.join(" ")
            ));
            continue;
        };
        declpaths.push(src.clone());
        if src.ends_with(".gate") {
            dispatching += 1;
        }
        declared += 1;

        let text = read(&src)?;
        // assertion G: the port declaration lives on the shell spelling and carries a cause —
        // a descriptor's existence is already the dispatch declaration, so a `# no-port:` line
        // there asserts the negation of the file it sits in
        // spec: gate-sdk/SPEC.md §check-gate-substrate-parity
        let noport = count_field(&text, "no-port");
        // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the held field's clauses are the
        // permanent field's over a second spelling, plus the one neither owns alone: a declaration
        // asserting both makes the two exclusion counts in port-blockers' trailer overlap
        let portuntil = count_field(&text, "port-until");
        if src.ends_with(".gate") {
            if noport > 0 {
                ctx.findings.push(format!("port declaration on a descriptor: {} carries a '# no-port:' line — a descriptor's existence is the dispatch declaration, so the field's domain is the <name>.sh spelling alone", src));
            }
            if portuntil > 0 {
                ctx.findings.push(format!("port declaration on a descriptor: {} carries a '# port-until:' line — a descriptor's existence is the dispatch declaration, so a ported member has no port question left to declare", src));
            }
            continue;
        }

        declpaths_shell += 1;
        // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — clauses 2 through 5 read the shared
        // shape verdict rather than a second implementation of it, over this half's whole-file scan
        // scope, which a widening may not narrow
        for fault in walk::shape(&text).faults {
            ctx.findings.push(shape_finding(&src, &fault));
        }
        if noport == 1 {
            noport_declared += 1;
        }
        if portuntil == 1 {
            portuntil_declared += 1;

            // assertion H: a held declaration's ground is reachable in one hop — the section
            // the declaration's own `# spec:` field names states the hold, so a reader arriving
            // at the declaration reaches the ground without resolving anything else
            // spec: gate-sdk/SPEC.md §check-gate-substrate-parity
            let specfield = first_payload(&text, "spec");
            let target = specfield
                .map(|t| trim(t.split(" — ").next().unwrap_or(t)))
                .unwrap_or("");
            if specfield.is_none() || !target.contains('§') {
                ctx.findings.push(format!("hold ground unreachable: {} carries '# port-until:' but no '# spec:' header field naming a section — a held member's ground lives in its own SPEC section, one hop from the declaration", src));
            } else {
                let (specpath, spechead) = target.split_once('§').unwrap_or((target, ""));
                let specpath = specpath.trim_end_matches([' ', '\t']);
                let spechead = ltrim(spechead);
                if std::fs::File::open(specpath).is_err() {
                    return Err(format!("{} points its '# spec:' field at {}, which is unreadable — assertion H could not read its corpus; treating as failure (not clean)", src, specpath));
                }
                let body = spec_section_body(&read(specpath)?, spechead);
                if body.contains("port-until") {
                    portuntil_grounded += 1;
                } else {
                    ctx.findings.push(format!("hold ground not in the pointed-at section: {} declares '# port-until:' and points at {} §{}, whose body never names the field — the section a reader reaches from the declaration is where the hold's ground lives", src, specpath, spechead));
                }
            }
        }
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion G's tree half: the corpus is
    // the union of the declaration set above and §port-blockers' tracked-shell-tree rule,
    // de-duplicated against the declaration set (assertion A resolves one set, so the sibling's
    // two-half dedup collapses to this). Scoped to the publishing tree on assertion F's own
    // predicate: a vendored kit's malformed cause is the kit author's to fix.
    let mut tree_scanned = 0usize;
    let mut tree_declared = 0usize;
    let mut tree_state = "out of scope here, this tree having authored no declaration it carries";
    if publishing {
        tree_state = "walked";
        for f in walk::tracked_shell_tree()? {
            if declpaths.contains(&f) {
                continue;
            }
            // spec: gate-sdk/SPEC.md §port-blockers — the header block alone over this corpus: it
            // carries scripts that *write* shell, and a line-anywhere scan cannot tell a
            // declaration from a heredoc literal
            let Ok(bytes) = std::fs::read(Path::new(&f)) else {
                continue;
            };
            let header = walk::header_block(&String::from_utf8_lossy(&bytes));
            tree_scanned += 1;
            let sh = walk::shape(&header);
            if sh.declared.is_some() {
                tree_declared += 1;
            }
            for fault in sh.faults {
                ctx.findings.push(shape_finding(&f, &fault));
            }
        }
    }

    // assertion B: the .gate descriptors on disk and the binary's own subcommand roster are the
    // same set, with 'reference-only' the one dispositioned exception
    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity
    let mut descriptors: Vec<String> = Vec::new();
    for d in &resolve_dirs {
        if !fresh::is_dir(d) {
            continue;
        }
        for f in walk::glob_entries(&format!("{}/*.gate", d)) {
            if !Path::new(&f).is_file() {
                continue;
            }
            let base = f.rsplit('/').next().unwrap_or(&f);
            if let Some(stem) = base.strip_suffix(".gate") {
                descriptors.push(stem.to_string());
            }
        }
    }
    descriptors.sort();
    descriptors.dedup();

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the roster is this binary's own
    // registry, read in process. The spawn the shell form needed is gone rather than answered:
    // a gate dispatched by the binary cannot skew against the roster that binary carries.
    let mut roster: Vec<(String, String)> = gates::names_with_owners()
        .into_iter()
        .map(|(n, o)| (n.to_string(), o.to_string()))
        .collect();
    roster.sort();
    roster.dedup();
    let verdict = roster_parity(
        &descriptors,
        &roster,
        &kit_names,
        publishing,
        &section_body,
    );
    ctx.findings.extend(verdict.findings);

    // assertion C: every derived substrate-sensitive member carries a disposition in
    // the conservation section — the anti-vacuity assertion, so a new meta-gate over
    // gate source reds until its disposition is recorded
    let mut sensitive = 0usize;
    for m in &members {
        let Some(src) = registry::resolve(m, &resolve_dirs) else {
            continue;
        };
        let text = read(&src)?;
        let Some(man) = registry::manifest_line(&text) else {
            continue;
        };
        let couples = registry::field(&registry::manifest_fields(man), "couples");
        if couples.is_empty() {
            continue;
        }
        let expanded = registry::expand_couples(&couples, &kit_roots);
        let hit = expanded
            .split(',')
            .any(|g| declpaths.iter().any(|p| walk::pattern_match(g, p)));
        if !hit {
            continue;
        }
        sensitive += 1;
        if !section_body.contains(&format!("`{}`", m)) {
            ctx.findings.push(format!("no recorded disposition: {} is substrate-sensitive (its couples= covers a gate declaration path) but {} does not name it", m, SECTION));
        }
    }

    // assertion D: manifest-class annotations live in the declaration only — a second
    // writable copy in the implementation is an SSOT violation that drifts silently,
    // and the manifest must stay readable with no build and no execution
    let impl_dir = walk::knob_scalar("GATE_SDK_NATIVE_SRC")?;
    let prune = walk::prune_dirs()?;
    let mut impl_scanned = 0usize;
    if fresh::is_dir(&impl_dir) {
        for f in walk::find_with_prune(Path::new(&impl_dir), &|n| prune.iter().any(|d| d == n))? {
            impl_scanned += 1;
            let p = f.display().to_string();
            let text = read(&p)?;
            for (i, line) in section::split_lines(&text).iter().enumerate() {
                if manifest_class(line) {
                    ctx.findings.push(format!("manifest-class annotation in implementation source: {}:{} — the '# graph:' manifest belongs to the declaration path alone", p, i + 1));
                }
            }
        }
    }

    // assertion E: opacity is held by structure — a ported gate's implementation source
    // may not reach the vendoring set, whose members are exactly the kit roots
    // spec: gate-sdk/SPEC.md §Consumer payload
    let mut kit_scanned = 0usize;
    for root in &kit_roots {
        let root = root.trim_end_matches('/');
        if crate_dir == root || crate_dir.starts_with(&format!("{}/", root)) {
            ctx.findings.push(format!("crate root inside the vendoring set: {} sits under kit root {} — a kit root vendors whole, so the implementation source would ship with it", crate_dir, root));
        }
        if descriptors.is_empty() || !fresh::is_dir(root) {
            continue;
        }
        kit_scanned += 1;
        for f in walk::find_with_prune(Path::new(root), &|n| prune.iter().any(|d| d == n))? {
            let p = f.display().to_string();
            let base = p.rsplit('/').next().unwrap_or(&p);
            let Some((stem, ext)) = base.rsplit_once('.') else {
                // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — an extensionless name is
                // out of reach by contract, and .gate/.sh are owned by the descriptor itself and
                // by assertion A
                continue;
            };
            if ext == "gate" || ext == "sh" {
                continue;
            }
            if descriptors.iter().any(|d| d == stem) {
                ctx.findings.push(format!("implementation sibling in the vendoring set: {} shares its name with the {}.gate descriptor — a ported gate's implementation may not sit under a kit root", p, stem));
            }
        }
    }

    // assertion F: the target roster has one owner and the publish path derives from it
    // — the roster is what asserts platform support, so a second spelling of it (a
    // platform literal in the build matrix) or a second producer of a published digest
    // is the failure this assertion exists to make impossible
    // spec: gate-sdk/SPEC.md §Consumer payload
    let roster_file = walk::knob_scalar("GATE_SDK_NATIVE_TARGETS_FILE")?;
    let mut roster_targets = 0usize;
    let mut roster_state = "absent";
    if Path::new(&roster_file).is_file() {
        roster_state = "read";
        let targets = registry::members(&read(&roster_file)?);
        if targets.is_empty() {
            ctx.findings.push(format!("empty target roster: {} declares no target — a roster asserting no platform support cannot be the surface that asserts it", roster_file));
        }
        for t in &targets {
            roster_targets += 1;
            if !well_formed_triple(t) {
                ctx.findings.push(format!(
                    "malformed target triple: '{}' in {} is not <arch>-<vendor>-<os>[-<env>]",
                    t, roster_file
                ));
            }
        }
    } else if dispatching > 0 && publishing {
        // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F rides the corrected
        // predicate *and* the publishing-tree test, a consumer receiving kit roots but never
        // the crate
        ctx.findings.push(format!("no target roster: {} is absent, but {} registered member(s) dispatch to the binary and {} carries tracked source here — a tree that builds the artifact declares the platforms it carries one for", roster_file, dispatching, crate_dir));
    }

    let workflow = walk::knob_scalar("GATE_SDK_NATIVE_PUBLISH_WORKFLOW")?;
    let mut wf_state = "absent";
    let mut wf_matrix = 0usize;
    let mut wf_jobs = 0usize;
    if Path::new(&workflow).is_file() {
        wf_state = "read";
        let text = read(&workflow)?;
        let (declarations, literals) = matrix_literals(&text);
        wf_matrix = declarations;
        for (lno, line) in literals {
            ctx.findings.push(format!("matrix declaration not roster-derived: {}:{} '{}' is a literal where an expression over {} belongs — a hand-written platform in a build matrix is a second spelling of the support commitment", workflow, lno, ltrim(&line), roster_file));
        }
        for j in workflow_digests(&text) {
            wf_jobs += 1;
            if j.computed > 1 {
                ctx.findings.push(format!("digest recomputed: job '{}' in {} computes {} digests — each is emitted once, where its bytes are produced, and moved thereafter", j.job, workflow, j.computed));
            } else if j.computed > 0 && j.downloads && !j.uploads {
                ctx.findings.push(format!("digest computed by a consumer: job '{}' in {} downloads a run artifact, produces none, and still computes a digest — it must move the sidecar it received, never re-derive it", j.job, workflow));
            }
        }
    }

    if !ctx.findings.is_empty() {
        println!("check-gate-substrate-parity: the gate substrate seam is not conserved:");
        for f in &ctx.findings {
            println!("  {}", f);
        }
        for line in help_lines(&doc) {
            println!("{}", line);
        }
        return Ok(1);
    }

    println!(
        "GATE-SUBSTRATE-PARITY: clean ({declared} member(s) with one declaration each, {dispatching} of them dispatching to the binary; {noport_declared} of the {declpaths_shell} shell declaration(s) declare '# no-port:' with a cause and {portuntil_declared} declare '# port-until:' with a slug, neither on any descriptor nor both on one declaration; the tracked shell tree beyond that set {tree_state}, {tree_scanned} file(s) read for header-declaration shape and {tree_declared} of them declaring, counted apart from the declaration set so an empty one stays visible; {portuntil_grounded} of those held declaration(s) reach their ground in one hop, the section their own '# spec:' field names stating the hold; {ndesc} descriptor(s) in parity with the {nsub}-subcommand roster ({in_scope} in scope, {out_of_scope} out of scope — an unvendored kit, or a consumer declaration from another tree), {refonly} reference-only; {sensitive} substrate-sensitive member(s) all dispositioned; {impl_scanned} implementation source(s) free of manifest-class annotation; {kit_scanned} kit root(s) scanned for an implementation sibling, crate root {crate_dir} outside every kit root; target roster {roster_state} at {roster_file} with {roster_targets} well-formed target(s); publish workflow {wf_state} at {workflow}, {wf_matrix} matrix declaration(s) roster-derived across {wf_jobs} job(s) with one producer per digest)",
        ndesc = descriptors.len(),
        nsub = roster.len(),
        in_scope = verdict.in_scope,
        out_of_scope = verdict.out_of_scope,
        refonly = verdict.refonly,
    );
    Ok(0)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the comment leader is matched as `#`, `//` or
// `/*`, so assertion D holds for whatever language sits behind a descriptor, not for Rust alone
fn manifest_class(line: &str) -> bool {
    let t = ltrim(line);
    for lead in ["#", "//", "/*"] {
        let Some(rest) = t.strip_prefix(lead) else {
            continue;
        };
        let rest = rest.trim_start_matches([' ', '\t']);
        if let Some(after) = rest.strip_prefix("graph:") {
            if after.starts_with([' ', '\t']) {
                return true;
            }
        }
    }
    false
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F's well-formedness:
// <arch>-<vendor>-<os>[-<env>], each segment word characters with dots allowed after the first
fn well_formed_triple(t: &str) -> bool {
    let parts: Vec<&str> = t.split('-').collect();
    if !(3..=4).contains(&parts.len()) {
        return false;
    }
    if parts[0].is_empty()
        || !parts[0]
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        return false;
    }
    parts[1..].iter().all(|p| {
        !p.is_empty()
            && p.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.')
    })
}

fn help_lines(doc: &str) -> Vec<String> {
    vec![
        "  help: one declaration per member — delete the stale .sh or .gate where a dir".to_string(),
        "        carries both. Keep the descriptor set and the binary's own subcommand roster equal:".to_string(),
        "        add the missing .gate, or drop the subcommand nothing declares — or,".to_string(),
        "        for an implementation deliberately kept ahead of any live port, give it".to_string(),
        format!("        a 'reference-only' disposition naming it in {}.", SECTION),
        "        A substrate-sensitive member with no disposition is recorded in".to_string(),
        format!("        {} {} — say ported, retained, or retired with cause;", doc, SECTION),
        "        an unrecorded one silently stops asserting when a gate ports.".to_string(),
        "  help: delete a manifest-class annotation from implementation source — the".to_string(),
        "        '# graph:' manifest has exactly one writable home, the declaration".to_string(),
        "        path, so that every reader of it works with no build and no execution.".to_string(),
        "  help: move a ported gate's implementation out of every kit root, and keep the".to_string(),
        "        crate root outside them too — a kit root vendors whole, so anything".to_string(),
        "        under one ships, and the payload withholds the predicate by structure.".to_string(),
        "  help: the target roster is the one surface asserting platform support — keep".to_string(),
        "        every live line a well-formed target triple, derive the publish".to_string(),
        "        workflow's matrix from it rather than spelling a platform there, and".to_string(),
        "        emit each artifact's digest in exactly one step, where its bytes are".to_string(),
        "        produced. A runner mapping may name a platform; a matrix may not.".to_string(),
        "  help: a '# no-port:' line declares a member the port will never take — it goes on".to_string(),
        "        the <name>.sh declaration only, at most once, and its cause names the SPEC".to_string(),
        "        section ruling the member permanent. A ported member has no port question".to_string(),
        "        left to declare, so drop the line rather than copying it into a descriptor.".to_string(),
        "  help: a '# port-until: <slug>' line declares a member the port still owes but cannot".to_string(),
        "        take yet, and names the live queue entry owning the blocker. Same domain and".to_string(),
        "        cardinality as '# no-port:', and never both on one declaration — permanent and".to_string(),
        "        temporarily-held are opposite verdicts about the same member.".to_string(),
        "  help: a held member's ground lives in its own SPEC section, one hop from the".to_string(),
        "        declaration — give the declaration a '# spec: <path> §<section>' field and".to_string(),
        "        state the hold in that section, naming 'port-until'. A ground reachable".to_string(),
        "        only through a queue entry or a shared worked-example passage is the one".to_string(),
        "        placement the field must not normalise.".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strs(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_string()).collect()
    }

    fn rows(v: &[(&str, &str)]) -> Vec<(String, String)> {
        v.iter()
            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
            .collect()
    }

    const DISPOSITION: &str = "| `check-reference` | Reference-only — carried by the binary with no descriptor. |";

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the subset vendoring: a subcommand
    // whose owning kit is absent is not a stranded implementation; it is out of scope, counted,
    // and said so. Every consumer is in this configuration once a second kit ports.
    #[test]
    fn a_subcommand_from_an_unvendored_kit_is_out_of_scope_and_counted() {
        let v = roster_parity(
            &strs(&["check-vendored"]),
            &rows(&[("check-vendored", "kitroot"), ("check-foreign", "otherkit")]),
            &strs(&["kitroot"]),
            false,
            "",
        );
        assert!(v.findings.is_empty(), "{:?}", v.findings);
        assert_eq!((v.in_scope, v.out_of_scope, v.refonly), (1, 1, 0));
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the near miss: the scoping must narrow
    // what the assertion speaks for and nothing else. A rule that scoped by "no descriptor" rather
    // than by owner passes the case above and this one too.
    #[test]
    fn an_in_scope_kit_missing_a_descriptor_still_reds() {
        let v = roster_parity(
            &strs(&["check-vendored"]),
            &rows(&[("check-vendored", "kitroot"), ("check-orphan", "kitroot")]),
            &strs(&["kitroot"]),
            false,
            "",
        );
        assert_eq!(v.findings.len(), 1, "{:?}", v.findings);
        assert!(v.findings[0].contains("the binary carries 'check-orphan' with no check-orphan.gate descriptor"));
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the consumer sentinel in an adopter,
    // which holds the subcommand the payload shipped and can never hold a descriptor for it
    #[test]
    fn a_consumer_declared_subcommand_is_out_of_scope_in_an_adopter() {
        let v = roster_parity(
            &strs(&["check-vendored"]),
            &rows(&[("check-vendored", "kitroot"), ("check-consumer", "-")]),
            &strs(&["kitroot"]),
            false,
            "",
        );
        assert!(v.findings.is_empty(), "{:?}", v.findings);
        assert_eq!((v.in_scope, v.out_of_scope), (1, 1));
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — and in scope in the publishing tree,
    // which is the only tree where a stranded implementation can be created: ruling these members
    // permanently out of scope would end, for the whole consumer-declared corpus, the one
    // assertion this half exists for.
    #[test]
    fn the_same_sentinel_is_in_scope_in_a_publishing_tree_and_clears_with_a_descriptor() {
        let red = roster_parity(
            &strs(&["check-vendored"]),
            &rows(&[("check-vendored", "kitroot"), ("check-consumer", "-")]),
            &strs(&["kitroot"]),
            true,
            "",
        );
        assert_eq!(red.findings.len(), 1, "{:?}", red.findings);
        assert!(red.findings[0].contains("the binary carries 'check-consumer' with no check-consumer.gate descriptor"));
        let green = roster_parity(
            &strs(&["check-consumer", "check-vendored"]),
            &rows(&[("check-vendored", "kitroot"), ("check-consumer", "-")]),
            &strs(&["kitroot"]),
            true,
            "",
        );
        assert!(green.findings.is_empty(), "{:?}", green.findings);
        assert_eq!((green.in_scope, green.out_of_scope), (2, 0));
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the other direction stays
    // unrestricted, the obvious implementation restricting one loop and accidentally both
    #[test]
    fn a_descriptor_naming_no_subcommand_reds_under_the_scoped_path() {
        let v = roster_parity(
            &strs(&["check-extra", "check-vendored"]),
            &rows(&[("check-vendored", "kitroot"), ("check-foreign", "otherkit")]),
            &strs(&["kitroot"]),
            false,
            "",
        );
        assert_eq!(v.findings.len(), 1, "{:?}", v.findings);
        assert!(v.findings[0].contains("descriptor names no subcommand: check-extra.gate"));
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the reference-only allowance, and the
    // post-revert configuration it is what keeps live: zero descriptors, where the roster half is
    // the only half with anything to say.
    #[test]
    fn a_reference_only_disposition_excuses_a_subcommand_nothing_declares() {
        let v = roster_parity(
            &[],
            &rows(&[("check-reference", "kitroot")]),
            &strs(&["kitroot"]),
            false,
            DISPOSITION,
        );
        assert!(v.findings.is_empty(), "{:?}", v.findings);
        assert_eq!((v.in_scope, v.refonly), (1, 1));
        let undisposed = roster_parity(
            &[],
            &rows(&[("check-reference", "kitroot")]),
            &strs(&["kitroot"]),
            false,
            "| `check-reference` | Ported. |",
        );
        assert_eq!(undisposed.findings.len(), 1, "{:?}", undisposed.findings);
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion H resolves the
    // declaration's own pointer and nothing else, over the two live heading levels
    #[test]
    fn a_pointed_at_section_is_read_at_its_own_depth() {
        let doc = "# top\n## check-held\nthe hold is port-until: blocked\n### deeper\nstill inside\n## next\nout\n";
        assert!(spec_section_body(doc, "check-held").contains("port-until"));
        assert!(spec_section_body(doc, "check-held").contains("still inside"));
        assert!(!spec_section_body(doc, "check-held").contains("out"));
        assert_eq!(spec_section_body(doc, "absent"), "");
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F, roster-derived matrix:
    // an expression passes, a literal is reported with its line, and the declaration is counted
    // either way
    #[test]
    fn a_matrix_literal_is_reported_and_an_expression_is_not() {
        let wf = "jobs:\n  build:\n    strategy:\n      matrix:\n        target: ${{ fromJson(x) }}\n        os: ubuntu-latest\n";
        let (n, lits) = matrix_literals(wf);
        assert_eq!(n, 1);
        assert_eq!(lits.len(), 1);
        assert!(lits[0].1.contains("ubuntu-latest"));
        assert_eq!(matrix_literals("      matrix: ${{ x }}\n").1.len(), 0);
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F, one producer per digest:
    // verification is unrestricted, and a job's own counters end at the next job
    #[test]
    fn a_digest_is_counted_per_job_and_verification_is_not_a_computation() {
        let wf = "jobs:\n  a:\n    steps:\n      - run: sha256sum x > x.sha256\n      - run: sha256sum y > y.sha256\n  b:\n    steps:\n      - uses: actions/download-artifact@v4\n      - run: sha256sum -c x.sha256\n";
        let jobs = workflow_digests(wf);
        assert_eq!(jobs.len(), 2);
        assert_eq!(jobs[0].computed, 2);
        assert_eq!(jobs[1].computed, 0);
        assert!(jobs[1].downloads && !jobs[1].uploads);
    }

    #[test]
    fn a_target_triple_takes_three_or_four_segments() {
        assert!(well_formed_triple("x86_64-unknown-linux-gnu"));
        assert!(well_formed_triple("aarch64-apple-darwin"));
        assert!(!well_formed_triple("linux"));
        assert!(!well_formed_triple("a-b-c-d-e"));
        assert!(!well_formed_triple("a-b-"));
    }
}
