// spec: installer/SPEC.md §The front door's verbs — the tables are the binary's rosters (A), and
// every advertised verb, flag, operand and arm is in the pinned release, or pending its release (B)
use super::pinned_release::{self, read, Disposition};
use crate::knobfile::{self, Form};
use crate::{fresh, walk};
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-front-door-verbs";
const RECIPE_KNOB: &str = "GATE_SDK_PAYLOAD_RECIPES";
const ROUTES: &[&str] = &["sh -s --", "install.ps1)))", "npx checkwright"];
const CODE_ROUTE: &str = "checkwright";
// spec: installer/SPEC.md §The front door's verbs — the one flag a route may lead with
const HELP_FLAGS: &[&str] = &["--help", "-h"];
// spec: installer/SPEC.md §The front door's verbs — the tokens that end a verb's flag walk
const STOPS: &[&str] = &["|", ";", "&&", "||", ")"];
// spec: installer/SPEC.md §The front door's verbs — the two flags whose operand is held to a roster
const PROFILE_FLAG: &str = "--profile";
const RECIPE_FLAG: &str = "--recipe";
const EMIT: &str = "--emit";

type Pair = (String, String);
type Flag = (String, Option<String>);

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

fn binary_verbs() -> BTreeSet<String> {
    crate::installer::VERBS
        .iter()
        .map(|(v, _)| v.trim_start_matches("--").to_string())
        .collect()
}

fn binary_flags() -> BTreeSet<Pair> {
    crate::installer::FLAGS
        .iter()
        .flat_map(|(f, vs)| vs.iter().map(move |v| (v.to_string(), f.to_string())))
        .collect()
}

fn binary_arms() -> BTreeSet<String> {
    crate::emit::arms()
        .into_iter()
        .chain(crate::TOP_LEVEL_FLAGS.iter().copied())
        .map(str::to_string)
        .collect()
}

// spec: installer/SPEC.md §The front door's verbs — HEAD's tree or the tag's, each read at the same
// repo-relative paths, so a roster read off either side is one parse
enum Tree {
    Dir(String),
    Tag(String),
}

impl Tree {
    fn label(&self) -> String {
        match self {
            Tree::Dir(d) | Tree::Tag(d) => d.clone(),
        }
    }

    fn path(&self, rel: &str) -> String {
        match self {
            Tree::Dir(d) if d == "." => rel.to_string(),
            Tree::Dir(d) => format!("{}/{}", d.trim_end_matches('/'), rel),
            Tree::Tag(t) => format!("{}:{}", t, rel),
        }
    }

    fn get(&self, rel: &str) -> Result<Option<String>, String> {
        match self {
            Tree::Dir(_) => {
                let p = self.path(rel);
                if Path::new(&p).is_file() {
                    Ok(Some(read(&p)?))
                } else {
                    Ok(None)
                }
            }
            Tree::Tag(t) => {
                if pinned_release::carries(t, rel)? {
                    Ok(Some(pinned_release::show_at(t, rel)?))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn required(&self, rel: &str) -> Result<String, String> {
        match self {
            Tree::Dir(_) => read(&self.path(rel)),
            Tree::Tag(t) => pinned_release::show_at(t, rel),
        }
    }
}

// spec: installer/SPEC.md §The front door's verbs — the gate-sdk knob seam file, in the directory
// `GATE_SDK_GATES_DIR` names
fn seam_path() -> Result<String, String> {
    let kit = crate::knobs::owner(RECIPE_KNOB).ok_or_else(|| format!("no static kit owns {}", RECIPE_KNOB))?;
    let dir = crate::walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    Ok(format!("{}/{}-config.knobs", dir.trim_end_matches('/'), kit.stem()))
}

// spec: installer/SPEC.md §The front door's verbs — the profile set: the roster's names with the
// derived one; a tree lacking the roster has the empty set
fn profile_set(text: Option<String>) -> BTreeSet<String> {
    text.map(|t| crate::installer::profile::names_in(&t).into_iter().collect())
        .unwrap_or_default()
}

// spec: installer/SPEC.md §The front door's verbs — the recipe set: the keys of the recipe knob's
// lines in the seam file; a tree lacking the file has the empty set
fn recipe_set(label: &str, text: Option<String>) -> Result<BTreeSet<String>, String> {
    let Some(text) = text else { return Ok(BTreeSet::new()) };
    Ok(knobfile::parse(&text, label)?
        .into_iter()
        .filter(|e| e.name == RECIPE_KNOB)
        .filter_map(|e| match e.form {
            Form::Keyed(k) => Some(k),
            _ => None,
        })
        .collect())
}

// spec: installer/SPEC.md §The front door's verbs — each string literal of the array `decl` names,
// with its depth and the ordinal of the top-level element it sits in; comments and char literals
// skipped. None where the declaration is absent or its array never closes.
fn array_literals(src: &str, decl: &str) -> Option<Vec<(usize, usize, String)>> {
    let at = src.find(&format!("const {}:", decl))?;
    let eq = at + src[at..].find('=')?;
    let open = eq + src[eq..].find('[')?;
    let b = src.as_bytes();
    let (mut i, mut depth, mut ordinal) = (open, 0usize, 0usize);
    let mut out = Vec::new();
    while i < b.len() {
        match b[i] {
            b'[' | b'(' | b'{' => {
                depth += 1;
                if depth == 2 {
                    ordinal += 1;
                }
            }
            b']' | b')' | b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(out);
                }
            }
            b'/' if b.get(i + 1) == Some(&b'/') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                i += src[i + 2..].find("*/")? + 3;
            }
            b'\'' if b.get(i + 1) == Some(&b'\\') => {
                i += src[i + 2..].find('\'')? + 2;
            }
            b'\'' if b.get(i + 2) == Some(&b'\'') => i += 2,
            b'"' => {
                let mut s = String::new();
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    if b[i] == b'\\' {
                        i += 1;
                    }
                    s.push(*b.get(i)? as char);
                    i += 1;
                }
                if depth == 1 || !out.iter().any(|(_, o, _)| *o == ordinal) {
                    out.push((depth, ordinal, s));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

// spec: installer/SPEC.md §The front door's verbs — the arm table off the two source arrays: the
// first literal of each top-level tuple of `ARMS`, with every literal of `TOP_LEVEL_FLAGS`; None
// where either array is absent or yields no member
fn arm_table(emit_src: &str, main_src: &str) -> Option<BTreeSet<String>> {
    let arms: Vec<String> = array_literals(emit_src, "ARMS")?
        .into_iter()
        .filter(|(d, _, _)| *d >= 2)
        .map(|(_, _, s)| s)
        .collect();
    let top: Vec<String> = array_literals(main_src, "TOP_LEVEL_FLAGS")?
        .into_iter()
        .map(|(_, _, s)| s)
        .collect();
    if arms.is_empty() || top.is_empty() {
        return None;
    }
    Some(arms.into_iter().chain(top).collect())
}

// spec: installer/SPEC.md §The front door's verbs — the repo-relative files each tree is read at:
// the installer's README and profile roster, the crate's two arm sources, and the page section
// describing a checkout, whose binary is built from the tree, so it is not read for arms
struct Layout {
    readme: String,
    profiles: String,
    emit_src: String,
    main_src: String,
    checkout: (String, String),
}

fn layout() -> Result<Layout, String> {
    Ok(Layout {
        // consumer-value-exempt: a file name under the installer directory knob, no layout of its own
        readme: fresh::knob_joined("GATE_LOCAL_INSTALLER_DIR", "README.md")?,
        profiles: fresh::knob_joined("GATE_LOCAL_INSTALLER_DIR", "profiles.list")?,
        emit_src: fresh::knob_joined("GATE_SDK_NATIVE_SRC", "emit/mod.rs")?,
        main_src: fresh::knob_joined("GATE_SDK_NATIVE_SRC", "main.rs")?,
        checkout: (
            walk::knob_scalar("GATE_LOCAL_README_FILE")?,
            walk::knob_scalar("GATE_LOCAL_CHECKOUT_SECTION")?,
        ),
    })
}

// spec: installer/SPEC.md §The front door's verbs — a tree carrying neither source file predates
// the gate binary and has the empty arm set; one carrying either must yield both arrays
fn pinned_arms(tree: &Tree, l: &Layout) -> Result<BTreeSet<String>, String> {
    let (emit_src, main_src) = (l.emit_src.as_str(), l.main_src.as_str());
    let (emit, main) = (tree.get(emit_src)?, tree.get(main_src)?);
    if emit.is_none() && main.is_none() {
        return Ok(BTreeSet::new());
    }
    arm_table(emit.as_deref().unwrap_or(""), main.as_deref().unwrap_or("")).ok_or_else(|| {
        format!(
            "{} carries no `ARMS` array in {} or no `TOP_LEVEL_FLAGS` array in {}, or one yielding no member, so its arm set could not be read",
            tree.label(),
            emit_src,
            main_src
        )
    })
}

fn cells(line: &str) -> Vec<&str> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    t.split('|').map(str::trim).collect()
}

fn code_span(cell: &str) -> Option<&str> {
    cell.strip_prefix('`').and_then(|s| s.strip_suffix('`'))
}

// spec: installer/SPEC.md §The front door's verbs — the body rows of the table whose header row's
// first cell is `header`; one reader for the HEAD file and the tag's blob alike
fn table_rows<'a>(text: &'a str, header: &str) -> Option<Vec<Vec<&'a str>>> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim_start().starts_with('|') && cells(l).first() == Some(&header))?;
    Some(
        lines[start + 1..]
            .iter()
            .take_while(|l| l.trim_start().starts_with('|'))
            .map(|l| cells(l))
            .collect(),
    )
}

fn verb_table(text: &str) -> Option<Vec<String>> {
    let rows = table_rows(text, "verb")?;
    Some(
        rows.iter()
            .filter_map(|r| code_span(r.first().copied().unwrap_or("")))
            .map(str::to_string)
            .collect(),
    )
}

// spec: installer/SPEC.md §The front door's verbs — one pair per verb the second column lists
// beside the first column's flag
fn flag_table(text: &str) -> Option<Vec<Pair>> {
    let rows = table_rows(text, "flag")?;
    let mut out = Vec::new();
    for r in rows {
        let Some(flag) = code_span(r.first().copied().unwrap_or("")) else { continue };
        for v in r.get(1).copied().unwrap_or("").split(',') {
            if let Some(v) = code_span(v.trim()) {
                out.push((v.to_string(), flag.to_string()));
            }
        }
    }
    Some(out)
}

// spec: installer/SPEC.md §The front door's verbs — a page's code: each inline code span, and each
// line of a fenced block
fn code_texts(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut fenced = crate::spec::Fence::with_tilde();
    for (i, line) in text.lines().enumerate() {
        if fenced.delimits(line) {
            continue;
        }
        if fenced.is_open() {
            out.push((i + 1, line));
            continue;
        }
        for (k, seg) in line.split('`').enumerate() {
            if k % 2 == 1 {
                out.push((i + 1, seg));
            }
        }
    }
    out
}

// spec: installer/SPEC.md §The front door's verbs — the arms a page names: each inline code span
// outside a fence and outside the checkout section whose first token opens with `--` and is no
// flag, `--emit <name>` read as `--emit-<name>`
fn advertised_arms(page: &str, text: &str, flags: &BTreeSet<String>, checkout: (&str, &str)) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let (mut fenced, mut in_checkout) = (crate::spec::Fence::with_tilde(), false);
    for (i, line) in text.lines().enumerate() {
        let t = line.trim_start();
        if fenced.delimits(line) {
            continue;
        }
        if fenced.is_open() {
            continue;
        }
        if t.starts_with("# ") || t.starts_with("## ") {
            in_checkout = page == checkout.0 && line.trim_end() == checkout.1;
            continue;
        }
        if in_checkout {
            continue;
        }
        for (k, seg) in line.split('`').enumerate() {
            if k % 2 == 1 {
                out.extend(arm_of(seg, flags).map(|a| (i + 1, a)));
            }
        }
    }
    out
}

fn arm_of(span: &str, flags: &BTreeSet<String>) -> Option<String> {
    let mut words = span.split_whitespace();
    let first = words.next()?;
    let name = first.split('=').next().unwrap_or(first);
    if !name.starts_with("--") || flags.contains(name) {
        return None;
    }
    if name == EMIT {
        let member = words.next().filter(|w| is_operand(w))?;
        return Some(format!("{}-{}", EMIT, member));
    }
    Some(name.to_string())
}

#[derive(Debug, PartialEq)]
enum Tok {
    Verb(String, Vec<Flag>),
    Flag(String),
    Placeholder,
}

fn is_verb(t: &str) -> bool {
    t.as_bytes().first().is_some_and(u8::is_ascii_lowercase) && is_operand(t)
}

fn is_operand(t: &str) -> bool {
    let b = t.as_bytes();
    !b.is_empty()
        && (b[0].is_ascii_lowercase() || b[0].is_ascii_digit())
        && b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}

fn is_stop(w: &str) -> bool {
    STOPS.contains(&w) || w.starts_with('#') || w.starts_with('>')
}

// spec: installer/SPEC.md §The front door's verbs — the flags after an advertised verb, read to the
// first stop token, each cut at its first `=`, a `--profile` or `--recipe` carrying its operand
fn flags_after(words: &[&str]) -> Vec<Flag> {
    let mut out = Vec::new();
    for (i, w) in words.iter().enumerate() {
        if is_stop(w) {
            break;
        }
        if !w.starts_with('-') || HELP_FLAGS.contains(w) {
            continue;
        }
        let (flag, value) = match w.split_once('=') {
            Some((f, v)) => (f, Some(v)),
            None => (*w, words.get(i + 1).copied().filter(|t| !is_stop(t) && !t.starts_with('-'))),
        };
        let operand = value
            .filter(|v| (flag == PROFILE_FLAG || flag == RECIPE_FLAG) && is_operand(v))
            .map(str::to_string);
        out.push((flag.to_string(), operand));
    }
    out
}

fn token(after: &str) -> Option<Tok> {
    if !after.starts_with(char::is_whitespace) {
        return None;
    }
    let words: Vec<&str> = after.split_whitespace().collect();
    let t = *words.first()?;
    Some(if HELP_FLAGS.contains(&t) {
        Tok::Placeholder
    } else if t.starts_with('-') {
        Tok::Flag(t.to_string())
    } else if is_verb(t) {
        Tok::Verb(t.to_string(), flags_after(&words[1..]))
    } else {
        Tok::Placeholder
    })
}

// spec: installer/SPEC.md §The front door's verbs — the token after each route in one code text,
// `checkwright` routing only where it opens the text
fn advertised(code: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    for route in ROUTES {
        let mut from = 0;
        while let Some(i) = code[from..].find(route) {
            let end = from + i + route.len();
            out.extend(token(&code[end..]));
            from = end;
        }
    }
    if let Some(after) = code.trim_start().strip_prefix(CODE_ROUTE) {
        out.extend(token(after));
    }
    out
}

fn table(label: &str, text: &str) -> Result<Vec<String>, String> {
    match verb_table(text) {
        Some(v) if !v.is_empty() => Ok(v),
        _ => Err(format!(
            "{} carries no verb table (a table whose header row's first cell is 'verb', listing code-spanned verbs)",
            label
        )),
    }
}

fn head_flags(label: &str, text: &str) -> Result<BTreeSet<Pair>, String> {
    match flag_table(text) {
        Some(v) if !v.is_empty() => Ok(v.into_iter().collect()),
        _ => Err(format!(
            "{} carries no flag table (a table whose header row's first cell is 'flag', listing a code-spanned flag beside its code-spanned verbs)",
            label
        )),
    }
}

struct Pinned {
    label: String,
    verbs: BTreeSet<String>,
    flags: BTreeSet<Pair>,
    profiles: BTreeSet<String>,
    recipes: BTreeSet<String>,
    arms: BTreeSet<String>,
}

// spec: installer/SPEC.md §The front door's verbs — the pending admission, one rule for every
// advertisement
enum Verdict<'a> {
    Released,
    Pending,
    Withheld(&'a str),
    Unknown,
}

fn verdict(disp: &Disposition, pinned: bool, head: bool) -> Verdict<'_> {
    if pinned {
        return Verdict::Released;
    }
    if !head {
        return Verdict::Unknown;
    }
    match disp {
        Disposition::Absent | Disposition::Release => Verdict::Pending,
        Disposition::Withheld(field) => Verdict::Withheld(field),
    }
}

#[derive(Default)]
struct Findings {
    verbs: Vec<String>,
    flags: Vec<String>,
    operands: Vec<String>,
    arms: Vec<String>,
    pending_verbs: BTreeSet<String>,
    pending_flags: BTreeSet<String>,
    pending_operands: BTreeSet<String>,
    pending_arms: BTreeSet<String>,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let positional = !args.is_empty();
    if positional && args.len() < 5 {
        return Err("usage: check-front-door-verbs [head pinned disposition queue page...]".to_string());
    }
    let head = Tree::Dir(if positional { args[0].clone() } else { ".".to_string() });
    // spec: installer/SPEC.md §The front door's verbs — the pinned tree is the tag the hosted pin
    // names, or the positional directory; an unresolvable tag leaves B dormant
    let pinned_tree: Option<Tree> = if positional {
        Some(Tree::Dir(args[1].clone()))
    } else {
        pinned_release::pinned_tag()?.map(Tree::Tag)
    };
    let (disposition_path, queue_path) = if positional {
        (args[2].clone(), args[3].clone())
    } else {
        pinned_release::live_paths()?
    };
    let pages: Vec<String> = if positional {
        args[4..].to_vec()
    } else {
        walk::knob_array("GATE_LOCAL_FRONT_DOOR_SURFACES")?
    };
    let l = layout()?;

    let (iteration, disp) = pinned_release::iteration_disposition(&disposition_path, &queue_path)?;
    let seam = seam_path()?;

    let binary = binary_verbs();
    let binary_flags = binary_flags();
    let binary_arms = binary_arms();
    let readme = head.path(&l.readme);
    let readme_text = head.required(&l.readme)?;
    let head_verbs: BTreeSet<String> = table(&readme, &readme_text)?.into_iter().collect();
    let head_flags = head_flags(&readme, &readme_text)?;
    let head_profiles = profile_set(head.get(&l.profiles)?);
    let head_recipes = recipe_set(&head.path(&seam), head.get(&seam)?)?;
    // spec: installer/SPEC.md §The front door's verbs — a tag carrying no flag table, roster or seam
    // file has the empty set for it, never a dormancy
    let pinned: Option<Pinned> = match &pinned_tree {
        Some(t) => {
            let label = t.label();
            let text = t.required(&l.readme)?;
            Some(Pinned {
                verbs: table(&label, &text)?.into_iter().collect(),
                flags: flag_table(&text).unwrap_or_default().into_iter().collect(),
                profiles: profile_set(t.get(&l.profiles)?),
                recipes: recipe_set(&t.path(&seam), t.get(&seam)?)?,
                arms: pinned_arms(t, &l)?,
                label,
            })
        }
        None => None,
    };
    let flag_names: BTreeSet<String> = binary_flags
        .iter()
        .chain(pinned.iter().flat_map(|p| p.flags.iter()))
        .map(|(_, f)| f.clone())
        .collect();

    let mut a_findings: Vec<String> = Vec::new();
    for v in binary.difference(&head_verbs) {
        a_findings.push(format!("  {}: the verb table lacks `{}`, which the binary's VERBS carries", readme, v));
    }
    for v in head_verbs.difference(&binary) {
        a_findings.push(format!("  {}: the verb table lists `{}`, which the binary's VERBS does not carry", readme, v));
    }
    for (v, f) in binary_flags.difference(&head_flags) {
        a_findings.push(format!("  {}: the flag table lacks `{}` for `{}`, which the binary's FLAGS carries", readme, f, v));
    }
    for (v, f) in head_flags.difference(&binary_flags) {
        a_findings.push(format!("  {}: the flag table lists `{}` for `{}`, which the binary's FLAGS does not carry", readme, f, v));
    }

    let mut b = Findings::default();
    let mut lead_findings: Vec<String> = Vec::new();
    let (mut sites, mut flag_sites, mut operand_sites, mut arm_sites) = (0usize, 0usize, 0usize, 0usize);
    for page in &pages {
        let text = read(page)?;
        for (n, code) in code_texts(&text) {
            for tok in advertised(code) {
                if let Tok::Flag(f) = &tok {
                    lead_findings.push(format!(
                        "  {}:{}: a route leads with `{}`, which reaches the binary as its first argument rather than a verb's",
                        page, n, f
                    ));
                    continue;
                }
                let Tok::Verb(v, flags) = tok else { continue };
                sites += 1;
                flag_sites += flags.len();
                operand_sites += flags.iter().filter(|(_, o)| o.is_some()).count();
                let Some(p) = &pinned else { continue };
                match verdict(&disp, p.verbs.contains(&v), binary.contains(&v)) {
                    Verdict::Released => {}
                    Verdict::Pending => {
                        b.pending_verbs.insert(v.clone());
                    }
                    Verdict::Withheld(field) => b.verbs.push(format!(
                        "  {}:{}: `{}` is advertised, and the pinned release {} lacks it while iteration {}'s disposition is {}",
                        page, n, v, p.label, iteration, field
                    )),
                    Verdict::Unknown => b.verbs.push(format!(
                        "  {}:{}: `{}` is advertised, and neither the pinned release {} nor the binary's VERBS carries it",
                        page, n, v, p.label
                    )),
                }
                for (f, operand) in flags {
                    let pair = (v.clone(), f.clone());
                    match verdict(&disp, p.flags.contains(&pair), binary_flags.contains(&pair)) {
                        Verdict::Released => {}
                        Verdict::Pending => {
                            b.pending_flags.insert(format!("{} {}", v, f));
                        }
                        Verdict::Withheld(field) => b.flags.push(format!(
                            "  {}:{}: `{} {}` is advertised, and the pinned release {} lacks it while iteration {}'s disposition is {}",
                            page, n, v, f, p.label, iteration, field
                        )),
                        Verdict::Unknown => b.flags.push(format!(
                            "  {}:{}: `{} {}` is advertised, and neither the pinned release {} nor the binary's FLAGS carries it",
                            page, n, v, f, p.label
                        )),
                    }
                    let Some(op) = operand else { continue };
                    let (kind, pinned_set, head_set) = if f == PROFILE_FLAG {
                        ("profile", &p.profiles, &head_profiles)
                    } else {
                        ("recipe", &p.recipes, &head_recipes)
                    };
                    match verdict(&disp, pinned_set.contains(&op), head_set.contains(&op)) {
                        Verdict::Released => {}
                        Verdict::Pending => {
                            b.pending_operands.insert(format!("{} {}", f, op));
                        }
                        Verdict::Withheld(field) => b.operands.push(format!(
                            "  {}:{}: `{} {}` is advertised, and the pinned release {} lacks the {} `{}` while iteration {}'s disposition is {}",
                            page, n, f, op, p.label, kind, op, iteration, field
                        )),
                        Verdict::Unknown => b.operands.push(format!(
                            "  {}:{}: `{} {}` is advertised, and neither the pinned release {} nor HEAD's {} set carries `{}`",
                            page, n, f, op, p.label, kind, op
                        )),
                    }
                }
            }
        }
        for (n, arm) in advertised_arms(page, &text, &flag_names, (&l.checkout.0, &l.checkout.1)) {
            arm_sites += 1;
            let Some(p) = &pinned else { continue };
            match verdict(&disp, p.arms.contains(&arm), binary_arms.contains(&arm)) {
                Verdict::Released => {}
                Verdict::Pending => {
                    b.pending_arms.insert(arm);
                }
                Verdict::Withheld(field) => b.arms.push(format!(
                    "  {}:{}: the arm `{}` is advertised, and the pinned release {} lacks it while iteration {}'s disposition is {}",
                    page, n, arm, p.label, iteration, field
                )),
                Verdict::Unknown => b.arms.push(format!(
                    "  {}:{}: the arm `{}` is advertised, and neither the pinned release {} nor the binary's arm table carries it",
                    page, n, arm, p.label
                )),
            }
        }
    }

    let red = [&b.verbs, &b.flags, &b.operands, &b.arms, &lead_findings, &a_findings];
    if red.iter().any(|f| !f.is_empty()) {
        println!("{}: the front door advertises a verb, flag, operand or arm its pinned release does not carry, leads a route with a flag, or a table is not the binary's roster (installer/SPEC.md §The front door's verbs):", NAME);
        for f in red.iter().flat_map(|f| f.iter()) {
            println!("{}", f);
        }
        if !b.verbs.is_empty() {
            println!("  help: release the verb so the pin carries it (RELEASING.md), or withdraw the advertisement from the page.");
        }
        if !b.flags.is_empty() {
            println!("  help: release the flag so the pin carries it (RELEASING.md), or withdraw it from the page.");
        }
        if !b.operands.is_empty() {
            println!("  help: release the profile or recipe so the pin carries it (RELEASING.md), or withdraw it from the page.");
        }
        if !b.arms.is_empty() {
            println!("  help: release the arm so the pin carries it (RELEASING.md), or withdraw it from the page.");
        }
        if !lead_findings.is_empty() {
            println!("  help: name the verb before its flags, as `sh -s -- init --profile prose`: the one-line install runs `init` only on an empty argument list, and the bootstrap forwards a leading flag unchanged.");
        }
        if !a_findings.is_empty() {
            println!("  help: make {}'s verb table list exactly the verbs, and its flag table exactly the flag-verb pairs, that native/src/installer/mod.rs's VERBS and FLAGS carry.", readme);
        }
        return Ok(1);
    }
    let pinned_state = match &pinned {
        Some(p) => format!("pinned set {}", p.label),
        None => "B dormant — the pinned tag does not resolve here".to_string(),
    };
    let mut pending_state = String::new();
    for (label, set, sep) in [
        ("pending release", &b.pending_verbs, " "),
        ("pending flags", &b.pending_flags, ", "),
        ("pending operands", &b.pending_operands, ", "),
        ("pending arms", &b.pending_arms, " "),
    ] {
        if !set.is_empty() {
            pending_state.push_str(&format!(", {}: {}", label, set.iter().cloned().collect::<Vec<_>>().join(sep)));
        }
    }
    println!(
        "FRONT-DOOR-VERBS: clean ({} page(s), {} advertised site(s), {} advertised flag(s), {} advertised operand(s), {} advertised arm(s), {}, tables equal to VERBS and FLAGS{})",
        pages.len(),
        sites,
        flag_sites,
        operand_sites,
        arm_sites,
        pinned_state,
        pending_state
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Tok {
        Tok::Verb(s.to_string(), Vec::new())
    }

    fn vf(s: &str, flags: &[(&str, Option<&str>)]) -> Tok {
        Tok::Verb(
            s.to_string(),
            flags.iter().map(|(f, o)| (f.to_string(), o.map(str::to_string))).collect(),
        )
    }

    // spec: installer/SPEC.md §The front door's verbs — each route, a leading flag, the help flag, a
    // placeholder, and a token outside any code text
    #[test]
    fn the_route_tokenizer_reads_each_route_a_flag_and_a_placeholder() {
        assert_eq!(advertised("curl -fsSL x | sh -s -- demo   # comment"), vec![v("demo")]);
        assert_eq!(advertised("& ([scriptblock]::Create((irm x/install.ps1))) doctor"), vec![v("doctor")]);
        assert_eq!(advertised("npx checkwright init"), vec![v("init")]);
        assert_eq!(advertised("checkwright uninstall"), vec![v("uninstall")]);
        assert_eq!(advertised("sh -s -- --profile full"), vec![Tok::Flag("--profile".to_string())]);
        assert_eq!(advertised("checkwright --help"), vec![Tok::Placeholder]);
        assert_eq!(advertised("checkwright <verb>"), vec![Tok::Placeholder]);
        assert_eq!(advertised("checkwright"), vec![]);
        assert_eq!(advertised("checkwright.lock"), vec![]);
        assert_eq!(advertised("npx checkwright-pwsh init"), vec![]);
        let page = "run npx checkwright demo here, or `npx checkwright init`\n```sh\nsh -s -- diff\ncheckwright doctor\n```\n";
        let found: Vec<(usize, Tok)> = code_texts(page)
            .into_iter()
            .flat_map(|(n, c)| advertised(c).into_iter().map(move |t| (n, t)))
            .collect();
        assert_eq!(found, vec![(1, v("init")), (3, v("diff")), (4, v("doctor"))]);
    }

    // spec: installer/SPEC.md §The front door's verbs — the flags after a verb, cut at `=`, the help
    // arm read as none, and the walk ending at a stop token
    #[test]
    fn the_flag_walk_reads_to_the_first_stop_token() {
        assert_eq!(advertised("checkwright init --help --dry-run"), vec![vf("init", &[("--dry-run", None)])]);
        assert_eq!(advertised("sh -s -- init --force | tee log --no"), vec![vf("init", &[("--force", None)])]);
        assert_eq!(advertised("checkwright init --force # --no-commit"), vec![vf("init", &[("--force", None)])]);
        assert_eq!(advertised("checkwright init > out --x"), vec![v("init")]);
        assert_eq!(
            advertised("checkwright init --force && git log --oneline"),
            vec![vf("init", &[("--force", None)])]
        );
        assert_eq!(advertised("checkwright <verb> --force"), vec![Tok::Placeholder]);
    }

    // spec: installer/SPEC.md §The front door's verbs — the operand of `--profile` and `--recipe`,
    // in its `=` and next-token forms; a placeholder, a stop token and a following flag carry none,
    // and no other flag's value is read
    #[test]
    fn the_operand_walk_reads_a_profile_or_recipe_value() {
        assert_eq!(
            advertised("sh -s -- init --profile full --recipe=speckit"),
            vec![vf("init", &[("--profile", Some("full")), ("--recipe", Some("speckit"))])]
        );
        assert_eq!(advertised("checkwright init --profile <profile>"), vec![vf("init", &[("--profile", None)])]);
        assert_eq!(advertised("checkwright init --recipe=<name>"), vec![vf("init", &[("--recipe", None)])]);
        assert_eq!(advertised("checkwright init --profile | x"), vec![vf("init", &[("--profile", None)])]);
        assert_eq!(advertised("checkwright init --profile"), vec![vf("init", &[("--profile", None)])]);
        assert_eq!(
            advertised("checkwright init --recipe --force"),
            vec![vf("init", &[("--recipe", None), ("--force", None)])]
        );
        assert_eq!(
            advertised("checkwright init --with-kit drift-kit"),
            vec![vf("init", &[("--with-kit", None)])]
        );
    }

    // spec: installer/SPEC.md §The front door's verbs — an inline span's leading `--` token, `--emit
    // <name>` normalized, an installer flag and a placeholder skipped, fenced lines and the checkout
    // section unread
    #[test]
    fn the_arm_read_takes_inline_spans_outside_the_checkout_section() {
        let flags: BTreeSet<String> = ["--profile".to_string()].into_iter().collect();
        let page = "# p\n\nRun `--emit env-probe`, `--measure-commit` or `--run x`; pass `--profile prose`.\n\
                    Not `--emit <name>`, nor `checkwright --run`.\n\n```sh\n--usage-poll\n```\n\n\
                    ## This repo, governed\n\n`--install-hooks`\n\n### Sub\n\n`--run-demo`\n\n## After\n\n`--list`\n";
        let arms = |p: &str| -> Vec<(usize, String)> { advertised_arms(p, page, &flags, ("README.md", "## This repo, governed")) };
        let s = |n: usize, a: &str| (n, a.to_string());
        let outside = vec![s(3, "--emit-env-probe"), s(3, "--measure-commit"), s(3, "--run"), s(20, "--list")];
        assert_eq!(arms("README.md"), outside);
        let mut everywhere = outside.clone();
        everywhere.insert(3, s(12, "--install-hooks"));
        everywhere.insert(4, s(16, "--run-demo"));
        assert_eq!(arms("docs/install.md"), everywhere);
    }

    // spec: installer/SPEC.md §The front door's verbs — the extractor applied to the crate's own two
    // source files yields exactly the in-process arm set, so a reshaped array reds where it lands
    #[test]
    fn the_arm_extractor_equals_the_binarys_own_arm_set() {
        let got = arm_table(include_str!("../emit/mod.rs"), include_str!("../main.rs"));
        assert_eq!(got, Some(binary_arms()));
    }

    // spec: installer/SPEC.md §The front door's verbs — a nested literal and a comment are skipped,
    // and an absent or memberless array is no arm set
    #[test]
    fn the_arm_extractor_takes_each_tuples_first_literal() {
        let emit = "pub const ARMS: &[(&str, Arm, &[&str])] = &[\n    // \"--commented\"\n    (\"--a\", Arm::Emit(f, Grammar::Flags(&[\"--write\"])), &[\"K\"]),\n    (\n        \"--b\",\n        Arm::Run(g),\n        &[],\n    ),\n];\n";
        let main = "const TOP_LEVEL_FLAGS: &[&str] = &[\"--help\", \"-h\"];\n";
        let set = |xs: &[&str]| -> BTreeSet<String> { xs.iter().map(|x| x.to_string()).collect() };
        assert_eq!(arm_table(emit, main), Some(set(&["--a", "--b", "--help", "-h"])));
        assert_eq!(arm_table("", main), None);
        assert_eq!(arm_table("pub const ARMS: &[(&str,)] = &[];\n", main), None);
        assert_eq!(arm_table(emit, "const TOP_LEVEL_FLAGS: &[&str] = &[];\n"), None);
    }

    // spec: installer/SPEC.md §The front door's verbs — the profile roster's names with the derived
    // one, and the recipe knob's keys, each empty where the file is absent
    #[test]
    fn the_operand_sets_read_the_roster_and_the_seam() {
        let set = |xs: &[&str]| -> BTreeSet<String> { xs.iter().map(|x| x.to_string()).collect() };
        assert_eq!(profile_set(Some("# c\nstarter\ta\nprose\tb # note\n".to_string())), set(&["starter", "prose", "full"]));
        assert_eq!(profile_set(None), set(&[]));
        let seam = "GATE_SDK_PAYLOAD_RECIPES[speckit] = x\nGATE_SDK_PAYLOAD_LICENSE = y\nGATE_SDK_PAYLOAD_RECIPES[openspec] = z\n";
        assert_eq!(recipe_set("s", Some(seam.to_string())), Ok(set(&["speckit", "openspec"])));
        assert_eq!(recipe_set("s", None), Ok(set(&[])));
        assert!(recipe_set("s", Some("no equals here\n".to_string())).is_err());
    }

    // spec: installer/SPEC.md §The front door's verbs — the table whose header's first cell is
    // `verb`, its first-column code spans, and nothing past the table; the flag table's pairs
    #[test]
    fn the_table_readers_take_each_tables_columns() {
        let t = "# x\n\n| verb | asks |\n| --- | --- |\n| `init` | a |\n| `demo` | b |\n\n| `other` | c |\n";
        assert_eq!(verb_table(t), Some(vec!["init".to_string(), "demo".to_string()]));
        assert_eq!(verb_table("| name | x |\n| --- | --- |\n| `a` | b |\n"), None);
        let f = "| flag | verbs | means |\n| --- | --- | --- |\n| `--force` | `init`, `uninstall` | x |\n";
        let pair = |v: &str, f: &str| (v.to_string(), f.to_string());
        assert_eq!(flag_table(f), Some(vec![pair("init", "--force"), pair("uninstall", "--force")]));
        assert_eq!(flag_table(t), None);
    }
}
