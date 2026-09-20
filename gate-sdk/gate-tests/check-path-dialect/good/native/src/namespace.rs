// Two values that are not filesystem locations, each declared out of the contract at its own site.
pub fn horizon_matches(field: &str, horizon: &str) -> bool {
    // path-dialect-exempt: a queue tag's `horizon/track` field — the `/` is the tag grammar's
    // field separator rather than a path separator
    field.starts_with(&format!("{}/", horizon))
}

pub fn root_relative(target: &str) -> bool {
    // path-dialect-exempt: a markdown link target, a location in the published site's namespace
    target.starts_with('/')
}
