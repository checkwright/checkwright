// spec: installer/SPEC.md §Payload recipes — the crate's owner of the package's `recipes/`: each
// named recipe's seam lines and the gates it drops, read as data, so no gate or toolkit name enters
// the crate.
use super::{profile, recipe, refuse, Refusal};
use std::collections::BTreeSet;
use std::path::Path;

pub const DIR: &str = "recipes";
const UNREGISTER: &str = "unregister.list";
// spec: installer/SPEC.md §The gate binary — the one seam every profile writes and no kit template
// seeds: the placement op writes it on every install
pub const PLACEMENT_SEAM: &str = "gate-sdk-config.knobs";

#[derive(Debug)]
pub struct Recipe {
    pub name: String,
    pub knobs: Vec<(String, Vec<String>)>,
    pub drops: Vec<String>,
}

// spec: installer/SPEC.md §Payload recipes — a line opening with `#` is commentary in both kinds of
// file, and a blank line carries nothing
fn lines(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim_end)
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .map(String::from)
        .collect()
}

pub fn available(pkg_root: &Path) -> Vec<String> {
    let mut out: Vec<String> = crate::walk::list_dir(&pkg_root.join(DIR))
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, is_dir)| *is_dir)
        .map(|(name, _)| name)
        .collect();
    out.sort();
    out
}

// spec: installer/SPEC.md §Payload recipes — two kinds of file are read and any other file is not
fn read(pkg_root: &Path, name: &str) -> Option<Recipe> {
    let dir = pkg_root.join(DIR).join(name);
    if name.is_empty() || name.contains(['/', '\\']) || !dir.is_dir() {
        return None;
    }
    let mut entries = crate::walk::list_dir(&dir).ok()?;
    entries.sort();
    let mut knobs: Vec<(String, Vec<String>)> = Vec::new();
    let mut drops: Vec<String> = Vec::new();
    for (file, is_dir) in entries {
        if is_dir {
            continue;
        }
        let text = std::fs::read_to_string(dir.join(&file)).unwrap_or_default();
        if file == UNREGISTER {
            drops = lines(&text).iter().map(|l| l.trim().to_string()).collect();
        } else if file.ends_with(".knobs") {
            knobs.push((file, lines(&text)));
        }
    }
    Some(Recipe {
        name: name.to_string(),
        knobs,
        drops,
    })
}

// spec: installer/SPEC.md §Payload recipes — a run passing neither flag re-applies the recorded
// set, and a run passing either replaces it
pub fn names_to_apply(given: Option<&[String]>, recorded: &str) -> Vec<String> {
    match given {
        Some(names) => names.to_vec(),
        None => recorded.split_whitespace().map(String::from).collect(),
    }
}

// spec: installer/SPEC.md §Payload recipes — the first refusal: a name the package does not carry
pub fn resolve(pkg_root: &Path, names: &[String]) -> Result<Vec<Recipe>, Refusal> {
    let mut out: Vec<Recipe> = Vec::new();
    for name in names {
        match read(pkg_root, name) {
            Some(r) => out.push(r),
            None => {
                return Err(refuse(
                    format!("unknown payload recipe: {}", name),
                    format!("payload recipes in this package: {} ", available(pkg_root).join(" ")),
                    2,
                ))
            }
        }
    }
    Ok(out)
}

// spec: installer/SPEC.md §What init seeds — the seam files a kit set writes: each kit's derived
// config templates, and the gate-sdk seam the placement op writes on every install
pub fn seams_written(payload: &Path, kits: &[String]) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = BTreeSet::new();
    out.insert(PLACEMENT_SEAM.to_string());
    for kit in kits {
        for (_, dest) in recipe::config_seam_plan(&payload.join(kit), "") {
            out.insert(dest.trim_start_matches('/').to_string());
        }
    }
    out
}

// spec: installer/SPEC.md §Payload recipes — the second refusal: a recipe file naming a seam the
// selected kit set does not write, whose help names the profiles that write every one
pub fn check_seams(pkg_root: &Path, profile_name: &str, kits: &[String], recipes: &[Recipe]) -> Result<(), Refusal> {
    let payload = pkg_root.join("payload");
    let written = seams_written(&payload, kits);
    let wanted: BTreeSet<&str> = recipes
        .iter()
        .flat_map(|r| r.knobs.iter().map(|(f, _)| f.as_str()))
        .collect();
    let Some((recipe, file)) = recipes.iter().find_map(|r| {
        r.knobs
            .iter()
            .find(|(f, _)| !written.contains(f))
            .map(|(f, _)| (r.name.as_str(), f.as_str()))
    }) else {
        return Ok(());
    };
    let fit: Vec<String> = profile::names(pkg_root)
        .into_iter()
        .filter(|p| {
            let seams = seams_written(&payload, &profile::kits(pkg_root, p));
            wanted.iter().all(|f| seams.contains(*f))
        })
        .collect();
    Err(refuse(
        format!(
            "payload recipe {} writes {}, which the selected kit set of the {} profile does not seed",
            recipe, file, profile_name
        ),
        format!("choose a profile that seeds every file the recipes write: {} ", fit.join(" ")),
        2,
    ))
}

// spec: installer/SPEC.md §Payload recipes — every applied recipe's lines for one seam, in
// application order
pub fn lines_for(recipes: &[Recipe], file: &str) -> Vec<String> {
    recipes
        .iter()
        .flat_map(|r| r.knobs.iter().filter(|(f, _)| f == file).flat_map(|(_, l)| l.clone()))
        .collect()
}

pub fn touches(recipes: &[Recipe], file: &str) -> Vec<String> {
    recipes
        .iter()
        .filter(|r| r.knobs.iter().any(|(f, l)| f == file && !l.is_empty()))
        .map(|r| r.name.clone())
        .collect()
}

// spec: installer/SPEC.md §Payload recipes — a seam is its kit template followed by each applied
// recipe's lines for it, never appended after the claim
pub fn compose(template: &str, recipes: &[Recipe], file: &str) -> String {
    let mut out = template.to_string();
    let add = lines_for(recipes, file);
    if add.is_empty() {
        return out;
    }
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    for l in add {
        out.push_str(&l);
        out.push('\n');
    }
    out
}

// spec: installer/SPEC.md §Payload recipes — the third refusal: a composed seam the knob grammar
// refuses, naming every recipe that wrote into it
pub fn check_composed(file: &str, text: &str, recipes: &[Recipe]) -> Result<(), Refusal> {
    crate::knobs::check_seam(file, text).map_err(|why| {
        refuse(
            format!(
                "the payload recipes {} compose a {} the knob grammar refuses: {}",
                touches(recipes, file).join(", "),
                file,
                why
            ),
            "apply the recipes one at a time, or drop the one whose line collides.",
            2,
        )
    })
}

pub fn drops(recipes: &[Recipe]) -> BTreeSet<String> {
    recipes.iter().flat_map(|r| r.drops.iter().cloned()).collect()
}

// spec: installer/SPEC.md §Payload recipes — the registry loses each dropped member; a name it does
// not carry is no finding
pub fn drop_gates(registry: &str, dropped: &BTreeSet<String>) -> String {
    let mut out = String::new();
    for l in registry.lines() {
        let t = l.trim();
        if !t.starts_with('#') && dropped.contains(t) {
            continue;
        }
        out.push_str(l);
        out.push('\n');
    }
    out
}

// spec: installer/SPEC.md §Payload recipes — a kept file is reported, never merged into: the lines
// it lacks, compared as the grammar reads them
pub fn missing(kept: &str, wanted: &[String]) -> Vec<String> {
    let have: BTreeSet<&str> = kept.lines().map(str::trim).collect();
    wanted
        .iter()
        .filter(|w| !have.contains(w.trim()))
        .cloned()
        .collect()
}

pub fn still_registered(kept_registry: &str, dropped: &BTreeSet<String>) -> Vec<String> {
    crate::registry::members(kept_registry)
        .into_iter()
        .map(|m| m.trim().to_string())
        .filter(|m| dropped.contains(m))
        .collect()
}

// spec: installer/SPEC.md §Payload recipes — the report a kept seam or registry gets, with the remedy
pub fn report_kept(file: &str, lines: &[String], gates: &[String]) {
    if lines.is_empty() && gates.is_empty() {
        return;
    }
    println!("\n{} is yours, so the payload recipes' or the selection's change is not placed there:", file);
    for l in lines {
        println!("  add:  {}", l);
    }
    for g in gates {
        println!("  drop: {}", g);
    }
    println!("  help: add them by hand, or pass --force to take init's file.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cw-precipe-{}-{}", tag, std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(dir.join(DIR)).expect("cannot make the scratch package");
        dir
    }

    fn write(dir: &Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        std::fs::create_dir_all(p.parent().expect("no parent")).expect("cannot make the dir");
        std::fs::write(p, body).expect("cannot write the scratch file");
    }

    // spec: installer/SPEC.md §Payload recipes — composition is template then recipe lines in
    // `--recipe` order, commentary and blanks dropped, and a file no recipe names is the template
    #[test]
    fn composition_is_the_template_then_each_recipe_in_order() {
        let dir = scratch("order");
        write(&dir, "recipes/one/canon-config.knobs", "# c\nA_KIT_X[] = one\n\n");
        write(&dir, "recipes/two/canon-config.knobs", "A_KIT_X[] = two\n");
        write(&dir, "recipes/two/notes.md", "A_KIT_X[] = never\n");
        let rs = resolve(&dir, &["two".to_string(), "one".to_string()]).expect("resolved");
        assert_eq!(compose("# t\nB = 1", &rs, "canon-config.knobs"), "# t\nB = 1\nA_KIT_X[] = two\nA_KIT_X[] = one\n");
        assert_eq!(compose("# t\n", &rs, "other.knobs"), "# t\n");
        assert_eq!(touches(&rs, "canon-config.knobs"), vec!["two", "one"]);
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §Payload recipes — an unknown name refuses at exit 2 naming the set
    #[test]
    fn an_unknown_recipe_refuses_naming_those_the_package_carries() {
        let dir = scratch("unknown");
        write(&dir, "recipes/alpha/x.knobs", "");
        let r = resolve(&dir, &["beta".to_string()]).expect_err("an unknown name resolved");
        assert_eq!(r.code, 2);
        assert!(r.message.contains("beta") && r.help.contains("alpha"), "{:?}", r);
        assert!(resolve(&dir, &["../alpha".to_string()]).is_err());
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §Payload recipes — a recipe file for a seam the profile does not
    // write refuses, and the help names the profiles that write it
    #[test]
    fn a_seam_the_profile_does_not_write_refuses_naming_a_profile_that_does() {
        let dir = scratch("seam");
        for stem in ["a", "b"] {
            write(&dir, &format!("payload/{}-kit/templates/{}-config.knobs", stem, stem), "");
        }
        write(&dir, "profiles.list", "small\ta-kit\n");
        write(&dir, "recipes/r/b-config.knobs", "B_KIT_Y = 1\n");
        write(&dir, "recipes/r/gate-sdk-config.knobs", "G = 1\n");
        let rs = resolve(&dir, &["r".to_string()]).expect("resolved");
        let small = profile::kits(&dir, "small");
        let r = check_seams(&dir, "small", &small, &rs).expect_err("an unwritten seam passed");
        assert_eq!(r.code, 2);
        assert!(r.message.contains("b-config.knobs") && r.help.contains("full") && !r.help.contains("small"), "{:?}", r);
        let full = profile::kits(&dir, "full");
        assert!(check_seams(&dir, "full", &full, &rs).is_ok());
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §Payload recipes — two recipes setting one scalar compose a seam the
    // grammar refuses, and the refusal names both
    #[test]
    fn a_composed_seam_the_grammar_refuses_names_both_recipes() {
        let dir = scratch("grammar");
        write(&dir, "recipes/one/gate-sdk-config.knobs", "GATE_SDK_PRUNE_EXTRA_DIRS = a\n");
        write(&dir, "recipes/two/gate-sdk-config.knobs", "GATE_SDK_PRUNE_EXTRA_DIRS = b\n");
        let both = resolve(&dir, &["one".to_string(), "two".to_string()]).expect("resolved");
        let text = compose("", &both, PLACEMENT_SEAM);
        let r = check_composed(PLACEMENT_SEAM, &text, &both).expect_err("a doubled scalar passed");
        assert_eq!(r.code, 2);
        assert!(r.message.contains("one, two"), "{:?}", r);
        let one = resolve(&dir, &["one".to_string()]).expect("resolved");
        assert!(check_composed(PLACEMENT_SEAM, &compose("", &one, PLACEMENT_SEAM), &one).is_ok());
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §Payload recipes — a kept file lacking the lines is reported with
    // them, and one already carrying every line has nothing to report
    #[test]
    fn a_kept_file_reports_only_the_lines_it_lacks() {
        let wanted = vec!["X[] = a".to_string(), "Y = b".to_string()];
        assert_eq!(missing("# mine\nX[] = a\n", &wanted), vec!["Y = b".to_string()]);
        assert!(missing("X[] = a\n  Y = b\n", &wanted).is_empty());
        let stem = format!("{}-dropped", "check");
        let dropped: BTreeSet<String> = [stem.clone()].into_iter().collect();
        assert_eq!(still_registered(&format!("# h\n{}\n", stem), &dropped), vec![stem.clone()]);
        assert!(still_registered("# h\n", &dropped).is_empty());
        assert_eq!(drop_gates(&format!("# {}\n{}\nkeep\n", stem, stem), &dropped), format!("# {}\nkeep\n", stem));
    }

    // spec: installer/SPEC.md §Payload recipes — neither flag re-applies the recorded set; a given
    // set, the empty one `--no-recipe` leaves included, replaces it
    #[test]
    fn the_recorded_set_applies_only_when_no_flag_names_one() {
        assert_eq!(names_to_apply(None, "a b"), vec!["a", "b"]);
        assert!(names_to_apply(None, "").is_empty());
        assert!(names_to_apply(Some(&[]), "a b").is_empty());
        assert_eq!(names_to_apply(Some(&["c".to_string()]), "a b"), vec!["c"]);
    }
}
