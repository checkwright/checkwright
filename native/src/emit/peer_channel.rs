// spec: lifecycle-kit/SPEC.md §The peer request channel — the four arms over one channel directory:
// the peer's filing and ready notice, the receiver's list and reply, one writer per file.
use crate::exit::Fault;
use std::path::{Path, PathBuf};

pub const KNOBS: &[&str] = &[KNOB];

const KNOB: &str = "LIFECYCLE_KIT_PEER_CHANNEL";

pub const FILE_USAGE: &str = "usage: --emit peer-file <peer>\n  files the request on standard input — its ## need, ## repro, ## wanted and ## evidence sections — to the peer channel and prints its id";

pub const REPLY_USAGE: &str = "usage: --emit peer-reply <id> filed <slug>\n       --emit peer-reply <id> refused \"<reason>\"\n       --emit peer-reply <id> landed\n       --emit peer-reply --landed <slug>\n       --emit peer-reply --shipped <tag>\n  appends a state change to a request's reply, or moves every request the bulk form matches";

pub const READY_USAGE: &str = "usage: --emit peer-ready <peer>\n  prints one line per request <peer> filed that shipped in a release later than this binary";

const REQUEST_HEAD: &str = "# peer-request v1";
const REPLY_HEAD: &str = "# peer-reply v1";
const SECTIONS: [&str; 4] = ["need", "repro", "wanted", "evidence"];
const ID_LINE: &str = "id: ";

// spec: lifecycle-kit/SPEC.md §The peer request channel — `r<YYYYMMDD>-<8 hex>`
fn is_id(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 18
        && b[0] == b'r'
        && b[1..9].iter().all(u8::is_ascii_digit)
        && b[9] == b'-'
        && b[10..].iter().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(c))
}

fn is_utc_stamp(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 20
        && s.is_char_boundary(10)
        && super::kpi::is_iso_day(&s[..10])
        && b[10] == b'T'
        && b[13] == b':'
        && b[16] == b':'
        && b[19] == b'Z'
        && [11, 12, 14, 15, 17, 18].iter().all(|i| b[*i].is_ascii_digit())
}

fn utc_now() -> String {
    crate::hook::utc_stamp(super::kpi::now_epoch())
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — a fence opens on three or more backticks
// or tildes after leading spaces and closes on at least as many of the same character
fn fence(line: &str) -> Option<(u8, usize)> {
    let t = line.trim_start_matches(' ');
    let c = *t.as_bytes().first()?;
    if c != b'`' && c != b'~' {
        return None;
    }
    let n = t.bytes().take_while(|b| *b == c).count();
    (n >= 3).then_some((c, n))
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the four sections in order, each body not
// blank, a `## ` line inside fenced code body text, and nothing but blank lines before the first
fn sections(lines: &[&str]) -> Result<Vec<String>, String> {
    let mut bodies: Vec<String> = Vec::new();
    let mut open: Option<(u8, usize)> = None;
    for line in lines {
        if let Some((c, n)) = open {
            if fence(line).is_some_and(|(d, m)| d == c && m >= n) {
                open = None;
            }
        } else if let Some(f) = fence(line) {
            open = Some(f);
        } else if let Some(name) = line.strip_prefix("## ") {
            let name = name.trim();
            match SECTIONS.get(bodies.len()) {
                None => return Err(format!("a fifth section, `## {}`, follows `## evidence`", name)),
                Some(want) if *want != name => {
                    return Err(format!("`## {}` stands where `## {}` belongs", name, want));
                }
                Some(_) => {}
            }
            bodies.push(String::new());
            continue;
        }
        match bodies.last_mut() {
            Some(b) => {
                b.push_str(line);
                b.push('\n');
            }
            None if line.trim().is_empty() => {}
            None => return Err("text stands before `## need`".to_string()),
        }
    }
    if let Some(missing) = SECTIONS.get(bodies.len()) {
        return Err(format!("`## {}` is missing", missing));
    }
    if let Some(i) = bodies.iter().position(|b| b.trim().is_empty()) {
        return Err(format!("`## {}` has a blank body", SECTIONS[i]));
    }
    if first_line(&bodies[0]).contains('\t') {
        return Err("the first line of `## need` holds a tab, which would split peer-list's row".to_string());
    }
    Ok(bodies)
}

fn first_line(body: &str) -> String {
    body.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or_default().to_string()
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the id hashes the file's bytes with the id
// line read as `id: `, so it is computed before the id exists and checked after
fn id_of(day: &str, bytes_with_blank_id: &[u8]) -> String {
    format!("r{}-{}", day.replace('-', ""), &crate::sha256::hex(bytes_with_blank_id)[..8])
}

#[derive(Debug, PartialEq)]
struct Request {
    id: String,
    from: String,
    filed: String,
    need: String,
}

fn parse_request(id: &str, bytes: &[u8]) -> Result<Request, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "it is not UTF-8".to_string())?;
    let lines: Vec<&str> = text.lines().collect();
    if lines.first() != Some(&REQUEST_HEAD) {
        return Err(format!("line 1 is not `{}`", REQUEST_HEAD));
    }
    let field = |i: usize, key: &str| -> Result<&str, String> {
        lines
            .get(i)
            .and_then(|l| l.strip_prefix(key))
            .ok_or_else(|| format!("line {} is not `{}<value>`", i + 1, key))
    };
    let named = field(1, ID_LINE)?;
    let from = field(2, "from: ")?;
    let filed = field(3, "filed: ")?;
    if named != id {
        return Err(format!("its id line names {}, not the file's own id", named));
    }
    if !crate::queue::is_slug(from) {
        return Err(format!("from: '{}' is outside [a-z0-9][a-z0-9-]*", from));
    }
    if !is_utc_stamp(filed) {
        return Err(format!("filed: '{}' is not a UTC time in YYYY-MM-DDTHH:MM:SSZ", filed));
    }
    let start = text.find('\n').map(|i| i + 1).unwrap_or(text.len());
    let blank: Vec<u8> = [&bytes[..start], ID_LINE.as_bytes(), &bytes[start + ID_LINE.len() + id.len()..]].concat();
    if id_of(&filed[..10], &blank) != id {
        return Err("its id is not the filing date and hash of its bytes, so it was edited after filing".to_string());
    }
    let bodies = sections(&lines[4..])?;
    Ok(Request { id: id.to_string(), from: from.to_string(), filed: filed.to_string(), need: first_line(&bodies[0]) })
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Received,
    Filed(String),
    Refused(String),
    Landed,
    Shipped(String),
}

impl State {
    fn wire(&self) -> String {
        match self {
            State::Received => "received".to_string(),
            State::Filed(s) => format!("filed {}", s),
            State::Refused(r) => format!("refused {}", r),
            State::Landed => "landed".to_string(),
            State::Shipped(t) => format!("shipped {}", t),
        }
    }

    fn parse(s: &str) -> Option<State> {
        let (word, operand) = s.split_once(' ').unwrap_or((s, ""));
        match (word, operand) {
            ("filed", o) if crate::queue::is_slug(o) => Some(State::Filed(o.to_string())),
            ("refused", o) if !o.trim().is_empty() && !o.contains('\t') => Some(State::Refused(o.to_string())),
            ("landed", "") => Some(State::Landed),
            ("shipped", o) if is_tag(o) => Some(State::Shipped(o.to_string())),
            _ => None,
        }
    }
}

fn is_tag(t: &str) -> bool {
    !t.is_empty() && !t.chars().any(char::is_whitespace)
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the legal moves; `refused` and `shipped`
// are final
fn legal(from: &State, to: &State) -> bool {
    matches!(
        (from, to),
        (State::Received, State::Filed(_) | State::Refused(_))
            | (State::Filed(_), State::Filed(_) | State::Landed)
            | (State::Landed, State::Shipped(_))
    )
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — a request's state is its last reply line,
// and its date that line's; a request with no line past the header is received
fn parse_reply(text: &str) -> Result<Option<(String, State)>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(REPLY_HEAD) {
        return Err(format!("line 1 is not `{}`", REPLY_HEAD));
    }
    let mut last = None;
    for (n, line) in lines.enumerate() {
        let parsed = line
            .split_once(' ')
            .filter(|(d, _)| super::kpi::is_iso_day(d))
            .and_then(|(d, s)| State::parse(s).filter(|s| *s != State::Received).map(|s| (d.to_string(), s)));
        match parsed {
            Some(p) => last = Some(p),
            None => return Err(format!("line {} is not `<YYYY-MM-DD> <state>`: {}", n + 2, line)),
        }
    }
    Ok(last)
}

struct Entry {
    request: Request,
    date: String,
    state: State,
    reply: PathBuf,
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the channel value is refused when relative,
// missing, not a directory, or inside a git work tree; unset is `None`
fn channel() -> Result<Option<PathBuf>, Fault> {
    let value = crate::walk::knob_scalar(KNOB).map_err(Fault::refused)?;
    if value.is_empty() {
        return Ok(None);
    }
    let refuse = |why: &str| Err(Fault::refused(format!("{} is '{}', which {} — set it to an absolute directory outside every git work tree", KNOB, value, why)));
    if !crate::walk::host_absolute(&value) {
        return refuse("is relative");
    }
    let path = Path::new(&value);
    if !path.exists() {
        return refuse("does not exist");
    }
    if !path.is_dir() {
        return refuse("is not a directory");
    }
    match crate::walk::toplevel_in_opt(&value) {
        Ok(None) => {}
        Ok(Some(top)) => {
            return refuse(&format!("is inside the git work tree {}, where a commit could publish a request body", top));
        }
        Err(e) => return Err(Fault::tool(format!("{} is '{}', a directory git refuses: {}", KNOB, value, e))),
    }
    Ok(Some(path.to_path_buf()))
}

fn bound() -> Result<PathBuf, Fault> {
    channel()?.ok_or_else(|| Fault::refused(format!("{} is unset, so there is no channel to write — bind it to an absolute directory outside every git work tree", KNOB)))
}

fn entries(dir: &Path) -> Result<Vec<Entry>, Fault> {
    let mut ids: Vec<String> = crate::walk::list_dir(dir)
        .map_err(Fault::input)?
        .into_iter()
        .filter(|(_, is_dir)| !is_dir)
        .filter_map(|(n, _)| n.strip_suffix(".md").filter(|s| is_id(s)).map(str::to_string))
        .collect();
    ids.sort_unstable();
    ids.iter()
        .map(|id| {
            let path = dir.join(format!("{}.md", id));
            let bytes = std::fs::read(&path).map_err(|e| Fault::input(format!("cannot read {}: {}", path.display(), e)))?;
            let request =
                parse_request(id, &bytes).map_err(|e| Fault::input(format!("{} is malformed: {}", path.display(), e)))?;
            let reply = dir.join(format!("{}.reply", id));
            let last = match std::fs::read(&reply) {
                Ok(b) => parse_reply(&String::from_utf8_lossy(&b))
                    .map_err(|e| Fault::input(format!("{} is malformed: {}", reply.display(), e)))?,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                Err(e) => return Err(Fault::input(format!("cannot read {}: {}", reply.display(), e))),
            };
            let (date, state) = last.unwrap_or_else(|| (request.filed[..10].to_string(), State::Received));
            Ok(Entry { request, date, state, reply })
        })
        .collect()
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the two peer arms run from any directory:
// with no repository they raise the tree-free mark, so only an absolute locator configures them
fn stand_free() -> Result<(), Fault> {
    if crate::walk::toplevel_opt().map_err(Fault::tool)?.is_none() {
        crate::hook::raise_tree_free();
    }
    Ok(())
}

fn stdin() -> Result<Vec<u8>, Fault> {
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).map_err(|e| Fault::input(format!("cannot read standard input: {}", e)))?;
    Ok(buf)
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the header the filing arm stamps over the
// peer's sections, the id line blank for the hash
fn request_bytes(peer: &str, filed: &str, id: &str, body: &str) -> Vec<u8> {
    let mut out = format!("{}\n{}{}\nfrom: {}\nfiled: {}\n\n{}", REQUEST_HEAD, ID_LINE, id, peer, filed, body);
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.into_bytes()
}

pub fn file(args: &[String]) -> Result<String, Fault> {
    file_with(args, stdin, utc_now)
}

fn file_with(
    args: &[String],
    read: impl FnOnce() -> Result<Vec<u8>, Fault>,
    now: impl FnOnce() -> String,
) -> Result<String, Fault> {
    let [peer] = args else { return Err(Fault::usage(FILE_USAGE)) };
    if !crate::queue::is_slug(peer) {
        return Err(Fault::usage(format!("the peer '{}' is outside [a-z0-9][a-z0-9-]*\n{}", peer, FILE_USAGE)));
    }
    stand_free()?;
    let dir = bound()?;
    let raw = read()?;
    let body = std::str::from_utf8(&raw).map_err(|_| Fault::input("standard input is not UTF-8"))?;
    sections(&body.lines().collect::<Vec<_>>()).map_err(|e| Fault::input(format!("the request is malformed: {}", e)))?;
    let filed = now();
    let id = id_of(&filed[..10], &request_bytes(peer, &filed, "", body));
    create_exclusive(&dir, &id, &request_bytes(peer, &filed, &id, body))?;
    Ok(format!("{}\n", id))
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — a linked temporary file, so a collision
// refuses rather than overwrites; the collision is the one refusal that prints, the existing id alone
fn create_exclusive(dir: &Path, id: &str, bytes: &[u8]) -> Result<(), Fault> {
    let target = dir.join(format!("{}.md", id));
    let tmp = dir.join(format!(".{}.{}.tmp", id, std::process::id()));
    std::fs::write(&tmp, bytes).map_err(|e| Fault::tool(format!("cannot write {}: {}", tmp.display(), e)))?;
    let linked = std::fs::hard_link(&tmp, &target);
    let _ = std::fs::remove_file(&tmp);
    match linked {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Err(Fault::refused(format!(
            "{} already exists, as the same ask filed in the same second does, so the ask is filed already as {}",
            target.display(),
            id
        ))
        .with_stdout(id)),
        Err(e) => Err(Fault::tool(format!("cannot create {}: {}", target.display(), e))),
    }
}

fn row(e: &Entry) -> String {
    format!("{}\t{}\t{}\t{}\t{}\n", e.request.id, e.request.from, e.date, e.state.wire(), e.request.need)
}

pub fn list(_args: &[String]) -> Result<String, Fault> {
    let Some(dir) = channel()? else { return Ok(String::new()) };
    Ok(entries(&dir)?.iter().map(row).collect())
}

fn append(e: &Entry, to: &State) -> Result<String, Fault> {
    use std::io::Write;
    let line = format!("{} {}\n", &utc_now()[..10], to.wire());
    let fresh = !e.reply.exists();
    let text = if fresh { format!("{}\n{}", REPLY_HEAD, line) } else { line };
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&e.reply)
        .and_then(|mut f| f.write_all(text.as_bytes()))
        .map_err(|err| Fault::tool(format!("cannot append to {}: {}", e.reply.display(), err)))?;
    Ok(format!("{} {}\n", e.request.id, to.wire()))
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — a bulk form moves every match from the one
// state its flag names, and one matching nothing prints nothing
fn bulk(entries: &[Entry], matches: impl Fn(&State) -> bool, to: &State) -> Result<String, Fault> {
    entries.iter().filter(|e| matches(&e.state)).map(|e| append(e, to)).collect()
}

pub fn reply(args: &[String]) -> Result<String, Fault> {
    let usage = || Fault::usage(REPLY_USAGE);
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    let single = |id: &str, to: State| (Some(id.to_string()), to);
    let (id, to) = match a.as_slice() {
        ["--landed", slug] if crate::queue::is_slug(slug) => (None, State::Landed),
        ["--shipped", tag] if is_tag(tag) => (None, State::Shipped(tag.to_string())),
        [id, "filed", slug] if crate::queue::is_slug(slug) => single(id, State::Filed(slug.to_string())),
        [_, "refused", reason] if reason.contains('\t') => {
            return Err(Fault::usage(
                "the reason holds a tab, which would split peer-list's row — give one line of prose with no tab",
            ));
        }
        [id, "refused", reason] if !reason.trim().is_empty() && !reason.contains('\n') => {
            single(id, State::Refused(reason.to_string()))
        }
        [id, "landed"] => single(id, State::Landed),
        _ => return Err(usage()),
    };
    // spec: lifecycle-kit/SPEC.md §The peer request channel — an unbound channel holds no request for a
    // bulk form to move, while a targeted move names one and refuses
    let dir = match id {
        None => match channel()? {
            Some(dir) => dir,
            None => return Ok(String::new()),
        },
        Some(_) => bound()?,
    };
    let all = entries(&dir)?;
    match (id, a[0]) {
        (None, "--landed") => bulk(&all, |s| *s == State::Filed(a[1].to_string()), &to),
        (None, _) => bulk(&all, |s| *s == State::Landed, &to),
        (Some(id), _) => {
            let e = all
                .iter()
                .find(|e| e.request.id == id)
                .ok_or_else(|| Fault::refused(format!("no request {} is in the channel {}", id, dir.display())))?;
            if !legal(&e.state, &to) {
                return Err(Fault::refused(format!(
                    "request {} is {}, and the move to {} is not legal",
                    id,
                    e.state.wire(),
                    to.wire()
                )));
            }
            append(e, &to)
        }
    }
}

fn version(v: &str) -> Option<(u64, u64, u64)> {
    let mut parts = v.strip_prefix('v').unwrap_or(v).split('.');
    let mut field = || parts.next().filter(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))?.parse().ok();
    let triple = (field()?, field()?, field()?);
    parts.next().is_none().then_some(triple)
}

// spec: lifecycle-kit/SPEC.md §The peer request channel — the stamp the notice prints where `tag` is
// a later release than the binary's, `unknown` where either side does not parse, none where adopted
fn unadopted(tag: &str, stamp: &str) -> Option<String> {
    let base = if crate::release_stamp::has_distance(stamp) { stamp.rsplitn(3, '-').nth(2).unwrap_or(stamp) } else { stamp };
    match (version(tag), version(base)) {
        (Some(t), Some(s)) => (t > s).then(|| stamp.to_string()),
        _ => Some("unknown".to_string()),
    }
}

fn notices(entries: &[Entry], peer: &str, stamp: &str) -> String {
    entries
        .iter()
        .filter(|e| e.request.from == peer)
        .filter_map(|e| match &e.state {
            State::Shipped(tag) => unadopted(tag, stamp).map(|shown| {
                format!(
                    "peer request {} shipped in {}, not in this binary ({}): {}\n",
                    e.request.id, tag, shown, e.request.need
                )
            }),
            _ => None,
        })
        .collect()
}

pub fn ready(args: &[String]) -> Result<String, Fault> {
    let [peer] = args else { return Err(Fault::usage(READY_USAGE)) };
    if !crate::queue::is_slug(peer) {
        return Err(Fault::usage(format!("the peer '{}' is outside [a-z0-9][a-z0-9-]*\n{}", peer, READY_USAGE)));
    }
    stand_free()?;
    let Some(dir) = channel()? else { return Ok(String::new()) };
    Ok(notices(&entries(&dir)?, peer, env!("CHECKWRIGHT_RELEASE")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "## need\nthe gate refuses a path\n\n## repro\nrun it\n\n## wanted\nadmit it\n\n## evidence\nthe log\n";

    fn lines(s: &str) -> Vec<&str> {
        s.lines().collect()
    }

    fn filed(peer: &str, at: &str, body: &str) -> (String, Vec<u8>) {
        let id = id_of(&at[..10], &request_bytes(peer, at, "", body));
        let bytes = request_bytes(peer, at, &id, body);
        (id, bytes)
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the four sections in order, none blank,
    // and a `## ` line in a fence is body text
    #[test]
    fn the_section_grammar_names_each_defect() {
        assert_eq!(sections(&lines(BODY)).map(|b| first_line(&b[0])), Ok("the gate refuses a path".to_string()));
        let fenced = BODY.replace("run it\n", "```\n## not a heading\n```\n");
        assert!(sections(&lines(&fenced)).is_ok(), "a fenced heading split the section");
        let cases = [
            (BODY.replace("## repro", "## wanted"), "`## wanted` stands where `## repro` belongs"),
            (format!("{}## extra\nmore\n", BODY), "a fifth section, `## extra`"),
            (BODY.replace("## evidence\nthe log\n", ""), "`## evidence` is missing"),
            (BODY.replace("admit it", "  "), "`## wanted` has a blank body"),
            (format!("preamble\n{}", BODY), "text stands before `## need`"),
        ];
        for (text, want) in cases {
            let err = sections(&lines(&text)).expect_err(want);
            assert!(err.contains(want), "{:?} gave {}", want, err);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the id is the filing day and the hash of
    // the bytes with a blank id line, and a byte edited after filing is caught
    #[test]
    fn a_filed_request_parses_and_an_edit_after_filing_is_refused() {
        let (id, bytes) = filed("peer-a", "2026-11-24T10:20:30Z", BODY);
        assert!(is_id(&id) && id.starts_with("r20261124-"), "{}", id);
        let r = parse_request(&id, &bytes).expect("a filed request did not parse");
        assert_eq!((r.from.as_str(), r.need.as_str()), ("peer-a", "the gate refuses a path"));
        let edited = String::from_utf8(bytes.clone()).unwrap().replace("admit it", "admit all");
        assert!(parse_request(&id, edited.as_bytes()).unwrap_err().contains("edited after filing"));
        let (other, _) = filed("peer-a", "2026-11-24T10:20:31Z", BODY);
        assert_ne!(id, other, "two filings a second apart shared an id");
        assert!(parse_request("r20261124-00000000", &bytes).unwrap_err().contains("not the file's own id"));
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the legal moves and nothing else
    #[test]
    fn only_the_legal_moves_are_admitted() {
        let f = || State::Filed("a".into());
        let x = || State::Refused("no".into());
        let s = || State::Shipped("v1.0.0".into());
        for (from, to) in [(State::Received, f()), (State::Received, x()), (f(), f()), (f(), State::Landed), (State::Landed, s())] {
            assert!(legal(&from, &to), "{:?} -> {:?} was refused", from, to);
        }
        for (from, to) in [(State::Received, State::Landed), (x(), f()), (s(), s()), (State::Landed, f()), (f(), s())] {
            assert!(!legal(&from, &to), "{:?} -> {:?} was admitted", from, to);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the last line is the state, and a line
    // outside the grammar is refused
    #[test]
    fn a_reply_reads_its_last_line() {
        let text = format!("{}\n2026-11-24 filed a-slug\n2026-11-25 landed\n", REPLY_HEAD);
        assert_eq!(parse_reply(&text), Ok(Some(("2026-11-25".into(), State::Landed))));
        assert_eq!(parse_reply(&format!("{}\n", REPLY_HEAD)), Ok(None));
        assert!(parse_reply("2026-11-24 landed\n").is_err());
        assert!(parse_reply(&format!("{}\n2026-11-24 received\n", REPLY_HEAD)).is_err());
        assert!(parse_reply(&format!("{}\n2026-11-24 filed Bad_Slug\n", REPLY_HEAD)).is_err());
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — a later release prints, the binary's own
    // or an earlier one does not, a distance suffix compares by its base, and an unparseable side is
    // `unknown`
    #[test]
    fn the_notice_compares_against_the_binary_stamp() {
        assert_eq!(unadopted("v0.37.0", "0.36.0"), Some("0.36.0".into()));
        assert_eq!(unadopted("v0.37.0", "0.36.0-6-gabc1234"), Some("0.36.0-6-gabc1234".into()));
        assert_eq!(unadopted("v0.36.0", "0.36.0-6-gabc1234"), None);
        assert_eq!(unadopted("0.35.9", "0.36.0"), None);
        assert_eq!(unadopted("v0.37.0", ""), Some("unknown".into()));
        assert_eq!(unadopted("v0.37.0-rc1", "0.36.0"), Some("unknown".into()));
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the list row and the notice line
    #[test]
    fn the_row_and_the_notice_carry_their_fields() {
        let (id, bytes) = filed("peer-a", "2026-11-24T10:20:30Z", BODY);
        let e = |state: State| Entry {
            request: parse_request(&id, &bytes).unwrap(),
            date: "2026-11-30".into(),
            state,
            reply: PathBuf::new(),
        };
        assert_eq!(row(&e(State::Landed)), format!("{}\tpeer-a\t2026-11-30\tlanded\tthe gate refuses a path\n", id));
        let shipped = [e(State::Shipped("v9.0.0".into()))];
        assert_eq!(
            notices(&shipped, "peer-a", "0.36.0"),
            format!("peer request {} shipped in v9.0.0, not in this binary (0.36.0): the gate refuses a path\n", id)
        );
        assert_eq!(notices(&shipped, "peer-b", "0.36.0"), "");
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — arity and shape misuse is a usage
    // refusal before any channel is read
    #[test]
    fn misuse_is_refused_before_the_channel_is_read() {
        let _knobs = crate::knobenv::lock();
        let argv = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        for a in [&[][..], &["Bad_Peer"][..], &["a", "b"][..]] {
            assert!(file(&argv(a)).is_err(), "{:?}", a);
            assert!(ready(&argv(a)).is_err(), "{:?}", a);
        }
        for a in [&[][..], &["r1", "filed"][..], &["r1", "refused", ""][..], &["--landed"][..], &["--shipped", "a b"][..], &["r1", "shipped", "v1"][..]] {
            assert_eq!(reply(&argv(a)), Err(Fault::usage(REPLY_USAGE)), "{:?}", a);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — a filing time with a multibyte character
    // across byte 10 is malformed, never a panic
    #[test]
    fn a_multibyte_filing_time_is_malformed() {
        assert!(is_utc_stamp("2026-11-24T10:20:30Z"));
        let skewed = "2026-11-2é10:20:30Z";
        assert_eq!(skewed.len(), 20);
        assert!(!is_utc_stamp(skewed));
        let (id, bytes) = filed("peer-a", "2026-11-24T10:20:30Z", BODY);
        let text = String::from_utf8(bytes).unwrap().replace("2026-11-24T10:20:30Z", skewed);
        assert!(parse_request(&id, text.as_bytes()).unwrap_err().contains("is not a UTC time"));
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the need line and a refused reason carry
    // no tab, since peer-list prints both into a tab-separated row
    #[test]
    fn a_tab_is_refused_where_a_row_prints_it() {
        let _knobs = crate::knobenv::lock();
        let tabbed = BODY.replace("the gate refuses a path", "the gate\trefuses a path");
        assert!(sections(&lines(&tabbed)).unwrap_err().contains("holds a tab"));
        assert!(sections(&lines(&BODY.replace("run it", "run\tit"))).is_ok(), "a tab off the need line was refused");
        assert_eq!(State::parse("refused out\tof scope"), None);
        let argv = ["r20261124-00000000", "refused", "out\tof scope"].map(String::from);
        assert!(reply(&argv).unwrap_err().contains("holds a tab"));
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — the same ask from one peer in one second
    // hashes to one id, and the second filing refuses, leaving the first byte-identical
    #[test]
    fn a_same_second_refiling_collides_and_refuses() {
        let (id, bytes) = filed("peer-a", "2026-11-24T10:20:30Z", BODY);
        assert_eq!(filed("peer-a", "2026-11-24T10:20:30Z", BODY).0, id);
        let dir = std::env::temp_dir().join(format!("cw-peer-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let first = create_exclusive(&dir, &id, &bytes);
        let second = create_exclusive(&dir, &id, b"other");
        let kept = std::fs::read(dir.join(format!("{}.md", id)));
        let left = crate::walk::list_dir(&dir).map(|d| d.len());
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(first, Ok(()));
        assert!(second.unwrap_err().contains("already exists"));
        assert_eq!(kept.unwrap(), bytes);
        assert_eq!(left.unwrap(), 1, "a temporary file was left behind");
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — a collision is exit 2 with the existing
    // id alone on stdout, and every other refusal or error of the arm prints nothing there
    #[test]
    fn a_collision_is_exit_2_with_the_id_alone_on_stdout() {
        let env = crate::knobenv::lock();
        let dir = std::env::temp_dir().join(format!("cw-peer-collide-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        env.set(KNOB, &dir.display().to_string());
        let at = "2026-11-24T10:20:30Z";
        let peer = ["peer-a".to_string()];
        let body = || Ok(BODY.as_bytes().to_vec());
        let first = file_with(&peer, body, || at.to_string());
        let again = file_with(&peer, body, || at.to_string());
        let malformed = file_with(&peer, || Ok(b"no sections\n".to_vec()), || at.to_string());
        let outside = file_with(&["Bad_Peer".to_string()], body, || at.to_string());
        let (other_id, _) = filed("peer-a", "2026-11-24T10:20:31Z", BODY);
        std::fs::create_dir(dir.join(format!(".{}.{}.tmp", other_id, std::process::id()))).unwrap();
        let unwritable = file_with(&peer, body, || "2026-11-24T10:20:31Z".to_string());
        env.remove(KNOB);
        let unbound = file_with(&peer, body, || at.to_string());
        let _ = std::fs::remove_dir_all(&dir);
        let id = first.expect("the first filing was refused");
        let again = again.expect_err("the same ask in the same second filed twice");
        assert_eq!(again.status(), 2);
        assert_eq!(again.stdout.as_deref().map(|l| format!("{}\n", l)), Some(id), "stdout is the id and a newline");
        for (name, r, status) in [("unbound", unbound, 2), ("malformed", malformed, 3), ("outside", outside, 2), ("unwritable", unwritable, 3)] {
            let f = r.expect_err(name);
            assert_eq!((f.status(), f.stdout), (status, None), "{}: {}", name, f.message);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The peer request channel — a fence closes only on its own character
    #[test]
    fn a_fence_closes_on_its_own_character() {
        assert_eq!(fence("```rust"), Some((b'`', 3)));
        assert_eq!(fence("  ~~~~"), Some((b'~', 4)));
        assert_eq!(fence("``"), None);
        let text = BODY.replace("run it\n", "~~~\n```\n## inside\n~~~\n");
        assert!(sections(&lines(&text)).is_ok());
    }
}
