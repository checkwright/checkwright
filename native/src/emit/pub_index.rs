// spec: context-kit/SPEC.md §Index-first reading — the public-surface index: a dispatcher over
// per-language extractors, whose seam that section rules survives the port intact.
use super::pub_lang;
use super::{corpus, read_text, relative, targets};
use std::path::Path;

pub const KNOBS: &[&str] = &[
    "CONTEXT_KIT_PRUNE_DIRS",
    "CONTEXT_KIT_PUB_LANGS",
    "CONTEXT_KIT_PUB_LANG_DIR",
];

// spec: context-kit/SPEC.md §Index-first reading — the consumer extractor's first spawn: source the
// file and print its declared globs, one per line. The globs are needed *before* the walk, which is
// why the seam costs two spawns per language rather than one.
const GLOBS_SCRIPT: &str =
    "source \"$1\"; printf '%s\\n' ${PUB_LANG_GLOBS[@]+\"${PUB_LANG_GLOBS[@]}\"}";

// spec: context-kit/SPEC.md §Index-first reading — the second of the two spawns per language.
// comment-tier-exempt: the US-delimited path line is the dispatcher's own per-file framing inside
// one spawn — below SPEC altitude, and named so it is not read as a third contract name.
const EXTRACT_SCRIPT: &str = "source \"$1\"; shift; \
     for f in \"$@\"; do printf '\\037%s\\n' \"$f\"; pub_lang_extract \"$f\" || true; done; exit 0";

const MARK: char = '\u{1f}';

enum Extractor {
    Consumer(String),
    Builtin(&'static pub_lang::Builtin),
}

// spec: context-kit/SPEC.md §Index-first reading — the consumer-first resolution order, with the
// built-in roster in the deleted `lib/pub-lang/` leg's place and that section's refusal beyond it.
fn resolve(lang: &str, dir: &str) -> Result<Extractor, String> {
    let path = format!("{}/{}.sh", dir, lang);
    if Path::new(&path).is_file() {
        return Ok(Extractor::Consumer(path));
    }
    match pub_lang::lookup(lang) {
        Some(b) => Ok(Extractor::Builtin(b)),
        None => Err(format!(
            "no extractor for language '{}': no {} and none built in (built in: {})",
            lang,
            path,
            pub_lang::langs().join(", ")
        )),
    }
}

fn shell(script: &str, args: &[&str]) -> Result<String, String> {
    let mut argv: Vec<&str> = vec!["-c", script, "bash"];
    argv.extend_from_slice(args);
    let done = crate::proc::run("bash", &argv)?;
    match done.stdout() {
        Some(o) => Ok(String::from_utf8_lossy(o).into_owned()),
        None => Err(format!(
            "the consumer extractor {} could not be sourced: {}",
            args.first().copied().unwrap_or("?"),
            done.failure_report().unwrap_or_default()
        )),
    }
}

fn globs(x: &Extractor) -> Result<Vec<String>, String> {
    match x {
        Extractor::Builtin(b) => Ok(b.globs.iter().map(|g| g.to_string()).collect()),
        Extractor::Consumer(path) => Ok(shell(GLOBS_SCRIPT, &[path])?
            .lines()
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect()),
    }
}

// spec: context-kit/SPEC.md §Index-first reading — one language's rows, keyed by the walked file:
// the built-in extractor runs in process and a consumer's runs through the second `bash` spawn,
// and both hand back the same unsorted `kind name lineno` rows.
fn rows_by_file(x: &Extractor, files: &[String]) -> Result<Vec<(String, Vec<String>)>, String> {
    match x {
        Extractor::Builtin(b) => {
            let mut out: Vec<(String, Vec<String>)> = Vec::new();
            for f in files {
                // spec: context-kit/SPEC.md §Index-first reading — an unreadable file contributes
                // no rows, the shell form's `pub_lang_extract … || true` per file
                let Ok(text) = read_text(f) else { continue };
                out.push((f.clone(), (b.extract)(&text)?));
            }
            Ok(out)
        }
        Extractor::Consumer(path) => {
            let mut argv: Vec<&str> = vec![path.as_str()];
            argv.extend(files.iter().map(String::as_str));
            let raw = shell(EXTRACT_SCRIPT, &argv)?;
            let mut out: Vec<(String, Vec<String>)> = Vec::new();
            for line in raw.lines() {
                match line.strip_prefix(MARK) {
                    Some(f) => out.push((f.to_string(), Vec::new())),
                    None => {
                        if let Some(last) = out.last_mut() {
                            if !line.is_empty() {
                                last.1.push(line.to_string());
                            }
                        }
                    }
                }
            }
            Ok(out)
        }
    }
}

// spec: context-kit/SPEC.md §Index-first reading — the kind-then-name sort: `LC_ALL=C sort -k1,1
// -k2,2`, so the compare is bytewise on the first field, then the second, then `sort`'s last-resort
// whole-line comparison.
fn field(row: &str, n: usize) -> &str {
    row.split_ascii_whitespace().nth(n).unwrap_or("")
}

fn sort_rows(rows: &mut [String]) {
    rows.sort_by(|a, b| {
        field(a, 0)
            .as_bytes()
            .cmp(field(b, 0).as_bytes())
            .then_with(|| field(a, 1).as_bytes().cmp(field(b, 1).as_bytes()))
            .then_with(|| a.as_bytes().cmp(b.as_bytes()))
    });
}

// spec: context-kit/SPEC.md §Index-first reading — the row shape, `  %-8s %s :%s` under a
// `<rel>  (<count>)` block header, which the index-tests goldens assert exactly.
fn block(rel: &str, rows: &[String]) -> String {
    let mut out = format!("{}  ({})\n", rel, rows.len());
    for r in rows {
        out.push_str(&format!(
            "  {:<8} {} :{}\n",
            field(r, 0),
            field(r, 1),
            field(r, 2)
        ));
    }
    out.push('\n');
    out
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let paths = targets(args)?;
    let root = crate::walk::toplevel_opt()?;
    let dir = crate::walk::knob_scalar("CONTEXT_KIT_PUB_LANG_DIR")?;
    // spec: context-kit/SPEC.md §lib/context.sh — an empty `CONTEXT_KIT_PUB_LANGS` means *derive
    // it*, not *no languages*: a repo-relative literal cannot express the shipped roster, so the
    // one reader of the knob expands it to the built-in set.
    let mut langs = crate::walk::knob_array("CONTEXT_KIT_PUB_LANGS")?;
    if langs.is_empty() {
        langs = pub_lang::langs().into_iter().map(String::from).collect();
    }
    let mut out = String::new();
    for lang in &langs {
        let extractor = resolve(lang, &dir)?;
        let patterns = globs(&extractor)?;
        let refs: Vec<&str> = patterns.iter().map(String::as_str).collect();
        let files = corpus(&paths, &refs)?;
        let mut lang_block = String::new();
        for (file, mut rows) in rows_by_file(&extractor, &files)? {
            if rows.is_empty() {
                continue;
            }
            sort_rows(&mut rows);
            lang_block.push_str(&block(&relative(&root, &file), &rows));
        }
        if !lang_block.is_empty() {
            out.push_str(lang_block.trim_end_matches('\n'));
            out.push('\n');
        }
    }
    if out.is_empty() {
        out.push_str(&format!(
            "No public items found in {}\n",
            paths.join(" ")
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §Index-first reading — the sort is kind then name, bytewise under
    // the C locale, so an uppercase name sorts before a lowercase one and equal keys fall back to
    // the whole row rather than to input order
    #[test]
    fn rows_sort_by_kind_then_name_bytewise() {
        let mut rows: Vec<String> = ["fn zeta 9", "const EPSILON 18", "fn Alpha 2", "fn Alpha 1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        sort_rows(&mut rows);
        assert_eq!(
            rows,
            vec!["const EPSILON 18", "fn Alpha 1", "fn Alpha 2", "fn zeta 9"]
        );
    }

    // spec: context-kit/SPEC.md §Index-first reading — the block shape the goldens pin: a header
    // carrying the row count, then one padded row per item, then the separating blank line
    #[test]
    fn a_block_carries_its_count_and_pads_the_kind_column_to_eight() {
        let rows: Vec<String> = ["fn delta 14", "interface Delta 7"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            block("corpus/sample.rs", &rows),
            "corpus/sample.rs  (2)\n  fn       delta :14\n  interface Delta :7\n\n"
        );
    }

    // spec: context-kit/SPEC.md §Index-first reading — resolution is consumer-first and the
    // built-in roster is the kit's deleted `lib/pub-lang/` leg, so an unknown language refuses
    // naming both the consumer path it looked for and what is built in
    #[test]
    fn resolution_falls_back_to_the_builtin_roster_and_refuses_an_unknown_language() {
        assert!(matches!(
            resolve("rust", "no/such/dir").expect("rust is not built in"),
            Extractor::Builtin(_)
        ));
        let Err(e) = resolve("cobol", "scripts/pub-lang") else {
            panic!("cobol resolved to an extractor")
        };
        assert!(e.contains("cobol") && e.contains("scripts/pub-lang/cobol.sh"), "{}", e);
    }
}
