// spec: installer/SPEC.md §The hosted install pin — the install pages: the member set
// `GATE_LOCAL_INSTALL_DOCS` names, read from the tree or from the members a tag carries, each
// block or line taken from the one member carrying it
use super::pinned_release;
use crate::{fresh, walk};
use std::path::Path;

pub(crate) const KNOB: &str = "GATE_LOCAL_INSTALL_DOCS";

pub(crate) struct Member {
    pub(crate) path: String,
    pub(crate) text: String,
}

// spec: installer/SPEC.md §The hosted install pin — a positional page is one file and wins over
// the knob
pub(crate) fn paths(args: &[String], n: usize) -> Result<Vec<String>, String> {
    if let Some(a) = args.get(n).filter(|a| !a.is_empty()) {
        return Ok(vec![a.clone()]);
    }
    let set = walk::knob_array(KNOB)?;
    if set.is_empty() {
        return Err(format!("{} names no install page", KNOB));
    }
    Ok(set)
}

// spec: installer/SPEC.md §The hosted install pin — a tree read takes every member's text, and a
// member that is no file refuses
pub(crate) fn read(paths: &[String], missing: &str) -> Result<Vec<Member>, String> {
    let mut out = Vec::new();
    for p in paths {
        if !Path::new(p).is_file() {
            return Err(format!("{}: {}", missing, p));
        }
        out.push(Member {
            path: p.clone(),
            text: fresh::read_captured(p)?,
        });
    }
    Ok(out)
}

// spec: installer/SPEC.md §The hosted install pin — a tag read takes the members the tag carries,
// and a tag carrying none refuses
pub(crate) fn at_tag(tag: &str, paths: &[String]) -> Result<Vec<Member>, String> {
    let mut out = Vec::new();
    for p in paths {
        let path = p.trim_start_matches("./");
        if pinned_release::carries(tag, path)? {
            out.push(Member {
                path: p.clone(),
                text: pinned_release::show_at(tag, path)?,
            });
        }
    }
    if out.is_empty() {
        return Err(format!("the tag {} exists and carries no {}", tag, paths.join(", ")));
    }
    Ok(out)
}

pub(crate) fn names(members: &[Member]) -> String {
    members.iter().map(|m| m.path.as_str()).collect::<Vec<&str>>().join(", ")
}

// spec: installer/SPEC.md §The hosted install pin — the one member carrying a block or line: none
// where no member does, and a refusal where two do
pub(crate) fn carrier<'a>(
    members: &'a [Member],
    what: &str,
    carries: impl Fn(&str) -> bool,
) -> Result<Option<&'a Member>, String> {
    let found: Vec<&Member> = members.iter().filter(|m| carries(&m.text)).collect();
    match found.as_slice() {
        [] => Ok(None),
        [one] => Ok(Some(one)),
        many => Err(format!(
            "{} install pages carry {} ({}); exactly one is admissible",
            many.len(),
            what,
            many.iter().map(|m| m.path.as_str()).collect::<Vec<&str>>().join(", ")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(path: &str, text: &str) -> Member {
        Member {
            path: path.to_string(),
            text: text.to_string(),
        }
    }

    // spec: installer/SPEC.md §The hosted install pin — one carrier, none, and the refusal of two
    #[test]
    fn a_block_is_read_from_the_one_member_carrying_it() {
        let set = [member("a.md", "x\n"), member("b.md", "<!-- k:begin -->\n"), member("c.md", "y\n")];
        let has = |t: &str| t.contains("<!-- k:begin -->");
        assert_eq!(carrier(&set, "a k block", has).unwrap().map(|m| m.path.as_str()), Some("b.md"));
        assert!(carrier(&set, "a z block", |t| t.contains("zzz")).unwrap().is_none());
        let two = [member("a.md", "<!-- k:begin -->\n"), member("b.md", "<!-- k:begin -->\n")];
        let e = carrier(&two, "a k block", has).err().unwrap();
        assert!(e.contains("2 install pages carry a k block (a.md, b.md)"), "{}", e);
        assert_eq!(names(&set), "a.md, b.md, c.md");
    }

    #[test]
    fn a_positional_page_wins_and_a_missing_member_refuses() {
        let args = vec!["x".to_string(), "one.md".to_string()];
        assert_eq!(paths(&args, 1), Ok(vec!["one.md".to_string()]));
        let e = read(&["no/such/install-docs.md".to_string()], "install page not found").err().unwrap();
        assert_eq!(e, "install page not found: no/such/install-docs.md");
    }
}
