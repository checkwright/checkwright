// spec: gate-sdk/SPEC.md §check-identity — every expectation in the identity manifest matches
// this clone's local git identity
use crate::fresh;
use crate::proc;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gates_list_members`: every line that is neither blank
// nor a comment, in file order
fn members(text: &str) -> Vec<String> {
    fresh::file_lines(text)
        .iter()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .map(|l| (*l).to_string())
        .collect()
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — `$(…)`'s own trim: trailing newlines and
// nothing else, so a value carrying real trailing whitespace crosses unchanged
fn captured(out: &[u8]) -> String {
    String::from_utf8_lossy(out).trim_end_matches('\n').to_string()
}

fn first_line(path: &str) -> Option<String> {
    let text = std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .ok()?;
    Some(text.lines().next().unwrap_or("").to_string())
}

// spec: gate-sdk/SPEC.md §check-identity — the host part is the SSH alias / hostname that
// selects the identity; parse scp-like and scheme:// URL forms
fn extract_host(url: &str) -> String {
    if let Some(rest) = url.split_once("://").map(|(_, r)| r) {
        let rest = rest.split_once('@').map(|(_, r)| r).unwrap_or(rest);
        let rest = rest.split('/').next().unwrap_or("");
        return rest.split(':').next().unwrap_or("").to_string();
    }
    if let Some((head, _)) = url.split_once(':') {
        return head.split_once('@').map(|(_, r)| r).unwrap_or(head).to_string();
    }
    String::new()
}

fn actual_email(email_file: &str) -> String {
    if !email_file.is_empty() {
        return first_line(email_file).unwrap_or_default();
    }
    proc::run("git", &["config", "user.email"])
        .ok()
        .and_then(|c| c.stdout().map(captured))
        .unwrap_or_default()
}

// spec: gate-sdk/SPEC.md §check-identity — a configured remote absent from this clone is red;
// `None` signals the missing remote to the comparison below
fn actual_remote_url(remote: &str, remotes_file: &str) -> Option<String> {
    if !remotes_file.is_empty() {
        let text = std::fs::read(remotes_file)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .ok()?;
        for line in members(&text) {
            let mut f = line.split_whitespace();
            if f.next() == Some(remote) {
                return Some(f.next().unwrap_or("").to_string());
            }
        }
        return None;
    }
    proc::run("git", &["remote", "get-url", remote])
        .ok()?
        .stdout()
        .map(captured)
}

pub fn run(_args: &[String]) -> i32 {
    let knob = |n: &str| walk::knob_scalar(n);
    let manifest = match knob("GATE_SDK_IDENTITY_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-identity: {}", e);
            return 2;
        }
    };
    let email_file = match knob("GATE_SDK_GIT_EMAIL_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-identity: {}", e);
            return 2;
        }
    };
    let remotes_file = match knob("GATE_SDK_GIT_REMOTES_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-identity: {}", e);
            return 2;
        }
    };

    if !Path::new(&manifest).exists() {
        println!(
            "IDENTITY: clean (no manifest at {} — optional consumer config absent)",
            manifest
        );
        return 0;
    }
    let text = match std::fs::read(&manifest) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => {
            eprintln!("check-identity: manifest not readable: {}", manifest);
            return 2;
        }
    };

    // spec: gate-sdk/SPEC.md §check-identity — the live-read preconditions bind only where an
    // actual is still read from the clone: with both actual-source knobs configured the run is a
    // configured read, not a self-check of a committing clone
    let reads_clone = email_file.is_empty() || remotes_file.is_empty();
    if reads_clone {
        // spec: gate-sdk/SPEC.md §check-identity — CI is not a committing clone (no local
        // identity to misattribute a commit/push with), so the guard steps aside
        if std::env::var("CI").map(|v| !v.is_empty()).unwrap_or(false) {
            println!("IDENTITY: clean (CI context — not a committing clone; identity guard skipped)");
            return 0;
        }
        let in_repo = proc::run("git", &["rev-parse", "--git-dir"])
            .ok()
            .and_then(|c| c.stdout().map(|_| ()))
            .is_some();
        if !in_repo {
            eprintln!("check-identity: not a git repository — cannot verify identity");
            return 2;
        }
    }

    let mut malformed: Vec<String> = Vec::new();
    let mut mismatches: Vec<String> = Vec::new();
    let mut checked = 0usize;
    for line in members(&text) {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f.first().copied() {
            Some("email") => {
                if f.len() != 2 {
                    malformed.push(line);
                    continue;
                }
                checked += 1;
                let act = actual_email(&email_file);
                if act != f[1] {
                    mismatches.push(format!(
                        "email: manifest expects '{}', clone has '{}'",
                        f[1],
                        if act.is_empty() { "<unset>" } else { act.as_str() }
                    ));
                }
            }
            Some("remote-host") => {
                if f.len() != 3 {
                    malformed.push(line);
                    continue;
                }
                checked += 1;
                match actual_remote_url(f[1], &remotes_file) {
                    Some(url) => {
                        let host = extract_host(&url);
                        if host != f[2] {
                            mismatches.push(format!(
                                "remote-host {}: manifest expects '{}', clone has '{}' (url: {})",
                                f[1],
                                f[2],
                                if host.is_empty() { "<unparseable>" } else { host.as_str() },
                                url
                            ));
                        }
                    }
                    None => mismatches.push(format!(
                        "remote-host {}: no such remote in this clone",
                        f[1]
                    )),
                }
            }
            _ => malformed.push(line),
        }
    }

    if !malformed.is_empty() {
        eprintln!(
            "check-identity: malformed line(s) in {} (expected 'email <addr>' or 'remote-host <remote> <host>'):",
            manifest
        );
        for m in &malformed {
            eprintln!("  {}", m);
        }
        return 2;
    }

    if !mismatches.is_empty() {
        println!(
            "check-identity: local git identity does not match {}:",
            manifest
        );
        for m in &mismatches {
            println!("  {}", m);
        }
        println!("  help: this clone commits/pushes under the wrong identity — fix the git");
        println!("        mapping (user.email via includeIf, the remote's SSH host alias via");
        println!("        core.sshCommand/remote URL), or — if the expectation itself moved —");
        println!("        update the matching line in {}.", manifest);
        return 1;
    }

    println!(
        "IDENTITY: clean ({} expectation(s) match this clone's git identity in {})",
        checked, manifest
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-identity — the host is the alias that selects the identity,
    // across both URL spellings a remote can carry
    #[test]
    fn the_host_is_parsed_out_of_both_url_forms() {
        assert_eq!(extract_host("git@alias:owner/repo.git"), "alias");
        assert_eq!(extract_host("alias:owner/repo.git"), "alias");
        assert_eq!(extract_host("ssh://git@host.example/owner/repo"), "host.example");
        assert_eq!(extract_host("https://host.example:443/owner/repo"), "host.example");
        assert_eq!(extract_host("/srv/local/repo.git"), "");
    }
}
