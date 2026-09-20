// Four locality findings: a leading-separator test, std::path's own absoluteness answer, a bare
// trailing-slash prefix composition, and a declaration whose reason is empty.
pub fn rooted(p: &str) -> bool {
    p.starts_with('/')
}

pub fn absolute(p: &std::path::Path) -> bool {
    p.is_absolute()
}

pub fn inside(root: &str, p: &str) -> bool {
    let prefix = format!("{}/", root);
    p.starts_with(&prefix)
}

pub fn under_checks(kit: &str, p: &str) -> bool {
    // path-dialect-exempt:
    p.starts_with(&format!("{}/{}", kit, "checks"))
}
