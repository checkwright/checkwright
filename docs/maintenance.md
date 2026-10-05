---
title: Maintenance
nav_parent: install
nav_child_order: 3
---

# Managing and upgrading an install

## Managing

`checkwright <verb>` below means the one line with `<verb>` as its argument (`sh -s -- <verb>` on macOS and Linux, the script-block form on Windows, since `irm … | iex` takes none), your install recipe's `init` line with `<verb>` in place of `init`, or `npx checkwright <verb>`. Each verb answers in its exit status, so a CI step can gate on it.

- `checkwright doctor` checks this machine against [Requirements](requirements.md) and reports what is installed.
- `checkwright diff` lists the vendored files you have changed. Exit `0` means none.
- `checkwright update` upgrades the install to the version you are running.
- `checkwright uninstall` reverses the install in one commit.

`uninstall` needs no version: any release that reads your `checkwright.lock` runs it, so the one line reverses an install an older release made. `uninstall` removes only files `init` wrote and you left untouched. It keeps and reports any you edited, and never removes a file you wrote. Run it with `--dry-run` first to see the plan. A remedy block changes your machine and not your repository, so `uninstall` leaves it in place.

### Requiring the CI check

The pre-commit hook is a local backstop anyone can skip. `init` also commits `.github/workflows/gates.yml`, which runs the battery on every push and pull request at the release you installed and marks each red on the pull request. Make its check required in your branch protection, so a red battery blocks the merge, and keep the workflow file where the authors it checks cannot edit it.

## Upgrading

Release channel: **preview**

While the channel reads `preview`, versions are `0.x`, a minor may break things, and every GitHub Release is marked pre-release, so none shows as Latest. Take a release from the releases list, or name an explicit version. Never rely on the Latest pointer.

An upgrade has two phases:

1. Run `checkwright update` from the new version, then run the commands it prints. If a kit now ships a `.knobs` config in place of a `*-config.sh`, move your settings into the `.knobs` file and delete the old one.
2. Run the full battery. The gates that go red are your worklist. The release note says why each one moved.

Every release note opens with **In brief**: a table linking each section below with its entry count and who acts on it, then a few plain bullets on what you get and whether you must act. Its **New and tightened gates**, **Knob changes**, **Gate-authoring changes**, **Platforms** and **Behavior changes** sections list what you reconcile. Each says "None." when there is nothing. All notes are on the [releases page](releases.md). The bump rules and the note grammar: [Versioning](installer/SPEC.md#versioning) and [The upgrade contract](installer/SPEC.md#the-upgrade-contract).

An installed tree checks for a newer release once a week with `git ls-remote`, which shows the upstream host your IP address, and `doctor` and the session-context hook say when one exists. Set `GATE_SDK_UPDATE_CHECK = off` in your gates directory's `gate-sdk-config.knobs` to turn it off.
