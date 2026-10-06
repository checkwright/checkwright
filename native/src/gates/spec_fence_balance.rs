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
        let queue = crate::walk::knob_scalar("CANON_KIT_QUEUE_FILE")?;
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
        if let Some(ln) = unclosed(&text) {
            bad.push(format!("{}:{} (the fence opened here is never closed)", f, ln));
        }
    }

    if !bad.is_empty() {
        println!("check-spec-fence-balance: markdown file(s) ending inside an open code fence —");
        println!("the fence-skipping parsers (embedded-source, tag-lead-line, the queue scanners)");
        println!("step one fence reader; an unclosed fence reads the rest of the file as its body,");
        println!("so every later finding fails open:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: close the fence with a backtick run at least as long as its opener's, or delete the stray opener.");
        return Ok(1);
    }

    println!(
        "SPEC-FENCE-BALANCE: clean ({} governed markdown file(s), every fence closed)",
        scanned
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-spec-fence-balance — the line of the opener the shared fence
// reader leaves open at the file's end
fn unclosed(text: &str) -> Option<usize> {
    let mut fence = spec::Fence::default();
    let mut opened = 0usize;
    for (idx, line) in text.lines().enumerate() {
        if fence.delimits(line) && fence.is_open() {
            opened = idx + 1;
        }
    }
    fence.is_open().then_some(opened)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_list_item_fence_and_a_longer_fence_are_balanced_and_a_stray_opener_is_not() {
        assert_eq!(unclosed("- ```sh\n  a\n  ```\ntext\n"), None);
        assert_eq!(unclosed("````\n```\n````\n"), None);
        assert_eq!(unclosed("```\na\n```\n\n- ```sh\n  a\n"), Some(5));
        assert_eq!(unclosed("````\n```\n"), Some(1));
    }
}
