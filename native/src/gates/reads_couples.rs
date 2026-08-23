// spec: gate-sdk/SPEC.md §check-reads-couples — every statically resolvable recursive walk in a
// registered gate has its tracked read set covered by the gate's expanded couples; the
// undecidable remainder is skipped-and-counted
use crate::gates;
use crate::proc;
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-reads-couples: {}", e);
            2
        }
    }
}

fn lstrip(s: &str) -> &str {
    s.trim_start_matches([' ', '\t'])
}

// spec: gate-sdk/SPEC.md §check-reads-couples — one record per command-position recursive walk. A
// leading-comment or trailing marker sets the exempt flag, and a trailing one excuses only its own
// line rather than the next walk.
struct Walk {
    lno: usize,
    prune: bool,
    exempt: bool,
    raw: String,
}

fn command_position(line: &str, cmd: &str) -> bool {
    let b = line.as_bytes();
    let c = cmd.as_bytes();
    let mut i = 0usize;
    while i + c.len() < b.len() {
        let opener = i == 0 || matches!(b[i - 1], b'|' | b'&' | b';' | b'(' | b'{' | b'`');
        if !opener {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
            j += 1;
        }
        if b[j..].starts_with(c)
            && j + c.len() < b.len()
            && (b[j + c.len()] == b' ' || b[j + c.len()] == b'\t')
        {
            return true;
        }
        i += 1;
    }
    false
}

fn extract_walks(text: &str) -> Vec<Walk> {
    let mut out = Vec::new();
    let mut prevmarker = false;
    for (idx, line) in text.lines().enumerate() {
        if lstrip(line).starts_with('#') {
            prevmarker = line.contains("reads-couples-exempt:");
            continue;
        }
        let prune = if command_position(line, "gate_find") {
            true
        } else if command_position(line, "find") {
            false
        } else {
            prevmarker = false;
            continue;
        };
        out.push(Walk {
            lno: idx + 1,
            prune,
            exempt: prevmarker || line.contains("reads-couples-exempt:"),
            raw: line.to_string(),
        });
        prevmarker = false;
    }
    out
}

// spec: gate-sdk/SPEC.md §check-reads-couples — the resolvable-root class; the three token shapes
// the shell parser resolves, and `None` for the undecidable remainder
fn resolve_root(cmd: &str, line: &str, src: &str) -> Option<String> {
    let needle = format!("{} ", cmd);
    let at = line.find(&needle)?;
    let rest = &line[at + needle.len()..];
    let tok = rest.split([' ', '\t']).next().unwrap_or("");
    if tok.len() >= 2 && tok.starts_with('"') && tok.ends_with('"') && !tok.contains('$') {
        let t = tok[1..tok.len() - 1]
            .trim_start_matches("./")
            .trim_end_matches('/');
        return Some(if t.is_empty() { ".".to_string() } else { t.to_string() });
    }
    if let Some(sub) = tok.strip_prefix("\"$KIT\"") {
        let sub = sub.trim_start_matches('/').trim_end_matches('/');
        let d1 = src.rsplit_once('/').map(|(a, _)| a).unwrap_or(src);
        let kit = d1.rsplit_once('/').map(|(a, _)| a).unwrap_or(d1);
        return Some(if sub.is_empty() {
            kit.to_string()
        } else {
            format!("{}/{}", kit, sub)
        });
    }
    if let Some(sub) = tok.strip_prefix("\"$REPO_ROOT\"") {
        let sub = sub.trim_start_matches('/').trim_end_matches('/');
        return Some(if sub.is_empty() {
            ".".to_string()
        } else {
            sub.to_string()
        });
    }
    None
}

// spec: gate-sdk/SPEC.md §check-reads-couples — the literal `-name <pat>` primary when one is
// extractable from the same invocation; a variable pattern is not
fn name_pattern(line: &str) -> String {
    if let Some(p) = quoted_after_name(line, '\'') {
        return p;
    }
    if let Some(p) = quoted_after_name(line, '"') {
        return if p.contains('$') { String::new() } else { p };
    }
    String::new()
}

fn quoted_after_name(line: &str, q: char) -> Option<String> {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i + 6 <= b.len() {
        if !line[i..].starts_with("-name") {
            i += 1;
            continue;
        }
        let mut j = i + 5;
        let ws = j;
        while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
            j += 1;
        }
        if j == ws || j >= b.len() || b[j] != q as u8 {
            i += 1;
            continue;
        }
        let close = b[j + 1..].iter().position(|&c| c == q as u8)?;
        return Some(line[j + 1..j + 1 + close].to_string());
    }
    None
}

// spec: gate-sdk/SPEC.md §check-reads-couples — couple glob semantics: segments never cross '/',
// so path and glob must have equal segment count. Deliberately narrower than the slash-spanning
// matcher assertion C reads the same field with.
fn path_matches_glob(path: &str, glob: &str) -> bool {
    if glob == "*" {
        return true;
    }
    let ps: Vec<&str> = path.split('/').collect();
    let gs: Vec<&str> = glob.split('/').collect();
    if ps.len() != gs.len() {
        return false;
    }
    ps.iter().zip(gs.iter()).all(|(p, g)| walk::pattern_match(g, p))
}


struct Ctx {
    prune: Vec<String>,
    findings: Vec<String>,
}

// spec: gate-sdk/SPEC.md §check-reads-couples — one root's demand: what it selects and what the
// member's expanded couples offer against it
struct Demand<'a> {
    root: &'a str,
    prune: bool,
    namepat: &'a str,
    gname: &'a str,
    where_: &'a str,
    globs: &'a [String],
    couples: &'a str,
}

impl Ctx {
    // spec: gate-sdk/SPEC.md §check-reads-couples — the per-root coverage assertion, shared by both
    // substrates: a root the parse resolved and a root the registry reported land here identically
    fn cover_root(&mut self, d: &Demand) -> Result<(), String> {
        let (root, prune, namepat, gname, where_, globs, couples) =
            (d.root, d.prune, d.namepat, d.gname, d.where_, d.globs, d.couples);
        let listing = if root == "." {
            proc::run("git", &["ls-files"])
        } else {
            proc::run("git", &["ls-files", "--", root])
        };
        let out = listing
            .ok()
            .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
            .ok_or_else(|| format!("git ls-files failed for root '{}'", root))?;
        for f in out.lines() {
            if f.is_empty() {
                continue;
            }
            if prune && walk::path_pruned(f, &self.prune) {
                continue;
            }
            if !namepat.is_empty() {
                let base = f.rsplit('/').next().unwrap_or(f);
                if !walk::pattern_match(namepat, base) {
                    continue;
                }
            }
            if globs.iter().any(|g| path_matches_glob(f, g)) {
                continue;
            }
            self.findings.push(format!(
                "{}: {} reads tracked '{}' — no couple covers it (couples: {})",
                gname, where_, f, couples
            ));
        }
        Ok(())
    }
}

fn manifest_field(text: &str, key: &str) -> String {
    for line in text.lines() {
        if let Some(man) = line.strip_prefix("# graph: ") {
            for kv in man.split_whitespace() {
                if let Some(v) = kv.strip_prefix(&format!("{}=", key)) {
                    return v.to_string();
                }
            }
            return String::new();
        }
    }
    String::new()
}

fn expand_couples(field: &str, kit_roots_rel: &[String]) -> String {
    let mut out: Vec<String> = Vec::new();
    for tok in field.split(',') {
        match tok.strip_prefix("kit:") {
            Some(glob) => {
                for r in kit_roots_rel {
                    out.push(format!("{}/{}", r.trim_end_matches('/'), glob));
                }
            }
            None => out.push(tok.to_string()),
        }
    }
    out.join(",")
}

fn rule(args: &[String]) -> Result<i32, String> {
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let list = format!("{}/gates.list", gates_dir);

    let mut sources: Vec<String> = Vec::new();
    if !args.is_empty() {
        sources = args.to_vec();
    } else {
        if !Path::new(&list).is_file() {
            return Err(format!("no registry at {}", list));
        }
        // spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_check_dirs`' own spelling: the gates dir as
        // configured, then each kit root re-absolutised, because a bridged root crosses relative
        // to the invoking directory and the resolved source path travels into every finding
        let mut resolve_dirs = vec![gates_dir.clone()];
        for k in walk::kit_roots_abs()? {
            if k.is_empty() {
                continue;
            }
            resolve_dirs.push(format!("{}/checks", k.trim_end_matches('/')));
        }
        let listing = std::fs::read(&list)
            .map_err(|e| format!("cannot read {}: {}", list, e))?;
        for line in String::from_utf8_lossy(&listing).lines() {
            let t = lstrip(line);
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            for d in &resolve_dirs {
                let sh = format!("{}/{}.sh", d, line);
                if Path::new(&sh).is_file() {
                    sources.push(sh);
                    break;
                }
                let g = format!("{}/{}.gate", d, line);
                if Path::new(&g).is_file() {
                    sources.push(g);
                    break;
                }
            }
        }
    }

    let kit_roots_rel = walk::kit_roots_rel()?;
    let mut ctx = Ctx {
        prune: walk::prune_dirs()?,
        findings: Vec::new(),
    };
    let mut analyzed = 0usize;
    let mut skipped = 0usize;
    let mut exempt = 0usize;

    for src in &sources {
        if !Path::new(src).is_file() {
            continue;
        }
        let base = src.rsplit('/').next().unwrap_or(src);
        let gname = base
            .strip_suffix(".sh")
            .or_else(|| base.strip_suffix(".gate"))
            .unwrap_or(base)
            .to_string();
        let text = std::fs::read(Path::new(src))
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read {}: {}", src, e))?;
        let couples = expand_couples(&manifest_field(&text, "couples"), &kit_roots_rel);
        let globs: Vec<String> = couples.split(',').map(String::from).collect();

        // spec: gate-sdk/SPEC.md §check-reads-couples — a `.gate` member's walks are unreadable to
        // a shell parse, so the registry answers instead. Compiled, that answer is an in-process
        // call rather than a spawn of the arm, which is what dissolves this member's own `c7`.
        if src.ends_with(".gate") {
            let roots = gates::roots(&gname).ok_or_else(|| {
                format!(
                    "{} dispatches to the binary, but no registered subcommand answers for '{}' — \
                     the gate's own read set is unavailable, so this check could not run; treating \
                     as failure (not clean).\n  help: every descriptor must name a subcommand the \
                     binary carries. There is deliberately no descriptor-level exemption: a port \
                     that could opt out of this in a sentence would end the assertion it must \
                     replace.",
                    src, gname
                )
            })?;
            for (root, fknob) in roots {
                // spec: gate-sdk/SPEC.md §check-reads-couples — '?' is the substrate's own honesty
                // marker, counted by the same skip counter the shell arm's unresolvable roots use
                if *root == "?" {
                    skipped += 1;
                    continue;
                }
                let mut namepat = String::new();
                let mut where_ = format!("declared read root '{}' (--reads)", root);
                if !fknob.is_empty() {
                    // spec: gate-sdk/SPEC.md §Fail-closed contract — a named knob the bridge did
                    // not carry is exit 2, never an empty filter silently widening the demand to
                    // the whole root: "cannot resolve" and "no filter" must not share a verdict
                    namepat = std::env::var(format!("GATE_SDK_KNOB_{}", fknob)).map_err(|_| {
                        format!(
                            "{} declares read root '{}' filtered by knob {}, which the config \
                             bridge could not resolve — the coverage assertion could not run; \
                             treating as failure (not clean)",
                            gname, root, fknob
                        )
                    })?;
                    where_ = format!(
                        "declared read root '{}' filtered by {}='{}' (--reads)",
                        root, fknob, namepat
                    );
                }
                analyzed += 1;
                ctx.cover_root(&Demand {
                    root,
                    prune: true,
                    namepat: &namepat,
                    gname: &gname,
                    where_: &where_,
                    globs: &globs,
                    couples: &couples,
                })?;
            }
            continue;
        }

        for w in extract_walks(&text) {
            if w.exempt {
                exempt += 1;
                continue;
            }
            let cmd = if w.prune { "gate_find" } else { "find" };
            let Some(root) = resolve_root(cmd, &w.raw, src) else {
                skipped += 1;
                continue;
            };
            analyzed += 1;
            let namepat = name_pattern(&w.raw);
            let where_ = format!("recursive walk over '{}' (line {})", root, w.lno);
            ctx.cover_root(&Demand {
                root: &root,
                prune: w.prune,
                namepat: &namepat,
                gname: &gname,
                where_: &where_,
                globs: &globs,
                couples: &couples,
            })?;
        }
    }

    if !ctx.findings.is_empty() {
        println!("check-reads-couples: a resolvable recursive walk reads a tracked path its '# graph: couples=' does not cover:");
        for f in &ctx.findings {
            println!("  {}", f);
        }
        println!("  help: add the covering sibling glob to the gate's '# graph: couples=' — a '<dir>/<sub>/*.ext' that matches the deeper path (globs never cross '/', so a shallow one-level couple misses a file one level down), then regenerate the hook + graph artifacts; or mark the walk '# reads-couples-exempt: <reason>' (same line, or the line directly above) when the uncoupled read is deliberate. Never widen a glob to cross '/' to pass a near-miss.");
        return Ok(1);
    }
    println!(
        "READS-COUPLES: clean ({} resolvable walk(s) covered; {} undecidable walk(s) skipped-and-counted; {} exempt; across {} gate(s))",
        analyzed,
        skipped,
        exempt,
        sources.len()
    );
    Ok(0)
}
