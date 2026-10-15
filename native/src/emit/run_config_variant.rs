// spec: gate-sdk/SPEC.md §run-config-variant — the battery twice over a history-bearing scratch of
// the tree, as committed and under a variant of the config seam, and the gates the variant moves
use crate::emit::csmoke;
use crate::emit::parse_gates_log::{sections, Section};
use crate::proc;
use crate::programs;
use crate::walk;
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &["GATE_SDK_TMP_DIR", "GATE_SDK_GATES_DIR", "GATE_SDK_NATIVE_BIN"];

const NAME: &str = "run-config-variant";
pub const USAGE: &str = "usage: --run-config-variant <dir> [--keep]";

// spec: gate-sdk/SPEC.md §run-config-variant — one battery run: its exit code and its merged log
type Battery<'a> = &'a dyn Fn(&Path) -> Result<(i32, String), String>;

struct Plan<'a> {
    source: &'a Path,
    rel_cwd: &'a str,
    rel_gates: &'a str,
    files: &'a [PathBuf],
    base: &'a Path,
    keep: bool,
}

struct Compared {
    baseline: String,
    variant: String,
    kept: Option<PathBuf>,
}

pub fn run(args: &[String]) -> i32 {
    let mut keep = false;
    let mut dir: Option<&str> = None;
    for a in args {
        match a.as_str() {
            "--keep" => keep = true,
            s if s.starts_with('-') || dir.is_some() => {
                eprintln!("{}: unexpected argument: {}\n{}", NAME, s, USAGE);
                return 2;
            }
            s => dir = Some(s),
        }
    }
    let Some(dir) = dir else {
        eprintln!("{}: needs a variant <dir>\n{}", NAME, USAGE);
        return 2;
    };
    let files = match variant_files(Path::new(dir)) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };
    for n in notes(&|v| std::env::var(v).ok()) {
        println!("{}", n);
    }
    match live(&files, keep) {
        Ok(c) => {
            let (text, rc) = report(dir, &c.baseline, &c.variant);
            print!("{}", text);
            if let Some(k) = c.kept {
                println!("kept: {}", k.display());
            }
            rc
        }
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §run-config-variant — the variant is a directory of regular files, refused
// before any scratch is built when it is absent, empty or holds anything else
fn variant_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let entries = walk::list_dir(dir).map_err(|e| format!("the variant directory: {}", e))?;
    let mut out = Vec::new();
    for (name, _) in entries {
        let p = dir.join(name);
        let regular = std::fs::symlink_metadata(&p).map(|m| m.is_file()).unwrap_or(false);
        if !regular {
            return Err(format!(
                "{} is not a regular file — a variant holds only the gates-directory files it replaces",
                p.display()
            ));
        }
        out.push(p);
    }
    if out.is_empty() {
        return Err(format!("the variant directory {} is empty, so it replaces nothing", dir.display()));
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §run-config-variant — a kit file locator the caller exported redirects that
// kit's layer away from the gates directory the variant writes, in both runs
fn notes(env: &dyn Fn(&str) -> Option<String>) -> Vec<String> {
    let mut out = Vec::new();
    for kit in crate::knobs::STATIC_KITS {
        for var in [kit.knob_file_var(), kit.local_knob_file_var()] {
            if let Some(v) = env(&var).filter(|v| !v.is_empty()) {
                out.push(format!(
                    "note: {} is exported ({}), so {}'s knob layer is read from there in both runs and the variant does not reach it",
                    var, v, kit.root
                ));
            }
        }
    }
    out
}

fn rel_to(top: &str, p: &str) -> Option<String> {
    if !walk::at_or_under(top, p) {
        return None;
    }
    Some(walk::rel_under(top, p).unwrap_or("").to_string())
}

fn live(files: &[PathBuf], keep: bool) -> Result<Compared, String> {
    let top = walk::toplevel()?;
    let here = walk::cwd()?;
    let rel_cwd = rel_to(&top, &here).ok_or_else(|| format!("{} is outside the work tree {}", here, top))?;
    let gates = walk::abs_against(&here, &walk::knob_scalar("GATE_SDK_GATES_DIR")?);
    let rel_gates = rel_to(&top, &gates)
        .ok_or_else(|| format!("the gates directory {} lies outside the work tree {}", gates, top))?;
    let base = PathBuf::from(walk::abs_against(&here, &walk::knob_scalar("GATE_SDK_TMP_DIR")?));
    std::fs::create_dir_all(&base).map_err(|e| format!("cannot create {}: {}", base.display(), e))?;
    let exe = proc::self_image()
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
    let exported_gates = std::env::var("GATE_SDK_GATES_DIR").map(|v| !v.is_empty()).unwrap_or(false);
    let battery = |cwd: &Path| -> Result<(i32, String), String> {
        let mut env = vec![
            ("GATE_SDK_VERBOSE".to_string(), "1".to_string()),
            ("GATE_SDK_ROOT".to_string(), String::new()),
        ];
        // spec: gate-sdk/SPEC.md §run-config-variant — an exported gates directory is re-pointed
        // into the scratch, or both runs would read the live tree's registry
        if exported_gates {
            let scratch_top = walk::toplevel_in(&cwd.display().to_string())?;
            env.push(("GATE_SDK_GATES_DIR".to_string(), walk::abs_against(&scratch_top, &rel_gates)));
        }
        let unset: Vec<String> = proc::GIT_REPO_LOCATORS.iter().map(|s| s.to_string()).collect();
        let done = proc::run_merged_scrubbed(
            &programs::CHECKWRIGHT_GATES.at(exe.clone()),
            &["--run"],
            &env,
            cwd,
            &unset,
        )?;
        Ok((done.reported_code(), String::from_utf8_lossy(done.output()).into_owned()))
    };
    let plan = Plan { source: Path::new(&top), rel_cwd: &rel_cwd, rel_gates: &rel_gates, files, base: &base, keep };
    compare(&plan, &place, &battery)
}

fn git(dir: &Path, args: &[&str]) -> Result<(), String> {
    let d = dir.display().to_string();
    let mut argv: Vec<&str> = vec!["-C", &d, "-c", "user.email=variant@example.invalid", "-c", "user.name=variant"];
    argv.extend_from_slice(args);
    let done = proc::run(&programs::GIT, &argv)?;
    match done.failure_report() {
        None => Ok(()),
        Some(r) => Err(format!("git {} failed in the scratch — {}", args.join(" "), r)),
    }
}

fn battery_in(battery: Battery, cwd: &Path, which: &str) -> Result<String, String> {
    let (rc, log) = battery(cwd)?;
    if rc != 0 && rc != 1 {
        return Err(format!("the {} battery could not run (exit {}):\n{}", which, rc, log.trim_end()));
    }
    Ok(log)
}

// spec: gate-sdk/SPEC.md §run-config-variant — the scratch, the baseline, the overlay committed,
// the variant run; the scratch guard removes it on every exit path unless it is kept
fn compare(plan: &Plan, place: &dyn Fn(&Path) -> Result<(), String>, battery: Battery) -> Result<Compared, String> {
    let Plan { source, rel_cwd, rel_gates, files, base, keep } = plan;
    let mut guard = csmoke::tracked_history_scratch(source, base, NAME)?;
    let root = guard.dir.clone().ok_or("the scratch was not created")?;
    place(&root)?;
    let cwd = root.join(rel_cwd);
    let baseline = battery_in(battery, &cwd, "baseline")?;
    // spec: gate-sdk/SPEC.md §run-config-variant — the scratch returns to its built state before the
    // overlay, so the variant run reads nothing the baseline run wrote
    git(&root, &["reset", "-q", "--hard"])?;
    git(&root, &["clean", "-q", "-f", "-d", "-x"])?;
    place(&root)?;
    let gates = root.join(rel_gates);
    std::fs::create_dir_all(&gates).map_err(|e| format!("cannot create {}: {}", gates.display(), e))?;
    let mut staged: Vec<String> = Vec::new();
    for f in files.iter() {
        let name = f.file_name().ok_or_else(|| format!("{} names no file", f.display()))?;
        let to = gates.join(name);
        std::fs::copy(f, &to).map_err(|e| format!("cannot copy {} to {}: {}", f.display(), to.display(), e))?;
        staged.push(to.display().to_string());
    }
    let mut add = vec!["add", "-f", "--"];
    add.extend(staged.iter().map(String::as_str));
    git(&root, &add)?;
    git(&root, &["commit", "-q", "--allow-empty", "-m", "config variant"])?;
    let variant = battery_in(battery, &cwd, "variant")?;
    let kept = if *keep { guard.dir.take() } else { None };
    Ok(Compared { baseline, variant, kept })
}

fn passed(s: &Section) -> bool {
    s.tail.as_deref().is_some_and(|t| t.ends_with(" pass"))
}

fn indent(out: &mut String, text: &str) {
    for l in text.lines() {
        out.push_str("      ");
        out.push_str(l);
        out.push('\n');
    }
}

// spec: gate-sdk/SPEC.md §run-config-variant — the rows, the count line and the exit: 1 exactly
// when the variant newly reds a gate
fn report(label: &str, baseline: &str, variant: &str) -> (String, i32) {
    let b = sections(baseline);
    let v = sections(variant);
    let mut names: Vec<&str> = b.iter().map(|s| s.name.as_str()).collect();
    for s in &v {
        if !names.contains(&s.name.as_str()) {
            names.push(&s.name);
        }
    }
    let mut rows = String::new();
    let (mut red, mut green, mut moved, mut added, mut removed) = (0, 0, 0, 0, 0);
    for n in &names {
        let bs = b.iter().find(|s| s.name == *n);
        let vs = v.iter().find(|s| s.name == *n);
        match (bs, vs) {
            (Some(bs), Some(vs)) => {
                if passed(bs) && !passed(vs) {
                    red += 1;
                    rows.push_str(&format!("  red: {}\n", n));
                    indent(&mut rows, &vs.output);
                } else if !passed(bs) && passed(vs) {
                    green += 1;
                    rows.push_str(&format!("  green: {}\n", n));
                } else if bs.output != vs.output {
                    moved += 1;
                    rows.push_str(&format!("  moved: {}\n", n));
                    indent(&mut rows, &vs.output);
                }
            }
            (None, Some(vs)) => {
                added += 1;
                if passed(vs) {
                    rows.push_str(&format!("  added: {} (pass)\n", n));
                } else {
                    red += 1;
                    rows.push_str(&format!("  added: {} (fail)\n", n));
                    indent(&mut rows, &vs.output);
                }
            }
            (Some(_), None) => {
                removed += 1;
                rows.push_str(&format!("  removed: {}\n", n));
            }
            (None, None) => {}
        }
    }
    let mut out = format!("CONFIG-VARIANT: {} against the baseline, {} gate(s) compared\n", label, names.len());
    out.push_str(&rows);
    out.push_str(&format!(
        "CONFIG-VARIANT: {} newly red, {} green, {} moved, {} added, {} removed\n",
        red, green, moved, added, removed
    ));
    (out, i32::from(red > 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sh_git(dir: &Path, args: &[&str]) {
        git(dir, args).expect("git runs");
    }

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("cw-config-variant-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("mkdir");
        d
    }

    // spec: gate-sdk/SPEC.md §run-config-variant — a stand-in battery over a two-member registry
    // whose verdicts a knob moves: `DEMO_RED_<n> = 1` reds member n, `DEMO_ARM_<n> = 1` arms its
    // clean line, and `probe.txt`'s presence is printed so the overlay rule is observable
    fn demo_battery(cwd: &Path) -> Result<(i32, String), String> {
        let read = |p: &str| std::fs::read_to_string(cwd.join(p)).unwrap_or_default();
        let knobs = read("scripts/demo-config.knobs");
        let set = |k: &str| knobs.lines().any(|l| l.trim() == format!("{} = 1", k));
        let state = cwd.join("state.txt");
        let stale = if state.exists() { " stale" } else { "" };
        std::fs::write(&state, "written by a run\n").map_err(|e| e.to_string())?;
        let mut log = String::new();
        let mut rc = 0;
        for m in crate::registry::members(&read("scripts/gates.list")) {
            let n = m.trim_start_matches("check-");
            let armed = if set(&format!("DEMO_ARM_{}", n)) { "armed" } else { "unarmed" };
            let probe = if cwd.join("probe.txt").exists() { read("probe.txt") } else { "no probe\n".into() };
            log.push_str(&format!("\n===== {} =====\n{}: {}{} {}", m, n, armed, stale, probe));
            if set(&format!("DEMO_RED_{}", n)) {
                rc = 1;
                log.push_str(&format!("  FAIL: {} (exit 1)\n", m));
            } else {
                log.push_str(&format!("  PASS: {}\n", m));
            }
        }
        log.push_str("\n===== gates summary =====\n");
        Ok((rc, log))
    }

    fn tree(tag: &str) -> PathBuf {
        let t = scratch(tag).join("tree");
        std::fs::create_dir_all(t.join("scripts")).expect("mkdir");
        std::fs::write(t.join("scripts/gates.list"), "check-one\ncheck-two\n").expect("write");
        std::fs::write(t.join("scripts/demo-config.knobs"), "# demo\n").expect("write");
        std::fs::write(t.join("probe.txt"), "committed\n").expect("write");
        sh_git(&t, &["init", "-q"]);
        sh_git(&t, &["add", "-A"]);
        sh_git(&t, &["commit", "-q", "-m", "seed"]);
        t
    }

    fn variant(tag: &str, files: &[(&str, &str)]) -> Vec<PathBuf> {
        let d = scratch(&format!("{}-variant", tag));
        for (n, body) in files {
            std::fs::write(d.join(n), body).expect("write");
        }
        variant_files(&d).expect("a readable variant")
    }

    fn run_case(tag: &str, t: &Path, files: &[PathBuf], keep: bool) -> (Compared, PathBuf) {
        let base = scratch(&format!("{}-base", tag));
        let plan = Plan { source: t, rel_cwd: "", rel_gates: "scripts", files, base: &base, keep };
        let c = compare(&plan, &|_| Ok(()), &demo_battery).expect("compares");
        (c, base)
    }

    #[test]
    fn a_variant_redding_a_member_prints_its_row_and_output_and_exits_1() {
        let t = tree("red");
        let files = variant("red", &[("demo-config.knobs", "DEMO_RED_two = 1\n")]);
        let (c, _) = run_case("red", &t, &files, false);
        let (text, rc) = report("v", &c.baseline, &c.variant);
        assert_eq!(rc, 1, "{}", text);
        assert!(text.starts_with("CONFIG-VARIANT: v against the baseline, 2 gate(s) compared\n"), "{}", text);
        assert!(text.contains("  red: check-two\n      two: unarmed committed\n"), "{}", text);
        assert!(!text.contains("check-one"), "{}", text);
    }

    #[test]
    fn a_variant_arming_a_clean_line_is_a_moved_row_at_exit_0() {
        let t = tree("moved");
        let files = variant("moved", &[("demo-config.knobs", "DEMO_ARM_one = 1\n")]);
        let (c, _) = run_case("moved", &t, &files, false);
        let (text, rc) = report("v", &c.baseline, &c.variant);
        assert_eq!(rc, 0, "{}", text);
        assert!(text.contains("  moved: check-one\n      one: armed committed\n"), "{}", text);
        assert!(text.contains("0 newly red, 0 green, 1 moved"), "{}", text);
    }

    #[test]
    fn a_variant_moving_nothing_prints_no_row_though_the_baseline_wrote_state() {
        let t = tree("inert");
        let files = variant("inert", &[("demo-config.knobs", "# a comment only\n")]);
        let (c, _) = run_case("inert", &t, &files, false);
        let (text, rc) = report("v", &c.baseline, &c.variant);
        assert_eq!(rc, 0, "{}", text);
        assert!(!c.variant.contains("stale"), "{}", c.variant);
        assert!(text.ends_with("0 newly red, 0 green, 0 moved, 0 added, 0 removed\n"), "{}", text);
    }

    #[test]
    fn a_variant_registry_drops_and_adds_members() {
        let t = tree("registry");
        let files = variant("registry", &[("gates.list", "check-one\ncheck-three\n")]);
        let (c, _) = run_case("registry", &t, &files, false);
        let (text, rc) = report("v", &c.baseline, &c.variant);
        assert_eq!(rc, 0, "{}", text);
        assert!(text.contains("  removed: check-two\n"), "{}", text);
        assert!(text.contains("  added: check-three (pass)\n"), "{}", text);
        assert!(text.contains("3 gate(s) compared"), "{}", text);
    }

    #[test]
    fn an_unusable_variant_dir_exits_2_before_any_scratch() {
        let d = scratch("refused");
        assert_eq!(run(&[d.join("absent").display().to_string()]), 2);
        assert_eq!(run(&[d.display().to_string()]), 2);
        std::fs::create_dir_all(d.join("sub")).expect("mkdir");
        std::fs::write(d.join("gates.list"), "check-one\n").expect("write");
        assert!(variant_files(&d).unwrap_err().contains("not a regular file"));
        assert_eq!(run(&[d.display().to_string()]), 2);
        assert_eq!(run(&[]), 2);
    }

    #[test]
    fn an_unstaged_edit_reaches_both_runs_and_an_untracked_file_neither() {
        let t = tree("worktree");
        std::fs::write(t.join("probe.txt"), "unstaged\n").expect("write");
        std::fs::write(t.join("untracked.txt"), "x\n").expect("write");
        let files = variant("worktree", &[("demo-config.knobs", "DEMO_ARM_two = 1\n")]);
        let (c, base) = run_case("worktree", &t, &files, true);
        assert!(c.baseline.contains("one: unarmed unstaged"), "{}", c.baseline);
        assert!(c.variant.contains("two: armed unstaged"), "{}", c.variant);
        let kept = c.kept.expect("kept");
        assert!(!kept.join("untracked.txt").exists());
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn the_scratch_is_removed_unless_kept() {
        let t = tree("keep");
        let files = variant("keep", &[("demo-config.knobs", "DEMO_ARM_one = 1\n")]);
        let (gone, base) = run_case("keep", &t, &files, false);
        assert!(gone.kept.is_none());
        assert!(walk::list_dir(&base).expect("base").is_empty(), "the scratch outlived the run");
        let (c, base) = run_case("keep2", &t, &files, true);
        let kept = c.kept.expect("--keep keeps the scratch");
        assert!(kept.join("scripts/demo-config.knobs").is_file());
        assert!(walk::under(&base.display().to_string(), &kept.display().to_string()));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn an_exported_knob_file_locator_prints_a_note() {
        let env = |v: &str| (v == "SITE_KIT_KNOB_FILE").then(|| "/elsewhere/site.knobs".to_string());
        let got = notes(&env);
        assert_eq!(got.len(), 1, "{:?}", got);
        assert!(got[0].starts_with("note: SITE_KIT_KNOB_FILE is exported (/elsewhere/site.knobs)"), "{}", got[0]);
        assert!(notes(&|_| None).is_empty());
    }
}
