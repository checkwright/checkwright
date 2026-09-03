// spec: guard-kit/SPEC.md §scan-prompts — the ranker: the friction log's fall-throughs split three
// ways against both settings files and the harness's read-only built-ins, the prompting share
// grouped by the ranking key. A bridged arm, because it resolves three consumer knobs.
use crate::guard;
use crate::walk;
use serde_json::Value;

// spec: guard-kit/SPEC.md §Layout and configuration — the three knobs `lib/guard.sh` defines and
// the bridge resolves by sourcing it; no default moves into the crate, so an absent guard-kit
// cannot resolve this arm at all.
pub const KNOBS: &[&str] = &[
    "GUARD_KIT_LOG",
    "GUARD_KIT_SETTINGS",
    "GUARD_KIT_SETTINGS_LOCAL",
];

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the usage a refusal prints, since the `-h`/
// `--help` half retires to the front-end and the shape half does not.
const USAGE: &str = "usage: --emit scan-prompts [--count] [--] [<log>]\n  ranks the friction log's prompting calls; \"--\" admits a log path beginning with \"-\"";

// spec: guard-kit/SPEC.md §scan-prompts — the harness's own built-in read-only auto-allows, a
// public and harness-wide vocabulary rather than any consumer's, so they are kit literals and
// mint no knob (§The generic ruleset).
const GIT_RO: &[&str] = &[
    "status", "log", "diff", "show", "blame", "branch", "tag", "remote", "ls-files", "ls-remote",
    "rev-parse", "describe", "shortlog", "cat-file", "for-each-ref", "worktree", "reflog",
];
const DOCKER_RO: &[&str] = &["ps", "images", "logs", "inspect", "version"];

// spec: guard-kit/SPEC.md §scan-prompts — the common multi-command binaries the ranking key
// sub-keys on: shell-substrate knowledge, naming no project's toolchain.
const MULTI_COMMAND: &[&str] = &[
    "git", "gh", "cargo", "docker", "npm", "bun", "yarn", "pnpm", "bash", "sh", "kubectl",
    "python", "python3",
];

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

fn trim_start(s: &str) -> &str {
    let b = s.as_bytes();
    let i = b.iter().position(|c| !is_space(*c)).unwrap_or(b.len());
    &s[i..]
}

fn trim(s: &str) -> &str {
    let s = trim_start(s);
    let b = s.as_bytes();
    let e = b.iter().rposition(|c| !is_space(*c)).map_or(0, |i| i + 1);
    &s[..e]
}

// spec: guard-kit/SPEC.md §scan-prompts — the leading decoration the matcher looks through, the
// holder's three parameter expansions in order: `sudo `, `timeout `, then a shortest `[0-9]* `,
// which is one leading digit and everything to the first space.
fn strip_decoration(c: &str) -> &str {
    let c = c.strip_prefix("sudo ").unwrap_or(c);
    let c = c.strip_prefix("timeout ").unwrap_or(c);
    match c.as_bytes().first() {
        Some(f) if f.is_ascii_digit() => match c.find(' ') {
            Some(i) => &c[i + 1..],
            None => c,
        },
        _ => c,
    }
}

fn word(s: &str) -> (&str, &str) {
    let b = s.as_bytes();
    let i = b.iter().position(|c| is_space(*c)).unwrap_or(b.len());
    (&s[..i], trim_start(&s[i..]))
}

// spec: guard-kit/SPEC.md §scan-prompts — the committed `Bash(...)` allow inners, parsed in-crate:
// the member carries no external-program dependency an absent `jq` could take, which is what keeps
// an unreadable allowlist from reading as an empty one on a machine that merely lacks a tool.
pub fn allow_inners(text: &str) -> Vec<String> {
    let doc: Value = match serde_json::from_str(text) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    doc.get("permissions")
        .and_then(|p| p.get("allow"))
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|e| e.as_str())
                .filter_map(|s| s.strip_prefix("Bash("))
                .map(|s| s.strip_suffix(')').unwrap_or(s).to_string())
                .collect()
        })
        .unwrap_or_default()
}

fn read_allow(path: &str) -> Vec<String> {
    std::fs::read(path)
        .map(|b| allow_inners(&String::from_utf8_lossy(&b)))
        .unwrap_or_default()
}

// spec: guard-kit/SPEC.md §scan-prompts — a settings glob is a bash pattern, and `:*` is the
// harness's own spelling of a trailing wildcard, so it is rewritten before the match rather than
// matched literally.
fn granted_by(c: &str, pats: &[String]) -> bool {
    pats.iter()
        .filter(|p| !p.is_empty())
        .any(|p| walk::glob_match(&p.replace(":*", "*"), c))
}

// spec: guard-kit/SPEC.md §scan-prompts — one segment granted by the committed allowlist, a
// harness read-only git/docker built-in, or — for the overlay pass — the uncommitted overlay.
fn segment_granted(seg: &str, allow: &[String], overlay: Option<&[String]>) -> bool {
    let c = trim(strip_decoration(seg));
    if c.is_empty() {
        return true;
    }
    let (t1, rest) = word(c);
    let (t2, _) = word(rest);
    if t1 == "git" && GIT_RO.contains(&t2) {
        return true;
    }
    if t1 == "docker" && DOCKER_RO.contains(&t2) {
        return true;
    }
    granted_by(c, allow) || overlay.is_some_and(|o| granted_by(c, o))
}

// spec: guard-kit/SPEC.md §scan-prompts — the quoted-span view `allowed()` matches on, which is
// deliberately not `guard_skeleton`'s: a whole quoted span collapses to one token so a separator
// inside it cannot split the command, and nothing else about the line is normalized.
fn quoted_view(cmd: &str) -> String {
    fn pass(s: &[u8], q: u8, rep: &[u8]) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(s.len());
        let mut i = 0usize;
        while i < s.len() {
            if s[i] == q {
                if let Some(k) = s[i + 1..].iter().position(|c| *c == q) {
                    out.extend_from_slice(rep);
                    i += k + 2;
                    continue;
                }
            }
            out.push(s[i]);
            i += 1;
        }
        out
    }
    let sq = pass(cmd.as_bytes(), b'\'', b"SQ");
    String::from_utf8_lossy(&pass(&sq, b'"', b"DQ")).into_owned()
}

// spec: guard-kit/SPEC.md §scan-prompts — granted only if EVERY segment is, so a whole-string glob
// spanning a compound the harness would split and refuse does not read as allowed.
fn granted(cmd: &str, allow: &[String], overlay: Option<&[String]>) -> bool {
    guard::split_compound(&quoted_view(cmd))
        .iter()
        .filter(|seg| !seg.bytes().all(|c| c == b' '))
        .all(|seg| segment_granted(seg, allow, overlay))
}

// spec: guard-kit/SPEC.md §scan-prompts — the key's write-shape suffix: the segment's own
// write-redirect operator normalized to `>` or `>>`, the descriptor dropped and an fd-dup excluded
// on rule 17's own target test, since an fd-dup is not a redirect to a file.
fn redirect_op(c: &str) -> &'static str {
    let pairs = match guard::redirect_pairs(c) {
        Ok(p) => p,
        Err(_) => return "",
    };
    for pair in pairs {
        let p = pair.trim_start_matches(|ch: char| ch.is_ascii_digit());
        let (op, tgt) = match p.strip_prefix(">>") {
            Some(t) => (">>", t),
            None => (">", p.strip_prefix('>').unwrap_or(p)),
        };
        let tgt = trim_start(tgt);
        if tgt.is_empty() || tgt.starts_with('&') {
            continue;
        }
        return op;
    }
    ""
}

// spec: guard-kit/SPEC.md §scan-prompts — a write redirect standing where a subcommand would is
// re-homed into the suffix rather than doubled into both tokens; a read redirect keys where it did.
fn is_write_token(t: &str) -> bool {
    let b = t.as_bytes();
    t.starts_with('>')
        || t.starts_with("&>")
        || (b.len() >= 2 && b[0].is_ascii_digit() && b[1] == b'>')
}

// spec: guard-kit/SPEC.md §scan-prompts — the ranking key: leading binary, plus subcommand for the
// common multi-command binaries, plus the write-shape suffix; word and suffix both come from the
// FIRST segment, so a key can never attribute a write to a command that performs none.
pub fn ranking_key(line: &str) -> String {
    let skel = match guard::skeleton(line, guard::Wants { sq: true, dq: true }) {
        Ok(s) => s,
        Err(guard::NewlineInInput) => return String::new(),
    };
    let segs = guard::split_compound(&skel);
    let c = trim_start(strip_decoration(segs.first().map_or("", String::as_str)));
    let (t1, rest) = word(c);
    let (t2, _) = word(rest);
    let mut key = if MULTI_COMMAND.contains(&t1) && !t2.is_empty() && !is_write_token(t2) {
        format!("{} {}", t1, t2)
    } else {
        t1.to_string()
    };
    let op = redirect_op(c);
    if !op.is_empty() {
        key.push(' ');
        key.push_str(op);
    }
    key
}

// spec: guard-kit/SPEC.md §scan-prompts — the three-way split's whole result: the prompting share,
// the overlay-covered share, and the logged denominator. Committed-covered is on neither ranking
// by construction — it is silently granted and reinforced.
pub struct Tally {
    pub prompting: Vec<(String, u64)>,
    pub overlay: Vec<(String, u64)>,
    pub total: u64,
    pub overlay_total: u64,
    pub logged: u64,
}

fn bump(into: &mut Vec<(String, u64)>, key: &str) {
    match into.iter_mut().find(|(k, _)| k == key) {
        Some(e) => e.1 += 1,
        None => into.push((key.to_string(), 1)),
    }
}

// spec: guard-kit/SPEC.md §scan-prompts — the ranking's order: occurrences descending, and a tie
// broken by descending key bytes, which is what the holder's `sort -rn` last-resort comparison
// under `-r` produces.
fn ranked(rows: &[(String, u64)]) -> Vec<&(String, u64)> {
    let mut out: Vec<&(String, u64)> = rows.iter().collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    out
}

pub fn tally(log_text: &str, allow: &[String], overlay: &[String]) -> Tally {
    let mut t = Tally {
        prompting: Vec::new(),
        overlay: Vec::new(),
        total: 0,
        overlay_total: 0,
        logged: log_text.bytes().filter(|b| *b == b'\n').count() as u64,
    };
    // spec: guard-kit/SPEC.md §scan-prompts — the reader is `while IFS= read -r line`, which
    // assigns an unterminated final line and returns non-zero, so the loop body never sees it.
    let mut lines: Vec<&str> = log_text.split('\n').collect();
    if !log_text.ends_with('\n') {
        lines.pop();
    }
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let key = ranking_key(line);
        if key.is_empty() {
            continue;
        }
        if granted(line, allow, None) {
            continue;
        }
        if granted(line, allow, Some(overlay)) {
            bump(&mut t.overlay, &key);
            t.overlay_total += 1;
        } else {
            bump(&mut t.prompting, &key);
            t.total += 1;
        }
    }
    t
}

fn scan(log: &str, settings: &str, settings_local: &str) -> Tally {
    let text = std::fs::read(log)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    tally(&text, &read_allow(settings), &read_allow(settings_local))
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the two integers `kpi-prompt-friction` reads, handed
// over in-crate: distinct prompting patterns and prompting calls, overlay-covered excluded.
pub fn count(log: &str, settings: &str, settings_local: &str) -> (u64, u64) {
    let t = scan(log, settings, settings_local);
    (t.prompting.len() as u64, t.total)
}

fn rank_section(rows: &[(String, u64)], out: &mut String) {
    for (key, n) in ranked(rows) {
        out.push_str(&format!("{:>5}x  {}\n", n, key));
    }
}

// spec: guard-kit/SPEC.md §scan-prompts — the overlay-covered share: a separate, visibly-advisory
// section below the headline, never mixed into it, and absent entirely when it is empty.
fn overlay_section(t: &Tally, settings_local: &str, out: &mut String) {
    if t.overlay.is_empty() {
        return;
    }
    out.push('\n');
    out.push_str("--- Overlay-covered (advisory — did NOT prompt; granted only by the uncommitted\n");
    out.push_str(&format!(
        "    local overlay {}). Promote the recurring-safe patterns to the\n",
        settings_local
    ));
    out.push_str("    committed allowlist or prune the one-offs (guard-kit/SPEC.md §The triage criterion): ---\n");
    out.push_str(&format!(
        "{} call(s) across {} pattern(s).\n",
        t.overlay_total,
        t.overlay.len()
    ));
    rank_section(&t.overlay, out);
}

fn render(log: &str, settings: &str, settings_local: &str, count_only: bool) -> String {
    // spec: guard-kit/SPEC.md §scan-prompts — an absent or empty log is not a clean tree with a
    // ranking of nothing: the count mode answers `0/0` and the report says the log is empty.
    let empty = std::fs::metadata(log).map(|m| m.len() == 0).unwrap_or(true);
    if empty {
        return if count_only {
            "0/0\n".to_string()
        } else {
            "PROMPT-FRICTION: clean (no fall-through commands logged this iteration)\n".to_string()
        };
    }
    let t = scan(log, settings, settings_local);
    if count_only {
        return format!("{}/{}\n", t.prompting.len(), t.total);
    }
    let mut out = String::new();
    if t.prompting.is_empty() {
        out.push_str(&format!(
            "PROMPT-FRICTION: clean ({} fall-through(s) logged, all allowlisted / auto-allowed)\n",
            t.logged
        ));
        overlay_section(&t, settings_local, &mut out);
        return out;
    }
    out.push_str("=== Prompt friction (advisory — triage at close, not a gate) ===\n");
    out.push_str(&format!(
        "{} prompting call(s) across {} pattern(s), from {} logged fall-through(s).\n",
        t.total,
        t.prompting.len(),
        t.logged
    ));
    out.push_str(&format!("log: {}\n\n", log));
    rank_section(&t.prompting, &mut out);
    out.push_str("\nTriage each by the criterion (guard-kit/SPEC.md §The triage criterion):\n");
    out.push_str("  (a) allowlist entry — safe & already in the form to reinforce,\n");
    out.push_str("  (b) guard rule — a better form exists (steer), or logic a glob can't express,\n");
    out.push_str("  (c) habit change — a true one-off.\n");
    overlay_section(&t, settings_local, &mut out);
    out.push_str(&format!("\nThen clear the log:  : > {}\n", log));
    out
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the log positional selects the rule's input corpus
// rather than resolved config, so it composes with `--count` in either order; an unrecognized
// `-`-prefixed positional is a refusal and `--` ends option processing.
fn parse(args: &[String]) -> Result<(bool, Option<&str>), String> {
    let mut count_only = false;
    let mut log_arg: Option<&str> = None;
    let mut options = true;
    for a in args {
        if options {
            if a == "--" {
                options = false;
                continue;
            }
            if a == "--count" {
                count_only = true;
                continue;
            }
            if a.starts_with('-') {
                return Err(format!("unrecognized option: {}\n{}", a, USAGE));
            }
        }
        if a.is_empty() {
            continue;
        }
        log_arg = Some(a);
    }
    Ok((count_only, log_arg))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let (count_only, log_arg) = parse(args)?;
    let log = match log_arg {
        Some(l) => l.to_string(),
        None => walk::knob_scalar("GUARD_KIT_LOG")?,
    };
    let settings = walk::knob_scalar("GUARD_KIT_SETTINGS")?;
    let settings_local = walk::knob_scalar("GUARD_KIT_SETTINGS_LOCAL")?;
    Ok(render(&log, &settings, &settings_local, count_only))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: guard-kit/SPEC.md §scan-prompts — the key's two tokens: the leading binary, and a
    // subcommand only for the multi-command set, so a single-command binary keys on one word
    #[test]
    fn the_key_sub_keys_only_the_multi_command_binaries() {
        assert_eq!(ranking_key("git status --short"), "git status");
        assert_eq!(ranking_key("make build"), "make");
        assert_eq!(ranking_key("python3 -"), "python3 -");
        assert_eq!(ranking_key("sudo timeout 30 git log"), "git log");
    }

    // spec: guard-kit/SPEC.md §scan-prompts — the write-shape suffix: create and append split, the
    // descriptor is dropped, and an fd-dup is not a redirect to a file
    #[test]
    fn the_write_shape_suffix_splits_create_from_append_and_skips_an_fd_dup() {
        assert_eq!(ranking_key("cat > .tmp/a.md <<EOF"), "cat >");
        assert_eq!(ranking_key("cat >> .tmp/b.md <<EOF"), "cat >>");
        assert_eq!(ranking_key("make build 2>&1"), "make");
        assert_eq!(ranking_key("sort -rn 2> err.txt"), "sort >");
        assert_eq!(ranking_key("wc -l < in.txt"), "wc");
    }

    // spec: guard-kit/SPEC.md §scan-prompts — word and suffix come from the FIRST segment, so a
    // compound whose write lives downstream never attributes it to the leading word
    #[test]
    fn a_downstream_write_is_never_attributed_to_the_leading_word() {
        assert_eq!(ranking_key("mkdir -p .tmp && cat > .tmp/c.md"), "mkdir");
        assert_eq!(ranking_key("git status && rm -rf x"), "git status");
    }

    // spec: guard-kit/SPEC.md §scan-prompts — a write redirect standing in subcommand position is
    // re-homed into the suffix rather than doubled into both tokens
    #[test]
    fn a_redirect_in_subcommand_position_is_re_homed_rather_than_doubled() {
        assert_eq!(ranking_key("bash > out.txt"), "bash >");
        assert_eq!(ranking_key("bash 2> out.txt"), "bash >");
    }

    // spec: guard-kit/SPEC.md §scan-prompts — granted only if EVERY segment is: a whole-string glob
    // spanning a compound the harness would split and refuse does not read as allowed
    #[test]
    fn a_whole_string_glob_does_not_grant_a_compound_the_harness_would_split() {
        let allow = vec!["git status*".to_string(), "ls".to_string()];
        assert!(granted("git status && ls", &allow, None));
        assert!(!granted("git status && rm -rf x", &allow, None));
        assert!(granted("git diff", &[], None), "a harness built-in did not grant");
        assert!(!granted("git push", &[], None));
    }

    // spec: guard-kit/SPEC.md §scan-prompts — a separator inside a quoted span is not a separator,
    // which is the whole reason the match runs on a quoted view rather than on the raw line
    #[test]
    fn a_separator_inside_a_quoted_span_does_not_split_the_command() {
        assert_eq!(quoted_view("echo 'a;b' && ls"), "echo SQ && ls");
        assert_eq!(quoted_view("echo \"a && b\""), "echo DQ");
        assert_eq!(quoted_view("echo 'unterminated"), "echo 'unterminated");
        assert!(granted("echo 'a;b'", &["echo SQ".to_string()], None));
    }

    // spec: guard-kit/SPEC.md §scan-prompts — the overlay share is excluded from the headline, so
    // the count is a true prompt count rather than an upper bound
    #[test]
    fn an_overlay_only_grant_is_off_the_headline_and_on_the_advisory_ranking() {
        let allow = vec!["git status:*".to_string(), "ls".to_string()];
        let overlay = vec!["npm test".to_string()];
        let log = "npm test\ngit status && rm -rf x\ngit status && ls\nmake build\n";
        let t = tally(log, &allow, &overlay);
        assert_eq!((t.prompting.len(), t.total), (2, 2));
        assert_eq!((t.overlay.len(), t.overlay_total), (1, 1));
        assert_eq!(t.overlay[0].0, "npm test");
        assert_eq!(t.logged, 4);
    }

    // spec: guard-kit/SPEC.md §scan-prompts — occurrences descending, a tie broken by descending
    // key bytes, which is what the holder's `sort -rn` produced
    #[test]
    fn the_ranking_orders_by_occurrences_then_by_descending_key_bytes() {
        let rows = vec![
            ("alpha".to_string(), 3),
            ("Mixed".to_string(), 3),
            ("mid".to_string(), 5),
            ("zebra".to_string(), 3),
        ];
        let order: Vec<&str> = ranked(&rows).iter().map(|r| r.0.as_str()).collect();
        assert_eq!(order, vec!["mid", "zebra", "alpha", "Mixed"]);
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape refusal crosses the port and the
    // `--` escape is what keeps it a fix rather than a capability loss
    #[test]
    fn an_unrecognized_dash_argument_is_refused_and_a_separator_admits_it() {
        let err = parse(&argv(&["--count", "--nonsense"]))
            .expect_err("a typo of the tool's own flag was absorbed as a log path");
        assert!(err.contains("--nonsense"), "the refusal named no offender: {}", err);
        for flag in ["-h", "--help"] {
            assert!(parse(&argv(&[flag])).is_err(), "{} was taken as a log path", flag);
        }
        assert_eq!(
            parse(&argv(&["--", "-dash.log"])).expect("the separator did not end option processing"),
            (false, Some("-dash.log"))
        );
    }

    // spec: guard-kit/SPEC.md §scan-prompts — the log positional and `--count` compose in either
    // order, which is what the single-argument parse this replaced silently broke
    #[test]
    fn the_log_positional_and_the_count_flag_compose_in_either_order() {
        for order in [["--count", "x.log"], ["x.log", "--count"]] {
            assert_eq!(
                parse(&argv(&order)).expect("a legitimate argv pair was refused"),
                (true, Some("x.log")),
                "the pair did not compose in the order {:?}",
                order
            );
        }
    }

    // spec: guard-kit/SPEC.md §scan-prompts — `--count` emits `<patterns>/<occurrences>`, and an
    // absent log answers `0/0` rather than a report of nothing
    #[test]
    fn the_count_mode_emits_the_two_integers_and_an_absent_log_is_zero_over_zero() {
        let dir = std::env::temp_dir().join(format!("cw-scan-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot make the fixture dir");
        let log = dir.join("friction.log");
        std::fs::write(&log, "make build\nmake build\n").expect("cannot write the fixture log");
        let p = log.display().to_string();
        let settings = dir.join("settings.json").display().to_string();
        assert_eq!(count(&p, &settings, &settings), (1, 2));
        assert_eq!(render(&p, &settings, &settings, true), "1/2\n");
        let absent = dir.join("nothing.log").display().to_string();
        assert_eq!(render(&absent, &settings, &settings, true), "0/0\n");
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: guard-kit/SPEC.md §scan-prompts — the allow inners are parsed in-crate, so a document
    // no parser can read is an empty allowlist rather than a machine-wide dependency
    #[test]
    fn the_allow_inners_are_the_bash_grants_stripped_of_their_wrapper() {
        assert_eq!(
            allow_inners(r#"{"permissions":{"allow":["Bash(ls)","Read(x)","Bash(git status:*)"]}}"#),
            vec!["ls".to_string(), "git status:*".to_string()]
        );
        assert!(allow_inners("{not json").is_empty());
        assert!(allow_inners(r#"{"permissions":{}}"#).is_empty());
    }
}
