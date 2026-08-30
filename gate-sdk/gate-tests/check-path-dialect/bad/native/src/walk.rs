// Standing guard: the crosser's own body stays green beside the violation in producer.rs.
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
