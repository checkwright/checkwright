// spec: gate-sdk/SPEC.md §check-script-preflight — no configured shell script breaks a rule in the
// consumer's rules file
use crate::ere::Ere;
use crate::exit::{self, Fault};
use crate::listing;
use crate::shell_lines::{self, Options, Script, Waived, Word};
use crate::walk;
use std::path::Path;

const NAME: &str = "check-script-preflight";

enum Kind {
    Command { cmd: String, allow: Vec<Ere>, before: Vec<Ere> },
    Forbidden { tokens: Vec<String> },
    EarlyExit,
    IdGrammar { cmd: String, flags: Vec<String>, grammar: Ere },
}

struct Rule {
    id: String,
    kind: Kind,
    waivers: Vec<String>,
}

struct Rules {
    floor: usize,
    body: Vec<String>,
    rules: Vec<Rule>,
}

struct Block {
    id: String,
    kind: &'static str,
    line: usize,
    keys: Vec<(&'static str, String, usize)>,
}

// spec: gate-sdk/SPEC.md §check-script-preflight — each kind's required keys, then its optional
// ones; `cmd` and `grammar` are given once, every other key repeats
const KINDS: &[(&str, &[&str], &[&str])] = &[
    ("command", &["cmd"], &["allow", "before", "waiver"]),
    ("forbidden-token", &["token"], &["waiver"]),
    ("early-exit", &[], &["waiver"]),
    ("id-grammar", &["cmd", "flag", "grammar"], &["waiver"]),
];
const ONCE: &[&str] = &["cmd", "grammar"];

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            let fault = fault.prefixed(NAME);
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

fn parse(file: &str, text: &str) -> Result<Rules, Fault> {
    let bad = |line: usize, what: String| Fault::refused(format!("{}:{}: {}", file, line, what));
    let (mut floor, mut body, mut blocks) = (1usize, Vec::new(), Vec::<Block>::new());
    for (i, raw) in text.lines().enumerate() {
        let n = i + 1;
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if line.starts_with([' ', '\t']) {
            let Some(b) = blocks.last_mut() else {
                return Err(bad(n, "a key line before any 'rule' line".to_string()));
            };
            let (key, value) = trimmed.split_once([' ', '\t']).map_or((trimmed, ""), |(k, v)| (k, v.trim()));
            let (_, req, opt) = KINDS.iter().find(|k| k.0 == b.kind).expect("a block's kind is known");
            let Some(key) = req.iter().chain(opt.iter()).find(|k| **k == key).copied() else {
                return Err(bad(n, format!("unknown key '{}' for a {} rule", key, b.kind)));
            };
            if value.is_empty() {
                return Err(bad(n, format!("key '{}' takes a value", key)));
            }
            if ONCE.contains(&key) && b.keys.iter().any(|k| k.0 == key) {
                return Err(bad(n, format!("key '{}' given twice in one rule", key)));
            }
            b.keys.push((key, value.to_string(), n));
            continue;
        }
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        match (words[0], blocks.is_empty()) {
            ("rule", _) => {
                if words.len() != 3 {
                    return Err(bad(n, "a rule line is 'rule <ID> <kind>'".to_string()));
                }
                if !crate::sarif::is_class(words[1]) {
                    return Err(bad(n, format!("rule id '{}' is not [A-Z][A-Z0-9_-]*", words[1])));
                }
                let Some(kind) = KINDS.iter().find(|k| k.0 == words[2]).map(|k| k.0) else {
                    return Err(bad(n, format!("unknown kind '{}'", words[2])));
                };
                blocks.push(Block { id: words[1].to_string(), kind, line: n, keys: Vec::new() });
            }
            ("waiver-min", true) => {
                floor = match words[1..] {
                    [v] if !v.is_empty() && v.bytes().all(|c| c.is_ascii_digit()) => {
                        v.parse().map_err(|_| bad(n, format!("waiver-min '{}' is out of range", v)))?
                    }
                    _ => return Err(bad(n, "waiver-min takes one decimal count".to_string())),
                };
            }
            ("body", true) if words.len() > 1 => body.extend(words[1..].iter().map(|w| w.to_string())),
            (w, _) => return Err(bad(n, format!("unknown line '{}' (a top-level line is 'rule', 'waiver-min' or 'body', the last two before the first rule)", w))),
        }
    }
    let mut rules = Vec::new();
    for b in blocks {
        let (_, req, _) = KINDS.iter().find(|k| k.0 == b.kind).expect("a block's kind is known");
        if let Some(missing) = req.iter().find(|r| !b.keys.iter().any(|k| k.0 == **r)) {
            return Err(bad(b.line, format!("rule {} lacks its '{}' key", b.id, missing)));
        }
        let values = |key: &str| -> Vec<(String, usize)> {
            b.keys.iter().filter(|k| k.0 == key).map(|k| (k.1.clone(), k.2)).collect()
        };
        let compile = |key: &str| -> Result<Vec<Ere>, Fault> {
            values(key).iter().map(|(v, n)| Ere::compile(v).map_err(|e| bad(*n, format!("{}: {}", key, e)))).collect()
        };
        let one = |key: &str| values(key).into_iter().next().map(|k| k.0).unwrap_or_default();
        let all = |key: &str| -> Vec<String> { values(key).into_iter().map(|k| k.0).collect() };
        let kind = match b.kind {
            "command" => Kind::Command { cmd: one("cmd"), allow: compile("allow")?, before: compile("before")? },
            "forbidden-token" => Kind::Forbidden { tokens: all("token") },
            "early-exit" => Kind::EarlyExit,
            _ => Kind::IdGrammar { cmd: one("cmd"), flags: all("flag"), grammar: compile("grammar")?.remove(0) },
        };
        let waivers = all("waiver").iter().flat_map(|w| w.split_whitespace().map(str::to_string)).collect();
        rules.push(Rule { id: b.id, kind, waivers });
    }
    Ok(Rules { floor, body, rules })
}

// spec: gate-sdk/SPEC.md §check-script-preflight — the operands, tracked or not, else the knob's
// pathspecs over the tracked tree, the shared prune dirs applied. `None` is no corpus configured
fn corpus(args: &[String]) -> Result<Option<Vec<String>>, Fault> {
    if !args.is_empty() {
        if let Some(p) = args.iter().find(|p| !Path::new(p).is_file()) {
            return Err(Fault::refused(format!("no script at {}", p)));
        }
        return Ok(Some(args.to_vec()));
    }
    let specs = walk::knob_words("GATE_SDK_SCRIPT_FILES").map_err(Fault::refused)?;
    if specs.is_empty() {
        return Ok(None);
    }
    let prune = walk::prune_dirs().map_err(Fault::refused)?;
    let specs: Vec<&str> = specs.iter().map(String::as_str).collect();
    let listed = listing::tracked(None, &specs).map_err(|e| Fault::tool(e.text()))?;
    Ok(Some(
        listed
            .into_iter()
            .filter(|p| !walk::path_pruned(p, &prune) && std::fs::symlink_metadata(p).is_ok())
            .collect(),
    ))
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

struct Site {
    line: usize,
    text: String,
}

// spec: gate-sdk/SPEC.md §check-script-preflight — a refused site, or its short waiver in its place
fn site(s: &Script, rule: &Rule, floor: usize, at: usize, line: usize, what: String, out: &mut Vec<Site>) {
    let (first, last) = s.logical_at(at).map_or((line, line), |l| (l.first, l.last));
    let text = match shell_lines::waived(s, first, last, &rule.waivers, floor) {
        Waived::Cleared => return,
        Waived::Short => format!("[{}] waiver reason under {} characters", rule.id, floor),
        Waived::No => format!("[{}] {}", rule.id, what),
    };
    out.push(Site { line, text });
}

fn invocations<'a>(s: &'a Script, cmd: &'a str) -> impl Iterator<Item = (usize, Vec<&'a Word>, usize)> + 'a {
    s.commands.iter().flat_map(move |c| {
        let args: Vec<&Word> = c.args().collect();
        (0..args.len())
            .filter(|k| !args[*k].quoted && args[*k].value == cmd)
            .map(|k| (k, args.clone(), c.end))
            .collect::<Vec<_>>()
    })
}

fn check(s: &Script, rule: &Rule, floor: usize, out: &mut Vec<Site>) {
    match &rule.kind {
        Kind::Command { cmd, allow, before } => {
            for (k, args, end) in invocations(s, cmd) {
                let w = args[k];
                let text = collapse(&s.code(w.start, end));
                if allow.iter().any(|e| e.is_match(&text)) {
                    continue;
                }
                let lead = s.logical_at(w.start).map(|l| s.code(l.start, w.start)).unwrap_or_default();
                if before.iter().any(|e| e.is_match(&lead)) {
                    continue;
                }
                site(s, rule, floor, w.start, w.line, cmd.clone(), out);
            }
        }
        Kind::Forbidden { tokens } => {
            for l in &s.logicals {
                let code = s.code(l.start, l.end);
                for (i, _) in tokens.iter().enumerate().filter(|(_, t)| code.contains(t.as_str())) {
                    site(s, rule, floor, l.start, l.first, format!("token {}", i + 1), out);
                }
            }
        }
        Kind::EarlyExit => {}
        Kind::IdGrammar { cmd, flags, grammar } => {
            for (k, args, _) in invocations(s, cmd) {
                for (j, a) in args.iter().enumerate().skip(k + 1) {
                    for f in flags {
                        let value = if !a.quoted && a.value == *f {
                            args.get(j + 1).map(|v| (v.value.as_str(), v.expands))
                        } else {
                            a.value.strip_prefix(f.as_str()).and_then(|r| r.strip_prefix('=')).map(|v| (v, a.expands))
                        };
                        match value {
                            Some((v, false)) if !grammar.is_match(v) => {
                                site(s, rule, floor, a.start, a.line, f.clone(), out)
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    for b in &s.bodies {
        check(b, rule, floor, out);
    }
}

fn judge(script: &Script, rules: &Rules) -> Vec<Site> {
    let mut out = Vec::new();
    for rule in &rules.rules {
        if let Kind::EarlyExit = rule.kind {
            for h in shell_lines::early_exits(script, &rule.waivers, rules.floor) {
                let text = if h.short_waiver {
                    format!("[{}] waiver reason under {} characters", rule.id, rules.floor)
                } else {
                    format!("[{}] {}", rule.id, h.reader)
                };
                out.push(Site { line: h.line, text });
            }
        } else {
            check(script, rule, rules.floor, &mut out);
        }
    }
    out.sort_by_key(|s| s.line);
    out
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    if let Some(flag) = args.iter().find(|a| a.starts_with('-')) {
        return Err(Fault::usage(format!("unknown option {} — usage: {} [<script>…]", flag, NAME)));
    }
    let Some(files) = corpus(args)? else {
        println!("SCRIPT-PREFLIGHT: clean (not applicable: no script configured)");
        return Ok(0);
    };
    let rules_file = walk::knob_scalar("GATE_SDK_SCRIPT_RULES").map_err(Fault::refused)?;
    let rules = parse(&rules_file, &exit::read_configured(&rules_file, "script rules file")?)?;
    let opts = Options { body: &rules.body, pipefail: false };

    let (mut records, mut unparsed) = (Vec::new(), Vec::new());
    for f in &files {
        let text = super::pipefail_readers::load(f)?;
        match shell_lines::read(&text, &opts) {
            Ok(script) => records.extend(judge(&script, &rules).into_iter().map(|s| format!("{}:{}: {}", f, s.line, s.text))),
            Err(e) => unparsed.push(format!("{}:{}: [LINT-PARSE] unparseable span", f, e.line)),
        }
    }
    if !unparsed.is_empty() {
        for r in records.iter().chain(&unparsed) {
            println!("{}", r);
        }
        return Ok(exit::INPUT);
    }
    if records.is_empty() {
        println!(
            "SCRIPT-PREFLIGHT: clean ({} script(s) read against {} rule(s) in {})",
            files.len(),
            rules.rules.len(),
            rules_file
        );
        return Ok(0);
    }
    println!("{}: script site(s) breaking a rule in {}:", NAME, rules_file);
    for r in &records {
        println!("{}", r);
    }
    println!("  help: rewrite each site to a form its rule allows, or, where the rule names a waiver token,");
    println!("        waive it with `# <token>: <reason>` on a line of its logical line, the reason at least");
    println!("        {} character(s).", rules.floor);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sites(rules: &str, text: &str) -> Vec<String> {
        let rules = parse("rules", rules).expect("parses");
        let script = shell_lines::read(text, &Options { body: &rules.body, pipefail: false }).expect("reads");
        judge(&script, &rules).into_iter().map(|s| format!("{}: {}", s.line, s.text)).collect()
    }

    fn refusal(rules: &str) -> String {
        parse("rules", rules).err().expect("refuses").to_string()
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — an invocation is refused unless an allow
    // matches its text, a before matches the code ahead of it, or a waiver clears it
    #[test]
    fn a_command_rule_reads_its_invocation() {
        let rules = "waiver-min 5\nrule KILL command\n  cmd pkill\n  allow ^pkill -P \"[$][A-Za-z_]+\"$\n  before timeout [0-9]+ $\n  waiver kill-ok\n";
        assert_eq!(sites(rules, "pkill -f job\n"), ["1: [KILL] pkill"]);
        assert!(sites(rules, "pkill -P \"$pid\"\n").is_empty());
        assert_eq!(sites(rules, "pkill \\\n  -P \"$pid\" x\n"), ["1: [KILL] pkill"]);
        assert!(sites(rules, "timeout 5 pkill -f job\n").is_empty());
        assert!(sites(rules, "pkill -f job # kill-ok: the whole job tree\n").is_empty());
        assert_eq!(sites(rules, "pkill -f job # kill-ok: why\n"), ["1: [KILL] waiver reason under 5 characters"]);
        assert!(sites(rules, "echo 'pkill -f job'\n\"pkill\" x\n").is_empty());
        assert_eq!(sites(rules, "x=$(pgrep a | pkill -f b)\n"), ["1: [KILL] pkill"]);
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — a token in a code half, quoted text kept and
    // comments left out, recorded by its position among the rule's tokens
    #[test]
    fn a_forbidden_token_names_its_position() {
        let rules = "rule MOUNT forbidden-token\n  token /mnt/a\n  token /scratch/b\n";
        assert_eq!(sites(rules, "cp x \\\n  \"/scratch/b/y\"\n# /mnt/a\n"), ["1: [MOUNT] token 2"]);
        assert!(sites(rules, "cat <<EOF\n/mnt/a\nEOF\n").is_empty());
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — the detector with the rule's waiver tokens,
    // and a named command's body read as a script of its own
    #[test]
    fn early_exit_and_bodies() {
        let rules = "body sbatch\nrule EARLY early-exit\n  waiver ok also-ok\nrule KILL command\n  cmd pkill\n";
        assert_eq!(sites(rules, "set -o pipefail\na | head -1\nb | head -1 # also-ok: one line\n"), ["2: [EARLY] head"]);
        assert_eq!(sites(rules, "sbatch <<'EOF'\nx\npkill -f y\nEOF\nsbatch --wrap 'pkill z'\n"), ["3: [KILL] pkill", "5: [KILL] pkill"]);
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — each listed flag's value, spaced or after
    // `=`, against the grammar; a value holding an expansion is skipped
    #[test]
    fn an_id_grammar_checks_each_flag_value() {
        let rules = "rule OWNER id-grammar\n  cmd submit\n  flag --owner\n  flag -u\n  grammar ^[a-z][a-z0-9]{2,}$\n";
        assert_eq!(sites(rules, "submit --owner Bob -u 'al'\n"), ["1: [OWNER] --owner", "1: [OWNER] -u"]);
        assert!(sites(rules, "submit --owner=bob -u \"$USER\" --owner=\"${who}\"\n").is_empty());
        assert_eq!(sites(rules, "submit --owner=x1\n"), ["1: [OWNER] --owner"]);
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — a malformed rules file names its file and line
    #[test]
    fn a_malformed_rules_file_names_its_line() {
        assert!(refusal("rule bad command\n  cmd x\n").starts_with("rules:1: rule id"));
        assert!(refusal("rule A nope\n").starts_with("rules:1: unknown kind"));
        assert!(refusal("rule A command\n  cmd x\n  token y\n").starts_with("rules:3: unknown key"));
        assert!(refusal("rule A command\n  allow x\n").starts_with("rules:1: rule A lacks its 'cmd'"));
        assert!(refusal("rule A command\n  cmd x\n  allow (\n").starts_with("rules:3: allow:"));
        assert!(refusal("  cmd x\n").starts_with("rules:1: a key line"));
        assert!(refusal("rule A early-exit\nwaiver-min 3\n").starts_with("rules:2: unknown line"));
        assert!(refusal("waiver-min x\n").starts_with("rules:1: waiver-min"));
        assert!(refusal("rule A command\n  cmd x\n  cmd y\n").starts_with("rules:3: key 'cmd' given twice"));
    }
}
