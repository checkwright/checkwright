// spec: installer/SPEC.md §The consumer smoke — the manifest arm's failure report: a straight-line
// sequence of prints, run while the disagreeing consumer is on disk and before the verdict, with no
// branch that can change it
use super::consumer::{digest_of, git_out, seam_bin, Lock, GATES_DIR};
use super::{in_consumer, text, Run};
use crate::programs;

type Entry = (String, String, String);

// spec: installer/SPEC.md §The consumer smoke — the shape test's verdict beside the first byte
// outside `0-9a-f`, so a reader gets a position and not only a refusal
fn shape_verdict(v: &str) -> String {
    let first = v.bytes().position(|b| !(b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    format!(
        "shape={} first={}",
        if super::consumer::is_hash(v) { "pass" } else { "fail" },
        first.map_or("none".to_string(), |i| i.to_string())
    )
}

// spec: installer/SPEC.md §The consumer smoke — the octet dump carries the byte claim, rendered from
// the value's bytes with no construct that could normalize
fn octets(v: &str) -> String {
    v.bytes().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
}

fn value_probe(label: &str, v: &str, call: &str, err: Option<&str>) {
    println!("    {:<7} {}", label, if v.is_empty() { "<empty>" } else { v });
    println!("            len    {}", v.len());
    println!("            octets {}", octets(v));
    println!("            tests  {}", shape_verdict(v));
    println!("            call   {}", call);
    if let Some(e) = err.filter(|e| !e.is_empty()) {
        println!("            stderr {}", e);
    }
}

// spec: installer/SPEC.md §The consumer smoke — a re-read: the command run now, its value, and its
// standard error so a refusal is named instead of showing as an empty value
fn reread(label: &str, cwd: &str, args: &[&str]) {
    let call = format!("git {}", args.join(" "));
    match in_consumer(&programs::GIT, args, &[], cwd) {
        Ok(done) => {
            let (o, e) = done.streams();
            value_probe(label, text(o).trim_end_matches('\n'), &call, Some(text(e).trim_end()));
        }
        Err(super::Outcome::Fail(e) | super::Outcome::Refuse(e)) => value_probe(label, "", &call, Some(&e)),
    }
}

fn first_n(cwd: &str, args: &[&str], n: usize) {
    let said = match in_consumer(&programs::GIT, args, &[], cwd) {
        Ok(done) => {
            let (o, e) = done.streams();
            format!("{}{}", text(o), text(e))
        }
        Err(super::Outcome::Fail(e) | super::Outcome::Refuse(e)) => e,
    };
    for line in said.lines().take(n) {
        println!("{}", line);
    }
}

// spec: installer/SPEC.md §The consumer smoke — one attribute lookup, printed as three
// distinguishable outcomes: attributes, git's refusal, or a clean silence
fn attr_probe(repo: &str, which: &str, path: &str) {
    println!("    {}, {}:", path, which);
    let (o, e) = match in_consumer(&programs::GIT, &["check-attr", "-a", "--", path], &[], repo) {
        Ok(done) => {
            let (o, e) = done.streams();
            (text(o).trim_end().to_string(), text(e).trim_end().to_string())
        }
        Err(super::Outcome::Fail(e) | super::Outcome::Refuse(e)) => (String::new(), e),
    };
    if !o.is_empty() {
        println!("{}", o);
    }
    if !e.is_empty() {
        println!("      refused: {}", e);
    }
    if o.is_empty() && e.is_empty() {
        println!("      <no attribute reported>");
    }
}

// spec: installer/SPEC.md §The consumer smoke — the refused operand(s) of the witness tuple, each with
// the shape verdict, both spelled out where both fail
pub(super) fn malformed_operands(w: &Entry) -> String {
    let mut which = Vec::new();
    if !super::consumer::is_hash(&w.1) {
        which.push(format!("want ({})", shape_verdict(&w.1)));
    }
    if !super::consumer::is_hash(&w.2) {
        which.push(format!("got ({})", shape_verdict(&w.2)));
    }
    which.join(" and ")
}

// spec: installer/SPEC.md §The consumer smoke — at most three samples, none chosen by arrival: the
// first disagreeing path, the artifact row, and the exit-2 verdict's witness deduplicated on the
// whole tuple, with the coincidence check's three outcomes printed
#[allow(clippy::too_many_arguments)]
pub(super) fn manifest_report(
    state: &Run,
    profile: &str,
    c: &str,
    lock: &Lock,
    mismatch: usize,
    checked: usize,
    witness: Option<&Entry>,
    malformed_n: usize,
    bad: &[Entry],
) {
    println!("  == manifest report: {}, {} of {} entries disagree ==", profile, mismatch, checked);
    println!("  read the values below against the truth table in installer/SPEC.md §The consumer smoke");
    let Some(first) = bad.first() else {
        println!("  every disagreement is a path the manifest names and the tree does not hold, so there is no hash to compare");
        return;
    };
    println!(
        "  {} of those {} carry an operand that is not 40 lowercase hex; the samples below are the first disagreeing path, the artifact row and the first such entry, deduplicated",
        malformed_n, mismatch
    );
    let mut samples: Vec<(Entry, &str)> = vec![(first.clone(), "")];
    let target = lock.artifact("target");
    let seam = format!("{}/gate-sdk-config.knobs", GATES_DIR);
    let art = if !target.is_empty() && lock.has_file(&seam) {
        seam_bin(&format!("{}/{}", c, seam)).unwrap_or_default()
    } else {
        String::new()
    };
    if target.is_empty() {
        println!("  the manifest records no artifact key, so the discriminating binary sample is absent from this payload");
    } else if art.is_empty() {
        println!("  the manifest records artifact {} but no config seam names its path, so the binary sample is unresolved", target);
    } else {
        match bad.iter().find(|e| e.0 == art) {
            None => println!("  the artifact row {} is not in the disagreeing set, so the sample is the first path alone", art),
            Some(found) if found == first => {
                println!("  the artifact row {} is also the first disagreeing path, so the two samples coincide", art)
            }
            Some(found) => samples.push((found.clone(), "")),
        }
    }
    match witness {
        None => println!(
            "  no disagreeing entry failed the operand shape test, so the verdict below is the manifest one and the samples are the {} path(s) above",
            samples.len()
        ),
        Some(w) if samples.iter().any(|(s, _)| s == w) => println!(
            "  the witness row {} is one of the samples already chosen, byte for byte, so the verdict row and that sample coincide",
            w.0
        ),
        Some(w) => {
            if samples.iter().any(|(s, _)| s.0 == w.0) {
                println!(
                    "  the witness row {} is path-equal to a sample already chosen and its bytes DIFFER, so both blocks are printed below: two decompositions of one recorded entry disagree, which is a statement about this harness and not about the consumer tree, and no reading of the consumer tree may be taken from this run manifest arm",
                    w.0
                );
            }
            samples.push((w.clone(), " — the witness row the exit-2 verdict below is computed from"));
        }
    }
    for ((p, want, got), role) in &samples {
        println!("  -- {}{}", p, role);
        value_probe("want", want, "the want the manifest loop paired with this path, read off the lock by the crate's JSON reader", None);
        value_probe("got", got, "the got the manifest loop paired off the git hash-object --stdin-paths batch", None);
        let abs = format!("{}/{}", c, p);
        reread("reread", &state.root, &["hash-object", "--", &abs]);
        reread("own", c, &["hash-object", "--", p]);
        reread("raw", &state.root, &["hash-object", "--no-filters", "--", &abs]);
    }
    println!("  -- the consumer worktree, the witness for a tree that has diverged from what init committed");
    first_n(c, &["status", "--porcelain"], 40);
    first_n(c, &["log", "-1", "--stat"], 40);
    println!("  -- core.autocrlf, core.eol and core.safecrlf with their origins, in both repositories");
    for r in [c, state.root.as_str()] {
        println!("    in {}", r);
        let listed = git_out(r, &["config", "--list", "--show-origin"]).unwrap_or_default();
        let set: Vec<&str> = listed
            .lines()
            .filter(|l| {
                let low = l.to_ascii_lowercase();
                ["core.autocrlf=", "core.eol=", "core.safecrlf="].iter().any(|k| low.contains(k))
            })
            .collect();
        if set.is_empty() {
            println!("      <none of the three is set>");
        }
        for l in set {
            println!("{}", l);
        }
    }
    println!("  -- git check-attr -a for each sampled path, in both repositories, since an attribute reaches a path the config does not");
    for ((p, _, _), _) in &samples {
        attr_probe(c, "in the consumer", p);
        attr_probe(&state.root, "in the smoke repository", &format!("{}/{}", c, p));
    }
    let art_path = format!("{}/{}", c, art);
    if !target.is_empty() && !art.is_empty() && super::consumer::is_file(&art_path) {
        println!("  -- the artifact digest, a control on the content question alone: SHA-256 is taken by no git filter and in no repository context");
        println!("    subject    {} ({})", art, target);
        println!("    recorded   {}", lock.artifact("digest"));
        println!("    recomputed {}", digest_of(&art_path));
        println!("    equal means the artifact bytes are exactly the bytes init published, which speaks for this path and no other");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The consumer smoke — the verdict names the first offending byte, and a
    // well-formed hash passes with none
    #[test]
    fn the_shape_verdict_names_the_first_offending_byte() {
        let hash = "0123456789abcdef0123456789abcdef01234567";
        assert_eq!(shape_verdict(hash), "shape=pass first=none");
        assert_eq!(shape_verdict(&format!("{}\r", hash)), "shape=fail first=40");
        assert_eq!(shape_verdict("01X3"), "shape=fail first=2");
        assert_eq!(octets("a\r"), "61 0d");
    }

    // spec: installer/SPEC.md §The consumer smoke — both refused operands are spelled out
    #[test]
    fn both_refused_operands_are_named() {
        let w = ("p".to_string(), "x".to_string(), "y".to_string());
        assert_eq!(malformed_operands(&w), "want (shape=fail first=0) and got (shape=fail first=0)");
    }
}
