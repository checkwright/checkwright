// spec: installer/SPEC.md §The install size — what an install places, derived from the index the
// packer packs; one function the gate calls in process
use crate::emit::pack_installer;
use crate::emit::support_table as st;
use crate::exit::Fault;
use crate::installer::profile;
use crate::proc::{self, Stderr};
use crate::programs;
use crate::walk;

pub const PAGE_KNOB: &str = "GATE_LOCAL_INSTALL_SIZE_PAGE";
pub const INSTALLER_KNOB: &str = "GATE_LOCAL_INSTALLER_DIR";
pub const WITHHOLD_KNOB: &str = "GATE_SDK_PAYLOAD_WITHHOLD";
const BEGIN: &str = "<!-- install-size:begin -->";
const END: &str = "<!-- install-size:end -->";
const GATE_TESTS: &str = "gate-tests";
const GITLINK: &str = "160000";

#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Tally {
    files: u64,
    tests: u64,
    bytes: u64,
}

impl Tally {
    fn add(&mut self, o: &Tally) {
        self.files += o.files;
        self.tests += o.tests;
        self.bytes += o.bytes;
    }
}

struct Entry {
    path: String,
    bytes: u64,
}

// spec: installer/SPEC.md §The install size — the packer's own pathspec over the index, each file
// sized by its staged blob, so the figure is the one the next commit carries
fn entries(top: Option<&str>, kit: &str, withhold: &[String]) -> Result<Vec<Entry>, String> {
    let spec = pack_installer::pathspec(kit, withhold);
    let mut ls: Vec<&str> = vec!["ls-files", "-s", "-z", "--"];
    ls.extend(spec.iter().map(String::as_str));
    let listing = crate::listing::names(top, &ls).map_err(|e| e.text_or(|_| format!("git {} failed", ls.join(" "))))?;
    let mut staged: Vec<(String, String)> = Vec::new();
    for line in &listing {
        let (meta, path) = line.split_once('\t').ok_or_else(|| format!("git ls-files -s: unreadable entry: {}", line))?;
        let mut f = meta.split(' ');
        let (mode, sha) = (f.next().unwrap_or(""), f.next().unwrap_or(""));
        if mode == GITLINK || sha.is_empty() {
            continue;
        }
        staged.push((sha.to_string(), path.to_string()));
    }
    if staged.is_empty() {
        return Ok(Vec::new());
    }
    let input: String = staged.iter().map(|(sha, _)| format!("{}\n", sha)).collect();
    let mut check: Vec<&str> = Vec::new();
    if let Some(t) = top {
        check.extend(["-C", t]);
    }
    check.extend(["cat-file", "--batch-check=%(objectsize)"]);
    let done = proc::run_streamed(&programs::GIT, &check, input.as_bytes(), Stderr::Discard)?;
    if done.code() != 0 {
        return Err(format!("git cat-file --batch-check exited {} sizing {}", done.code(), kit));
    }
    let sizes: Vec<&str> = std::str::from_utf8(done.stdout())
        .map_err(|_| "git cat-file --batch-check printed a non-UTF-8 size".to_string())?
        .lines()
        .collect();
    if sizes.len() != staged.len() {
        return Err(format!("git cat-file --batch-check sized {} of {} blob(s) under {}", sizes.len(), staged.len(), kit));
    }
    staged
        .into_iter()
        .zip(sizes)
        .map(|((sha, path), size)| match size.trim().parse::<u64>() {
            Ok(bytes) => Ok(Entry { path, bytes }),
            Err(_) => Err(format!("{}: blob {} has no readable size ({})", path, sha, size.trim())),
        })
        .collect()
}

fn tally(kit: &str, entries: &[Entry]) -> Tally {
    let tests = format!("{}/{}/", kit.trim_end_matches('/'), GATE_TESTS);
    let mut t = Tally::default();
    for e in entries {
        t.files += 1;
        t.bytes += e.bytes;
        if e.path.starts_with(&tests) {
            t.tests += 1;
        }
    }
    t
}

// spec: installer/SPEC.md §The install size — two significant figures, rounded half up
fn sig2(n: u64) -> u64 {
    let mut p = 1u64;
    while n / p >= 100 {
        p *= 10;
    }
    if p == 1 {
        return n;
    }
    (n + p / 2) / p * p
}

fn grouped(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn count(n: u64) -> String {
    grouped(sig2(n))
}

// spec: installer/SPEC.md §The install size — decimal units, `kB` below 1 MB and `MB` with one
// decimal place from it, rounded before the unit is chosen
pub fn size(bytes: u64) -> String {
    let r = sig2(bytes);
    if r >= 1_000_000 {
        format!("{:.1} MB", r as f64 / 1_000_000.0)
    } else {
        format!("{} kB", r as f64 / 1_000.0)
    }
}

fn leaf(kit: &str) -> String {
    let k = kit.trim_end_matches('/');
    k.rsplit('/').next().unwrap_or(k).to_string()
}

fn table(profiles: &[(String, Vec<String>)], kits: &[(String, Tally)]) -> String {
    let mut out = vec![
        "| Profile | Kits | Files | Of which gate-tests | Size |".to_string(),
        "| --- | --- | --- | --- | --- |".to_string(),
    ];
    for (name, members) in profiles {
        let mut t = Tally::default();
        for (_, k) in kits.iter().filter(|(n, _)| members.contains(n)) {
            t.add(k);
        }
        out.push(format!(
            "| `{}` | {} | {} | {} | {} |",
            name,
            members.len(),
            count(t.files),
            count(t.tests),
            size(t.bytes)
        ));
    }
    out.push(String::new());
    out.push("| Kit | Files | Of which gate-tests | Size |".to_string());
    out.push("| --- | --- | --- | --- |".to_string());
    let mut sorted: Vec<&(String, Tally)> = kits.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, t) in sorted {
        out.push(format!("| {} | {} | {} | {} |", name, count(t.files), count(t.tests), size(t.bytes)));
    }
    format!("\n{}\n", out.join("\n"))
}

// spec: installer/SPEC.md §The install size — the block between the markers' blank lines
pub fn render() -> Result<String, String> {
    render_fault().map_err(|f| f.message)
}

fn render_fault() -> Result<String, Fault> {
    walk::toplevel_fault()?;
    let withhold: Vec<String> = walk::knob_scalar(WITHHOLD_KNOB).map_err(Fault::refused)?.split_whitespace().map(String::from).collect();
    let installer = walk::knob_scalar(INSTALLER_KNOB).map_err(Fault::refused)?;
    let roster = format!("{}/profiles.list", installer.trim_end_matches('/'));
    let text = crate::emit::read_text(&roster).map_err(|e| Fault::reading(&roster, e))?;
    let roots: Vec<String> = walk::kit_roots().map_err(Fault::refused)?
        .iter()
        .map(|k| k.trim_end_matches('/').to_string())
        .filter(|k| !k.is_empty() && std::path::Path::new(k).is_dir())
        .collect();
    if roots.is_empty() {
        return Err(Fault::refused("no kit roots enumerated — there is no payload to size"));
    }
    let mut kits: Vec<(String, Tally)> = Vec::new();
    for root in &roots {
        kits.push((leaf(root), tally(root, &entries(None, root, &withhold).map_err(Fault::tool)?)));
    }
    let payload: Vec<String> = kits.iter().map(|(n, _)| n.clone()).collect();
    let profiles: Vec<(String, Vec<String>)> = profile::names_in(&text)
        .into_iter()
        .map(|p| {
            let members = profile::kits_among(&text, &p, payload.clone());
            (p, members)
        })
        .collect();
    Ok(table(&profiles, &kits))
}

pub fn block(page: &str, text: &str) -> Result<st::Block, String> {
    st::marked_block(page, text, BEGIN, END)
}

// spec: installer/SPEC.md §The install size — the bare arm prints the block; `--write` rewrites it
// in place and touches nothing else
pub fn emit(args: &[String]) -> Result<String, Fault> {
    let table = render_fault()?;
    if args.iter().any(|a| a == "--write") {
        let page = walk::knob_scalar(PAGE_KNOB).map_err(Fault::refused)?;
        return st::write_marked("install-size", &page, BEGIN, END, &table);
    }
    Ok(format!("{}\n", table.trim_matches('\n')))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_round_to_two_significant_figures_and_sizes_take_their_unit_after_rounding() {
        assert_eq!([sig2(0), sig2(7), sig2(99), sig2(104), sig2(1_449), sig2(1_450), sig2(1_612)], [0, 7, 99, 100, 1_400, 1_500, 1_600]);
        assert_eq!(count(1_612), "1,600");
        assert_eq!(count(1_234_567), "1,200,000");
        assert_eq!(size(0), "0 kB");
        assert_eq!(size(950), "0.95 kB");
        assert_eq!(size(45_300), "45 kB");
        assert_eq!(size(999_600), "1.0 MB");
        assert_eq!(size(1_812_345), "1.8 MB");
        assert_eq!(size(12_600_000), "13.0 MB");
    }

    #[test]
    fn a_profile_row_sums_its_kits_and_the_kit_table_runs_in_name_order() {
        let t = |files, tests, bytes| Tally { files, tests, bytes };
        let kits = vec![("b-kit".to_string(), t(10, 4, 2_000)), ("a-kit".to_string(), t(5, 1, 500))];
        let profiles = vec![
            ("small".to_string(), vec!["a-kit".to_string()]),
            ("full".to_string(), vec!["b-kit".to_string(), "a-kit".to_string()]),
        ];
        let got = table(&profiles, &kits);
        let lines: Vec<&str> = got.trim().lines().collect();
        assert_eq!(lines[2], "| `small` | 1 | 5 | 1 | 0.5 kB |");
        assert_eq!(lines[3], "| `full` | 2 | 15 | 5 | 2.5 kB |");
        assert_eq!(lines[7], "| a-kit | 5 | 1 | 0.5 kB |");
        assert_eq!(lines[8], "| b-kit | 10 | 4 | 2 kB |");
        assert!(got.starts_with('\n') && got.ends_with('\n'));
    }

    // spec: installer/SPEC.md §The install size — a withheld file is outside the figure, an
    // unstaged edit is not in it, and a file under the kit's gate-tests is counted in both columns
    #[test]
    fn the_index_decides_what_is_counted() {
        let root = proc::temp_base().unwrap().join(format!("install-size-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("k/gate-tests/check-x/good")).unwrap();
        std::fs::create_dir_all(root.join("k/smoke")).unwrap();
        std::fs::write(root.join("k/README.md"), "12345").unwrap();
        std::fs::write(root.join("k/SPEC.md"), "withheld").unwrap();
        std::fs::write(root.join("k/smoke/install.sh"), "withheld too").unwrap();
        std::fs::write(root.join("k/gate-tests/check-x/good/expect.txt"), "abc").unwrap();
        let top = root.display().to_string();
        let git = |args: &[&str]| {
            let mut a = vec!["-C", top.as_str()];
            a.extend(args);
            let done = proc::run(&programs::GIT, &a).expect("git ran");
            assert!(done.stdout().is_some(), "git {:?} failed", args);
        };
        git(&["init", "-q"]);
        git(&["add", "-A"]);
        std::fs::write(root.join("k/README.md"), "an unstaged edit, longer than the staged blob").unwrap();
        let got = entries(Some(&top), "k", &["SPEC.md".to_string(), "smoke".to_string()]).expect("sized");
        assert_eq!(tally("k", &got), Tally { files: 2, tests: 1, bytes: 8 });
        let _ = std::fs::remove_dir_all(&root);
    }
}
