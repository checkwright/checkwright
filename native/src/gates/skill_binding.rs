// spec: lifecycle-kit/SPEC.md §check-skill-binding — every binding-shim skill names an
// existing template and binds exactly that template's slot set
use crate::walk;
use std::path::Path;

const LEAD: &str = "Execute the template at ";
const TAIL: &str = ", applying the bindings below.";

// spec: lifecycle-kit/SPEC.md §check-skill-binding — the binding directive, anchored at both
// ends as the shell form's `sed -nE` is, and the first such line in the shim wins
pub fn template_of(text: &str) -> Option<&str> {
    text.lines().find_map(|l| {
        l.strip_prefix(LEAD)
            .and_then(|r| r.strip_suffix(TAIL))
            .filter(|p| !p.is_empty())
    })
}

fn sorted_unique(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

// spec: lifecycle-kit/SPEC.md §check-skill-binding — a template slot is `*<name:`, the
// gate-owned literal shape the shell form greps for
fn slots_of(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let b = line.as_bytes();
        let mut from = 0usize;
        while let Some(rel) = line[from..].find("*<") {
            let at = from + rel;
            let mut i = at + 2;
            if i < b.len() && b[i].is_ascii_lowercase() {
                let s = i;
                while i < b.len() && (b[i].is_ascii_lowercase() || b[i].is_ascii_digit() || b[i] == b'-') {
                    i += 1;
                }
                if i < b.len() && b[i] == b':' {
                    out.push(line[s..i].to_string());
                }
            }
            from = at + 2;
        }
    }
    sorted_unique(out)
}

// spec: lifecycle-kit/SPEC.md §check-skill-binding — a binding is `**name** — …` below the
// shim's `## Bindings` heading, the shape the shell form's awk-then-sed pair selects
fn bindings_of(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inb = false;
    for line in text.lines() {
        if line.starts_with("## Bindings") {
            inb = true;
            continue;
        }
        if !inb {
            continue;
        }
        let Some(rest) = line.strip_prefix("**") else {
            continue;
        };
        let b = rest.as_bytes();
        if b.is_empty() || !b[0].is_ascii_lowercase() {
            continue;
        }
        let mut i = 0usize;
        while i < b.len() && (b[i].is_ascii_lowercase() || b[i].is_ascii_digit() || b[i] == b'-') {
            i += 1;
        }
        if rest[i..].starts_with("** —") {
            out.push(rest[..i].to_string());
        }
    }
    sorted_unique(out)
}

pub fn run(args: &[String]) -> i32 {
    let dir = match args.first().filter(|a| !a.is_empty()) {
        Some(d) => d.clone(),
        None => match walk::knob_scalar("LIFECYCLE_KIT_SKILLS_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-skill-binding: {}", e);
                return 2;
            }
        },
    };
    if !Path::new(&dir).is_dir() {
        eprintln!("check-skill-binding: skills dir not found: {}", dir);
        return 2;
    }

    let files = match walk::glob_files(Path::new(&dir), &["*.md".to_string()]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-skill-binding: {}", e);
            return 2;
        }
    };

    let mut findings: Vec<String> = Vec::new();
    let mut shims = 0usize;
    for f in &files {
        let text = match std::fs::read(f) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => {
                eprintln!("check-skill-binding: cannot read {}: {}", f.display(), e);
                return 2;
            }
        };
        let base = f.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        let Some(tmpl) = template_of(&text) else {
            continue;
        };
        shims += 1;
        if !Path::new(tmpl).is_file() {
            findings.push(format!(
                "{}: binding directive names template '{}' — no such file",
                base, tmpl
            ));
            continue;
        }
        let ttext = match std::fs::read(tmpl) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => {
                eprintln!("check-skill-binding: cannot read {}: {}", tmpl, e);
                return 2;
            }
        };
        let slots = slots_of(&ttext);
        let binds = bindings_of(&text);
        for s in &slots {
            if !binds.contains(s) {
                findings.push(format!("{}: template slot '{}' has no binding", base, s));
            }
        }
        for b in &binds {
            if !slots.contains(b) {
                findings.push(format!("{}: binding '{}' names no slot in {}", base, b, tmpl));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-skill-binding: a binding shim must name an existing template and bind");
        println!("exactly that template's slot set — an unbound slot or an orphan binding is drift:");
        for m in &findings {
            println!("  {}", m);
        }
        println!("  help: fix the binding directive's template path, or align the shim's ## Bindings");
        println!("        entries with the template's *<slot-name: ...>* set (one entry per slot).");
        return 1;
    }

    println!(
        "SKILL-BINDING: clean ({} binding-shim(s); each names an existing template and binds its exact slot set)",
        shims
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_binding_directive_is_anchored_at_both_ends() {
        assert_eq!(
            template_of("Execute the template at t/x.md, applying the bindings below.\n"),
            Some("t/x.md")
        );
        assert_eq!(template_of("see: Execute the template at t/x.md, applying the bindings below.\n"), None);
        assert_eq!(template_of("Execute the template at t/x.md, applying the bindings below\n"), None);
    }

    #[test]
    fn slots_and_bindings_are_sorted_unique_and_shape_bound() {
        assert_eq!(
            slots_of("*<ritual: a> and *<consistency-gate: b> and *<ritual: c>\n*<Bad: x> *<9: y>\n"),
            vec!["consistency-gate".to_string(), "ritual".to_string()]
        );
        assert_eq!(
            bindings_of("prose\n## Bindings\n\n**ritual** — a\n**b1** — c\n**nope** - d\nplain\n"),
            vec!["b1".to_string(), "ritual".to_string()]
        );
        assert!(bindings_of("**ritual** — above the heading\n").is_empty());
    }
}
