// spec: guard-kit/SPEC.md §The generic ruleset — the text operations the reader and the rules share,
// each reproducing the shell construct the ruleset was written in; none of them is a view.
use crate::ere::Ere;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — every pattern is a crate constant, compiled once
// per process; a constant the engine refuses is a build defect the unit tests catch.
pub fn re(src: &'static str) -> &'static Ere {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, &'static Ere>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut held = cache.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(e) = held.get(src) {
        return e;
    }
    let compiled = match Ere::compile(src) {
        Ok(e) => e,
        Err(e) => panic!("guard pattern {:?} does not compile: {}", src, e),
    };
    let leaked: &'static Ere = Box::leak(Box::new(compiled));
    held.insert(src, leaked);
    leaked
}

// spec: guard-kit/SPEC.md §The generic ruleset — `grep -qE <re> <<<"$text"`: a match on any line.
pub fn grep_q(src: &'static str, text: &str) -> bool {
    let e = re(src);
    text.split('\n').any(|l| e.is_match(l))
}

// spec: guard-kit/SPEC.md §The generic ruleset — `grep -oE`: every non-empty leftmost-longest match,
// line by line and non-overlapping, `^` holding only at a line's start.
pub fn grep_o(src: &'static str, text: &str) -> Vec<String> {
    let e = re(src);
    let mut out = Vec::new();
    for line in text.split('\n') {
        let mut pos = 0usize;
        while pos <= line.len() {
            match e.find_from(line, pos) {
                Some((s, t)) if t > s => {
                    out.push(String::from_utf8_lossy(&line.as_bytes()[s..t]).into_owned());
                    pos = t;
                }
                Some((s, _)) => pos = s + 1,
                None => break,
            }
        }
    }
    out
}

// spec: guard-kit/SPEC.md §The generic ruleset — `[[ $s =~ <re> ]]`: the whole subject, newlines
// included.
pub fn matches(src: &'static str, text: &str) -> bool {
    re(src).is_match(text)
}

pub fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

pub fn trim_start(s: &str) -> &str {
    let i = s.bytes().position(|c| !is_space(c)).unwrap_or(s.len());
    &s[i..]
}

pub fn trim_end(s: &str) -> &str {
    let i = s.bytes().rposition(|c| !is_space(c)).map_or(0, |i| i + 1);
    &s[..i]
}

pub fn trim(s: &str) -> &str {
    trim_end(trim_start(s))
}

// spec: guard-kit/SPEC.md §The generic ruleset — `$( … )` drops every trailing newline of what it
// captures, so a view taken that way never ends in one.
pub fn chomp(s: &str) -> &str {
    s.trim_end_matches('\n')
}

// spec: guard-kit/SPEC.md §The generic ruleset — `read -ra words <<<"$s"`: the first line only, split
// on blanks.
pub fn words(s: &str) -> Vec<&str> {
    let line = s.split('\n').next().unwrap_or("");
    line.split([' ', '\t']).filter(|w| !w.is_empty()).collect()
}

// spec: guard-kit/SPEC.md §The generic ruleset — an unquoted `for w in $s`: every line split on
// blanks, glob expansion aside.
pub fn all_words(s: &str) -> Vec<&str> {
    s.split([' ', '\t', '\n']).filter(|w| !w.is_empty()).collect()
}

// spec: guard-kit/SPEC.md §The generic ruleset — `${s%%[[:space:]]*}`, the head word.
pub fn head_word(s: &str) -> &str {
    let i = s.bytes().position(is_space).unwrap_or(s.len());
    &s[..i]
}

// spec: guard-kit/SPEC.md §The generic ruleset — the dequoted view's sentinels back to the bytes
// they hold.
pub fn unsentinel(s: &str) -> String {
    s.replace('\x01', " ")
        .replace('\x02', "\t")
        .replace('\x03', ";")
        .replace('\x04', "|")
        .replace('\x05', "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grep_o_is_line_wise_and_anchors_at_each_line_start() {
        assert_eq!(grep_o("^a+", "aa b aa\naaa"), vec!["aa", "aaa"]);
        assert_eq!(grep_o("[0-9]+", "a1b22\n333"), vec!["1", "22", "333"]);
        assert!(grep_o("x*", "abc").is_empty());
    }

    #[test]
    fn the_word_splits_read_as_the_shell_reads_them() {
        assert_eq!(words("  a\tb  c\nd e"), vec!["a", "b", "c"]);
        assert_eq!(all_words("a b\nc"), vec!["a", "b", "c"]);
        assert_eq!(head_word("git\tlog"), "git");
        assert_eq!(chomp("a\n\n"), "a");
        assert_eq!(trim("  a b \n"), "a b");
    }
}
