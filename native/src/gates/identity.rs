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

// spec: gate-sdk/SPEC.md §check-identity — the graded reading of the CLI's persisted hosts
// file: absent is clean with the caveat in the clean line, unreadable is fail-closed, and a
// present file carrying no block for the configured host is a violation
enum GhRead {
    Absent(String),
    Unreadable(String),
    NoHostBlock,
    Login(String),
}

// spec: gate-sdk/SPEC.md §check-identity — the default derivation, and its refusal where there
// is no `HOME` to stand on
fn gh_hosts_path(knob: &str) -> Result<String, String> {
    if !knob.is_empty() {
        return Ok(knob.to_string());
    }
    let dir = |v: &str| std::env::var(v).ok().filter(|s| !s.is_empty());
    if let Some(d) = dir("GH_CONFIG_DIR") {
        return Ok(format!("{}/hosts.yml", d.trim_end_matches('/')));
    }
    if let Some(x) = dir("XDG_CONFIG_HOME") {
        return Ok(format!("{}/gh/hosts.yml", x.trim_end_matches('/')));
    }
    let home = dir("HOME").ok_or_else(|| {
        "HOME is unset and neither GH_CONFIG_DIR nor XDG_CONFIG_HOME is set — the GitHub CLI \
         hosts file cannot be derived; treating as failure (not clean)"
            .to_string()
    })?;
    Ok(format!("{}/.config/gh/hosts.yml", home.trim_end_matches('/')))
}

fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

// spec: gate-sdk/SPEC.md §check-identity — the key token is what precedes the first `:`,
// compared whole; the value is the remainder less one layer of matching quotes
fn key_value(line: &str) -> Option<(&str, &str)> {
    let t = line.trim_end();
    let at = t.find(':')?;
    let key = t[..at].trim();
    let mut val = t[at + 1..].trim();
    for q in ['"', '\''] {
        if val.len() >= 2 && val.starts_with(q) && val.ends_with(q) {
            val = &val[1..val.len() - 1];
            break;
        }
    }
    Some((key, val))
}

// spec: gate-sdk/SPEC.md §check-identity — the active account is the value of the key spelled
// exactly `user` at the host block's own key indent, which is what keeps the sibling `users`
// map and any login spelled `user` inside it from being read as the active account
fn gh_active_account(path: &str, host: &str) -> GhRead {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return GhRead::Absent(path.to_string()),
        Err(e) => {
            return GhRead::Unreadable(format!(
                "{} is present but not readable ({}) — the check could not run; treating as \
                 failure (not clean)",
                path, e
            ))
        }
    };
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let mut found_block = false;
    let mut in_block = false;
    let mut child_indent: Option<usize> = None;
    let mut login: Option<String> = None;
    for raw in text.lines() {
        let t = raw.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if indent_of(raw) == 0 {
            if in_block {
                break;
            }
            if key_value(raw).map(|(k, _)| k == host).unwrap_or(false) {
                found_block = true;
                in_block = true;
            }
            continue;
        }
        if !in_block {
            continue;
        }
        if indent_of(raw) != *child_indent.get_or_insert(indent_of(raw)) {
            continue;
        }
        if let Some((k, v)) = key_value(raw) {
            if k == "user" {
                login = Some(v.to_string());
                break;
            }
        }
    }
    if !found_block {
        return GhRead::NoHostBlock;
    }
    match login.filter(|l| !l.is_empty()) {
        Some(l) => GhRead::Login(l),
        // spec: gate-sdk/SPEC.md §check-identity — an unrecognized shape is fail-closed, which
        // is the one posture that keeps a format change from silently retiring the assertion
        None => GhRead::Unreadable(format!(
            "{} carries a '{}' block with no active-account key — the GitHub CLI config shape is \
             not the one this gate reads; treating as failure (not clean)",
            path, host
        )),
    }
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
    let hosts_knob = match knob("GATE_SDK_GH_HOSTS_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-identity: {}", e);
            return 2;
        }
    };
    let gh_host = match knob("GATE_SDK_GH_HOST") {
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
    let mut unreadable: Vec<String> = Vec::new();
    let mut unverified: Vec<String> = Vec::new();
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
            // spec: gate-sdk/SPEC.md §check-identity — the third kind: the CLI's *persisted*
            // active account for the configured host, matched by exact string
            Some("gh-account") => {
                if f.len() != 2 {
                    malformed.push(line);
                    continue;
                }
                let path = match gh_hosts_path(&hosts_knob) {
                    Ok(p) => p,
                    Err(e) => {
                        unreadable.push(e);
                        continue;
                    }
                };
                match gh_active_account(&path, &gh_host) {
                    GhRead::Login(act) => {
                        checked += 1;
                        if act != f[1] {
                            mismatches.push(format!(
                                "gh-account: manifest expects '{}', the GitHub CLI's persisted \
                                 active account for {} is '{}'",
                                f[1], gh_host, act
                            ));
                        }
                    }
                    GhRead::NoHostBlock => {
                        checked += 1;
                        mismatches.push(format!(
                            "gh-account: manifest expects '{}', but {} carries no block for {} \
                             — this machine is not logged in to that host at all",
                            f[1], path, gh_host
                        ));
                    }
                    GhRead::Unreadable(why) => unreadable.push(why),
                    GhRead::Absent(p) => unverified.push(format!(
                        "no GitHub CLI hosts file at {}, so a switched account cannot be \
                         detected on this clone",
                        p
                    )),
                }
            }
            _ => malformed.push(line),
        }
    }

    if !malformed.is_empty() {
        eprintln!(
            "check-identity: malformed line(s) in {} (expected 'email <addr>', 'remote-host <remote> <host>' or 'gh-account <login>'):",
            manifest
        );
        for m in &malformed {
            eprintln!("  {}", m);
        }
        return 2;
    }

    // spec: gate-sdk/SPEC.md §check-identity — a surface that exists and cannot be read is
    // fail-closed: a clean there is a false clean on the one condition the kind exists to catch
    if !unreadable.is_empty() {
        eprintln!("check-identity: an actual could not be read:");
        for u in &unreadable {
            eprintln!("  {}", u);
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
        println!("        core.sshCommand/remote URL) or switch the GitHub CLI back to the");
        println!("        expected account, or — if the expectation itself moved —");
        println!("        update the matching line in {}.", manifest);
        return 1;
    }

    // spec: gate-sdk/SPEC.md §check-identity — the fail-open caveat is named *in the clean
    // line*, the shape context-kit/SPEC.md §check-memory-off takes: an absent machine surface
    // proves nothing, and a clean that does not say so reads as a verified account
    let caveat = if unverified.is_empty() {
        String::new()
    } else {
        format!(
            "; {} expectation(s) unverified — {}",
            unverified.len(),
            unverified.join("; ")
        )
    };
    println!(
        "IDENTITY: clean ({} expectation(s) match this clone's identity in {}{})",
        checked, manifest, caveat
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

    // spec: gate-sdk/SPEC.md §check-identity — the collision the parse is ruled over: `users`
    // is the map of accounts available here, and a startswith match on `user` reads its header
    // as a login. The map is written first by the CLI, so a prefix match hits it first.
    fn hosts_fixture() -> String {
        [
            "host.example:",
            "    users:",
            "        alpha:",
            "            oauth_token: xxx",
            "        user:",
            "            oauth_token: yyy",
            "    git_protocol: ssh",
            "    user: alpha",
            "other.example:",
            "    user: beta",
            "",
        ]
        .join("\n")
    }

    // spec: gate-sdk/SPEC.md §check-identity — the actual is a path, so a case writes one; the
    // counter keeps concurrently-running cases off each other's sandbox
    fn read(text: &str, host: &str) -> GhRead {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static N: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "cw-identity-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("hosts.yml");
        std::fs::write(&p, text).unwrap();
        let got = gh_active_account(p.to_str().unwrap(), host);
        std::fs::remove_dir_all(&dir).ok();
        got
    }

    #[test]
    fn the_active_account_is_the_exact_user_key_at_the_blocks_own_indent() {
        match read(&hosts_fixture(), "host.example") {
            GhRead::Login(l) => assert_eq!(l, "alpha"),
            _ => panic!("expected the block's own `user` value, not the `users` map header"),
        }
        match read(&hosts_fixture(), "other.example") {
            GhRead::Login(l) => assert_eq!(l, "beta"),
            _ => panic!("expected the second block to be read on its own"),
        }
    }

    // spec: gate-sdk/SPEC.md §check-identity — the graded postures: absent is clean-with-caveat
    // at the caller, a shape yielding no active-account key is fail-closed, and a file with no
    // block for the configured host is the violation the manifest's claim contradicts
    #[test]
    fn the_absence_postures_are_graded_rather_than_binary() {
        assert!(matches!(
            gh_active_account("/nonexistent/checkwright/hosts.yml", "host.example"),
            GhRead::Absent(_)
        ));
        assert!(matches!(
            read(&hosts_fixture(), "absent.example"),
            GhRead::NoHostBlock
        ));
        assert!(matches!(
            read("host.example:\n    git_protocol: ssh\n", "host.example"),
            GhRead::Unreadable(_)
        ));
        assert!(matches!(
            read("host.example:\n    user:\n", "host.example"),
            GhRead::Unreadable(_)
        ));
    }

    #[test]
    fn a_quoted_login_reads_as_its_unquoted_value() {
        match read("host.example:\n    user: \"alpha\"\n", "host.example") {
            GhRead::Login(l) => assert_eq!(l, "alpha"),
            _ => panic!("expected one layer of matching quotes stripped"),
        }
    }
}
