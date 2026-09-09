// spec: installer/README.md §Profiles — the crate's owner of the profile rosters: it parses
// `profiles.list`, derives `full` from the payload rather than reading a row, and is the one place
// a profile name resolves to a kit set.
use super::recipe;
use std::path::Path;

// spec: installer/README.md §Profiles — the derived profile is never a row in `profiles.list`; a
// hand-maintained "all the kits" roster is drift the day a kit lands.
pub const DERIVED: &str = "full";

pub fn payload_kits(installer: &Path) -> Vec<String> {
    crate::walk::list_dir(&installer.join("payload"))
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, is_dir)| *is_dir)
        .map(|(name, _)| name)
        .collect()
}

// spec: installer/README.md §Profiles — the `<profile><TAB><kit>` rows, comments and blanks
// dropped; a `#` anywhere on a line ends it, which is what lets a row carry a trailing note.
fn rows(installer: &Path) -> Vec<(String, String)> {
    let Ok(text) = std::fs::read_to_string(installer.join("profiles.list")) else {
        return Vec::new();
    };
    text.lines()
        .filter_map(|line| {
            let body = match line.find('#') {
                Some(i) => &line[..i],
                None => line,
            };
            if body.trim().is_empty() {
                return None;
            }
            let (p, k) = body.split_once('\t')?;
            Some((p.trim().to_string(), k.trim().to_string()))
        })
        .collect()
}

// spec: installer/README.md §Profiles — every selectable profile, the derived one last.
pub fn names(installer: &Path) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (p, _) in rows(installer) {
        if !p.is_empty() && !out.contains(&p) {
            out.push(p);
        }
    }
    out.push(DERIVED.to_string());
    out
}

// spec: installer/README.md §Profiles — emit in payload order so a roster's line order never
// decides install order.
pub fn kits(installer: &Path, want: &str) -> Vec<String> {
    if want == DERIVED {
        return payload_kits(installer);
    }
    let members: Vec<String> = rows(installer)
        .into_iter()
        .filter(|(p, _)| p == want)
        .map(|(_, k)| k)
        .collect();
    if members.is_empty() {
        return Vec::new();
    }
    payload_kits(installer)
        .into_iter()
        .filter(|k| members.contains(k))
        .collect()
}

pub fn known(installer: &Path, want: &str) -> bool {
    names(installer).iter().any(|n| n == want)
}

// spec: installer/README.md §Profiles — what an adopter meets is the battery, not the directory
// list, so the gate set is a derivation in its own right: one function answers it for the registry
// init writes and for the smoke's monotonicity assertion, rather than each unioning it itself.
pub fn gate_set(installer: &Path, profile: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for kit in kits(installer, profile) {
        out.extend(recipe::gates(&installer.join("payload").join(&kit), profile));
    }
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/README.md §Profiles — the derived profile is every payload kit and is never a
    // row; a declared profile emits in payload order rather than in row order; an unknown one
    // resolves to nothing.
    #[test]
    fn the_derived_profile_is_the_payload_and_a_declared_one_emits_in_payload_order() {
        let dir = std::env::temp_dir().join(format!("cw-profile-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        for k in ["a-kit", "b-kit", "c-kit"] {
            std::fs::create_dir_all(dir.join("payload").join(k)).expect("cannot make the tree");
        }
        std::fs::write(
            dir.join("profiles.list"),
            "# a comment\n\nsmall\tc-kit\t# trailing note\nsmall\ta-kit\n",
        )
        .expect("cannot write the scratch roster");

        assert_eq!(names(&dir), vec!["small".to_string(), DERIVED.to_string()]);
        assert_eq!(
            kits(&dir, DERIVED),
            vec!["a-kit".to_string(), "b-kit".to_string(), "c-kit".to_string()]
        );
        assert_eq!(
            kits(&dir, "small"),
            vec!["a-kit".to_string(), "c-kit".to_string()],
            "the row order decided the install order"
        );
        assert!(kits(&dir, "nope").is_empty());
        assert!(known(&dir, DERIVED) && known(&dir, "small") && !known(&dir, "nope"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
