// spec: context-kit/SPEC.md §Index-first reading — the *bundled* members of the public-surface
// extractor registry, in-crate behind the dispatcher rather than arms of their own.
pub mod rust;
pub mod ts;

use crate::ere::Ere;

// spec: context-kit/SPEC.md §Index-first reading — an extractor is the same two names a consumer's
// sourced file defines: the find globs, and the rule producing unsorted `kind name lineno` rows.
pub struct Builtin {
    pub lang: &'static str,
    pub globs: &'static [&'static str],
    pub extract: fn(&str) -> Result<Vec<String>, String>,
}

// spec: context-kit/SPEC.md §Index-first reading — the roster the empty `CONTEXT_KIT_PUB_LANGS`
// derives to, in the order the deleted `find … | sort` over `lib/pub-lang/` produced: the default
// stays derived from the shipped set rather than maintained as a list.
pub const BUILTIN: &[Builtin] = &[
    Builtin {
        lang: "rust",
        globs: &["*.rs"],
        extract: rust::extract,
    },
    Builtin {
        lang: "ts",
        globs: &["*.ts", "*.tsx"],
        extract: ts::extract,
    },
];

pub fn lookup(lang: &str) -> Option<&'static Builtin> {
    BUILTIN.iter().find(|b| b.lang == lang)
}

pub fn langs() -> Vec<&'static str> {
    BUILTIN.iter().map(|b| b.lang).collect()
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the extractors' grammars are re-expressed against
// it rather than hand-scanned, so `grep -nE` piped into `awk` becomes one engine's leftmost-longest
// spans and the verdict is identical either side of the substitution
pub fn compile(pattern: &str) -> Result<Ere, String> {
    Ere::compile(pattern).map_err(|e| format!("cannot compile /{}/: {}", pattern, e))
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — awk's anchored `match` + `substr` pair: an
// anchored pattern's span and the tail beyond it, or nothing where the head does not match.
pub fn take<'a>(re: &Ere, s: &'a str) -> Option<(&'a str, &'a str)> {
    match re.find(s) {
        Some((0, end)) => Some((&s[..end], &s[end..])),
        _ => None,
    }
}

// spec: context-kit/SPEC.md §Index-first reading — awk's `sub(/^…/, "")`: the tail where the head
// matched, the subject unchanged where it did not
pub fn strip<'a>(re: &Ere, s: &'a str) -> &'a str {
    take(re, s).map(|(_, rest)| rest).unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §Index-first reading — the roster is the empty knob's derivation,
    // so its order is the sorted order the deleted directory listing produced; a member added out
    // of order would silently reorder every default run's language blocks
    #[test]
    fn the_builtin_roster_is_sorted_by_language_and_every_member_declares_a_glob() {
        let names = langs();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        assert_eq!(names, sorted, "the built-in roster is not in sorted order");
        assert!(BUILTIN.iter().all(|b| !b.globs.is_empty()));
        assert!(lookup("rust").is_some());
        assert!(lookup("nosuchlang").is_none());
    }
}
