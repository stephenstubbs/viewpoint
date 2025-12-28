# Code Quality - Clippy Suppression Policy

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

## ADDED Requirements

### Requirement: Suppression Justification Policy

Clippy lint suppressions SHALL only be used when there is a clear, documented reason and the code cannot be reasonably refactored.

#### Scenario: Allowed suppressions
- **WHEN** a `#[allow(...)]` attribute is used
- **THEN** it SHALL be accompanied by a comment explaining why
- **AND** the suppression SHALL be as narrow as possible (item-level, not module-level)

#### Scenario: Prohibited blanket suppressions
- **WHEN** a `#![allow(...)]` crate-level suppression exists
- **THEN** it SHALL be removed and issues fixed individually
- **AND** only workspace-level configuration in `Cargo.toml` is acceptable for project-wide policy

#### Scenario: Dead code handling
- **WHEN** dead code warnings appear
- **THEN** the code SHALL be either used, removed, or gated with `#[cfg(...)]`
- **AND** `#[allow(dead_code)]` SHALL NOT be used to hide incomplete features
