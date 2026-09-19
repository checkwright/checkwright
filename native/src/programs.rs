// spec: gate-sdk/SPEC.md §The program roster — the binary's whole spawn set as a type only this
// module constructs, so `proc`'s faces cannot be handed a program the roster does not name

// spec: gate-sdk/SPEC.md §The program roster
#[derive(Clone, Debug)]
pub struct Program {
    identity: Identity,
    path: Option<String>,
}

#[derive(Clone, Debug)]
enum Identity {
    Member {
        name: &'static str,
        // spec: gate-sdk/SPEC.md §The program roster — read by the parity tests alone
        #[cfg_attr(not(test), allow(dead_code))]
        audience: &'static str,
    },
    Consumer {
        command: String,
        ground: &'static str,
    },
}

// spec: gate-sdk/SPEC.md §The program roster — the ground a gate dispatch argv's head carries,
// whether a `.sh` declaration or `gate_command`'s answer
pub const GATE_DECLARATION: &str = "gate declaration";

// spec: gate-sdk/SPEC.md §The program roster — one row per member; the test-only `ALL` is emitted
// from the same rows, so it is complete by construction
macro_rules! roster {
    ($($(#[$m:meta])* $id:ident = $name:expr, $audience:expr;)*) => {
        $(
            $(#[$m])*
            pub const $id: Program = Program {
                identity: Identity::Member { name: $name, audience: $audience },
                path: None,
            };
        )*

        #[cfg(test)]
        pub const ALL: &[Program] = &[$($(#[$m])* $id,)*];
    };
}

roster! {
    GIT = "git", "";
    BASH = "bash", "context-kit+delegation-kit+drift-kit+guard-kit+lifecycle-kit";
    #[cfg(not(unix))]
    DATE = "date", "";
    MKTEMP = "mktemp", "contributor";
    CP = "cp", "contributor";
    JQ = "jq", "contributor";
    SHELLCHECK = "shellcheck", "registered";
    #[cfg(not(unix))]
    PS = "ps", "";
    CURL = "curl", "delegation-kit";
    CARGO = "cargo", "contributor";
    RUSTC = "rustc", "contributor";
    TAR = "tar", "contributor";
    NPM = "npm", "contributor";
    UNAME = "uname", "contributor";
    PWSH = "pwsh", "contributor";
    POWERSHELL = "powershell", "contributor";
    CHECKWRIGHT_GATES = env!("CARGO_PKG_NAME"), "";
}

// spec: gate-sdk/SPEC.md §The program roster — the members a `PROBE_SET` walk names; a unit test
// holds this set equal to `PROBE_SET`'s names, so it keeps no row live that the walk never spawns
fn probed() -> [Program; 6] {
    [GIT, BASH, JQ, SHELLCHECK, CURL, CARGO]
}

// spec: gate-sdk/SPEC.md §The program roster — an existing member or nothing; it constructs no
// new identity
pub fn by_name(name: &str) -> Option<Program> {
    probed().into_iter().find(|p| p.name() == name)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — a program is named as `PATH` names it: a path
// is its final component minus the host's executable suffix, a bare name is as passed
pub fn name_of(program: &str) -> String {
    if !program.chars().any(std::path::is_separator) {
        return program.to_string();
    }
    let last = program
        .rsplit(std::path::is_separator)
        .next()
        .unwrap_or(program);
    let suffix = std::env::consts::EXE_SUFFIX;
    let cut = last.len().saturating_sub(suffix.len());
    if !suffix.is_empty()
        && last.len() > suffix.len()
        && last.is_char_boundary(cut)
        && last[cut..].eq_ignore_ascii_case(suffix)
    {
        return last[..cut].to_string();
    }
    last.to_string()
}

impl Program {
    // spec: gate-sdk/SPEC.md §The program roster — the one constructor for a program named outside
    // the crate's source; `ground` is the knob or `GATE_DECLARATION` that named it
    pub fn consumer(ground: &'static str, command: impl Into<String>) -> Program {
        Program {
            identity: Identity::Consumer {
                command: command.into(),
                ground,
            },
            path: None,
        }
    }

    pub fn at(self, path: impl Into<String>) -> Program {
        Program {
            identity: self.identity,
            path: Some(path.into()),
        }
    }

    pub fn name(&self) -> String {
        match &self.identity {
            Identity::Member { name, .. } => name.to_string(),
            Identity::Consumer { command, .. } => name_of(command),
        }
    }

    pub fn invocation(&self) -> &str {
        if let Some(p) = &self.path {
            return p;
        }
        match &self.identity {
            Identity::Member { name, .. } => name,
            Identity::Consumer { command, .. } => command,
        }
    }

    pub fn ground(&self) -> Option<&'static str> {
        match &self.identity {
            Identity::Member { .. } => None,
            Identity::Consumer { ground, .. } => Some(ground),
        }
    }

    // spec: gate-sdk/SPEC.md §The program roster — the spawn-failure subject: the name, and for a
    // consumer command the ground that named it
    pub fn label(&self) -> String {
        match self.ground() {
            Some(g) => format!("{} (named by {})", self.name(), g),
            None => self.name(),
        }
    }

    #[cfg(test)]
    pub fn audience(&self) -> Option<&'static str> {
        match &self.identity {
            Identity::Member { audience, .. } => Some(audience),
            Identity::Consumer { .. } => None,
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolfloor;
    use std::path::Path;

    fn floor_default() -> Vec<&'static str> {
        let row = crate::knobs::gate_sdk::KIT
            .rows
            .iter()
            .find(|r| r.name == "GATE_SDK_PROGRAM_FLOOR")
            .expect("GATE_SDK_PROGRAM_FLOOR has no row in gate-sdk's knob table");
        match row.default {
            crate::knobs::Default::Indexed(v) => v.to_vec(),
            _ => panic!("GATE_SDK_PROGRAM_FLOOR's default is not an indexed literal"),
        }
    }

    fn probe_elements() -> Vec<toolfloor::Element> {
        toolfloor::PROBE_SET.iter().map(|e| toolfloor::parse(e)).collect()
    }

    // spec: gate-sdk/SPEC.md §The program roster — assertion A: an adopter-side member, a
    // conditional audience included, is on the floor's default, on `PROBE_SET`, or is the payload
    #[test]
    fn every_adopter_side_member_is_on_the_floor_the_probe_set_or_the_payload() {
        let floor = floor_default();
        let probed: Vec<String> = probe_elements()
            .into_iter()
            .filter(|e| e.audience != toolfloor::CONTRIBUTOR)
            .map(|e| e.name)
            .collect();
        let offenders: Vec<String> = ALL
            .iter()
            .filter(|p| p.audience().is_some_and(|a| a != toolfloor::CONTRIBUTOR))
            .map(Program::name)
            .filter(|n| {
                !floor.contains(&n.as_str())
                    && !probed.contains(n)
                    && n.as_str() != env!("CARGO_PKG_NAME")
            })
            .collect();
        assert!(
            offenders.is_empty(),
            "adopter-side roster members on neither GATE_SDK_PROGRAM_FLOOR's default nor \
             PROBE_SET, and not the payload: {:?} — add each to the one that owns it, or give it \
             the contributor audience",
            offenders
        );
    }

    // spec: gate-sdk/SPEC.md §The program roster — assertion B
    #[test]
    fn a_member_on_the_probe_set_carries_one_audience() {
        let offenders: Vec<String> = probe_elements()
            .into_iter()
            .filter_map(|e| {
                let member = ALL.iter().find(|p| p.name() == e.name)?;
                let theirs = member.audience().unwrap_or("");
                (theirs != e.audience).then(|| {
                    format!("{}: roster {:?}, PROBE_SET {:?}", e.name, theirs, e.audience)
                })
            })
            .collect();
        assert!(
            offenders.is_empty(),
            "roster and PROBE_SET disagree on audience: {:?}",
            offenders
        );
    }

    // spec: gate-sdk/SPEC.md §The program roster — assertion C, both directions: the walk's lookup
    // is total over `PROBE_SET`, and names nothing the walk never probes
    #[test]
    fn the_probe_set_walk_resolves_every_element_and_nothing_else() {
        let names: Vec<String> = probe_elements().into_iter().map(|e| e.name).collect();
        let unresolved: Vec<&String> = names.iter().filter(|n| by_name(n).is_none()).collect();
        assert!(
            unresolved.is_empty(),
            "PROBE_SET elements with no roster member through by_name: {:?}",
            unresolved
        );
        let unprobed: Vec<String> = probed()
            .iter()
            .map(Program::name)
            .filter(|n| !names.contains(n))
            .collect();
        assert!(
            unprobed.is_empty(),
            "by_name answers for members PROBE_SET does not carry: {:?}",
            unprobed
        );
    }

    // spec: gate-sdk/SPEC.md §The program roster — the declared grounds a literal may name: every
    // static knob name, and each family as its prefix and `*`
    fn declared_grounds() -> Vec<String> {
        let mut out = crate::knobs::static_names();
        for kit in crate::knobs::STATIC_KITS {
            out.extend(kit.families.iter().map(|f| format!("{}*", f.prefix)));
        }
        out
    }

    fn is_ident(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    // spec: gate-sdk/SPEC.md §The program roster — a call's arguments from just past its `(`, split at
    // top-level commas and ending at the matching `)`; string literals are skipped whole
    fn call_args(text: &str, open: usize) -> Vec<String> {
        let (mut depth, mut in_str, mut esc) = (0i32, false, false);
        let (mut out, mut cur) = (Vec::new(), String::new());
        for c in text[open..].chars() {
            if in_str {
                cur.push(c);
                if esc {
                    esc = false;
                } else if c == '\\' {
                    esc = true;
                } else if c == '"' {
                    in_str = false;
                }
                continue;
            }
            match c {
                '"' => {
                    in_str = true;
                    cur.push(c);
                }
                '(' | '[' | '{' => {
                    depth += 1;
                    cur.push(c);
                }
                ')' | ']' | '}' if depth == 0 => {
                    if !cur.trim().is_empty() {
                        out.push(cur.trim().to_string());
                    }
                    return out;
                }
                ')' | ']' | '}' => {
                    depth -= 1;
                    cur.push(c);
                }
                ',' if depth == 0 => {
                    out.push(cur.trim().to_string());
                    cur.clear();
                }
                _ => cur.push(c),
            }
        }
        out
    }

    // spec: gate-sdk/SPEC.md §The program roster — the function a position sits in: its name,
    // whether it is `pub`, and its parameter list
    fn enclosing_fn(text: &str, pos: usize) -> Option<(String, bool, Vec<String>)> {
        let head = text[..pos]
            .rmatch_indices("fn ")
            .map(|(i, _)| i)
            .find(|i| *i == 0 || !text[..*i].ends_with(is_ident))?;
        let name: String = text[head + 3..].chars().take_while(|c| is_ident(*c)).collect();
        let line_start = text[..head].rfind('\n').map_or(0, |i| i + 1);
        let public = text[line_start..head].contains("pub");
        let open = head + text[head..].find('(')? + 1;
        Some((name, public, call_args(text, open)))
    }

    struct Scan<'a> {
        declared: Vec<String>,
        files: &'a [(String, String)],
        forwarders: Vec<(String, usize, usize)>,
        offenders: Vec<String>,
    }

    impl Scan<'_> {
        // spec: gate-sdk/SPEC.md §The program roster — a ground expression is a declared literal, the
        // gate-declaration constant, or a `&'static str` parameter of the function it sits in, whose
        // own callers are then read at the same argument position
        fn check(&mut self, file: usize, pos: usize, expr: &str) {
            if let Some(lit) = expr.strip_prefix('"') {
                if !self.declared.iter().any(|d| d == lit.trim_end_matches('"')) {
                    self.offenders.push(format!("{}: {}", self.files[file].0, expr));
                }
                return;
            }
            if expr == "GATE_DECLARATION" || expr.ends_with("programs::GATE_DECLARATION") {
                return;
            }
            let forwarded = expr.chars().all(is_ident).then(|| enclosing_fn(&self.files[file].1, pos)).flatten();
            let param = forwarded.and_then(|(name, public, params)| {
                params
                    .iter()
                    .position(|p| p.starts_with(&format!("{}:", expr)) && p.contains("&'static str"))
                    .map(|i| (name, public, i))
            });
            match param {
                Some((name, public, i)) => {
                    let scope = if public { usize::MAX } else { file };
                    if !self.forwarders.iter().any(|(n, s, j)| *n == name && *s == scope && *j == i) {
                        self.forwarders.push((name, scope, i));
                    }
                }
                None => self.offenders.push(format!("{}: {}", self.files[file].0, expr)),
            }
        }
    }

    // spec: gate-sdk/SPEC.md §The program roster — assertion D over shipped scope
    #[test]
    fn every_consumer_commands_ground_names_a_knob_or_the_gate_declaration() {
        let _knobs = crate::knobenv::lock();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files: Vec<(String, String)> = crate::walk::find_files(&src, &["rs"])
            .expect("cannot enumerate the crate modules")
            .iter()
            .map(|f| {
                let text = std::fs::read_to_string(f)
                    .unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
                (f.display().to_string(), crate::proc::tests::shipped_scope(&text))
            })
            .collect();
        let mut scan = Scan {
            declared: declared_grounds(),
            files: &files,
            forwarders: Vec::new(),
            offenders: Vec::new(),
        };
        let spelling = concat!("Program::consumer", "(");
        let mut seen = 0usize;
        for (i, (_, text)) in files.iter().enumerate() {
            for (at, _) in text.match_indices(spelling) {
                seen += 1;
                let args = call_args(text, at + spelling.len());
                scan.check(i, at, args.first().map_or("", String::as_str));
            }
        }
        let mut done = 0usize;
        while done < scan.forwarders.len() {
            let (name, scope, idx) = scan.forwarders[done].clone();
            done += 1;
            let call = format!("{}(", name);
            for (i, (_, text)) in files.iter().enumerate() {
                if scope != usize::MAX && scope != i {
                    continue;
                }
                for (at, _) in text.match_indices(&call) {
                    if text[..at].ends_with(is_ident) || text[..at].ends_with("fn ") {
                        continue;
                    }
                    let args = call_args(text, at + call.len());
                    scan.check(i, at, args.get(idx).map_or("", String::as_str));
                }
            }
        }
        assert!(seen > 0, "the scan found no Program::consumer call at all");
        assert!(
            scan.offenders.is_empty(),
            "consumer-command grounds naming no declared knob and no GATE_DECLARATION, and not a \
             `&'static str` parameter whose callers pass one: {:?}",
            scan.offenders
        );
    }

    // spec: gate-sdk/SPEC.md §The program roster — the scan is asserted rather than trusted: a
    // forwarded ground reaches its caller's literal, and an undeclared one is named
    #[test]
    fn the_ground_scan_follows_a_forwarded_parameter_to_its_callers() {
        let files = vec![(
            "x.rs".to_string(),
            "fn f(g: &'static str, c: &str) { Program::consumer(g, c); }\nfn a() { f(\"NOT_A_KNOB\", \"x\"); f(\"EVIDENCE_KIT_PARSER\", \"y\"); }\n".to_string(),
        )];
        let mut scan = Scan {
            declared: declared_grounds(),
            files: &files,
            forwarders: Vec::new(),
            offenders: Vec::new(),
        };
        let at = files[0].1.find("Program::consumer(").expect("fixture carries the call");
        scan.check(0, at, "g");
        assert_eq!(scan.forwarders, vec![("f".to_string(), 0, 0)]);
        let text = &files[0].1;
        for (at, _) in text.match_indices("f(").filter(|(i, _)| !text[..*i].ends_with("fn ")) {
            let args = call_args(text, at + 2);
            scan.check(0, at, &args[0]);
        }
        assert_eq!(scan.offenders, vec!["x.rs: \"NOT_A_KNOB\"".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §The program roster — retargeting keeps the identity it was handed
    #[test]
    fn a_retargeted_member_keeps_its_name_and_spawns_its_path() {
        let p = JQ.at("/opt/bin/jq");
        assert_eq!(p.name(), "jq");
        assert_eq!(p.invocation(), "/opt/bin/jq");
        assert_eq!(p.ground(), None);
        let c = Program::consumer("EVIDENCE_KIT_PARSER", "scripts/parse.sh");
        assert_eq!(c.name(), "parse.sh");
        assert_eq!(c.label(), "parse.sh (named by EVIDENCE_KIT_PARSER)");
    }
}
