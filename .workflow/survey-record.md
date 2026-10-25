# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-25 scope — Do the four carried gap bullets' code premises hold at HEAD?
- corpus: native/src/walk.rs native/src/emit/install_hooks.rs native/src/marker.rs native/src/fresh.rs native/src/installer/mod.rs native/src/emit/installer_smoke
- oracle: grep -rn 'GIT_CONFIG' native/src/emit/installer_smoke; grep -n 'cfg(unix)' native/src/installer/mod.rs
- rev: ca3535582d41a77ff185a9f22f7c78d8e03fa06e
- finding: All hold: install_hooks prior() resolves through walk::abs_against -> normalize_abs, a lexical '..' fold; marker install_block/uninstall_block push '\n' after kept lines; fresh::file_lines splits on LF alone, keeping a CR; link_below's link test is #[cfg(unix)] with no windows case; no GIT_CONFIG name under installer_smoke (only fence_run.rs). Owner lookup in the queue, icebox and disposed-findings: none. Git floor 2.15; git hook run needs 2.36.
- inferred: marker.rs's (a) non-LIFO two-worktree newline outcome is read off source, not executed
