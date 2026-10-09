// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — the generator of every agent
// definition's `model:` line, and its `effort:` line where the class binds one: the pair bound to
// the class its `tier:` field declares.
use crate::walk;
use std::path::Path;

pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_AGENT_DIR",
    "DELEGATION_KIT_TIER_MODEL",
    "GATE_SDK_PRUNE_DIRS",
    "GATE_SDK_PRUNE_EXTRA_DIRS",
];

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — one tiered definition's reading: its
// class, its current `model:` and `effort:` values, the pair the binding expects, and the whole file
// as `--write` would leave it.
pub struct Tiered {
    pub path: String,
    pub current: Option<String>,
    pub expected: String,
    pub current_effort: Option<String>,
    pub expected_effort: Option<String>,
    pub before: String,
    pub after: String,
}

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — a field read inside the first
// frontmatter block only, the block `check-agent-tier-explicit` reads `model:` in
pub fn field(text: &str, key: &str) -> Option<String> {
    let mut lines = text.lines();
    if lines.next() != Some("---") {
        return None;
    }
    for line in lines {
        if line == "---" {
            return None;
        }
        if let Some(rest) = line.strip_prefix(key).and_then(|r| r.strip_prefix(':')) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — `--write`'s rewrite of the `model:` line
// and, under a bound effort, the `effort:` line: each replaced where it stands or inserted, and no
// other byte moves, line endings included.
pub fn rewrite(text: &str, expected: &str, effort: Option<&str>) -> String {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let close = lines.iter().skip(1).position(|l| l.trim_end_matches(['\n', '\r']) == "---").map(|i| i + 1);
    let Some(close) = close.filter(|_| lines.first().map(|l| l.trim_end_matches(['\n', '\r'])) == Some("---")) else {
        return text.to_string();
    };
    let eol = |l: &str| if l.ends_with("\r\n") { "\r\n" } else { "\n" };
    let is = |l: &str, key: &str| l.strip_prefix(key).is_some_and(|r| r.starts_with(':'));
    let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    let mut close = close;
    let model = if let Some(i) = (1..close).find(|i| is(lines[*i], "model")) {
        out[i] = format!("model: {}{}", expected, eol(lines[i]));
        Some(i)
    } else if let Some(i) = (1..close).find(|i| is(lines[*i], "tier")) {
        out.insert(i + 1, format!("model: {}{}", expected, eol(lines[i])));
        close += 1;
        Some(i + 1)
    } else {
        None
    };
    if let Some(effort) = effort {
        if let Some(i) = (1..close).find(|i| is(&out[*i], "effort")) {
            out[i] = format!("effort: {}{}", effort, eol(&out[i]));
        } else if let Some(i) = model {
            let line = format!("effort: {}{}", effort, eol(&out[i]));
            out.insert(i + 1, line);
        }
    }
    out.concat()
}

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — every tiered definition under the
// directory, resolved against the binding before anything is written: an unbound class is an
// error naming the definition, and an untiered definition is left out.
pub fn plan(dir: &str, binding: &[String]) -> Result<Vec<Tiered>, String> {
    let root = Path::new(dir);
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut files = walk::find_files(root, &["md"])?;
    files.sort();
    let mut out: Vec<Tiered> = Vec::new();
    let mut unbound: Vec<String> = Vec::new();
    for f in &files {
        let before = std::fs::read_to_string(f).map_err(|e| format!("cannot read {}: {}", f.display(), e))?;
        let Some(class) = field(&before, "tier") else {
            continue;
        };
        let path = f.display().to_string();
        let Some(expected) = crate::tier::bound(binding, &class) else {
            unbound.push(format!("{}: tier '{}' names no bound class", path, class));
            continue;
        };
        let effort = crate::tier::effort(binding, &class);
        out.push(Tiered {
            current: field(&before, "model"),
            expected: expected.to_string(),
            current_effort: field(&before, "effort"),
            expected_effort: effort.map(str::to_string),
            after: rewrite(&before, expected, effort),
            path,
            before,
        });
    }
    if !unbound.is_empty() {
        return Err(format!(
            "{}\n  help: bind the class in DELEGATION_KIT_TIER_MODEL, or declare a bound class in the definition's tier: field",
            unbound.join("\n")
        ));
    }
    Ok(out)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let write = args.iter().any(|a| a == "--write");
    let binding = walk::knob_array("DELEGATION_KIT_TIER_MODEL")?;
    let dir = walk::knob_scalar("DELEGATION_KIT_AGENT_DIR")?;
    generate(dir.trim_end_matches('/'), &binding, write)
}

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — an empty binding is off: nothing is
// read or written
fn generate(dir: &str, binding: &[String], write: bool) -> Result<String, String> {
    if binding.is_empty() {
        return Ok("binding off\n".to_string());
    }
    report(&plan(dir, binding)?, write)
}

fn report(defs: &[Tiered], write: bool) -> Result<String, String> {
    let mut out = String::new();
    for d in defs {
        if d.before == d.after {
            out.push_str(&format!("{}: current\n", d.path));
            continue;
        }
        let effort = d.expected_effort.as_ref().map_or(String::new(), |e| {
            format!(", effort: {} -> {}", d.current_effort.as_deref().unwrap_or("-"), e)
        });
        out.push_str(&format!(
            "{}: model: {} -> {}{}\n",
            d.path,
            d.current.as_deref().unwrap_or("-"),
            d.expected,
            effort
        ));
    }
    if write {
        for d in defs.iter().filter(|d| d.before != d.after) {
            std::fs::write(&d.path, &d.after).map_err(|e| format!("cannot write {}: {}", d.path, e))?;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> String {
        let dir = std::env::temp_dir().join(format!("checkwright-agent-tiers.{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("the scratch agent dir must be creatable");
        dir.display().to_string()
    }

    fn binding() -> Vec<String> {
        vec!["judgment=big".to_string(), "mechanical=small".to_string()]
    }

    // spec: delegation-kit/SPEC.md §check-agent-tier-explicit — the rewrite replaces a stale model,
    // inserts a missing one after tier:, and moves no other byte
    #[test]
    fn the_rewrite_touches_the_model_line_alone() {
        let stale = "---\nname: a\ntier: judgment\nmodel: small\n---\nbody model: x\n";
        assert_eq!(rewrite(stale, "big", None), "---\nname: a\ntier: judgment\nmodel: big\n---\nbody model: x\n");
        let missing = "---\nname: a\ntier: judgment\ndescription: d\n---\n";
        assert_eq!(rewrite(missing, "big", None), "---\nname: a\ntier: judgment\nmodel: big\ndescription: d\n---\n");
        let crlf = "---\r\ntier: judgment\r\nmodel: small\r\n---\r\n";
        assert_eq!(rewrite(crlf, "big", None), "---\r\ntier: judgment\r\nmodel: big\r\n---\r\n");
        assert_eq!(rewrite("no frontmatter\n", "big", None), "no frontmatter\n");
    }

    // spec: delegation-kit/SPEC.md §check-agent-tier-explicit — a bound effort replaces the effort
    // line where it stands, is inserted after model: where none exists, and an unbound one leaves
    // the line as written
    #[test]
    fn the_rewrite_holds_the_effort_line_where_the_class_binds_one() {
        let deep = Some("deep");
        let stale = "---\ntier: judgment\nmodel: big\neffort: low\n---\nbody effort: x\n";
        assert_eq!(rewrite(stale, "big", deep), "---\ntier: judgment\nmodel: big\neffort: deep\n---\nbody effort: x\n");
        let apart = "---\neffort: low\ntier: judgment\nmodel: big\n---\n";
        assert_eq!(rewrite(apart, "big", deep), "---\neffort: deep\ntier: judgment\nmodel: big\n---\n");
        let missing = "---\ntier: judgment\nmodel: small\nname: a\n---\n";
        assert_eq!(rewrite(missing, "big", deep), "---\ntier: judgment\nmodel: big\neffort: deep\nname: a\n---\n");
        let bare = "---\r\ntier: judgment\r\n---\r\n";
        assert_eq!(rewrite(bare, "big", deep), "---\r\ntier: judgment\r\nmodel: big\r\neffort: deep\r\n---\r\n");
        assert_eq!(rewrite(stale, "big", None), stale, "an unbound effort leaves the line as written");
        assert_eq!(rewrite(&rewrite(missing, "big", deep), "big", deep), rewrite(missing, "big", deep));
        let dir = scratch("effort");
        std::fs::write(format!("{}/a.md", dir), missing).unwrap();
        std::fs::write(format!("{}/b.md", dir), "---\ntier: mechanical\nmodel: small\neffort: any\n---\n").unwrap();
        let defs = plan(&dir, &["judgment=big,deep".to_string(), "mechanical=small".to_string()]).expect("a plan");
        let bare = report(&defs, false).expect("the bare report");
        assert!(bare.contains(&format!("{}/a.md: model: small -> big, effort: - -> deep\n", dir)), "{}", bare);
        assert!(bare.contains(&format!("{}/b.md: current\n", dir)), "{}", bare);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: delegation-kit/SPEC.md §check-agent-tier-explicit — the bare report's two line shapes,
    // `--write` leaving the current definition's bytes alone, and an untiered definition left out
    #[test]
    fn the_report_and_the_write_agree() {
        let dir = scratch("write");
        std::fs::write(format!("{}/a.md", dir), "---\ntier: judgment\nmodel: small\n---\n").unwrap();
        std::fs::write(format!("{}/b.md", dir), "---\ntier: mechanical\nmodel: small\n---\n").unwrap();
        std::fs::write(format!("{}/c.md", dir), "---\nmodel: inherit\n---\n").unwrap();
        let defs = plan(&dir, &binding()).expect("a bound plan resolves");
        assert_eq!(defs.len(), 2, "the untiered definition is left out");
        let bare = report(&defs, false).expect("the bare report");
        assert!(bare.contains(&format!("{}/a.md: model: small -> big\n", dir)), "{}", bare);
        assert!(bare.contains(&format!("{}/b.md: current\n", dir)), "{}", bare);
        assert_eq!(std::fs::read_to_string(format!("{}/a.md", dir)).unwrap(), "---\ntier: judgment\nmodel: small\n---\n");
        report(&defs, true).expect("the write");
        assert_eq!(std::fs::read_to_string(format!("{}/a.md", dir)).unwrap(), "---\ntier: judgment\nmodel: big\n---\n");
        assert_eq!(std::fs::read_to_string(format!("{}/c.md", dir)).unwrap(), "---\nmodel: inherit\n---\n");
        let again = plan(&dir, &binding()).expect("a second plan");
        assert!(again.iter().all(|d| d.before == d.after), "the write is idempotent");
        let stale = "---\ntier: judgment\nmodel: small\n---\n";
        std::fs::write(format!("{}/a.md", dir), stale).unwrap();
        assert_eq!(generate(&dir, &[], true).expect("an empty binding"), "binding off\n");
        assert_eq!(std::fs::read_to_string(format!("{}/a.md", dir)).unwrap(), stale, "an empty binding writes nothing");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: delegation-kit/SPEC.md §check-agent-tier-explicit — an unbound class leaves the tree
    // untouched and names the definition
    #[test]
    fn an_unbound_class_writes_nothing() {
        let dir = scratch("unbound");
        let stale = "---\ntier: judgment\nmodel: small\n---\n";
        std::fs::write(format!("{}/a.md", dir), stale).unwrap();
        std::fs::write(format!("{}/z.md", dir), "---\ntier: routing\n---\n").unwrap();
        let err = plan(&dir, &binding()).err().expect("an unbound class refuses");
        assert!(err.contains("z.md: tier 'routing' names no bound class"), "{}", err);
        assert_eq!(std::fs::read_to_string(format!("{}/a.md", dir)).unwrap(), stale);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: delegation-kit/SPEC.md §check-agent-tier-explicit — the field is read inside the first
    // frontmatter block and nowhere past its close
    #[test]
    fn a_field_is_read_inside_the_first_block_only() {
        assert_eq!(field("---\ntier: judgment\n---\n", "tier").as_deref(), Some("judgment"));
        assert_eq!(field("---\n---\ntier: judgment\n", "tier"), None);
        assert_eq!(field("---\ntiers: x\n---\n", "tier"), None);
        assert_eq!(field("tier: judgment\n", "tier"), None);
    }
}
