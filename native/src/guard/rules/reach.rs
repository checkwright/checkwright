// spec: guard-kit/SPEC.md §The generic ruleset — the rules that judge what a command reaches: a
// history rewrite, a tracked deletion, a scratch body, a wrapped payload, the main checkout.
use super::{
    command_read, command_word, git_read, passed_read, git_walk_of, has_expansion, inert_target, is_banner, is_ro_segment,
    moves_repository, names_command, plain_word, program_bearing, ro_forms_clear, segment_core, shell_backgrounds, GitWalk,
};
use crate::guard::engine::{Cmd, Ctx, Decided, Fault, Shell, Verdict};
use crate::guard::host::under;
use crate::guard::powershell;
use crate::guard::reader::View::{SqDqHd, SqHdq};
use crate::guard::reader::Word;
use crate::guard::text::{self, grep_q, head_word, trim_start, unsentinel, words};
use crate::walk;

fn block(m: impl Into<String>) -> Decided {
    Ok(Some(Verdict::Block(m.into())))
}

pub fn git_rewrite(ctx: &Ctx) -> Decided {
    let carries = |ws: &[String], opts: &[&str]| {
        ws.iter().any(|w| {
            opts.iter().any(|o| w.strip_prefix(o).is_some_and(|r| !r.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')))
        })
    };
    let rewrites = git_read(ctx, ctx.cmd())?.iter().flatten().any(|words| {
        let ws = git_walk_of(words, GitWalk::Args).unwrap_or_default();
        match ws.first().map(String::as_str) {
            Some("commit") => carries(&ws[1..], &["-F", "--file", "--amend"]),
            Some("reset") => carries(&ws[1..], &["--soft"]),
            _ => false,
        }
    });
    if !rewrites || refused(ctx, ctx.cmd())? {
        return Ok(None);
    }
    Ok(Some(Verdict::Advise("re-verify volatile git state before this history rewrite (DOCTRINE.md: Re-verify volatile state before a git history rewrite): confirm HEAD with 'git log --oneline -3' before an amend or squash; after a 'git reset --soft' re-stage and verify staged content with 'git show :<path>' before committing (the soft reset keeps the old index snapshot); carry the message in the command ('-m', or '-F -' from a heredoc) — a scratch message file may be another session's, and a leftover lands the wrong message with exit 0; if you must use a file, write it in this same command and read the result back with 'git log -1 --format=%B'; and rewrite the message when amending so it states the combined change.".to_string())))
}

// spec: guard-kit/SPEC.md §The rule roster — rule `commit_only_paths`'s long options of `git commit`,
// each resolved by git's unambiguous-prefix rule; the first group takes a separate value.
const COMMIT_VALUED: &[&str] = &[
    "message",
    "file",
    "reuse-message",
    "reedit-message",
    "template",
    "author",
    "date",
    "fixup",
    "squash",
    "cleanup",
    "trailer",
    "pathspec-from-file",
];
const COMMIT_FLAGS: &[&str] = &[
    "all",
    "include",
    "only",
    "quiet",
    "verbose",
    "reset-author",
    "short",
    "branch",
    "porcelain",
    "long",
    "null",
    "signoff",
    "verify",
    "allow-empty",
    "allow-empty-message",
    "edit",
    "amend",
    "post-rewrite",
    "pathspec-file-nul",
    "untracked-files",
    "gpg-sign",
    "dry-run",
    "status",
    "patch",
    "interactive",
];

enum CommitLong {
    Valued(&'static str),
    Flag(&'static str),
}

fn commit_long(name: &str) -> Option<CommitLong> {
    let exact = COMMIT_VALUED.iter().map(|n| CommitLong::Valued(n)).chain(COMMIT_FLAGS.iter().map(|n| CommitLong::Flag(n)));
    let full = |o: &CommitLong| match o {
        CommitLong::Valued(n) | CommitLong::Flag(n) => *n,
    };
    let mut hits: Vec<CommitLong> = Vec::new();
    for o in exact {
        if full(&o) == name {
            return Some(o);
        }
        if full(&o).starts_with(name) {
            hits.push(o);
        }
    }
    (hits.len() == 1).then(|| hits.remove(0))
}

// spec: guard-kit/SPEC.md §The rule roster — rule `commit_only_paths`'s test on a commit's words: true
// when it takes the whole index, `None` where a word cannot be read without guessing.
fn commits_whole_index(args: &[String]) -> Option<bool> {
    let (mut paths, mut whole, mut skip, mut ends) = (false, false, false, false);
    let mut i = 0usize;
    while i < args.len() {
        let w = args[i].as_str();
        i += 1;
        if std::mem::take(&mut skip) {
            continue;
        }
        if ends {
            paths = true;
            continue;
        }
        if w == "--" {
            ends = true;
            continue;
        }
        if let Some(long) = w.strip_prefix("--") {
            let (name, attached) = match long.split_once('=') {
                Some((n, _)) => (n, true),
                None => (long, false),
            };
            if name.starts_with("no-") {
                continue;
            }
            match commit_long(name)? {
                CommitLong::Valued(n) => {
                    if n == "pathspec-from-file" {
                        paths = true;
                    }
                    skip = !attached;
                }
                CommitLong::Flag("all" | "include") => whole = true,
                CommitLong::Flag(_) => {}
            }
            continue;
        }
        if let Some(bundle) = w.strip_prefix('-').filter(|b| !b.is_empty()) {
            for (k, letter) in bundle.char_indices() {
                match letter {
                    'a' | 'i' => whole = true,
                    'o' | 'q' | 'v' | 's' | 'n' | 'e' | 'z' | 'p' => {}
                    'u' | 'S' => break,
                    'm' | 'F' | 'C' | 'c' | 't' => {
                        skip = k + 1 == bundle.len();
                        break;
                    }
                    _ => return None,
                }
            }
            continue;
        }
        paths = true;
    }
    Some(whole || !paths)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `commit_only_paths`: a `git add` and, after it, a
// commit of the whole index, in one call.
pub fn commit_only_paths(ctx: &Ctx) -> Decided {
    let c = ctx.cmd();
    let live = ctx.view(c, SqHdq)?;
    let declines = match ctx.shell() {
        Shell::Bash => has_expansion(&live),
        Shell::PowerShell => live.contains('$'),
    };
    if declines {
        return Ok(None);
    }
    let mut added = false;
    for words in git_read(ctx, c)?.iter().flatten() {
        let Some(ws) = git_walk_of(words, GitWalk::Args) else { continue };
        match ws.first().map(String::as_str) {
            Some("add") => added = true,
            Some("commit") if added && commits_whole_index(&ws[1..]) == Some(true) => {
                return block("don't stage with 'git add' and then commit the whole index in one call — a pathless 'git commit', or one carrying -a or -i, commits everything staged, so a path a concurrent session staged in this shared index rides your commit under your message. Name the paths: 'git commit -o <path>… -m '<msg>'' commits exactly those paths whatever else is staged; a new file is 'git add'ed first, in this same call or its own. A lone pathless 'git commit' is untouched. If you genuinely mean the whole index, run it yourself with !<command>.");
            }
            _ => {}
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `git_stash`'s verb read: whether the word after
// `stash` names a form that rewrites the working tree or the index; `None` on a verb git's own set
// does not carry.
fn stash_rewrites(verb: Option<&str>) -> Option<bool> {
    match verb {
        None | Some("push" | "save" | "pop" | "apply" | "branch") => Some(true),
        Some(v) if v.starts_with('-') => Some(true),
        Some("list" | "show" | "create" | "store" | "drop" | "clear") => Some(false),
        Some(_) => None,
    }
}

// spec: guard-kit/SPEC.md §The rule roster — rule `git_stash`: a tree-rewriting stash where the
// session's working tree is the main checkout.
pub fn git_stash(ctx: &Ctx) -> Decided {
    let c = ctx.cmd();
    if ctx.expands(c, has_expansion)? {
        return Ok(None);
    }
    for passed in git_read(ctx, c)?.iter().flatten() {
        let Some(globals) = git_walk_of(passed, GitWalk::Globals) else { continue };
        if globals.iter().any(|g| moves_repository(g)) {
            continue;
        }
        let ws = git_walk_of(passed, GitWalk::Args).unwrap_or_default();
        if ws.first().map(String::as_str) != Some("stash") || stash_rewrites(ws.get(1).map(String::as_str)) != Some(true) {
            continue;
        }
        let h = ctx.host();
        if !h.roots().main.is_empty() {
            return Ok(None);
        }
        let form = ws.get(1).filter(|v| !v.starts_with('-')).map_or("git stash".to_string(), |v| format!("git stash {}", v));
        return block(format!("don't run '{form}' on the main checkout — a stash rewrites the working tree and the index every session in this checkout stands on: it takes every uncommitted edit in the tree, a concurrent session's among them, out from under it and puts them back at a moment that session did not choose. Two exits: to read or build another revision, 'git worktree add {home}/<name> <rev>', removed with 'git worktree remove {home}/<name>' when done; to set your own edits aside, commit them, naming their paths ('git commit -o <path>… -m '<msg>''). 'git stash list', 'show', 'create', 'store', 'drop' and 'clear' are untouched, and so is every stash from a linked worktree, which has an index and files of its own. If you genuinely need the stash, run it yourself with !<command>.", form = form, home = h.first_scratch()));
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `rm_tracked`'s test, a block for it and a
// predicate for rule `bounded_wait`'s arm (B): the first refusal, or none.
pub fn rm_tracked_reach(ctx: &Ctx, c: &Cmd) -> Result<Option<String>, Fault> {
    if ctx.expands(c, has_expansion)? {
        return Ok(None);
    }
    let s = ctx.view(c, SqDqHd)?;
    let gits = git_read(ctx, c)?;
    let commands = command_read(ctx, c)?;
    let words = ctx.word_read(c)?;
    for (i, seg) in ctx.segments(&s).iter().enumerate() {
        let seg = trim_start(seg);
        if let Some(passed) = gits.get(i).and_then(Option::as_ref) {
            let ws = git_walk_of(passed, GitWalk::Args).unwrap_or_default();
            if ws.first().map(String::as_str) != Some("rm") {
                continue;
            }
            for w in &ws[1..] {
                if w == "--" {
                    break;
                }
                let force = matches!(w.as_str(), "--f" | "--fo" | "--for" | "--forc" | "--force")
                    || (w.len() >= 2 && w.starts_with('-') && !w[1..].starts_with('-') && w[1..].contains('f'));
                if force {
                    return Ok(Some(format!("don't force a 'git rm' with '{}' — the force flag is the one spelling of 'git rm' that destroys uncommitted work, silently when a committed grant matches it. Three exits: drop the flag, since 'git rm' refuses a file with local modifications and says so; use 'git rm --cached <path>' to untrack the file and keep it; or, where the loss is intended, run it yourself with !<command>.", w)));
                }
            }
            continue;
        }
        let ps = ctx.shell() == Shell::PowerShell;
        if !ps {
            let Some((head, args)) = commands.get(i).and_then(|ws| ws.split_first()) else { continue };
            if !names_command(head, "rm") {
                continue;
            }
            let args = args.iter().filter(|w| !w.value.starts_with('-'));
            let operands: Vec<Operand> = if words.is_some() {
                args.filter_map(bash_operand).collect()
            } else {
                args.map(|w| Operand { test: w.value.clone(), glob: false, value: w.value.clone(), spelling: w.value.clone() })
                    .collect()
            };
            if let Some(m) = first_tracked(ctx, operands, &head.value) {
                return Ok(Some(m));
            }
            continue;
        }
        let skel = head_word(seg);
        let read = words.as_ref().and_then(|w| w.get(i)).filter(|ws| ws.first().is_some_and(|w| w.live == skel));
        let word = read.and_then(|ws| ws.first()).map_or(skel, |w| w.spelling.as_str());
        let value = read.and_then(|ws| ws.first()).map_or(skel, |w| w.value.as_str());
        if !PS_DELETE.contains(&value.to_ascii_lowercase().as_str()) {
            continue;
        }
        let operands: Vec<Operand> = match read {
            Some(ws) => match ps_delete_paths(&ws[1..]) {
                Some(p) => p
                    .into_iter()
                    .map(|(path, spelling)| {
                        let value = path.replace('\\', "/");
                        Operand { test: format!(":(glob){}", value), glob: true, value, spelling }
                    })
                    .collect(),
                None => continue,
            },
            None => {
                let bare: Vec<Word> = text::all_words(&seg[word.len()..]).into_iter().map(plain_word).collect();
                match ps_delete_paths(&bare) {
                    Some(p) => p
                        .into_iter()
                        .map(|(path, spelling)| Operand { test: path.clone(), glob: false, value: path, spelling })
                        .collect(),
                    None => continue,
                }
            }
        };
        if let Some(m) = first_tracked(ctx, operands, word) {
            return Ok(Some(m));
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `rm_tracked`'s steer for the first operand naming a
// tracked path, or a directory holding one.
fn first_tracked(ctx: &Ctx, operands: Vec<Operand>, word: &str) -> Option<String> {
    let h = ctx.host();
    for o in operands {
        let dir = if h.tracked(&o.test) {
            std::path::Path::new(&o.value).is_dir()
        } else if o.glob && h.tracked(&format!("{}/**", o.test)) {
            true
        } else {
            continue;
        };
        return Some(rm_steer(&o, word, dir));
    }
    None
}

// spec: guard-kit/SPEC.md §The rule roster — rule `rm_tracked`'s operand: the pathspec the tracked
// test takes, whether it is a glob, the path the shell passes and the word the steer prints.
struct Operand {
    test: String,
    glob: bool,
    value: String,
    spelling: String,
}

// spec: guard-kit/SPEC.md §The rule roster — rule `rm_tracked`'s bash operand: a literal one by its
// value, a bare glob as the shell expands it, and none for a mixed word or a literal git would read
// as a pattern.
fn bash_operand(w: &Word) -> Option<Operand> {
    const GLOB: [char; 3] = ['*', '?', '['];
    let mut live_glob = false;
    let mut held = false;
    for ch in w.live.chars() {
        if std::mem::take(&mut held) {
            continue;
        }
        held = ch == '\\';
        live_glob |= GLOB.contains(&ch);
    }
    let test = if !live_glob {
        if w.value.contains(GLOB) || w.value.starts_with(':') {
            return None;
        }
        format!(":(literal){}", w.value)
    } else if w.live.contains(['\0', '\\']) {
        return None;
    } else {
        format!(":(glob){}", w.value)
    };
    Some(Operand { test, glob: live_glob, value: w.value.clone(), spelling: w.spelling.clone() })
}

// spec: guard-kit/SPEC.md §The rule roster — rule `rm_tracked`'s steer prints the operand as the
// command spelled it; a directory takes `-r`, without which `git rm` refuses it.
fn rm_steer(o: &Operand, word: &str, dir: bool) -> String {
    let form = if dir { "git rm -q -r" } else { "git rm -q" };
    format!("don't delete the git-tracked path '{v}' with a bare '{w}' — use '{f} {a}': it removes the path and stages exactly that deletion in one motion, so no later 'git add -A' is needed to pick it up (which risks staging a concurrent session's foreign path). An '{w}' of an untracked or gitignored path is untouched. If you genuinely need {w}, run it yourself with !<command>.", v = o.value, a = o.spelling, w = word, f = form)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(spelling: &str, value: &str, live: &str) -> Word {
        Word::new(spelling, value, live)
    }

    #[test]
    fn the_rm_steer_carries_r_for_a_directory_only() {
        let o = |w: &str| bash_operand(&plain_word(w)).unwrap();
        assert!(rm_steer(&o("kit"), "rm", true).contains("use 'git rm -q -r kit'"));
        assert!(rm_steer(&o("a.md"), "rm", false).contains("use 'git rm -q a.md'"));
    }

    // spec: guard-kit/SPEC.md §The rule roster — the stash forms that rewrite, the ones that leave the
    // tree as it is, and a verb the read does not know
    #[test]
    fn the_stash_verb_read_splits_rewriting_from_inert() {
        for v in [None, Some("-u"), Some("push"), Some("save"), Some("pop"), Some("apply"), Some("branch")] {
            assert_eq!(stash_rewrites(v), Some(true), "{:?}", v);
        }
        for v in ["list", "show", "create", "store", "drop", "clear"] {
            assert_eq!(stash_rewrites(Some(v)), Some(false), "{}", v);
        }
        assert_eq!(stash_rewrites(Some("frobnicate")), None);
    }

    // spec: guard-kit/SPEC.md §The rule roster — the steer prints a quoted and a blank-bearing operand
    // as the command spelled it, and the tracked test takes the path the shell passes
    #[test]
    fn the_rm_steer_prints_the_operand_as_spelled() {
        let quoted = bash_operand(&word("'two words.md'", "two words.md", "\0SQ")).unwrap();
        assert_eq!(quoted.test, ":(literal)two words.md");
        assert!(rm_steer(&quoted, "rm", false).contains("use 'git rm -q 'two words.md''"));
        assert!(rm_steer(&quoted, "rm", false).contains("path 'two words.md' with"));
        let escaped = bash_operand(&word("two\\ words.md", "two words.md", "two\\ words.md")).unwrap();
        assert_eq!(escaped.test, ":(literal)two words.md");
        assert!(rm_steer(&escaped, "rm", false).contains("use 'git rm -q two\\ words.md'"));
    }

    // spec: guard-kit/SPEC.md §The rule roster — a bare glob is tested as a glob, and a mixed word, a
    // quoted glob character and a leading `:` are skipped
    #[test]
    fn the_bash_operand_splits_literal_glob_and_skipped() {
        let bare = bash_operand(&plain_word("*.md")).unwrap();
        assert_eq!((bare.test.as_str(), bare.glob), (":(glob)*.md", true));
        assert!(bash_operand(&word("'*.md'", "*.md", "\0SQ")).is_none());
        assert!(bash_operand(&word("\\*.md", "*.md", "\\*.md")).is_none());
        assert!(bash_operand(&word("'a'*.md", "a*.md", "\0SQ*.md")).is_none());
        assert!(bash_operand(&word("a\\ *.md", "a *.md", "a\\ *.md")).is_none());
        assert!(bash_operand(&plain_word(":x")).is_none());
    }

    // spec: guard-kit/SPEC.md §The rule roster — under PowerShell every path is a glob by its value,
    // `\` folded, and the steer prints a lone path's own word or a comma part quoted where it must be
    #[test]
    fn the_powershell_paths_are_globs_by_value() {
        let ps = ps_delete_paths(&[
            word("'docs\\a b.md'", "docs\\a b.md", "\0SQ"),
            word("x,'it''s'", "x,it's", "x,\0SQ"),
            plain_word("-Force"),
            plain_word("-Path:a,b"),
            word("\"-Path:c d\"", "-Path:c d", "\0DQ"),
        ])
        .unwrap();
        let got: Vec<(&str, &str)> = ps.iter().map(|(p, s)| (p.as_str(), s.as_str())).collect();
        assert_eq!(
            got,
            vec![("docs\\a b.md", "'docs\\a b.md'"), ("x", "x"), ("it's", "'it''s'"), ("a", "a"), ("b", "b"), ("c d", "'c d'")]
        );
        assert!(ps_delete_paths(&[plain_word("a"), plain_word("-wh")]).is_none());
        assert!(ps_delete_paths(&[plain_word("-Filter"), plain_word("a")]).unwrap().is_empty());
    }

    // spec: guard-kit/SPEC.md §The rule roster — only a comma standing bare parts a word into paths
    #[test]
    fn a_quoted_or_escaped_comma_parts_no_path() {
        let paths = |w: Word| ps_delete_paths(&[w]).unwrap().into_iter().map(|(p, _)| p).collect::<Vec<_>>();
        assert_eq!(paths(word("'a,b'", "a,b", "\0SQ")), vec!["a,b"]);
        assert_eq!(paths(word("a`,b", "a,b", "a`,b")), vec!["a,b"]);
        assert_eq!(paths(word("a,'b,c'", "a,b,c", "a,\0SQ")), vec!["a", "b,c"]);
        assert_eq!(paths(word("-Path:'a,b',c", "-Path:a,b,c", "-Path:\0SQ,c")), vec!["a,b", "c"]);
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `rm_tracked`'s PowerShell deletion commands,
// matched without regard to case.
const PS_DELETE: &[&str] = &["rm", "del", "erase", "ri", "rd", "rmdir", "remove-item"];

// spec: guard-kit/SPEC.md §The generic ruleset — the value-taking parameters whose value is no path.
const PS_VALUE_PARAMS: &[&str] = &[
    "filter",
    "include",
    "exclude",
    "stream",
    "credential",
    "erroraction",
    "errorvariable",
    "warningaction",
    "warningvariable",
    "informationaction",
    "informationvariable",
    "outvariable",
    "outbuffer",
    "pipelinevariable",
    "progressaction",
];

// spec: guard-kit/SPEC.md §The generic ruleset — the paths a PowerShell deletion names, each
// parameter read by its prefix off the word's value, and each path with the word a steer prints for
// it; `None` where a `-WhatIf` prefix makes the segment delete nothing.
fn ps_delete_paths(args: &[Word]) -> Option<Vec<(String, String)>> {
    let mut out = Vec::new();
    let mut skip = false;
    for w in args {
        if std::mem::take(&mut skip) {
            continue;
        }
        let mut parts = powershell::array_elements(&w.spelling).unwrap_or_else(|| vec![w.clone()]).into_iter();
        let Some(head) = parts.next() else { continue };
        let Some(param) = head.value.strip_prefix('-') else {
            out.extend(std::iter::once(head).chain(parts).map(|p| (p.value, p.spelling)));
            continue;
        };
        let (spelled, value) = match param.split_once(':') {
            Some((n, v)) => (n, Some(v)),
            None => (param, None),
        };
        let name = spelled.to_ascii_lowercase();
        if name.len() >= 2 && "whatif".starts_with(&name) {
            return None;
        }
        let valued = !name.is_empty() && PS_VALUE_PARAMS.iter().any(|p| p.starts_with(&name));
        match value {
            None => skip = valued,
            Some(v) if !valued => {
                let word = head.spelling.strip_prefix(&format!("-{}:", spelled)).map_or_else(|| ps_word(v), str::to_string);
                out.extend(std::iter::once((v.to_string(), word)).chain(parts.map(|p| (p.value, p.spelling))));
            }
            Some(_) => {}
        }
    }
    out.retain(|(path, _)| !path.is_empty());
    Some(out)
}

// spec: guard-kit/SPEC.md §The rule roster — a path as a PowerShell steer prints it where the command
// spelled no word for it alone: bare where PowerShell reads it as written, single-quoted otherwise.
fn ps_word(p: &str) -> String {
    if p.bytes().all(|b| b.is_ascii_alphanumeric() || b"_./\\:+%^~-".contains(&b)) {
        p.to_string()
    } else {
        format!("'{}'", p.replace('\'', "''"))
    }
}

pub fn rm_tracked(ctx: &Ctx) -> Decided {
    match rm_tracked_reach(ctx, ctx.cmd())? {
        Some(m) => block(m),
        None => Ok(None),
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — a path word as the rule compares it: under
// PowerShell `\` folds to `/`, rule `worktree_confinement`'s precedent.
pub(super) fn path_word(ctx: &Ctx, w: &str) -> String {
    match ctx.shell() {
        Shell::PowerShell => w.replace('\\', "/"),
        Shell::Bash => w.to_string(),
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — the interpreter classification: arm (a) the bash/sh
// pair the runner serves, arm (b) the interpreter roster.
fn interpreter_arm(ctx: &Ctx, w: &str) -> Option<char> {
    let w = w.rsplit('/').next().unwrap_or(w);
    if matches!(w, "bash" | "sh") {
        return Some('a');
    }
    ctx.host().interpreters.iter().any(|i| i == w).then_some('b')
}

// spec: guard-kit/SPEC.md §The rule roster — arm (c)'s host word: the final path component less a
// trailing `.exe`, compared without regard to case under PowerShell.
fn powershell_host(ctx: &Ctx, w: &str) -> bool {
    let base = w.rsplit('/').next().unwrap_or(w);
    match ctx.shell() {
        Shell::PowerShell => {
            let b = base.to_ascii_lowercase();
            matches!(b.strip_suffix(".exe").unwrap_or(&b), "pwsh" | "powershell")
        }
        Shell::Bash => matches!(base.strip_suffix(".exe").unwrap_or(base), "pwsh" | "powershell"),
    }
}

enum Body {
    Inline,
    File(String),
    Stdin,
}

// spec: guard-kit/SPEC.md §The generic ruleset — a redirect word in either reader's grammar: `Some`
// with whether it stands alone and so takes the next word as its target.
pub(super) fn redirect_word(tok: &str) -> Option<bool> {
    let t = tok.trim_start_matches(|c: char| c.is_ascii_digit() || c == '*' || c == '&');
    let op = t.strip_prefix(">>").or_else(|| t.strip_prefix('>')).or_else(|| t.strip_prefix('<'));
    let lead = tok.len() - t.len();
    match op {
        Some(rest) if lead <= 1 || tok.starts_with("&>") => Some(rest.is_empty()),
        _ => None,
    }
}

// spec: guard-kit/SPEC.md §The generic ruleset — an interpreter takes its program body from a -c/-e
// argument, its first bare operand, or stdin; `None` on an option the walk cannot size.
fn interpreter_body(toks: &[&str], arm: char) -> Option<Body> {
    let mut skip = false;
    for &tok in toks.iter().skip(1) {
        if skip {
            skip = false;
            continue;
        }
        match tok {
            "-c" | "--command" | "-m" | "--module" => return Some(Body::Inline),
            "-e" | "--eval" => {
                if arm != 'a' {
                    return Some(Body::Inline);
                }
            }
            "-" | "/dev/stdin" | "/dev/fd/0" => return Some(Body::Stdin),
            "--" => {}
            _ if redirect_word(tok).is_some() => skip = redirect_word(tok) == Some(true),
            _ if tok.starts_with('-') => {
                let rest = &tok[1..];
                if rest.starts_with('-') || !rest.chars().all(|c| "BEIOilnstuvx".contains(c)) {
                    return None;
                }
            }
            _ => return Some(Body::File(tok.to_string())),
        }
    }
    Some(Body::Stdin)
}

// spec: guard-kit/SPEC.md §The rule roster — rule `script_interpreter`'s `<` sources by value: a bare
// `<` opening no heredoc, herestring, descriptor dup or read-write open, its target glued or the next word.
fn input_sources(ws: &[Word]) -> Vec<String> {
    let mut out = Vec::new();
    for (k, w) in ws.iter().enumerate() {
        let mut held = false;
        let at = w.live.char_indices().find(|&(_, ch)| {
            let bare = !std::mem::take(&mut held);
            held = bare && ch == '\\';
            bare && ch == '<'
        });
        let Some((at, _)) = at else { continue };
        if w.live[at + 1..].starts_with(['<', '&', '>', '(']) {
            continue;
        }
        let src = if at + 1 == w.live.len() { ws.get(k + 1).map(|n| n.value.as_str()) } else { w.value_after(at + 1) };
        out.extend(src.map(str::to_string));
    }
    out
}

// spec: guard-kit/SPEC.md §The rule roster — a PowerShell host's options, each matched by a prefix
// of its name without regard to case; the first three carry the body, the valued ones skip their
// value, and the order settles a prefix two names share.
const PS_BODY_OPTS: &[&str] = &["file", "command", "encodedcommand"];
const PS_VALUED_OPTS: &[&str] = &[
    "executionpolicy",
    "workingdirectory",
    "windowstyle",
    "outputformat",
    "inputformat",
    "configurationname",
    "configurationfile",
    "custompipename",
    "settingsfile",
    "psconsolefile",
    "version",
];
const PS_SWITCHES: &[&str] = &[
    "noprofile",
    "nologo",
    "noninteractive",
    "noexit",
    "interactive",
    "login",
    "sta",
    "mta",
    "noprofileloadtime",
];

// spec: guard-kit/SPEC.md §The rule roster — the host's own CLI grammar: `-File`'s operand or the
// first bare word is the body, `-Command` and `-EncodedCommand` carry it inline, a `-` after `-File`
// or `-Command` or no body word at all is stdin, and any other option declines.
fn powershell_body(toks: &[&str]) -> Option<Body> {
    let mut i = 1usize;
    while i < toks.len() {
        let tok = toks[i];
        i += 1;
        if let Some(alone) = redirect_word(tok) {
            i += usize::from(alone);
            continue;
        }
        if tok == "-" {
            return Some(Body::Stdin);
        }
        let Some(name) = tok.strip_prefix('-').map(str::to_ascii_lowercase) else {
            return Some(Body::File(tok.to_string()));
        };
        let is = |full: &str| !name.is_empty() && full.starts_with(name.as_str());
        match PS_BODY_OPTS.iter().find(|o| is(o)) {
            Some(&"file") => {
                return match toks.get(i) {
                    Some(&"-") => Some(Body::Stdin),
                    Some(w) => Some(Body::File(w.to_string())),
                    None => None,
                }
            }
            Some(&"command") if toks.get(i) == Some(&"-") => return Some(Body::Stdin),
            Some(_) => return Some(Body::Inline),
            None => {}
        }
        if matches!(name.as_str(), "ep" | "wd") || PS_VALUED_OPTS.iter().any(|o| is(o)) {
            i += 1;
        } else if !PS_SWITCHES.iter().any(|o| is(o)) {
            return None;
        }
    }
    Some(Body::Stdin)
}

// spec: guard-kit/SPEC.md §The generic ruleset — where a body source sits: under the main checkout's
// scratch dir seen from a linked worktree, or under a scratch dir the runner here serves.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Home {
    Here,
    Main,
}

// spec: guard-kit/SPEC.md §The generic ruleset — the scratch-source test on one token: a member as
// written, and from a linked worktree a member of the main checkout or of the session's own
// worktree, each compared lexically with `..` folded.
fn scratch_home(ctx: &Ctx, p: &str) -> Option<Home> {
    let p = path_word(ctx, p);
    let h = ctx.host();
    let own = h.own_scratch_homes();
    if !own.is_empty() {
        let t = h.lexical(&p);
        let main_homes = h.scratch_homes();
        if main_homes.iter().zip(&own).any(|(m, o)| m != o && under(&t, &h.lexical(m))) {
            return Some(Home::Main);
        }
        if own.iter().any(|o| under(&t, o)) {
            return Some(Home::Here);
        }
    }
    h.scratch_prefixes()
        .iter()
        .any(|(bare, dotted)| p.starts_with(bare.as_str()) || p.starts_with(dotted.as_str()))
        .then_some(Home::Here)
}

fn names_scratch(ctx: &Ctx, s: &str) -> bool {
    let s = path_word(ctx, s);
    ctx.host().scratch_prefixes().iter().any(|(bare, _)| s.contains(bare.as_str()))
}

// spec: guard-kit/SPEC.md §The generic ruleset — the bash expansion decline: a substitution is a
// body source this rule reads, so only the parameter and process forms decline.
fn declines_expansion(raw: &str) -> bool {
    grep_q(r"\$\{|<\(|>\(|\$[A-Za-z_]", raw)
}

pub type Reach = (char, String, String, Home);

// spec: guard-kit/SPEC.md §The rule roster — arm (c) case 2, the PowerShell invocation forms, none
// under a lead word opening on a quote: the call or dot-source operator before a scratch path, a
// scratch path in command position, and `Invoke-Expression` in a segment or behind a producer.
fn powershell_form(ctx: &Ctx, toks: &[&str], string_lead: bool, producers: &[Vec<&str>]) -> Option<Reach> {
    let word = *toks.first()?;
    let hit = |src: &str, home: Home| Some(('c', word.to_string(), src.to_string(), home));
    if string_lead {
        return None;
    }
    if matches!(word, "&" | ".") {
        let next = toks.get(1)?;
        return scratch_home(ctx, next).and_then(|home| hit(next, home));
    }
    if let Some(home) = scratch_home(ctx, word) {
        return hit(word, home);
    }
    if !matches!(word.to_ascii_lowercase().as_str(), "invoke-expression" | "iex") {
        return None;
    }
    let sources = toks[1..].iter().chain(producers.iter().flatten()).copied();
    for tok in sources {
        if let Some(home) = scratch_home(ctx, tok) {
            return hit(tok, home);
        }
    }
    None
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `script_interpreter`'s test, a block for it and
// a predicate for rule `bounded_wait`'s arm (B): the first interpreter taking its body from a scratch
// path, as its arm, its word and the body source.
pub fn interpreter_reach(ctx: &Ctx, c: &Cmd) -> Result<Option<Reach>, Fault> {
    let raw = ctx.raw(c)?;
    let ps = ctx.shell() == Shell::PowerShell;
    let read = ctx.word_read(c)?;
    let valued = read.iter().flatten().flatten().any(|w| names_scratch(ctx, &w.value));
    if !(names_scratch(ctx, raw) || valued) || ctx.expands(c, declines_expansion)? {
        return Ok(None);
    }
    let s = ctx.view(c, SqDqHd)?;
    let members: Vec<Vec<String>> = ctx.statements(&s).iter().map(|stmt| ctx.pipes(stmt)).collect();
    let commands = Some(command_read(ctx, c)?).filter(|cr| cr.len() == members.iter().map(Vec::len).sum::<usize>());
    let mut at = 0usize;
    for pipes in &members {
        for (i, pipe) in pipes.iter().enumerate() {
            at += 1;
            let seg = command_word(pipe);
            let lifted = commands.as_ref().map(|cr| &cr[at - 1]);
            let toks: Vec<&str> = match lifted {
                Some(ws) => ws.iter().map(|w| w.value.as_str()).collect(),
                None => words(&seg),
            };
            let Some(&head) = toks.first() else { continue };
            let word = path_word(ctx, head);
            if word.is_empty() {
                continue;
            }
            if ps {
                let producers: Vec<Vec<&str>> = (0..i)
                    .map(|j| match commands.as_ref() {
                        Some(cr) => cr[at - 1 - i + j].iter().map(|w| w.value.as_str()).collect(),
                        None => words(&pipes[j]),
                    })
                    .collect();
                let string_lead = lifted.and_then(|ws| ws.first()).is_some_and(|w| w.spelling.starts_with(['\'', '"']));
                if let Some(r) = powershell_form(ctx, &toks, string_lead, &producers) {
                    return Ok(Some(r));
                }
            }
            let (arm, body) = if powershell_host(ctx, &word) {
                ('c', powershell_body(&toks))
            } else {
                let Some(arm) = interpreter_arm(ctx, &word) else { continue };
                (arm, interpreter_body(&toks, arm))
            };
            let hit = |src: &str, home: Home| Some((arm, word.clone(), src.to_string(), home));
            let own = read.as_ref().filter(|_| lifted.is_some()).and_then(|r| r.get(at - 1));
            match body {
                None => continue,
                Some(Body::File(p)) => {
                    if let Some(home) = scratch_home(ctx, &p) {
                        return Ok(hit(&p, home));
                    }
                }
                Some(Body::Stdin) => {
                    let sources: Vec<String> = match own {
                        Some(ws) => input_sources(ws),
                        None => text::grep_o("(^|[^<])<[[:blank:]]*[^[:blank:]\n<>|;&]+", pipe)
                            .iter()
                            .map(|m| {
                                let src = m.strip_prefix(|ch: char| ch != '<').unwrap_or(m);
                                text::trim_blank_start(src.strip_prefix('<').unwrap_or(src)).to_string()
                            })
                            .collect(),
                    };
                    for src in &sources {
                        if let Some(home) = scratch_home(ctx, src) {
                            return Ok(hit(src, home));
                        }
                    }
                    for (j, prev) in pipes[..i].iter().enumerate() {
                        let toks: Vec<String> = match own.and(read.as_ref().and_then(|r| r.get(at - 1 - i + j))) {
                            Some(ws) => ws.iter().map(|w| w.value.clone()).collect(),
                            None => words(prev).into_iter().map(str::to_string).collect(),
                        };
                        for tok in &toks {
                            if let Some(home) = scratch_home(ctx, tok) {
                                return Ok(hit(tok, home));
                            }
                        }
                    }
                }
                Some(Body::Inline) if !ps => {
                    let subst = text::grep_o(r"`[^`]*`|\$\([^)]*\)", raw);
                    if subst.iter().any(|span| names_scratch(ctx, span)) {
                        return Ok(hit("a command substitution", Home::Here));
                    }
                }
                Some(Body::Inline) => {}
            }
        }
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The shell guard — the steer door as the deciding reader's shell runs it:
// under PowerShell the call operator before the door single-quoted, any `'` doubled.
fn runner(ctx: &Ctx) -> String {
    let door = &ctx.host().door;
    match ctx.shell() {
        Shell::PowerShell => format!("& '{}' --scratch-run", door.replace('\'', "''")),
        Shell::Bash => format!("{} --scratch-run", door),
    }
}

pub fn script_interpreter(ctx: &Ctx) -> Decided {
    let Some((arm, word, src, home)) = interpreter_reach(ctx, ctx.cmd())? else { return Ok(None) };
    let runner = runner(ctx);
    let host = &ctx.host().scratch_powershell;
    let ps_on = arm == 'c' && !host.is_empty();
    let inline = if arm == 'c' {
        "A body carried in the command string — a '-Command' argument, a here-string — is untouched"
    } else {
        "A body carried in the command string — a '-c' argument, a heredoc, a herestring — is untouched"
    };
    let off = if arm == 'c' && host.is_empty() {
        "the PowerShell scratch path is off in this project (GUARD_KIT_SCRATCH_POWERSHELL is empty; guard-kit/SPEC.md §scratch-run), and "
    } else {
        ""
    };
    if home == Home::Main {
        let own = ctx.host().own_scratch_homes().into_iter().next().unwrap_or_default();
        let lead = if arm == 'a' || ps_on {
            let script = if ps_on { "<script>.ps1" } else { "<script>" };
            format!("run a scratch script through the runner from this linked worktree: write it under the worktree's own scratch dir ({own}) and run '{runner} {script} [args…]' there, since the runner refuses a script under the main checkout's scratch dir from here (guard-kit/SPEC.md §scratch-run). This call takes the program body for '{word}' from '{src}'")
        } else if arm == 'c' {
            format!("{off}this call takes the program body for '{word}' from '{src}'. Write the body as a bash script under this linked worktree's own scratch dir ({own}) and run it through '{runner} <script> [args…]' there, since the runner refuses a script under the main checkout's scratch dir from here. The body sits")
        } else {
            format!("scratch execution runs in bash, or in PowerShell where the project names a host (guard-kit/SPEC.md §scratch-run), and '{word}' is neither: this call takes its program body from '{src}'. Write the body as a shell script under this linked worktree's own scratch dir ({own}) and run it through '{runner} <script> [args…]' there, since the runner refuses a script under the main checkout's scratch dir from here. The body sits")
        };
        return block(format!("{lead} in a scratch dir any session can rewrite, so the body reviewed at the permission decision need not be the body that runs; the runner echoes the body as it executes, which is the compensating control a direct run has none of. {inline}. If you genuinely need the direct form, run it yourself with !<command>."));
    }
    if arm == 'a' || ps_on {
        let (script, runs) = if ps_on {
            ("<script>.ps1", format!(", which echoes the body and runs it under the project's PowerShell host ({host})"))
        } else {
            ("<script>", String::new())
        };
        return block(format!("run a scratch script through the runner: '{runner} {script} [args…]'{runs} (guard-kit/SPEC.md §scratch-run). This call takes the program body for '{word}' from '{src}', which sits in a scratch dir any session can rewrite, so the body reviewed at the permission decision need not be the body that runs. The runner is allowlistable and echoes the body as it executes, which is the compensating control a direct run has none of. {inline}. If you genuinely need the direct form, run it yourself with !<command>."));
    }
    if arm == 'c' {
        return block(format!("{off}this call takes the program body for '{word}' from '{src}' under a scratch dir, where no compensating control reaches it. Write the body as a bash script under the scratch dir and run it through '{runner} <script> [args…]', which echoes the body as it executes. {inline}, because the approver and the friction log both see it verbatim. If you genuinely need the direct run, run it yourself with !<command>."));
    }
    block(format!("scratch execution runs in bash, or in PowerShell where the project names a host (guard-kit/SPEC.md §scratch-run), and '{word}' is neither: this call takes its program body from '{src}' under a scratch dir, where no compensating control reaches it. Write the body as a shell script and run it through '{runner} <script> [args…]', which echoes the body as it executes; a script whose shebang names a non-bash interpreter is refused there too. {inline}, because the approver and the friction log both see it verbatim. If you genuinely need the direct run, run it yourself with !<command>."))
}

pub fn shell_wrapper(ctx: &Ctx) -> Decided {
    let raw = ctx.raw(ctx.cmd())?;
    let dequoted = ctx.dequoted(ctx.cmd())?.unwrap_or_default();
    if !(raw.contains("sh") || dequoted.contains("sh")) {
        return Ok(None);
    }
    for ws in passed_read(ctx, ctx.cmd())? {
        let lead = ws.iter().take_while(|w| w.value == "command").count();
        if ws.get(lead).is_some_and(|w| w.value.starts_with('-')) {
            continue;
        }
        let Some((head, args)) = ws[lead.min(ws.len())..].split_first() else { continue };
        let word = head.value.rsplit('/').next().unwrap_or("");
        if !matches!(word, "bash" | "sh") || args.first().map(|w| w.value.as_str()) != Some("-c") {
            continue;
        }
        return block(format!("don't wrap a command in '{} -c': the payload sits inside one quoted argument, so neither the allowlist nor any guard rule can read what it runs. Run the payload as the command itself, or, for a body that needs a shell of its own, write it to a scratch script and run it through '{} --scratch-run <script>'. If you genuinely need the wrapper, run it yourself with !<command>.", word, ctx.host().door));
    }
    Ok(None)
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `worktree_confinement`'s test as a predicate,
// taken under that rule's own declared views; from a linked worktree it also refuses a call carrying
// an ANSI-C quoted word, whose value its dequoted path-word read does not hold.
pub fn refused(ctx: &Ctx, c: &Cmd) -> Result<bool, Fault> {
    ctx.under("worktree_confinement", |w| {
        let linked = !w.host().roots().main.is_empty() && w.shell() == Shell::Bash;
        if linked && w.word_read(c)?.is_some_and(|r| r.iter().flatten().any(Word::ansi_c)) {
            return Ok(true);
        }
        Ok(worktree_refuses(w, c)?.is_some())
    })
}

// spec: guard-kit/SPEC.md §The generic ruleset — rule `worktree_confinement`'s test: from a linked
// worktree, the first path word resolving into the main checkout outside its scratch dirs, unless
// the command is the admitted read.
pub fn worktree_refuses(ctx: &Ctx, c: &Cmd) -> Result<Option<(String, String)>, Fault> {
    let h = ctx.host();
    if h.roots().main.is_empty() {
        return Ok(None);
    }
    let ps = ctx.shell() == Shell::PowerShell;
    let live = ctx.view(c, SqHdq)?;
    let declines = if ps {
        live.contains('$')
    } else {
        grep_q(r"\$\{|\$\(|<\(|>\(|\$[A-Za-z_]", &live) || ctx.raw(c)?.contains('`')
    };
    if declines {
        return Ok(None);
    }
    let s = ctx.view(c, SqDqHd)?;
    let v = ctx.dequoted(c)?.unwrap_or_else(|| s.clone());
    let mut found: Option<(String, String)> = None;
    'lines: for line in v.split('\n') {
        for w in words(line) {
            let mut w = unsentinel(w);
            if ps {
                w = w.replace('\\', "/");
                if w.starts_with("*>") {
                    w.remove(0);
                }
            }
            let digits = w.bytes().take_while(u8::is_ascii_digit).count();
            let after = &w[digits..];
            let op = [">>", ">&", "&>", ">", "<"].iter().find(|o| after.starts_with(*o));
            if let Some(op) = op {
                w = after[op.len()..].to_string();
            }
            if w.starts_with("--") && w.len() > 3 && w.as_bytes()[3..].contains(&b'=') {
                w = w.split_once('=').map_or(w.clone(), |(_, r)| r.to_string());
            }
            if walk::path_root(&w).is_none() {
                let dotdot = w == ".." || w.starts_with("../") || w.ends_with("/..") || w.contains("/../");
                if !dotdot {
                    continue;
                }
            }
            let t = h.lexical(&w);
            if !h.in_main(&t) || h.in_scratch(&t) {
                continue;
            }
            found = Some((w, t));
            break 'lines;
        }
    }
    let Some(hit) = found else { return Ok(None) };
    if !ps && worktree_admitted(ctx, c, &s)? {
        return Ok(None);
    }
    Ok(Some(hit))
}

// spec: guard-kit/SPEC.md §The rule roster — rule `worktree_confinement`'s second admitted form: the
// gate binary, or the front end under bash, running a read-only arm, `--emit <name>` normalized to
// `--emit-<name>` as the binary normalizes it.
fn read_only_arm(h: &crate::guard::host::Host, cw: &str) -> bool {
    let ws = words(cw);
    let r = h.roots();
    let at = |w: &str, rel: &str| {
        let rel = rel.strip_prefix("./").unwrap_or(rel);
        let t = h.lexical(w);
        t == h.lexical(rel) || (!r.main.is_empty() && t == h.lexical(&format!("{}/{}", r.main, rel)))
    };
    let arm_at = match ws.first() {
        Some(&w) if w == h.door || (walk::path_root(&h.door).is_none() && at(w, &h.door)) => 1,
        Some(&"bash") if ws.get(1).is_some_and(|w| at(w, "gate-sdk/bin/run-gates.sh")) => 2,
        _ => return false,
    };
    let arm = match ws.get(arm_at) {
        Some(&"--emit") => match ws.get(arm_at + 1) {
            Some(name) => format!("--emit-{}", name),
            None => return false,
        },
        Some(a) => a.to_string(),
        None => return false,
    };
    crate::emit::READ_ONLY_ARMS.contains(&arm.as_str())
}

// spec: guard-kit/SPEC.md §The rule roster — the read-only arms as a steer prints them, on the door.
fn read_only_arm_spellings(h: &crate::guard::host::Host) -> String {
    crate::emit::READ_ONLY_ARMS
        .iter()
        .map(|a| format!("'{} {}'", h.door, a.replacen("--emit-", "--emit ", 1)))
        .collect::<Vec<_>>()
        .join(", ")
}

// spec: guard-kit/SPEC.md §The generic ruleset — the admitted read: every segment a roster read in
// none of its declared write and execute forms, none led by a program-bearing tool, and every
// redirect target inert, in the own worktree or under a main-checkout scratch dir.
fn worktree_admitted(ctx: &Ctx, c: &Cmd, s: &str) -> Result<bool, Fault> {
    let h = ctx.host();
    if h.worktree_reads != "read-only" || shell_backgrounds(s) {
        return Ok(false);
    }
    let mut reads = 0usize;
    for seg in ctx.segments(s) {
        let seg = trim_start(&seg);
        if seg.is_empty() || is_banner(seg) {
            continue;
        }
        let cw = command_word(&segment_core(seg));
        if read_only_arm(h, &cw) {
            reads += 1;
            continue;
        }
        if program_bearing(h, head_word(&cw)) || !is_ro_segment(h, &cw) {
            return Ok(false);
        }
        reads += 1;
    }
    if reads == 0 {
        return Ok(false);
    }
    for pair in ctx.redirect_pairs(s) {
        if pair.is_empty() {
            continue;
        }
        let tgt = super::strip_redirect_op(&pair);
        if inert_target(tgt) {
            continue;
        }
        let t = h.lexical(tgt);
        if under(&t, &h.roots().own) || (h.in_main(&t) && h.in_scratch(&t)) {
            continue;
        }
        return Ok(false);
    }
    ro_forms_clear(ctx, c)
}

pub fn worktree_confinement(ctx: &Ctx) -> Decided {
    let h = ctx.host();
    let Some((word, path)) = worktree_refuses(ctx, ctx.cmd())? else { return Ok(None) };
    let reads = if h.worktree_reads == "read-only" && ctx.shell() == Shell::Bash {
        let roster: Vec<&str> = h.ro_bins.iter().map(String::as_str).filter(|b| !program_bearing(h, b)).collect();
        format!(" a search or read of it as a read-only pipeline, every segment led by one of the read-only roster ({}) in none of its write or execute forms and no segment led by sed, awk or an interpreter, redirecting only to /dev/null, your own worktree or the main checkout's scratch dir; a read-only arm of the gate binary ({});", roster.join(" "), read_only_arm_spellings(h))
    } else {
        String::new()
    };
    block(format!("'{}' reaches the main checkout ({}) from this linked worktree, and a shell command naming the main checkout outside its scratch dir is refused here: an isolated session never alters the work tree it was isolated from. Lawful routes: the Read tool for a named file;{} a history read (git log, git show) in your own worktree, which shares the main checkout's refs and objects; and your journal, appended by shell under the main checkout's {} dir. If the command is genuinely needed, return and have your dispatcher run it in the main checkout.", word, path, reads, h.first_scratch()))
}
