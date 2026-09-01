// spec: context-kit/SPEC.md §Index-first reading — the Rust public-item extractor, grep-grade: the
// eight declared kinds under `pub` or `pub(...)`. Re-exports and multi-line declarations are stated
// honest limits, not parsed — the port moved the wrapper, never the rule.
use super::{compile, strip, take};
use crate::section;

const PUB: &str = r"^pub(\([^)]*\))?[[:blank:]]+";
const ASYNC: &str = r"^async[[:blank:]]+";
const KIND: &str = r"^(fn|struct|enum|trait|type|const|static|mod)[[:blank:]]+";
const NAME: &str = r"^[A-Za-z_][A-Za-z0-9_]*";

pub fn extract(text: &str) -> Result<Vec<String>, String> {
    let (visibility, asynchronous) = (compile(PUB)?, compile(ASYNC)?);
    let (kind, name) = (compile(KIND)?, compile(NAME)?);
    let mut out: Vec<String> = Vec::new();
    for (i, line) in section::split_lines(text).iter().enumerate() {
        let rest = line.trim_start_matches([' ', '\t']);
        // spec: context-kit/SPEC.md §Index-first reading — the visibility gate before the strip,
        // so `pubfn` and a bare `pub` are rejected on the same reading the shell form used
        if !(rest.starts_with("pub ") || rest.starts_with("pub\t") || rest.starts_with("pub(")) {
            continue;
        }
        let Some((_, rest)) = take(&visibility, rest) else {
            continue;
        };
        let rest = strip(&asynchronous, rest);
        let Some((declared, rest)) = take(&kind, rest) else {
            continue;
        };
        let Some((item, _)) = take(&name, rest) else {
            continue;
        };
        out.push(format!(
            "{} {} {}",
            declared.trim_end_matches([' ', '\t']),
            item,
            i + 1
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §Index-first reading — every declared kind, the restricted
    // visibility form, and the `async fn` the grammar sees through; a private item is not surface
    #[test]
    fn every_declared_kind_is_taken_and_a_private_item_is_not() {
        let src = "pub mod things;\npub struct Alpha;\npub enum Beta {}\npub trait Gamma {}\n\
                   pub fn delta() {}\npub(crate) fn theta() {}\npub const EPSILON: u32 = 3;\n\
                   pub static ZETA: u32 = 4;\npub type Eta = u32;\npub async fn iota() {}\n\
                   fn private_not_shown() {}\n";
        let got = extract(src).expect("the Rust grammar failed to compile");
        assert_eq!(
            got,
            vec![
                "mod things 1",
                "struct Alpha 2",
                "enum Beta 3",
                "trait Gamma 4",
                "fn delta 5",
                "fn theta 6",
                "const EPSILON 7",
                "static ZETA 8",
                "type Eta 9",
                "fn iota 10",
            ]
        );
    }

    // spec: context-kit/SPEC.md §Index-first reading — the near misses the grep-grade rule refuses:
    // a `pub` glued to its keyword, a restricted form with no separating blank, an undeclared kind,
    // and a name that does not open with an identifier character
    #[test]
    fn a_near_miss_is_refused_rather_than_half_read() {
        for src in [
            "pubfn alpha() {}\n",
            "pub(crate)fn alpha() {}\n",
            "pub impl Alpha {}\n",
            "pub fn 9alpha() {}\n",
            "pub\n",
        ] {
            assert!(
                extract(src).expect("the Rust grammar failed to compile").is_empty(),
                "{:?} was read as a public item",
                src
            );
        }
    }
}
