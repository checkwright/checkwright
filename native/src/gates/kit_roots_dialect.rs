// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — the dialect rule's executable form: a kit tree
// vendored at the root and under a subdirectory must yield one corpus, since every reader that
// joins a root onto a path is byte-identical on both spellings only while the root is not nested
use crate::{proc, programs, walk};
use std::path::Path;

const NAME: &str = "check-kit-roots-dialect";

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — the compared arms are the ones whose whole
// corpus is a kit-root join and which read no path-shaped consumer glob, so a count difference
// between the layouts can only be the dialect and never a knob file spelled for one of them
const ARMS: &[&str] = &["--emit-fixture-suites", "--emit-enum-sets"];

struct Layout {
    name: &'static str,
    dir: String,
    root: String,
}

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — the per-run scratch base is reaped on every
// ordinary exit, the upgrade-smoke `Scratch` shape: `Drop` runs on every return path `rule`
// takes, `?`-propagated ones included
struct ScratchGuard {
    base: String,
}

impl Drop for ScratchGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — the tree defaults to the fixture this kit
// ships, resolved off the gate-sdk locator so a consumer that vendored the kit elsewhere still
// finds it; a positional overrides it, which is how the fixture pair names its two trees
fn tree_of(args: &[String]) -> Result<String, String> {
    let given = args.first().filter(|a| !a.is_empty()).cloned();
    let tree = match given {
        Some(a) => a,
        None => format!(
            "{}/gate-tests/{}/good/tree",
            walk::sdk_root().trim_end_matches('/'),
            NAME
        ),
    };
    if !Path::new(&tree).is_dir() {
        return Err(format!(
            "no kit tree to vendor at {} — the check could not run; treating as failure (not clean)",
            tree
        ));
    }
    Ok(tree)
}

fn rule(args: &[String]) -> Result<i32, String> {
    let tree = tree_of(args)?;
    let base = scratch()?;
    let _guard = ScratchGuard { base: base.clone() };
    let layouts = [
        vendor(&tree, &base, "flat")?,
        vendor(&tree, &base, "nested")?,
    ];

    // spec: gate-sdk/SPEC.md §check-kit-roots-dialect — assertion A: the derivation sees the layout
    // it stands in. `vendor` has already proven the nested tree keeps its kits under the
    // subdirectory, so a nested run answering the flat spelling is a violation, not a broken fixture
    let flat_roots = kit_roots(&layouts[0])?;
    let nested_roots = kit_roots(&layouts[1])?;
    if flat_roots.is_empty() {
        return Err(format!(
            "the flat vendoring of {} enumerated no kit root — the check could not run; treating as failure (not clean)",
            tree
        ));
    }
    let basenames = |v: &[String]| -> Vec<String> {
        v.iter()
            .map(|r| r.rsplit('/').next().unwrap_or(r).to_string())
            .collect()
    };
    let mut violations: Vec<String> = Vec::new();
    if flat_roots == nested_roots {
        violations.push(format!(
            "--emit-kit-roots answers [{}] in both, though the nested tree keeps its kits under vendor/ — every path composed from these names misses them",
            flat_roots.join(" ")
        ));
    }
    if basenames(&flat_roots) != basenames(&nested_roots) {
        violations.push(format!(
            "--emit-kit-roots names [{}] vendored flat and [{}] vendored nested",
            basenames(&flat_roots).join(" "),
            basenames(&nested_roots).join(" ")
        ));
    }

    // spec: gate-sdk/SPEC.md §check-kit-roots-dialect — assertion B: one corpus per arm across the
    // two layouts. A count and not a diff, because the failure this closes is a corpus that shrank
    // silently rather than one whose rows changed, and an arm's rows carry the layout in them.
    let mut counted: Vec<String> = Vec::new();
    for arm in ARMS {
        let flat = corpus(&layouts[0], arm)?;
        let nested = corpus(&layouts[1], arm)?;
        counted.push(format!("{}={}", arm, flat));
        if flat != nested {
            violations.push(format!(
                "{} emits {} row(s) vendored flat and {} vendored nested",
                arm, flat, nested
            ));
        }
    }

    if !violations.is_empty() {
        println!("{}: a kit-root-derived corpus differs between the flat and the nested vendoring of {} — a reader joining a root onto a path is on the kit-parent spelling, and the shrink is silent:", NAME, tree);
        println!();
        for v in &violations {
            println!("  {}", v);
        }
        println!("  help: the reader takes walk::kit_roots (or walk::kit_roots_at/kit_roots_under");
        println!("        where it composes a git pathspec), never walk::kit_roots_rel, which is");
        println!("        for text naming kits from their common parent — gate-sdk/SPEC.md");
        println!("        §Layout and configuration. A hand-listed GATE_SDK_KIT_DIRS spelled for");
        println!("        one layout does this too, and is fixed in the consumer's knob file.");
        return Ok(1);
    }
    println!(
        "{}: clean ({} arm(s) agree across a flat and a nested vendoring of {}; {} kit root(s) each layout, spelled [{}] and [{}]; {})",
        NAME.to_ascii_uppercase().replace("CHECK-", ""),
        ARMS.len(),
        tree,
        flat_roots.len(),
        flat_roots.join(" "),
        nested_roots.join(" "),
        counted.join(", ")
    );
    Ok(0)
}

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — one scratch base per process, under the same
// knob `upgrade-smoke` takes, so a consumer repoints both with one value
fn scratch() -> Result<String, String> {
    let dir = walk::knob_scalar("GATE_SDK_TMP_DIR")?;
    let here = walk::cwd()?;
    let base = format!(
        "{}/{}-{}",
        walk::abs_against(&here, dir.trim_end_matches('/')),
        NAME,
        std::process::id()
    );
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base)
        .map_err(|e| format!("cannot create scratch base {}: {}", base, e))?;
    Ok(base)
}

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — the gates directory stays at the toplevel in
// both layouts and only the kits move, which is exactly the difference between a root-layout
// vendoring and a subdirectory one; each becomes its own repository, since the arms read git
fn vendor(tree: &str, base: &str, layout: &'static str) -> Result<Layout, String> {
    let dir = format!("{}/{}", base, layout);
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let gates_leaf = gates_dir.trim_end_matches('/').rsplit('/').next().unwrap_or("scripts").to_string();
    std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {}", dir, e))?;
    let mut sdk_parent = dir.clone();
    if layout == "nested" {
        sdk_parent = format!("{}/vendor", dir);
        std::fs::create_dir_all(&sdk_parent)
            .map_err(|e| format!("cannot create {}: {}", sdk_parent, e))?;
    }
    for (name, _) in walk::list_dir(Path::new(tree))? {
        let src = format!("{}/{}", tree.trim_end_matches('/'), name);
        if !Path::new(&src).is_dir() {
            continue;
        }
        let into = if name == gates_leaf { &dir } else { &sdk_parent };
        copy_tree(&src, &format!("{}/{}", into, name))?;
    }
    let sdk = if layout == "nested" {
        "vendor/gate-sdk".to_string()
    } else {
        "gate-sdk".to_string()
    };
    if !Path::new(&format!("{}/{}", dir, sdk)).is_dir() {
        return Err(format!(
            "the {} vendoring of {} carries no gate-sdk root at {} — the check could not run; treating as failure (not clean)",
            layout, tree, sdk
        ));
    }
    for argv in [
        vec!["-C", &dir, "init", "-q", "."],
        vec!["-C", &dir, "add", "-A", "."],
        vec!["-C", &dir, "-c", "user.email=fixture@localhost", "-c", "user.name=fixture", "commit", "-q", "-m", "vendored"],
    ] {
        let c = proc::run(&programs::GIT, &argv)?;
        if c.code() != Some(0) {
            return Err(format!(
                "git {} failed (exit {}) preparing the {} vendoring — the check could not run; treating as failure (not clean)",
                argv[2],
                c.code().unwrap_or(-1),
                layout
            ));
        }
    }
    Ok(Layout {
        name: layout,
        dir,
        root: sdk,
    })
}

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — a recursive copy in-crate rather than a `cp`
// spawn: the payload's program floor is the adopter constraint, and the fixture tree is small
fn copy_tree(src: &str, dst: &str) -> Result<(), String> {
    std::fs::create_dir_all(dst).map_err(|e| format!("cannot create {}: {}", dst, e))?;
    for (name, _) in walk::list_dir(Path::new(src))? {
        let from = format!("{}/{}", src, name);
        let to = format!("{}/{}", dst, name);
        if Path::new(&from).is_dir() {
            copy_tree(&from, &to)?;
        } else {
            std::fs::copy(&from, &to).map_err(|e| format!("cannot copy {} to {}: {}", from, to, e))?;
        }
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — the binary comes from the knob rather than
// from `current_exe`, the spelling `check-gate-binary-fresh` already takes: it is the one a
// declaration can name, and the one the battery actually dispatches every other member through
fn binary() -> Result<String, String> {
    // spec: gate-sdk/SPEC.md §check-kit-roots-dialect — the knob's default is repo-root-relative
    // like every other, so it resolves against the toplevel rather than the cwd: every child runs
    // inside a vendoring, and this gate's own fixture cases run from a case dir
    let anchor = walk::toplevel().or_else(|_| walk::cwd())?;
    let bin = walk::abs_against(&anchor, &walk::knob_scalar("GATE_SDK_NATIVE_BIN")?);
    if !proc::is_executable(Path::new(&bin)) {
        return Err(format!(
            "{} is absent or not executable, and every compared arm runs through it — the check could not run; treating as failure (not clean)",
            bin
        ));
    }
    Ok(bin)
}

// spec: gate-sdk/SPEC.md §check-kit-roots-dialect — every arm runs through that binary from inside
// the vendoring, so the corpus measured is the deployed derivation rather than one this gate
// re-implemented, and a difference between the layouts is a difference in the shipped readers
fn emit(layout: &Layout, arm: &str) -> Result<String, String> {
    let bin = binary()?;
    // spec: gate-sdk/SPEC.md §check-kit-roots-dialect — only the locator that names where the tree
    // was vendored is handed down. GATE_SDK_KIT_DIRS especially is not: an env value outranks a
    // knob file, so passing one would overwrite the very config a vendoring's own scripts/ carries
    let env = vec![("GATE_SDK_ROOT".to_string(), layout.root.clone())];
    let c = proc::run_with_env_in(
        &programs::CHECKWRIGHT_GATES.at(bin.as_str()),
        &[arm],
        &env,
        Some(Path::new(&layout.dir)),
    )?;
    match c.stdout() {
        Some(o) if c.code() == Some(0) => Ok(String::from_utf8_lossy(o).into_owned()),
        _ => Err(format!(
            "{} exited {} on the {} vendoring — the check could not run; treating as failure (not clean)",
            arm,
            c.code().unwrap_or(-1),
            layout.name
        )),
    }
}

fn kit_roots(layout: &Layout) -> Result<Vec<String>, String> {
    Ok(emit(layout, "--emit-kit-roots")?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_string)
        .collect())
}

fn corpus(layout: &Layout, arm: &str) -> Result<usize, String> {
    Ok(emit(layout, arm)?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count())
}
