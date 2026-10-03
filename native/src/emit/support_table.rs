// spec: companion/SPEC.md §The support table — one row per check the fixture tree proves or
// native.list rosters, one column per toolkits.list line; one function the gate calls in process
use crate::fresh;
use std::path::Path;

pub const COMPANION_KNOB: &str = "GATE_LOCAL_COMPANION_DIR";
pub const PAGE_KNOB: &str = "GATE_LOCAL_SUPPORT_TABLE_PAGE";
const BEGIN: &str = "<!-- support-table:begin -->";
const END: &str = "<!-- support-table:end -->";
// spec: companion/SPEC.md §The support table — the directory is the tier, in the order a cell
// resolves it
const TIERS: &[(&str, &str)] = &[("defects", "prose"), ("full", "full"), ("overlap", "opt-in")];
const ABSENT: &str = "—";

struct Toolkit {
    key: String,
    name: String,
}

fn read(path: &str) -> Result<String, String> {
    fresh::read_captured(path).map_err(|e| format!("{}: {}", path, e))
}

fn toolkits(dir: &str) -> Result<Vec<Toolkit>, String> {
    let path = Path::new(dir).join("toolkits.list").display().to_string();
    let text = read(&path)?;
    let mut out = Vec::new();
    for (n, line) in fresh::file_lines(&text).into_iter().enumerate() {
        if !fresh::live_line(line) {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() < 4 {
            return Err(format!("{}:{}: no display name after the version", path, n + 1));
        }
        out.push(Toolkit {
            key: words[0].to_string(),
            name: words[3..].join(" "),
        });
    }
    Ok(out)
}

fn natives(dir: &str, kits: &[Toolkit]) -> Result<Vec<(String, String)>, String> {
    let path = Path::new(dir).join("native.list").display().to_string();
    let text = read(&path)?;
    let mut out = Vec::new();
    for (n, line) in fresh::file_lines(&text).into_iter().enumerate() {
        if !fresh::live_line(line) {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        if !kits.iter().any(|k| k.key == words[0]) {
            return Err(format!("{}:{}: toolkit '{}' has no toolkits.list line", path, n + 1, words[0]));
        }
        if words.len() < 2 {
            return Err(format!("{}:{}: '{}' carries no arguments", path, n + 1, words[0]));
        }
        out.push((words[0].to_string(), words[1..].join(" ")));
    }
    Ok(out)
}

fn subdirs(p: &Path) -> Result<Vec<String>, String> {
    if !p.is_dir() {
        return Ok(Vec::new());
    }
    Ok(crate::walk::list_dir(p)?
        .into_iter()
        .filter(|(_, is_dir)| *is_dir)
        .map(|(name, _)| name)
        .collect())
}

// spec: companion/SPEC.md §The support table — the table between the markers' blank lines
pub fn render(dir: &str) -> Result<String, String> {
    let kits = toolkits(dir)?;
    let natives = natives(dir, &kits)?;
    let fixtures = Path::new(dir).join("fixtures");
    for tk in subdirs(&fixtures)? {
        if !kits.iter().any(|k| k.key == tk) {
            return Err(format!(
                "{}: toolkit '{}' has no toolkits.list line",
                fixtures.join(&tk).display(),
                tk
            ));
        }
    }
    let mut proven: Vec<(String, usize, &str)> = Vec::new();
    for (i, k) in kits.iter().enumerate() {
        for (tier, _) in TIERS {
            for gate in subdirs(&fixtures.join(&k.key).join(tier))? {
                proven.push((gate, i, tier));
            }
        }
    }
    let mut gates: Vec<&str> = proven.iter().map(|(g, _, _)| g.as_str()).collect();
    gates.sort();
    gates.dedup();

    let mut out: Vec<String> = Vec::new();
    let head: Vec<&str> = kits.iter().map(|k| k.name.as_str()).collect();
    out.push(format!("| Check | {} |", head.join(" | ")));
    out.push(format!("|{}", " --- |".repeat(kits.len() + 1)));
    for gate in gates {
        let cells: Vec<&str> = (0..kits.len())
            .map(|i| {
                TIERS
                    .iter()
                    .find(|(tier, _)| proven.iter().any(|(g, k, t)| g == gate && *k == i && t == tier))
                    .map(|(_, mark)| *mark)
                    .unwrap_or(ABSENT)
            })
            .collect();
        out.push(format!("| `{}` | {} |", gate, cells.join(" | ")));
    }
    for (tk, args) in &natives {
        let cells: Vec<&str> = kits
            .iter()
            .map(|k| if &k.key == tk { "toolkit" } else { ABSENT })
            .collect();
        out.push(format!("| `{} {}` | {} |", tk, args, cells.join(" | ")));
    }
    Ok(format!("\n{}\n", out.join("\n")))
}

// spec: companion/SPEC.md §The support table — the block's line span; an absent or repeated
// block is no place to write, so it refuses
pub struct Block {
    pub line: usize,
    start: usize,
    end: usize,
    pub actual: String,
}

pub fn block(page: &str, text: &str) -> Result<Block, String> {
    let lines = fresh::file_lines(text);
    let at = |m: &str| -> Vec<usize> { lines.iter().enumerate().filter(|(_, l)| **l == m).map(|(i, _)| i).collect() };
    let (b, e) = (at(BEGIN), at(END));
    match (b.as_slice(), e.as_slice()) {
        ([b], [e]) if b < e => Ok(Block {
            line: b + 1,
            start: b + 1,
            end: *e,
            actual: lines[b + 1..*e].join("\n"),
        }),
        _ => Err(format!(
            "{}: needs exactly one {} … {} pair, in order ({} begin, {} end)",
            page,
            BEGIN,
            END,
            b.len(),
            e.len()
        )),
    }
}

fn rewrite(page: &str, text: &str, table: &str) -> Result<String, String> {
    let blk = block(page, text)?;
    let lines = fresh::file_lines(text);
    let mut out: Vec<String> = lines[..blk.start].iter().map(|l| l.to_string()).collect();
    out.push(table.to_string());
    out.extend(lines[blk.end..].iter().map(|l| l.to_string()));
    let mut rendered = out.join("\n");
    if text.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}

// spec: companion/SPEC.md §The support table — the bare arm prints the block; `--write` rewrites
// it in place and touches nothing else
pub fn emit(args: &[String]) -> Result<String, String> {
    let table = render(&crate::walk::knob_scalar(COMPANION_KNOB)?)?;
    if args.iter().any(|a| a == "--write") {
        let page = crate::walk::knob_scalar(PAGE_KNOB)?;
        let text = crate::emit::read_text(&page)?;
        let new = rewrite(&page, &text, &table)?;
        if new == text {
            return Ok(format!("support-table: {} already fresh\n", page));
        }
        std::fs::write(&page, new).map_err(|e| format!("cannot write {}: {}", page, e))?;
        return Ok(format!("support-table: rewrote {}\n", page));
    }
    Ok(format!("{}\n", table.trim_matches('\n')))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tree(name: &str) -> String {
        let root = std::env::temp_dir().join(format!("support-table-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&root);
        for d in [
            "fixtures/a/defects/check-x",
            "fixtures/b/defects/check-x",
            "fixtures/b/full/check-y",
            "fixtures/a/overlap/check-z",
            "fixtures/a/layout",
        ] {
            fs::create_dir_all(root.join(d)).unwrap();
        }
        fs::write(root.join("toolkits.list"), "# c\na pa 1 Kit A\nb pb 2 B\n").unwrap();
        fs::write(root.join("native.list"), "# c\nb check --all\n").unwrap();
        root.display().to_string()
    }

    #[test]
    fn each_cell_reads_its_tier_and_the_toolkit_rows_follow() {
        let dir = tree("render");
        assert_eq!(
            render(&dir).unwrap(),
            "\n| Check | Kit A | B |\n| --- | --- | --- |\n| `check-x` | prose | prose |\n\
             | `check-y` | — | full |\n| `check-z` | opt-in | — |\n| `b check --all` | — | toolkit |\n"
        );
    }

    #[test]
    fn a_malformed_roster_or_an_unlisted_fixture_toolkit_refuses() {
        let dir = tree("refuse");
        fs::write(Path::new(&dir).join("native.list"), "c check\n").unwrap();
        assert!(render(&dir).is_err());
        fs::write(Path::new(&dir).join("native.list"), "b\n").unwrap();
        assert!(render(&dir).is_err());
        fs::write(Path::new(&dir).join("native.list"), "b check\n").unwrap();
        fs::write(Path::new(&dir).join("toolkits.list"), "a pa 1 A\nb pb 2\n").unwrap();
        assert!(render(&dir).is_err());
        fs::write(Path::new(&dir).join("toolkits.list"), "b pb 2 B\n").unwrap();
        assert!(render(&dir).is_err());
    }

    #[test]
    fn a_write_touches_the_block_alone_and_an_absent_or_doubled_block_refuses() {
        let md = format!("# T\n\n{}\nold\n{}\n\nrest\n", BEGIN, END);
        let got = rewrite("p.md", &md, "\n| t |\n").unwrap();
        assert_eq!(got, format!("# T\n\n{}\n\n| t |\n\n{}\n\nrest\n", BEGIN, END));
        assert_eq!(rewrite("p.md", &got, "\n| t |\n").unwrap(), got);
        assert!(block("p.md", "none\n").is_err());
        assert!(block("p.md", &format!("{0}\n{1}\n{0}\n{1}\n", BEGIN, END)).is_err());
    }
}
