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
documents that manual path. What the digest *says* is the consumer's to trim — a consumer that
rejects a rule edits its own digest — because the gate asserts only the link,
never the block body.

Positional overrides `install-doctrine.sh [agent-file [doctrine-file]]` let a
smoke or a fixture point both paths at a scratch tree without touching consumer
config; unset, they fall to the knob defaults.

## check-doctrine-registration

Invariant: the configured agent file carries a markdown link to the configured
doctrine file. The gate greps the agent file for a `](<doctrine-file>` link
token; absent, it is a finding with the install remedy. It asserts link
*presence* only — that a session loading the agent file is pointed at the
doctrine — and leaves link-target resolution to the consumer's doc gates
(canon-kit's `check-md-refs` over the manifest). A missing agent file is
fail-closed (exit 2): the gate cannot certify a file it cannot read. A grep that
errors rather than simply not-matching is likewise fail-closed.

The gate satisfies the four gate-sdk contracts (gate-sdk/SPEC.md §The gate
model): the single `DOCTRINE-REGISTRATION: clean` success line and a `help:`
remedy on the finding path (output); exit 2 on the unreadable agent file and the
errored grep (fail-closed); a `good/`+`bad/` fixture pair under `gate-tests/`
(fixture-pair); and registration in this repo's `gates.list` where its own
always-loaded file is the scan target (self-lint). Positional form
`check-doctrine-registration.sh [agent-file [doctrine-file]]` lets the fixtures
point at a synthetic agent file.

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
- `DOCTRINE_KIT_CONFIG_FILE` — the loader override; when set it is sourced if it
  exists, and a `.local.sh` sibling sources last for private overlay values.

The defaults are this repo's own layout, so this repo runs the kit on itself
with no config file: `CLAUDE.md` is the always-loaded agent file, and its
`## Delivery doctrine` reference block links `doctrine-kit/DOCTRINE.md`.

## Out of scope

Folding the doctrine into canon-kit — ruled out above. Copy-install of the
doctrine file — a copied doctrine drifts; the reference is the mechanism.
Per-rule enable/disable knobs — the doctrine is one document, a consumer that
rejects a rule trims its digest, and the gate asserts only the link, not the
block body. A standing consultation step in place of packaging — the
always-loaded anti-pattern this load-triggered kit exists to replace. The kit
holds no opinion on *which* rules a consumer keeps resident versus behind the
link; it ships the statements and the wiring, and the consumer's always-loaded
budget rules the digest.
