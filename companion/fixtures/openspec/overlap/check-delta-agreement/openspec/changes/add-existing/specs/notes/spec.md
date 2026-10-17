## ADDED Requirements
### Requirement: Entry length
The system SHALL cut each entry to 100 characters.

#### Scenario: Long subject
- **WHEN** a merged change's subject runs past the limit
- **THEN** its entry ends at the limit with an ellipsis
