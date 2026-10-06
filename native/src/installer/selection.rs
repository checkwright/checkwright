// spec: installer/SPEC.md §Selecting kits and gates — the adopter's adjustments to a profile's kit
// set and registry, read from argv or the manifest as data, so no kit or gate name enters the crate
use super::{payload_recipe, profile, recipe, refuse, Refusal, GATES_DIR};
use std::collections::BTreeSet;
use std::path::Path;

// spec: installer/SPEC.md §The manifest — the `selection` object's array keys
pub const WITH_KITS: &str = "with-kits";
pub const WITHOUT_KITS: &str = "without-kits";
pub const WITH_GATES: &str = "with-gates";
pub const WITHOUT_GATES: &str = "without-gates";

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Selection {
    pub with_kits: Vec<String>,
    pub without_kits: Vec<String>,
    pub with_gates: Vec<String>,
    pub without_gates: Vec<String>,
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.lists().iter().all(|(_, l)| l.is_empty())
    }

    pub fn lists(&self) -> [(&'static str, &Vec<String>); 4] {
        [
            (WITH_KITS, &self.with_kits),
            (WITHOUT_KITS, &self.without_kits),
            (WITH_GATES, &self.with_gates),
            (WITHOUT_GATES, &self.without_gates),
        ]
    }

    pub fn from_lists(get: impl Fn(&str) -> Vec<String>) -> Selection {
        Selection {
            with_kits: get(WITH_KITS),
            without_kits: get(WITHOUT_KITS),
            with_gates: get(WITH_GATES),
            without_gates: get(WITHOUT_GATES),
        }
    }

    // spec: installer/SPEC.md §Selecting kits and gates — the one-line summary a dry run and a
    // re-run report print
    pub fn summary(&self) -> String {
        let mut out: Vec<String> = Vec::new();
        for (flag, list) in [
            ("--with-kit", &self.with_kits),
            ("--without-kit", &self.without_kits),
            ("--with-gate", &self.with_gates),
            ("--without-gate", &self.without_gates),
        ] {
            out.extend(list.iter().map(|n| format!("{} {}", flag, n)));
        }
        out.join(" ")
    }
}

// spec: installer/SPEC.md §Selecting kits and gates — a kit every profile carries is the lattice
// minimum's, the runner every other kit's gates resolve through, so no selection removes it
pub fn floor_kits(pkg_root: &Path) -> Vec<String> {
    let mut floor: Option<Vec<String>> = None;
    for p in profile::names(pkg_root) {
        let kits = profile::kits(pkg_root, &p);
        floor = Some(match floor {
            None => kits,
            Some(f) => f.into_iter().filter(|k| kits.contains(k)).collect(),
        });
    }
    floor.unwrap_or_default()
}

// spec: installer/SPEC.md §Selecting kits and gates — the profile's set, plus each added kit, less
// each removed one, in payload order
pub fn kit_set(pkg_root: &Path, profile_kits: &[String], s: &Selection) -> Vec<String> {
    profile::payload_kits(pkg_root)
        .into_iter()
        .filter(|k| (profile_kits.contains(k) || s.with_kits.contains(k)) && !s.without_kits.contains(k))
        .collect()
}

fn resolve_dirs(pkg_root: &Path, root: &Path, kits: &[String]) -> Vec<String> {
    let roots: Vec<String> = kits
        .iter()
        .map(|k| pkg_root.join("payload").join(k).to_string_lossy().into_owned())
        .collect();
    crate::registry::resolve_dirs(&root.join(GATES_DIR).to_string_lossy(), &roots)
}

fn shipped_gates(pkg_root: &Path, kits: &[String]) -> Vec<String> {
    let mut out: BTreeSet<String> = BTreeSet::new();
    for k in kits {
        let checks = pkg_root.join("payload").join(k).join("checks");
        for (name, is_dir) in crate::walk::list_dir(&checks).unwrap_or_default() {
            let member = name.starts_with("check-") && (name.ends_with(".sh") || name.ends_with(".gate"));
            if !is_dir && member {
                out.insert(name[..name.rfind('.').unwrap_or(name.len())].to_string());
            }
        }
    }
    out.into_iter().collect()
}

fn both<'a>(a: &'a [String], b: &[String]) -> Option<&'a String> {
    a.iter().find(|n| b.contains(n))
}

// spec: installer/SPEC.md §Selecting kits and gates — every selection the run cannot honour is
// refused at exit 2 before any write, each help line naming what the package or tree carries
pub fn check(pkg_root: &Path, root: &Path, kits: &[String], s: &Selection) -> Result<(), Refusal> {
    if let Some(n) = both(&s.with_kits, &s.without_kits) {
        return Err(refuse(
            format!("--with-kit and --without-kit both name {}", n),
            "name each kit once: added or removed.",
            2,
        ));
    }
    if let Some(n) = both(&s.with_gates, &s.without_gates) {
        return Err(refuse(
            format!("--with-gate and --without-gate both name {}", n),
            "name each gate once: added or dropped.",
            2,
        ));
    }
    let payload = profile::payload_kits(pkg_root);
    if let Some(k) = s.with_kits.iter().chain(&s.without_kits).find(|k| !payload.contains(k)) {
        return Err(refuse(
            format!("unknown kit: {}", k),
            format!("kits in this payload: {} ", payload.join(" ")),
            2,
        ));
    }
    let floor = floor_kits(pkg_root);
    if let Some(k) = s.without_kits.iter().find(|k| floor.contains(k)) {
        return Err(refuse(
            format!("{} cannot be removed: every profile carries it", k),
            format!(
                "it is the runner, the git hooks and the registry every other kit's gates resolve through; kits a selection may remove: {} ",
                payload.iter().filter(|k| !floor.contains(k)).cloned().collect::<Vec<_>>().join(" ")
            ),
            2,
        ));
    }
    let dirs = resolve_dirs(pkg_root, root, kits);
    let carried = || {
        format!(
            "the selected kits ship: {} — or add a gate of your own to {}/",
            shipped_gates(pkg_root, kits).join(" "),
            GATES_DIR
        )
    };
    for g in &s.with_gates {
        match crate::registry::resolve(g, &dirs) {
            None => {
                return Err(refuse(
                    format!("--with-gate {} resolves to no gate on this tree", g),
                    carried(),
                    2,
                ))
            }
            Some(p) if recipe::install_disposition(Path::new(&p)) == "never" => {
                return Err(refuse(
                    format!("--with-gate {} declares '# install: never'", g),
                    "its kit declares that it cannot hold on a vendored tree, so no install registers it.",
                    2,
                ))
            }
            Some(_) => {}
        }
    }
    if let Some(g) = s.without_gates.iter().find(|g| crate::registry::resolve(g, &dirs).is_none()) {
        return Err(refuse(
            format!("--without-gate {} names no gate a selected kit ships or {}/ carries", g, GATES_DIR),
            carried(),
            2,
        ));
    }
    Ok(())
}

// spec: installer/SPEC.md §Selecting kits and gates — the gates the registry loses: every recipe
// drop and every `--without-gate`, less every `--with-gate`, which outranks a recipe
pub fn dropped(recipe_drops: &BTreeSet<String>, s: &Selection) -> BTreeSet<String> {
    recipe_drops
        .iter()
        .chain(&s.without_gates)
        .filter(|g| !s.with_gates.contains(g))
        .cloned()
        .collect()
}

// spec: installer/SPEC.md §Selecting kits and gates — the derived registry less the dropped gates,
// plus each added gate it does not already carry, in the order given
pub fn adjust(registry: &str, drop: &BTreeSet<String>, s: &Selection) -> String {
    let mut text = payload_recipe::drop_gates(registry, drop);
    let have: BTreeSet<String> = crate::registry::members(&text).iter().map(|m| m.trim().to_string()).collect();
    let mut added: Vec<&String> = Vec::new();
    for g in &s.with_gates {
        if !have.contains(g) && !added.contains(&g) {
            added.push(g);
        }
    }
    if !added.is_empty() {
        text.push_str("# added by --with-gate\n");
        for g in added {
            text.push_str(g);
            text.push('\n');
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cw-select-{}-{}", tag, std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        dir
    }

    fn write(dir: &Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        std::fs::create_dir_all(p.parent().expect("no parent")).expect("cannot make the dir");
        std::fs::write(p, body).expect("cannot write the scratch file");
    }

    fn names(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // spec: installer/SPEC.md §Selecting kits and gates — the fixture names are composed, as the
    // recipe module's are: the crate must carry no literal gate name
    fn gate(stem: &str) -> String {
        format!("{}-{}", "check", stem)
    }

    fn package(tag: &str) -> std::path::PathBuf {
        let dir = scratch(tag);
        write(&dir, "package.json", "{}");
        write(&dir, "profiles.list", "small\ta-kit\nwide\ta-kit\nwide\tb-kit\n");
        for (kit, file, disposition) in [
            ("a-kit", format!("{}.gate", gate("one")), "zero-config"),
            ("b-kit", format!("{}.gate", gate("two")), "on-surface"),
            ("b-kit", format!("{}.gate", gate("never")), "never"),
            ("c-kit", format!("{}.sh", gate("three")), "zero-config"),
        ] {
            let at = dir.join("payload").join(kit).join("checks").join(file);
            std::fs::create_dir_all(at.parent().expect("no parent")).expect("cannot make the dir");
            std::fs::write(at, format!("# install: {}\n", disposition)).expect("cannot write a scratch gate");
        }
        dir
    }

    // spec: installer/SPEC.md §Selecting kits and gates — the kit set is the profile's plus the
    // added less the removed, in payload order; the floor is the kits every profile carries
    #[test]
    fn the_kit_set_composes_in_payload_order_and_the_floor_is_every_profiles() {
        let dir = package("kits");
        let s = Selection { with_kits: names(&["c-kit"]), ..Default::default() };
        assert_eq!(kit_set(&dir, &names(&["a-kit"]), &s), names(&["a-kit", "c-kit"]));
        let s = Selection { without_kits: names(&["b-kit"]), ..Default::default() };
        assert_eq!(kit_set(&dir, &names(&["a-kit", "b-kit"]), &s), names(&["a-kit"]));
        assert_eq!(floor_kits(&dir), names(&["a-kit"]));
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §Selecting kits and gates — each refusal, and a selection the tree
    // can honour passing, the adopter's own gate among it
    #[test]
    fn a_selection_the_run_cannot_honour_is_refused_at_exit_2() {
        let dir = package("refuse");
        let root = dir.join("consumer");
        write(&root, &format!("{}/{}.sh", GATES_DIR, gate("mine")), "#!/bin/sh\n");
        let kits = names(&["a-kit", "b-kit"]);
        let refused = |s: Selection| check(&dir, &root, &kits, &s).expect_err("a selection passed").code;
        let sel = |wk: &[&str], wok: &[&str], wg: &[String], wog: &[String]| Selection {
            with_kits: names(wk),
            without_kits: names(wok),
            with_gates: wg.to_vec(),
            without_gates: wog.to_vec(),
        };
        assert_eq!(refused(sel(&["z-kit"], &[], &[], &[])), 2);
        assert_eq!(refused(sel(&[], &["a-kit"], &[], &[])), 2);
        assert_eq!(refused(sel(&["c-kit"], &["c-kit"], &[], &[])), 2);
        assert_eq!(refused(sel(&[], &[], &[gate("nope")], &[])), 2);
        assert_eq!(refused(sel(&[], &[], &[gate("never")], &[])), 2);
        assert_eq!(refused(sel(&[], &[], &[], &[gate("nope")])), 2);
        assert_eq!(refused(sel(&[], &[], &[gate("one")], &[gate("one")])), 2);
        assert_eq!(refused(sel(&[], &[], &[gate("three")], &[])), 2, "an unselected kit's gate resolved");
        let fine = sel(&["c-kit"], &["b-kit"], &[gate("mine"), gate("two")], &[gate("one")]);
        assert!(check(&dir, &root, &kits, &fine).is_ok());
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/SPEC.md §Selecting kits and gates — an added gate outranks a recipe's drop,
    // a dropped one leaves the registry, and an added one lands once, after the derived members
    #[test]
    fn the_registry_loses_the_dropped_and_gains_the_added() {
        let (one, two, three) = (gate("one"), gate("two"), gate("three"));
        let registry = format!("# h\n# a-kit\n{}\n{}\n", one, two);
        let recipe_drops: BTreeSet<String> = [two.clone()].into_iter().collect();
        let s = Selection {
            with_gates: vec![two.clone(), three.clone(), three.clone()],
            without_gates: vec![one.clone()],
            ..Default::default()
        };
        let drop = dropped(&recipe_drops, &s);
        assert_eq!(drop.into_iter().collect::<Vec<_>>(), vec![one.clone()]);
        let text = adjust(&registry, &dropped(&recipe_drops, &s), &s);
        assert_eq!(text, format!("# h\n# a-kit\n{}\n# added by --with-gate\n{}\n", two, three));
        assert_eq!(adjust(&registry, &BTreeSet::new(), &Selection::default()), registry);
    }
}
