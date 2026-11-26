// spec: gate-sdk/SPEC.md §check-pipefail-readers — no early-exit reader is fed under `pipefail` in
// the configured shell tree beyond its baselined count
use crate::count_baseline;
use crate::exit::{self, Fault};
use crate::shell_lines::{self, Hit, Options};
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-pipefail-readers";
const ID: &str = "PIPEFAIL-EARLY-EXIT";

// spec: gate-sdk/SPEC.md §check-pipefail-readers — the waiver token, its reason floor the knob's
pub const TOKEN: &str = "pipefail-ok";
pub const FLOOR_KNOB: &str = "GATE_SDK_PIPEFAIL_WAIVER_MIN";

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

pub struct Config {
    libraries: Vec<String>,
    all: bool,
    floor: usize,
}

pub fn config() -> Result<Config, Fault> {
    let libraries = walk::knob_words("GATE_SDK_PIPEFAIL_LIBRARIES").map_err(Fault::refused)?;
    let all = walk::knob_scalar("GATE_SDK_PIPEFAIL_ALL").map_err(Fault::refused)? == "1";
    let raw = walk::knob_scalar(FLOOR_KNOB).map_err(Fault::refused)?;
    let floor = crate::knobs::gate_sdk::positive(&raw)
        .ok_or_else(|| Fault::refused(format!("{} must be a positive integer (got '{}')", FLOOR_KNOB, raw)))?;
    Ok(Config { libraries, all, floor })
}

fn normalized(p: &str) -> String {
    let p = p.strip_prefix("./").unwrap_or(p);
    let p = p.trim_end_matches('/');
    if p.is_empty() { ".".to_string() } else { p.to_string() }
}

// spec: gate-sdk/SPEC.md §check-pipefail-readers — the operands, else the knob; a file is taken as
// named and a directory searched for `*.sh` under the shared prune dirs. `None` is no corpus
// configured, and the bool says whether operands named it
pub fn corpus(args: &[String]) -> Result<Option<(Vec<String>, bool)>, Fault> {
    let operands = !args.is_empty();
    let named = if operands {
        args.to_vec()
    } else {
        walk::knob_words("GATE_SDK_PIPEFAIL_PATHS").map_err(Fault::refused)?
    };
    if named.is_empty() {
        return Ok(None);
    }
    let prune = walk::prune_dirs().map_err(Fault::refused)?;
    let mut files = BTreeSet::new();
    for p in &named {
        let path = Path::new(p);
        if path.is_file() {
            files.insert(normalized(p));
        } else if path.is_dir() {
            let found = walk::find_with_prune(path, &|n| prune.iter().any(|d| d == n)).map_err(Fault::input)?;
            for f in found.iter().filter(|f| f.extension().is_some_and(|e| e == "sh")) {
                files.insert(normalized(&f.to_string_lossy()));
            }
        } else {
            return Err(Fault::refused(format!("no shell file or directory at {}", p)));
        }
    }
    Ok(Some((files.into_iter().collect(), operands)))
}

pub fn load(path: &str) -> Result<String, Fault> {
    let bytes = std::fs::read(path).map_err(|e| Fault::input(format!("cannot read {}: {}", path, e)))?;
    String::from_utf8(bytes).map_err(|_| Fault::input(format!("{} is not UTF-8 text", path)))
}

pub enum Lint {
    Off,
    Hits(Vec<Hit>),
    Unparsed(usize),
}

// spec: gate-sdk/SPEC.md §check-pipefail-readers — linted when a code line sets `pipefail`, when
// the basename matches a library glob, or when every file is
pub fn lint(path: &str, text: &str, cfg: &Config) -> Lint {
    let base = path.rsplit('/').next().unwrap_or(path);
    let whole = cfg.all || cfg.libraries.iter().any(|g| walk::glob_match(g, base));
    let script = match shell_lines::read(text, &Options { body: &[], pipefail: whole }) {
        Ok(s) => s,
        Err(e) => return Lint::Unparsed(e.line),
    };
    if !whole && !script.sets_pipefail {
        return Lint::Off;
    }
    Lint::Hits(shell_lines::early_exits(&script, &[TOKEN.to_string()], cfg.floor))
}

pub fn record(path: &str, hit: &Hit, cfg: &Config) -> String {
    let fix = if hit.fix.is_some() { " [auto-fix]" } else { "" };
    if hit.short_waiver {
        return format!("{}:{}: [{}] waiver reason under {} characters{}", path, hit.line, ID, cfg.floor, fix);
    }
    format!("{}:{}: [{}] {}{}", path, hit.line, ID, hit.reader, fix)
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    if let Some(flag) = args.iter().find(|a| a.starts_with('-')) {
        return Err(Fault::usage(format!("{}: unknown option {} — usage: {} [<path>…]", NAME, flag, NAME)));
    }
    let Some((files, operands)) = corpus(args).map_err(|e| e.prefixed(NAME))? else {
        println!("PIPEFAIL-READERS: clean (not applicable: no shell path configured)");
        return Ok(0);
    };
    let cfg = config().map_err(|e| e.prefixed(NAME))?;
    let baseline_path = walk::knob_scalar("GATE_SDK_PIPEFAIL_BASELINE").map_err(|e| Fault::refused(e).prefixed(NAME))?;
    let baseline = if baseline_path.is_empty() {
        None
    } else {
        Some(count_baseline::read(&baseline_path).map_err(|e| e.prefixed(NAME))?)
    };

    let (mut records, mut unparsed, mut stale) = (Vec::new(), Vec::new(), Vec::new());
    let (mut linted, mut within, mut fixable) = (0usize, 0usize, 0usize);
    let mut found = std::collections::BTreeMap::new();
    for f in &files {
        let text = load(f).map_err(|e| e.prefixed(NAME))?;
        let hits = match lint(f, &text, &cfg) {
            Lint::Off => continue,
            Lint::Unparsed(line) => {
                unparsed.push(format!("{}:{}: [LINT-PARSE] unparseable span", f, line));
                continue;
            }
            Lint::Hits(h) => h,
        };
        linted += 1;
        found.insert(f.clone(), hits.len());
        let ceiling = baseline.as_ref().map_or(0, |b| b.get(f).copied().unwrap_or(0));
        if hits.len() > ceiling {
            fixable += hits.iter().filter(|h| h.fix.is_some()).count();
            records.extend(hits.iter().map(|h| record(f, h, &cfg)));
        } else {
            within += hits.len();
        }
    }
    // spec: gate-sdk/SPEC.md §The count baseline — a ceiling: a row over its file's hits is stale;
    // a row naming a file outside the corpus is read only when the knob, not operands, named it
    if let Some(b) = &baseline {
        for (path, count) in b {
            let n = match found.get(path) {
                Some(n) => *n,
                None if operands && !files.contains(path) => continue,
                None => 0,
            };
            if n < *count && !unparsed.iter().any(|u: &String| u.starts_with(&format!("{}:", path))) {
                stale.push(format!("{}: [BASELINE-STALE] {}/{}", path, n, count));
            }
        }
    }

    if !unparsed.is_empty() {
        for r in records.iter().chain(&stale).chain(&unparsed) {
            println!("{}", r);
        }
        return Ok(exit::INPUT);
    }
    if records.is_empty() && stale.is_empty() {
        println!(
            "PIPEFAIL-READERS: clean ({} shell file(s) read, {} linted under `pipefail`; {} early-exit reader(s) within the baseline)",
            files.len(),
            linted,
            within
        );
        return Ok(0);
    }
    if !records.is_empty() {
        println!("{}: early-exit reader(s) fed under `pipefail`, whose stop the producer's SIGPIPE turns into the pipeline's status:", NAME);
        for r in &records {
            println!("{}", r);
        }
    }
    for s in &stale {
        println!("{}", s);
    }
    if !records.is_empty() {
        if fixable > 0 {
            let door = super::door_command("--emit pipefail-fix <path>…").map_err(Fault::refused)?;
            println!("  help: rewrite every [auto-fix] site with {}", door);
            println!("        and fix any other by hand so the reader reads its input to the end, or waive");
        } else {
            println!("  help: fix each site by hand so the reader reads its input to the end, or waive");
        }
        println!("        one with `# {}: <reason>` on a line of its pipeline.", TOKEN);
    }
    if !stale.is_empty() {
        println!("  help: lower each BASELINE-STALE row of {} to the found count.", baseline_path);
    }
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(libraries: &[&str], all: bool) -> Config {
        Config { libraries: libraries.iter().map(|s| s.to_string()).collect(), all, floor: 1 }
    }

    fn readers(path: &str, text: &str, c: &Config) -> Option<Vec<String>> {
        match lint(path, text, c) {
            Lint::Hits(h) => Some(h.iter().map(|h| record(path, h, c)).collect()),
            Lint::Off => None,
            Lint::Unparsed(l) => panic!("unparsed at {}", l),
        }
    }

    // spec: gate-sdk/SPEC.md §check-pipefail-readers — linted from the set, by a library glob, or
    // all; a file in none of the three is not linted
    #[test]
    fn a_file_is_linted_from_its_set_by_its_name_or_by_the_switch() {
        let c = cfg(&["*lib.sh", "*-lib*"], false);
        let text = "a | head -1\nset -o pipefail\nb | grep -q x\n";
        assert_eq!(readers("s.sh", text, &c).unwrap(), ["s.sh:3: [PIPEFAIL-EARLY-EXIT] grep -q [auto-fix]"]);
        assert_eq!(readers("d/x-lib.sh", text, &c).unwrap().len(), 2);
        assert!(readers("plain.sh", "a | head -1\n", &c).is_none());
        assert_eq!(readers("plain.sh", "a | head -1\n", &cfg(&[], true)).unwrap().len(), 1);
        let waived = readers("w.sh", "set -o pipefail\na | read x # pipefail-ok:\n", &c).unwrap();
        assert_eq!(waived, ["w.sh:2: [PIPEFAIL-EARLY-EXIT] waiver reason under 1 characters"]);
    }

    #[test]
    fn operands_normalize() {
        assert_eq!(normalized("./a/b.sh"), "a/b.sh");
        assert_eq!(normalized("dir/"), "dir");
        assert_eq!(normalized("./"), ".");
    }
}
