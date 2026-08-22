# Security policy

## Reporting a vulnerability

Report privately through GitHub, never in a public issue: open the repository's
**Security** tab and use **Report a vulnerability**. That opens a private
advisory thread visible only to you and the maintainers.

There is no security email address on purpose. This repo keeps maintainer
identities out of tracked files, and GitHub's private reporting gives the same
confidentiality without publishing one.

Please include the vendored version (the tag or commit you copied), the kit and
gate involved, and what an attacker gains. A reproduction a gate or a fixture
can run is worth more than a description of one.

## What to expect

A solo maintainer runs this project, so what follows is an aspiration rather
than a service level. Expect an acknowledgement within about a week and a first
assessment within about two. A confirmed issue is fixed in the next release, and
the advisory is published once the fix ships, crediting you unless you ask
otherwise. If a report goes unanswered past those windows, it has been missed
rather than declined — say so on the same private thread.

## Supported versions

While the version line is pre-1.0, only the newest `v*` tag is supported. There
are no backports: a fix lands on `master` and ships in the next tag, and
upgrading to it is the remedy. Releases otherwise defer by default, but a
security or supply-chain fix is a release trigger and does not wait out the
ordinary cadence — the trigger set is `.claude/commands/close.md`'s release
policy. The two-phase upgrade contract
([docs/install.md](docs/install.md)) is what makes that a mechanical move rather
than a migration.

## Threat boundary

Checkwright is vendored, not installed. Understanding what that does and does
not protect matters more than any of the above.

<!-- measured: gate-substrates=native+shell -->
**A vendored kit is code you run with your own privileges.** You copy the kit
directories into your repository and commit them, and from then on your git
hooks and your CI execute it as you — the shell that ships in the tree and the
prebuilt binary the ported gates dispatch to. Nothing is fetched at gate time and
no dependency channel exists, so the code that runs is the code sitting in your
tree. That also means adopting a kit is granting it whatever your hooks and CI
already hold.

**Vendoring is a copy you review before you run it.** Reviewing the diff at
adoption and again at each upgrade is the trust step for everything that ships
as source, and no mechanism in any kit substitutes for it. What a vendored gate
puts in front of you is ruled and bounded at
[gate-sdk/SPEC.md](gate-sdk/SPEC.md) §Consumer payload: a gate whose
implementation is compiled withholds that implementation, and what stands in
for reading it is a published per-target digest verified before the artifact is
written. This is the reason the distribution model is a committed copy rather
than a package pull: the reviewable artifact is the point.

**A gate is a consistency check on a tree, not a security boundary.** Every gate
answers one question — does this tree agree with itself — and it answers it for
a contributor who is trying to get the change right. A committer who already
holds write access and wants a check to pass can edit the gate, drop its
registration, or change the workflow that runs it. Gates raise the cost of
accidental drift and of an agent taking a shortcut. They are not an authorization
layer, and a threat model that treats them as one is wrong about what they do.

The honest limit on the outermost enforcement tier is stated where that tier is
specified, and is not restated here: see [gate-sdk/SPEC.md](gate-sdk/SPEC.md)
§Enforcement tiers for what consumer-owned CI does and does not stop, and for
the rung that would close the remainder.
