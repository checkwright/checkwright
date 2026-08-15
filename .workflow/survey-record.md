# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

















## 2026-08-15 scope — Which gates compose the eleventh native-port cohort, and is the selection rule's size arm still live?
- corpus: the 57 still-shell members of the 104 registered gates, plus gate-sdk/SPEC.md's ten cohort sections and TASK-QUEUE.md's cohort-held-members-port-prerequisites
- oracle: bash gate-sdk/bin/port-blockers.sh --group (and the default arm), read against gate-sdk/SPEC.md section The first cohort, and the rule that selects the next
- rev: 60b9e43ab4deb494e6c3934ee000eba24ee2ee29
- finding: SIZE ARM EXHAUSTED A THIRD TIME - the entry currently says twice. Group 1 (14 members, libs=fail_closed globs=-) is a ruled NON-COHORT: fail_closed derives no corpus, so its members share the ABSENCE of a derivation (operator-ruled 2026-08-14, restated at section The canonical-spec cohort and section The declaration cohort). Group 2 contains check-install-disposition, a standing stays-shell exclusion; group 3 contains check-docs-render-fidelity at c7=ruby. So every group above size 1 is either the ruled non-cohort or holds an excluded member, and the largest takeable shared-derivation group is ONE. Criteria shape of the still-shell 57: c2=pair on all 57 (zero fixture failures); c3 is 54 precommit, 2 commit-msg, 1 align-only; c7 is 46 clean, 7 with a real external program (jq x2, shellcheck x2, ruby, paste, cargo) and 4 undecidable meta-gates auditing the substrate itself. CONSUMER TRANCHE IS COHERENT AND LARGEST: of consumer-gate-port-disposition's 10 remaining, 5 sit in the residual group 1 and 5 are singletons, but 8 of 10 are fully criteria-clearing with no blocker beyond ordinary port effort, because the tenth cohort already merged and paid the first-mover design they would each have needed. Only check-installer-no-deps (jq, and named the cheapest single-gate first mover) and check-docs-kit-parity (c7 undecidable) carry a flag. REMAINING BLOCKERS: the associative-array config bridge, one prerequisite shared by check-stage-entry and check-evidence-baseline across two kits, still open and invisible to --group because the two land in different key groups; and check-roadmap-fresh's emitter design, three candidates and none ruled, so it is NOT port-ready today. The POSIX ERE engine blocker is PAID and confirmed closed.

## 2026-08-15 close — Does any shell gate or bin/ tool in this tree believe it fails closed on a mapfile-fed producer, when reading the status after a mapfile fed by a process substitution captures mapfile's own status and never the child's?
- corpus: every tracked shell script in the tree (git ls-files on the sh glob), scanning for a mapfile line with a status read or fail_closed within the next three lines
- oracle: awk over the tracked shell scripts: record each mapfile line, flag any following line within three that spells a status read or a fail_closed call, then read each hit by hand to decide whether that status belongs to the mapfile or to a later command substitution
- rev: 999fce5f210dad12054f2510061e9fa46c82672a
- finding: ZERO LIVE HOLES. Six candidate hits, all false positives: in every one the status read belongs to a later command substitution, which propagates the child status correctly. One genuine dead-idiom spelling survives at installer/consumer-smoke/run-smoke.sh line 211, where the mapfile carries an or-exit-2 that can never fire; it is harmless because line 212 tests the array for emptiness and exits 2 there, so it fails closed by a different mechanism than its author wrote. The class is therefore EMPTY in this tree and no scanner was filed: a gate for a zero-instance class is the registry-for-one-member move the payload-claim design already refuses on its own axis. Re-run the oracle before assuming this still holds after any new shell gate lands.
