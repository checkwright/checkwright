# doctrine-kit — the experience-packaging rung

The delivery doctrine the other kits enforce piecemeal — the cross-kit practice
rules a session must hold to work this methodology — becomes one kit-shipped,
customer-deliverable rules file, installed by reference into a consumer's
always-loaded agent file and held there by a gate. The other kits own the
*mechanisms*; doctrine-kit owns the *statements* that name why those mechanisms
exist, at one tier, in one place.

The kit is a new remit rather than a fold into canon-kit: the doctrine spans kit
charges — enforcement-first is gate-sdk-flavoured, load-trigger residency is
context-kit-flavoured, content-tiering is canon-kit-flavoured — and canon-kit's
own out-of-scope boundary holds that a consumer's tier contract is not
canon-kit's to own.

## The doctrine deliverable

`DOCTRINE.md` is the deliverable. It carries each cross-kit rule as its
statement, why it holds under coding-agent work, and a pointer to the kit
mechanism that enforces it; the mechanism and its knob rosters live in the cited
kit SPEC, never restated in the doctrine. The rule *statements* are the
doctrine's to own — a governed surface elsewhere cites a rule by name and links
the doctrine rather than restating it.

The roster carries two registers: methodology-maintenance rules, which govern
how the methodology's own surfaces stay honest and bear on every surface edit,
and engineering-craft rules, which govern how the work built under the
methodology is written and are triggered by test and rename work. The
maintenance register forms the installed digest; the craft register lives behind
the link, load-triggered — the doctrine applying its own load-trigger residency
rule to itself.

The file is referenced in place, never copy-installed. A consumer vendors the
kit and links the vendored path; re-vendoring the kit *is* the doctrine upgrade.
A copied doctrine drifts the moment the source moves and the copy does not; a
linked one cannot. This is point-never-restate applied to the doctrine mechanism
itself.

`DOCTRINE.md` joins the consumer's spec manifest (this repo wires it into
`scripts/canon-config.sh`), so its links and commands resolve under the
canon-kit doc gates like any governed page.

## install-doctrine

`bin/install-doctrine.sh` writes the reference block into the always-loaded
agent file, idempotently. The block is bounded by fixed marker lines
(`<!-- doctrine-kit:begin -->` … `<!-- doctrine-kit:end -->`); a run replaces
the content between the markers when they are present and appends the block when
they are not, so re-running never duplicates and a marker set left in place is
updated where it sits. A begin marker without its end is a malformed target: the
installer refuses (exit 2) rather than guess the block bounds. The agent file
must already exist — the installer edits an always-loaded file, it does not mint
one — so a missing target is exit 2.

The block is the always-loaded shape applied to the doctrine itself: a one-line
digest of the methodology-maintenance rules plus a markdown link to the doctrine
file. The engineering-craft register is not digested — it is load-triggered and
reached through the link, so the always-loaded surface carries only what bears
on every edit. The installer is the single source of the block text, so a manual
insertion for a harness-less consumer copies what the tool would emit; the README
documents that manual path. The installed digest names every methodology rule,
so a fresh consumer is in per-rule lockstep out of the box (installer and gate
agree on the `## Delivery doctrine` heading — that agreement is part of
check-doctrine-registration's contract). Trimming a rule the consumer does not
keep resident stays legal, but rides a declared-trim marker rather than a silent
deletion: the gate asserts name-lockstep modulo declared trims (§check-doctrine-registration
assertion B).

Positional overrides `install-doctrine.sh [agent-file [doctrine-file]]` let a
smoke or a fixture point both paths at a scratch tree without touching consumer
config; unset, they fall to the knob defaults.

## check-doctrine-registration

Invariant, in three assertions: the configured agent file (A) carries a markdown
link to the configured doctrine file and (B, C) holds its methodology-rule digest
in per-rule lockstep with the doctrine. The digest is the surface the
always-loaded-shape rule requires, and a re-vendored `DOCTRINE.md` that adds or
renames a methodology-maintenance rule staling every consumer's digest *by
construction* — on the exact path the kit advertises as its upgrade story — is
the drift-prone-surface-that-must-exist case where a gate is owed (the
enforcement-first weighing). Extending the existing gate rather than adding one
keeps the gate count flat; the coupling is unchanged (the `# graph:` manifest
already couples the agent file and the doctrine file).

- **Assertion A (link).** The gate greps the agent file for a
  `](<doctrine-file>` link token; absent, it is a finding with the install
  remedy. It asserts link *presence* only — that a session loading the agent
  file is pointed at the doctrine, not that the link was followed or the
  doctrine read (the honest limit: a link is not a read) — and leaves
  link-target resolution to the consumer's doc gates (canon-kit's
  `check-md-refs` over the manifest).
- **Assertion B (doctrine → digest).** Every rule name under the doctrine's
  `## Methodology-maintenance rules` section — the bold text of each numbered
  rule, trailing period dropped (e.g. `Content-tiering / SSOT`) — appears as a
  bold digest lead-in (`- **<name>**`) in the agent file's digest section.
  Engineering-craft rules are exempt: they live behind the link by the
  doctrine's own two-register design, so the gate scans only the
  methodology section for the required set. The consumer's right to reject a
  rule survives as a *declared* trim: a
  `<!-- doctrine-digest-trim: <rule name> — <reason> -->` line inside the
  digest section satisfies assertion B for that rule; a silent omission stays
  red. Declared-not-silent is the reconciliation — the re-vendor moment
  surfaces every added or renamed rule, and the consumer's decision (adopt the
  bullet or trim it with cause) is recorded beside the digest it governs.
- **Assertion C (digest → doctrine).** Every bold bullet lead-in in the digest
  section matches a methodology-maintenance rule name — a digest line with no
  owning rule is a rule stated nowhere the doctrine governs.

Section resolution fails closed. The digest section is the agent-file heading
named by `DOCTRINE_KIT_DIGEST_SECTION` (Layout and configuration); a configured
heading matching nothing exits 2 — a renamed digest section must not disarm the
gate into passing an empty set. The doctrine-side heading
(`Methodology-maintenance rules`) is kit mechanism, not config: the kit ships
`DOCTRINE.md`, so it owns that name, and its absence is likewise exit 2 (the
gate cannot certify the digest against an unreadable rule set). A missing agent
or doctrine file is fail-closed for the same reason, as is a grep or awk that
errors rather than simply not-matching.

The gate satisfies the four gate-sdk contracts (gate-sdk/SPEC.md §The gate
model): the single `DOCTRINE-REGISTRATION: clean` success line and a `help:`
remedy on each finding path (output); exit 2 on an unreadable file, an
unresolved section, or an errored capture (fail-closed); a `good/`+`bad/`
fixture pair under `gate-tests/` — the pair carries the lockstep-clean and the
digest-missing-a-rule cases, and a sibling `*.test.sh` drives the extra-line,
declared-trim, link-absent, and three fail-closed cases the one-pair harness
cannot hold (fixture-pair); and registration in this repo's `gates.list` where
its own always-loaded file is the scan target (self-lint). Positional form
`check-doctrine-registration.sh [agent-file [doctrine-file]]` lets the fixtures
point at a synthetic agent and doctrine file.

## lib/doctrine.sh

The sourced config loader shared by the installer and the gate: it loads
`DOCTRINE_KIT_CONFIG_FILE` (or the gates-dir `doctrine-config.sh` when that env
is unset), then a gitignored `.local.sh` overlay beside it, then fills each
knob's default — so the installer and the gate read one resolved configuration.
It carries no gate logic: structure stays in the check, values in config,
defaults here.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `doctrine-kit/`); its
gate is registered in the consumer's `gates.list` by name and resolves through
gate-sdk's multi-kit path. Config follows the kit pattern: an optional
`doctrine-config.sh` in the gates dir (or a `DOCTRINE_KIT_CONFIG_FILE`
elsewhere) overrides any knob; defaults fill what the consumer left unset. Knobs:

- `DOCTRINE_KIT_AGENT_FILE` — the always-loaded file the installer edits and the
  gate scans, default `CLAUDE.md`.
- `DOCTRINE_KIT_DOCTRINE_FILE` — the link target the installer writes and the
  gate asserts, default `doctrine-kit/DOCTRINE.md`.
- `DOCTRINE_KIT_DIGEST_SECTION` — the agent-file heading whose bullet list the
  gate reads as the methodology-rule digest (assertions B and C), default
  `## Delivery doctrine`. The default is `install-doctrine.sh`'s installed block
  heading, so a zero-config consumer that installed via the tool is green out of
  the box; a consumer that renamed the heading repoints this knob (a rename that
  leaves it stale exits 2 rather than passing an empty set).
- `DOCTRINE_KIT_CONFIG_FILE` — the loader override; when set it is sourced if it
  exists, and a `.local.sh` sibling sources last for private overlay values.

The defaults are this repo's own layout, so this repo runs the kit on itself
with no config file: `CLAUDE.md` is the always-loaded agent file, and its
`## Delivery doctrine` reference block links `doctrine-kit/DOCTRINE.md`.

## Out of scope

Folding the doctrine into canon-kit — ruled out above. Copy-install of the
doctrine file — a copied doctrine drifts; the reference is the mechanism.
Per-rule enable/disable knobs — the doctrine is one document, and a consumer
that rejects a rule declares a trim marker beside its digest rather than toggling
a knob. A standing consultation step in place of packaging — the always-loaded
anti-pattern this load-triggered kit exists to replace. The kit holds no opinion
on *which* methodology rules a consumer keeps resident versus trims with cause;
it ships the statements and the wiring, and the consumer's always-loaded budget
rules the digest — but a trim is declared, not silent, so the gate holds the
resident set and the doctrine in name-lockstep modulo those declarations.
