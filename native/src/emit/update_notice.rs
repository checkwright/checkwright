// spec: installer/SPEC.md §The update notice — one line when the upstream's newest release tag is
// above the release the lock records, probed at most once per interval, silent on any failure
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Emit`: its exit grammar is the collapse, 0
// always and 2 for a usage error, and it declares the three knobs it reads
use crate::installer::lock;
use crate::{proc, programs, toolfloor};
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &["GATE_SDK_UPDATE_CHECK", "GATE_SDK_UPDATE_UPSTREAM", "GATE_SDK_UPDATE_TIMEOUT"];

pub const CACHE_FILE: &str = "checkwright-update-check";

const DEFAULT_TIMEOUT: u64 = 5;

pub struct Settings {
    pub interval: Option<u64>,
    pub interval_word: String,
    pub upstream: String,
    pub timeout: u64,
}

impl Settings {
    // spec: installer/SPEC.md §The update notice — an unresolvable interval reads as `off`, so a
    // refused knob never probes; an unparseable timeout takes the default
    pub fn resolve(read: impl Fn(&str) -> Result<String, String>) -> Settings {
        let word = read("GATE_SDK_UPDATE_CHECK").unwrap_or_default();
        let interval = match word.trim() {
            "daily" => Some(86_400),
            "weekly" => Some(604_800),
            _ => None,
        };
        Settings {
            interval_word: if interval.is_some() { word.trim().to_string() } else { "off".to_string() },
            interval,
            upstream: read("GATE_SDK_UPDATE_UPSTREAM").unwrap_or_default().trim().to_string(),
            timeout: read("GATE_SDK_UPDATE_TIMEOUT")
                .ok()
                .and_then(|t| t.trim().parse::<u64>().ok())
                .filter(|t| *t > 0)
                .unwrap_or(DEFAULT_TIMEOUT),
        }
    }

    pub fn on(&self) -> bool {
        self.interval.is_some() && !self.upstream.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub enum Reading {
    Off,
    NoUpstream,
    NoInstall,
    Newer { newest: String, installed: String },
    Current { newest: String },
    Unknown,
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    let settings = Settings::resolve(crate::walk::knob_scalar);
    let Ok(root) = crate::walk::toplevel() else {
        return Ok(String::new());
    };
    Ok(line(&reading(Path::new(&root), &settings, now())))
}

// spec: installer/SPEC.md §The update notice — the line, printed for a newer release alone
pub fn line(r: &Reading) -> String {
    match r {
        Reading::Newer { newest, installed } => format!(
            "checkwright v{} is available; this tree has v{}. Run update to upgrade, or set GATE_SDK_UPDATE_CHECK = off to stop this check.\n",
            newest, installed
        ),
        _ => String::new(),
    }
}

pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// spec: installer/SPEC.md §The update notice — the cache lives in the clone's common git directory,
// shared by every worktree and outside the scratch sweep; `uninstall` reclaims it
pub(crate) fn cache_path(root: &Path) -> Option<PathBuf> {
    let r = root.to_string_lossy().into_owned();
    let out = proc::run(&programs::GIT, &["-C", &r, "rev-parse", "--git-common-dir"]).ok()?;
    let dir = String::from_utf8_lossy(out.stdout()?).trim().to_string();
    if dir.is_empty() {
        return None;
    }
    Some(PathBuf::from(crate::walk::abs_against(&r, &dir)).join(CACHE_FILE))
}

// spec: installer/SPEC.md §The update notice — the four conditions in order; doctor renders every
// outcome and the arm prints only `Newer`
pub fn reading(root: &Path, s: &Settings, now: u64) -> Reading {
    let Some(interval) = s.interval else {
        return Reading::Off;
    };
    if s.upstream.is_empty() {
        return Reading::NoUpstream;
    }
    let installed = lock::Manifest::read(&lock::path(root))
        .filter(lock::Manifest::schema_ok)
        .map(|m| m.field("version"))
        .unwrap_or_default();
    if installed.is_empty() {
        return Reading::NoInstall;
    }
    let Some(cache) = cache_path(root) else {
        return Reading::Unknown;
    };
    let cached = std::fs::read_to_string(&cache)
        .ok()
        .and_then(|t| parse_cache(&t))
        .filter(|r| fresh(r.at, now, interval) && r.upstream == s.upstream);
    let found = match cached {
        Some(r) => r.newest,
        None => {
            let probed = probe(&s.upstream, s.timeout);
            let _ = std::fs::write(&cache, record(now, probed.as_deref(), &s.upstream));
            probed
        }
    };
    let Some(newest) = found else {
        return Reading::Unknown;
    };
    if toolfloor::floor_met(&newest, &installed) == Some(false) {
        Reading::Newer { newest, installed }
    } else {
        Reading::Current { newest }
    }
}

// spec: installer/SPEC.md §The update notice — one line: the attempt's time, the newest version or
// `-` for a failed attempt, and the upstream probed, so a record of another upstream is due
#[derive(Debug, PartialEq)]
struct Record {
    at: u64,
    newest: Option<String>,
    upstream: String,
}

const FAILED: &str = "-";

fn record(at: u64, newest: Option<&str>, upstream: &str) -> String {
    format!("{} {} {}\n", at, newest.unwrap_or(FAILED), upstream)
}

fn parse_cache(text: &str) -> Option<Record> {
    let line = text.lines().next()?;
    let (at, rest) = line.split_once(' ')?;
    let (v, upstream) = rest.split_once(' ')?;
    let newest = match v {
        FAILED => None,
        v => Some(release(v).map(|_| v.to_string())?),
    };
    let upstream = upstream.trim();
    (!upstream.is_empty()).then_some(())?;
    Some(Record { at: at.parse::<u64>().ok()?, newest, upstream: upstream.to_string() })
}

// spec: installer/SPEC.md §The update notice — a probe is due when the record is older than the
// interval or lies in the future
fn fresh(at: u64, now: u64, interval: u64) -> bool {
    at <= now && now - at < interval
}

// spec: installer/SPEC.md §The update notice — only `v<major>.<minor>.<patch>` counts, so a
// prerelease or build suffix is never a candidate
fn release(v: &str) -> Option<()> {
    let parts: Vec<&str> = v.split('.').collect();
    (parts.len() == 3 && parts.iter().all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))).then_some(())
}

fn newest(refs: &str) -> Option<String> {
    refs.lines()
        .filter_map(|l| l.split_whitespace().nth(1))
        .filter_map(|r| r.strip_prefix("refs/tags/v"))
        .filter(|v| release(v).is_some())
        .fold(None, |best: Option<String>, v| match best {
            Some(b) if toolfloor::floor_met(v, &b) == Some(true) => Some(b),
            _ => Some(v.to_string()),
        })
}

fn probe(upstream: &str, timeout: u64) -> Option<String> {
    let env = [("GIT_TERMINAL_PROMPT", "0")];
    let (code, out) =
        proc::run_bounded_capture(&programs::GIT, &["ls-remote", "--tags", "--refs", upstream], timeout, &env).ok()??;
    if code != 0 {
        return None;
    }
    newest(&String::from_utf8_lossy(&out))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(word: &str, upstream: &str) -> Settings {
        Settings::resolve(|k| {
            Ok(match k {
                "GATE_SDK_UPDATE_CHECK" => word.to_string(),
                "GATE_SDK_UPDATE_UPSTREAM" => upstream.to_string(),
                _ => String::new(),
            })
        })
    }

    fn git(dir: &Path, args: &[&str]) {
        let d = dir.to_string_lossy().into_owned();
        let mut argv = vec!["-C", d.as_str()];
        argv.extend_from_slice(args);
        let out = proc::run(&programs::GIT, &argv).expect("git");
        assert!(out.stdout().is_some(), "git {:?} failed", args);
    }

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("cw-update-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        d
    }

    fn installed_tree(dir: &Path, version: &str) {
        git(dir, &["init", "-q"]);
        std::fs::write(lock::path(dir), lock::Emit::new().ident("version", version).render()).expect("lock");
    }

    // spec: installer/SPEC.md §The update notice — a suffixed tag and a non-`v` tag are skipped
    #[test]
    fn only_a_plain_v_release_tag_is_a_candidate() {
        let refs = "a\trefs/tags/v1.2.3\nb\trefs/tags/v9.0.0-rc1\nc\trefs/tags/9.9.9\nd\trefs/tags/v1.2\n";
        assert_eq!(newest(refs), Some("1.2.3".to_string()));
        assert_eq!(newest("e\trefs/heads/main\n"), None);
    }

    #[test]
    fn the_newest_is_chosen_over_digit_runs() {
        let refs = "a\trefs/tags/v0.9.0\nb\trefs/tags/v0.10.0\nc\trefs/tags/v0.2.11\n";
        assert_eq!(newest(refs), Some("0.10.0".to_string()));
    }

    // spec: installer/SPEC.md §The update notice — absent, old and future records are stale, and a
    // record naming no upstream, the earlier two-field shape, is unreadable and so due
    #[test]
    fn the_staleness_rule() {
        assert_eq!(parse_cache(""), None);
        assert_eq!(parse_cache("x 1.0.0 u"), None);
        assert_eq!(parse_cache("100 1.0.0\n"), None);
        assert_eq!(parse_cache("100 1.0.0-rc1 u\n"), None);
        let up = "https://example.invalid/a b.git";
        assert_eq!(
            parse_cache(&record(100, Some("1.0.0"), up)),
            Some(Record { at: 100, newest: Some("1.0.0".to_string()), upstream: up.to_string() })
        );
        assert_eq!(
            parse_cache(&record(100, None, up)),
            Some(Record { at: 100, newest: None, upstream: up.to_string() })
        );
        assert!(fresh(100, 150, 86_400));
        assert!(!fresh(100, 100 + 86_400, 86_400));
        assert!(!fresh(200, 100, 86_400));
    }

    // spec: installer/SPEC.md §The update notice — silent on each failed condition
    #[test]
    fn the_arm_is_silent_on_each_failed_condition() {
        let dir = scratch("silent");
        assert_eq!(reading(&dir, &settings("off", "x"), 0), Reading::Off);
        assert_eq!(reading(&dir, &settings("fortnightly", "x"), 0), Reading::Off);
        assert_eq!(reading(&dir, &settings("weekly", ""), 0), Reading::NoUpstream);
        git(&dir, &["init", "-q"]);
        assert_eq!(reading(&dir, &settings("weekly", "x"), 0), Reading::NoInstall);
        installed_tree(&dir, "1.0.0");
        std::fs::write(dir.join(".git").join(CACHE_FILE), "50 2.0.0 x\n").expect("cache");
        let fresh_newer = reading(&dir, &settings("weekly", "x"), 100);
        assert!(line(&fresh_newer).starts_with("checkwright v2.0.0 is available; this tree has v1.0.0."), "{:?}", fresh_newer);
        std::fs::write(dir.join(".git").join(CACHE_FILE), "50 1.0.0 x\n").expect("cache");
        assert_eq!(line(&reading(&dir, &settings("weekly", "x"), 100)), "");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: installer/SPEC.md §The update notice — a real probe by path finds the newest tag and
    // caches it keyed on its upstream; an unreachable upstream prints nothing and records the attempt,
    // so no read inside the interval probes it again
    #[test]
    fn a_probe_reads_the_upstream_and_an_unreachable_one_is_silent() {
        let up = scratch("upstream");
        git(&up, &["init", "-q"]);
        git(&up, &["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "--allow-empty", "-m", "seed"]);
        git(&up, &["tag", "v0.1.0"]);
        git(&up, &["tag", "v9.0.0"]);
        let bare = scratch("bare");
        let _ = std::fs::remove_dir_all(&bare);
        let up_s = up.to_string_lossy().into_owned();
        let bare_s = bare.to_string_lossy().into_owned();
        assert!(proc::run(&programs::GIT, &["clone", "-q", "--bare", &up_s, &bare_s]).expect("clone").stdout().is_some());
        let tree = scratch("tree");
        installed_tree(&tree, "1.0.0");
        let printed = line(&reading(&tree, &settings("daily", &bare_s), 1_000));
        assert!(printed.starts_with("checkwright v9.0.0 is available; this tree has v1.0.0."), "{}", printed);
        let cache = tree.join(".git").join(CACHE_FILE);
        let cached = std::fs::read_to_string(&cache).expect("cache written");
        assert_eq!(cached, format!("1000 9.0.0 {}\n", bare_s));

        std::fs::write(&cache, "1000 2.0.0 another-upstream\n").expect("cache");
        let rekeyed = line(&reading(&tree, &settings("daily", &bare_s), 1_001));
        assert!(rekeyed.starts_with("checkwright v9.0.0 is available"), "{}", rekeyed);
        assert_eq!(std::fs::read_to_string(&cache).expect("cache"), format!("1001 9.0.0 {}\n", bare_s));

        let lonely = scratch("lonely");
        installed_tree(&lonely, "1.0.0");
        let gone = lonely.join("no-such-upstream");
        let gone_s = gone.to_string_lossy().into_owned();
        let lonely_cache = lonely.join(".git").join(CACHE_FILE);
        assert_eq!(reading(&lonely, &settings("weekly", &gone_s), 1_000), Reading::Unknown);
        assert_eq!(std::fs::read_to_string(&lonely_cache).expect("attempt recorded"), format!("1000 - {}\n", gone_s));
        assert!(proc::run(&programs::GIT, &["clone", "-q", "--bare", &up_s, &gone_s]).expect("clone").stdout().is_some());
        assert_eq!(reading(&lonely, &settings("weekly", &gone_s), 2_000), Reading::Unknown);
        let due = line(&reading(&lonely, &settings("weekly", &gone_s), 1_000 + 604_800));
        assert!(due.starts_with("checkwright v9.0.0 is available"), "{}", due);
        for d in [up, bare, tree, lonely] {
            let _ = std::fs::remove_dir_all(&d);
        }
    }
}
