// spec: canon-kit/SPEC.md §The reference-link grammar — the on-site SPEC mirror, preserving
// mirrored-page links and rewriting only source/directory links to the self-repo blob/tree
// grammar. Its source set is derived from the tree, so its one knob is a URL prefix.
use crate::emit::self_repo_prefix;
use crate::fresh;
use crate::walk;
use std::path::Path;

const USAGE: &str = "\
usage: --emit docs-mirror [--write|--list|--emit <src>] [--root <dir>]
  --write (default) writes every mirror page under <root>/docs/; --list prints the
  source set; --emit prints one page.
";

const BLOB_REF_KNOB: &str = "CANON_KIT_DOCS_BLOB_REF";

enum Mode {
    Write,
    List,
    One(String),
    Help,
}

fn parse(args: &[String]) -> Result<(Mode, String), String> {
    let mut mode = Mode::Write;
    let mut root = ".".to_string();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => mode = Mode::Write,
            "--list" => mode = Mode::List,
            "--emit" => {
                mode = Mode::One(args.get(i + 1).cloned().unwrap_or_default());
                i += 1;
            }
            "--root" => {
                root = args
                    .get(i + 1)
                    .cloned()
                    .filter(|r| !r.is_empty())
                    .unwrap_or_else(|| ".".to_string());
                i += 1;
            }
            "-h" | "--help" => mode = Mode::Help,
            other => return Err(format!("unknown argument: {}", other)),
        }
        i += 1;
    }
    Ok((mode, root))
}

// spec: canon-kit/SPEC.md §The reference-link grammar — a kit-level SPEC/README or the doctrine
// deliverable: the set this generator emits, and therefore the set whose links stay relative
fn is_mirrored(rel: &str) -> bool {
    if rel == "doctrine-kit/DOCTRINE.md" {
        return true;
    }
    if !(rel.ends_with("/SPEC.md") || rel.ends_with("/README.md")) {
        return false;
    }
    rel.matches('/').count() == 1
}

// spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — one of the two
// GNU-only invocations this port retires. Lexical by construction: `-m` requires no component to
// exist and no mirrored source is a symlink, so normalising the join is the whole of it.
fn relative_to(root: &str, target: &str) -> String {
    let r: Vec<&str> = root.split('/').filter(|s| !s.is_empty()).collect();
    let t: Vec<&str> = target.split('/').filter(|s| !s.is_empty()).collect();
    let mut i = 0usize;
    while i < r.len() && i < t.len() && r[i] == t[i] {
        i += 1;
    }
    let mut parts: Vec<&str> = vec![".."; r.len() - i];
    parts.extend_from_slice(&t[i..]);
    if parts.is_empty() {
        return ".".to_string();
    }
    parts.join("/")
}

struct Ctx {
    root: String,
    abs_root: String,
    blob: String,
    tree: String,
}

impl Ctx {
    fn under(&self, rel: &str) -> String {
        format!("{}/{}", self.root, rel)
    }
}

// spec: canon-kit/SPEC.md §The reference-link grammar — a link into the mirrored set keeps its
// relative target; anything else resolving to a file becomes a blob link and a directory a tree
// link. An unresolved target is left alone, because a fenced example is not a link.
fn rewrite_target(ctx: &Ctx, srcdir: &str, tgt: &str) -> String {
    if tgt.contains("://") || tgt.starts_with("mailto:") || tgt.starts_with('#') || tgt.contains(' ')
    {
        return tgt.to_string();
    }
    let (path, anchor) = match tgt.split_once('#') {
        Some((p, a)) => (p, format!("#{}", a)),
        None => (tgt, String::new()),
    };
    if path.is_empty() {
        return tgt.to_string();
    }
    let res = relative_to(
        &ctx.abs_root,
        &walk::normalize_abs(&format!("{}/{}/{}", ctx.abs_root, srcdir, path)),
    );
    if res.is_empty() {
        return tgt.to_string();
    }
    let on_disk = ctx.under(&res);
    if is_mirrored(&res) && Path::new(&on_disk).is_file() {
        tgt.to_string()
    } else if Path::new(&on_disk).is_file() {
        format!("{}{}{}", ctx.blob, res, anchor)
    } else if Path::new(&on_disk).is_dir() {
        format!("{}{}/", ctx.tree, res)
    } else {
        tgt.to_string()
    }
}

// spec: canon-kit/SPEC.md §check-md-refs — a link target is the text between `](` and the next
// `)`, the shape check-md-refs and check-docs-link-convention key on
fn rewrite_line(ctx: &Ctx, srcdir: &str, line: &str) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(at) = rest.find("](") {
        let (pre, after) = (&rest[..at], &rest[at + 2..]);
        match after.find(')') {
            Some(close) => {
                let new = rewrite_target(ctx, srcdir, &after[..close]);
                out.push_str(pre);
                out.push_str("](");
                out.push_str(&new);
                out.push(')');
                rest = &after[close + 1..];
            }
            None => {
                out.push_str(pre);
                out.push_str("](");
                out.push_str(after);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

fn emit_one(ctx: &Ctx, src: &str) -> Result<String, String> {
    let (srcdir, base) = match src.rsplit_once('/') {
        Some((d, b)) => (d, b),
        None => ("", src),
    };
    let title = match base {
        "README.md" => "README",
        "SPEC.md" => "SPEC",
        "DOCTRINE.md" => "DOCTRINE",
        _ => return Err(format!("not a mirrored source: {}", src)),
    };

    let text = fresh::read_captured(&ctx.under(src))?;
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&format!("title: {}\n", title));
    out.push_str("generated: true\n");
    out.push_str("---\n");
    out.push_str(&format!(
        "<!-- Generated by `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` from {}. \
         Do not edit; run the generator. -->\n",
        src
    ));
    for line in fresh::file_lines(&text) {
        out.push_str(&rewrite_line(ctx, srcdir, line));
        out.push('\n');
    }
    Ok(out)
}

// spec: canon-kit/SPEC.md §The reference-link grammar — every mirrored source path, repo-relative,
// in byte order: one directory level down, a SPEC.md and its optional README.md, plus the doctrine
// deliverable. Derived from the tree, so a new kit joins the mirror by existing.
fn sources(ctx: &Ctx) -> Result<Vec<String>, String> {
    let mut dirs: Vec<String> = Vec::new();
    for (name, is_dir) in walk::list_dir(Path::new(&ctx.root))? {
        if is_dir && !name.starts_with('.') {
            dirs.push(name);
        }
    }
    dirs.sort();
    let mut out: Vec<String> = Vec::new();
    for d in dirs {
        if !Path::new(&ctx.under(&format!("{}/SPEC.md", d))).is_file() {
            continue;
        }
        out.push(format!("{}/SPEC.md", d));
        if Path::new(&ctx.under(&format!("{}/README.md", d))).is_file() {
            out.push(format!("{}/README.md", d));
        }
    }
    if Path::new(&ctx.under("doctrine-kit/DOCTRINE.md")).is_file() {
        out.push("doctrine-kit/DOCTRINE.md".to_string());
    }
    Ok(out)
}

fn context(root: &str) -> Result<Ctx, String> {
    let root = fresh::strip_trailing_slash(root).to_string();
    if !fresh::is_dir(&root) {
        return Err(format!("not a directory: {}", root));
    }
    let here = std::env::current_dir()
        .map_err(|e| format!("cannot read the current directory: {}", e))?
        .display()
        .to_string();
    let abs_root = if root.starts_with('/') {
        walk::normalize_abs(&root)
    } else {
        walk::normalize_abs(&format!("{}/{}", here, root))
    };
    let blob = self_repo_prefix(&walk::knob_scalar(BLOB_REF_KNOB)?);
    let tree = blob.replace("/blob/", "/tree/");
    Ok(Ctx {
        root,
        abs_root,
        blob,
        tree,
    })
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the three modes ride the arm's own argv tail, so one
// member serves the freshness comparator (`--list`, `--emit <src>`) and the regen command
// (`--write`) exactly as the one shell tool did.
pub fn emit(args: &[String]) -> Result<String, String> {
    let (mode, root) = parse(args)?;
    if let Mode::Help = mode {
        return Ok(USAGE.to_string());
    }
    let ctx = context(&root)?;

    match mode {
        Mode::Help => unreachable!(),
        Mode::List => {
            let mut out = String::new();
            for s in sources(&ctx)? {
                out.push_str(&s);
                out.push('\n');
            }
            Ok(out)
        }
        Mode::One(src) => {
            if src.is_empty() {
                return Err("--emit needs a source path".to_string());
            }
            if !Path::new(&ctx.under(&src)).is_file() {
                return Err(format!("source not found: {}", ctx.under(&src)));
            }
            emit_one(&ctx, &src)
        }
        Mode::Write => {
            let srcs = sources(&ctx)?;
            for src in &srcs {
                let dest = ctx.under(&format!("docs/{}", src));
                if let Some((dir, _)) = dest.rsplit_once('/') {
                    std::fs::create_dir_all(dir)
                        .map_err(|e| format!("cannot create {}: {}", dir, e))?;
                }
                let page = emit_one(&ctx, src)?;
                std::fs::write(&dest, page)
                    .map_err(|e| format!("cannot write {}: {}", dest, e))?;
            }
            Ok(format!(
                "docs-mirror: wrote {} mirror page(s) under {}/docs/\n",
                srcs.len(),
                ctx.root
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §The reference-link grammar — the mirrored set is exactly one
    // directory level of SPEC/README plus the doctrine deliverable, so a nested or root-level
    // page is not topology-preserved and takes a blob link instead
    #[test]
    fn the_mirrored_set_is_one_directory_level_plus_the_doctrine_deliverable() {
        assert!(is_mirrored("gate-sdk/SPEC.md"));
        assert!(is_mirrored("gate-sdk/README.md"));
        assert!(is_mirrored("doctrine-kit/DOCTRINE.md"));
        assert!(!is_mirrored("SPEC.md"));
        assert!(!is_mirrored("a/b/SPEC.md"));
        assert!(!is_mirrored("gate-sdk/SPEC-amendment.md"));
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the crate path arithmetic that replaces
    // `realpath -m --relative-to`, including the climb-out the shell form also produced
    #[test]
    fn a_target_resolves_relative_to_the_root_and_may_climb_out_of_it() {
        assert_eq!(relative_to("/r", &walk::normalize_abs("/r/a/../b/c")), "b/c");
        assert_eq!(relative_to("/r", &walk::normalize_abs("/r/a/./b")), "a/b");
        assert_eq!(relative_to("/r/x", &walk::normalize_abs("/r/x/../y")), "../y");
        assert_eq!(relative_to("/r", &walk::normalize_abs("/r")), ".");
    }
}
