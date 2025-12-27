## ADDED Requirements

### Requirement: Folder Module Structure

All non-trivial modules SHALL use the folder module pattern with `mod.rs` rather than single `.rs` files.

#### Scenario: Module with multiple concerns
- **WHEN** a module contains types, implementations, and tests
- **THEN** the module SHALL be organized as a directory with `mod.rs`
- **AND** tests SHALL be in a `tests/` subdirectory

#### Scenario: Small utility module exception
- **WHEN** a module is a crate entry point (`lib.rs`) or less than 100 lines with single responsibility
- **THEN** the module MAY remain as a single `.rs` file
- **AND** the decision MUST be justified in code comments if exceeding typical patterns

### Requirement: External Unit Tests

Unit tests SHALL be organized in separate `tests/` directories within each module, not as inline `#[cfg(test)] mod tests` blocks.

#### Scenario: Test module structure
- **WHEN** a module has unit tests
- **THEN** tests SHALL be placed in `<module>/tests/mod.rs`
- **AND** the source file SHALL reference tests with `#[cfg(test)] mod tests;`

#### Scenario: Test file contents
- **WHEN** tests are extracted to a tests directory
- **THEN** the test module SHALL use `use super::*;` to access parent module items
- **AND** test helper functions SHOULD be marked `pub(crate)` or kept private to tests

#### Scenario: Integration tests location
- **WHEN** tests require a running browser or external resources
- **THEN** tests SHALL be placed in `crate_root/tests/` directory
- **AND** tests SHALL use the `#![cfg(feature = "integration")]` attribute

### Requirement: Integration Test Feature Flag

Crates with integration tests that require external resources (browser, network) SHALL use the `integration` feature flag to gate those tests.

#### Scenario: Feature flag definition
- **WHEN** a crate has integration tests requiring Chromium or external resources
- **THEN** the crate's `Cargo.toml` SHALL define `integration = []` in `[features]`

#### Scenario: Test file gating
- **WHEN** an integration test file requires Chromium
- **THEN** the file SHALL begin with `#![cfg(feature = "integration")]`
- **AND** tests SHALL only run when `cargo test --features integration` is used

#### Scenario: Unit tests remain ungated
- **WHEN** tests do not require external resources (pure unit tests, mocked dependencies)
- **THEN** tests SHALL NOT use the `integration` feature gate
- **AND** tests SHALL run with plain `cargo test`

## MODIFIED Requirements

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
