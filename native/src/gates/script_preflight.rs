// spec: gate-sdk/SPEC.md §check-script-preflight — no configured shell script breaks a rule in the
// consumer's rules file
use crate::ere::Ere;
use crate::exit::{self, Fault};
use crate::listing;
use crate::rule_blocks::{self, Grammar};
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

// spec: gate-sdk/SPEC.md §check-script-preflight — each kind's required keys, then its optional
// ones; `cmd` and `grammar` are given once, every other key repeats
const GRAMMAR: Grammar = Grammar {
    kinds: &[
        ("command", &["cmd"], &["allow", "before", "waiver"]),
        ("forbidden-token", &["token"], &["waiver"]),
        ("early-exit", &[], &["waiver"]),
        ("id-grammar", &["cmd", "flag", "grammar"], &["waiver"]),
    ],
    once: &["cmd", "grammar"],
    tops: &["waiver-min", "body"],
};

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
    let (tops, blocks) = rule_blocks::parse(file, text, &GRAMMAR)?;
    let (mut floor, mut body) = (1usize, Vec::new());
    for top in tops {
        match (top.word, top.values.as_slice()) {
            ("waiver-min", [v]) if v.bytes().all(|c| c.is_ascii_digit()) => {
                floor = v.parse().map_err(|_| rule_blocks::refusal(file, top.line, format!("waiver-min '{}' is out of range", v)))?
            }
            ("waiver-min", _) => return Err(rule_blocks::refusal(file, top.line, "waiver-min takes one decimal count")),
            _ => body.extend(top.values),
        }
    }
    let mut rules = Vec::new();
    for b in blocks {
        let kind = match b.kind {
            "command" => Kind::Command { cmd: b.one("cmd"), allow: b.eres(file, "allow")?, before: b.eres(file, "before")? },
            "forbidden-token" => Kind::Forbidden { tokens: b.all("token") },
            "early-exit" => Kind::EarlyExit,
            _ => Kind::IdGrammar { cmd: b.one("cmd"), flags: b.all("flag"), grammar: b.eres(file, "grammar")?.remove(0) },
        };
        rules.push(Rule { waivers: b.words("waiver"), id: b.id, kind });
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
