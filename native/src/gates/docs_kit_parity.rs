// spec: gate-sdk/SPEC.md §check-kit-registration — docs/kits.md carries a registry row for
// every kit root (this consumer re-scopes that invariant onto the Kit Reference page; wrapper,
// not mechanism) and every docs kit page carries the nav child block
use crate::fresh;
use crate::gates::kit_registration;
use crate::walk;
use std::path::Path;

const DEFAULT_REGISTRY: &str = "docs/kits.md";
// spec: gate-sdk/SPEC.md §check-kit-registration — POSIX `[[:space:]]` minus the newline the
// record separator already consumed, so a CRLF front matter reads as the shell form read it
const SPACE: [char; 5] = [' ', '\t', '\r', '\x0b', '\x0c'];

fn trim(s: &str) -> &str {
    s.trim_start_matches(SPACE).trim_end_matches(SPACE)
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-docs-kit-parity: {}", e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-kit-registration — the nav child block: front matter nesting the
// page under the Kit Reference parent, `nav_parent: kits` plus a `nav_child_order` slot
fn has_nav_block(text: &str) -> bool {
    let mut fm = false;
    let (mut parent, mut order) = (false, false);
    for (n, line) in text.lines().enumerate() {
        if n == 0 {
            if line != "---" {
                return false;
            }
            fm = true;
            continue;
        }
        if fm && line == "---" {
            fm = false;
            continue;
        }
        if !fm {
            continue;
        }
        if let Some(rest) = line.strip_prefix("nav_parent:") {
            if trim(rest) == "kits" {
                parent = true;
            }
        }
        if let Some(rest) = line.strip_prefix("nav_child_order:") {
            if rest
                .trim_start_matches(SPACE)
                .starts_with(|c: char| c.is_ascii_digit())
            {
                order = true;
            }
        }
    }
    parent && order
}

// spec: gate-sdk/SPEC.md §check-kit-registration — `dirname`, on the shell's own terms: the
// path up to the last '/', '.' when there is none
fn dirname(p: &str) -> String {
    match p.rfind('/') {
        None => ".".to_string(),
        Some(0) => "/".to_string(),
        Some(i) => p[..i].to_string(),
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let reg = fresh::positional(args, 0, DEFAULT_REGISTRY);

    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the wrapped rule is called, never
    // spawned; the shell form's combined capture is the two sinks in order, which never
    // interleave: a refusal arm writes only stderr and the report arm only stdout.
    let mut out: Vec<String> = Vec::new();
    let mut err: Vec<String> = Vec::new();
    let rc = kit_registration::run_captured(&[reg.to_string()], &mut out, &mut err);
    let combined = out
        .into_iter()
        .chain(err)
        .collect::<Vec<String>>()
        .join("\n");

    if rc == 2 {
        eprintln!("{}", combined);
        return Ok(2);
    }
    if rc != 0 {
        println!(
            "check-docs-kit-parity: a kit root is missing its row in the docs index ({}):",
            reg
        );
        println!("{}", combined);
        println!(
            "  help: add the kit's '[<kit>](<kit>/index.md)' row to {} (docs/<kit>/ is",
            reg
        );
        println!(
            "        the kit's docs page dir), so a landed kit cannot fall out of the docs site."
        );
        return Ok(1);
    }

    let base = dirname(reg);
    let mut navbad: Vec<String> = Vec::new();
    for idx in walk::glob_files(Path::new(&base), &["*/index.md".to_string()])? {
        let p = idx.display().to_string();
        let text = fresh::read_captured(&p)?;
        if !has_nav_block(&text) {
            navbad.push(p);
        }
    }
    if !navbad.is_empty() {
        println!("check-docs-kit-parity: a docs kit page lacks the nav child block — it would render but fall out of the sidebar nav:");
        for p in &navbad {
            println!("  {}", p);
        }
        println!("  help: give each docs/<kit>/index.md a front-matter block with 'nav_parent: kits'");
        println!("        (nesting it under the Kit Reference page) and 'nav_child_order: <n>' (its slot).");
        return Ok(1);
    }

    println!(
        "DOCS-KIT-PARITY: clean ({} registers every kit root; every docs kit page carries the nav child block)",
        reg
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-kit-registration — both keys, inside the first front-matter
    // block, or the page holds no nav slot
    #[test]
    fn the_nav_block_needs_both_keys_inside_the_front_matter() {
        assert!(has_nav_block("---\nnav_parent: kits\nnav_child_order: 3\n---\n"));
        assert!(!has_nav_block("---\nnav_parent: kits\n---\n"));
        assert!(!has_nav_block("---\nnav_child_order: 3\n---\n"));
        assert!(!has_nav_block("# no front matter\nnav_parent: kits\nnav_child_order: 3\n"));
        assert!(!has_nav_block(
            "---\n---\nnav_parent: kits\nnav_child_order: 3\n"
        ));
        assert!(!has_nav_block("---\nnav_parent: kitsch\nnav_child_order: 3\n---\n"));
        assert!(!has_nav_block("---\nnav_parent: kits\nnav_child_order: x\n---\n"));
    }

    #[test]
    fn dirname_follows_the_shells_own_rule() {
        assert_eq!(dirname("docs/kits.md"), "docs");
        assert_eq!(dirname("kits.md"), ".");
        assert_eq!(dirname("/a/b"), "/a");
        assert_eq!(dirname("/b"), "/");
    }
}
