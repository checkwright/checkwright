---
title: Home
nav_order: 1
---

# Checkwright

**Verification for coding-agent delivery.** Checkwright is the verification layer under agent orchestration: spec drift, skipped stages, and unsupported *done* claims become failing checks before a merge, instead of review findings after one.

It is for the maintainer of a repository coding agents write most of, who has to answer at merge time whether the work is actually done and cannot answer it by reading every diff.

**It complements the workflow you already run.** Keep your spec process, your prompts, your harness. Add Checkwright where a claim has to be mechanically proven rather than asserted: the instructions shape, the gates enforce. Why that split is the whole design is the layer model on [Where Checkwright sits](positioning.md).

## Try it first

`demo` runs the whole arc in a scratch repository of its own and removes it, without touching yours; [Install](install.md#install) says what each act does. It installs the `full` profile, which needs bash 4.3 or later: on stock macOS run the [Homebrew bash step](install.md#macos-and-linux) first, and on Windows the [Git for Windows step](install.md#windows).

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- demo                  # macOS and Linux
```

```powershell
& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) demo      # Windows, in PowerShell
```

```sh
npx checkwright demo                                                           # with Node
```

## Install

From the root of a clean git repository, the same three routes without `demo` install the kits as one commit, the first two from the Release tarball:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh                             # macOS and Linux
```

```powershell
irm https://checkwright.dev/install.ps1 | iex                                  # Windows, in PowerShell
```

```sh
npx checkwright init                                                           # with Node
```

Profiles, the other verbs and how to pass them arguments: [Install](install.md#install).

## What that buys you

**Before.** A session finishes a task, moves it to Done and stamps the iteration validated and closed; the evidence is the session's own say-so. The commit goes in green, and the next stateless session reads it as ground truth.

**After.** The battery reds on the claim. This is `demo`'s third act as it prints, trimmed of its remedy, help and spec text:

<!-- demo-proof:begin -->

```text
  An agent reports task add-login-page done: it moves the task to Done and stamps
  validate and close for iteration first-release, but no test run was ever recorded.

  | ===== check-evidence-manifest =====
  | EVIDENCE-MANIFEST: 1 issue(s) coupling .workflow/validate-evidence.txt to .workflow/WORKFLOW-STATE.txt:
  |   iteration 'first-release' has a validate stamp but no evidence line — validate ran and recorded nothing
  |   FAIL: check-evidence-manifest (exit 1)
  → caught. Registered as a hook, this would have refused the commit.
```

<!-- demo-proof:end -->

Nothing there is a review opinion. Each finding is cheap and mechanically decidable, which is what earns it the right to block a commit rather than open a thread; the semantic residue stays with the human or the agent, undiluted.

That is what **verification under delegation** means, and it is the prerequisite for scaling agent [orchestration](orchestration.md) past the point where a human reads every hop: coordination is only worth parallelizing once each coordinated result is checkable.

Which harnesses it runs under, and what adapts to yours: the [tiered compatibility claim](positioning.md#the-tiered-compatibility-claim).

## Start here

1. [Why Checkwright](methodology.md) — the delivery-methodology essay: what goes wrong when agents write, and the shape of the remedy.
2. [Install](install.md) — vendoring the kits into your repo and the upgrade contract.
3. [Value](value.md) — what each kit enforces set against what it costs your context budget, joined from the registries; drills down to the [enforcement map](enforcement.md) and the [footprint](footprint.md).
4. [Coupling graph](check-graph.html) — which content surfaces each gate binds together, emitted from the per-gate manifests.
5. The [Kit Reference](kits.md) — one page per kit, in reading order.
6. [Roadmap](https://github.com/checkwright/checkwright/blob/master/ROADMAP.md) — where the project is heading and what moves an item.
7. [Announcing Checkwright](posts/2026-07-09-announcing-checkwright.md) — the launch note.

## The kits

One page per kit, in reading order — each kit assumes the machinery of the ones above it. The full map, with a one-line gloss per kit, lives on the [Kit Reference](kits.md) page.

## Positioning

Where Checkwright sits against practices you may already run, one page per angle.

- [Where Checkwright sits](positioning.md) — the layer model: Checkwright as layer-4 content beneath a closed harness prompt, plus its tiered harness-compatibility claim and memory-off position.
- [Domain-driven design](ddd.md) — Checkwright as the enforcement layer for a ubiquitous language: banned synonyms, comment and naming directives, and one home per definition.
- [Agent orchestration](orchestration.md) — Checkwright as the verification layer beneath a coordination framework: the gates, budget guard, stage stamps, and evidence manifest that make delegated work checkable.

## License

Checkwright is Apache-2.0. Adoption is the goal.
