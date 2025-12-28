# code-quality Specification

## Purpose
TBD - created by archiving change refactor-codebase-cleanup. Update Purpose after archive.
## Requirements
### Requirement: Zero Compiler Warnings

The codebase SHALL produce zero warnings when running `cargo check`.

#### Scenario: Clean compile check
- **WHEN** running `cargo check` on the workspace
- **THEN** the command completes with zero warnings

#### Scenario: All imports used
- **WHEN** an import statement exists in the codebase
- **THEN** that imported item is used somewhere in the file

#### Scenario: All fields read
- **WHEN** a struct field is defined
- **THEN** that field is either read somewhere or marked with `#[allow(dead_code)]` with justification

### Requirement: Zero Clippy Warnings

The codebase SHALL produce zero warnings when running `cargo clippy` with pedantic lints enabled.

#### Scenario: Clean clippy check
- **WHEN** running `cargo clippy` on the workspace
- **THEN** the command completes with zero warnings

#### Scenario: Documentation follows conventions
- **WHEN** documentation contains code identifiers
- **THEN** those identifiers are wrapped in backticks

#### Scenario: Error documentation present
- **WHEN** a public function returns `Result`
- **THEN** the function has an `# Errors` documentation section

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

### Requirement: Module Organization

Large modules SHALL be organized into submodules for maintainability.

#### Scenario: Extract related methods
- **WHEN** a module file exceeds 500 lines
- **THEN** related method groups SHOULD be extracted to submodules
- **AND** the main module file SHOULD primarily contain re-exports and core type definitions

#### Scenario: Test file organization
- **WHEN** an integration test file exceeds 500 lines
- **THEN** tests SHOULD be split by feature area into separate test files
- **AND** common test utilities SHOULD be extracted to a shared module

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

### Requirement: Dependency Management
The workspace SHALL centralize all dependency versions in the root `Cargo.toml` under `[workspace.dependencies]`. Dependencies SHALL be kept reasonably current to benefit from security patches and bug fixes.

#### Scenario: Workspace dependency inheritance
- **WHEN** a crate needs an external dependency
- **THEN** it SHALL reference the workspace version using `.workspace = true` syntax
- **AND** the version SHALL be defined only in the root `Cargo.toml`

#### Scenario: Dependency update process
- **WHEN** dependencies are updated
- **THEN** all workspace tests MUST pass before merging
- **AND** compilation MUST succeed with `cargo check --workspace`
- **AND** no new clippy warnings MUST be introduced

