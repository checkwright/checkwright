---

description: "Task list for the release notes export"
---

# Tasks: Release notes export

**Input**: Design documents from `specs/001-release-notes/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md)

## Phase 1: Setup (Shared Infrastructure)

- [ ] T001 Create the export command's script

## Phase 2: User Story 1 - Export one release (Priority: P1)

- [ ] T002 [US1] Read the merged changes between two tags
- [X] T003 [US1] Write them as one markdown file in src/format-notes.awk

## Parallel Example: User Story 1

```bash
# Launch both tasks for User Story 1 together:
Task: "Read the merged changes between two tags"
Task: "Write them as one markdown file"

## Dependencies & Execution Order

- Phase 2 depends on Phase 1.
