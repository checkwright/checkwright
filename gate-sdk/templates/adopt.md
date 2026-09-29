The `adopt` walk — fitting an installed Checkwright to this repository and triaging its first reds, with the user. Not an iteration stage: it stamps no state and runs in any session after `init`. Exit condition: the full battery is green, or every red carries a disposition the user accepted.

## Steps

1. **Confirm the install.** `checkwright.lock` exists at the repository root and the worktree is clean. If the lock is missing, Checkwright is not installed here: say so, and stop. Read the lock for the profile, the kits and any selection.
2. **Find the work.** Run `doctor` the way `init` was run. Each `disarmed` line names a gate that asserts nothing until the knob it names is set: a configuration item. Then run the battery on the gate binary `GATE_SDK_NATIVE_BIN` names, the one `init`'s `next:` block runs, with `--run` and `GATE_SDK_VERBOSE` set to any non-empty value so every gate prints its line, and keep the output. Each red is a triage item. Each clean line counting zero files, pages or entries names a gate that reads nothing on this tree, and is a configuration item too.
3. **Fit the knobs to the layout.** For each configuration item, and each red whose finding names a path the gate should not read, find the knob that sets its corpus or its exemptions: `--reads <gate>` names the knobs a gate reads, `--emit knob-roster` lists every knob with its type and default, and `--emit knob-values <NAME>` gives its resolved value here. Survey the tree for where its specs, docs, sources, and generated or third-party directories are. Tell the user each change and the paths that ground it, then write it into the `*-config.knobs` file in the gates directory named for the knob's kit: one `NAME = value` line for a scalar, one `NAME[] = element` line per element for an `indexed` knob. Never edit a vendored kit file.
4. **Triage each red that remains.** Read the finding, its `help:` line and the section its `spec:` pointer names. Then give it one disposition, in this order of preference:
   - a defect in the tree: fix the tree;
   - a legitimate exception the gate provides a valve for, which its `help:` line or its section names: apply the valve, with its reason;
   - a gate of no value to this repository: remove it by re-running `init` with `--without-gate <gate>`, which the lock records, or replace it with a gate of the same name in the gates directory;
   - a red the gate should not raise here: dispose of it as above, and report the finding to the kit's publisher, where its README says.

   Never bypass a hook, and never edit a vendored gate. Tell the user each disposition before applying it.
5. **Re-run the battery**, and triage each new red as in step 4; a generated file a knob change left stale names its own regeneration command. **Commit** the knob changes and dispositions in one commit naming them, once the battery is green or each remaining red's disposition is accepted.
