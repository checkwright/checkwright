// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-amendment-age: age of the oldest amendment on disk
use super::{na, now_epoch, Ctx};
use crate::proc;

const LABEL: &str = "amendment age";

// spec: drift-kit/SPEC.md §Bundled KPIs — `(^|/)SPEC-[^/]*\.md$` less the `gate-tests/` and
// `templates/` path segments: a fixture amendment and a shipped template are not amendments on disk.
pub fn is_amendment(path: &str) -> bool {
    let base = path.rsplit('/').next().unwrap_or(path);
    if !(base.starts_with("SPEC-") && base.ends_with(".md")) {
        return false;
    }
    !(path.contains("/gate-tests/") || path.contains("/templates/"))
}

fn git_lines(args: &[&str]) -> Vec<String> {
    proc::run("git", args)
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
        .map(|s| s.lines().map(String::from).collect())
        .unwrap_or_default()
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let _ = ctx;
    let amends: Vec<String> = git_lines(&["ls-files"])
        .into_iter()
        .filter(|f| is_amendment(f))
        .collect();
    if amends.is_empty() {
        return na("lead", LABEL, "no amendment on disk", trend);
    }

    let now = now_epoch();
    let mut oldest_ts = now;
    let mut oldest_file = String::new();
    for f in &amends {
        let ts = git_lines(&[
            "log",
            "--diff-filter=A",
            "--follow",
            "--format=%at",
            "--",
            f,
        ])
        .pop()
        .and_then(|l| l.trim().parse::<i64>().ok());
        if let Some(ts) = ts {
            if ts < oldest_ts {
                oldest_ts = ts;
                oldest_file = f.clone();
            }
        }
    }
    if oldest_file.is_empty() {
        return na("lead", LABEL, "no add-date resolvable", trend);
    }

    let days = (now - oldest_ts) / 86400;
    if trend {
        return Some(format!("amend {}d\n", days));
    }
    let base = oldest_file.rsplit('/').next().unwrap_or(&oldest_file);
    Some(format!("lead\t{}\toldest {}d ({})\n", LABEL, days, base))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — an amendment is a `SPEC-*.md` outside the two
    // excluded segments; the fixture and template exclusions are what keep a shipped example out
    #[test]
    fn a_fixture_or_template_spec_file_is_not_an_amendment_on_disk() {
        assert!(is_amendment("drift-kit/SPEC-kpi-port.md"));
        assert!(is_amendment("SPEC-root.md"));
        assert!(!is_amendment("gate-sdk/gate-tests/x/SPEC-fix.md"));
        assert!(!is_amendment("canon-kit/templates/SPEC-amendment.md"));
        assert!(!is_amendment("drift-kit/SPEC.md"));
        assert!(!is_amendment("drift-kit/SPEC-kpi-port.md.bak"));
    }
}
