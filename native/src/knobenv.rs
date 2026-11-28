// spec: gate-sdk/SPEC.md §lib/gate.sh — the crate's one serialization point for the
// process-global environment: a case writes a variable only while holding this guard, so
// cargo's threads cannot interleave two writers of one variable
use std::sync::{Mutex, MutexGuard};

static LOCK: Mutex<()> = Mutex::new(());

// spec: gate-sdk/SPEC.md §lib/gate.sh — the guard IS the write API, so what a caller must
// hold to write at all is the lock itself; a free-standing mutex a case may forget to take
// would leave the discipline this replaces
pub struct KnobEnv {
    _guard: MutexGuard<'static, ()>,
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — a poisoned lock is a sibling case that panicked, which
// says nothing about the environment, so the guard is recovered rather than cascading one
// case's failure into every case that writes a knob
pub fn lock() -> KnobEnv {
    KnobEnv {
        _guard: LOCK.lock().unwrap_or_else(|e| e.into_inner()),
    }
}

impl KnobEnv {
    #[cfg(test)]
    pub fn set(&self, name: &str, value: &str) {
        std::env::set_var(name, value);
    }

    pub fn set_os(&self, name: &str, value: &std::ffi::OsStr) {
        std::env::set_var(name, value);
    }

    pub fn remove(&self, name: &str) {
        std::env::remove_var(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::Path;

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the roster of process-global environment writes; a
    // spelling outside this module is a write nothing serializes
    const ENV_WRITE_APIS: &[&str] = &["set_var", "remove_var"];

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the machine side of the serialization contract, in
    // the roster shape §check-reads-couples' unit test B uses: a guard a later case bypasses by
    // calling the std API directly leaves the race the guard was landed to delete
    #[test]
    fn no_module_outside_this_one_writes_the_environment() {
        let _knobs = lock();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = crate::walk::find_files(&src, &["rs"]).expect("cannot enumerate the sources");
        assert!(!files.is_empty(), "no crate source found to scan");
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            if f.file_name().and_then(|n| n.to_str()) == Some("knobenv.rs") {
                continue;
            }
            let text = std::fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            for api in ENV_WRITE_APIS {
                if text.contains(api) {
                    offenders.push(format!("{} names {}", f.display(), api));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "an environment write outside knobenv.rs is unserialized, so a sibling case on \
             another cargo thread can observe it: {:?} — take knobenv::lock() and write through \
             the guard it returns",
            offenders
        );
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the walk's own reads of the knob set: a walk.rs
    // function spelling one is a root, and every reader in the crate is derived from the roots by
    // closure, so a new reader joins the roster unlisted
    const KNOB_READS: &[&str] = &["crate::knobs::", "std::env::var"];

    fn ident(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a file's module is named by its last path segment, the
    // segment a caller elsewhere qualifies the call with
    fn module_of(src: &Path, f: &Path) -> String {
        let rel = f.strip_prefix(src).unwrap_or(f);
        let stem = rel.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        match stem {
            "mod" => rel.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string(),
            "main" => "crate".to_string(),
            s => s.to_string(),
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — what a file's `use` items bring into scope: a named item
    // as (module, item) under the name it is called by, and a glob as the module it opens
    #[derive(Default)]
    struct Scope {
        items: Vec<(String, String, String)>,
        globs: Vec<String>,
    }

    fn uses(text: &str) -> Scope {
        let mut scope = Scope::default();
        let mut rest = text;
        while let Some(at) = rest.find("use ") {
            let line_start = rest[..at].rfind('\n').map_or(0, |n| n + 1);
            let lead = &rest[line_start..at];
            let tail = &rest[at + 4..];
            let Some(end) = tail.find(';') else { break };
            if lead.trim().is_empty() || lead.trim() == "pub" || lead.trim() == "pub(crate)" {
                let tree: String = tail[..end].chars().filter(|c| !c.is_whitespace() || *c == ' ').collect();
                expand_use("", tree.trim(), &mut scope);
            }
            rest = &tail[end..];
        }
        scope
    }

    fn expand_use(prefix: &str, tree: &str, scope: &mut Scope) {
        let tree = tree.trim();
        if let Some(open) = tree.find('{') {
            let head = format!("{}{}", prefix, tree[..open].trim_end_matches("::"));
            let inner = tree[open + 1..].trim_end().trim_end_matches('}');
            let mut depth = 0usize;
            let mut start = 0usize;
            for (i, c) in inner.char_indices() {
                match c {
                    '{' => depth += 1,
                    '}' => depth = depth.saturating_sub(1),
                    ',' if depth == 0 => {
                        expand_use(&format!("{}::", head), &inner[start..i], scope);
                        start = i + 1;
                    }
                    _ => {}
                }
            }
            expand_use(&format!("{}::", head), &inner[start..], scope);
            return;
        }
        let path = format!("{}{}", prefix, tree);
        let (path, alias) = match path.split_once(" as ") {
            Some((p, a)) => (p.trim().to_string(), Some(a.trim().to_string())),
            None => (path.trim().to_string(), None),
        };
        let segs: Vec<&str> = path.split("::").filter(|s| !s.is_empty()).collect();
        match segs.as_slice() {
            [.., m, "*"] => scope.globs.push(m.to_string()),
            [.., m, item] => {
                let name = alias.unwrap_or_else(|| item.to_string());
                scope.items.push((name, m.to_string(), item.to_string()));
            }
            _ => {}
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — each call written before a `(`, resolved by module from
    // its qualifier or the file's `use` items; a method call and a definition's own name are not calls
    fn called(body: &str, module: &str, scope: &Scope) -> BTreeSet<String> {
        let b = body.as_bytes();
        let mut out = BTreeSet::new();
        for (i, _) in body.match_indices('(') {
            let mut s = i;
            while s > 0 && (ident(b[s - 1] as char) || b[s - 1] == b':') {
                s -= 1;
            }
            let path = body[s..i].trim_start_matches(':');
            if path.is_empty() || (s > 0 && b[s - 1] == b'.') || body[..s].ends_with("fn ") {
                continue;
            }
            let segs: Vec<&str> = path.split("::").filter(|p| !p.is_empty()).collect();
            let imported = |n: &str| scope.items.iter().find(|(name, _, _)| name == n);
            match segs.as_slice() {
                [n] => match imported(n) {
                    Some((_, m, item)) => {
                        out.insert(format!("{}::{}", m, item));
                    }
                    None => {
                        out.insert(format!("{}::{}", module, n));
                        for g in &scope.globs {
                            out.insert(format!("{}::{}", g, n));
                        }
                    }
                },
                [.., q, n] if *q == "super" || *q == "self" => {
                    out.insert(format!("{}::{}", module, n));
                }
                [.., m, q, n] if q.starts_with(|c: char| c.is_ascii_uppercase()) => {
                    let m = if *m == "super" || *m == "self" { module } else { m };
                    out.insert(format!("{}::{}::{}", m, q, n));
                }
                [q, n] if q.starts_with(|c: char| c.is_ascii_uppercase()) => {
                    let m = imported(q).map_or(module, |(_, m, _)| m.as_str());
                    out.insert(format!("{}::{}::{}", m, q, n));
                }
                [.., q, n] => {
                    out.insert(format!("{}::{}", q, n));
                }
                [] => {}
            }
        }
        out
    }

    fn fn_name(line: &str) -> Option<&str> {
        let t = line.trim_start();
        let rest = t.strip_prefix("pub(crate) ").or_else(|| t.strip_prefix("pub ")).unwrap_or(t);
        let rest = rest.strip_prefix("fn ")?;
        let end = rest.find(|c: char| !ident(c))?;
        Some(&rest[..end])
    }

    fn indent(l: &str) -> usize {
        l.len() - l.trim_start().len()
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — an indented fn's enclosing item is the nearest line
    // above it at a shallower indent, and inside `impl <Type>` the fn is called `Type::name`
    fn impl_type(lines: &[&str], i: usize) -> Option<String> {
        let at = indent(lines[i]);
        let open = (0..i).rev().map(|j| lines[j]).find(|l| !l.trim().is_empty() && indent(l) < at)?;
        let mut t = open.trim_start().strip_prefix("impl")?;
        if t.starts_with('<') {
            let mut depth = 0usize;
            let end = t.char_indices().find(|&(_, c)| {
                match c {
                    '<' => depth += 1,
                    '>' => depth -= 1,
                    _ => {}
                }
                depth == 0
            })?;
            t = &t[end.0 + 1..];
        }
        let t = t.rsplit(" for ").next()?.trim_start();
        let end = t.find(|c: char| !ident(c)).unwrap_or(t.len());
        (end > 0).then(|| t[..end].to_string())
    }

    struct Func {
        key: String,
        calls: BTreeSet<String>,
        case: bool,
        own: bool,
        reads: bool,
        held: bool,
        returns: String,
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a type owning the guard by value, so a function
    // returning one hands its caller the lock rather than releasing it
    fn guard_types(text: &str) -> Vec<String> {
        let lines: Vec<&str> = text.lines().collect();
        let mut out = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim_start();
            let t = t.strip_prefix("pub(crate) ").or_else(|| t.strip_prefix("pub ")).unwrap_or(t);
            let Some(rest) = t.strip_prefix("struct ") else { continue };
            let name: String = rest.chars().take_while(|c| ident(*c)).collect();
            let close = format!("{}}}", &l[..indent(l)]);
            let fields = lines[i + 1..].iter().take_while(|f| **f != close);
            if !name.is_empty() && fields.into_iter().any(|f| f.contains("KnobEnv") && !f.contains('&')) {
                out.push(name);
            }
        }
        out
    }

    fn returns_guard(f: &Func, guards: &[String]) -> bool {
        let named = |n: &str| {
            f.returns.match_indices(n).any(|(at, _)| {
                let before = !matches!(f.returns[..at].chars().next_back(), Some(c) if ident(c));
                let after = !matches!(f.returns[at + n.len()..].chars().next(), Some(c) if ident(c));
                before && after
            })
        };
        named("KnobEnv") || guards.iter().any(|g| named(g))
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a taker whose return type carries the guard holds it
    // for its caller, as does a function returning one it was handed; any other taker releases it
    // on return, so calling it holds nothing after
    fn handers(all: &[Func], guards: &[String]) -> BTreeSet<String> {
        let mut set: BTreeSet<String> =
            all.iter().filter(|f| !f.case && f.held && returns_guard(f, guards)).map(|f| f.key.clone()).collect();
        loop {
            let more: Vec<String> = all
                .iter()
                .filter(|f| !f.case && !set.contains(&f.key) && returns_guard(f, guards))
                .filter(|f| f.calls.iter().any(|n| set.contains(n)))
                .map(|f| f.key.clone())
                .collect();
            if more.is_empty() {
                return set;
            }
            set.extend(more);
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a case reaching a reader is held only where the guard
    // spans the reader: taken in the case, or handed to it by a taker's return value
    fn unheld(all: &[Func], readers: &BTreeSet<String>, guards: &[String]) -> Vec<String> {
        let handers = handers(all, guards);
        all.iter()
            .filter(|f| f.case && !f.held && !f.calls.iter().any(|n| handers.contains(n)))
            .filter_map(|f| f.calls.iter().find(|n| readers.contains(*n)).map(|r| format!("{} calls {}", f.key, r)))
            .collect()
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a fn runs from its `fn` line to the first `}` line at its
    // own indent; in the roots' file a fn with no such line is refused rather than read as ending at
    // the file's end, and elsewhere it is a `fn` line inside a literal and is no function
    fn fns(src: &Path, f: &Path, text: &str, strict: bool) -> Vec<Func> {
        let module = module_of(src, f);
        let scope = uses(text);
        let home = f.file_name().and_then(|n| n.to_str()) == Some("knobenv.rs");
        let lines: Vec<&str> = text.lines().collect();
        let mut out = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let Some(name) = fn_name(l) else { continue };
            let close = format!("{}}}", &l[..indent(l)]);
            let Some(end) =
                (i..lines.len()).find(|&j| lines[j] == close || (j == i && l.trim_end().ends_with('}')))
            else {
                assert!(!strict, "{}:{}: fn {} has no closing line at its indent", f.display(), i + 1, name);
                continue;
            };
            let body = lines[i..=end].join("\n");
            let signature = body.split('{').next().unwrap_or_default();
            let ty = impl_type(&lines, i);
            let returns = signature.split_once("->").map_or("", |(_, r)| r).trim();
            let returns = match &ty {
                Some(t) if returns == "Self" => t.clone(),
                _ => returns.to_string(),
            };
            let key = match &ty {
                Some(t) => format!("{}::{}::{}", module, t, name),
                None => format!("{}::{}", module, name),
            };
            out.push(Func {
                key,
                calls: called(&body, &module, &scope),
                case: (0..i).rev().map(|j| lines[j].trim()).take_while(|l| l.starts_with("#[")).any(|l| l == "#[test]"),
                own: indent(l) == 0 || ty.is_some(),
                reads: KNOB_READS.iter().any(|r| body.contains(r)),
                held: body.contains(concat!("knobenv::", "lock()")) || (home && body.contains(concat!("= ", "lock()"))),
                returns,
            });
        }
        out
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — every function outside a case reaching the seed through
    // its calls, stopping at a barrier
    fn closure(all: &[Func], mut set: BTreeSet<String>, barrier: &BTreeSet<String>) -> BTreeSet<String> {
        loop {
            let more: Vec<String> = all
                .iter()
                .filter(|f| !f.case && !set.contains(&f.key) && !barrier.contains(&f.key))
                .filter(|f| f.calls.iter().any(|n| set.contains(n)))
                .map(|f| f.key.clone())
                .collect();
            if more.is_empty() {
                return set;
            }
            set.extend(more);
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the roots are walk.rs's own functions, top-level or an
    // impl's, never a case's or a test helper's; a function reaching the guard is the barrier, since
    // its reads are serialized
    fn knob_readers(all: &[Func], lockers: &BTreeSet<String>) -> BTreeSet<String> {
        let roots = all
            .iter()
            .filter(|f| f.key.starts_with("walk::") && f.own && !f.case && f.reads)
            .map(|f| f.key.clone())
            .collect();
        closure(all, roots, lockers)
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a taker releasing the guard on return holds nothing for
    // the walk its caller makes next, and one handing the guard back holds it
    #[test]
    fn a_released_taker_leaves_the_walk_after_it_unheld() {
        let src = Path::new("/probe/src");
        let walk = concat!("pub fn find_files() {\n    let _ = std::env::", "var(\"K\");\n}\n");
        let probe = concat!(
            "struct Held {\n    env: KnobEnv,\n}\n",
            "fn render() -> String {\n    let _k = crate::knobenv::", "lock();\n    String::new()\n}\n",
            "fn hold() -> Held {\n    let env = crate::knobenv::", "lock();\n    Held { env }\n}\n",
            "#[test]\nfn released() {\n    render();\n    walk::", "find_files();\n}\n",
            "#[test]\nfn handed() {\n    let _h = hold();\n    walk::", "find_files();\n}\n",
        );
        let mut all = fns(src, &src.join("walk.rs"), walk, true);
        all.extend(fns(src, &src.join("probe.rs"), probe, false));
        let guards = guard_types(probe);
        assert_eq!(guards, vec!["Held".to_string()]);
        let lockers = closure(&all, all.iter().filter(|f| !f.case && f.held).map(|f| f.key.clone()).collect(), &BTreeSet::new());
        let readers = knob_readers(&all, &lockers);
        assert_eq!(unheld(&all, &readers, &guards), vec!["probe::released calls walk::find_files".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a walk reads the knob set, so a case walking unlocked,
    // directly or through any function in the crate, reads it mid-write of a sibling's temporary
    // knobs and refuses; every case calling a reader holds the guard
    #[test]
    fn every_case_reaching_a_knob_reading_walk_holds_the_lock() {
        let _knobs = lock();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = crate::walk::find_files(&src, &["rs"]).expect("cannot enumerate the sources");
        let walk = src.join("walk.rs");
        let mut all: Vec<Func> = Vec::new();
        let mut guards: Vec<String> = Vec::new();
        for f in &files {
            let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            all.extend(fns(&src, f, &text, *f == walk));
            guards.extend(guard_types(&text));
        }
        for known in ["Marked", "Sandbox"] {
            assert!(guards.iter().any(|g| g == known), "the guard types lost {}", known);
        }
        let lockers = closure(&all, all.iter().filter(|f| !f.case && f.held).map(|f| f.key.clone()).collect(), &BTreeSet::new());
        let readers = knob_readers(&all, &lockers);
        for known in [
            "walk::prune_dirs",
            "walk::find_files",
            "walk::Descent::at",
            "walk::glob_files",
            "registry::couples_knob_names",
        ] {
            assert!(readers.contains(known), "the closure lost {}", known);
        }
        // spec: gate-sdk/SPEC.md §lib/gate.sh — the guard is not reentrant, so a function taking it
        // never reaches another function that does: the thread would wait on itself
        let reentrant: Vec<String> = all
            .iter()
            .filter(|f| f.held)
            .filter_map(|f| {
                f.calls.iter().find(|n| **n != f.key && lockers.contains(*n)).map(|r| format!("{} calls {}", f.key, r))
            })
            .collect();
        assert!(
            reentrant.is_empty(),
            "a function holding the knob lock reaches one that takes it again and deadlocks: {:?} — \
             take the guard in one of the two",
            reentrant
        );
        let offenders = unheld(&all, &readers, &guards);
        assert!(
            offenders.is_empty(),
            "a case reaching a knob-reading walk without the knob lock races a sibling's knob write: \
             {:?} — take crate::knobenv::lock() for the case's length",
            offenders
        );
    }
}
