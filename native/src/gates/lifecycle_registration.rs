// spec: lifecycle-kit/SPEC.md §check-lifecycle-registration — the always-loaded agent file
// carries a lifecycle-kit marker block whose content byte-matches the block regenerated from
// the live stage machine, fail-closed when the target or a marker is missing
use crate::stages;
use crate::walk;
use std::path::Path;

const BEGIN: &str = "<!-- lifecycle-kit:begin -->";
const END: &str = "<!-- lifecycle-kit:end -->";

// spec: lifecycle-kit/SPEC.md §check-lifecycle-registration — the stale-block report is
// `diff`'s normal format; the alignment is an LCS walk rather than a spawn, so the gate
// reaches no subprocess to render it
fn normal_diff(a: &[&str], b: &[&str]) -> Vec<String> {
    let (n, m) = (a.len(), b.len());
    let mut lcs = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    let range = |lo: usize, hi: usize| {
        if lo == hi {
            format!("{}", lo)
        } else {
            format!("{},{}", lo, hi)
        }
    };
    let mut out: Vec<String> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < n || j < m {
        if i < n && j < m && a[i] == b[j] {
            i += 1;
            j += 1;
            continue;
        }
        let (di, dj) = (i, j);
        while i < n && (j >= m || a[i] != b[j]) && (j >= m || lcs[i + 1][j] >= lcs[i][j + 1]) {
            i += 1;
        }
        while j < m && (i >= n || a[i] != b[j]) {
            j += 1;
        }
        let (dels, adds) = (i - di, j - dj);
        if dels > 0 && adds > 0 {
            out.push(format!(
                "{}c{}",
                range(di + 1, i),
                range(dj + 1, j)
            ));
        } else if dels > 0 {
            out.push(format!("{}d{}", range(di + 1, i), dj));
        } else {
            out.push(format!("{}a{}", di, range(dj + 1, j)));
        }
        for l in &a[di..i] {
            out.push(format!("< {}", l));
        }
        if dels > 0 && adds > 0 {
            out.push("---".to_string());
        }
        for l in &b[dj..j] {
            out.push(format!("> {}", l));
        }
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    let agent = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("LIFECYCLE_KIT_AGENT_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-lifecycle-registration: {}", e);
                return 2;
            }
        },
    };
    if !Path::new(&agent).is_file() {
        eprintln!(
            "check-lifecycle-registration: agent file not found: {}",
            agent
        );
        return 2;
    }
    let text = match std::fs::read(&agent) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            eprintln!("check-lifecycle-registration: cannot read {}: {}", agent, e);
            return 2;
        }
    };

    if !text.contains(BEGIN) {
        println!(
            "check-lifecycle-registration: no lifecycle-kit registration block in {}",
            agent
        );
        println!("  help: install the resident registration block into the always-loaded agent file —");
        println!("        bash lifecycle-kit/bin/install-lifecycle.sh — so a session that loads it");
        println!("        is pointed at the stage machine. Override the path with LIFECYCLE_KIT_AGENT_FILE.");
        return 1;
    }
    if !text.contains(END) {
        eprintln!(
            "check-lifecycle-registration: begin marker present but end marker missing in {} — the block bounds are unreadable",
            agent
        );
        return 2;
    }

    let mut present: Vec<&str> = Vec::new();
    let mut inb = false;
    for line in text.lines() {
        if line == BEGIN {
            inb = true;
            continue;
        }
        if line == END {
            inb = false;
            continue;
        }
        if inb {
            present.push(line);
        }
    }

    let block = match stages::registration_block() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("check-lifecycle-registration: {}", e);
            return 2;
        }
    };
    let expected: Vec<&str> = block.lines().collect();
    let stage_count = match stages::stages() {
        Ok(v) => v.len(),
        Err(e) => {
            eprintln!("check-lifecycle-registration: {}", e);
            return 2;
        }
    };

    if present != expected {
        println!(
            "check-lifecycle-registration: the registration block in {} is stale — it does not match the block derived from the live stage machine:",
            agent
        );
        for l in normal_diff(&expected, &present) {
            println!("  {}", l);
        }
        println!("  help: a reshaped stage machine (LIFECYCLE_KIT_STAGES / LIFECYCLE_KIT_QUEUE_FILE) or a");
        println!("        hand-edited block staled the registration — regenerate it in place:");
        println!("        bash lifecycle-kit/bin/install-lifecycle.sh");
        return 1;
    }

    println!(
        "LIFECYCLE-REGISTRATION: clean ({} carries the lifecycle-kit registration block in byte-lockstep with the derived stage machine; {} stage(s))",
        agent, stage_count
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §check-lifecycle-registration — the three hunk kinds in
    // `diff`'s normal format, held to the format rather than to this gate's one live shape
    #[test]
    fn the_three_hunk_kinds_render_in_diff_normal_format() {
        assert_eq!(
            normal_diff(&["a", "b", "c"], &["a", "x", "c"]),
            vec!["2c2", "< b", "---", "> x"]
        );
        assert_eq!(normal_diff(&["a", "b"], &["a"]), vec!["2d1", "< b"]);
        assert_eq!(normal_diff(&["a"], &["a", "b"]), vec!["1a2", "> b"]);
        assert!(normal_diff(&["a", "b"], &["a", "b"]).is_empty());
    }

    #[test]
    fn a_multi_line_hunk_prints_a_range_on_each_side() {
        assert_eq!(
            normal_diff(&["a", "b", "c", "d"], &["a", "d"]),
            vec!["2,3d1", "< b", "< c"]
        );
        assert_eq!(
            normal_diff(&["a", "d"], &["a", "b", "c", "d"]),
            vec!["1a2,3", "> b", "> c"]
        );
    }
}
