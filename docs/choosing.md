---
title: Choosing
nav_order: 2
description: When Checkwright fits a repository where coding agents commit, when it does not, and how it compares with the alternatives.
---

# Choosing Checkwright

Checkwright catches reproducible classes of unsupported claims in coding-agent delivery before merge, checking that the evidence a claim rests on exists rather than that the code is correct ([the bounded promise](positioning.md#the-bounded-promise)). This page is for a person or a coding agent deciding whether Checkwright fits a repository.

## When it fits

Checkwright fits a repository where coding agents commit and the team wants three claims caught before merge: a task marked done with no recorded test run, a skipped lifecycle stage, and a spec naming a path the tree lacks. Checkwright's checks run at commit and in CI with no agent harness, on a host where git is the one tool already required ([Requirements](requirements.md)). A team using Spec Kit or OpenSpec gets the artifacts those toolkits write held to the tree ([Spec toolkits](spec-toolkits.md)). Trying Checkwright is reversible: `demo` runs it in a scratch repository, and `uninstall` removes it ([Install](install.md)).

## When it does not fit

Checkwright does not fit a team that needs code judged correct, because its gates check that evidence exists and agrees with the tree, never what the code does ([What the gates check against your code](spec-toolkits.md#what-the-gates-check-against-your-code)). Checkwright does not fit a team that needs signed records of agent sessions or a party outside the repository vouching for a run. Checkwright signs no record of a run. The evidence-kit manifest a run writes is unsigned, the input such a party would verify ([`evidence-kit/SPEC.md` §Evidence manifest](evidence-kit/SPEC.md#evidence-manifest)).

## What you install

Each install profile is a set of kits ([`installer/SPEC.md` §Profiles](installer/SPEC.md#profiles)). The `starter` profile is gate-sdk alone. The `workflow` profile is `starter` plus lifecycle-kit and evidence-kit, the least on which a done claim is caught. The `prose` profile adds canon-kit. The `delegation` profile adds the kits whose subject is the agent session. The `full` profile is every kit. The checks and the workflow install without each other, on one base. gate-sdk's binary is that base: it runs the checks, places the git hooks and carries the commands the workflow's stages run, so every profile carries gate-sdk. The checks run without the workflow: `starter` and `prose` carry no lifecycle-kit, and the companion recipes gate another toolkit's workflow. The workflow runs without the document and site checks, since `workflow` and `delegation` carry neither canon-kit nor site-kit. Both build on `starter`, so gate-sdk's own starting checks come with them, and `--without-gate` drops any of those. The two part on what they write. The checks read what your repository tracks, and the workflow commits its own record: the stage stamps, the task queue and the recorded test runs. Under `delegation`, lifecycle-kit's checks enforce nothing until the files each one reads exist.

## Alternatives

The paragraphs on Chainloop and agent-attest state each project as its README did on 2026-10-17.

**Your own CI.** Pre-commit hooks, scripts the team writes and required status checks can reproduce any single check here, for a team that writes and maintains it. Checkwright adds checks already written for the claims coding agents make, each shipping with a passing and a failing fixture and failing closed when it cannot run. It also adds the process state those checks read: the task queue, the stage stamps and the recorded test runs.

**[Chainloop](https://github.com/chainloop-dev/chainloop)** (Apache-2.0) records coding-agent sessions, attesting them on `git push`, and CI outputs such as software bills of materials (SBOMs), scans and test reports. Each record is a signed, timestamped in-toto attestation kept in the team's Open Container Initiative (OCI) registry, S3 or Azure Blob, the CLI sending records to Chainloop Cloud by default. Policies in Rego or WebAssembly (WASM), which the team writes, say what each record must satisfy. A policy marked gating fails the CI job when a rule breaks, and a push is rejected where no session was recorded. Its PR merge check is in the paid Platform. Chainloop fits where the requirement is signed, stored evidence of what agents and pipelines did, judged by the team's own policies. Checkwright fits where the claim to check is in the repository's own files, with no keys and no record store.

**[agent-attest](https://github.com/zhengqiuyang/agent-attest)** (MIT) writes an ed25519-signed in-toto attestation per commit under `.agent-attest/`, recording the agent, a hash of the prompt, the run's timing and exit code, and the result of a verification command. It marks whether that command was executed or only claimed. Its `gate` fails a commit touching a configured protected path when the commit has no valid passing attestation. It runs on Node. agent-attest fits where the requirement is a signed chain of custody for agent commits on protected paths. Checkwright fits where the requirement is checks over the task, stage and spec state a claim rests on.

## Limits

- What a gate promises, and what it does not, is stated in the bounded promise.
- What the toolkit companion does not cover is stated in the [Spec toolkits limits](spec-toolkits.md#limits).
- Which installs have been observed, and on which systems, is recorded in the [install evidence](install-evidence.md).
- No check holds the Alternatives section's statements about other projects, which read as of the date that section gives.
