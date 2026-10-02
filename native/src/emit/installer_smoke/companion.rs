// spec: installer/SPEC.md §The consumer smoke — the companion arm: each payload recipe applied by
// its toolkit's documented install line to its fixture tree, the full and complement lines likewise,
// and the OpenSpec lifecycle layer's audit leg
use super::consumer::{
    consumer, copy_tree, failed, find_line, git, out, porcelain, seam_of, sorted, tree, Lock,
};
use super::{fail, merged_in, say, Outcome, Run, Step};
use crate::{programs, walk};
use std::collections::BTreeMap;
use std::path::Path;

// spec: companion/SPEC.md §The tested claim — the claimed gates, the claim's oracle, so a fixture
// tree missing a defect directory reds rather than shrinking the claim
const CLAIMED: &[&str] = &[
    "check-md-refs",
    "check-spec-pointer",
    "check-spec-fence-balance",
    "check-docs-cmd",
    "check-task-path-claim",
];
const CLAIMED_SPECKIT: &[&str] = &["check-task-label-resolution"];

// spec: installer/SPEC.md §The consumer smoke — the toolkits this table carries an arm header for,
// held against the recipe directories so a recipe with no header reds
const ARMED: &[&str] = &["openspec", "speckit"];

fn companion(state: &Run) -> String {
    format!("{}/companion", state.root)
}

// spec: installer/SPEC.md §The consumer smoke — a marked block's lines, fences and blank lines
// dropped
fn block(file: &str, marker: &str) -> Vec<String> {
    let body = std::fs::read_to_string(file).unwrap_or_default();
    let (begin, end) = (format!("<!-- {}:begin -->", marker), format!("<!-- {}:end -->", marker));
    let mut on = false;
    let mut kept = Vec::new();
    for line in body.lines() {
        if line == end {
            on = false;
        }
        if on && !line.trim().is_empty() && !line.starts_with("```") {
            kept.push(line.to_string());
        }
        if line == begin {
            on = true;
        }
    }
    kept
}

// spec: installer/SPEC.md §The consumer smoke — a block's one documented line, its words from `init`
// to its end, refused where the block is absent, empty, longer than a line, runs no init or lacks
// the words the line must carry
fn line_words(state: &Run, file: &str, marker: &str, must: &str) -> Result<Vec<String>, String> {
    let where_ = walk::rel_under(&state.root, file).unwrap_or(file);
    let lines = block(file, marker);
    let [line] = lines.as_slice() else {
        return Err(if lines.is_empty() {
            format!("{} carries no {} block, or an empty one, so there is no documented line to run", where_, marker)
        } else {
            format!("the {} block in {} holds more than one line", marker, where_)
        });
    };
    let words: Vec<String> = line.split_whitespace().skip_while(|w| *w != "init").map(str::to_string).collect();
    if words.is_empty() {
        return Err(format!("the {} line in {} runs no init: {}", marker, where_, line));
    }
    if !format!(" {} ", words.join(" ")).contains(&format!(" {} ", must)) {
        return Err(format!("the {} line in {} carries no {}: {}", marker, where_, must, line));
    }
    Ok(words)
}

fn words_or(state: &Run, file: &str, marker: &str, must: &str, why: &str) -> Result<Vec<String>, Outcome> {
    line_words(state, file, marker, must).map_err(|cause| {
        eprintln!("companion arm: {}", cause);
        fail(why)
    })
}

fn toolkits(state: &Run) -> Vec<String> {
    walk::list_dir(Path::new(&companion(state)))
        .unwrap_or_default()
        .into_iter()
        .filter(|(n, dir)| *dir && Path::new(&format!("{}/{}/recipe", companion(state), n)).is_dir())
        .map(|(n, _)| n)
        .collect()
}

// spec: companion/SPEC.md §The component — the exclusion record, refused before any leg on a row no
// complement leg could honour
fn exclusions(state: &Run) -> Result<BTreeMap<String, Vec<String>>, Outcome> {
    let root = companion(state);
    let list = format!("{}/exclusions.list", root);
    let Ok(body) = std::fs::read_to_string(&list) else {
        return Err(fail(
            "companion arm: companion/exclusions.list does not exist, so no complement line has a record to be held to",
        ));
    };
    let known: Vec<String> = std::fs::read_to_string(format!("{}/toolkits.list", root))
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .collect();
    let mut excluded: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut pairs: Vec<(String, String)> = Vec::new();
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let mut f = t.split_whitespace();
        let (tk, kit, surface) = (f.next().unwrap_or(""), f.next().unwrap_or(""), f.next().unwrap_or(""));
        if !known.iter().any(|k| k == tk) {
            return Err(fail(format!(
                "companion arm: companion/exclusions.list names {}, which has no companion/toolkits.list line: {}",
                tk, line
            )));
        }
        if surface.is_empty() {
            return Err(fail(format!(
                "companion arm: companion/exclusions.list's row '{}' records no toolkit surface the left-out kit defers to",
                line
            )));
        }
        let pair = (tk.to_string(), kit.to_string());
        if pairs.contains(&pair) {
            return Err(fail(format!("companion arm: companion/exclusions.list carries '{} {}' twice", tk, kit)));
        }
        pairs.push(pair);
        if !state.payload_kits.iter().any(|k| k == kit) {
            return Err(fail(format!(
                "companion arm: companion/exclusions.list leaves out {} for {}, and the packed payload carries no payload/{}/",
                kit, tk, kit
            )));
        }
        excluded.entry(tk.to_string()).or_default().push(kit.to_string());
    }
    for tk in toolkits(state) {
        let Some(kits) = excluded.get_mut(&tk) else {
            return Err(fail(format!(
                "companion arm: companion/{}/recipe/ exists and companion/exclusions.list has no {} row, so its complement line would leave out nothing and be the full line",
                tk, tk
            )));
        };
        *kits = sorted(std::mem::take(kits));
    }
    Ok(excluded)
}

fn gates_bin(c: &str) -> Result<programs::Program, Outcome> {
    let bin = seam_of(c).ok_or_else(|| fail(format!("{} names no gate binary in its seam", c)))?;
    Ok(programs::CHECKWRIGHT_GATES.at(format!("{}/{}", c, bin)))
}

// spec: installer/SPEC.md §The consumer smoke — a consumer holding the toolkit's fixture layout,
// installed by the line in one commit with the line's recipes recorded, hooks on
fn install(state: &Run, tk: &str, label: &str, words: &[String]) -> Result<String, Outcome> {
    let why = |cause: String| fail(format!("companion arm, {}: could not install {} on the fixture tree: {}", label, words.join(" "), cause));
    let c = consumer(state, &format!("companion-{}", label))?;
    let layout = format!("{}/fixtures/{}/layout", companion(state), tk);
    copy_tree(Path::new(&layout), Path::new(&c)).map_err(why)?;
    super::profiles::commit_all(&c, "the toolkit's tree", "could not commit the toolkit's tree")?;
    let before = super::consumer::commits(&c)?;
    let args: Vec<&str> = words.iter().map(String::as_str).collect();
    let m = state.verb(&c, &args)?;
    if !m.succeeded() {
        super::show(&out(&m));
        return Err(why(format!("{} exited non-zero on the fixture tree", words.join(" "))));
    }
    if !super::profiles::commits_since(&c, before)? {
        return Err(why(format!("{} did not make exactly one commit", words.join(" "))));
    }
    let dirty = porcelain(&c)?;
    if !dirty.is_empty() {
        super::show(&dirty);
        return Err(why(format!("{} left the worktree dirty", words.join(" "))));
    }
    let want: Vec<String> = words.windows(2).filter(|w| w[0] == "--recipe").map(|w| w[1].clone()).collect();
    let got = Lock::of(&c)?.list("recipes");
    if got != want {
        return Err(why(format!(
            "the manifest records recipes [{}] where the line applies [{}]",
            got.join(" "),
            want.join(" ")
        )));
    }
    let m = merged_in(&gates_bin(&c)?, &["--install-hooks"], &[], &c)?;
    if !m.succeeded() {
        return Err(failed(&m, format!("companion arm, {}: --install-hooks failed", label)));
    }
    Ok(c)
}

// spec: installer/SPEC.md §The consumer smoke — the battery green by its summary line, and a bare
// init re-run leaving the tree object unchanged
fn green(state: &Run, label: &str, c: &str) -> Step {
    let m = merged_in(&gates_bin(c)?, &["--run"], &[], c)?;
    let said = out(&m);
    let summary = find_line(&said, |l| {
        l.strip_prefix("All ")
            .and_then(|r| r.strip_suffix(" gates passed."))
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
    });
    let Some(summary) = summary.filter(|_| m.succeeded()) else {
        return Err(failed(&m, format!(
            "companion arm, {}: the battery is not green after the install (exit {})",
            label,
            m.reported_code()
        )));
    };
    let before = tree(c)?;
    let r = state.verb(c, &["init"])?;
    if !r.succeeded() {
        return Err(failed(&r, format!("companion arm, {}: a bare init re-run exited non-zero", label)));
    }
    if tree(c)? != before || !porcelain(c)?.is_empty() {
        return Err(fail(format!(
            "companion arm, {}: a bare init re-run changed the tree, so it did not re-apply the recorded payload recipes",
            label
        )));
    }
    say(&format!(
        "{}: installed in one commit, the battery reads '{}', and a bare init re-run leaves the tree object unchanged",
        label, summary
    ));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — installed on the fixture tree, green, idempotent,
// and each planted defect red by the gate that owns it
fn arm(state: &Run, tk: &str, label: &str, words: &[String]) -> Result<String, Outcome> {
    let root = companion(state);
    if !Path::new(&format!("{}/{}/recipe", root, tk)).is_dir() {
        return Err(fail(format!("companion arm: this arm names {}, and companion/{}/recipe/ does not exist", tk, tk)));
    }
    if !Path::new(&format!("{}/recipes/{}", state.pkg_root, tk)).is_dir() {
        return Err(fail(format!(
            "companion arm, {}: the installed package carries no recipes/{}/, so GATE_SDK_PAYLOAD_RECIPES does not cover companion/{}/recipe/",
            tk, tk, tk
        )));
    }
    let fixtures = format!("{}/fixtures/{}", root, tk);
    if !Path::new(&format!("{}/layout", fixtures)).is_dir() {
        return Err(fail(format!(
            "companion arm: companion/fixtures/{}/layout/ does not exist, so the {} recipe has no tree to govern",
            tk, tk
        )));
    }
    let own: &[&str] = if tk == "speckit" { CLAIMED_SPECKIT } else { &[] };
    for gate in CLAIMED.iter().chain(own) {
        if !Path::new(&format!("{}/defects/{}", fixtures, gate)).is_dir() {
            return Err(fail(format!(
                "companion arm, {}: no defects/{}/ under companion/fixtures/{}/, so the claim that {} catches its class in a {} tree is untested",
                tk, gate, tk, gate, tk
            )));
        }
    }
    let c = install(state, tk, label, words)?;
    green(state, &format!("{}, {}", label, words.join(" ")), &c)?;
    let defects = walk::list_dir(Path::new(&format!("{}/defects", fixtures))).unwrap_or_default();
    for (gate, _) in defects.into_iter().filter(|(_, dir)| *dir) {
        copy_tree(Path::new(&format!("{}/defects/{}", fixtures, gate)), Path::new(&c))
            .map_err(|e| fail(format!("companion arm, {}: could not plant defects/{}/: {}", label, gate, e)))?;
        let m = merged_in(&gates_bin(&c)?, &["--run"], &[], &c)?;
        let lead = format!("  FAIL: {}", gate);
        let reds = find_line(&out(&m), |l| l == lead || l.starts_with(&format!("{} ", lead))).is_some();
        if m.succeeded() || !reds {
            return Err(failed(&m, format!(
                "companion arm, {}: the battery did not red {} on its planted defect (exit {})",
                label,
                gate,
                m.reported_code()
            )));
        }
        let restored = git(&c, &["checkout", "-q", "--", "."])?.succeeded() && git(&c, &["clean", "-fdq"])?.succeeded();
        if !restored {
            return Err(fail(format!("companion arm, {}: could not restore the consumer after defects/{}/", label, gate)));
        }
        if !porcelain(&c)?.is_empty() {
            return Err(fail(format!(
                "companion arm, {}: the consumer is not clean after restoring defects/{}/",
                label, gate
            )));
        }
        say(&format!("{}: defects/{}/ reds {}", label, gate, gate));
    }
    Ok(c)
}

// spec: companion/SPEC.md §The tiers — the manifest's kits hold none of the left-out kits, and its
// recorded selection removes exactly them
fn held_out(label: &str, c: &str, want: &[String], moment: &str) -> Step {
    let lock = Lock::of(c)?;
    let stray: Vec<String> = sorted(lock.list("kits")).into_iter().filter(|k| want.contains(k)).collect();
    if !stray.is_empty() {
        return Err(fail(format!(
            "companion arm, {}: after {} the manifest's kits carry the left-out {}",
            label,
            moment,
            stray.join(" ")
        )));
    }
    let sel = sorted(lock.selection_list("without-kits"));
    if sel != want {
        return Err(fail(format!(
            "companion arm, {}: after {} the recorded selection removes [{}] where companion/exclusions.list leaves out [{}]",
            label,
            moment,
            sel.join(" "),
            want.join(" ")
        )));
    }
    Ok(())
}

// spec: companion/SPEC.md §The tiers — a complement line: its --without-kit set held to the
// toolkit's exclusion rows, installed green and catching each planted defect, and the left-out kits
// held out after the install, a bare re-run and a bare update
fn complement(state: &Run, tk: &str, words: &[String]) -> Step {
    let label = format!("{}-complement", tk);
    let want = exclusions(state)?.remove(tk).unwrap_or_default();
    let mut line_kits: Vec<String> = Vec::new();
    for (i, w) in words.iter().enumerate() {
        if w == "--without-kit" {
            line_kits.extend(words.get(i + 1).cloned());
        } else if let Some(k) = w.strip_prefix("--without-kit=") {
            line_kits.push(k.to_string());
        }
    }
    let line_kits = sorted(line_kits);
    let only_line: Vec<&String> = line_kits.iter().filter(|k| !want.contains(k)).collect();
    if !only_line.is_empty() {
        return Err(fail(format!(
            "companion arm, {}: the line leaves out {}, which companion/exclusions.list does not record for {}",
            label,
            only_line.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" "),
            tk
        )));
    }
    let only_list: Vec<&String> = want.iter().filter(|k| !line_kits.contains(k)).collect();
    if !only_list.is_empty() {
        return Err(fail(format!(
            "companion arm, {}: companion/exclusions.list leaves out {} for {}, and the line does not",
            label,
            only_list.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" "),
            tk
        )));
    }
    say(&format!("{}: the line's --without-kit set is {}'s exclusions.list rows, {}", label, tk, want.join(" ")));
    let c = arm(state, tk, &label, words)?;
    held_out(&label, &c, &want, "the install and a bare re-run")?;
    let before = tree(&c)?;
    let m = state.verb(&c, &["update"])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("companion arm, {}: a bare update exited non-zero", label)));
    }
    if tree(&c)? != before || !porcelain(&c)?.is_empty() {
        return Err(fail(format!(
            "companion arm, {}: a bare update changed the tree, so it did not re-apply the recorded selection",
            label
        )));
    }
    held_out(&label, &c, &want, "a bare update")?;
    say(&format!(
        "{}: the manifest holds {} out, after the install, a bare re-run and a bare update",
        label,
        want.join(" ")
    ));
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — a page's line held to the extension install
// command's words from init
fn same_words(state: &Run, label: &str, marker: &str, command: &[String]) -> Step {
    let page = format!("{}/docs/speckit.md", state.root);
    let what = if marker == "companion-full" { "full" } else { "complement" };
    let words = words_or(
        state,
        &page,
        marker,
        "--profile full",
        &format!("companion arm, {}: the Spec Kit page carries no {} line", label, what),
    )?;
    if words != command {
        return Err(fail(format!(
            "companion arm, {}: the Spec Kit page's {} line and the extension install command's differ from init — page: {}; command: {}",
            label,
            what,
            block(&page, marker).join(" "),
            block(&format!("{}/speckit/commands/install.md", companion(state)), marker).join(" ")
        )));
    }
    say(&format!(
        "{}: the Spec Kit page's {} line carries the extension install command's words from init",
        label, what
    ));
    Ok(())
}

pub(super) fn speckit(state: &mut Run) -> Step {
    exclusions(state)?;
    say("companion/exclusions.list: every row names a pinned toolkit, a payload kit and a surface, and every toolkit with a recipe has one");
    let words = words_or(
        state,
        &format!("{}/speckit/commands/install.md", companion(state)),
        "companion-install",
        "--recipe",
        "companion arm, speckit: the extension install command carries no install line to run",
    )?;
    arm(state, "speckit", "speckit", &words).map(|_| ())
}

pub(super) fn speckit_full(state: &mut Run) -> Step {
    let words = words_or(
        state,
        &format!("{}/speckit/commands/install.md", companion(state)),
        "companion-full",
        "--profile full",
        "companion arm, speckit full: the extension install command carries no full line to run",
    )?;
    same_words(state, "speckit full", "companion-full", &words)?;
    arm(state, "speckit", "speckit-full", &words).map(|_| ())
}

pub(super) fn speckit_complement(state: &mut Run) -> Step {
    let words = words_or(
        state,
        &format!("{}/speckit/commands/install.md", companion(state)),
        "companion-complement",
        "--profile full",
        "companion arm, speckit complement: the extension install command carries no complement line to run",
    )?;
    same_words(state, "speckit complement", "companion-complement", &words)?;
    complement(state, "speckit", &words)
}

fn openspec_page(state: &Run) -> String {
    format!("{}/docs/openspec.md", state.root)
}

pub(super) fn openspec(state: &mut Run) -> Step {
    let words = words_or(
        state,
        &openspec_page(state),
        "companion-install",
        "--recipe",
        "companion arm, openspec: the OpenSpec page carries no install line to run",
    )?;
    arm(state, "openspec", "openspec", &words).map(|_| ())
}

pub(super) fn openspec_complement(state: &mut Run) -> Step {
    let words = words_or(
        state,
        &openspec_page(state),
        "companion-complement",
        "--profile full",
        "companion arm, openspec complement: the OpenSpec page carries no complement line to run",
    )?;
    complement(state, "openspec", &words)
}

// spec: companion/SPEC.md §The lifecycle layer — every change delta under a tree: a `spec.md` below a
// `specs/` directory of openspec/changes
fn deltas(root: &str, archived: bool) -> Result<Vec<String>, Outcome> {
    let changes = format!("{}/openspec/changes", root);
    if !Path::new(&changes).is_dir() {
        return Ok(Vec::new());
    }
    Ok(super::consumer::files_under(Path::new(&changes), "md")?
        .into_iter()
        .map(|f| format!("openspec/changes/{}", f))
        .filter(|f| f.ends_with("/spec.md") && f.contains("/specs/"))
        .filter(|f| archived || !f.starts_with("openspec/changes/archive/"))
        .collect())
}

// spec: companion/SPEC.md §The lifecycle layer — the OpenSpec full line, check-stage-entry red on a
// change touching two capabilities at a build cursor with no align stamp, and clean once it touches
// one
pub(super) fn openspec_lifecycle(state: &mut Run) -> Step {
    let overlay = format!("{}/fixtures/openspec/full/check-stage-entry", companion(state));
    let planted = deltas(&overlay, true)?;
    if planted.is_empty() {
        return Err(fail(
            "companion arm, openspec lifecycle: the overlay under companion/fixtures/openspec/full/check-stage-entry/ carries no change delta, so no second component is planted",
        ));
    }
    let words = words_or(
        state,
        &openspec_page(state),
        "companion-full",
        "--profile full",
        "companion arm, openspec lifecycle: the OpenSpec page carries no full line to run",
    )?;
    let c = install(state, "openspec", "openspec-lifecycle", &words)?;
    green(state, &format!("openspec lifecycle, {}", words.join(" ")), &c)?;
    copy_tree(Path::new(&overlay), Path::new(&c))
        .map_err(|e| fail(format!("companion arm, openspec lifecycle: could not copy the overlay in: {}", e)))?;
    let m = merged_in(&gates_bin(&c)?, &["--only", "check-stage-entry"], &[], &c)?;
    let said = out(&m);
    if m.code() != Some(1) || !said.contains("amendments span 2 component dirs") {
        return Err(failed(&m, format!(
            "companion arm, openspec lifecycle: check-stage-entry did not red the two-capability change at a build cursor with no align stamp (exit {})",
            m.reported_code()
        )));
    }
    for d in deltas(&c, false)? {
        let dir = d.strip_suffix("/spec.md").unwrap_or(&d);
        if !said.contains(dir) {
            return Err(failed(&m, format!(
                "companion arm, openspec lifecycle: the finding does not name the delta directory {}",
                dir
            )));
        }
    }
    say("openspec lifecycle: check-stage-entry reds the two-capability change, naming both delta directories");
    for d in &planted {
        let dir = d.strip_suffix("/spec.md").unwrap_or(d);
        let cap = dir.rsplit('/').next().unwrap_or(dir);
        for gone in [format!("{}/{}", c, dir), format!("{}/openspec/specs/{}", c, cap)] {
            if Path::new(&gone).exists() && std::fs::remove_dir_all(&gone).is_err() {
                return Err(fail(format!("companion arm, openspec lifecycle: could not remove the overlay's delta {}", dir)));
            }
        }
    }
    let m = merged_in(&gates_bin(&c)?, &["--only", "check-stage-entry"], &[], &c)?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "companion arm, openspec lifecycle: check-stage-entry did not clear once the change touches one capability (exit {})",
            m.reported_code()
        )));
    }
    say("openspec lifecycle: check-stage-entry clears once the change touches one capability");
    let have = toolkits(state);
    if have != ARMED {
        return Err(fail(format!(
            "companion arm: the recipe directories are [{}] and the arm ran [{}]; a recipe with no arm header is untested",
            have.join(" "),
            ARMED.join(" ")
        )));
    }
    Ok(())
}
