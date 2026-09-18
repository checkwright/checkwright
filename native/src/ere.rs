// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the crate's one pattern mechanism for
// consumer-supplied regexes: a hand-written POSIX ERE matcher with leftmost-longest span
// reporting, carrying no vocabulary and no substitution engine
use std::fmt;

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — a construct outside the accepted grammar
// is a refusal a member turns into exit 2, never a literal the scan then mis-reads
#[derive(Debug)]
pub struct EreError {
    what: String,
}

impl fmt::Display for EreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

fn err<T>(what: String) -> Result<T, EreError> {
    Err(EreError { what })
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — byte-wise, the C-locale semantics awk's
// RSTART/RLENGTH arithmetic is reproduced against; a char-wise engine shifts every span
// offset its callers hand to a byte-indexed substr
#[derive(Clone)]
struct Set {
    bits: [u64; 4],
}

impl Set {
    fn new() -> Self {
        Set { bits: [0; 4] }
    }
    fn one(b: u8) -> Self {
        let mut s = Set::new();
        s.add(b);
        s
    }
    fn all() -> Self {
        Set {
            bits: [u64::MAX; 4],
        }
    }
    fn add(&mut self, b: u8) {
        self.bits[(b >> 6) as usize] |= 1u64 << (b & 63);
    }
    fn add_range(&mut self, lo: u8, hi: u8) {
        for b in lo..=hi {
            self.add(b);
        }
    }
    fn negate(&mut self) {
        for w in self.bits.iter_mut() {
            *w = !*w;
        }
    }
    fn has(&self, b: u8) -> bool {
        self.bits[(b >> 6) as usize] & (1u64 << (b & 63)) != 0
    }
}

enum Node {
    Empty,
    Set(Set),
    Cat(Vec<Node>),
    Alt(Vec<Node>),
    Rep(Box<Node>, usize, Option<usize>),
    Group(Box<Node>),
    Bol,
    Eol,
}

enum Inst {
    Byte(Set),
    Split(usize, usize),
    Jmp(usize),
    Bol,
    Eol,
    Match,
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — POSIX RE_DUP_MAX bounds an interval, and
// the compiled program is bounded too: an interval expands by copying, so an unbounded
// bound is a memory fault wearing a pattern's clothes
const DUP_MAX: usize = 255;
const PROG_MAX: usize = 20000;

pub struct Ere {
    prog: Vec<Inst>,
}

impl Ere {
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the whole POSIX ERE grammar, sized to
    // the language rather than to this consumer's patterns
    pub fn compile(pattern: &str) -> Result<Ere, EreError> {
        let bytes = pattern.as_bytes();
        let mut p = Parser { b: bytes, i: 0 };
        let node = p.parse_alt()?;
        if p.i < bytes.len() {
            return err(format!(
                "unbalanced ')' at byte {} of the pattern",
                p.i
            ));
        }
        Ere::from_nodes(std::slice::from_ref(&node))
    }

    fn from_nodes(nodes: &[Node]) -> Result<Ere, EreError> {
        let mut prog: Vec<Inst> = Vec::new();
        for n in nodes {
            emit(n, &mut prog)?;
        }
        prog.push(Inst::Match);
        Ok(Ere { prog })
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — awk's `$0 ~ p`: does the pattern match
    // anywhere in the subject
    pub fn is_match(&self, hay: &str) -> bool {
        let b = hay.as_bytes();
        let len = b.len();
        let mut seen = vec![usize::MAX; self.prog.len()];
        let mut clist: Vec<usize> = Vec::new();
        let mut nlist: Vec<usize> = Vec::new();
        self.add(&mut clist, &mut seen, 0, 0, 0, len);
        let mut pos = 0usize;
        loop {
            if clist.iter().any(|&pc| matches!(self.prog[pc], Inst::Match)) {
                return true;
            }
            if pos == len {
                return false;
            }
            let c = b[pos];
            nlist.clear();
            let gen = pos + 1;
            for &pc in clist.iter() {
                if let Inst::Byte(s) = &self.prog[pc] {
                    if s.has(c) {
                        self.add(&mut nlist, &mut seen, gen, pc + 1, pos + 1, len);
                    }
                }
            }
            // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — an unanchored search seeds a
            // fresh start at every position, so `^` still fails everywhere but position 0
            self.add(&mut nlist, &mut seen, gen, 0, pos + 1, len);
            std::mem::swap(&mut clist, &mut nlist);
            pos += 1;
        }
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the leftmost-longest span as byte
    // offsets, the `RSTART`/`RLENGTH` pair awk reports: the earliest start that matches at
    // all, and from it the longest end
    pub fn find(&self, hay: &str) -> Option<(usize, usize)> {
        self.find_from(hay, 0)
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the leftmost-longest span starting at or
    // after `from`, searched in the whole subject so `^` still holds only at its position 0
    pub fn find_from(&self, hay: &str, from: usize) -> Option<(usize, usize)> {
        let b = hay.as_bytes();
        let mut seen = vec![usize::MAX; self.prog.len()];
        // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — one monotone generation counter
        // across every start, or a later start reuses an earlier one's stamp and drops the
        // states it should have added
        let mut gen = 0usize;
        for start in from..=b.len() {
            if let Some(end) = self.longest_from(b, start, &mut seen, &mut gen) {
                return Some((start, end));
            }
        }
        None
    }

    fn longest_from(
        &self,
        b: &[u8],
        start: usize,
        seen: &mut [usize],
        gen: &mut usize,
    ) -> Option<usize> {
        let mut best: Option<usize> = None;
        self.ends_from(b, start, seen, gen, |pos| best = Some(pos));
        best
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — every end at which the pattern matches
    // exactly from `start`, anchors held at their absolute subject positions
    fn match_ends(&self, b: &[u8], start: usize) -> Vec<bool> {
        let mut seen = vec![usize::MAX; self.prog.len()];
        let mut gen = 0usize;
        let mut ends = vec![false; b.len() + 1];
        self.ends_from(b, start, &mut seen, &mut gen, |pos| ends[pos] = true);
        ends
    }

    fn ends_from(
        &self,
        b: &[u8],
        start: usize,
        seen: &mut [usize],
        gen: &mut usize,
        mut on_end: impl FnMut(usize),
    ) {
        let len = b.len();
        let mut clist: Vec<usize> = Vec::new();
        let mut nlist: Vec<usize> = Vec::new();
        *gen += 1;
        self.add(&mut clist, seen, *gen, 0, start, len);
        let mut pos = start;
        loop {
            if clist.iter().any(|&pc| matches!(self.prog[pc], Inst::Match)) {
                on_end(pos);
            }
            if pos == len || clist.is_empty() {
                return;
            }
            let c = b[pos];
            nlist.clear();
            *gen += 1;
            for &pc in clist.iter() {
                if let Inst::Byte(s) = &self.prog[pc] {
                    if s.has(c) {
                        self.add(&mut nlist, seen, *gen, pc + 1, pos + 1, len);
                    }
                }
            }
            std::mem::swap(&mut clist, &mut nlist);
            pos += 1;
        }
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the epsilon closure, with the two
    // anchors resolved at closure time against the position the thread would occupy; a
    // generation stamp rather than a cleared flag array, so a subject's cost stays linear
    fn add(&self, list: &mut Vec<usize>, seen: &mut [usize], gen: usize, pc0: usize, pos: usize, len: usize) {
        let mut stack = vec![pc0];
        while let Some(pc) = stack.pop() {
            if seen[pc] == gen {
                continue;
            }
            seen[pc] = gen;
            match &self.prog[pc] {
                Inst::Jmp(t) => stack.push(*t),
                Inst::Split(a, c) => {
                    stack.push(*c);
                    stack.push(*a);
                }
                Inst::Bol => {
                    if pos == 0 {
                        stack.push(pc + 1);
                    }
                }
                Inst::Eol => {
                    if pos == len {
                        stack.push(pc + 1);
                    }
                }
                _ => list.push(pc),
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the one-group capture: the group stands in
// the top-level concatenation, so the pattern splits into prefix, group and suffix and the
// subpattern rule is answered with the span engine alone
pub struct EreCapture {
    whole: Ere,
    prefix: Ere,
    group: Ere,
    suffix: Ere,
}

fn count_groups(n: &Node) -> usize {
    match n {
        Node::Group(inner) => 1 + count_groups(inner),
        Node::Rep(inner, _, _) => count_groups(inner),
        Node::Cat(v) | Node::Alt(v) => v.iter().map(count_groups).sum(),
        Node::Empty | Node::Set(_) | Node::Bol | Node::Eol => 0,
    }
}

fn contains_alt(n: &Node) -> bool {
    match n {
        Node::Alt(_) => true,
        Node::Group(inner) | Node::Rep(inner, _, _) => contains_alt(inner),
        Node::Cat(v) => v.iter().any(contains_alt),
        Node::Empty | Node::Set(_) | Node::Bol | Node::Eol => false,
    }
}

impl EreCapture {
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — any shape outside the admitted one is a
    // refusal naming the rule it broke, never an approximated span
    pub fn compile(pattern: &str) -> Result<EreCapture, EreError> {
        let bytes = pattern.as_bytes();
        let mut p = Parser { b: bytes, i: 0 };
        let node = p.parse_alt()?;
        if p.i < bytes.len() {
            return err(format!("unbalanced ')' at byte {} of the pattern", p.i));
        }
        match count_groups(&node) {
            0 => return err("the pattern declares no capture group — exactly one is required".to_string()),
            1 => {}
            _ => return err("the pattern declares a second capture group — exactly one is admitted".to_string()),
        }
        let items: Vec<Node> = match node {
            Node::Alt(_) => {
                return err(
                    "the capture group sits inside an alternation — it must stand in the \
                     pattern's top-level concatenation"
                        .to_string(),
                )
            }
            Node::Cat(v) => v,
            other => vec![other],
        };
        let k = match items.iter().position(|n| matches!(n, Node::Group(_))) {
            Some(k) => k,
            None => {
                return err(
                    "the capture group is quantified — its POSIX span would be its last \
                     iteration, which the one-group capture does not implement"
                        .to_string(),
                )
            }
        };
        // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the host regcomp resolves an alternated
        // group body by first alternative, not longest, so admitting one changes a consumer's pid
        if let Node::Group(body) = &items[k] {
            if contains_alt(body) {
                return err(
                    "the capture group's body contains an alternation — the one-group capture \
                     admits none"
                        .to_string(),
                );
            }
        }
        Ok(EreCapture {
            whole: Ere::from_nodes(&items)?,
            prefix: Ere::from_nodes(&items[..k])?,
            group: Ere::from_nodes(&items[k..=k])?,
            suffix: Ere::from_nodes(&items[k + 1..])?,
        })
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — within the leftmost-longest whole match,
    // the largest group start the prefix can reach, then the largest group end the suffix
    // can still finish from
    pub fn capture(&self, hay: &str) -> Option<(usize, usize)> {
        let (s, e) = self.whole.find(hay)?;
        let b = hay.as_bytes();
        let prefix_ends = self.prefix.match_ends(b, s);
        for i in (s..=e).rev().filter(|&i| prefix_ends[i]) {
            let group_ends = self.group.match_ends(b, i);
            for j in (i..=e).rev().filter(|&j| group_ends[j]) {
                if self.suffix.match_ends(b, j)[e] {
                    return Some((i, j));
                }
            }
        }
        None
    }
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — Thompson construction: an interval
// expands by copying its operand, which is why the bounds and the program are capped
fn emit(n: &Node, prog: &mut Vec<Inst>) -> Result<(), EreError> {
    if prog.len() > PROG_MAX {
        return err(format!(
            "pattern compiles to more than {} instructions — an interval bound expands by \
             copying its operand",
            PROG_MAX
        ));
    }
    match n {
        Node::Empty => {}
        Node::Bol => prog.push(Inst::Bol),
        Node::Eol => prog.push(Inst::Eol),
        Node::Set(s) => prog.push(Inst::Byte(s.clone())),
        Node::Group(inner) => emit(inner, prog)?,
        Node::Cat(v) => {
            for c in v {
                emit(c, prog)?;
            }
        }
        Node::Alt(v) => {
            let mut jumps: Vec<usize> = Vec::new();
            for (k, branch) in v.iter().enumerate() {
                if k + 1 < v.len() {
                    let split = prog.len();
                    prog.push(Inst::Split(0, 0));
                    emit(branch, prog)?;
                    jumps.push(prog.len());
                    prog.push(Inst::Jmp(0));
                    let next = prog.len();
                    prog[split] = Inst::Split(split + 1, next);
                } else {
                    emit(branch, prog)?;
                }
            }
            let end = prog.len();
            for j in jumps {
                prog[j] = Inst::Jmp(end);
            }
        }
        Node::Rep(inner, lo, hi) => {
            for _ in 0..*lo {
                emit(inner, prog)?;
            }
            match hi {
                None => {
                    let split = prog.len();
                    prog.push(Inst::Split(0, 0));
                    emit(inner, prog)?;
                    prog.push(Inst::Jmp(split));
                    let next = prog.len();
                    prog[split] = Inst::Split(split + 1, next);
                }
                Some(h) => {
                    let mut splits: Vec<usize> = Vec::new();
                    for _ in *lo..*h {
                        splits.push(prog.len());
                        prog.push(Inst::Split(0, 0));
                        emit(inner, prog)?;
                    }
                    let end = prog.len();
                    for s in splits {
                        prog[s] = Inst::Split(s + 1, end);
                    }
                }
            }
        }
    }
    Ok(())
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the specials a backslash may escape; a
// backslash before anything else is refused rather than read as that literal
const ESCAPABLE: &[u8] = b".[]\\()*+?{}|^$";

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the GNU ERE extensions this engine
// refuses by name, because a pattern that means one of them and scans as a literal reports
// a clean verdict off a scan that never ran
const GNU_ESCAPES: &[u8] = b"ywWsSbB<>`'";

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn at(&self, k: usize) -> Option<u8> {
        self.b.get(self.i + k).copied()
    }

    fn parse_alt(&mut self) -> Result<Node, EreError> {
        let mut branches = vec![self.parse_cat()?];
        while self.peek() == Some(b'|') {
            self.i += 1;
            branches.push(self.parse_cat()?);
        }
        if branches.len() == 1 {
            Ok(branches.pop().unwrap_or(Node::Empty))
        } else {
            Ok(Node::Alt(branches))
        }
    }

    fn parse_cat(&mut self) -> Result<Node, EreError> {
        let mut items: Vec<Node> = Vec::new();
        while let Some(c) = self.peek() {
            if c == b'|' || c == b')' {
                break;
            }
            items.push(self.parse_rep(items.is_empty())?);
        }
        match items.len() {
            0 => Ok(Node::Empty),
            1 => Ok(items.pop().unwrap_or(Node::Empty)),
            _ => Ok(Node::Cat(items)),
        }
    }

    fn parse_rep(&mut self, first: bool) -> Result<Node, EreError> {
        let mut n = self.parse_atom(first)?;
        loop {
            match self.peek() {
                Some(b'*') => {
                    self.i += 1;
                    n = Node::Rep(Box::new(n), 0, None);
                }
                Some(b'+') => {
                    self.i += 1;
                    n = Node::Rep(Box::new(n), 1, None);
                }
                Some(b'?') => {
                    self.i += 1;
                    n = Node::Rep(Box::new(n), 0, Some(1));
                }
                Some(b'{') => match self.try_interval()? {
                    Some((lo, hi)) => n = Node::Rep(Box::new(n), lo, hi),
                    None => break,
                },
                _ => break,
            }
        }
        Ok(n)
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — a `{` that does not open a well-formed
    // interval is a literal brace, which is what awk does and what a prose vocabulary
    // spelling `${VAR}` depends on
    fn try_interval(&mut self) -> Result<Option<(usize, Option<usize>)>, EreError> {
        let save = self.i;
        let mut j = self.i + 1;
        let ds = j;
        while j < self.b.len() && self.b[j].is_ascii_digit() {
            j += 1;
        }
        if j == ds {
            return Ok(None);
        }
        let lo: usize = match std::str::from_utf8(&self.b[ds..j]).ok().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return Ok(None),
        };
        let hi: Option<usize>;
        if self.b.get(j) == Some(&b'}') {
            hi = Some(lo);
            j += 1;
        } else if self.b.get(j) == Some(&b',') {
            j += 1;
            let hs = j;
            while j < self.b.len() && self.b[j].is_ascii_digit() {
                j += 1;
            }
            if self.b.get(j) != Some(&b'}') {
                self.i = save;
                return Ok(None);
            }
            if hs == j {
                hi = None;
            } else {
                hi = std::str::from_utf8(&self.b[hs..j]).ok().and_then(|s| s.parse().ok());
                if hi.is_none() {
                    self.i = save;
                    return Ok(None);
                }
            }
            j += 1;
        } else {
            self.i = save;
            return Ok(None);
        }
        if lo > DUP_MAX || hi.map(|h| h > DUP_MAX).unwrap_or(false) {
            return err(format!(
                "interval bound above the POSIX RE_DUP_MAX of {}",
                DUP_MAX
            ));
        }
        if let Some(h) = hi {
            if h < lo {
                return err("interval's upper bound is below its lower bound".to_string());
            }
        }
        self.i = j;
        Ok(Some((lo, hi)))
    }

    fn parse_atom(&mut self, first: bool) -> Result<Node, EreError> {
        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(Node::Empty),
        };
        match c {
            b'(' => {
                self.i += 1;
                let inner = self.parse_alt()?;
                if self.peek() != Some(b')') {
                    return err("unbalanced '(' — no closing ')'".to_string());
                }
                self.i += 1;
                Ok(Node::Group(Box::new(inner)))
            }
            b'.' => {
                self.i += 1;
                Ok(Node::Set(Set::all()))
            }
            b'[' => self.parse_bracket(),
            b'^' => {
                self.i += 1;
                Ok(Node::Bol)
            }
            b'$' => {
                self.i += 1;
                Ok(Node::Eol)
            }
            b'*' | b'+' | b'?' if first => err(format!(
                "'{}' with nothing to repeat at byte {} of the pattern",
                c as char, self.i
            )),
            b'\\' => self.parse_escape(),
            _ => {
                self.i += 1;
                Ok(Node::Set(Set::one(c)))
            }
        }
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — delta (3)'s refusal, named by what the
    // consumer wrote: a backreference and a GNU word-boundary form each say so
    fn parse_escape(&mut self) -> Result<Node, EreError> {
        let e = match self.at(1) {
            Some(e) => e,
            None => return err("pattern ends in a trailing backslash".to_string()),
        };
        if ESCAPABLE.contains(&e) {
            self.i += 2;
            return Ok(Node::Set(Set::one(e)));
        }
        if e.is_ascii_digit() {
            return err(format!(
                "'\\{}' is a backreference, which POSIX ERE does not carry and the compiled \
                 substrate does not implement",
                e as char
            ));
        }
        if GNU_ESCAPES.contains(&e) {
            return err(format!(
                "'\\{}' is a GNU regex extension the compiled substrate does not implement",
                e as char
            ));
        }
        err(format!(
            "'\\{}' is an escape outside the POSIX ERE grammar — escape a special character \
             ({}), and write an ordinary character bare",
            e as char,
            String::from_utf8_lossy(ESCAPABLE)
        ))
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — a bracket expression with ranges,
    // negation and the POSIX character classes; a backslash inside one is refused rather
    // than resolved, because POSIX reads it as a literal and GNU reads it as an escape
    fn parse_bracket(&mut self) -> Result<Node, EreError> {
        let mut set = Set::new();
        self.i += 1;
        let negate = self.peek() == Some(b'^');
        if negate {
            self.i += 1;
        }
        let mut first = true;
        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return err("unterminated bracket expression — no closing ']'".to_string()),
            };
            if c == b']' && !first {
                self.i += 1;
                break;
            }
            first = false;
            if c == b'\\' {
                return err(
                    "a backslash inside a bracket expression is ambiguous — POSIX reads it as a \
                     literal backslash and GNU as an escape, so the compiled substrate refuses \
                     it rather than picking one"
                        .to_string(),
                );
            }
            if c == b'[' && matches!(self.at(1), Some(b':')) {
                self.class_into(&mut set)?;
                continue;
            }
            if c == b'[' && matches!(self.at(1), Some(b'.') | Some(b'=')) {
                return err(
                    "a collating-symbol or equivalence-class bracket ([. .] / [= =]) is outside \
                     the grammar the compiled substrate implements"
                        .to_string(),
                );
            }
            self.i += 1;
            if self.peek() == Some(b'-')
                && !matches!(self.at(1), None | Some(b']'))
                && !matches!(self.at(1), Some(b'\\'))
            {
                let hi = match self.at(1) {
                    Some(h) => h,
                    None => return err("unterminated bracket expression".to_string()),
                };
                if hi < c {
                    return err(format!(
                        "bracket range '{}-{}' runs backwards",
                        c as char, hi as char
                    ));
                }
                set.add_range(c, hi);
                self.i += 2;
            } else {
                set.add(c);
            }
        }
        if negate {
            set.negate();
        }
        Ok(Node::Set(set))
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the POSIX character classes on their
    // C-locale definitions, the same locale the differential oracle runs awk in
    fn class_into(&mut self, set: &mut Set) -> Result<(), EreError> {
        let start = self.i + 2;
        let mut j = start;
        while j + 1 < self.b.len() && !(self.b[j] == b':' && self.b[j + 1] == b']') {
            j += 1;
        }
        if j + 1 >= self.b.len() {
            return err("unterminated character class — no closing ':]'".to_string());
        }
        let name = String::from_utf8_lossy(&self.b[start..j]).into_owned();
        let f: fn(u8) -> bool = match name.as_str() {
            "alpha" => |c| c.is_ascii_alphabetic(),
            "digit" => |c| c.is_ascii_digit(),
            "alnum" => |c| c.is_ascii_alphanumeric(),
            "upper" => |c| c.is_ascii_uppercase(),
            "lower" => |c| c.is_ascii_lowercase(),
            "space" => |c| matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r'),
            "blank" => |c| matches!(c, b' ' | b'\t'),
            "punct" => |c| c.is_ascii_punctuation(),
            "print" => |c| (0x20..=0x7e).contains(&c),
            "graph" => |c| (0x21..=0x7e).contains(&c),
            "cntrl" => |c| c < 0x20 || c == 0x7f,
            "xdigit" => |c| c.is_ascii_hexdigit(),
            _ => {
                return err(format!(
                    "'[:{}:]' is not a POSIX character class",
                    name
                ))
            }
        };
        for b in 0u8..=255 {
            if f(b) {
                set.add(b);
            }
        }
        self.i = j + 2;
        Ok(())
    }
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — delta (4): the acceptance oracle is a
// differential run against the shell's own awk, because a hand-written engine is the one
// component whose author cannot write both sides of a unit test and learn anything
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::process::Command;

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the constructs this tree's live
    // vocabularies never write are exactly the branches no fixture pair reaches, so the
    // generator covers them rather than the five patterns the repo happens to configure
    const FRAGMENTS: &[&str] = &[
        "a", "ab", "a*", "a+", "a?", "a.c", ".*", "(a|b)", "(ab|a)", "(ab|a)+", "(a|b)*c",
        "((a|b)c){1,2}", "x{2}", "x{2,}", "x{1,3}", "[a-c]", "[^a-c]", "[abc]+", "[]a]", "[a-]",
        "[-a]", "[[:alpha:]]+", "[[:digit:]]{2,4}", "[[:space:]]+", "[[:upper:]][[:lower:]]*",
        "[[:punct:]]", "[^[:space:]]{3}", "^ab", "ab$", "^(ab|a)", "(a$|b)", "(^a|b$)", "^$",
        "#{2,6}[[:space:]]+", "was (retired|removed|renamed|replaced)", "npx( -y)? checkwright",
        "[Rr]elease tarball", "\\.", "\\(a\\)", "a\\*b", "(|a)b", "(a|)+b", "[0-9]{1,}",
        "(ab)*c", "x[a-c]?y", ".{2,3}", "[^a]*", "a|b|cd", "((a))", "[[:alnum:]_-]+",
    ];

    const SUBJECTS: &[&str] = &[
        "", "a", "b", "ab", "abc", "aab", "aabb", "ac", "abcabc", "xx", "xxx", "xxxx", "x",
        "## Heading", "###### deep  head", "# one", "  ## indented", "was retired",
        "was removed later", "was renamed from", "] bracket", "[bracket]", "a-b", "-a", "(a)",
        "3.14", "  spaced", "\tTabbed", "AbC123", "npx -y checkwright", "npx checkwright",
        "Release tarball", "release tarball", "a*b", "cab", "cb", "c", "abab", "ABC", "abc def",
        "_x-9", "!?", "café", "naïve text", "0", "42", "12345",
    ];

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — one awk invocation per pattern over
    // every subject at once, and `ENVIRON` rather than `-v` so awk's own string-escape pass
    // never rewrites the pattern before its regex compiler sees it
    fn awk_verdicts(pattern: &str, subjects_file: &std::path::Path) -> Vec<(bool, usize, i64)> {
        let out = Command::new("awk")
            .arg(
                r#"BEGIN { p = ENVIRON["CW_ERE_PATTERN"] }
                   { if ($0 ~ p) m = 1; else m = 0
                     if (match($0, p)) print m, RSTART, RLENGTH; else print m, 0, -1 }"#,
            )
            .arg(subjects_file)
            .env("CW_ERE_PATTERN", pattern)
            .env("LC_ALL", "C")
            .output()
            .expect("cannot run awk — the differential oracle is not optional");
        assert!(
            out.status.success(),
            "awk rejected the pattern {:?}: {}",
            pattern,
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|l| {
                let f: Vec<&str> = l.split_whitespace().collect();
                (
                    f[0] == "1",
                    f[1].parse::<usize>().expect("RSTART is not a number"),
                    f[2].parse::<i64>().expect("RLENGTH is not a number"),
                )
            })
            .collect()
    }

    #[test]
    fn the_engine_agrees_with_awk_on_a_generated_cross_product() {
        let dir = std::env::temp_dir().join(format!("cw-ere-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot create the differential corpus dir");
        let corpus = dir.join("subjects");
        {
            let mut f = std::fs::File::create(&corpus).expect("cannot write the corpus");
            for s in SUBJECTS {
                writeln!(f, "{}", s).expect("cannot write a subject");
            }
        }

        let mut patterns: Vec<String> = FRAGMENTS.iter().map(|s| (*s).to_string()).collect();
        // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — nested alternation under a
        // quantifier and an anchor inside a group are the two shapes a leftmost-first engine
        // passes every boolean test on and still spans wrongly
        for a in ["a", "(a|ab)", "[a-c]"] {
            for b in ["b*", "(b|bc)+", "[[:alpha:]]{1,2}"] {
                patterns.push(format!("{}{}", a, b));
                patterns.push(format!("^({}|{})$", a, b));
            }
        }

        let mut compared = 0usize;
        for p in &patterns {
            let ere = Ere::compile(p).unwrap_or_else(|e| panic!("{:?} failed to compile: {}", p, e));
            let verdicts = awk_verdicts(p, &corpus);
            assert_eq!(
                verdicts.len(),
                SUBJECTS.len(),
                "awk reported {} verdicts for {} subjects",
                verdicts.len(),
                SUBJECTS.len()
            );
            for (s, (m, rstart, rlength)) in SUBJECTS.iter().zip(verdicts) {
                assert_eq!(
                    ere.is_match(s),
                    m,
                    "is_match disagreed with awk: pattern {:?} subject {:?}",
                    p,
                    s
                );
                let mine = ere.find(s).map(|(a, b)| (a + 1, (b - a) as i64));
                let theirs = if rlength < 0 { None } else { Some((rstart, rlength)) };
                assert_eq!(
                    mine, theirs,
                    "find disagreed with awk's RSTART/RLENGTH: pattern {:?} subject {:?}",
                    p, s
                );
                compared += 1;
            }
        }
        std::fs::remove_dir_all(&dir).ok();
        assert!(
            compared > 1000,
            "only {} comparisons ran — the cross product collapsed",
            compared
        );
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — prefixes and suffixes that compete with
    // the group for the same bytes are the cases the subpattern rule exists to arbitrate
    const CAPTURE_PREFIXES: &[&str] = &[
        "", "a*", "^", "^a*", "[ab]*", "a", "ab?", "x?", ".*", "[[:alpha:]]*", "^x", "a?",
        "a{0,2}", "[^c]*", "[ab]{0,3}", "b*a?",
    ];
    const CAPTURE_GROUPS: &[&str] = &[
        "(a*)", "(a+)", "()", "(bcd?)", "(ab?)", "([0-9]+)", "([a-c]*)", "(.*)", "(a?b?)",
        "(^a)", "(a$)", "(b*)", "(a*b*)", "([ab]*)", "(a{1,2})", "(b?)", "(.)", "([^b]*)",
        "(a*$)", "(ab*)", "(.*b)", "([[:digit:]]*)", "(b?c?)",
    ];
    const CAPTURE_SUFFIXES: &[&str] = &[
        "", "a*", "$", "a*$", "b", "c", "bcd", "[b-d]*", "d*$", ".*", "a?b?", "b?c?d?",
    ];
    const CAPTURE_SUBJECTS: &[&str] = &[
        "", "a", "aa", "aaa", "ab", "abc", "abcd", "abbcd", "aabb", "xab", "b", "ba", "123",
        "pid 42", "abab", "cab", "bcd", "xaaab", "held by pid 77",
    ];

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — one bash over the whole cross product,
    // every pattern and subject crossing in argv, never interpolated into the script
    const BASH_CAPTURE: &str = r#"n=$1; shift
subjects=("${@:1:n}"); shift "$n"
for p in "$@"; do
  for s in "${subjects[@]}"; do
    [[ $s =~ $p ]]; rc=$?
    if [ "$rc" -eq 0 ]; then printf 'm\t%s\t%s\n' "${BASH_REMATCH[0]}" "${BASH_REMATCH[1]}"
    else printf 'rc%s\t\t\n' "$rc"; fi
  done
done"#;

    #[test]
    fn the_one_group_capture_agrees_with_bash_rematch_on_a_generated_cross_product() {
        let mut patterns: Vec<String> = Vec::new();
        for p in CAPTURE_PREFIXES {
            for g in CAPTURE_GROUPS {
                for x in CAPTURE_SUFFIXES {
                    patterns.push(format!("{}{}{}", p, g, x));
                }
            }
        }
        let out = Command::new("bash")
            .arg("-c")
            .arg(BASH_CAPTURE)
            .arg("_")
            .arg(CAPTURE_SUBJECTS.len().to_string())
            .args(CAPTURE_SUBJECTS)
            .args(&patterns)
            .env("LC_ALL", "C")
            .output()
            .expect("cannot run bash — the differential oracle is not optional");
        assert!(out.status.success(), "bash failed: {}", String::from_utf8_lossy(&out.stderr));
        let stdout = String::from_utf8_lossy(&out.stdout);
        let mut lines = stdout.split('\n');
        let mut compared = 0usize;
        let mut divergences: Vec<String> = Vec::new();
        for p in &patterns {
            let cap = EreCapture::compile(p).unwrap_or_else(|e| panic!("{:?} failed to compile: {}", p, e));
            for s in CAPTURE_SUBJECTS {
                let line = lines.next().expect("bash reported fewer verdicts than cases");
                let f: Vec<&str> = line.split('\t').collect();
                assert_eq!(f.len(), 3, "malformed bash verdict {:?}", line);
                let theirs = match f[0] {
                    "m" => Some((f[1].to_string(), f[2].to_string())),
                    "rc1" => None,
                    other => panic!("bash's regcomp refused {:?} ({})", p, other),
                };
                let mine = cap.capture(s).map(|(i, j)| {
                    let (ws, we) = cap.whole.find(s).expect("a capture implies a whole match");
                    (s[ws..we].to_string(), s[i..j].to_string())
                });
                if mine != theirs {
                    divergences.push(format!("{:?} on {:?}: engine {:?}, bash {:?}", p, s, mine, theirs));
                }
                compared += 1;
            }
        }
        assert!(
            divergences.is_empty(),
            "{} capture divergence(s) from BASH_REMATCH (whole, group):\n{}",
            divergences.len(),
            divergences.join("\n")
        );
        assert!(
            compared > 10000,
            "only {} comparisons ran — the cross product collapsed",
            compared
        );
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — each shape outside the admitted one is
    // refused by the rule it broke
    #[test]
    fn a_capture_shape_outside_the_admitted_one_is_refused_by_name() {
        for (p, why) in [
            ("a \\([0-9]+\\)", "no capture group"),
            ("[(]a", "no capture group"),
            ("(a)(b)", "second capture group"),
            ("((a))", "second capture group"),
            ("(a)*", "quantified"),
            ("x(a){2}", "quantified"),
            ("(a)|b", "alternation"),
            ("b|x(a)", "inside an alternation"),
            ("(a|ab)b*", "body contains an alternation"),
            ("x(a$|b)", "body contains an alternation"),
            ("(c(a|b)*)", "second capture group"),
            ("([ab]x|c)", "body contains an alternation"),
        ] {
            let e = match EreCapture::compile(p) {
                Ok(_) => panic!("{:?} compiled", p),
                Err(e) => e,
            };
            assert!(e.to_string().contains(why), "{:?} refused as {:?}, not {:?}", p, e.to_string(), why);
        }
        assert!(EreCapture::compile("([\"").is_err());
        assert!(EreCapture::compile("(\\w+)").is_err());
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — delta (3): the refusals awk would
    // silently accept, which is why they cannot be asserted differentially
    #[test]
    fn a_gnu_extension_is_refused_by_name_rather_than_read_as_a_literal() {
        for p in ["\\ya", "a\\<b", "\\w+", "\\s", "\\B", "(a)\\1"] {
            let e = match Ere::compile(p) {
                Ok(_) => panic!("{:?} compiled", p),
                Err(e) => e,
            };
            assert!(
                e.to_string().contains("does not implement"),
                "{:?} was refused, but not as an unimplemented construct: {}",
                p,
                e
            );
        }
        for p in ["[a", "a{2,1}", "(a", "a)", "[[:nope:]]", "\\", "*a"] {
            assert!(Ere::compile(p).is_err(), "{:?} compiled", p);
        }
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — delta (2): the one assertion the
    // differential corpus above would also carry, stated alone because it is the whole
    // reason the engine is leftmost-longest rather than leftmost-first
    #[test]
    fn alternation_reports_the_longest_span_not_the_first_branch() {
        let e = Ere::compile("(deprecated|deprecated-since)").expect("compiles");
        assert_eq!(e.find("x deprecated-since y"), Some((2, 18)));
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the start-offset search keeps `^` at the
    // subject's position 0, returns a span beginning at the offset, and admits the empty match at
    // the subject's end
    #[test]
    fn a_start_offset_search_keeps_the_anchor_at_the_subject_start() {
        let bol = Ere::compile("^a").expect("compiles");
        assert_eq!(bol.find_from("aa", 1), None);
        let star = Ere::compile("a*").expect("compiles");
        assert_eq!(star.find_from("baa", 2), Some((2, 3)));
        let empty = Ere::compile("x*").expect("compiles");
        assert_eq!(empty.find_from("abc", 3), Some((3, 3)));
    }
}
