// spec: canon-kit/SPEC.md §check-prose-enum — within one manifest-prose paragraph, naming
// two or more members of a declared governed set must name every member, unless an exempt
// site holds; the vocabulary arrives as bridged data, never from an emitter spawned here
use crate::spec;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-prose-enum: {}", e);
            2
        }
    }
}

struct Set {
    name: String,
    members: Vec<String>,
    lowered: Vec<String>,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    if spec::knob_pub("CANON_KIT_ENUM_SETS_CMD")?.is_empty() {
        println!("PROSE-ENUM: clean (CANON_KIT_ENUM_SETS_CMD unset — no declared sets to check)");
        return Ok(0);
    }
    let names = spec::knob_array_pub("CANON_KIT_ENUM_SET_NAMES")?;
    let members = spec::knob_array_pub("CANON_KIT_ENUM_SET_MEMBERS")?;
    if names.len() != members.len() {
        return Err(format!(
            "the bridged enum vocabulary is not index-aligned: {} set name(s) against {} \
             member(s) — the config bridge could not carry it; treating as failure (not clean)",
            names.len(),
            members.len()
        ));
    }
    if names.is_empty() {
        println!("PROSE-ENUM: clean (CANON_KIT_ENUM_SETS_CMD declared no members)");
        return Ok(0);
    }
    let mut sets: Vec<Set> = Vec::new();
    for (n, m) in names.iter().zip(members.iter()) {
        match sets.iter_mut().find(|s| &s.name == n) {
            Some(s) => {
                s.members.push(m.clone());
                s.lowered.push(m.to_ascii_lowercase());
            }
            None => sets.push(Set {
                name: n.clone(),
                members: vec![m.clone()],
                lowered: vec![m.to_ascii_lowercase()],
            }),
        }
    }

    let manifests = spec::manifest_files_sorted_stripped(root)?;
    if manifests.is_empty() {
        println!("PROSE-ENUM: clean (0 manifest file(s) found)");
        return Ok(0);
    }

    struct Sink {
        sets: Vec<Set>,
        out: Vec<String>,
    }
    impl spec::ProseSink for Sink {
        fn on_pflush(&mut self, file: &str, para: &spec::Para) {
            if para.len() < 1 {
                return;
            }
            let text = para.join(1, para.len());
            let low: Vec<u8> = text.bytes().map(|c| c.to_ascii_lowercase()).collect();
            for s in &self.sets {
                let mut hits: Vec<(usize, usize)> = Vec::new();
                let mut miss: Vec<String> = Vec::new();
                for (k, m) in s.lowered.iter().enumerate() {
                    match present(&low, m.as_bytes()) {
                        Some(p) => hits.push((p, m.len())),
                        None => miss.push(s.members[k].clone()),
                    }
                }
                if hits.len() < 2 || miss.is_empty() {
                    continue;
                }
                hits.sort();
                let mut maxrun = 1usize;
                let mut run = 1usize;
                for i in 1..hits.len() {
                    let gs = hits[i - 1].0 + hits[i - 1].1;
                    let ge = hits[i].0;
                    let gap = if ge > gs { &low[gs..ge] } else { &low[0..0] };
                    if adjacent(gap) {
                        run += 1;
                        if run > maxrun {
                            maxrun = run;
                        }
                    } else {
                        run = 1;
                    }
                }
                if maxrun < 2 {
                    continue; // scattered mentions, not a hand list
                }
                if subset_marked(&low) {
                    continue;
                }
                let first = hits[0].0;
                let pre = &low[first.saturating_sub(32)..first];
                if partitive(pre) {
                    continue;
                }
                self.out.push(format!(
                    "  {}:{}  set '{}' lists {} of {} member(s) but omits: {}",
                    file,
                    para.fnr[0],
                    s.name,
                    hits.len(),
                    hits.len() + miss.len(),
                    miss.join(", ")
                ));
            }
        }
    }

    let mut sink = Sink {
        sets,
        out: Vec::new(),
    };
    spec::walk_prose(&manifests, "prose-enum-exempt:", &mut sink)?;

    if !sink.out.is_empty() {
        println!("check-prose-enum: incomplete prose enumeration of a governed set — naming a subset drifts silently when the set grows:");
        println!();
        for l in &sink.out {
            println!("{}", l);
        }
        println!("  help: cite the owning set by name, or complete the enumeration — never trim to a silent subset. A genuinely illustrative list marks itself ('e.g.', 'such as', 'among them') or is partitive ('some of the set'); a legitimately partial site takes a 'prose-enum-exempt: <reason>' comment on the line or the one above");
        return Ok(1);
    }
    println!("PROSE-ENUM: clean ({} manifest file(s); every prose paragraph naming 2+ members of a declared set names them all or is exempt)", manifests.len());
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-prose-enum — word-bounded member presence, so a member
// name inside a longer token is not a mention of it
fn present(low: &[u8], m: &[u8]) -> Option<usize> {
    if m.is_empty() || m.len() > low.len() {
        return None;
    }
    let edge = |c: u8| c.is_ascii_alphanumeric() || c == b'_' || c == b'-';
    for p in 0..=low.len() - m.len() {
        if &low[p..p + m.len()] != m {
            continue;
        }
        let bc = if p > 0 { low[p - 1] } else { b' ' };
        let ac = if p + m.len() < low.len() {
            low[p + m.len()]
        } else {
            b' '
        };
        if !edge(bc) && !edge(ac) {
            return Some(p);
        }
    }
    None
}

// spec: canon-kit/SPEC.md §check-prose-enum — the delimited-adjacency test that separates a
// hand list from scattered mentions
fn adjacent(gap: &[u8]) -> bool {
    const PUNCT: &[u8] = b"][ \t,/:().`|";
    if gap.len() <= 8 && gap.iter().all(|c| PUNCT.contains(c)) {
        return true;
    }
    if gap.len() <= 16 {
        for w in [&b"and"[..], &b"or"[..]] {
            let mut i = 0usize;
            while i + w.len() <= gap.len() {
                if &gap[i..i + w.len()] == w
                    && gap[..i].iter().all(|c| !c.is_ascii_alphabetic())
                    && gap[i + w.len()..].iter().all(|c| !c.is_ascii_alphabetic())
                {
                    return true;
                }
                i += 1;
            }
        }
    }
    false
}

fn subset_marked(low: &[u8]) -> bool {
    [&b"e.g."[..], &b"such as"[..], &b"among them"[..]]
        .iter()
        .any(|n| contains(low, n))
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    needle.len() <= hay.len() && (0..=hay.len() - needle.len()).any(|i| &hay[i..i + needle.len()] == needle)
}

// spec: canon-kit/SPEC.md §check-prose-enum — a partitive lead-in makes a list explicitly
// partial, which is the second exempt escape beside the illustrative markers above
fn partitive(pre: &[u8]) -> bool {
    let end = rtrim_space(pre);
    for w in [
        &b"one"[..],
        &b"some"[..],
        &b"several"[..],
        &b"any"[..],
        &b"each"[..],
        &b"few"[..],
        &b"many"[..],
    ] {
        for i in 0..pre.len() {
            if !pre[i..].starts_with(w) {
                continue;
            }
            let mut j = i + w.len();
            let sp = j;
            while j < pre.len() && is_space(pre[j]) {
                j += 1;
            }
            if j == sp {
                continue;
            }
            if pre[j..].starts_with(b"of") && rtrim_space(&pre[j + 2..]) == 0 {
                return true;
            }
        }
    }
    for i in 0..pre.len() {
        if !pre[i..].starts_with(b"of") {
            continue;
        }
        let mut j = i + 2;
        let sp = j;
        while j < pre.len() && is_space(pre[j]) {
            j += 1;
        }
        if j == sp {
            continue;
        }
        let mut k = j;
        if pre[k..].starts_with(b"the") {
            let mut t = k + 3;
            let tsp = t;
            while t < pre.len() && is_space(pre[t]) {
                t += 1;
            }
            if t > tsp {
                k = t;
            }
        }
        for w in [&b"set"[..], &b"these"[..], &b"those"[..], &b"them"[..]] {
            if pre[k..].starts_with(w) && rtrim_space(&pre[k + w.len()..]) == 0 {
                return true;
            }
        }
    }
    let _ = end;
    false
}

fn rtrim_space(b: &[u8]) -> usize {
    let mut e = b.len();
    while e > 0 && is_space(b[e - 1]) {
        e -= 1;
    }
    e
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\x0b' | b'\x0c' | b'\r' | b'\n')
}
