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
