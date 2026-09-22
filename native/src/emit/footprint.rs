// spec: context-kit/SPEC.md §bin/footprint — per-kit two-tier context footprint. Two levels:
// `measure` produces the figures and `emit` renders them, so the value-rollup join consumes the
// data rather than re-parsing the page.
use crate::fresh;
use crate::walk;
use std::path::Path;

const SURFACES_KNOB: &str = "CONTEXT_KIT_SURFACES";

pub struct Tally {
    pub cp: usize,
    pub bytes: usize,
}

pub struct KitRow {
    pub kit: String,
    pub always: Tally,
    pub triggered: Tally,
}

pub struct Footprint {
    pub rows: Vec<KitRow>,
    pub always_total: Tally,
    pub triggered_total: Tally,
}

// spec: context-kit/SPEC.md §bin/footprint — the measured set is the kit roster, derived not
// maintained: the shell form's `*/SPEC.md` glob, whose pathname-expansion rule is what skips
// dotted names.
fn kit_roster() -> Result<Vec<String>, String> {
    let mut kits: Vec<String> = walk::glob_files(Path::new("."), &["*/SPEC.md".to_string()])?
        .into_iter()
        .filter_map(|p| {
            p.parent()
                .and_then(|d| d.file_name())
                .map(|n| n.to_string_lossy().into_owned())
        })
        .collect();
    kits.sort();
    kits.dedup();
    Ok(kits)
}

// spec: context-kit/SPEC.md §bin/footprint — always-loaded tier: the content between a kit's
// generated begin/end markers. The marker test is a substring match rather than whole-line
// equality, because the shell form's awk uses index().
fn kit_always(kit: &str, surfaces: &[String]) -> Result<Tally, String> {
    let begin = format!("<!-- {}:begin -->", kit);
    let end = format!("<!-- {}:end -->", kit);
    let mut cp = 0usize;
    let mut bytes = 0usize;
    for sf in surfaces {
        if !Path::new(sf).is_file() {
            continue;
        }
        let text = fresh::read_captured(sf)?;
        let mut inb = false;
        let mut block: Vec<&str> = Vec::new();
        for line in fresh::file_lines(&text) {
            if line.contains(&begin) {
                inb = true;
                continue;
            }
            if line.contains(&end) {
                inb = false;
                continue;
            }
            if inb {
                block.push(line);
            }
        }
        // spec: context-kit/SPEC.md §bin/footprint — the block's trailing newlines are stripped and
        // exactly one re-terminates it for the byte count; an empty block is skipped. The size is
        // the meter's code-point count, so a reflow of the block moves nothing.
        let joined = block.join("\n");
        let joined = joined.trim_end_matches('\n');
        if joined.is_empty() {
            continue;
        }
        cp += crate::section::cp_size(joined.lines());
        bytes += joined.len() + 1;
    }
    Ok(Tally { cp, bytes })
}

// spec: context-kit/SPEC.md §bin/footprint — load-triggered tier: the kit's shipped markdown under
// its templates tree, sized with the meter's code-point count; bytes are bytes.
fn kit_triggered(kit: &str) -> Result<Tally, String> {
    let mut files: Vec<String> =
        walk::glob_files(Path::new(kit), &["templates/**/*.md".to_string()])?
            .into_iter()
            .map(|p| p.display().to_string())
            .collect();
    files.sort();
    files.dedup();
    let mut cp = 0usize;
    let mut bytes = 0usize;
    for f in &files {
        let b = std::fs::read(f).map_err(|e| format!("cannot read {}: {}", f, e))?;
        cp += crate::section::cp_size(String::from_utf8_lossy(&b).lines());
        bytes += b.len();
    }
    Ok(Tally { cp, bytes })
}

pub fn measure() -> Result<Footprint, String> {
    let surfaces = walk::knob_array(SURFACES_KNOB)?;
    let mut rows: Vec<KitRow> = Vec::new();
    let mut always_total = Tally { cp: 0, bytes: 0 };
    let mut triggered_total = Tally { cp: 0, bytes: 0 };
    for kit in kit_roster()? {
        let always = kit_always(&kit, &surfaces)?;
        let triggered = kit_triggered(&kit)?;
        always_total.cp += always.cp;
        always_total.bytes += always.bytes;
        triggered_total.cp += triggered.cp;
        triggered_total.bytes += triggered.bytes;
        rows.push(KitRow {
            kit,
            always,
            triggered,
        });
    }
    Ok(Footprint {
        rows,
        always_total,
        triggered_total,
    })
}

// spec: context-kit/SPEC.md §bin/footprint — code points exact, tokens a bytes/4 estimate computed
// at render; an empty tier is an em dash rather than a zero.
fn cell(t: &Tally) -> String {
    if t.cp == 0 && t.bytes == 0 {
        return "\u{2014}".to_string();
    }
    format!("{}cp \u{b7} ~{}t", t.cp, t.bytes / 4)
}

pub fn table(f: &Footprint) -> String {
    let mut out = String::from("| kit | always-loaded | load-triggered |\n| --- | --- | --- |\n");
    for r in &f.rows {
        out.push_str(&format!(
            "| {} | {} | {} |\n",
            r.kit,
            cell(&r.always),
            cell(&r.triggered)
        ));
    }
    out.push_str(&format!(
        "| **total** | {} | {} |\n",
        cell(&f.always_total),
        cell(&f.triggered_total)
    ));
    out
}

// spec: guard-kit/SPEC.md §check-door-binding — the file-scoped declaration stands before the first
// heading and is written by the emitter, one in emitted output being erased by the next regeneration
const PREAMBLE: &str = r#"---
title: Footprint
nav_parent: value
nav_child_order: 2
---
<!-- door-contributor: a generated projection whose only door is the emitter command behind its own numbers, read by the contributor who regenerates it -->

# Context footprint

What vendoring Checkwright costs a consumer's context budget, measured per kit and split by when the cost is paid. Every number here is generated from the tracked kit surfaces by `bash gate-sdk/bin/run-gates.sh --emit footprint` and held current by a freshness gate, so the page cannot drift from what the kits actually ship.

## What is measured

Each kit's footprint splits by when its cost lands in a session:

- **Always-loaded** — the fixed block a kit injects into the consumer's always-loaded agent file, so it rides every session's context. Measured as the content a kit generates between its own `begin`/`end` markers in the configured surface files.
- **Load-triggered** — the kit's shipped skill and template markdown, pulled into context only when its trigger fires. Measured over the markdown the kit ships under its templates directory.

Code-point counts (`cp`) are exact. The token column is a labeled estimate — a bytes-over-four heuristic, marked with a leading `~` because the true count is model-tokenizer-dependent; read it as an order of magnitude, never a precise figure.

## What is excluded

The figures are kit-share only — what a kit itself ships. A consumer's own bindings (the skill shims that point at a vendored template), consumer configuration, the reference SPEC and README pages a reader opens on demand, and the session hook's dynamic body (which is consumer state, not fixed kit text) are all left out, so each number reflects the kit's advertised cost rather than a host repository's residue.

## Per-kit footprint

"#;

pub fn emit(_args: &[String]) -> Result<String, String> {
    Ok(format!("{}{}", PREAMBLE, table(&measure()?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §bin/footprint — an empty tier renders as an em dash, and the
    // token figure is integer bytes/4, so a sub-4-byte tier reads ~0t rather than rounding up
    #[test]
    fn an_empty_tier_is_an_em_dash_and_tokens_are_integer_bytes_over_four() {
        assert_eq!(cell(&Tally { cp: 0, bytes: 0 }), "\u{2014}");
        assert_eq!(cell(&Tally { cp: 2, bytes: 9 }), "2cp \u{b7} ~2t");
        assert_eq!(cell(&Tally { cp: 1, bytes: 3 }), "1cp \u{b7} ~0t");
    }
}
