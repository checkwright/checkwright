// spec: canon-kit/SPEC.md §check-spec-fence-balance — every governed markdown file has an
// even fence-delimiter count, so the fence-skipping parsers never desync and fail open
use crate::spec;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-spec-fence-balance: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let files: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        let mut v: Vec<String> = spec::manifest_files(".")?
            .into_iter()
            .map(|p| p.display().to_string())
            .collect();
        let queue = std::env::var("GATE_SDK_KNOB_CANON_KIT_QUEUE_FILE").map_err(|_| {
            "GATE_SDK_KNOB_CANON_KIT_QUEUE_FILE is unset — the gate was invoked without the \
             config bridge gate_command emits"
                .to_string()
        })?;
        if Path::new(&queue).is_file() {
            v.push(queue);
        }
        v
    };

    let mut bad: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    for f in &files {
        if !Path::new(f).is_file() {
            continue;
        }
        scanned += 1;
        let text = spec::read_text(Path::new(f))?;
        let n = text.lines().filter(|l| is_fence(l)).count();
        if n % 2 != 0 {
            bad.push(format!("{} ({} fence delimiters — odd)", f, n));
        }
    }

    if !bad.is_empty() {
        println!("check-spec-fence-balance: markdown file(s) with an odd fence-delimiter count —");
        println!("the fence-skipping parsers (embedded-source, tag-lead-line, the queue scanners)");
        println!("toggle a fence flag; an odd count desyncs it and the rest of the file fails open:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: close the unbalanced code fence, or delete the stray delimiter line.");
        return Ok(1);
    }

    println!(
        "SPEC-FENCE-BALANCE: clean ({} governed markdown file(s), all even fence counts)",
        scanned
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-spec-fence-balance — the fence delimiter counted, on POSIX
// space byte-wise as grep matches it
pub fn is_fence(line: &str) -> bool {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\x0b' | b'\x0c' | b'\r') {
        i += 1;
    }
    b[i..].starts_with(b"```")
}
