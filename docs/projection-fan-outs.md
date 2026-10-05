---
title: Projection fan-outs
---
<!-- door-contributor: maintainer governance, off-nav by design and reached only by citation; every door on the page is a regeneration command for a generated projection -->

# Projection fan-outs

This page holds the edits whose wake reaches several generated surfaces, and the rule for which derived surface earns a roster row.

## The KPI-roster fan-out

A `scripts/kpis.list` edit is the widest single trigger on this page: adding or removing one KPI moves **three** byte-gated surfaces, not the one an amendment naturally names. They are the on-site SPEC mirror (the owning kit's SPEC documents the KPI); `docs/enforcement.md` (the KPI joins the class registry); and `docs/value.md`'s rollup block (its per-kit Advisory count is derived from the enforcement map). Each of the three gates names its own regen command on a red, so recovery is mechanical once the fan-out is known. Knowing it in advance is the part nothing else states. `docs/footprint.md` is **not** in this fan-out, though the shape of the list invites the guess: the footprint measures no script, so a KPI's bytes never reach it; the enforcement map's row on the [roster page](generated-projections.md#generated-projections-and-their-freshness-gates) says why.

## The new-tag-class-member fan-out

Adding a member to `check-tag-lead-line`'s class table is a one-line edit with a three-surface wake, and it is invisible from the edit: `--emit enum-sets` derives the tag members, so `check-prose-enum` reds every hand-written prose enumeration of the queue's tag set — `README.md` and `docs/queue-kit/index.md`, neither of them generated, each repaired by hand. The new-gate fan-out below is the one this page already rostered; this is the other one, and nothing derived it for the author.

## The new-gate fan-out

The other wide trigger, and the one with no single owner elsewhere. `gate-sdk/SPEC.md`'s kit-landing checklist covers the kit-side obligations (SPEC section, `good/`+`bad/` fixture pair, the README's `<!-- gate-roster:begin -->` block, `smoke/`, registration in `scripts/gates.list`) and is silent on the projections a new gate stales, because a kit may not name a consumer's docs surfaces. Assembled here so the next author reads the list instead of discovering it one red gate at a time:

- the on-site SPEC mirror (`bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`);
- `docs/enforcement.md`, which the gate joins as a class-registry member (`bash gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md`);
- `docs/value.md`'s rollup block, derived from that map (`bash gate-sdk/bin/run-gates.sh --emit value-rollup --write`);
- `docs/check-graph.html` (`bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`);
- the owning kit's `smoke/install.sh` registry heredoc, registering the gate or declaring it `# unregistered:` (`check-gate-substrate-parity` assertion J); [`gate-sdk/SPEC.md` §Consumer smoke](gate-sdk/SPEC.md#consumer-smoke) rules when an omission also owes `# smoke-unregistered:`;
- `.workflow/surface-ceiling.txt`, since `check-surface-ratchet` governs the `docs/` pages and `docs/enforcement.md` grows by the new row (`bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`);
- for the first `tier=commit-msg` gate, the generated hooks, which gain a commit-msg hook (`bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`);
- for a freshness gate, a `# projection: <outputs>` header line, which makes its keyed row on this roster mandatory (`check-projection-roster` holds both; [gate-sdk/SPEC.md §check-projection-roster](gate-sdk/SPEC.md#check-projection-roster) owns the key).

`docs/footprint.md` is absent for the reason the enforcement map's row gives: a gate is a script or a crate module and the footprint measures neither. A prose-only SPEC edit reds the on-site mirror alone. **One of these regenerations is staging-ordered.** The gate binary derives through `git ls-files`, so a unit adding a file stages first and builds second. The hazard and its route are this page's closing paragraphs; a new-gate unit adds files by definition.

## Which derived surface earns a row

**A derived surface earns a roster row only when it has a reader who cannot run the emitter** — a public page, a file a fresh clone needs before its tooling works. Otherwise deriving on demand satisfies derivation-first, and a committed copy of a high-churn source's derivation buys a per-commit regeneration tax for nobody. So **a tool with no stored projection has nothing to hold fresh** and stays off this roster: queue-kit's `queue-index` and `queue-edges` arms are the standing instances, the latter with its refusal reasoned in its own contract ([queue-kit/SPEC.md §The queue-edges arm](queue-kit/SPEC.md#the-queue-edges-arm)). Their absence is a ruling. Ask of a new derived surface who reads it, not whether it could be generated. **Both rulings survived a port onto the binary on that stored-projection ground alone**, and the shell-consumer half of each stopped being true at its own port, when the consumer became a session reaching a compiled arm through the `--emit` front-end.

**The compiled gate binary is the third standing instance, and it fails the admission test in both directions.** It is never committed. With `native/target/` gitignored there is no tracked copy for a freshness gate to byte-compare, and every reader of it in this repo can run the emitter, which is `cargo build`. A consumer is not a counter-example: a consumer never receives the crate source and never builds, and the artifact they do receive is held by a published digest verified before it is written ([gate-sdk/SPEC.md §Consumer payload](gate-sdk/SPEC.md#consumer-payload)), a different guarantee. The binary owes build currency, discharged by an oracle rather than a roster row: `check-gate-binary-fresh` ([gate-sdk/SPEC.md §check-gate-binary-fresh](gate-sdk/SPEC.md#check-gate-binary-fresh)) compares the binary's baked source stamp against the crate's tracked source whenever a `.gate` descriptor makes it load-bearing. **It carries a staging-order hazard.** The stamp is computed over *tracked* crate source, so a unit adding a crate file builds after `git add`, never before, or the binary is stamped against a source set the gate does not hash; the rule is the owner's (gate-sdk/SPEC.md §check-gate-binary-fresh), named here because the reflex is to read the hazard off the artifact's own freshness rule, which is not where it lives.

**The generated hooks carry none, and that makes the binary's the whole set.** Each hook bakes the resolved binary path alone, and the members a commit runs are selected from the manifests at commit time ([gate-sdk/SPEC.md §git-hook](gate-sdk/SPEC.md#git-hook)), so no hook derivation lags a `git add`. Every other roster row reads the worktree and is order-free.
