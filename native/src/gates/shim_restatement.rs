// spec: lifecycle-kit/SPEC.md §check-shim-restatement — no binding shim shares an >=N-word
// normalized n-gram with the dedup corpus (the agent file + every kit's templates)
use crate::walk;
use std::collections::HashMap;
use std::path::Path;

const LEAD: &str = "Execute the template at ";
const TAIL: &str = ", applying the bindings below.";

fn read(p: &Path) -> Result<String, String> {
    std::fs::read(p)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", p.display(), e))
}

// spec: lifecycle-kit/SPEC.md §check-shim-restatement — normalize, then emit every N-word
// window: an ASCII alphanumeric run is a word, everything else is a separator, and the word
// stream runs across line breaks as one document
fn ngrams(text: &str, n: usize) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut cur = String::new();
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            cur.push(c.to_ascii_lowercase());
        } else if !cur.is_empty() {
            words.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    if n == 0 || words.len() < n {
        return Vec::new();
    }
    (0..=words.len() - n)
        .map(|i| words[i..i + n].join(" "))
        .collect()
}

fn sorted_unique(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

fn is_shim(text: &str) -> bool {
    text.lines()
        .any(|l| l.starts_with(LEAD) && l.ends_with(TAIL) && l.len() > LEAD.len() + TAIL.len())
}

pub fn run(args: &[String]) -> i32 {
    let dir = match args.first().filter(|a| !a.is_empty()) {
        Some(d) => d.clone(),
        None => match walk::knob_scalar("LIFECYCLE_KIT_SKILLS_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-shim-restatement: {}", e);
                return 2;
            }
        },
    };
    if !Path::new(&dir).is_dir() {
        eprintln!("check-shim-restatement: skills dir not found: {}", dir);
        return 2;
    }
    let rest: &[String] = if args.is_empty() { &[] } else { &args[1..] };

    let n: usize = match walk::knob_scalar("LIFECYCLE_KIT_SHIM_NGRAM") {
        Ok(v) => match v.parse() {
            Ok(k) => k,
            Err(_) => {
                eprintln!(
                    "check-shim-restatement: LIFECYCLE_KIT_SHIM_NGRAM '{}' is not a positive integer",
                    v
                );
                return 2;
            }
        },
        Err(e) => {
            eprintln!("check-shim-restatement: {}", e);
            return 2;
        }
    };

    // spec: lifecycle-kit/SPEC.md §check-shim-restatement — corpus resolution order
    let corpus: Vec<String> = if !rest.is_empty() {
        rest.to_vec()
    } else {
        let declared = match walk::knob_array("LIFECYCLE_KIT_SHIM_DEDUP_CORPUS") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-shim-restatement: {}", e);
                return 2;
            }
        };
        if !declared.is_empty() {
            declared
        } else {
            let agent = match walk::knob_scalar("LIFECYCLE_KIT_AGENT_FILE") {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-shim-restatement: {}", e);
                    return 2;
                }
            };
            let roots = match walk::kit_roots_rel() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-shim-restatement: {}", e);
                    return 2;
                }
            };
            let mut c: Vec<String> = Vec::new();
            if Path::new(&agent).is_file() {
                c.push(agent);
            }
            for root in &roots {
                let tdir = format!("{}/templates", root.trim_end_matches('/'));
                if !Path::new(&tdir).is_dir() {
                    continue;
                }
                match walk::find_files(Path::new(&tdir), &["md"]) {
                    Ok(v) => c.extend(v.into_iter().map(|p| p.display().to_string())),
                    Err(e) => {
                        eprintln!("check-shim-restatement: {}", e);
                        return 2;
                    }
                }
            }
            c
        }
    };

    // spec: lifecycle-kit/SPEC.md §check-shim-restatement — the index maps an n-gram to the
    // first corpus file carrying it, which is what the finding names
    let mut index: HashMap<String, String> = HashMap::new();
    for cf in &corpus {
        let p = Path::new(cf);
        if !p.is_file() {
            continue;
        }
        let text = match read(p) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("check-shim-restatement: {}", e);
                return 2;
            }
        };
        for g in sorted_unique(ngrams(&text, n)) {
            index.entry(g).or_insert_with(|| cf.clone());
        }
    }
    if index.is_empty() {
        eprintln!(
            "check-shim-restatement: dedup corpus produced no {}-word n-grams (corpus files missing or shorter than N)",
            n
        );
        return 2;
    }

    let files = match walk::glob_files(Path::new(&dir), &["*.md".to_string()]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-shim-restatement: {}", e);
            return 2;
        }
    };
    let mut findings: Vec<String> = Vec::new();
    let mut shims = 0usize;
    for f in &files {
        let text = match read(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("check-shim-restatement: {}", e);
                return 2;
            }
        };
        if !is_shim(&text) {
            continue;
        }
        shims += 1;
        let base = f.file_name().and_then(|x| x.to_str()).unwrap_or_default();
        for g in sorted_unique(ngrams(&text, n)) {
            if let Some(cf) = index.get(&g) {
                findings.push(format!(
                    "{} shares a {}-word phrase with {}: \"{}\"",
                    base, n, cf, g
                ));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-shim-restatement: a binding shim restates the dedup corpus — bind consumer");
        println!("residue and cite kit-owned procedure, never restate it:");
        for m in &findings {
            println!("  {}", m);
        }
        println!("  help: delete the restated span from the shim and replace it with a citation");
        println!("        (a path plus a §heading) to the corpus surface that owns it. The n-gram");
        println!("        holds the copy shape only; a paraphrase below {} words passes this gate", n);
        println!("        and is still the same defect to fix on sight.");
        return 1;
    }
    println!(
        "SHIM-RESTATEMENT: clean ({} binding-shim(s); no {}-word phrase shared with the dedup corpus)",
        shims, n
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §check-shim-restatement — the normalization is what makes
    // the n-gram a copy detector rather than a string compare: case, punctuation and markup
    // fall out, and the word stream crosses line breaks
    #[test]
    fn normalization_folds_case_and_markup_and_runs_across_lines() {
        assert_eq!(
            ngrams("The **Queue**, and\nits Rule.", 3),
            vec![
                "the queue and".to_string(),
                "queue and its".to_string(),
                "and its rule".to_string()
            ]
        );
        assert!(ngrams("only two", 3).is_empty());
        assert!(ngrams("a b c", 0).is_empty());
    }

    #[test]
    fn a_shim_is_a_file_carrying_the_anchored_binding_directive() {
        assert!(is_shim("prose\nExecute the template at t.md, applying the bindings below.\n"));
        assert!(!is_shim("Execute the template at , applying the bindings below.\n"));
        assert!(!is_shim("x Execute the template at t.md, applying the bindings below.\n"));
    }
}
