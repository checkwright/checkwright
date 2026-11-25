// spec: gate-sdk/SPEC.md §check-hook-knobs-fresh — one knob table per hook page, the member's own
// knob slice in slice order; one function the gate calls in process
use crate::emit::support_table as st;
use crate::exit::Fault;
use crate::hook;
use crate::knobs::{self, Value};
use crate::walk;
use std::path::Path;

pub const PAGES_KNOB: &str = "GATE_SDK_HOOK_PAGES_DIR";
const BEGIN: &str = "<!-- hook-knobs:begin -->";
const END: &str = "<!-- hook-knobs:end -->";
const EMPTY: &str = "(empty)";

fn code(s: &str) -> String {
    format!("`{}`", s.replace('|', "\\|"))
}

fn cell(v: Value) -> String {
    let parts: Vec<String> = match v {
        Value::Scalar(s) if s.is_empty() => Vec::new(),
        Value::Scalar(s) => vec![s],
        Value::Indexed(e) => e,
        Value::Keyed(m) => m.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect(),
    };
    if parts.is_empty() {
        return EMPTY.to_string();
    }
    parts.iter().map(|p| code(p)).collect::<Vec<_>>().join(", ")
}

// spec: gate-sdk/SPEC.md §check-hook-knobs-fresh — a table with no row renders as no table
const NO_KNOB: &str = "This hook declares no knob of its own.";

// spec: gate-sdk/SPEC.md §check-hook-knobs-fresh — the table between the markers' blank lines
pub fn render(member: &str) -> Result<String, String> {
    let slice = hook::knobs(member).ok_or_else(|| format!("{} is not a hook member", member))?;
    if slice.is_empty() {
        return Ok(format!("\n{}\n", NO_KNOB));
    }
    let mut out = vec!["| Knob | Kit | Shape | Default |".to_string(), "| --- | --- | --- | --- |".to_string()];
    for name in slice {
        let (kit, shape, v) = knobs::default_of(name)?;
        out.push(format!("| {} | {} | {} | {} |", code(name), kit, shape, cell(v)));
    }
    Ok(format!("\n{}\n", out.join("\n")))
}

pub fn block(page: &str, text: &str) -> Result<st::Block, String> {
    st::marked_block(page, text, BEGIN, END)
}

// spec: gate-sdk/SPEC.md §check-hook-knobs-fresh — a page is `<member>.md` directly under a named
// pages dir; the member is its stem
pub fn pages(dirs: &[String]) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for dir in dirs {
        let dir = dir.trim_end_matches('/');
        if !Path::new(dir).is_dir() {
            return Err(format!("hook pages dir not found: {}", dir));
        }
        for f in walk::glob_files(Path::new(dir), &["*.md".to_string()])? {
            let stem = f.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            out.push((f.display().to_string(), stem));
        }
    }
    Ok(out)
}

pub fn is_member(stem: &str) -> bool {
    hook::members().contains(&stem)
}

// spec: gate-sdk/SPEC.md §check-hook-knobs-fresh — the bare arm prints each page's block; `--write`
// rewrites the blocks and touches nothing else
pub fn emit(args: &[String]) -> Result<String, Fault> {
    let dirs = walk::knob_words(PAGES_KNOB).map_err(Fault::refused)?;
    if dirs.is_empty() {
        return Ok(format!("hook-knobs: {} is empty — no hook page is governed\n", PAGES_KNOB));
    }
    let write = args.iter().any(|a| a == "--write");
    let mut out = String::new();
    for (page, member) in pages(&dirs).map_err(Fault::input)? {
        if !is_member(&member) {
            return Err(Fault::refused(format!("{}: names no hook member — `--hook` with no member lists them", page)));
        }
        let table = render(&member).map_err(Fault::tool)?;
        if write {
            out.push_str(&st::write_marked("hook-knobs", &page, BEGIN, END, &table)?);
        } else {
            out.push_str(&format!("{}:\n{}\n", page, table.trim_matches('\n')));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_follows_the_slice_and_renders_an_empty_default_as_a_word() {
        let t = render("subagent-stop-liveness").expect("render");
        let rows: Vec<&str> = t.trim().lines().skip(2).collect();
        let slice = hook::knobs("subagent-stop-liveness").unwrap();
        assert_eq!(rows.len(), slice.len());
        for (row, name) in rows.iter().zip(slice.iter()) {
            assert!(row.starts_with(&format!("| `{}` |", name)), "{}", row);
        }
        assert!(t.contains("| `DELEGATION_KIT_LIVENESS_CMD` | delegation-kit | indexed | (empty) |"), "{}", t);
        assert!(t.contains("| `GATE_SDK_TMP_DIR` | gate-sdk | scalar | `.tmp` |"), "{}", t);
        assert!(!t.contains("GATE_SDK_HOOK_OBSERVE_LOG"), "{}", t);
    }

    #[test]
    fn an_empty_slice_renders_its_line_and_no_table() {
        assert_eq!(hook::knobs("escalation-guard"), Some(&[][..]));
        assert_eq!(render("escalation-guard").expect("render"), format!("\n{}\n", NO_KNOB));
    }

    #[test]
    fn a_non_member_and_a_value_with_a_pipe() {
        assert!(render("no-such-hook").is_err());
        assert_eq!(cell(Value::Indexed(vec!["a|b".into(), "c".into()])), "`a\\|b`, `c`");
        assert_eq!(cell(Value::Keyed(Vec::new())), EMPTY);
        assert_eq!(cell(Value::Scalar(String::new())), EMPTY);
    }
}
