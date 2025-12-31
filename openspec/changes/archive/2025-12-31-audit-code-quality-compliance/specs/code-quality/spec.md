## MODIFIED Requirements

### Requirement: Zero Clippy Warnings

The codebase SHALL produce zero warnings when running `cargo clippy` with pedantic lints enabled. Temporary suppressions SHALL NOT be used to hide incomplete work.

#### Scenario: Clean clippy check
- **WHEN** running `cargo clippy` on the workspace
- **THEN** the command completes with zero warnings

#### Scenario: Documentation follows conventions
- **WHEN** documentation contains code identifiers
- **THEN** those identifiers are wrapped in backticks

#### Scenario: Error documentation present
- **WHEN** a public function returns `Result`
- **THEN** the function has an `# Errors` documentation section

#### Scenario: Panic documentation present
- **WHEN** a public function can panic
- **THEN** the function has a `# Panics` documentation section

#### Scenario: No blanket suppressions
- **WHEN** a crate-level `#![allow(...)]` is considered
- **THEN** the underlying issue SHALL be fixed instead
- **AND** suppressions SHALL only be used for intentional design decisions with documented justification

#### Scenario: Pedantic lint compliance
- **WHEN** clippy suggests using `map_or` instead of `map().unwrap_or()`
- **THEN** the code SHALL be updated to use the suggested pattern
- **AND** similar pedantic improvements SHALL be applied consistently

### Requirement: Maintainable File Sizes

Source files SHALL be kept to a maintainable size to enable effective code review and comprehension.

#### Scenario: Source file size limit
- **WHEN** a source file (`.rs`) is created or modified
- **THEN** the file SHALL NOT exceed 500 lines
- **AND** files approaching 400 lines SHOULD be reviewed for refactoring opportunities

#### Scenario: Test file size limit
- **WHEN** a test file is created or modified
- **THEN** the file SHOULD NOT exceed 500 lines
- **AND** related tests SHOULD be grouped in separate files by feature area

#### Scenario: Large file refactoring
- **WHEN** a file exceeds 500 lines
- **THEN** the file SHALL be refactored into smaller modules
- **AND** related functionality SHALL be grouped into logical submodules

#### Scenario: Existing violations addressed
- **WHEN** an audit identifies files exceeding 500 lines
- **THEN** those files SHALL be refactored before new features are added
- **AND** each refactored module SHALL have a single, clear responsibility
