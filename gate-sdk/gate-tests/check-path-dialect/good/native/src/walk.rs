// The crate's crosser. The three Rust producer forms live here and nowhere else, which is the
// monopoly this arm of the gate asserts.
pub fn cwd() -> String {
    let d = std::env::current_dir().expect("cwd");
    d.display().to_string()
}

pub fn toplevel_args() -> Vec<&'static str> {
    vec!["rev-parse", "--show-toplevel"]
}

pub fn canonical(p: &str) -> bool {
    std::fs::canonicalize(p).is_ok()
}

// Standing guard for the locality arm: the text-level primitives are spelled here, in the
// speller's own body, and are counted as the speller's rather than reported.
pub fn path_root(p: &str) -> bool {
    p.starts_with('/')
}

pub fn under(parent: &str, p: &str) -> bool {
    p.starts_with(&format!("{}/", parent))
}
