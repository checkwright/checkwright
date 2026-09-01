// spec: context-kit/SPEC.md §Index-first reading — the TypeScript public-surface extractor, whose
// grep-grade grammar and its two stated honest limits that section owns.
use super::{compile, strip, take};
use crate::section;

const EXPORT: &str = r"^[[:blank:]]*export[[:blank:]]+";
const DEFAULT: &str = r"^default([[:blank:]]|$)";
const DEFAULT_HEAD: &str = r"^default[[:blank:]]*";
const CALLABLE: &str = r"^(function|class)[[:blank:]]+";
const ASYNC: &str = r"^async[[:blank:]]+";
const KIND: &str = r"^(function|class|interface|type|enum|const|let|var)[[:blank:]]";
const ENUM: &str = r"^enum[[:blank:]]+";
const NAME: &str = r"^[A-Za-z_$][A-Za-z0-9_$]*";

pub fn extract(text: &str) -> Result<Vec<String>, String> {
    let (export, default, default_head) = (compile(EXPORT)?, compile(DEFAULT)?, compile(DEFAULT_HEAD)?);
    let (callable, asynchronous) = (compile(CALLABLE)?, compile(ASYNC)?);
    let (kind, enumeration, name) = (compile(KIND)?, compile(ENUM)?, compile(NAME)?);
    let mut out: Vec<String> = Vec::new();
    for (i, line) in section::split_lines(text).iter().enumerate() {
        let Some((_, rest)) = take(&export, line) else {
            continue;
        };
        // spec: context-kit/SPEC.md §Index-first reading — `export default` is its own kind, and an
        // anonymous one keeps the literal `default` as its name rather than being dropped
        if take(&default, rest).is_some() {
            let rest = strip(&default_head, rest);
            let rest = strip(&asynchronous, rest);
            let rest = strip(&callable, rest);
            let item = take(&name, rest).map(|(n, _)| n).unwrap_or("default");
            out.push(format!("default {} {}", item, i + 1));
            continue;
        }
        let rest = strip(&asynchronous, rest);
        let Some((declared, rest)) = take(&kind, rest) else {
            continue;
        };
        // spec: context-kit/SPEC.md §Index-first reading — the kind pattern consumes exactly one
        // blank, so the tail is re-trimmed before the name is read
        let mut declared = &declared[..declared.len() - 1];
        let mut rest = rest.trim_start_matches([' ', '\t']);
        if declared == "const" {
            if let Some((_, tail)) = take(&enumeration, rest) {
                declared = "enum";
                rest = tail;
            }
        }
        let Some((item, _)) = take(&name, rest) else {
            continue;
        };
        out.push(format!("{} {} {}", declared, item, i + 1));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §Index-first reading — every kind the grammar claims, `const enum`
    // folded to `enum`, `export default` named, and the two re-export forms it skips by design
    #[test]
    fn every_claimed_kind_is_taken_and_a_re_export_is_skipped() {
        let src = "export function alpha() {}\nexport async function beta(): Promise<void> {}\n\
                   export class Gamma<T> {}\nexport interface Delta {}\n\
                   export type Epsilon = number;\nexport enum Zeta {}\nexport const enum Eta {}\n\
                   export const theta = 1;\nexport let iota = 2;\nexport var kappa = 3;\n\
                   export default function main() {}\n\
                   export { alpha as renamedAlpha } from \"./other\";\nexport * from \"./more\";\n\
                   const notExported = 4;\n";
        let got = extract(src).expect("the TypeScript grammar failed to compile");
        assert_eq!(
            got,
            vec![
                "function alpha 1",
                "function beta 2",
                "class Gamma 3",
                "interface Delta 4",
                "type Epsilon 5",
                "enum Zeta 6",
                "enum Eta 7",
                "const theta 8",
                "let iota 9",
                "var kappa 10",
                "default main 11",
            ]
        );
    }

    // spec: context-kit/SPEC.md §Index-first reading — an anonymous default keeps the literal
    // `default` as its name, which is the one row whose name is not read out of the source
    #[test]
    fn an_anonymous_default_is_named_default() {
        for src in [
            "export default {\n",
            "export default function () {}\n",
            "export default\n",
        ] {
            assert_eq!(
                extract(src).expect("the TypeScript grammar failed to compile"),
                vec!["default default 1"],
                "{:?} did not fall back to the literal name",
                src
            );
        }
    }
}
