use std::fs;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::process::Command;

pub fn every_site(p: &str, fd: i32, mode: u32) {
    let _ = fs::Permissions::from_mode(0o755);
    let _ = fs::Permissions::from_raw_mode(0o700);
    let _ = fs::OpenOptions::new().mode(0o711);
    let mut perms = fs::metadata(p).unwrap().permissions();
    perms.set_mode(mode);
    let _ = fs::set_permissions(p, perms);
    unsafe { libc::chmod(p.as_ptr().cast(), 0o755) };
    unsafe { libc::fchmodat(fd, p.as_ptr().cast(), 0o755, 0) };
    unsafe { libc::open(p.as_ptr().cast(), libc::O_CREAT, 0o777) };
    unsafe { libc::creat(p.as_ptr().cast(), 0o755) };
    let _ = fs::copy(p, "out");
    let _ = Command::new("chmod").arg("+x").arg(p);
    let _ = libc::S_IXUSR;
    let _ = std::fs ::
        copy(p, "spaced");
    let _ = std::fs::copy:: <&str, &str>(p, "after");
    let _ = std::fs::copy ::<&str, &str>(p, "before");
}
