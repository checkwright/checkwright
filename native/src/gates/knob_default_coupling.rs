// spec: canon-kit/SPEC.md §check-knob-default-coupling — every literal kit-knob default in kit
// source agrees across its sites and with the default the owning SPEC states
use crate::spec;
use crate::walk;
use std::collections::BTreeMap;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-knob-default-coupling: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — the knob prefix is derived, never
// listed: one SCREAMING_SNAKE form per kit root (dir uppercased, hyphens to underscores), so the
// gate ships no term list and the provenance seam holds
fn prefix_pairs() -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for kr in walk::kit_roots_rel()? {
        let kr = kr.trim_end_matches('/');
        if kr.is_empty() {
            continue;
        }
        let base = kr.rsplit('/').next().unwrap_or(kr);
        out.push((
            format!("{}_", base.to_ascii_uppercase().replace('-', "_")),
            kr.to_string(),
        ));
    }
    Ok(out)
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — one record per default site, class
// literal or skip
struct Record {
    literal: bool,
    knob: String,
    kit: String,
    val: String,
    file: String,
    lno: usize,
}

fn knob_owner(pairs: &[(String, String)], knob: &str) -> String {
    for (p, kit) in pairs {
        if knob.starts_with(p.as_str()) {
            return kit.clone();
        }
    }
    String::new()
}

fn is_knobname(pairs: &[(String, String)], tok: &str) -> bool {
    pairs.iter().any(|(p, _)| tok.starts_with(p.as_str()))
}

fn classify_literal(v: &str) -> bool {
    if v.is_empty() || v.starts_with('(') {
        return false;
    }
    !v.contains('$') && !v.contains('`')
}

fn strip_quotes(v: &str) -> String {
    let b = v.as_bytes();
    if b.len() >= 2 && ((b[0] == b'"' && b[b.len() - 1] == b'"') || (b[0] == b'\'' && b[b.len() - 1] == b'\''))
    {
        return v[1..v.len() - 1].to_string();
    }
    v.to_string()
}

fn ident_len(b: &[u8], at: usize) -> usize {
    if at >= b.len() || !(b[at].is_ascii_alphabetic() || b[at] == b'_') {
        return 0;
    }
    let mut j = at + 1;
    while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
        j += 1;
    }
    j - at
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — idiom 1, the `${PREFIX_KNOB:-value}`
// fallback expansion, taken left to right across the line
fn fallback_sites(line: &str) -> Vec<(String, String)> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 1 < b.len() {
        if b[i] != b'$' || b[i + 1] != b'{' {
            i += 1;
            continue;
        }
        let name_at = i + 2;
        let nl = ident_len(b, name_at);
        if nl == 0 || name_at + nl + 1 >= b.len() || b[name_at + nl] != b':' || b[name_at + nl + 1] != b'-'
        {
            i += 1;
            continue;
        }
        let val_at = name_at + nl + 2;
        let close = match b[val_at..].iter().position(|&c| c == b'}') {
            Some(off) => val_at + off,
            None => {
                i += 1;
                continue;
            }
        };
        out.push((
            String::from_utf8_lossy(&b[name_at..name_at + nl]).into_owned(),
            String::from_utf8_lossy(&b[val_at..close]).into_owned(),
        ));
        i = close + 1;
    }
    out
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — idiom 2, the guarded assignment that is
// the dominant form in the kits' libraries: `[[ -v KNOB ]] || KNOB=value`, or the `declare -p`
// spelling an array default takes
fn guarded_site(line: &str) -> Option<(String, String)> {
    let b = line.as_bytes();
    let mut guarded = false;
    let mut last: Option<usize> = None;
    let mut i = 0usize;
    while i + 1 < b.len() {
        if b[i] == b'|' && b[i + 1] == b'|' {
            let mut after = i + 2;
            while after < b.len() && (b[after] == b' ' || b[after] == b'\t') {
                after += 1;
            }
            // spec: canon-kit/SPEC.md §check-knob-default-coupling — the shell form's guard is a
            // whole-line test that any `]` or `&>/dev/null` precedes a `||` and an assignment
            let mut before = i;
            while before > 0 && (b[before - 1] == b' ' || b[before - 1] == b'\t') {
                before -= 1;
            }
            let nl = ident_len(b, after);
            if nl > 0
                && after + nl < b.len()
                && b[after + nl] == b'='
                && ((before > 0 && b[before - 1] == b']') || b[..before].ends_with(b"&>/dev/null"))
            {
                guarded = true;
            }
            // spec: canon-kit/SPEC.md §check-knob-default-coupling — and its extraction strips
            // through the *last* `||` on the line, whichever one that is
            last = Some(after);
        }
        i += 1;
    }
    if !guarded {
        return None;
    }
    let after = last?;
    let nl = ident_len(b, after);
    if nl == 0 || after + nl >= b.len() || b[after + nl] != b'=' {
        return None;
    }
    let knob = String::from_utf8_lossy(&b[after..after + nl]).into_owned();
    let val = String::from_utf8_lossy(&b[after + nl + 1..])
        .trim_end_matches([' ', '\t'])
        .to_string();
    Some((knob, strip_quotes(&val)))
}

fn extract(pairs: &[(String, String)], file: &str, text: &str) -> Vec<Record> {
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let lno = idx + 1;
        let mut sites = fallback_sites(line);
        if let Some(s) = guarded_site(line) {
            sites.push(s);
        }
        for (knob, val) in sites {
            let kit = knob_owner(pairs, &knob);
            if kit.is_empty() {
                continue;
            }
            out.push(Record {
                literal: classify_literal(&val),
                knob,
                kit,
                val,
                file: file.to_string(),
                lno,
            });
        }
    }
    out
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — reduce every `${VAR:-tail}` deferral
// expression in a window to its tail literal, so a SPEC default stated as the same deferral the
// source uses compares tail-to-tail rather than expression-to-literal
fn reduce_deferrals(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::new();
    let mut i = 0usize;
    let mut copied = 0usize;
    while i + 1 < b.len() {
        if b[i] != b'$' || b[i + 1] != b'{' {
            i += 1;
            continue;
        }
        let name_at = i + 2;
        let nl = ident_len(b, name_at);
        if nl == 0 || name_at + nl + 1 >= b.len() || b[name_at + nl] != b':' || b[name_at + nl + 1] != b'-'
        {
            i += 1;
            continue;
        }
        let val_at = name_at + nl + 2;
        let close = match b[val_at..].iter().position(|&c| c == b'}') {
            Some(off) => val_at + off,
            None => {
                i += 1;
                continue;
            }
        };
        out.push_str(&String::from_utf8_lossy(&b[copied..i]));
        out.push_str(&String::from_utf8_lossy(&b[val_at..close]));
        copied = close + 1;
        i = close + 1;
    }
    out.push_str(&String::from_utf8_lossy(&b[copied..]));
    out
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — `v` appears in `win` as a full delimited
// token: bounded on both sides by a non-value character, so a suffix of a longer literal never
// matches
fn fulltoken(win: &str, v: &str) -> bool {
    if v.is_empty() {
        return false;
    }
    let w = win.as_bytes();
    let vb = v.as_bytes();
    let boundary = |c: u8| {
        !(c.is_ascii_alphanumeric() || c == b'_' || c == b'.' || c == b'/' || c == b'*' || c == b'-')
    };
    let mut start = 0usize;
    while start + vb.len() <= w.len() {
        match w[start..].windows(vb.len()).position(|s| s == vb) {
            Some(off) => {
                let at = start + off;
                let before = if at > 0 { w[at - 1] } else { b' ' };
                let after = if at + vb.len() < w.len() {
                    w[at + vb.len()]
                } else {
                    b' '
                };
                if boundary(before) && boundary(after) {
                    return true;
                }
                start = at + vb.len();
            }
            None => return false,
        }
    }
    false
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — truncate at the first kit-knob token, so
// a knob's default window never bleeds into the next knob's statement
fn bound_next_knob(pairs: &[(String, String)], s: &str) -> String {
    let mut best = s.len();
    for (p, _) in pairs {
        if let Some(pos) = s.find(p.as_str()) {
            if pos < best {
                best = pos;
            }
        }
    }
    s[..best].to_string()
}

enum Verdict {
    Found,
    Disagree,
    Described,
    Absent,
}

// spec: canon-kit/SPEC.md §check-knob-default-coupling — assertion 2 over one kit's SPEC, read as
// one blob so a default statement wrapped across lines still binds its knob
fn spec_verdict(pairs: &[(String, String)], blob: &[char], knob: &str, want: &str) -> Verdict {
    let k: Vec<char> = knob.chars().collect();
    let grammar = spec::DefaultGrammar {
        is_knobname: &|t: &str| is_knobname(pairs, t),
    };
    let mut any_disagree = false;
    let mut any_described = false;
    let mut start = 0usize;
    while start + k.len() <= blob.len() {
        let hit = match (start..=blob.len() - k.len()).find(|&i| blob[i..i + k.len()] == k[..]) {
            Some(i) => i,
            None => break,
        };
        let after = blob.get(hit + k.len()).copied().unwrap_or(' ');
        if after.is_ascii_alphanumeric() || after == '_' {
            start = hit + k.len();
            continue;
        }
        let end = std::cmp::min(blob.len(), hit + k.len() + 400);
        let win = reduce_deferrals(&blob[hit..end].iter().collect::<String>());
        let low = win.to_lowercase();
        let haskw = low.find("default");
        let mut slit = String::new();
        if let Some(at) = haskw {
            let tail = &win[at + "default".len()..];
            slit = grammar.literal_at(bound_next_knob(pairs, tail).as_bytes());
        }
        if slit == want || (haskw.is_some() && fulltoken(&win, want)) {
            return Verdict::Found;
        }
        if !slit.is_empty() {
            any_disagree = true;
        } else if haskw.is_some() {
            any_described = true;
        }
        start = hit + k.len();
    }
    if any_disagree {
        Verdict::Disagree
    } else if any_described {
        Verdict::Described
    } else {
        Verdict::Absent
    }
}

fn rule(_args: &[String]) -> Result<i32, String> {
    let pairs = prefix_pairs()?;
    if pairs.is_empty() {
        return Err("no kit roots enumerated".to_string());
    }

    let mut sources: Vec<String> = Vec::new();
    for kr in walk::kit_roots_rel()? {
        let kr = kr.trim_end_matches('/');
        if kr.is_empty() {
            continue;
        }
        for p in walk::find_files(Path::new(kr), &["sh"])? {
            let rel = spec::strip_dot_slash(&p.display().to_string());
            if rel.contains("/templates/") {
                continue;
            }
            sources.push(rel);
        }
    }
    sources.sort();
    sources.dedup();
    if sources.is_empty() {
        println!("KNOB-DEFAULT-COUPLING: clean (0 kit source file(s) found)");
        return Ok(0);
    }

    let mut records: Vec<Record> = Vec::new();
    for f in &sources {
        let text = spec::read_text(Path::new(f))?;
        records.extend(extract(&pairs, f, &text));
    }
    let skipped = records.iter().filter(|r| !r.literal).count();
    let lit_count = records.iter().filter(|r| r.literal).count();

    let mut findings: Vec<String> = Vec::new();
    let mut described = 0usize;

    // spec: canon-kit/SPEC.md §check-knob-default-coupling — assertion 1: every literal site for
    // one knob carries the same literal, else the source disagrees with itself before any SPEC is
    // read (and the knob's assertion-2 check is suppressed)
    let mut first: BTreeMap<String, (String, String, String)> = BTreeMap::new();
    let mut conflict: BTreeMap<String, ()> = BTreeMap::new();
    for r in records.iter().filter(|r| r.literal) {
        match first.get(&r.knob) {
            None => {
                first.insert(
                    r.knob.clone(),
                    (r.val.clone(), format!("{}:{}", r.file, r.lno), r.kit.clone()),
                );
            }
            Some((val, at, _)) if *val != r.val => {
                conflict.insert(r.knob.clone(), ());
                findings.push(format!(
                    "  {}:{}  {} default '{}' disagrees with '{}' at {} — a knob's default has one literal across its sites",
                    r.file, r.lno, r.knob, r.val, val, at
                ));
            }
            _ => {}
        }
    }

    let mut kit_subset: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for (knob, (val, _, kit)) in &first {
        if conflict.contains_key(knob) {
            continue;
        }
        kit_subset
            .entry(kit.clone())
            .or_default()
            .push((knob.clone(), val.clone()));
    }

    let spec_name = spec::spec_name()?;
    for (kit, subset) in &kit_subset {
        let spec_path = format!("{}/{}", kit, spec_name);
        if !Path::new(&spec_path).is_file() {
            for (knob, val) in subset {
                findings.push(format!(
                    "  {}  {} default `{}` has no owning SPEC to state it — the SPEC owns knob defaults",
                    spec_path, knob, val
                ));
            }
            continue;
        }
        let text = spec::read_text(Path::new(&spec_path))?;
        let blob: Vec<char> = text
            .lines()
            .collect::<Vec<&str>>()
            .join(" ")
            .chars()
            .collect();
        for (knob, val) in subset {
            match spec_verdict(&pairs, &blob, knob, val) {
                Verdict::Found => {}
                Verdict::Disagree => findings.push(format!(
                    "  {}  {} — source default `{}` but the SPEC states a different default",
                    spec_path, knob, val
                )),
                Verdict::Absent => findings.push(format!(
                    "  {}  {} — source default `{}` is stated nowhere in the owning SPEC",
                    spec_path, knob, val
                )),
                Verdict::Described => described += 1,
            }
        }
    }

    if !findings.is_empty() {
        println!("check-knob-default-coupling: kit-knob default(s) drift between source sites or from the owning SPEC — a default has one home, and a divergent copy is a silent regression:");
        println!();
        findings.sort();
        for l in &findings {
            println!("{}", l);
        }
        println!("  help: make every fallback site for the knob carry the same literal, and state that literal as the knob's default in the owning kit's SPEC (the default-statement grammar check-knob-citation reads); a computed default belongs in the SPEC as prose, not a coupled literal");
        return Ok(1);
    }

    println!(
        "KNOB-DEFAULT-COUPLING: clean ({} kit source file(s); {} literal default site(s) agree across sites and with the owning SPEC; {} computed/array/empty + {} descriptively-stated default(s) skipped-and-counted)",
        sources.len(),
        lit_count,
        skipped,
        described
    );
    Ok(0)
}
