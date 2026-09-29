# cli Specification

## Purpose
The command line: how a maintainer asks for the notes of one release.

## Requirements
### Requirement: Export command
The system SHALL export the notes of the release named by the `export` command's one argument.

#### Scenario: Named release
- **WHEN** a maintainer runs `export v1.2.0`
- **THEN** the notes for v1.2.0 are written
