# notes Specification

## Purpose
Release notes: how the notes for one release are gathered and written. The export writes the file the [digest change](../../changes/add-digest/proposal.md) extends.

## Requirements
### Requirement: Export one release
The system SHALL write the notes for one named release as a markdown file.

#### Scenario: Tagged release
- **WHEN** a maintainer exports a tagged release
- **THEN** the file lists every merged change since the previous tag

#### Scenario: Unknown release
- **WHEN** a maintainer exports a name that is not a tag
- **THEN** the export refuses and writes no file

### Requirement: Entry length
The system SHALL cut each entry to 120 characters.

#### Scenario: Long subject
- **WHEN** a merged change's subject runs past 120 characters
- **THEN** its entry ends at the limit with an ellipsis
