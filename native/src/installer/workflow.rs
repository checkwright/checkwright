// spec: installer/SPEC.md §What init seeds — the CI workflow seed: one job calling the CI action at
// the package's own repository and pack-stamped commit, written only where absent

pub const PATH: &str = ".github/workflows/gates.yml";

const CHECKOUT: &str = "actions/checkout@fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09 # v5.1.0";

// spec: installer/SPEC.md §What init seeds — the `<owner>/<repo>` a GitHub `repository` URL names,
// in any spelling npm accepts for one; anything else names no action a runner can fetch
pub fn slug(url: &str) -> Option<String> {
    let u = url.trim();
    let u = u.strip_prefix("git+").unwrap_or(u);
    let rest = [
        "https://github.com/",
        "http://github.com/",
        "git://github.com/",
        "ssh://git@github.com/",
        "git@github.com:",
        "github:",
    ]
    .iter()
    .find_map(|p| u.strip_prefix(p))?;
    let rest = rest.trim_end_matches('/');
    let rest = rest.strip_suffix(".git").unwrap_or(rest);
    let mut parts = rest.split('/');
    let (owner, repo) = (parts.next()?, parts.next()?);
    let ok = |s: &str| {
        !s.is_empty()
            && !s.starts_with('.')
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
    };
    if parts.next().is_some() || !ok(owner) || !ok(repo) {
        return None;
    }
    Some(format!("{}/{}", owner, repo))
}

fn stamped(commit: &str) -> bool {
    commit.len() == 40 && commit.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

// spec: installer/SPEC.md §What init seeds — the seed's body, or why the package can seed none
pub fn body(repository: &str, commit: &str, version: &str, branch: Option<&str>) -> Result<String, String> {
    let Some(slug) = slug(repository) else {
        return Err(format!(
            "the package's repository ({}) is not a GitHub URL, so the action step would name nothing a runner can fetch",
            if repository.is_empty() { "absent" } else { repository }
        ));
    };
    if !stamped(commit) {
        return Err("the package carries no stamped commit, so the action step would pin nothing".to_string());
    }
    let push = match branch {
        Some(b) => format!("  push:\n    branches: ['{}']\n", b.replace('\'', "''")),
        None => "  push:\n".to_string(),
    };
    Ok(format!(
        "# Written once by 'checkwright init': the gate battery on every push and pull request, at the\n\
         # release checkwright.lock names, with each red marked on the pull request. It is yours to extend.\n\
         name: gates\n\
         \n\
         on:\n\
         {push}  pull_request:\n\
         \n\
         jobs:\n\
         \x20 gates:\n\
         \x20   runs-on: ubuntu-latest\n\
         \x20   permissions:\n\
         \x20     contents: read\n\
         \x20     security-events: write\n\
         \x20   steps:\n\
         \x20     - uses: {checkout}\n\
         \x20       with:\n\
         \x20         fetch-depth: 0\n\
         \x20     - uses: {slug}/installer@{commit} # v{version}\n",
        push = push,
        checkout = CHECKOUT,
        slug = slug,
        commit = commit,
        version = version,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

    // spec: installer/SPEC.md §What init seeds — every GitHub spelling of `repository` yields the
    // slug, and a foreign host or a malformed path yields none
    #[test]
    fn the_slug_is_read_from_every_github_spelling_and_nothing_else() {
        for u in [
            "git+https://github.com/o/r.git",
            "https://github.com/o/r",
            "https://github.com/o/r/",
            "git://github.com/o/r.git",
            "git+ssh://git@github.com/o/r.git",
            "git@github.com:o/r.git",
            "github:o/r",
        ] {
            assert_eq!(slug(u).as_deref(), Some("o/r"), "{}", u);
        }
        for u in ["", "https://gitlab.com/o/r", "https://github.com/o", "https://github.com/o/r/x", "github:o/r s"] {
            assert_eq!(slug(u), None, "{}", u);
        }
    }

    // spec: installer/SPEC.md §What init seeds — the action step names the slug, the full commit
    // and the version as a trailing comment; a detached HEAD triggers on every branch
    #[test]
    fn the_seed_pins_the_action_and_triggers_on_the_branch() {
        let b = body("git+https://github.com/o/r.git", SHA, "1.2.3", Some("main")).unwrap();
        assert!(b.contains(&format!("      - uses: o/r/installer@{} # v1.2.3\n", SHA)), "{}", b);
        assert!(b.contains("    branches: ['main']\n"));
        assert!(b.contains("      contents: read\n      security-events: write\n"));
        assert!(!b.contains("run:"));
        let d = body("github:o/r", SHA, "1.2.3", None).unwrap();
        assert!(d.contains("on:\n  push:\n  pull_request:\n"), "{}", d);
        let q = body("github:o/r", SHA, "1.2.3", Some("it's")).unwrap();
        assert!(q.contains("['it''s']"));
    }

    // spec: installer/SPEC.md §What init seeds — a foreign repository or an unstamped commit seeds
    // nothing, and the reason says which
    #[test]
    fn a_foreign_repository_or_an_unstamped_commit_seeds_nothing() {
        assert!(body("https://gitlab.com/o/r", SHA, "1", None).unwrap_err().contains("not a GitHub URL"));
        assert!(body("github:o/r", "", "1", None).unwrap_err().contains("no stamped commit"));
        assert!(body("github:o/r", "abc", "1", None).is_err());
    }
}
