// spec: queue-kit/SPEC.md §check-queue-slug-liveness — every slug-shaped bold-code token in a
// configured prose surface resolves against the queue's live slug set
use crate::queue;
use crate::walk;
use std::path::Path;

// spec: queue-kit/SPEC.md §check-queue-slug-liveness — ``**`slug`**``: the bold-code form that
// claims queue membership, scanned for every occurrence on the line
fn bold_code_tokens(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 6 < b.len() {
        if !(b[i] == b'*' && b[i + 1] == b'*' && b[i + 2] == b'`') {
            i += 1;
            continue;
        }
        let start = i + 3;
        if start >= b.len() || !(b[start].is_ascii_lowercase() || b[start].is_ascii_digit()) {
            i += 1;
            continue;
        }
        let mut j = start + 1;
        while j < b.len()
            && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-')
        {
            j += 1;
        }
        if j + 2 < b.len() && b[j] == b'`' && b[j + 1] == b'*' && b[j + 2] == b'*' {
            out.push(&line[start..j]);
            i = j + 3;
            continue;
        }
        i += 1;
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    let globs = match queue::knob_array("QUEUE_KIT_PROSE_SURFACE_GLOBS") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-slug-liveness: {}", e);
            return 2;
        }
    };
    let queue_knob = match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-slug-liveness: {}", e);
            return 2;
        }
    };
    let sec = match queue::Sections::active_and_deferred() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-queue-slug-liveness: {}", e);
            return 2;
        }
    };

    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    let root = Path::new(scanroot);
    if !root.is_dir() {
        eprintln!("check-queue-slug-liveness: not a directory: {}", scanroot);
        return 2;
    }

    let files = if globs.is_empty() {
        Vec::new()
    } else {
        match walk::glob_files(root, &globs) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "check-queue-slug-liveness: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        }
    };

    if files.is_empty() {
        println!("QUEUE-SLUG-LIVENESS: clean (no prose surface configured in QUEUE_KIT_PROSE_SURFACE_GLOBS — nothing to resolve)");
        return 0;
    }

    // spec: queue-kit/SPEC.md §check-queue-slug-liveness — the queue is looked for at the
    // configured path first and under the scan root second, so a case dir and a repo root
    // both resolve it
    let mut qpath = std::path::PathBuf::from(&queue_knob);
    if !qpath.is_file() {
        qpath = root.join(&queue_knob);
    }
    if !qpath.is_file() {
        eprintln!(
            "check-queue-slug-liveness: queue file not found: {}",
            queue_knob
        );
        return 2;
    }
    let qtext = match std::fs::read_to_string(&qpath) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "check-queue-slug-liveness: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                qpath.display(),
                e
            );
            return 2;
        }
    };
    let live = queue::live_slugs(&qtext, &sec);

    let mut findings: Vec<String> = Vec::new();
    for f in &files {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "check-queue-slug-liveness: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    f.display(),
                    e
                );
                return 2;
            }
        };
        for (i, line) in text.lines().enumerate() {
            for tok in bold_code_tokens(line) {
                if !live.iter().any(|s| s == tok) {
                    findings.push(format!("{}:{}:{}", f.display(), i + 1, tok));
                }
            }
        }
    }

    if !findings.is_empty() {
        println!("check-queue-slug-liveness: bold-code token claims queue membership but names no live task:");
        for x in &findings {
            println!("  {}", x);
        }
        println!("  help: a **`slug`** token claims the slug is a live queue task. If the task");
        println!("        landed, drop the bold-code form and cite its owning SPEC; otherwise fix");
        println!("        the slug or restore the task to the queue.");
        return 1;
    }

    println!(
        "QUEUE-SLUG-LIVENESS: clean ({} prose surface(s) scanned; every slug-shaped bold-code token resolves to a live task in {})",
        files.len(),
        qpath.display()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bold_code_token_is_lowercase_kebab_inside_backticks_and_bold() {
        assert_eq!(bold_code_tokens("see **`a-b`** here"), vec!["a-b"]);
        assert_eq!(bold_code_tokens("**`x`** and **`y`**"), vec!["x", "y"]);
        assert!(bold_code_tokens("**bare-bold**").is_empty());
        assert!(bold_code_tokens("`just-code`").is_empty());
        assert!(bold_code_tokens("**`Upper`**").is_empty());
    }
}
