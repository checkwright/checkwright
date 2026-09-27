## ADDED Requirements
### Requirement: Digest section
The system SHALL open the exported notes with a digest of at most five lines.

#### Scenario: Release with many changes
- **WHEN** a release carries more than five merged changes
- **THEN** the digest names the five with the largest diffs

## MODIFIED Requirements
### Requirement: Entry length
The system SHALL cut each entry to 80 characters.
(Previously: 120 characters)

#### Scenario: Long subject
- **WHEN** a merged change's subject runs past 80 characters
- **THEN** its entry ends at the limit with an ellipsis
