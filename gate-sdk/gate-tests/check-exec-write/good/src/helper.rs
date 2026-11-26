use std::os::unix::fs::PermissionsExt;

pub fn make_runnable(perms: &mut std::fs::Permissions) {
    perms.set_mode(0o755);
}
