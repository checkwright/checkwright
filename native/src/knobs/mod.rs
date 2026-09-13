// spec: gate-sdk/SPEC.md §lib/gate.sh — a knob has exactly one producer, and which one is a property
// of its owning kit: a static kit's knobs resolve here from its defaults table and knob files, every
// other name crosses the config bridge, and no name is ever both
use crate::knobfile::{self, Form};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub mod doctrine_kit;
pub mod site_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Scalar,
    Indexed,
    // spec: gate-sdk/SPEC.md §The knob file — a shape of the grammar every later cut inherits, held
    // before any static kit declares one
    #[cfg_attr(not(test), allow(dead_code))]
    Keyed,
}

impl Shape {
    pub fn word(self) -> &'static str {
        match self {
            Shape::Scalar => "scalar",
            Shape::Indexed => "indexed",
            Shape::Keyed => "keyed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Scalar(String),
    Indexed(Vec<String>),
    Keyed(Vec<(String, String)>),
}

impl Value {
    // spec: gate-sdk/SPEC.md §lib/gate.sh — the bridge's own serialization, so a reader parses a
    // static value exactly as it parses a bridged one
    pub fn wire(&self) -> String {
        match self {
            Value::Scalar(s) => s.clone(),
            Value::Indexed(v) => v.join("\t"),
            Value::Keyed(m) => m
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("\t"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Env,
    Local,
    Tracked,
    Default,
}

// spec: gate-sdk/SPEC.md §The knob file — a derived default is a function of already-resolved knobs,
// handed the resolver so it can ask a sibling's value and where that value came from
pub type Resolve<'a> = &'a dyn Fn(&str) -> Result<(Value, Origin), String>;

pub enum Default {
    Scalar(&'static str),
    Indexed(&'static [&'static str]),
    Derived(fn(Resolve) -> Result<Value, String>),
}

pub struct Row {
    pub name: &'static str,
    pub shape: Shape,
    pub default: Default,
}

pub struct Kit {
    pub root: &'static str,
    pub rows: &'static [Row],
}

impl Kit {
    pub fn stem(&self) -> &'static str {
        self.root.strip_suffix("-kit").unwrap_or(self.root)
    }

    pub fn prefix(&self) -> String {
        format!("{}_", self.root.to_ascii_uppercase().replace('-', "_"))
    }

    pub fn knob_file_var(&self) -> String {
        format!("{}KNOB_FILE", self.prefix())
    }

    fn row(&self, name: &str) -> Option<&'static Row> {
        self.rows.iter().find(|r| r.name == name)
    }
}

pub const STATIC_KITS: &[&Kit] = &[&doctrine_kit::KIT, &site_kit::KIT];

// spec: canon-kit/SPEC.md §check-docs-cmd — every name a static kit's reader reads: its declared
// knobs, its file locator, and the retired locator the legacy refusal still reads
pub fn static_names() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for kit in STATIC_KITS {
        out.extend(kit.rows.iter().map(|r| r.name.to_string()));
        out.push(kit.knob_file_var());
        out.push(format!("{}CONFIG_FILE", kit.prefix()));
    }
    out
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — ownership is the prefix rule the bridge's own owning-kit
// derivation uses, so a static kit owns every name its prefix spells, declared or not
pub fn owner(name: &str) -> Option<&'static Kit> {
    STATIC_KITS
        .iter()
        .copied()
        .find(|k| name.starts_with(&k.prefix()))
}

pub fn is_static(name: &str) -> bool {
    owner(name.trim_end_matches('*')).is_some()
}

// spec: gate-sdk/SPEC.md §The knob file — the gates directory is a locator, env-or-default, because
// a file cannot name its own directory
pub const GATES_DIR_DEFAULT: &str = "scripts";

fn gates_dir() -> String {
    std::env::var("GATE_SDK_GATES_DIR")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| GATES_DIR_DEFAULT.to_string())
}

type Layer = BTreeMap<String, Value>;

struct Layers {
    local: Layer,
    tracked: Layer,
}

type CacheKey = (&'static str, String, String, String);
type Loaded = Arc<Result<Layers, String>>;
type Cache = Mutex<Vec<(CacheKey, Loaded)>>;

fn cache() -> &'static Cache {
    static CACHE: Cache = Mutex::new(Vec::new());
    &CACHE
}

#[cfg(test)]
pub fn reset(_guard: &crate::knobenv::KnobEnv) {
    cache().lock().unwrap_or_else(|e| e.into_inner()).clear();
}

fn layers(kit: &'static Kit) -> Loaded {
    let key: CacheKey = (
        kit.root,
        gates_dir(),
        std::env::var(kit.knob_file_var()).unwrap_or_default(),
        std::env::var(format!("{}CONFIG_FILE", kit.prefix())).unwrap_or_default(),
    );
    let mut held = cache().lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, l)) = held.iter().find(|(k, _)| *k == key) {
        return l.clone();
    }
    let l = Arc::new(load(kit, &key.1, &key.2, &key.3));
    held.push((key, l.clone()));
    l
}

// spec: gate-sdk/SPEC.md §The knob file — the legacy refusals come first, because a shell config left
// behind at upgrade would otherwise be silently ignored and drop the consumer's values
fn load(kit: &'static Kit, dir: &str, knob_file: &str, config_file: &str) -> Result<Layers, String> {
    let stem = kit.stem();
    for legacy in [
        format!("{}/{}-config.sh", dir, stem),
        format!("{}/{}-config.local.sh", dir, stem),
    ] {
        if Path::new(&legacy).exists() {
            return Err(format!(
                "{} is a shell config, and {}'s knobs are read from a knob file now — rewrite it as \
                 {}/{}-config{}.knobs (`NAME = value`, one `NAME[] = element` line per array \
                 element) and delete it (gate-sdk/SPEC.md §The knob file)",
                legacy,
                kit.root,
                dir,
                stem,
                if legacy.ends_with(".local.sh") { ".local" } else { "" }
            ));
        }
    }
    if !config_file.is_empty()
        && std::fs::metadata(config_file).map(|m| m.len() > 0).unwrap_or(false)
    {
        return Err(format!(
            "{}CONFIG_FILE names {}, a shell config — rewrite it in the knob-file grammar and point \
             {} at it instead (gate-sdk/SPEC.md §The knob file)",
            kit.prefix(),
            config_file,
            kit.knob_file_var()
        ));
    }
    let tracked_path = if knob_file.is_empty() {
        format!("{}/{}-config.knobs", dir, stem)
    } else {
        if !Path::new(knob_file).is_file() {
            return Err(format!(
                "{} names {}, which does not exist — point it at the knob file, or unset it to read \
                 {}/{}-config.knobs",
                kit.knob_file_var(),
                knob_file,
                dir,
                stem
            ));
        }
        knob_file.to_string()
    };
    let local_path = format!("{}/{}-config.local.knobs", dir, stem);
    Ok(Layers {
        local: read_layer(kit, &local_path)?,
        tracked: read_layer(kit, &tracked_path)?,
    })
}

fn read_layer(kit: &'static Kit, path: &str) -> Result<Layer, String> {
    if !Path::new(path).is_file() {
        return Ok(Layer::new());
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    layer(kit, path, &text)
}

// spec: gate-sdk/SPEC.md §The knob file — a knob named in a file is replaced whole, and a line whose
// form disagrees with the declared shape is refused with its file and line
fn layer(kit: &'static Kit, path: &str, text: &str) -> Result<Layer, String> {
    let mut out = Layer::new();
    let mut emptied: Vec<String> = Vec::new();
    let at = |lno: usize, what: String| format!("{}:{}: {}", path, lno, what);
    for e in knobfile::parse(text, path)? {
        let Some(row) = kit.row(&e.name) else {
            return Err(at(
                e.lno,
                format!(
                    "{} is not a {} knob — `--emit knob-roster` lists the names this file may set",
                    e.name, kit.root
                ),
            ));
        };
        let twice = || at(e.lno, format!("{} is given twice — keep one line for it", e.name));
        let shape_word = row.shape.word();
        let mismatch = || {
            at(
                e.lno,
                format!(
                    "{} is declared {} and this line is not that form — write {}",
                    e.name,
                    shape_word,
                    match row.shape {
                        Shape::Scalar => format!("`{} = value`", e.name),
                        Shape::Indexed => format!("one `{}[] = element` line per element", e.name),
                        Shape::Keyed => format!("one `{}[key] = value` line per pair", e.name),
                    }
                ),
            )
        };
        match (row.shape, &e.form) {
            (Shape::Scalar, Form::Scalar) => {
                if out.contains_key(&e.name) {
                    return Err(twice());
                }
                out.insert(e.name.clone(), Value::Scalar(e.value.clone()));
            }
            (Shape::Indexed | Shape::Keyed, Form::Scalar) if e.value.is_empty() => {
                if out.contains_key(&e.name) {
                    return Err(twice());
                }
                emptied.push(e.name.clone());
                let v = if row.shape == Shape::Indexed {
                    Value::Indexed(Vec::new())
                } else {
                    Value::Keyed(Vec::new())
                };
                out.insert(e.name.clone(), v);
            }
            (Shape::Indexed, Form::Indexed) => {
                if emptied.contains(&e.name) {
                    return Err(twice());
                }
                match out
                    .entry(e.name.clone())
                    .or_insert_with(|| Value::Indexed(Vec::new()))
                {
                    Value::Indexed(v) => v.push(e.value.clone()),
                    _ => return Err(mismatch()),
                }
            }
            (Shape::Keyed, Form::Keyed(key)) => {
                if emptied.contains(&e.name) {
                    return Err(twice());
                }
                match out
                    .entry(e.name.clone())
                    .or_insert_with(|| Value::Keyed(Vec::new()))
                {
                    Value::Keyed(m) => {
                        if m.iter().any(|(k, _)| k == key) {
                            return Err(twice());
                        }
                        m.push((key.clone(), e.value.clone()));
                        m.sort();
                    }
                    _ => return Err(mismatch()),
                }
            }
            _ => return Err(mismatch()),
        }
    }
    Ok(out)
}

fn default_value(row: &'static Row, resolve: Resolve) -> Result<Value, String> {
    Ok(match &row.default {
        Default::Scalar(s) => Value::Scalar(s.to_string()),
        Default::Indexed(v) => Value::Indexed(v.iter().map(|s| s.to_string()).collect()),
        Default::Derived(f) => f(resolve)?,
    })
}

fn undeclared(kit: &Kit, name: &str) -> String {
    format!(
        "{} is not a {} knob — `--emit knob-roster` lists the names it declares",
        name, kit.root
    )
}

// spec: gate-sdk/SPEC.md §The knob file — the precedence, highest first: the environment for a
// scalar only, the local overlay, the tracked file, the kit default
pub fn resolve(name: &str) -> Result<(Value, Origin), String> {
    let kit = owner(name).ok_or_else(|| format!("{} is not a statically owned knob", name))?;
    let row = kit.row(name).ok_or_else(|| undeclared(kit, name))?;
    let l = layers(kit);
    let l = l.as_ref().as_ref().map_err(String::clone)?;
    if row.shape == Shape::Scalar {
        if let Ok(v) = std::env::var(name) {
            return Ok((Value::Scalar(v), Origin::Env));
        }
    }
    if let Some(v) = l.local.get(name) {
        return Ok((v.clone(), Origin::Local));
    }
    if let Some(v) = l.tracked.get(name) {
        return Ok((v.clone(), Origin::Tracked));
    }
    Ok((default_value(row, &resolve)?, Origin::Default))
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the one function every knob read resolves through: a
// statically owned name from its kit, any other from its `GATE_SDK_KNOB_<NAME>` variable, whose
// absence is the caller's to report
pub fn wire(name: &str) -> Result<Option<String>, String> {
    if owner(name).is_some() {
        return resolve(name).map(|(v, _)| Some(v.wire()));
    }
    Ok(std::env::var(format!("GATE_SDK_KNOB_{}", name)).ok())
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `--emit knob-roster`: every statically owned knob with
// its shape and default, the default rendered with every sibling at its own default
pub fn roster() -> Result<String, String> {
    fn defaults_only(name: &str) -> Result<(Value, Origin), String> {
        let kit = owner(name).ok_or_else(|| format!("{} is not a statically owned knob", name))?;
        let row = kit.row(name).ok_or_else(|| undeclared(kit, name))?;
        Ok((default_value(row, &defaults_only)?, Origin::Default))
    }
    let mut out = String::new();
    for kit in STATIC_KITS {
        for row in kit.rows {
            let (v, _) = defaults_only(row.name)?;
            let lines: Vec<String> = match v {
                Value::Scalar(s) => vec![s],
                Value::Indexed(e) => e,
                Value::Keyed(m) => m.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect(),
            };
            if lines.is_empty() {
                out.push_str(&format!("{}\t{}\t\n", row.name, row.shape.word()));
            }
            for l in lines {
                out.push_str(&format!("{}\t{}\t{}\n", row.name, row.shape.word(), l));
            }
        }
    }
    Ok(out)
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    roster()
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `--knobs` answers the knobs a member needs the bridge to
// carry, so a statically owned name never reaches a kit library that no longer defines it
pub fn bridged<'a>(names: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
    names.into_iter().filter(|n| !is_static(n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knobenv;

    struct Scratch(std::path::PathBuf);

    impl Scratch {
        fn new(tag: &str) -> Scratch {
            let d = std::env::temp_dir().join(format!("checkwright-knobs-{}.{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&d);
            std::fs::create_dir_all(&d).expect("scratch");
            Scratch(d)
        }
        fn write(&self, rel: &str, body: &str) -> String {
            let p = self.0.join(rel);
            std::fs::write(&p, body).expect("write");
            p.display().to_string()
        }
        fn dir(&self) -> String {
            self.0.display().to_string()
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn clean(env: &knobenv::KnobEnv, dir: &str) {
        env.set("GATE_SDK_GATES_DIR", dir);
        for k in STATIC_KITS {
            env.remove(&k.knob_file_var());
            env.remove(&format!("{}CONFIG_FILE", k.prefix()));
            for r in k.rows {
                env.remove(r.name);
            }
        }
        reset(env);
    }

    fn restore(env: &knobenv::KnobEnv) {
        env.remove("GATE_SDK_GATES_DIR");
        reset(env);
    }

    #[test]
    fn precedence_is_env_then_local_then_tracked_then_default() {
        let env = knobenv::lock();
        let s = Scratch::new("precedence");
        clean(&env, &s.dir());
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap(), (Value::Scalar("CLAUDE.md".into()), Origin::Default));
        s.write("doctrine-config.knobs", "DOCTRINE_KIT_AGENT_FILE = TRACKED.md\n");
        reset(&env);
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap().1, Origin::Tracked);
        s.write("doctrine-config.local.knobs", "DOCTRINE_KIT_AGENT_FILE = LOCAL.md\n");
        reset(&env);
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap(), (Value::Scalar("LOCAL.md".into()), Origin::Local));
        env.set("DOCTRINE_KIT_AGENT_FILE", "ENV.md");
        assert_eq!(resolve("DOCTRINE_KIT_AGENT_FILE").unwrap(), (Value::Scalar("ENV.md".into()), Origin::Env));
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn an_indexed_knob_takes_no_environment_override_and_is_replaced_whole() {
        let env = knobenv::lock();
        let s = Scratch::new("indexed");
        clean(&env, &s.dir());
        env.set("SITE_KIT_EXEMPT_PATHS", "from-env");
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap().unwrap(), "*/gate-tests/*\t*docs/posts/*");
        s.write("site-config.knobs", "SITE_KIT_EXEMPT_PATHS[] = only one\n");
        reset(&env);
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap().unwrap(), "only one");
        s.write("site-config.knobs", "SITE_KIT_EXEMPT_PATHS =\n");
        reset(&env);
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap().unwrap(), "");
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn every_file_refusal_names_its_file_and_line() {
        let env = knobenv::lock();
        let s = Scratch::new("refusals");
        let undeclared_line = format!("{}NOPE = 1\n", site_kit::KIT.prefix());
        for (body, want) in [
            (undeclared_line.as_str(), "not a site-kit knob"),
            ("SITE_KIT_CNAME[] = x\n", "is declared scalar"),
            ("SITE_KIT_ALIASES = x\n", "is declared indexed"),
            ("SITE_KIT_ALIASES[k] = x\n", "is declared indexed"),
            ("SITE_KIT_CNAME = a\nSITE_KIT_CNAME = b\n", "given twice"),
            ("SITE_KIT_ALIASES =\nSITE_KIT_ALIASES[] = a\n", "given twice"),
            ("GATE_SDK_TMP_DIR = x\n", "not a site-kit knob"),
            ("site_kit_cname = x\n", "SCREAMING_SNAKE"),
        ] {
            clean(&env, &s.dir());
            let p = s.write("site-config.knobs", &format!("# header\n{}", body));
            reset(&env);
            let e = resolve("SITE_KIT_CNAME").expect_err(body);
            let lno = body.lines().count() + 1;
            assert!(e.starts_with(&format!("{}:{}: ", p, lno)), "{:?}: {}", body, e);
            assert!(e.contains(want), "{:?}: {}", body, e);
        }
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn the_legacy_configs_and_a_set_missing_knob_file_are_refused() {
        let env = knobenv::lock();
        let s = Scratch::new("legacy");
        clean(&env, &s.dir());
        s.write("site-config.local.sh", "SITE_KIT_CNAME=x\n");
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("site-config.local.knobs"));
        std::fs::remove_file(s.0.join("site-config.local.sh")).unwrap();
        s.write("site-config.sh", "SITE_KIT_CNAME=x\n");
        reset(&env);
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("is a shell config"));
        std::fs::remove_file(s.0.join("site-config.sh")).unwrap();

        let empty = s.write("empty.sh", "");
        env.set("SITE_KIT_CONFIG_FILE", &empty);
        reset(&env);
        assert!(resolve("SITE_KIT_CNAME").is_ok(), "an empty pinned shell config is inert");
        let full = s.write("full.sh", "SITE_KIT_CNAME=x\n");
        env.set("SITE_KIT_CONFIG_FILE", &full);
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("SITE_KIT_KNOB_FILE"));
        env.remove("SITE_KIT_CONFIG_FILE");

        env.set("SITE_KIT_KNOB_FILE", &format!("{}/absent.knobs", s.dir()));
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("does not exist"));
        let pinned = s.write("pinned.knobs", "SITE_KIT_CNAME = PINNED\n");
        env.set("SITE_KIT_KNOB_FILE", &pinned);
        assert_eq!(wire("SITE_KIT_CNAME").unwrap().unwrap(), "PINNED");
        clean(&env, &s.dir());
        restore(&env);
    }

    static KEYED_KIT: Kit = Kit {
        root: "probe-kit",
        rows: &[Row {
            name: "PROBE_KIT_MAP",
            shape: Shape::Keyed,
            default: Default::Indexed(&[]),
        }],
    };

    #[test]
    fn a_keyed_knob_takes_pairs_sorted_and_refuses_a_repeated_key_or_an_element() {
        let got = layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[b] = 2 = two\nPROBE_KIT_MAP[a] = 1\n").expect("parses");
        assert_eq!(got["PROBE_KIT_MAP"].wire(), "a=1\tb=2 = two");
        assert!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[a] = 1\nPROBE_KIT_MAP[a] = 2\n").unwrap_err().starts_with("f:2: "));
        assert!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[] = 1\n").unwrap_err().contains("is declared keyed"));
        assert_eq!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP =\n").expect("empties")["PROBE_KIT_MAP"].wire(), "");
    }

    #[test]
    fn an_undeclared_static_name_is_an_error_and_a_bridged_name_reads_the_wire() {
        let env = knobenv::lock();
        assert!(wire(&format!("{}NOT_A_KNOB", site_kit::KIT.prefix())).unwrap_err().contains("not a site-kit knob"));
        env.set("GATE_SDK_KNOB_PROBE_ROUTED", "a\tb");
        assert_eq!(wire("PROBE_ROUTED").unwrap().as_deref(), Some("a\tb"));
        env.remove("GATE_SDK_KNOB_PROBE_ROUTED");
        assert_eq!(wire("PROBE_ROUTED").unwrap(), None);
    }

    #[test]
    fn the_roster_prints_one_line_per_element_and_an_empty_default_as_an_empty_field() {
        let r = roster().expect("renders");
        assert!(r.contains("SITE_KIT_CNAME\tscalar\tdocs/CNAME\n"));
        assert!(r.contains("SITE_KIT_ALIASES\tindexed\t\n"));
        assert!(r.contains("SITE_KIT_EXEMPT_PATHS\tindexed\t*/gate-tests/*\nSITE_KIT_EXEMPT_PATHS\tindexed\t*docs/posts/*\n"));
        for line in r.lines() {
            assert_eq!(line.split('\t').count(), 3, "{:?}", line);
        }
    }

    #[test]
    fn a_static_name_is_never_bridged() {
        assert_eq!(
            bridged(["SITE_KIT_CNAME", "GATE_PRUNE_DIRS", "DOCTRINE_KIT_AGENT_FILE"]),
            vec!["GATE_PRUNE_DIRS"]
        );
    }

    // spec: gate-sdk/SPEC.md §The knob file — the gates-directory default is spelled here and in
    // lib/gate.sh, and no single owner can hold a locator until gate-sdk migrates
    #[test]
    fn the_gates_dir_default_agrees_with_the_shell_library() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let out = std::process::Command::new("env")
            .args(["-u", "GATE_SDK_GATES_DIR", "bash", "-c", "source gate-sdk/lib/gate.sh; gate_sdk_gates_dir"])
            .current_dir(&root)
            .output()
            .expect("cannot run the shell library");
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), GATES_DIR_DEFAULT);
    }
}
