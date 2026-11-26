// spec: gate-sdk/SPEC.md §The since-base corpus — the merge base a since-base reader compares the
// working tree against, the base ref's own commit beside it, and every read it makes of either
use crate::exit::Fault;
use crate::{listing, proc, programs, walk};

pub const KNOB: &str = "GATE_SDK_BASE_REF";

pub struct Base {
    pub top: String,
    pub commit: String,
    pub tip: String,
}

// spec: gate-sdk/SPEC.md §The since-base corpus — one changed path's hunks: the merge base's line
// ranges it removed and the working tree's it added, each `[first, past)`
#[derive(Debug, Default, PartialEq)]
pub struct Hunks {
    pub removed: Vec<(usize, usize)>,
    pub added: Vec<(usize, usize)>,
}

pub fn within(ranges: &[(usize, usize)], line: usize) -> bool {
    ranges.iter().any(|&(a, b)| line >= a && line < b)
}

fn git(top: &str, args: &[&str]) -> Result<proc::Completed, Fault> {
    let mut argv = vec!["-C", top];
    argv.extend(args);
    proc::run(&programs::GIT, &argv).map_err(Fault::tool)
}

fn failed(c: &proc::Completed, what: &str) -> Fault {
    Fault::tool(c.failure_report().map_or_else(|| what.to_string(), |r| format!("{}: {}", what, r)))
}

fn commit_of(top: &str, rev: &str) -> Result<Option<String>, Fault> {
    let c = git(top, &["rev-parse", "--verify", "--quiet", &format!("{}^{{commit}}", rev)])?;
    Ok(c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()).filter(|s| !s.is_empty()))
}

// spec: gate-sdk/SPEC.md §The since-base corpus — `--base` else the knob; a ref naming no commit or
// sharing no history with `HEAD` is the caller's to fix, so both refuse
pub fn resolve(flag: Option<&str>) -> Result<Base, Fault> {
    let top = walk::toplevel_opt()
        .map_err(Fault::tool)?
        .ok_or_else(|| Fault::refused("not a git repository — there is no merge base to compare the working tree against"))?;
    let rev = match flag {
        Some(r) => r.to_string(),
        None => walk::knob_scalar(KNOB).map_err(Fault::refused)?,
    };
    let refuse = |what: &str| Fault::refused(format!("base '{}' {} — pass --base <ref> or set {}", rev, what, KNOB));
    let named = if rev.is_empty() || rev.starts_with('-') { None } else { commit_of(&top, &rev)? };
    let Some(commit) = named else {
        return Err(refuse("names no commit"));
    };
    let Some(head) = commit_of(&top, "HEAD")? else {
        return Err(refuse("shares no merge base with HEAD, which names no commit"));
    };
    let c = git(&top, &["merge-base", &commit, &head])?;
    match (c.stdout(), c.code()) {
        (Some(o), _) => Ok(Base { commit: String::from_utf8_lossy(o).trim().to_string(), tip: commit, top }),
        (None, Some(1)) => Err(refuse("shares no merge base with HEAD")),
        (None, _) => Err(failed(&c, "git merge-base failed")),
    }
}

impl Base {
    pub fn short(&self) -> &str {
        self.commit.get(..12).unwrap_or(&self.commit)
    }

    fn names(&self, args: &[&str]) -> Result<Vec<String>, Fault> {
        listing::names(Some(&self.top), args).map_err(|e| Fault::tool(e.text_or(|code| crate::fresh::fail_closed("git", code))))
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — the tracked paths whose working-tree content
    // differs from the merge base's, a rename read as a delete and an add
    pub fn changed(&self, under: &[&str]) -> Result<Vec<String>, Fault> {
        let mut args = vec!["diff", "--name-only", "-z", "--no-renames", "--no-ext-diff", &self.commit, "--"];
        args.extend(under);
        self.names(&args)
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — the merge base's files under a directory
    pub fn files_at_base(&self, dir: &str) -> Result<Vec<String>, Fault> {
        self.names(&["ls-tree", "-r", "-z", "--name-only", &self.commit, "--", dir])
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — the base ref's own commit's files under a directory
    pub fn files_at_tip(&self, dir: &str) -> Result<Vec<String>, Fault> {
        self.names(&["ls-tree", "-r", "-z", "--name-only", &self.tip, "--", dir])
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — untracked files git does not ignore: the
    // working tree's, so a file not yet added counts
    pub fn untracked(&self) -> Result<Vec<String>, Fault> {
        self.names(&["ls-files", "-z", "-o", "--exclude-standard", "--"])
    }

    pub fn tracked(&self) -> Result<Vec<String>, Fault> {
        self.names(&["ls-files", "-z", "--"])
    }

    pub fn blob(&self, path: &str) -> Result<Vec<u8>, Fault> {
        let c = git(&self.top, &["cat-file", "blob", &format!("{}:{}", self.commit, path)])?;
        c.stdout().map(<[u8]>::to_vec).ok_or_else(|| failed(&c, &format!("cannot read {} at the merge base", path)))
    }

    pub fn hunks(&self, path: &str) -> Result<Hunks, Fault> {
        let c = git(
            &self.top,
            &["diff", "-U0", "--no-color", "--no-ext-diff", "--no-textconv", "--no-renames", &self.commit, "--", path],
        )?;
        let out = c.stdout().ok_or_else(|| failed(&c, &format!("git diff of {} failed", path)))?;
        parse_hunks(&String::from_utf8_lossy(out)).ok_or_else(|| Fault::tool(format!("git diff of {} printed a hunk header it cannot parse", path)))
    }

    pub fn path(&self, rel: &str) -> std::path::PathBuf {
        std::path::Path::new(&self.top).join(rel)
    }
}

fn range(spec: &str) -> Option<(usize, usize)> {
    let (first, n) = match spec.split_once(',') {
        Some((a, b)) => (a.parse().ok()?, b.parse().ok()?),
        None => (spec.parse().ok()?, 1),
    };
    Some((first, first + n))
}

// spec: gate-sdk/SPEC.md §The since-base corpus — a zero-context diff's `@@ -a,b +c,d @@` headers;
// a zero count is an empty range
pub fn parse_hunks(diff: &str) -> Option<Hunks> {
    let mut h = Hunks::default();
    for line in diff.lines().filter(|l| l.starts_with("@@ ")) {
        let mut words = line.split(' ').skip(1);
        let (old, new) = (words.next()?.strip_prefix('-')?, words.next()?.strip_prefix('+')?);
        let (old, new) = (range(old)?, range(new)?);
        if old.1 > old.0 {
            h.removed.push(old);
        }
        if new.1 > new.0 {
            h.added.push(new);
        }
    }
    Some(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The since-base corpus — a count of zero names no line, an absent
    // count names one
    #[test]
    fn a_hunk_header_reads_as_its_two_ranges() {
        let diff = "diff --git a/x b/x\n--- a/x\n+++ b/x\n@@ -3 +3,2 @@ fn x\n-a\n+b\n+c\n@@ -9,2 +10,0 @@\n-d\n-e\n@@ -20,0 +19 @@\n+f\n";
        let h = parse_hunks(diff).expect("parses");
        assert_eq!(h.removed, [(3, 4), (9, 11)]);
        assert_eq!(h.added, [(3, 5), (19, 20)]);
        assert!(within(&h.added, 4) && !within(&h.added, 5));
        assert!(parse_hunks("@@ -x +1 @@\n").is_none());
    }
}
