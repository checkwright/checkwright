// spec: drift-kit/SPEC.md §The install-observation record — the capture affordance: one appended
// line per observation event, the date stamped by the producer rather than by its author.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and not a hardcoded flag, because the
// arm reads a consumer knob, which a hardcoded flag would hide from the knob-file derivation.

pub const KNOBS: &[&str] = &["DRIFT_KIT_INSTALL_RECORD"];

pub const USAGE: &str = "usage: --emit file-install [--] <kind> <field>...\n  \
     install <id> <profile> <floor> <ttfg>\n  \
     red <id> <gate> <verdict> <disposition> <behaviour>\n  \
     checkin <id> <day> <kit> <retained>\n  \
     appends one dated line to the install-observation record; \
     \"--\" files a field beginning with \"-\" (a `-` ttfg among them)";

// spec: drift-kit/SPEC.md §The install-observation record — the closed value sets, one per field a
// published count aggregates: an open value there yields a column no projection renders honestly.
pub const VERDICTS: &[&str] = &["true-positive", "false-positive", "unclear"];
pub const DISPOSITIONS: &[&str] = &["fixed", "worked-around", "bypassed", "abandoned"];
pub const BEHAVIOURS: &[&str] = &["changed", "unchanged"];
pub const DAYS: &[&str] = &["7", "30"];
pub const RETAINED: &[&str] = &["yes", "no"];

// spec: drift-kit/SPEC.md §The install-observation record — `-` is a value rather than a gap: an
// install that never went green is the observation this channel most needs.
pub const NO_GREEN: &str = "-";

// spec: drift-kit/SPEC.md §The install-observation record — a field is checked against a closed set,
// against the ttfg grammar, or for shape alone; the third class names a consumer's own vocabulary,
// which a kit literal enumerating it would publish (gate-sdk/SPEC.md §The provenance seam).
enum Check {
    Set(&'static [&'static str]),
    Ttfg,
    Free,
}

// spec: drift-kit/SPEC.md §The install-observation record — the grammars less the date the arm
// stamps, each carrying its key rule as the field positions a re-file replaces on; an empty key is
// the pure-append rule, which is `red`'s.
struct Kind {
    name: &'static str,
    fields: &'static [(&'static str, Check)],
    key: &'static [usize],
}

const KINDS: &[Kind] = &[
    Kind {
        name: "install",
        fields: &[
            ("id", Check::Free),
            ("profile", Check::Free),
            ("floor", Check::Free),
            ("ttfg", Check::Ttfg),
        ],
        key: &[0],
    },
    Kind {
        name: "red",
        fields: &[
            ("id", Check::Free),
            ("gate", Check::Free),
            ("verdict", Check::Set(VERDICTS)),
            ("disposition", Check::Set(DISPOSITIONS)),
            ("behaviour", Check::Set(BEHAVIOURS)),
        ],
        key: &[],
    },
    Kind {
        name: "checkin",
        fields: &[
            ("id", Check::Free),
            ("day", Check::Set(DAYS)),
            ("kit", Check::Free),
            ("retained", Check::Set(RETAINED)),
        ],
        key: &[0, 1, 2],
    },
];

fn kind(name: &str) -> Option<&'static Kind> {
    KINDS.iter().find(|k| k.name == name)
}

// spec: drift-kit/SPEC.md §The install-observation record — a non-negative integer of minutes, or
// the no-green sentinel; a leading `+` or `-` is not a count of minutes.
fn is_ttfg(v: &str) -> bool {
    v == NO_GREEN || (!v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()))
}

// spec: drift-kit/SPEC.md §The install-observation record — the refusals are the protocol: an
// observer who runs the arm is told what the record wants, which is why the closed sets are
// enforced at capture rather than at projection.
fn validate(k: &Kind, fields: &[String]) -> Result<(), String> {
    if fields.len() != k.fields.len() {
        return Err(USAGE.to_string());
    }
    for (v, (label, check)) in fields.iter().zip(k.fields.iter()) {
        if v.is_empty() {
            return Err(format!("{}: <{}> is empty\n{}", k.name, label, USAGE));
        }
        match check {
            Check::Set(set) => {
                if !set.iter().any(|m| m == v) {
                    return Err(format!(
                        "{}: <{}> is \"{}\", outside its closed set {{{}}}\n{}",
                        k.name,
                        label,
                        v,
                        set.join(" | "),
                        USAGE
                    ));
                }
            }
            Check::Ttfg => {
                if !is_ttfg(v) {
                    return Err(format!(
                        "{}: <{}> is \"{}\"; minutes to the first green as a non-negative integer, \
                         or \"{}\" where no green was reached\n{}",
                        k.name, label, v, NO_GREEN, USAGE
                    ));
                }
            }
            Check::Free => {}
        }
    }
    Ok(())
}

// spec: drift-kit/SPEC.md §The install-observation record — the line grammar is the record's
// contract, so it is stamped here byte for byte and the same string is the arm's confirmation.
fn line(today: &str, name: &str, fields: &[String]) -> String {
    let mut out = format!("{} {}", today, name);
    for f in fields {
        out.push(' ');
        out.push_str(f);
    }
    out
}

// spec: drift-kit/SPEC.md §The install-observation record — a stored line's key: its kind plus the
// kind's key field positions, joined by a separator no whitespace-split field can carry.
fn line_key(k: &Kind, fields: &[&str]) -> Option<String> {
    let mut out = k.name.to_string();
    for i in k.key {
        out.push('\u{1f}');
        out.push_str(fields.get(*i)?);
    }
    Some(out)
}

// spec: drift-kit/SPEC.md §The install-observation record — a replace is in place: the key's first
// line is rewritten where it stands and any later duplicate of it is dropped, so a re-file of
// unchanged fields leaves the record byte-identical.
fn replace_in_place(body: &str, k: &Kind, want: &str, new: &str) -> Option<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen = false;
    for l in body.lines() {
        let fields: Vec<&str> = l.split_whitespace().collect();
        let matches = fields.len() > 2
            && kind(fields[1]).is_some_and(|lk| lk.name == k.name)
            && line_key(k, &fields[2..]).as_deref() == Some(want);
        if !matches {
            out.push(l.to_string());
            continue;
        }
        if seen {
            continue;
        }
        seen = true;
        out.push(new.to_string());
    }
    if !seen {
        return None;
    }
    let mut text = out.join("\n");
    text.push('\n');
    Some(text)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let argv = super::file_survey::positionals(args, "field")?;
    let name = argv.first().ok_or_else(|| USAGE.to_string())?;
    let k = kind(name).ok_or_else(|| {
        format!(
            "unknown kind: {} — one of {}\n{}",
            name,
            KINDS
                .iter()
                .map(|k| k.name)
                .collect::<Vec<_>>()
                .join(" | "),
            USAGE
        )
    })?;
    let fields = &argv[1..];
    validate(k, fields)?;

    let (record, spelled) = super::file_survey::anchored_capture("DRIFT_KIT_INSTALL_RECORD")?;
    let path = std::path::Path::new(&record);
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let line = line(&super::kpi::today_iso(), k.name, fields);

    let mut replaced = false;
    if !k.key.is_empty() {
        let refs: Vec<&str> = fields.iter().map(String::as_str).collect();
        let want = line_key(k, &refs).ok_or_else(|| USAGE.to_string())?;
        let body = std::fs::read(path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        if let Some(text) = replace_in_place(&body, k, &want, &line) {
            std::fs::write(path, text)
                .map_err(|e| format!("cannot rewrite {}: {}", spelled, e))?;
            replaced = true;
        }
    }
    if !replaced {
        super::kfric::append_creating(path, &format!("{}\n", line))
            .map_err(|e| format!("cannot append to {}: {}", spelled, e))?;
    }
    Ok(format!(
        "file-install: {}{}\n",
        line,
        if replaced { " (replaced)" } else { "" }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::file_survey;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    fn fields(a: &[&str]) -> Vec<String> {
        argv(a)
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the hazard belongs to the argument rather
    // than to the substrate, and six slots leave arity safe in none, so the scan covers every one.
    #[test]
    fn a_flag_in_any_slot_is_refused_and_a_separator_files_it() {
        for i in 0..6 {
            let mut a = argv(&["red", "i1", "check-x", "unclear", "fixed", "changed"]);
            a[i] = "--list".to_string();
            let err = file_survey::positionals(&a, "field")
                .err()
                .unwrap_or_else(|| panic!("a flag in slot {} was captured", i));
            assert!(err.contains("--list"), "the refusal named no offender: {}", err);
        }
        let sep = argv(&["--", "install", "i1", "p", "f", "-"]);
        assert_eq!(
            file_survey::positionals(&sep, "field")
                .expect("the separator did not end option processing"),
            &sep[1..]
        );
    }

    // spec: drift-kit/SPEC.md §The install-observation record — `--help` is a refusal rather than
    // usage, the dispatcher printing the usage block beneath it.
    #[test]
    fn a_help_flag_is_a_refusal_rather_than_a_capture() {
        for flag in ["-h", "--help"] {
            assert!(
                file_survey::positionals(&argv(&[flag]), "field").is_err(),
                "{} was taken as a field",
                flag
            );
        }
    }

    // spec: drift-kit/SPEC.md §The install-observation record — every refusal class the arm owes,
    // asserted before any write: an unknown kind, the wrong arity for the kind, an empty
    // positional, and a closed-set field whose value is outside its set.
    #[test]
    fn each_refusal_class_refuses_before_a_write() {
        assert!(emit(&argv(&[])).is_err(), "no kind at all");
        assert!(emit(&argv(&["deploy", "i1"])).is_err(), "unknown kind");
        let k = kind("red").expect("the red kind");
        assert!(validate(k, &fields(&["i1", "check-x", "unclear"])).is_err(), "short arity");
        assert!(
            validate(k, &fields(&["i1", "check-x", "unclear", "fixed", "changed", "extra"])).is_err(),
            "long arity"
        );
        assert!(
            validate(k, &fields(&["i1", "", "unclear", "fixed", "changed"])).is_err(),
            "empty positional"
        );
        assert!(
            validate(k, &fields(&["i1", "check-x", "probably", "fixed", "changed"])).is_err(),
            "verdict outside its closed set"
        );
        assert!(
            validate(k, &fields(&["i1", "check-x", "unclear", "reverted", "changed"])).is_err(),
            "disposition outside its closed set"
        );
        assert!(
            validate(k, &fields(&["i1", "check-x", "unclear", "fixed", "maybe"])).is_err(),
            "behaviour outside its closed set"
        );
        assert!(validate(k, &fields(&["i1", "check-x", "unclear", "fixed", "changed"])).is_ok());
    }

    // spec: drift-kit/SPEC.md §The install-observation record — `<ttfg>` is minutes or the no-green
    // sentinel, and the sentinel is a value: an install that never went green is the observation the
    // channel most needs and the one a missing line would erase.
    #[test]
    fn ttfg_is_minutes_or_the_no_green_sentinel() {
        assert!(is_ttfg("0"));
        assert!(is_ttfg("47"));
        assert!(is_ttfg(NO_GREEN));
        assert!(!is_ttfg(""));
        assert!(!is_ttfg("-1"));
        assert!(!is_ttfg("+3"));
        assert!(!is_ttfg("3m"));
        assert!(!is_ttfg("3.5"));
    }

    // spec: drift-kit/SPEC.md §The install-observation record — one key rule per kind, because
    // assuming a single rule would be wrong for two of them.
    #[test]
    fn the_key_rules_are_one_per_kind() {
        let install = kind("install").expect("the install kind");
        let red = kind("red").expect("the red kind");
        let checkin = kind("checkin").expect("the checkin kind");
        assert!(red.key.is_empty(), "a red event has no natural key");
        assert_eq!(
            line_key(install, &["i1", "linux", "bash5", "12"]),
            line_key(install, &["i1", "macos", "bash3", "-"]),
            "an install re-measure corrects one install's row"
        );
        assert_ne!(
            line_key(checkin, &["i1", "7", "gate-sdk", "yes"]),
            line_key(checkin, &["i1", "30", "gate-sdk", "yes"]),
            "a check-in keys on the day too"
        );
        assert_eq!(
            line_key(checkin, &["i1", "7", "gate-sdk", "yes"]),
            line_key(checkin, &["i1", "7", "gate-sdk", "no"]),
            "a re-run of one check-in must not double-count retention"
        );
    }

    // spec: drift-kit/SPEC.md §The install-observation record — the replace is in place: the key's
    // first line is rewritten where it stands, a later duplicate is dropped, and a sibling key's
    // line is untouched, so a re-file corrects rather than reorders.
    #[test]
    fn a_replace_rewrites_in_place_and_leaves_its_siblings_alone() {
        let install = kind("install").expect("the install kind");
        let body = "2026-01-01 install i1 linux bash5 12\n\
                    2026-01-02 red i1 check-x unclear fixed changed\n\
                    2026-01-03 install i2 macos bash3 -\n\
                    2026-01-04 install i1 linux bash5 99\n";
        let want = line_key(install, &["i1"]).expect("a key");
        let out = replace_in_place(body, install, &want, "2026-02-01 install i1 linux bash5 7")
            .expect("the key was present");
        assert_eq!(
            out,
            "2026-02-01 install i1 linux bash5 7\n\
             2026-01-02 red i1 check-x unclear fixed changed\n\
             2026-01-03 install i2 macos bash3 -\n"
        );
        let absent = line_key(install, &["i9"]).expect("a key");
        assert!(
            replace_in_place(body, install, &absent, "x").is_none(),
            "a key new to the record appends rather than rewriting"
        );
    }

    // spec: drift-kit/SPEC.md §The install-observation record — the line grammar, byte for byte:
    // the date first and the event kind second, then the kind's positionals in their fixed order.
    #[test]
    fn the_line_grammar_is_byte_preserved() {
        assert_eq!(
            line("2026-01-01", "checkin", &fields(&["i1", "30", "gate-sdk", "no"])),
            "2026-01-01 checkin i1 30 gate-sdk no"
        );
    }
}
