// spec: gate-sdk/SPEC.md §projection-witness — the differential witness of a projection's declared
// trigger: perturb a history-bearing tracked-tree scratch outside each declaring gate's trigger set
// and red where the gate goes red, reporting a coupled file whose edit moves nothing
use crate::emit::csmoke;
use crate::proc;
use crate::programs;
use crate::registry;
use crate::walk;
use std::collections::BTreeMap;
use std::path::Path;

pub const KNOBS: &[&str] = &["GATE_SDK_GATES_DIR", "GATE_SDK_KIT_DIRS", "GATE_SDK_NATIVE_BIN"];

const NAME: &str = "projection-witness";
const USAGE: &str = "usage: --projection-witness";
const MARK: &str = "projection-witness";

struct Member {
    name: String,
    everything: bool,
    globs: Vec<String>,
}

impl Member {
    fn triggers(&self, path: &str) -> bool {
        self.everything || self.globs.iter().any(|g| registry::couple_matches(path, g))
    }
}

// spec: gate-sdk/SPEC.md §projection-witness — one member's verdict in the scratch: green, and the
// first non-blank line it printed
type Verdict = Result<(bool, String), String>;

#[derive(Default)]
struct Report {
    classes: usize,
    perturbations: usize,
    findings: Vec<String>,
    inert: Vec<String>,
}

pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!("{}: unknown option: {}; {}", NAME, a, USAGE);
        return 2;
    }
    let outcome = members().and_then(|members| {
        if members.is_empty() {
            return Ok(None);
        }
        let exe = std::env::current_exe()
            .map_err(|e| format!("cannot locate the running binary: {}", e))?
            .display()
            .to_string();
        let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")?;
        let place = |root: &Path| -> Result<(), String> {
            let r = root.display().to_string();
            let placed = walk::abs_against(&r, &bin);
            if walk::under(&r, &placed) {
                csmoke::place_artifact(&exe, &placed)?;
            }
            Ok(())
        };
        let gate = |dir: &Path, name: &str| spawn_gate(&exe, dir, name);
        witness_in(&members, Path::new("."), &place, &gate).map(|r| Some((members.len(), r)))
    });
    match outcome {
        Err(e) => {
            println!("WITNESS: FAIL(env) {}", e);
            2
        }
        Ok(None) => {
            println!(
                "PROJECTION-WITNESS: clean (0 projections; no registered gate declares '{}')",
                registry::PROJECTION
            );
            0
        }
        Ok(Some((n, r))) => {
            for l in &r.inert {
                println!("{}", l);
            }
            if !r.findings.is_empty() {
                for f in &r.findings {
                    println!("{}", f);
                }
                println!("  help: a projection read a path its gate's trigger does not declare, so the hook");
                println!("        never re-runs the gate when that path changes; add the path to the gate's");
                println!("        couples= (and trigger= where it has one), then regenerate the hooks");
                println!("        (gate-sdk/SPEC.md §projection-witness).");
                return 1;
            }
            println!(
                "PROJECTION-WITNESS: clean ({} projection(s), {} class(es), {} perturbation(s), {} coupled but inert; no gate went red outside its declared trigger)",
                n,
                r.classes,
                r.perturbations,
                r.inert.len()
            );
            0
        }
    }
}

// spec: gate-sdk/SPEC.md §projection-witness — the members and their expanded trigger sets, the one
// `run-gates --for` computes: the effective field kit-expanded, then the derived couples
fn members() -> Result<Vec<Member>, String> {
    let gates_dir = crate::knobs::gates_dir();
    let kit_roots = walk::kit_roots()?;
    let resolve_dirs = registry::resolve_dirs(&gates_dir, &kit_roots);
    let mut out: Vec<Member> = Vec::new();
    for d in crate::gates::projection_roster::declaring_gates()? {
        let src = registry::resolve(&d.name, &resolve_dirs)
            .ok_or_else(|| format!("cannot resolve {}", d.name))?;
        let body = std::fs::read_to_string(&src).map_err(|e| format!("cannot read {}: {}", src, e))?;
        let fields = registry::manifest_line(&body).map(registry::manifest_fields).unwrap_or_default();
        let trigger = registry::expand_couples(&registry::effective_trigger(&fields), &kit_roots)
            .map_err(|e| format!("cannot expand {}'s trigger: {}", d.name, e))?;
        let mut globs: Vec<String> = trigger.split(',').filter(|g| !g.is_empty()).map(String::from).collect();
        globs.extend(registry::derived_couples(&d.name, &resolve_dirs)?);
        out.push(Member {
            everything: trigger == "*",
            name: d.name,
            globs,
        });
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §projection-witness — a member spawned by name, the arm's own executable
// with the scratch as its working directory; the locator is blanked so the child resolves the
// scratch's own gate-sdk root rather than an absolute one this process inherited
fn spawn_gate(exe: &str, dir: &Path, name: &str) -> Verdict {
    let done = proc::run_merged_in(
        &programs::CHECKWRIGHT_GATES.at(exe.to_string()),
        &[name],
        &[("GATE_SDK_ROOT".to_string(), String::new())],
        Some(dir),
    )?;
    let text = String::from_utf8_lossy(done.output()).into_owned();
    let first = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("").trim().to_string();
    Ok((done.succeeded(), first))
}

// spec: gate-sdk/SPEC.md §projection-witness — a class is the tracked set's (first path segment,
// extension) pair, its members in byte order
fn classes(files: &[String]) -> BTreeMap<(String, String), Vec<String>> {
    let mut out: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for f in files {
        let seg = f.split_once('/').map(|(s, _)| s).unwrap_or("").to_string();
        let base = f.rsplit('/').next().unwrap_or(f);
        let ext = base.rsplit_once('.').map(|(_, e)| e).unwrap_or("").to_string();
        out.entry((seg, ext)).or_default().push(f.clone());
    }
    for v in out.values_mut() {
        v.sort();
    }
    out
}

fn created_path(seg: &str, ext: &str) -> String {
    let leaf = if ext.is_empty() {
        format!(".{}", MARK)
    } else {
        format!(".{}.{}", MARK, ext)
    };
    if seg.is_empty() {
        leaf
    } else {
        format!("{}/{}", seg, leaf)
    }
}

fn git(dir: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let d = dir.display().to_string();
    let mut argv: Vec<&str> = vec!["-C", &d];
    argv.extend_from_slice(args);
    let done = proc::run(&programs::GIT, &argv)?;
    if let Some(r) = done.failure_report() {
        return Err(format!("git {} failed in the scratch — {}", args.join(" "), r));
    }
    Ok(done.stdout().unwrap_or(&[]).to_vec())
}

// spec: gate-sdk/SPEC.md §projection-witness — the edit appends one line and the file's bytes are
// restored after, which is the whole reset for a path that is only edited
fn edit(dir: &Path, path: &str, gate: &dyn Fn(&Path) -> Verdict) -> Verdict {
    let p = dir.join(path);
    let before = std::fs::read(&p).map_err(|e| format!("cannot read {}: {}", path, e))?;
    let mut after = before.clone();
    if !after.is_empty() && !after.ends_with(b"\n") {
        after.push(b'\n');
    }
    after.extend_from_slice(format!("{}\n", MARK).as_bytes());
    std::fs::write(&p, &after).map_err(|e| format!("cannot write {}: {}", path, e))?;
    let verdict = gate(dir);
    std::fs::write(&p, &before).map_err(|e| format!("cannot restore {}: {}", path, e))?;
    verdict
}

// spec: gate-sdk/SPEC.md §projection-witness — the create stages a new file, and is reset by
// unstaging and removing it
fn create(dir: &Path, path: &str, gate: &dyn Fn(&Path) -> Verdict) -> Verdict {
    let p = dir.join(path);
    std::fs::write(&p, format!("{}\n", MARK)).map_err(|e| format!("cannot write {}: {}", path, e))?;
    git(dir, &["add", "--", path])?;
    let verdict = gate(dir);
    git(dir, &["rm", "-q", "--cached", "--", path])?;
    std::fs::remove_file(&p).map_err(|e| format!("cannot remove {}: {}", path, e))?;
    verdict
}

fn regular(dir: &Path, path: &str) -> bool {
    std::fs::symlink_metadata(dir.join(path))
        .map(|m| m.is_file())
        .unwrap_or(false)
}

// spec: gate-sdk/SPEC.md §projection-witness — the witness over one source tree: the scratch, the
// baseline, then per member and class the out-of-trigger edit and create and the in-trigger edit
fn witness_in(
    members: &[Member],
    source: &Path,
    place: &dyn Fn(&Path) -> Result<(), String>,
    gate: &dyn Fn(&Path, &str) -> Verdict,
) -> Result<Report, String> {
    let guard = csmoke::tracked_history_scratch(source, &crate::gates::fence_run::scratch_base(), NAME)?;
    let dir = guard.dir.clone().ok_or("the scratch was not created")?;
    place(&dir)?;
    for m in members {
        let (green, first) = gate(&dir, &m.name)?;
        if !green {
            return Err(format!(
                "{} is red at baseline in the scratch, so nothing can be witnessed: {}",
                m.name, first
            ));
        }
    }
    let listed = git(&dir, &["ls-files", "-z"])?;
    let files: Vec<String> = String::from_utf8_lossy(&listed)
        .split('\0')
        .filter(|f| !f.is_empty())
        .map(String::from)
        .collect();
    let classes = classes(&files);
    let mut r = Report {
        classes: classes.len(),
        ..Report::default()
    };
    for m in members {
        let run = |d: &Path| gate(d, &m.name);
        for ((seg, ext), paths) in &classes {
            if let Some(p) = paths.iter().find(|p| !m.triggers(p) && regular(&dir, p)) {
                r.perturbations += 1;
                let (green, first) = edit(&dir, p, &run)?;
                if !green {
                    r.findings.push(finding(&m.name, "an edit", p, &first));
                }
            }
            let created = created_path(seg, ext);
            if !m.triggers(&created) && !dir.join(&created).exists() {
                r.perturbations += 1;
                let (green, first) = create(&dir, &created, &run)?;
                if !green {
                    r.findings.push(finding(&m.name, "a create", &created, &first));
                }
            }
            // spec: gate-sdk/SPEC.md §projection-witness — the positive direction is a report: a
            // coupled file whose edit leaves the gate green is sanctioned over-approximation
            if let Some(p) = paths.iter().find(|p| m.triggers(p) && regular(&dir, p)) {
                r.perturbations += 1;
                if edit(&dir, p, &run)?.0 {
                    r.inert.push(format!("coupled but inert: {} {}", m.name, p));
                }
            }
        }
    }
    drop(guard);
    Ok(r)
}

fn finding(gate: &str, what: &str, path: &str, first: &str) -> String {
    format!(
        "WITNESS: FAIL {} went red on {} of {}, outside its declared trigger: {}",
        gate, what, path, first
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §projection-witness — the class key is the first segment and the
    // extension, a root file's segment empty, and members sort byte-wise
    #[test]
    fn the_tracked_set_groups_by_segment_and_extension() {
        let files: Vec<String> = ["docs/b.md", "docs/a.md", "CLAUDE.md", "docs/x.html", "Makefile"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let c = classes(&files);
        assert_eq!(c[&("docs".to_string(), "md".to_string())], vec!["docs/a.md", "docs/b.md"]);
        assert_eq!(c[&(String::new(), "md".to_string())], vec!["CLAUDE.md"]);
        assert_eq!(c[&(String::new(), String::new())], vec!["Makefile"]);
        assert_eq!(created_path("docs", "md"), "docs/.projection-witness.md");
        assert_eq!(created_path("", ""), ".projection-witness");
    }

    fn sh_git(dir: &Path, args: &[&str]) {
        let d = dir.display().to_string();
        let mut argv: Vec<&str> = vec!["-C", &d, "-c", "user.email=t@example.invalid", "-c", "user.name=t"];
        argv.extend_from_slice(args);
        let done = proc::run(&programs::GIT, &argv).expect("git runs");
        assert!(done.failure_report().is_none(), "git {:?} failed", args);
    }

    // spec: gate-sdk/SPEC.md §projection-witness — witnessed on a known defect: a toy projection
    // whose emitter reads `in/source.txt` while its declared trigger is `out/*` reds naming that
    // file, the create beside it stays green, and its own output is live rather than inert
    #[test]
    fn a_projection_reading_an_undeclared_file_is_a_finding() {
        let tree = std::env::temp_dir().join(format!("projection-witness-test.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tree);
        std::fs::create_dir_all(tree.join("in")).expect("mkdir");
        std::fs::create_dir_all(tree.join("out")).expect("mkdir");
        std::fs::write(tree.join("in/source.txt"), "alpha\n").expect("write");
        std::fs::write(tree.join("out/proj.txt"), "alpha\n").expect("write");
        std::fs::write(tree.join("notes.md"), "unrelated\n").expect("write");
        sh_git(&tree, &["init", "-q"]);
        sh_git(&tree, &["add", "-A"]);
        sh_git(&tree, &["commit", "-q", "-m", "seed"]);
        let members = vec![Member {
            name: "check-toy-fresh".to_string(),
            everything: false,
            globs: vec!["out/*".to_string()],
        }];
        let toy = |dir: &Path, _: &str| -> Verdict {
            let a = std::fs::read(dir.join("in/source.txt")).map_err(|e| e.to_string())?;
            let b = std::fs::read(dir.join("out/proj.txt")).map_err(|e| e.to_string())?;
            Ok((a == b, "check-toy-fresh: out/proj.txt is stale".to_string()))
        };
        let r = witness_in(&members, &tree, &|_| Ok(()), &toy).expect("the witness runs");
        let _ = std::fs::remove_dir_all(&tree);
        assert_eq!(r.findings.len(), 1, "{:?}", r.findings);
        assert!(r.findings[0].contains("an edit of in/source.txt"), "{}", r.findings[0]);
        assert!(r.inert.is_empty(), "{:?}", r.inert);
        assert_eq!(r.classes, 3);
    }
}
