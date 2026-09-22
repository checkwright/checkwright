# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-22 scope — Which deferred entries lead the next unit set under the admission filter?
- corpus: TASK-QUEUE.md
- oracle: checkwright-gates --emit queue-edges
- rev: 570de3764ca41b8deb80ef683557abc67aea0bec
- finding: No session-class row; the one iteration-class row and every roadmap row are filter-excluded or observation-gated; the fill surface is queue-kit (11 rows)
- inferred: none

## 2026-09-22 spec — Which native arms measure markdown size in lines, and which read line width?
- corpus: native/src
- oracle: grep -n 'lines().count\|newline\|LINE_CAP\|BREVITY_BUDGET' over native/src, each module read
- rev: 14430e5b8cfa5c46bff1e2b51296000653a13af7
- finding: Markdown line-as-size: always_loaded.rs (meter+ratchet), footprint.rs, brevity.rs, queue_entry_budget.rs; queue_index.rs prints extent lines (display). Not markdown: overhead_meter.rs (bytes), scan_prompts.rs, port_blockers.rs (.sh). Width: queue_wrap.rs (code points), audit_roster.rs (bytes, .txt).
- inferred: none

## 2026-09-22 spec — Which native readers parse the queue entry shape (column-0 bold-slug bullet)?
- corpus: native/src
- oracle: git grep -n 'is_top_level_bullet\|bullet_slug\|first_bold_slug\|is_bullet\|live_slugs\|backtick_slugs' -- native/src, plus grep for independent '- **' scans outside native/src/queue.rs
- rev: 14430e5b8cfa5c46bff1e2b51296000653a13af7
- finding: queue.rs adapters and callers (queue_index, queue_counts, queue_edges, entry_history, roadmap, task_names, queue_entry_budget, task_conservation, deferred_board_tags, tag_lead_line, roadmap_fresh, queue_wrap, queue_slug_liveness, queue_hygiene); independent holders: spec.rs queue_slugs, amendment_queue.rs, provenance_seam.rs lead_slugs, gate_exemption_tasks.rs, evidence_baseline.rs, kpi queue_net_delta and deferred_age, stage_entry.rs, file_gap.rs entry_slug, queue_prose_precondition.rs; task_split.rs reads only the done grammar.
- inferred: none
