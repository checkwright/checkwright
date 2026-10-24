---
title: Releases
nav_order: 8
nav_children_key: release
---

# Release notes

Every tagged release carries one dated note — the phase-B checklist the [upgrade contract](installer/SPEC.md#the-upgrade-contract) names, its bump chosen by that file's [§Versioning](installer/SPEC.md#versioning) criteria. The GitHub Release for each tag points at the same note.

The table reads every note for you, newest first. **Action on upgrade** says whether a clean upgrade may turn a gate red or leave a knob you set renamed or removed. Each column after it counts that section's entries, and a dash marks a note published before the section existed. The table is generated from the notes, so a note joins it by carrying its `release:` front-matter key.

<!-- release-table:begin -->

| Release | Date | Bump | Action on upgrade | New and tightened gates | Knob changes | Gate-authoring changes | Platforms | Behavior changes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [v0.33.0](posts/2026-10-24-checkwright-v0-33-0.md) | 2026-10-24 | minor | no | 0 | 0 | 0 | 0 | 2 |
| [v0.32.1](posts/2026-10-24-checkwright-v0-32-1.md) | 2026-10-24 | patch | no | 0 | 0 | 0 | 0 | 0 |
| [v0.32.0](posts/2026-10-23-checkwright-v0-32-0.md) | 2026-10-23 | minor | yes | 74 | 29 | 8 | 0 | 212 |
| [v0.31.0](posts/2026-09-30-checkwright-v0-31-0.md) | 2026-09-30 | minor | no | 0 | 0 | — | — | 8 |
| [v0.30.0](posts/2026-09-29-checkwright-v0-30-0.md) | 2026-09-29 | minor | yes | 5 | 0 | — | — | 8 |
| [v0.29.0](posts/2026-09-28-checkwright-v0-29-0.md) | 2026-09-28 | minor | yes | 3 | 0 | — | — | 7 |
| [v0.28.0](posts/2026-09-28-checkwright-v0-28-0.md) | 2026-09-28 | minor | yes | 1 | 0 | — | — | 2 |
| [v0.27.0](posts/2026-09-28-checkwright-v0-27-0.md) | 2026-09-28 | minor | yes | 5 | 0 | — | — | 16 |
| [v0.26.0](posts/2026-09-26-checkwright-v0-26-0.md) | 2026-09-26 | minor | yes | 166 | 13 | — | — | 303 |
| [v0.25.0](posts/2026-08-23-checkwright-v0-25-0.md) | 2026-08-23 | minor | yes | 5 | 0 | — | — | 5 |
| [v0.24.0](posts/2026-08-21-checkwright-v0-24-0.md) | 2026-08-21 | minor | yes | 6 | 1 | — | — | 13 |
| [v0.23.0](posts/2026-08-14-checkwright-v0-23-0.md) | 2026-08-14 | minor | yes | 26 | 0 | — | — | 9 |
| [v0.22.0](posts/2026-08-06-checkwright-v0-22-0.md) | 2026-08-06 | minor | yes | 12 | 0 | — | — | 18 |
| [v0.21.0](posts/2026-08-01-checkwright-v0-21-0.md) | 2026-08-01 | minor | yes | 2 | 0 | — | — | 4 |
| [v0.20.0](posts/2026-08-01-checkwright-v0-20-0.md) | 2026-08-01 | minor | no | 0 | 0 | — | — | 1 |
| [v0.19.0](posts/2026-07-31-checkwright-v0-19-0.md) | 2026-07-31 | minor | yes | 1 | 0 | — | — | 3 |
| [v0.18.0](posts/2026-07-31-checkwright-v0-18-0.md) | 2026-07-31 | minor | yes | 3 | 0 | — | — | 3 |
| [v0.17.0](posts/2026-07-27-checkwright-v0-17-0.md) | 2026-07-27 | minor | yes | 1 | 0 | — | — | 2 |
| [v0.16.0](posts/2026-07-26-checkwright-v0-16-0.md) | 2026-07-26 | minor | no | 0 | 0 | — | — | 5 |
| [v0.15.0](posts/2026-07-25-checkwright-v0-15-0.md) | 2026-07-25 | minor | yes | 1 | 0 | — | — | 6 |
| [v0.14.0](posts/2026-07-25-checkwright-v0-14-0.md) | 2026-07-25 | minor | no | 0 | 0 | — | — | 2 |
| [v0.13.0](posts/2026-07-22-checkwright-v0-13-0.md) | 2026-07-22 | minor | yes | 3 | 0 | — | — | 3 |
| [v0.12.0](posts/2026-07-22-checkwright-v0-12-0.md) | 2026-07-22 | minor | no | 0 | 0 | — | — | 3 |
| [v0.11.0](posts/2026-07-22-checkwright-v0-11-0.md) | 2026-07-22 | minor | no | 0 | 0 | — | — | 5 |
| [v0.10.0](posts/2026-07-20-checkwright-v0-10-0.md) | 2026-07-20 | minor | yes | 4 | 0 | — | — | 2 |
| [v0.9.0](posts/2026-07-19-checkwright-v0-9-0.md) | 2026-07-19 | minor | no | 0 | 0 | — | — | 3 |
| [v0.8.0](posts/2026-07-19-checkwright-v0-8-0.md) | 2026-07-19 | minor | no | 0 | 0 | — | — | 4 |
| [v0.7.0](posts/2026-07-19-checkwright-v0-7-0.md) | 2026-07-19 | minor | yes | 1 | 0 | — | — | 4 |
| [v0.6.0](posts/2026-07-18-checkwright-v0-6-0.md) | 2026-07-18 | minor | no | 0 | 0 | — | — | 3 |
| [v0.5.0](posts/2026-07-17-checkwright-v0-5-0.md) | 2026-07-17 | minor | yes | 1 | 0 | — | — | 2 |
| [v0.4.0](posts/2026-07-17-checkwright-v0-4-0.md) | 2026-07-17 | minor | yes | 1 | 0 | — | — | 2 |
| [v0.3.0](posts/2026-07-17-checkwright-v0-3-0.md) | 2026-07-17 | minor | no | 0 | 0 | — | — | 2 |
| [v0.2.0](posts/2026-07-17-checkwright-v0-2-0.md) | 2026-07-17 | minor | yes | 7 | 0 | — | — | 4 |
| [v0.1.0](posts/2026-07-14-checkwright-v0-1-0.md) | 2026-07-14 | — | no | 0 | 0 | — | — | 0 |

<!-- release-table:end -->
