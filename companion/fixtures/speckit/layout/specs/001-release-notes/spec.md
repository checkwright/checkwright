# Feature Specification: Release notes export

**Feature Branch**: `001-release-notes`

**Created**: 2026-09-27

**Status**: Draft

**Input**: User description: "Export the notes for one release as a markdown file."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Export one release (Priority: P1)

A maintainer names a release and receives its notes as one markdown file.

**Why this priority**: Without it there is no release note at all.

**Independent Test**: Export a tagged release and compare the file with the merged changes.

**Acceptance Scenarios**:

1. **Given** a tagged release, **When** the maintainer exports it, **Then** the file lists every merged change.
2. **Given** an unknown release, **When** the maintainer exports it, **Then** the export refuses and writes no file.

### Edge Cases

- A release with no merged change exports a file that says so.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The export MUST list each merged change once.
- **FR-002**: The export MUST refuse a release name it cannot resolve.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer exports a release in one command.

## Assumptions

- Releases are git tags.
