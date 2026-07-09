# Why Checkwright

## The problem: discipline that does not hold

Software teams keep their conventions in prose — style guides, spec documents,
the comment at the top of the tricky file, the shared understanding that this
list stays in sync with that one. This works because a human developer reads
the prose, remembers it, and feels the friction when a change violates it.

Coding agents break that mechanism. An agent session is stateless: it does not
reliably re-read the convention it violated last week, because it was not there
last week. It restates a spec in a comment and neither notices when the two
diverge. It adds the ninth item to a list and forgets the registry that was
supposed to grow with it. None of this is incompetence — it is the predictable
result of asking a fresh reader, every time, to hold context that was never
written down mechanically. The drift is silent, and silence is the whole
problem: nobody is signalled.

## The remedy: mechanize the decidable, isolate the semantic

Checkwright's premise is a division of labour. Some consistency questions are
mechanically decidable and cheap to check — does this link resolve, does this
command exist, does every kit have a registry row, was this stage actually
entered. Those belong to a machine. What remains is the irreducibly semantic
judgment — is this the right design, does this prose actually explain the thing
— and that is where human (or agent) attention should go, undiluted.

So every cheap, low-false-positive, mechanically-decidable axis becomes a
**gate**: a small program that scans a surface and blocks the commit when it
finds a violation, naming the finding and the fix. The residue — the judgment a
gate cannot make — is left to a reviewer who is freed from also playing linter.

A gate earns its place only when it is cheap, rarely wrong, and guards a real
failure mode. A gate that cries wolf trains its readers to bypass it, so a false
positive is treated as a defect in the gate, not a cost of doing business.

## Why kits

Checkwright is not a monolith. It is a set of kits, each owning one axis of the
problem — the lint framework, the iteration lifecycle, the task queue, the spec
discipline, permission friction, delegation, context economics, drift, and
evidence. A kit is vendored into your repository whole and governs it from the
inside. You adopt the ones that pay for themselves and leave the rest.

The kits were extracted from a working platform's governance layer, and this
repository governs itself with them from day one — the same gates that ship to
you run on the tree that builds them. A methodology that its own authors will
not run is a slide deck; this one is dogfooded or it is nothing.

## Where to go next

- [Install](install.md) — vendor the kits and wire the upgrade contract.
- [The kits](index.md) — one page each, in reading order.
