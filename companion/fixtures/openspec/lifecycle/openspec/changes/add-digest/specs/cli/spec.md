## MODIFIED Requirements
### Requirement: Export command
The system SHALL export the notes of the release named by the `export` command's one argument, with a digest when `--digest` is given.

#### Scenario: Named release
- **WHEN** a maintainer runs `export v1.2.0`
- **THEN** the notes for v1.2.0 are written

#### Scenario: Digest asked for
- **WHEN** a maintainer runs `export --digest v1.2.0`
- **THEN** the notes for v1.2.0 open with a digest
