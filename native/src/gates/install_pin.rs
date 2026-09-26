// spec: installer/SPEC.md §The hosted install pin — the two hosted install scripts carry one pin
// each, the pins agree (A), the pin is the newest release (B), and every fetch surface spells only
// the pinned release's asset names (C)
use super::release_assets::{declaration, DEFAULT_DOC};
use super::release_channel_parity::newest_tag;
use crate::{fresh, proc, programs};
use std::path::Path;

const DEFAULT_INSTALL_SH: &str = "docs/install.sh";
const DEFAULT_INSTALL_PS1: &str = "docs/install.ps1";
const DEFAULT_INSTALL_MD: &str = "docs/install.md";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-pin: {}", e);
            2
        }
    }
}

// spec: installer/SPEC.md §The hosted install pin — `<major>.<minor>.<patch>`, each a run of ASCII
// digits, with no prefix and no suffix
pub(crate) fn is_triple(v: &str) -> bool {
    let fields: Vec<&str> = v.split('.').collect();
    fields.len() == 3
        && fields
            .iter()
            .all(|f| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()))
}

// spec: installer/SPEC.md §The hosted install pin — a pin line is any line whose trimmed form opens
// on the script's pin variable and an `=`; exactly one such line is admissible, and it must be the
// single-quoted triple
pub(crate) fn pin_of(path: &str, text: &str, name: &str) -> Result<String, String> {
    let lines: Vec<(usize, &str)> = fresh::file_lines(text)
        .into_iter()
        .enumerate()
        .map(|(n, l)| (n + 1, l.trim()))
        .filter(|(_, l)| {
            l.strip_prefix(name)
                .is_some_and(|rest| rest.trim_start().starts_with('='))
        })
        .collect();
    let (lineno, line) = match lines.as_slice() {
        [one] => *one,
        [] => {
            return Err(format!(
                "{} carries no '{}' pin line — the hosted install's version cannot be established (installer/SPEC.md §The hosted install pin)",
                path, name
            ))
        }
        many => {
            let shown: Vec<String> = many.iter().map(|(n, l)| format!("{}:{}", n, l)).collect();
            return Err(format!(
                "{} carries {} '{}' pin lines; exactly one is admissible:\n{}",
                path,
                many.len(),
                name,
                shown.join("\n")
            ));
        }
    };
    let value = line[name.len()..]
        .trim_start()
        .strip_prefix('=')
        .map(str::trim)
        .and_then(|v| v.strip_prefix('\''))
        .and_then(|v| v.strip_suffix('\''))
        .filter(|v| is_triple(v))
        .ok_or_else(|| {
            format!(
                "{}:{}: the pin is not a single-quoted <major>.<minor>.<patch>: {}",
                path, lineno, line
            )
        })?;
    Ok(value.to_string())
}

// spec: installer/SPEC.md §The hosted install pin — an asset token is a maximal run of
// `[A-Za-z0-9._{}$-]` containing `.tgz` or `.tar.gz`, each `$name` or `${name}` in it read as
// `{version}`; the raw run is returned beside its normalized form for the finding
fn asset_tokens(text: &str) -> Vec<(usize, String)> {
    let in_class = |c: char| c.is_ascii_alphanumeric() || "._{}$-".contains(c);
    let mut out = Vec::new();
    for (n, line) in fresh::file_lines(text).into_iter().enumerate() {
        for run in line.split(|c: char| !in_class(c)) {
            if run.contains(".tgz") || run.contains(".tar.gz") {
                out.push((n + 1, normalize(run)));
            }
        }
    }
    out
}

fn normalize(run: &str) -> String {
    let name = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut out = String::new();
    let mut rest = run;
    while let Some(at) = rest.find('$') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let braced = after
            .strip_prefix('{')
            .and_then(|b| b.find('}').map(|close| (close, &b[..close])))
            .filter(|(_, n)| !n.is_empty() && n.chars().all(name));
        if let Some((close, _)) = braced {
            out.push_str("{version}");
            rest = &after[close + 2..];
            continue;
        }
        let len = after.find(|c: char| !name(c)).unwrap_or(after.len());
        if len == 0 {
            out.push('$');
        } else {
            out.push_str("{version}");
        }
        rest = &after[len..];
    }
    out.push_str(rest);
    out
}

// spec: installer/SPEC.md §The hosted install pin — invariant C's findings over the three fetch
// surfaces: an undeclared token, a surface with none, and a matched non-sidecar template whose
// `.sha256` sidecar the pinned declaration lacks
fn invariant_c(surfaces: &[(&str, String)], templates: &[&str], pinned: &str) -> (Vec<String>, usize) {
    let mut findings = Vec::new();
    let mut matched: Vec<&str> = Vec::new();
    let mut count = 0;
    for (path, text) in surfaces {
        let tokens = asset_tokens(text);
        if tokens.is_empty() {
            findings.push(format!(
                "  invariant C: {} spells no asset name, so what it fetches cannot be held to the declaration",
                path
            ));
        }
        count += tokens.len();
        for (line, token) in &tokens {
            match templates.iter().find(|t| **t == token.as_str()) {
                Some(t) => {
                    if !matched.contains(t) {
                        matched.push(t);
                    }
                }
                None => findings.push(format!(
                    "  invariant C: {}:{}: '{}' is no asset of the pinned release v{}",
                    path, line, token, pinned
                )),
            }
        }
    }
    for t in matched {
        if !t.ends_with(".sha256") && !templates.contains(&format!("{}.sha256", t).as_str()) {
            findings.push(format!(
                "  invariant C: the fetched asset '{}' has no '{}.sha256' sidecar in v{}'s declaration",
                t, t, pinned
            ));
        }
    }
    (findings, count)
}

// spec: installer/SPEC.md §The hosted install pin — the pinned declaration: from the positional
// file when one is given, else the declaring doc at `v<pin>`; `None` is C's dormancy, a tag that
// does not resolve
fn pinned_declaration(doc_arg: Option<&str>, pin: &str) -> Result<Option<(String, Vec<String>)>, String> {
    let (source, text) = match doc_arg {
        Some(p) => {
            if !Path::new(p).is_file() {
                return Err(format!("not found: {}", p));
            }
            (p.to_string(), fresh::read_captured(p)?)
        }
        None => {
            let tag = format!("v{}", pin);
            let peeled = format!("refs/tags/{}^{{commit}}", tag);
            let resolves = proc::run(&programs::GIT, &["rev-parse", "-q", "--verify", &peeled])
                .map(|c| c.stdout().is_some())
                .unwrap_or(false);
            if !resolves {
                return Ok(None);
            }
            let spec = format!("{}:{}", tag, DEFAULT_DOC);
            let shown = proc::run(&programs::GIT, &["show", &spec])?;
            let text = shown
                .stdout()
                .map(|o| String::from_utf8_lossy(o).into_owned())
                .ok_or_else(|| format!("the pinned tag {} carries no declaring doc {}", tag, DEFAULT_DOC))?;
            (spec, text)
        }
    };
    let decls: Vec<Vec<String>> = fresh::file_lines(&text)
        .iter()
        .filter_map(|l| declaration(l).map(|t| t.iter().map(|s| s.to_string()).collect()))
        .collect();
    match decls.as_slice() {
        [one] => Ok(Some((source, one.clone()))),
        _ => Err(format!(
            "{} carries {} release-assets lines; exactly one is admissible (gate-sdk/SPEC.md §check-release-assets)",
            source,
            decls.len()
        )),
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let install_sh = fresh::positional(args, 0, DEFAULT_INSTALL_SH);
    let install_ps1 = fresh::positional(args, 1, DEFAULT_INSTALL_PS1);
    let install_md = fresh::positional(args, 2, DEFAULT_INSTALL_MD);
    let pinned_doc = args.get(3).map(String::as_str).filter(|a| !a.is_empty());
    let version_arg = args.get(4).map(String::as_str).unwrap_or("");

    for p in [install_sh, install_ps1, install_md] {
        if !Path::new(p).is_file() {
            return Err(format!("not found: {}", p));
        }
    }
    let sh_text = fresh::read_captured(install_sh)?;
    let ps_text = fresh::read_captured(install_ps1)?;
    let md_text = fresh::read_captured(install_md)?;
    let sh_pin = pin_of(install_sh, &sh_text, "pin")?;
    let ps_pin = pin_of(install_ps1, &ps_text, "$pin")?;

    let mut findings: Vec<String> = Vec::new();
    if sh_pin != ps_pin {
        findings.push(format!(
            "  invariant A: the twins disagree — {} pins {} and {} pins {}",
            install_sh, sh_pin, install_ps1, ps_pin
        ));
    }

    // spec: installer/SPEC.md §The hosted install pin — B is dormant, and says so, where no tag exists
    let version = if version_arg.is_empty() {
        newest_tag()
    } else {
        version_arg.to_string()
    };
    let b_state = if version.is_empty() {
        None
    } else {
        let bare = version.strip_prefix('v').unwrap_or(&version).to_string();
        if !is_triple(&bare) {
            return Err(format!(
                "the newest tag ('{}') does not parse as <major>.<minor>.<patch>, so the pin cannot be compared against it",
                version
            ));
        }
        for (path, pin) in [(install_sh, &sh_pin), (install_ps1, &ps_pin)] {
            if *pin != bare {
                findings.push(format!(
                    "  invariant B: {} pins {}, and the newest release is v{}",
                    path, pin, bare
                ));
            }
        }
        Some(bare)
    };
    let ab_red = !findings.is_empty();

    let c_state = match pinned_declaration(pinned_doc, &sh_pin)? {
        None => None,
        Some((source, templates)) => {
            let templates: Vec<&str> = templates.iter().map(String::as_str).collect();
            let surfaces = [(install_sh, sh_text), (install_ps1, ps_text), (install_md, md_text)];
            let (c_findings, count) = invariant_c(&surfaces, &templates, &sh_pin);
            findings.extend(c_findings);
            Some((source, count))
        }
    };

    if !findings.is_empty() {
        println!("check-install-pin: a hosted install surface disagrees with what it is held to (installer/SPEC.md §The hosted install pin):");
        for f in &findings {
            println!("{}", f);
        }
        if ab_red {
            println!("  help: set both scripts' pin line to the newest release tag, in the commit after that tag — RELEASING.md step 4 moves it with the release declaration surface's drain.");
        }
        if findings.iter().any(|f| f.contains("invariant C")) {
            println!("  help: spell the pinned release's asset names on every fetch surface, or, in the commit moving the pin, the new release's names.");
        }
        return Ok(1);
    }
    let b_note = match b_state {
        None => "invariant B dormant — no tags, so there is no newest release to compare".to_string(),
        Some(v) => format!("the newest release v{}", v),
    };
    let c_note = match c_state {
        None => format!("invariant C dormant — v{} does not resolve here", sh_pin),
        Some((source, count)) => format!(
            "{} asset token(s) across the three fetch surfaces, each declared by v{}'s release-assets line in {}",
            count, sh_pin, source
        ),
    };
    println!("INSTALL-PIN: clean (both scripts pin {}; {}; {})", sh_pin, b_note, c_note);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: installer/SPEC.md §The hosted install pin — the pin grammar is the bare triple
    #[test]
    fn only_a_bare_triple_is_a_pin() {
        assert!(is_triple("0.25.0"));
        for bad in ["v0.25.0", "0.25", "0.25.0-rc1", "0.25.x", "", "0..1"] {
            assert!(!is_triple(bad), "{} should not be a triple", bad);
        }
    }

    // spec: installer/SPEC.md §The hosted install pin — one pin line per script, in its own
    // grammar, indented or not
    #[test]
    fn a_pin_line_is_read_in_each_scripts_grammar() {
        assert_eq!(pin_of("s", "f() {\n    pin='1.2.3'\n}\n", "pin"), Ok("1.2.3".to_string()));
        assert_eq!(pin_of("p", "    $pin = '1.2.3'\n", "$pin"), Ok("1.2.3".to_string()));
        assert_eq!(pin_of("s", "pin_dir=x\npin='1.2.3'\n", "pin"), Ok("1.2.3".to_string()));
    }

    // spec: installer/SPEC.md §The hosted install pin — the exit-2 set: a missing, duplicated or
    // malformed pin line
    #[test]
    fn a_missing_duplicated_or_malformed_pin_fails_closed() {
        assert!(pin_of("s", "echo hi\n", "pin").is_err());
        assert!(pin_of("s", "pin='1.2.3'\npin='1.2.4'\n", "pin").is_err());
        assert!(pin_of("s", "pin=\"1.2.3\"\n", "pin").is_err());
        assert!(pin_of("p", "$pin = 'v1.2.3'\n", "$pin").is_err());
    }

    fn names(text: &str) -> Vec<String> {
        asset_tokens(text).into_iter().map(|(_, t)| t).collect()
    }

    // spec: installer/SPEC.md §The hosted install pin — the token rule: a path or backslash ends a
    // run, both variable forms read as {version}, and a name suffixed onto a variable is no token
    #[test]
    fn the_tokenizer_reads_each_spelling_the_fetch_surfaces_use() {
        assert_eq!(names("curl -o \"$cw/checkwright-$v.tgz\" x\n"), vec!["checkwright-{version}.tgz"]);
        assert_eq!(names("-OutFile \"$cw\\checkwright-$v.tgz.sha256\"\n"), vec!["checkwright-{version}.tgz.sha256"]);
        assert_eq!(names("t=\"checkwright-gates-${ver}-x86.tar.gz\"\n"), vec!["checkwright-gates-{version}-x86.tar.gz"]);
        assert!(names("for f in \"$cw_tgz\" \"$cw_tgz.sha256\"; do\n").is_empty());
        assert_eq!(names("a\ncw_tgz=\"checkwright-$cw_version.tgz\"\n"), vec!["checkwright-{version}.tgz"]);
        assert_eq!(asset_tokens("a\nb checkwright-$v.tgz\n")[0].0, 2);
    }

    // spec: installer/SPEC.md §The hosted install pin — C's three readings, and a sidecar token
    // owing no sidecar of its own
    #[test]
    fn invariant_c_names_an_undeclared_token_an_empty_surface_and_a_missing_sidecar() {
        let full = ["checkwright-{version}.tgz", "checkwright-{version}.tgz.sha256"];
        let ok = [
            ("s", "x checkwright-$v.tgz\n".to_string()),
            ("m", "checkwright-$v.tgz checkwright-$v.tgz.sha256\n".to_string()),
        ];
        let (f, n) = invariant_c(&ok, &full, "1.0.0");
        assert!(f.is_empty(), "{:?}", f);
        assert_eq!(n, 3);

        let bad = [
            ("s", "checkwright-$v.tar.gz\n".to_string()),
            ("m", "no names here\n".to_string()),
            ("p", "checkwright-$v.tgz\n".to_string()),
        ];
        let (f, _) = invariant_c(&bad, &["checkwright-{version}.tgz"], "1.0.0");
        assert_eq!(f.len(), 3, "{:?}", f);
        assert!(f[0].contains("no asset of the pinned release"));
        assert!(f[1].contains("spells no asset name"));
        assert!(f[2].contains("no 'checkwright-{version}.tgz.sha256' sidecar"));
    }

    // spec: installer/SPEC.md §The hosted install pin — a pinned doc with no declaration or two
    // fails closed, as an absent one does
    #[test]
    fn a_pinned_doc_needs_exactly_one_declaration() {
        let dir = std::env::temp_dir().join(format!("install-pin-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let case = |name: &str, body: &str| -> String {
            let p = dir.join(name).display().to_string();
            std::fs::write(&p, body).expect("write");
            p
        };
        let line = "<!-- release-assets: a-{version}.tgz a-{version}.tgz.sha256 -->\n";
        let one = case("one.md", &format!("x\n{}", line));
        let got = pinned_declaration(Some(&one), "1.0.0").expect("one line reads");
        assert_eq!(got.map(|(_, t)| t), Some(vec!["a-{version}.tgz".to_string(), "a-{version}.tgz.sha256".to_string()]));
        assert!(pinned_declaration(Some(&case("none.md", "x\n")), "1.0.0").is_err());
        assert!(pinned_declaration(Some(&case("two.md", &format!("{}{}", line, line))), "1.0.0").is_err());
        assert!(pinned_declaration(Some(&dir.join("absent.md").display().to_string()), "1.0.0").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
