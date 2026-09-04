// spec: lifecycle-kit/SPEC.md §check-scratch-citation — no permanent surface carries a
// retrieval pointer into a boundary-truncated one, because that pointer resolves to nothing
// at the next iteration boundary
use crate::spec;
use crate::stages;
use crate::walk;
use std::path::Path;

const EXEMPT: &str = "scratch-citation-exempt:";

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\u{b}' || c == '\u{c}'
}

// spec: lifecycle-kit/SPEC.md §check-scratch-citation — the blank-line paragraph join a
// citation-liveness scan needs: a wrapped bullet routinely splits the colon from the path it
// introduces, and a physical-line scanner is blind on exactly that case
struct Para {
    fnr: Vec<usize>,
    text: Vec<Vec<char>>,
}

impl Para {
    fn new() -> Self {
        Para {
            fnr: Vec::new(),
            text: Vec::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.fnr.is_empty()
    }
    fn reset(&mut self) {
        self.fnr.clear();
        self.text.clear();
    }
}

fn ends_with_colon_space(pre: &[char]) -> bool {
    let mut i = pre.len();
    let mut spaces = 0usize;
    while i > 0 && is_space(pre[i - 1]) {
        i -= 1;
        spaces += 1;
    }
    spaces > 0 && i > 0 && pre[i - 1] == ':'
}

// spec: lifecycle-kit/SPEC.md §check-scratch-citation — the trailing context a colon-introduced
// citation is allowed to carry: an optional closing backtick, then end-of-paragraph or one of
// the sentence punctuation marks
fn closes_citation(after: &[char]) -> bool {
    let a = if after.first() == Some(&'`') {
        &after[1..]
    } else {
        after
    };
    match a.first() {
        None => true,
        Some(&c) if c == '"' || c == '.' || c == ',' || c == ')' => true,
        _ => a.iter().all(|c| is_space(*c)),
    }
}

fn is_link_target(before: &[char], after: &[char]) -> bool {
    let a = if after.first() == Some(&'`') {
        &after[1..]
    } else {
        after
    };
    a.first() == Some(&')') && before.len() >= 2 && before[before.len() - 2..] == [']', '(']
}

fn find_from(hay: &[char], needle: &[char], from: usize) -> Option<usize> {
    if needle.is_empty() || from + needle.len() > hay.len() {
        return None;
    }
    (from..=hay.len() - needle.len()).find(|&i| hay[i..i + needle.len()] == *needle)
}

// spec: lifecycle-kit/SPEC.md §check-scratch-citation — one hit per (paragraph, target,
// position). The shell form iterates targets in awk's hash order; the port iterates the
// derived set in its own sorted order, so a multi-target paragraph reports deterministically.
fn scan_para(para: &Para, targets: &[Vec<char>], out: &mut Vec<(usize, &'static str, usize)>) {
    if para.is_empty() {
        return;
    }
    let mut joined: Vec<char> = Vec::new();
    let mut lstart: Vec<usize> = Vec::with_capacity(para.text.len());
    for (i, t) in para.text.iter().enumerate() {
        lstart.push(joined.len() + 1);
        if i > 0 {
            joined.push(' ');
        }
        joined.extend_from_slice(t);
    }
    for (ti, t) in targets.iter().enumerate() {
        let mut scanpos = 0usize;
        while let Some(m) = find_from(&joined, t, scanpos) {
            let mend = m + t.len();
            let mut li = 0usize;
            for (i, s) in lstart.iter().enumerate() {
                if *s <= m + 1 {
                    li = i;
                }
            }
            let before = &joined[..m];
            let after = &joined[mend..];
            let pre: &[char] = if before.last() == Some(&'`') {
                &before[..before.len() - 1]
            } else {
                before
            };
            if is_link_target(before, after) {
                out.push((para.fnr[li], "markdown link target", ti));
            } else if ends_with_colon_space(pre) && closes_citation(after) {
                out.push((para.fnr[li], "colon-introduced citation", ti));
            }
            scanpos = mend;
        }
    }
}

pub fn run(args: &[String]) -> i32 {
    let globs: Vec<String> = if args.iter().any(|a| !a.is_empty()) {
        args.to_vec()
    } else {
        match walk::knob_array("LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-scratch-citation: {}", e);
                return 2;
            }
        }
    };

    // spec: lifecycle-kit/SPEC.md §check-scratch-citation — the forbidden-target set is the
    // supersede set's third reader, so a consumer adding a LIFECYCLE_KIT_BOUNDARY_TRUNCATE
    // member gets citation enforcement over it with no second roster to update
    let mut targets: Vec<String> = match stages::supersede_set() {
        Ok(v) => v.into_iter().filter(|s| !s.is_empty()).collect(),
        Err(e) => {
            eprintln!("check-scratch-citation: {}", e);
            return 2;
        }
    };
    targets.sort();
    targets.dedup();
    if targets.is_empty() {
        eprintln!("check-scratch-citation: the derived boundary-truncated set is empty — the state machine names no such surface (a lifecycle always owns at least its state + lesson-evidence files)");
        return 2;
    }
    let tchars: Vec<Vec<char>> = targets.iter().map(|t| t.chars().collect()).collect();

    let files: Vec<String> = match walk::glob_files(Path::new("."), &globs) {
        Ok(v) => v
            .into_iter()
            .map(|p| spec::strip_dot_slash(&p.display().to_string()))
            .collect(),
        Err(e) => {
            eprintln!("check-scratch-citation: {}", e);
            return 2;
        }
    };
    if files.is_empty() {
        println!("SCRATCH-CITATION: clean (no permanent surface configured — LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS matched nothing)");
        return 0;
    }

    let mut findings: Vec<String> = Vec::new();
    for f in &files {
        let text = match std::fs::read(f) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => {
                eprintln!("check-scratch-citation: cannot read {}: {}", f, e);
                return 2;
            }
        };
        let lines: Vec<&str> = text.lines().collect();
        let mut hits: Vec<(usize, &'static str, usize)> = Vec::new();
        let mut para = Para::new();
        let mut fence = false;
        for (idx, raw) in lines.iter().enumerate() {
            let lead = raw.trim_start_matches(is_space);
            if lead.starts_with("```") {
                scan_para(&para, &tchars, &mut hits);
                para.reset();
                fence = !fence;
                continue;
            }
            if fence || lead.is_empty() {
                scan_para(&para, &tchars, &mut hits);
                para.reset();
                continue;
            }
            para.fnr.push(idx + 1);
            para.text.push(raw.chars().collect());
        }
        scan_para(&para, &tchars, &mut hits);

        // spec: lifecycle-kit/SPEC.md §check-scratch-citation — the escape hatch is checked on
        // the line before the hit's physical line, the repo's established per-line opt-out
        // shape, for a surface that must quote a dead citation verbatim in order to describe it
        for (lineno, kind, ti) in hits {
            if lineno > 1 && lines[lineno - 2].contains(EXEMPT) {
                continue;
            }
            findings.push(format!(
                "  {}:{}: {} into the boundary-truncated {}",
                f, lineno, kind, targets[ti]
            ));
        }
    }

    if !findings.is_empty() {
        println!("check-scratch-citation: permanent surface(s) point a reader into per-iteration scratch:");
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: a boundary-truncated surface is emptied by the next --enter-stage boundary reset, so the");
        println!("        pointer resolves to nothing one iteration after it is written. Inline the finding instead —");
        println!("        bash gate-sdk/bin/run-gates.sh --emit cite-survey \"<heading-substring>\" emits the block's");
        println!("        heading and all five witness fields, which is what keeps it re-usable rather than");
        println!("        merely readable.");
        println!("        A surface that must quote a dead citation verbatim tags the line above it");
        println!("        'scratch-citation-exempt: <reason>'.");
        return 1;
    }

    println!(
        "SCRATCH-CITATION: clean ({} permanent surface(s) carry no retrieval pointer into the derived boundary-truncated set)",
        files.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn para_of(lines: &[&str]) -> Para {
        let mut p = Para::new();
        for (i, l) in lines.iter().enumerate() {
            p.fnr.push(i + 1);
            p.text.push(l.chars().collect());
        }
        p
    }

    fn hits(lines: &[&str]) -> Vec<(usize, &'static str, usize)> {
        let t = vec![".workflow/survey-record.md".chars().collect::<Vec<char>>()];
        let mut out = Vec::new();
        scan_para(&para_of(lines), &t, &mut out);
        out
    }

    // spec: lifecycle-kit/SPEC.md §check-scratch-citation — the wrapped case is the one the
    // gate exists for: the colon ends one physical line and the path opens the next, and the
    // hit must still report the physical line the path sits on
    #[test]
    fn a_colon_introduced_pointer_is_found_across_a_line_wrap() {
        assert_eq!(
            hits(&["The census and its witness:", "`.workflow/survey-record.md`. Rest."]),
            vec![(2, "colon-introduced citation", 0)]
        );
        assert_eq!(
            hits(&["Full finding: `.workflow/survey-record.md`"]),
            vec![(1, "colon-introduced citation", 0)]
        );
    }

    #[test]
    fn a_link_target_is_its_own_kind_and_a_bare_mention_is_neither() {
        assert_eq!(
            hits(&["see [the record](.workflow/survey-record.md) and more"]),
            vec![(1, "markdown link target", 0)]
        );
        assert!(hits(&["`.workflow/survey-record.md` exists so a later stage need not"]).is_empty());
        assert!(hits(&["the last stamp in", "`.workflow/survey-record.md`, and stage motion"]).is_empty());
    }
}
