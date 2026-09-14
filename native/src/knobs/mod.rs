// spec: gate-sdk/SPEC.md §lib/gate.sh — a knob has exactly one producer: its owning kit's defaults
// table and knob files, resolved here
use crate::knobfile::{self, Form};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub mod canon_kit;
pub mod context_kit;
pub mod delegation_kit;
pub mod doctrine_kit;
pub mod drift_kit;
pub mod evidence_kit;
pub mod gate_sdk;
pub mod guard_kit;
pub mod lifecycle_kit;
pub mod queue_kit;
pub mod site_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Scalar,
    Indexed,
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
    // spec: gate-sdk/SPEC.md §lib/gate.sh — the one serialization every in-process reader parses:
    // elements tab-joined, a keyed pair as `<key>=<value>`
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
    // spec: gate-sdk/SPEC.md §The knob file — the roster's stand-in for a locator input, `${NAME}`: a
    // derivation that probes the filesystem renders its first candidate rather than probe this path
    Placeholder,
}

impl Origin {
    // spec: gate-sdk/SPEC.md §The knob file — a consumer set the value: the environment or a file
    pub fn is_set(self) -> bool {
        matches!(self, Origin::Env | Origin::Local | Origin::Tracked)
    }
}

// spec: gate-sdk/SPEC.md §The knob file — a derived default is a function of already-resolved knobs,
// handed the resolver so it can ask a sibling's value and where that value came from
pub type Resolve<'a> = &'a dyn Fn(&str) -> Result<(Value, Origin), String>;

pub enum Default {
    Scalar(&'static str),
    Indexed(&'static [&'static str]),
    Keyed(&'static [(&'static str, &'static str)]),
    Derived(fn(Resolve) -> Result<Value, String>),
}

pub struct Row {
    pub name: &'static str,
    pub shape: Shape,
    pub default: Default,
    // spec: gate-sdk/SPEC.md §The knob file — every name a derived default reads, so the set is known
    // without running the derivation; empty on every other row
    pub inputs: &'static [&'static str],
    // spec: gate-sdk/SPEC.md §The knob file — an empty layered value answers the default on this row
    pub empty_takes_default: bool,
}

impl Row {
    pub const fn scalar(name: &'static str, v: &'static str) -> Row {
        Row { name, shape: Shape::Scalar, default: Default::Scalar(v), inputs: &[], empty_takes_default: false }
    }

    pub const fn indexed(name: &'static str, v: &'static [&'static str]) -> Row {
        Row { name, shape: Shape::Indexed, default: Default::Indexed(v), inputs: &[], empty_takes_default: false }
    }

    pub const fn keyed(name: &'static str, v: &'static [(&'static str, &'static str)]) -> Row {
        Row { name, shape: Shape::Keyed, default: Default::Keyed(v), inputs: &[], empty_takes_default: false }
    }

    pub const fn derived(
        name: &'static str,
        shape: Shape,
        f: fn(Resolve) -> Result<Value, String>,
        inputs: &'static [&'static str],
    ) -> Row {
        Row { name, shape, default: Default::Derived(f), inputs, empty_takes_default: false }
    }

    pub const fn empty_takes_default(self) -> Row {
        Row { empty_takes_default: true, ..self }
    }
}

// spec: gate-sdk/SPEC.md §The knob file — a declared scalar family: each member's name is the prefix
// and a suffix; a derivation returns `(suffix, value)` defaults, reading only its declared inputs
pub type FamilyDerivation = fn(Resolve) -> Result<Vec<(String, String)>, String>;

pub struct Family {
    pub prefix: &'static str,
    pub derive: Option<FamilyDerivation>,
    pub inputs: &'static [&'static str],
}

// spec: gate-sdk/SPEC.md §The knob file — a member's suffix: non-empty and identifier-shaped
fn is_suffix(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

// spec: gate-sdk/SPEC.md §The knob file — what a kit validator reads: each row's resolved value beside
// its origin
pub type Values = BTreeMap<&'static str, (Value, Origin)>;

pub type Validator = fn(&Values) -> Vec<String>;

pub struct Kit {
    pub root: &'static str,
    pub rows: &'static [Row],
    // spec: gate-sdk/SPEC.md §The knob file — the config noun the refusal's lead line names, and the
    // check returning every finding, so a malformed config gates nothing
    pub validate: Option<(&'static str, Validator)>,
    // spec: gate-sdk/SPEC.md §The knob file — an undeclared scalar under the prefix is a consumer knob
    pub open_family: bool,
    pub families: &'static [Family],
    // spec: gate-sdk/SPEC.md §The knob file — each retired name with the name that replaced it
    pub retired: &'static [(&'static str, &'static str)],
    // spec: gate-sdk/SPEC.md §The knob file — names under the prefix the environment alone sets
    pub env_only: &'static [&'static str],
}

pub fn scalar<'a>(v: &'a Values, name: &str) -> Option<&'a str> {
    match v.get(name) {
        Some((Value::Scalar(s), _)) => Some(s.as_str()),
        _ => None,
    }
}

pub fn indexed<'a>(v: &'a Values, name: &str) -> Option<&'a [String]> {
    match v.get(name) {
        Some((Value::Indexed(e), _)) => Some(e.as_slice()),
        _ => None,
    }
}

pub fn keyed<'a>(v: &'a Values, name: &str) -> Option<&'a [(String, String)]> {
    match v.get(name) {
        Some((Value::Keyed(m), _)) => Some(m.as_slice()),
        _ => None,
    }
}

pub fn origin(v: &Values, name: &str) -> Option<Origin> {
    v.get(name).map(|(_, o)| *o)
}

// spec: gate-sdk/SPEC.md §The knob file — the validator rule that turns on set-ness: an existing-file
// knob a consumer set to a path that is not a file; an empty value is each kit's own rule
pub fn set_but_missing(v: &Values, name: &str) -> Option<String> {
    let path = scalar(v, name).filter(|p| !p.is_empty())?;
    if origin(v, name).is_some_and(Origin::is_set) && !Path::new(path).is_file() {
        return Some(format!("{} not found: {}", name, path));
    }
    None
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

    // spec: gate-sdk/SPEC.md §The knob file — the declared family a name is a member of: under its
    // prefix with a valid suffix, and never a declared row the prefix also spells
    fn family_of(&self, name: &str) -> Option<&'static Family> {
        if self.row(name).is_some() {
            return None;
        }
        self.families
            .iter()
            .find(|f| name.strip_prefix(f.prefix).is_some_and(is_suffix))
    }

    fn replacement(&self, name: &str) -> Option<&'static str> {
        self.retired.iter().find(|(n, _)| *n == name).map(|(_, r)| *r)
    }

    pub fn is_env_only(&self, name: &str) -> bool {
        self.env_only.contains(&name)
    }

    // spec: gate-sdk/SPEC.md §The knob file — an undeclared scalar line an open kit admits: under its
    // prefix, and neither a locator nor a retired name
    fn admits_consumer_scalar(&self, name: &str) -> bool {
        self.open_family
            && name.starts_with(&self.prefix())
            && name != self.knob_file_var()
            && name != format!("{}CONFIG_FILE", self.prefix())
            && self.replacement(name).is_none()
    }
}

fn retired_message(name: &str, replacement: &str) -> String {
    format!("{} is retired — set {} instead", name, replacement)
}

pub const STATIC_KITS: &[&Kit] = &[
    &canon_kit::KIT,
    &context_kit::KIT,
    &delegation_kit::KIT,
    &doctrine_kit::KIT,
    &drift_kit::KIT,
    &evidence_kit::KIT,
    &gate_sdk::KIT,
    &guard_kit::KIT,
    &lifecycle_kit::KIT,
    &queue_kit::KIT,
    &site_kit::KIT,
];

// spec: canon-kit/SPEC.md §check-docs-cmd — every name a static kit's reader reads: its declared
// knobs, its file locator, the retired locator the legacy refusal still reads, each retired name
// the refusal names, and the environment-only names
pub fn static_names() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for kit in STATIC_KITS {
        out.extend(kit.rows.iter().map(|r| r.name.to_string()));
        out.push(kit.knob_file_var());
        out.push(format!("{}CONFIG_FILE", kit.prefix()));
        out.extend(kit.retired.iter().map(|(n, _)| n.to_string()));
        out.extend(kit.env_only.iter().map(|n| n.to_string()));
    }
    out
}

// spec: gate-sdk/SPEC.md §The knob file — a kit owns every name its `<KIT>_` prefix spells, declared
// or not
pub fn owner(name: &str) -> Option<&'static Kit> {
    STATIC_KITS
        .iter()
        .copied()
        .find(|k| name.starts_with(&k.prefix()))
}

// spec: gate-sdk/SPEC.md §The knob file — the gates directory is a locator, env-or-default, because
// a file cannot name its own directory
pub const GATES_DIR_DEFAULT: &str = "scripts";

// spec: gate-sdk/SPEC.md §The knob file — a locator a derived default reads: its value, set when the
// environment carries it
fn locator(name: &str) -> Option<(Value, Origin)> {
    let value = match name {
        "GATE_SDK_GATES_DIR" => gates_dir(),
        "GATE_SDK_ROOT" => crate::walk::sdk_root(),
        _ => return None,
    };
    let origin = if std::env::var(name).is_ok_and(|v| !v.is_empty()) { Origin::Env } else { Origin::Default };
    Some((Value::Scalar(value), origin))
}

pub fn gates_dir() -> String {
    std::env::var("GATE_SDK_GATES_DIR")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| GATES_DIR_DEFAULT.to_string())
}

// spec: gate-sdk/SPEC.md §The knob file — an indexed knob's lines in file order: an element, or a
// reference carrying the `<file>:<line>` its refusal names
#[derive(Debug, Clone)]
enum Piece {
    Element(String),
    Reference { other: String, at: String },
}

#[derive(Debug, Clone)]
enum Held {
    Value(Value),
    Pieces(Vec<Piece>),
}

type Layer = BTreeMap<String, Held>;

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

type Checked = Mutex<Vec<(CacheKey, Result<(), String>)>>;

fn checked() -> &'static Checked {
    static CHECKED: Checked = Mutex::new(Vec::new());
    &CHECKED
}

#[cfg(test)]
pub fn reset(_guard: &crate::knobenv::KnobEnv) {
    cache().lock().unwrap_or_else(|e| e.into_inner()).clear();
    checked().lock().unwrap_or_else(|e| e.into_inner()).clear();
}

fn cache_key(kit: &'static Kit) -> CacheKey {
    (
        kit.root,
        gates_dir(),
        std::env::var(kit.knob_file_var()).unwrap_or_default(),
        std::env::var(format!("{}CONFIG_FILE", kit.prefix())).unwrap_or_default(),
    )
}

fn layers(kit: &'static Kit) -> Loaded {
    let key = cache_key(kit);
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
        let twice = || at(e.lno, format!("{} is given twice — keep one line for it", e.name));
        if let Some(replacement) = kit.replacement(&e.name) {
            return Err(at(e.lno, retired_message(&e.name, replacement)));
        }
        if kit.is_env_only(&e.name) {
            return Err(at(
                e.lno,
                format!("{} is not read from a knob file — set it in the environment", e.name),
            ));
        }
        let Some(row) = kit.row(&e.name) else {
            if let Some(fam) = kit.family_of(&e.name) {
                if e.form != Form::Scalar {
                    return Err(at(
                        e.lno,
                        format!(
                            "{} is a member of the {} family, and a member is a scalar — write `{} = value`",
                            e.name, fam.prefix, e.name
                        ),
                    ));
                }
                if out.contains_key(&e.name) {
                    return Err(twice());
                }
                out.insert(e.name.clone(), Held::Value(Value::Scalar(e.value.clone())));
                continue;
            }
            if e.form == Form::Scalar && kit.admits_consumer_scalar(&e.name) {
                if out.contains_key(&e.name) {
                    return Err(twice());
                }
                out.insert(e.name.clone(), Held::Value(Value::Scalar(e.value.clone())));
                continue;
            }
            return Err(at(
                e.lno,
                format!(
                    "{} is not a {} knob — `--emit knob-roster` lists the names this file may set",
                    e.name, kit.root
                ),
            ));
        };
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
                out.insert(e.name.clone(), Held::Value(Value::Scalar(e.value.clone())));
            }
            (Shape::Indexed | Shape::Keyed, Form::Scalar) if e.value.is_empty() => {
                if out.contains_key(&e.name) {
                    return Err(twice());
                }
                emptied.push(e.name.clone());
                let v = if row.shape == Shape::Indexed {
                    Held::Pieces(Vec::new())
                } else {
                    Held::Value(Value::Keyed(Vec::new()))
                };
                out.insert(e.name.clone(), v);
            }
            (Shape::Indexed, Form::Indexed | Form::Reference(_)) => {
                if emptied.contains(&e.name) {
                    return Err(twice());
                }
                let piece = match &e.form {
                    Form::Reference(other) => {
                        refuse_referent(row, other).map_err(|what| at(e.lno, what))?;
                        Piece::Reference {
                            other: other.clone(),
                            at: format!("{}:{}", path, e.lno),
                        }
                    }
                    _ => Piece::Element(e.value.clone()),
                };
                match out
                    .entry(e.name.clone())
                    .or_insert_with(|| Held::Pieces(Vec::new()))
                {
                    Held::Pieces(v) => v.push(piece),
                    _ => return Err(mismatch()),
                }
            }
            (Shape::Scalar | Shape::Keyed, Form::Reference(_)) => {
                return Err(at(
                    e.lno,
                    format!(
                        "{} is declared {}, and a reference splices elements into an indexed knob — \
                         write {}",
                        e.name,
                        shape_word,
                        if row.shape == Shape::Scalar {
                            format!("`{} = value`", e.name)
                        } else {
                            format!("one `{}[key] = value` line per pair", e.name)
                        }
                    ),
                ));
            }
            (Shape::Keyed, Form::Keyed(key)) => {
                if emptied.contains(&e.name) {
                    return Err(twice());
                }
                match out
                    .entry(e.name.clone())
                    .or_insert_with(|| Held::Value(Value::Keyed(Vec::new())))
                {
                    Held::Value(Value::Keyed(m)) => {
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
        Default::Keyed(m) => {
            let mut pairs: Vec<(String, String)> =
                m.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            pairs.sort();
            Value::Keyed(pairs)
        }
        Default::Derived(f) => f(resolve)?,
    })
}

fn undeclared(kit: &Kit, name: &str) -> String {
    if let Some(replacement) = kit.replacement(name) {
        return retired_message(name, replacement);
    }
    format!(
        "{} is not a {} knob — `--emit knob-roster` lists the names it declares",
        name, kit.root
    )
}

fn static_row(name: &str) -> Result<(&'static Kit, &'static Row), String> {
    let kit = owner(name).ok_or_else(|| format!("{} is not a statically owned knob", name))?;
    let row = kit.row(name).ok_or_else(|| undeclared(kit, name))?;
    Ok((kit, row))
}

// spec: gate-sdk/SPEC.md §The knob file — the referent refusals a file line can be held to alone:
// itself, and an unowned or non-indexed name
fn refuse_referent(named: &'static Row, other: &str) -> Result<(), String> {
    let name = named.name;
    if other == name {
        return Err(format!("{} references itself — write its elements instead", name));
    }
    let Some(kit) = owner(other) else {
        return Err(format!(
            "{} is not statically owned — write its elements here instead",
            other
        ));
    };
    if kit.row(other).filter(|r| r.shape == Shape::Indexed).is_none() {
        return Err(format!(
            "{} is not a declared indexed {} knob — a reference splices an indexed knob's elements",
            other, kit.root
        ));
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §The knob file — a held value made whole, one pass
fn materialize(held: &Held) -> Result<(Value, bool), String> {
    let pieces = match held {
        Held::Value(v) => return Ok((v.clone(), false)),
        Held::Pieces(p) => p,
    };
    let mut out: Vec<String> = Vec::new();
    let mut spliced = false;
    for piece in pieces {
        match piece {
            Piece::Element(e) => out.push(e.clone()),
            Piece::Reference { other, at } => {
                spliced = true;
                let (v, _, nested) = lookup_held(other)?;
                if nested {
                    return Err(format!(
                        "{}: {}'s own value carries a reference, and resolution is one pass — write its \
                         elements here instead",
                        at, other
                    ));
                }
                match v {
                    Value::Indexed(e) => out.extend(e),
                    _ => return Err(format!("{}: {} did not resolve to elements", at, other)),
                }
            }
        }
    }
    Ok((Value::Indexed(out), spliced))
}

// spec: gate-sdk/SPEC.md §The knob file — the precedence, highest first: the environment for a
// scalar only, the local overlay, the tracked file; `None` leaves the kit default to the caller
fn layered(kit: &'static Kit, name: &str, shape: Shape) -> Result<Option<(Value, Origin, bool)>, String> {
    let l = layers(kit);
    let l = l.as_ref().as_ref().map_err(String::clone)?;
    if shape == Shape::Scalar {
        if let Ok(v) = std::env::var(name) {
            return Ok(Some((Value::Scalar(v), Origin::Env, false)));
        }
    }
    for (layer, origin) in [(&l.local, Origin::Local), (&l.tracked, Origin::Tracked)] {
        if let Some(held) = layer.get(name) {
            let (v, spliced) = materialize(held)?;
            return Ok(Some((v, origin, spliced)));
        }
    }
    Ok(None)
}

// spec: gate-sdk/SPEC.md §The knob file — a row flagged `empty_takes_default` answers its default for
// an empty layered scalar, where every other row keeps the empty value
fn lookup_held(name: &str) -> Result<(Value, Origin, bool), String> {
    let (kit, row) = static_row(name)?;
    match layered(kit, row.name, row.shape)? {
        Some((Value::Scalar(s), _, _)) if s.is_empty() && row.empty_takes_default => {
            Ok((default_value(row, &input)?, Origin::Default, false))
        }
        Some(v) => Ok(v),
        None => Ok((default_value(row, &input)?, Origin::Default, false)),
    }
}

fn lookup(name: &str) -> Result<(Value, Origin), String> {
    lookup_held(name).map(|(v, o, _)| (v, o))
}

// spec: gate-sdk/SPEC.md §The knob file — a derivation's resolver: a static sibling through the
// table, a locator from the environment or its default
fn input(name: &str) -> Result<(Value, Origin), String> {
    if let Some(l) = locator(name) {
        return Ok(l);
    }
    lookup(name)
}

// spec: gate-sdk/SPEC.md §The knob file — the kit validator runs once per process per kit, at its
// first resolution, and refuses with every finding under the kit's malformed-config lead line; a
// retired scalar exported in the environment is refused there too
fn validated(kit: &'static Kit) -> Result<(), String> {
    let key = cache_key(kit);
    if let Some((_, r)) = checked()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|(k, _)| *k == key)
    {
        return r.clone();
    }
    let run = || -> Result<(), String> {
        if let Some((name, replacement)) = kit.retired.iter().find(|(n, _)| std::env::var_os(n).is_some()) {
            return Err(format!("{}: {}", kit.root, retired_message(name, replacement)));
        }
        let Some((noun, check)) = kit.validate else {
            return Ok(());
        };
        let mut values = Values::new();
        for row in kit.rows {
            values.insert(row.name, lookup(row.name)?);
        }
        let findings = check(&values);
        if findings.is_empty() {
            return Ok(());
        }
        Err(format!(
            "{}: malformed {} — the gates cannot run:\n  {}",
            kit.root,
            noun,
            findings.join("\n  ")
        ))
    };
    let r = run();
    checked()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push((key, r.clone()));
    r
}

pub fn resolve(name: &str) -> Result<(Value, Origin), String> {
    let (kit, _) = static_row(name)?;
    validated(kit)?;
    lookup(name)
}

// spec: gate-sdk/SPEC.md §The knob file — a kit's family read. A declared family's prefix answers its
// members: derived, then either file's, then the environment's, each replacing the one before. Any
// other prefix answers every declared scalar under it, then an open kit's consumer scalars
pub fn family(prefix: &str) -> Result<Vec<(String, String)>, String> {
    let kit = owner(prefix).ok_or_else(|| format!("{} is not a statically owned prefix", prefix))?;
    validated(kit)?;
    if let Some(fam) = kit.families.iter().find(|f| f.prefix == prefix) {
        return declared_family(kit, fam);
    }
    let mut out: Vec<(String, String)> = Vec::new();
    for row in kit.rows.iter().filter(|r| r.shape == Shape::Scalar && r.name.starts_with(prefix)) {
        out.push((row.name.to_string(), lookup(row.name)?.0.wire()));
    }
    if kit.open_family {
        let l = layers(kit);
        let l = l.as_ref().as_ref().map_err(String::clone)?;
        let consumer: std::collections::BTreeSet<&String> = l
            .local
            .keys()
            .chain(l.tracked.keys())
            .filter(|n| kit.row(n).is_none() && n.starts_with(prefix))
            .collect();
        for name in consumer {
            if let Some((v, _, _)) = layered(kit, name, Shape::Scalar)? {
                out.push((name.clone(), v.wire()));
            }
        }
    }
    Ok(out)
}

fn declared_family(kit: &'static Kit, fam: &'static Family) -> Result<Vec<(String, String)>, String> {
    let mut members: BTreeMap<String, String> = BTreeMap::new();
    if let Some(derive) = fam.derive {
        for (suffix, value) in derive(&input)? {
            members.insert(format!("{}{}", fam.prefix, suffix), value);
        }
    }
    let l = layers(kit);
    let l = l.as_ref().as_ref().map_err(String::clone)?;
    for layer in [&l.tracked, &l.local] {
        for (name, held) in layer {
            if let (Some(f), Held::Value(Value::Scalar(v))) = (kit.family_of(name), held) {
                if f.prefix == fam.prefix {
                    members.insert(name.clone(), v.clone());
                }
            }
        }
    }
    for (name, v) in std::env::vars() {
        if kit.family_of(&name).is_some_and(|f| f.prefix == fam.prefix) {
            members.insert(name, v);
        }
    }
    Ok(members.into_iter().collect())
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the one function every knob read resolves through, in the
// serialization every reader parses: a locator from the environment or its default, any other name
// from its owning kit, and a name no static kit owns is refused
pub fn wire(name: &str) -> Result<String, String> {
    if let Some((v, _)) = locator(name) {
        return Ok(v.wire());
    }
    resolve(name).map(|(v, _)| v.wire())
}

// spec: gate-sdk/SPEC.md §The non-gate arm — one line per element in the roster's grammar, and one
// line with an empty third field for an empty collection
fn render(out: &mut String, row: &'static Row, v: Value) {
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

// spec: gate-sdk/SPEC.md §The non-gate arm — `--emit knob-roster`: every statically owned knob with
// its shape and default, the default rendered with every sibling at its own default and a locator
// input as `${NAME}`, because a locator's value is the invoking environment's
pub fn roster() -> Result<String, String> {
    fn defaults_only(name: &str) -> Result<(Value, Origin), String> {
        if locator(name).is_some() {
            return Ok((Value::Scalar(format!("${{{}}}", name)), Origin::Placeholder));
        }
        let (_, row) = static_row(name)?;
        Ok((default_value(row, &defaults_only)?, Origin::Default))
    }
    let mut out = String::new();
    for kit in STATIC_KITS {
        for row in kit.rows {
            let (v, _) = defaults_only(row.name)?;
            render(&mut out, row, v);
        }
    }
    Ok(out)
}

pub fn emit(_args: &[String]) -> Result<String, String> {
    roster()
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `--emit knob-values`: the resolved value of each named
// static knob in the invoking tree, in the roster's grammar; its declared set is its argv's closure
pub const ARGV_STATIC_KNOBS: &str = "@argv-static-knobs";

pub fn values(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("needs at least one static knob name — usage: --emit knob-values <NAME>...".to_string());
    }
    let mut out = String::new();
    for name in args {
        let (_, row) = static_row(name)?;
        let (v, _) = resolve(name)?;
        render(&mut out, row, v);
    }
    Ok(out)
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
            for (n, _) in k.retired {
                env.remove(n);
            }
        }
        reset(env);
    }

    const SDK_INPUTS: &[&str] = &["GATE_SDK_WORKFLOW_DIR", "GATE_SDK_TMP_DIR", "GATE_SDK_QUEUE_FILE"];

    fn pin_inputs(env: &knobenv::KnobEnv, dir: &str) {
        for n in SDK_INPUTS {
            env.set(n, dir);
        }
    }

    fn unpin(env: &knobenv::KnobEnv) {
        for n in SDK_INPUTS {
            env.remove(n);
        }
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
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap(), "*/gate-tests/*\t*docs/posts/*");
        s.write("site-config.knobs", "SITE_KIT_EXEMPT_PATHS[] = only one\n");
        reset(&env);
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap(), "only one");
        s.write("site-config.knobs", "SITE_KIT_EXEMPT_PATHS =\n");
        reset(&env);
        assert_eq!(wire("SITE_KIT_EXEMPT_PATHS").unwrap(), "");
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
        assert_eq!(wire("SITE_KIT_CNAME").unwrap(), "PINNED");
        clean(&env, &s.dir());
        restore(&env);
    }

    static KEYED_KIT: Kit = Kit {
        root: "probe-kit",
        rows: &[Row::keyed("PROBE_KIT_MAP", &[])],
        validate: None,
        open_family: false,
        families: &[],
        retired: &[],
        env_only: &[],
    };

    #[test]
    fn a_keyed_knob_takes_pairs_sorted_and_refuses_a_repeated_key_or_an_element() {
        let got = layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[b] = 2 = two\nPROBE_KIT_MAP[a] = 1\n").expect("parses");
        assert_eq!(materialize(&got["PROBE_KIT_MAP"]).unwrap().0.wire(), "a=1\tb=2 = two");
        assert!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[a] = 1\nPROBE_KIT_MAP[a] = 2\n").unwrap_err().starts_with("f:2: "));
        assert!(layer(&KEYED_KIT, "f", "PROBE_KIT_MAP[] = 1\n").unwrap_err().contains("is declared keyed"));
        assert_eq!(materialize(&layer(&KEYED_KIT, "f", "PROBE_KIT_MAP =\n").expect("empties")["PROBE_KIT_MAP"]).unwrap().0.wire(), "");
    }

    // spec: gate-sdk/SPEC.md §The knob file — a scratch tree holding one kit root with a fixture suite
    // and its checks, named by the kit-root override the family derivation reads
    fn fixture_tree(env: &knobenv::KnobEnv, s: &Scratch) -> String {
        let kit = s.0.join("probe-kit");
        std::fs::create_dir_all(kit.join("gate-tests")).expect("tests dir");
        std::fs::create_dir_all(kit.join("checks")).expect("checks dir");
        pin_inputs(env, &s.dir());
        let here = crate::walk::cwd().expect("the test has a working directory");
        let rel = crate::walk::relative_to(&here, &kit.display().to_string());
        env.set("GATE_SDK_KIT_DIRS", &rel);
        env.set("GATE_SDK_ROOT", "sdk");
        reset(env);
        rel
    }

    fn unfixture(env: &knobenv::KnobEnv) {
        unpin(env);
        env.remove("GATE_SDK_KIT_DIRS");
        env.remove("GATE_SDK_ROOT");
        reset(env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — a declared family's members: derived, replaced by a file
    // member of the same name, outranked by the environment; a declared row the prefix spells is no
    // member, and an indexed line under the prefix is refused
    #[test]
    fn a_declared_family_resolves_its_members_and_refuses_the_rest() {
        let env = knobenv::lock();
        let s = Scratch::new("family");
        clean(&env, &s.dir());
        let kit = fixture_tree(&env, &s);
        env.remove("EVIDENCE_KIT_RUN_probe_kit");
        env.remove("EVIDENCE_KIT_RUN_demo");
        let get = |n: &str| {
            family("EVIDENCE_KIT_RUN_").expect("the family reads").into_iter().find(|(k, _)| k == n).map(|(_, v)| v)
        };
        assert_eq!(
            get("EVIDENCE_KIT_RUN_probe_kit").as_deref(),
            Some(format!("bash sdk/bin/run-gates.sh --run-gate-tests {0}/gate-tests {0}/checks", kit).as_str())
        );
        assert_eq!(get("EVIDENCE_KIT_RUN_ID"), None, "a declared row is never a member");
        s.write("evidence-config.knobs", "EVIDENCE_KIT_RUN_demo = from-file\nEVIDENCE_KIT_RUN_probe_kit = replaced\n");
        reset(&env);
        assert_eq!(get("EVIDENCE_KIT_RUN_demo").as_deref(), Some("from-file"));
        assert_eq!(get("EVIDENCE_KIT_RUN_probe_kit").as_deref(), Some("replaced"));
        env.set("EVIDENCE_KIT_RUN_demo", "from-env");
        assert_eq!(get("EVIDENCE_KIT_RUN_demo").as_deref(), Some("from-env"));
        env.remove("EVIDENCE_KIT_RUN_demo");
        assert_eq!(
            walk_suffixes("EVIDENCE_KIT_PARSER_"),
            Vec::<String>::new(),
            "a family with no derivation and no member is empty"
        );
        for (body, want) in [
            ("EVIDENCE_KIT_RUN_demo[] = x\n", "a member is a scalar"),
            ("EVIDENCE_KIT_RUN_demo[k] = x\n", "a member is a scalar"),
            ("EVIDENCE_KIT_RUN_ = x\n", "not a evidence-kit knob"),
            ("EVIDENCE_KIT_RUN_demo = a\nEVIDENCE_KIT_RUN_demo = b\n", "given twice"),
        ] {
            s.write("evidence-config.knobs", body);
            reset(&env);
            assert!(family("EVIDENCE_KIT_RUN_").unwrap_err().contains(want), "{:?}", body);
        }
        unfixture(&env);
        clean(&env, &s.dir());
        restore(&env);
    }

    fn walk_suffixes(prefix: &str) -> Vec<String> {
        family(prefix).expect("the family reads").into_iter().map(|(k, _)| k).collect()
    }

    // spec: gate-sdk/SPEC.md §The knob file — a reference to a row whose default derives from the kit
    // roots splices its resolved elements, from any referencing row
    #[test]
    fn a_reference_to_a_derived_row_splices_its_resolved_elements() {
        let env = knobenv::lock();
        let s = Scratch::new("referents");
        clean(&env, &s.dir());
        fixture_tree(&env, &s);
        s.write(
            "evidence-config.knobs",
            "EVIDENCE_KIT_SUITES[] = gates\nEVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES\nEVIDENCE_KIT_SUITES[] = tail\n",
        );
        reset(&env);
        assert_eq!(wire("EVIDENCE_KIT_SUITES").unwrap(), "gates\tprobe_kit\ttail");
        s.write("evidence-config.knobs", "EVIDENCE_KIT_PERMANENT_SLUGS[] <- EVIDENCE_KIT_FIXTURE_SUITES\n");
        reset(&env);
        assert_eq!(wire("EVIDENCE_KIT_PERMANENT_SLUGS").unwrap(), "probe_kit");
        unfixture(&env);
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — the validator reads a spliced value whole, so the elements
    // a file writes beside a reference are checked with the spliced ones
    #[test]
    fn the_validator_checks_the_elements_written_beside_a_splice() {
        let env = knobenv::lock();
        let s = Scratch::new("referent-validate");
        clean(&env, &s.dir());
        pin_inputs(&env, &s.dir());
        s.write(
            "evidence-config.knobs",
            "EVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES\nEVIDENCE_KIT_SUITES[] = bad-name\n",
        );
        reset(&env);
        let e = resolve("EVIDENCE_KIT_RUNNER_DOC").unwrap_err();
        assert!(e.contains("malformed evidence config") && e.contains("'bad-name'"), "{}", e);
        s.write("evidence-config.knobs", "EVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES\n");
        reset(&env);
        assert_eq!(resolve("EVIDENCE_KIT_RUNNER_DOC").unwrap().0, Value::Scalar("README.md".into()));
        unpin(&env);
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a name no static kit owns is refused, an undeclared name under
    // a kit's prefix is that kit's refusal, and a locator reads the environment or its default
    #[test]
    fn an_unowned_name_is_refused_and_a_locator_reads_the_environment() {
        let env = knobenv::lock();
        assert!(wire(&format!("{}NOT_A_KNOB", site_kit::KIT.prefix())).unwrap_err().contains("not a site-kit knob"));
        assert!(wire("PROBE_ROUTED").unwrap_err().contains("not a statically owned knob"));
        env.set("GATE_SDK_ROOT", "vendored/gate-sdk");
        assert_eq!(wire("GATE_SDK_ROOT").unwrap(), "vendored/gate-sdk");
        env.remove("GATE_SDK_ROOT");
        assert_eq!(wire("GATE_SDK_ROOT").unwrap(), crate::walk::SDK_ROOT_DEFAULT);
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

    // spec: gate-sdk/SPEC.md §The knob file — an environment-only name is refused in a file with the
    // remedy, and a retired name exported in the environment is refused naming its replacement
    #[test]
    fn a_locator_in_a_file_and_a_retired_gate_sdk_name_are_refused() {
        let env = knobenv::lock();
        let s = Scratch::new("env-only");
        clean(&env, &s.dir());
        for name in gate_sdk::KIT.env_only {
            let p = s.write("gate-sdk-config.knobs", &format!("{} = x\n", name));
            reset(&env);
            let e = resolve("GATE_SDK_TMP_DIR").unwrap_err();
            assert!(e.starts_with(&format!("{}:1: ", p)) && e.contains("set it in the environment"), "{}", e);
        }
        std::fs::remove_file(s.0.join("gate-sdk-config.knobs")).unwrap();
        env.set("GATE_SDK_GRAPH_THEME", "old.sh");
        reset(&env);
        assert!(resolve("GATE_SDK_TMP_DIR").unwrap_err().contains("set GATE_SDK_GRAPH_THEME_DIR instead"));
        env.remove("GATE_SDK_GRAPH_THEME");
        s.write("gate-sdk-config.sh", "GATE_SDK_TMP_DIR=x\n");
        reset(&env);
        assert!(resolve("GATE_SDK_TMP_DIR").unwrap_err().contains("gate-sdk-config.knobs"));
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — a row flagged `empty_takes_default` answers its default for
    // an empty value from the environment or a file, and an unflagged row keeps the empty value
    #[test]
    fn an_empty_value_takes_the_default_exactly_on_a_flagged_row() {
        let env = knobenv::lock();
        let s = Scratch::new("empty-default");
        clean(&env, &s.dir());
        let flagged: Vec<&str> = gate_sdk::KIT.rows.iter().filter(|r| r.empty_takes_default).map(|r| r.name).collect();
        assert!(!flagged.is_empty(), "no flagged row to hold");
        for row in gate_sdk::KIT.rows.iter().filter(|r| r.shape == Shape::Scalar) {
            let default = default_value(row, &input).expect("the default renders");
            env.set(row.name, "");
            reset(&env);
            let (got, _) = resolve(row.name).expect("an empty scalar resolves");
            if row.empty_takes_default {
                assert_eq!(got, default, "{} is flagged and did not take its default", row.name);
            } else {
                assert_eq!(got, Value::Scalar(String::new()), "{} is unflagged and dropped its empty value", row.name);
            }
            env.remove(row.name);
        }
        s.write("gate-sdk-config.knobs", "GATE_SDK_GH_HOST =\nGATE_SDK_QUEUE_FILE =\n");
        reset(&env);
        assert_eq!(resolve("GATE_SDK_GH_HOST").unwrap(), (Value::Scalar("github.com".into()), Origin::Default));
        assert_eq!(resolve("GATE_SDK_QUEUE_FILE").unwrap(), (Value::Scalar(String::new()), Origin::Tracked));
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §run-gate-tests — the harness pin is an environment value, so a case's
    // own knob file naming the scratch dir cannot redirect a member's writes into the corpus
    #[test]
    fn a_case_knob_file_does_not_outrank_the_harness_pin() {
        let env = knobenv::lock();
        let s = Scratch::new("harness-pin");
        clean(&env, &s.dir());
        s.write("gate-sdk-config.knobs", "GATE_SDK_TMP_DIR = inside-the-case\n");
        s.write("gate-sdk-config.local.knobs", "GATE_SDK_TMP_DIR = inside-the-overlay\n");
        env.set("GATE_SDK_TMP_DIR", "/pinned/scratch");
        reset(&env);
        assert_eq!(
            resolve("GATE_SDK_TMP_DIR").unwrap(),
            (Value::Scalar("/pinned/scratch".into()), Origin::Env)
        );
        env.remove("GATE_SDK_TMP_DIR");
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — each derived row's declared inputs are held to what its
    // derivation asks, under a recording resolver that answers a locator with its placeholder
    #[test]
    fn every_derived_row_reads_only_its_declared_inputs() {
        let mut derived = 0usize;
        for kit in STATIC_KITS {
            for row in kit.rows {
                let Default::Derived(f) = row.default else {
                    assert!(row.inputs.is_empty(), "{} declares inputs but derives nothing", row.name);
                    continue;
                };
                derived += 1;
                let asked = std::cell::RefCell::new(Vec::<String>::new());
                let record = |n: &str| -> Result<(Value, Origin), String> {
                    asked.borrow_mut().push(n.to_string());
                    let (_, r) = match static_row(n) {
                        Ok(v) => v,
                        Err(_) => return Ok((Value::Scalar("in".into()), Origin::Placeholder)),
                    };
                    Ok((default_value(r, &|_| Ok((Value::Scalar("in".into()), Origin::Placeholder)))?, Origin::Default))
                };
                f(&record).unwrap_or_else(|e| panic!("{}: {}", row.name, e));
                for n in asked.borrow().iter() {
                    assert!(row.inputs.contains(&n.as_str()), "{} reads {}, which its row does not declare", row.name, n);
                }
            }
            for fam in kit.families {
                let Some(f) = fam.derive else {
                    assert!(fam.inputs.is_empty(), "{} declares inputs but derives nothing", fam.prefix);
                    continue;
                };
                derived += 1;
                let asked = std::cell::RefCell::new(Vec::<String>::new());
                let record = |n: &str| -> Result<(Value, Origin), String> {
                    asked.borrow_mut().push(n.to_string());
                    Ok((Value::Scalar("in".into()), Origin::Placeholder))
                };
                f(&record).unwrap_or_else(|e| panic!("{}: {}", fam.prefix, e));
                for n in asked.borrow().iter() {
                    assert!(fam.inputs.contains(&n.as_str()), "{} reads {}, which it does not declare", fam.prefix, n);
                }
            }
        }
        assert!(derived > 0, "no derived row to hold");
    }

    // spec: gate-sdk/SPEC.md §The knob file — every kit default passes its own validator, so a consumer
    // setting nothing is never refused
    #[test]
    fn every_kit_default_passes_its_validator() {
        for kit in STATIC_KITS {
            let Some((_, check)) = kit.validate else {
                continue;
            };
            let mut values = Values::new();
            for row in kit.rows {
                let v = default_value(row, &|n| match static_row(n) {
                    Ok((_, r)) => Ok((default_value(r, &|b| Ok((Value::Scalar(format!("dir-{}", b)), Origin::Placeholder)))?, Origin::Default)),
                    Err(_) => Ok((Value::Scalar(format!("dir-{}", n)), Origin::Placeholder)),
                })
                .expect("a default renders");
                values.insert(row.name, (v, Origin::Default));
            }
            assert_eq!(check(&values), Vec::<String>::new(), "{}'s defaults fail its validator", kit.root);
        }
    }

    // spec: gate-sdk/SPEC.md §The knob file — a malformed file value is refused at the kit's first
    // read with every finding
    #[test]
    fn a_malformed_config_is_refused_at_first_resolution_with_every_finding() {
        let env = knobenv::lock();
        let s = Scratch::new("validate");
        clean(&env, &s.dir());
        assert_eq!(resolve("LIFECYCLE_KIT_FIRST_STAGE").unwrap().0, Value::Scalar("scope".into()));
        s.write("lifecycle-config.knobs", "LIFECYCLE_KIT_FIRST_STAGE = nope\nLIFECYCLE_KIT_SHIM_NGRAM = x\n");
        reset(&env);
        let e = resolve("LIFECYCLE_KIT_STAGES").unwrap_err();
        assert!(e.starts_with("lifecycle-kit: malformed stage-machine config"), "{}", e);
        assert!(e.contains("'nope' is not in LIFECYCLE_KIT_STAGES") && e.contains("'x' is not a positive integer"), "{}", e);
        clean(&env, &s.dir());
        restore(&env);
    }

    #[test]
    fn knob_values_prints_the_resolved_value_and_refuses_an_unowned_name() {
        let env = knobenv::lock();
        let s = Scratch::new("values");
        clean(&env, &s.dir());
        s.write("queue-config.knobs", "QUEUE_KIT_TRACKS[] = a\nQUEUE_KIT_TRACKS[] = b\nQUEUE_KIT_HORIZONS[] = now\n");
        reset(&env);
        let args = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            values(&args(&["QUEUE_KIT_TRACKS", "QUEUE_KIT_PROSE_SURFACE_GLOBS"])).unwrap(),
            "QUEUE_KIT_TRACKS\tindexed\ta\nQUEUE_KIT_TRACKS\tindexed\tb\nQUEUE_KIT_PROSE_SURFACE_GLOBS\tindexed\t\n"
        );
        assert!(values(&args(&["GATE_PRUNE_DIRS"])).is_err());
        assert!(values(&[format!("{}NOPE", queue_kit::KIT.prefix())]).is_err());
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — the roster renders a sibling at its own default and a
    // locator input as `${NAME}`
    #[test]
    fn the_roster_renders_a_sibling_default_and_a_locator_by_name() {
        let r = roster().expect("renders");
        assert!(r.contains("LIFECYCLE_KIT_STATE_FILE\tscalar\t.workflow/WORKFLOW-STATE.txt\n"), "{}", r);
        assert!(r.contains("LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS\tindexed\tTASK-QUEUE.md\n"), "{}", r);
        assert!(r.contains("GATE_SDK_HOOKS_DIR\tscalar\t${GATE_SDK_GATES_DIR}/git-hooks\n"), "{}", r);
        assert!(r.contains("LIFECYCLE_KIT_PREDECESSOR\tkeyed\talign=scope\n"), "{}", r);
    }

    // spec: gate-sdk/SPEC.md §The knob file — a reference contributes its referent's resolved elements
    // at its own position among the knob's element lines, same-kit or across kits, whatever layer
    // supplied the referent
    #[test]
    fn a_reference_splices_the_referents_resolved_elements_at_its_position() {
        let env = knobenv::lock();
        let s = Scratch::new("splice");
        clean(&env, &s.dir());
        s.write(
            "canon-config.knobs",
            "CANON_KIT_MANIFEST_FILES[] = a.md\nCANON_KIT_MANIFEST_FILES[] = b.md\n\
             CANON_KIT_MEASURED_SURFACE_GLOBS[] = first\n\
             CANON_KIT_MEASURED_SURFACE_GLOBS[] <- CANON_KIT_MANIFEST_FILES\n\
             CANON_KIT_MEASURED_SURFACE_GLOBS[] = last\n\
             CANON_KIT_PROSE_TELL_GLOBS[] <- CANON_KIT_MANIFEST_FILES\n\
             CANON_KIT_PROSE_TELL_GLOBS[] = tail\n",
        );
        reset(&env);
        assert_eq!(wire("CANON_KIT_MEASURED_SURFACE_GLOBS").unwrap(), "first\ta.md\tb.md\tlast");
        assert_eq!(wire("CANON_KIT_PROSE_TELL_GLOBS").unwrap(), "a.md\tb.md\ttail");
        s.write("drift-config.knobs", "DRIFT_KIT_STAGES[] <- LIFECYCLE_KIT_STAGES\n");
        reset(&env);
        assert_eq!(wire("DRIFT_KIT_STAGES").unwrap(), "scope\talign\tbuild\tvalidate\tclose");
        s.write("lifecycle-config.local.knobs", "LIFECYCLE_KIT_STAGES[] = one\nLIFECYCLE_KIT_STAGES[] = two\n");
        reset(&env);
        assert_eq!(wire("DRIFT_KIT_STAGES").unwrap(), "one\ttwo");
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — every reference refusal names the file and line
    #[test]
    fn every_reference_refusal_names_its_file_and_line() {
        let env = knobenv::lock();
        let s = Scratch::new("reference-refusals");
        for (body, want) in [
            ("CANON_KIT_SPEC_NAME[] <- CANON_KIT_MANIFEST_FILES\n", "is declared scalar"),
            ("CANON_KIT_MANIFEST_FILES[] <- CANON_KIT_SPEC_NAME\n", "not a declared indexed"),
            ("CANON_KIT_MANIFEST_FILES[] <- GATE_PRUNE_DIRS\n", "not statically owned"),
            ("CANON_KIT_MANIFEST_FILES[] <- CANON_KIT_MANIFEST_FILES\n", "references itself"),
            ("CANON_KIT_MANIFEST_FILES =\nCANON_KIT_MANIFEST_FILES[] <- CANON_KIT_DUP_SURFACES\n", "given twice"),
        ] {
            clean(&env, &s.dir());
            let p = s.write("canon-config.knobs", &format!("# header\n{}", body));
            reset(&env);
            let e = resolve("CANON_KIT_SPEC_NAME").expect_err(body);
            let lno = body.lines().count() + 1;
            assert!(e.starts_with(&format!("{}:{}: ", p, lno)), "{:?}: {}", body, e);
            assert!(e.contains(want), "{:?}: {}", body, e);
        }
        clean(&env, &s.dir());
        let p = s.write(
            "canon-config.knobs",
            "CANON_KIT_MANIFEST_FILES[] <- CANON_KIT_DUP_SURFACES\nCANON_KIT_MEASURED_SURFACE_GLOBS[] <- CANON_KIT_MANIFEST_FILES\n",
        );
        reset(&env);
        let e = wire("CANON_KIT_SPEC_NAME").unwrap_err();
        assert!(e.starts_with(&format!("{}:2: ", p)) && e.contains("one pass"), "{}", e);
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — an open kit admits an undeclared scalar under its prefix
    // into its family read, with the environment first; an indexed form, a locator and a closed kit's
    // undeclared name stay refused
    #[test]
    fn an_open_family_admits_a_consumer_scalar_and_refuses_the_rest() {
        let env = knobenv::lock();
        let s = Scratch::new("open-family");
        clean(&env, &s.dir());
        pin_inputs(&env, &s.dir());
        s.write("drift-config.knobs", "DRIFT_KIT_SMOKE_CUSTOM = reached\n");
        reset(&env);
        let get = |n: &str| {
            let fam = family("DRIFT_KIT_").expect("the family reads");
            fam.iter().find(|(k, _)| k == n).map(|(_, v)| v.clone())
        };
        assert_eq!(get("DRIFT_KIT_SMOKE_CUSTOM").as_deref(), Some("reached"));
        assert_eq!(get("DRIFT_KIT_DONE_SECTION").as_deref(), Some("Done"));
        assert_eq!(get("DRIFT_KIT_STAGES"), None, "an indexed row is not a family scalar");
        env.set("DRIFT_KIT_SMOKE_CUSTOM", "env");
        assert_eq!(get("DRIFT_KIT_SMOKE_CUSTOM").as_deref(), Some("env"));
        env.remove("DRIFT_KIT_SMOKE_CUSTOM");
        assert!(wire("DRIFT_KIT_SMOKE_CUSTOM").unwrap_err().contains("not a drift-kit knob"));
        for (body, want) in [
            ("DRIFT_KIT_SMOKE_CUSTOM[] = x\n", "not a drift-kit knob"),
            ("DRIFT_KIT_KNOB_FILE = x\n", "not a drift-kit knob"),
            ("DRIFT_KIT_CONFIG_FILE = x\n", "not a drift-kit knob"),
            ("DRIFT_KIT_SMOKE_CUSTOM = a\nDRIFT_KIT_SMOKE_CUSTOM = b\n", "given twice"),
        ] {
            s.write("drift-config.knobs", body);
            reset(&env);
            assert!(family("DRIFT_KIT_").unwrap_err().contains(want), "{:?}", body);
        }
        s.write("site-config.knobs", &format!("{}CUSTOM = x\n", site_kit::KIT.prefix()));
        reset(&env);
        assert!(resolve("SITE_KIT_CNAME").unwrap_err().contains("not a site-kit knob"));
        unpin(&env);
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — a probing derivation renders its first candidate for the
    // roster's placeholder rather than an empty value true of no tree
    #[test]
    fn the_roster_renders_a_probing_default_through_the_placeholder() {
        let r = roster().expect("renders");
        assert!(r.contains("DRIFT_KIT_KPIS_FILE\tscalar\t${GATE_SDK_GATES_DIR}/kpis.list\n"), "{}", r);
        assert!(
            r.contains("CONTEXT_KIT_HOOK_CMD\tindexed\tbash\nCONTEXT_KIT_HOOK_CMD\tindexed\t${GATE_SDK_GATES_DIR}/run-gates.sh\n"),
            "{}",
            r
        );
        assert!(r.contains("DELEGATION_KIT_USAGE_FILE\tscalar\t\n"), "{}", r);
    }

    // spec: gate-sdk/SPEC.md §The knob file — a validator reads origin beside value: a consumer-set
    // existing-file knob naming no file is refused, the same path at its default is not
    #[test]
    fn a_set_but_missing_file_knob_is_refused_by_origin() {
        let env = knobenv::lock();
        let s = Scratch::new("origin");
        clean(&env, &s.dir());
        pin_inputs(&env, &s.dir());
        assert!(resolve("CONTEXT_KIT_BREVITY_FILE").is_ok(), "the default settings path may be absent");
        s.write("context-config.knobs", "CONTEXT_KIT_SETTINGS_FILE = nowhere/settings.json\n");
        reset(&env);
        let e = resolve("CONTEXT_KIT_BREVITY_FILE").unwrap_err();
        assert!(e.contains("CONTEXT_KIT_SETTINGS_FILE not found: nowhere/settings.json"), "{}", e);
        clean(&env, &s.dir());
        pin_inputs(&env, &s.dir());
        assert_eq!(resolve("DRIFT_KIT_KPIS_FILE").unwrap().0, Value::Scalar(String::new()));
        env.set("DRIFT_KIT_KPIS_FILE", "");
        reset(&env);
        assert!(resolve("DRIFT_KIT_DONE_SECTION").is_ok(), "an empty registry is the not-adopted answer");
        env.set("DRIFT_KIT_KPIS_FILE", &format!("{}/absent.list", s.dir()));
        reset(&env);
        assert!(resolve("DRIFT_KIT_DONE_SECTION").unwrap_err().contains("DRIFT_KIT_KPIS_FILE not found"));
        let present = s.write("kpis.list", "kpi-task-split\n");
        env.set("DRIFT_KIT_KPIS_FILE", &present);
        reset(&env);
        assert!(resolve("DRIFT_KIT_DONE_SECTION").is_ok());
        unpin(&env);
        clean(&env, &s.dir());
        restore(&env);
    }

    // spec: gate-sdk/SPEC.md §The knob file — a retired name is refused naming its replacement, from a
    // file line with its file and line, and from the environment at the kit's first resolution
    #[test]
    fn a_retired_name_is_refused_naming_its_replacement() {
        let env = knobenv::lock();
        let s = Scratch::new("retired");
        clean(&env, &s.dir());
        pin_inputs(&env, &s.dir());
        let p = s.write("context-config.knobs", "# h\nCONTEXT_KIT_BREVITY_SECTION = ## X\n");
        reset(&env);
        let e = resolve("CONTEXT_KIT_BREVITY_FILE").unwrap_err();
        assert!(e.starts_with(&format!("{}:2: ", p)) && e.contains("set CONTEXT_KIT_BREVITY_SECTIONS instead"), "{}", e);
        std::fs::remove_file(&p).unwrap();
        env.set("CONTEXT_KIT_BREVITY_SECTION", "## X");
        reset(&env);
        assert!(resolve("CONTEXT_KIT_BREVITY_FILE").unwrap_err().contains("CONTEXT_KIT_BREVITY_SECTION is retired"));
        assert!(wire("CONTEXT_KIT_BREVITY_SECTION").unwrap_err().contains("CONTEXT_KIT_BREVITY_SECTIONS"));
        assert!(static_names().contains(&"CONTEXT_KIT_BREVITY_SECTION".to_string()));
        unpin(&env);
        clean(&env, &s.dir());
        restore(&env);
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
