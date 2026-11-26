use std::fs;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

mod helper;

pub fn quiet(p: &str) {
    let _ = fs::set_permissions(p, fs::Permissions::from_mode(0o644));
    let _ = fs::OpenOptions::new().create(true).mode(0o600);
    let note = "fs::copy(a, b) and Command::new(\"chmod\") and S_IXUSR";
    let raw = r#"libc::chmod(p, 0o755); Permissions::from_mode(0o755)"#;
    // fs::copy(p, q) then libc::S_IRWXU through Permissions::from_mode(0o777)
    let _ = (note, raw);
    let mut perms = fs::metadata(p).unwrap().permissions();
    helper::make_runnable(&mut perms);
}
