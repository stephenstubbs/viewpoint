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

The codebase SHALL produce zero warnings when running `cargo clippy` with pedantic lints enabled. Temporary suppressions SHALL NOT be used to hide incomplete work.

#### Scenario: Clean clippy check
- **WHEN** running `cargo clippy` on the workspace
- **THEN** the command completes with zero warnings

#### Scenario: Clean clippy check with integration features
- **WHEN** running `cargo clippy --workspace --all-targets --features integration`
- **THEN** the command completes with zero warnings
- **AND** no new warnings are introduced by integration test code

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

#### Scenario: Format string inlining
- **WHEN** a format string contains a single variable reference like `"{}", var`
- **THEN** the code SHALL use inlined format args like `"{var}"`
- **AND** the `uninlined_format_args` clippy lint SHALL be satisfied

#### Scenario: Raw string hash minimization
- **WHEN** a raw string literal uses hash delimiters
- **THEN** only the minimum necessary number of hashes SHALL be used
- **AND** the `needless_raw_string_hashes` clippy lint SHALL be satisfied

#### Scenario: Closure simplification
- **WHEN** a closure simply calls a method with no additional logic
- **THEN** the closure SHALL be replaced with a method reference where possible
- **AND** the `redundant_closure_for_method_calls` clippy lint SHALL be satisfied

#### Scenario: Single character patterns
- **WHEN** a string method like `contains()` or `split()` receives a single-character string
- **THEN** a char literal SHALL be used instead of a string literal
- **AND** the `single_char_pattern` clippy lint SHALL be satisfied

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

#### Scenario: Doc test feature gating
- **WHEN** a doc test demonstrates browser automation APIs requiring Chromium
- **THEN** the doc test SHALL use the `#[cfg(feature = "integration")]` attribute in a hidden setup block
- **AND** the doc test SHALL include hidden boilerplate to launch browser, create context, and page
- **AND** the doc test SHALL compile with `cargo test` but only execute with `cargo test --features integration`

#### Scenario: Doc test structure
- **WHEN** a feature-gated doc test is written
- **THEN** the doc test SHALL follow this pattern:
  - Use ```` ``` ```` (no `ignore` or `no_run`)
  - Include hidden (`# `) lines for: feature gate, async wrapper, browser setup, cleanup
  - Show only the API usage in visible lines
- **AND** the doc test MUST be self-contained and executable

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

#### Scenario: Float comparison in tests
- **WHEN** tests compare floating point values for equality
- **THEN** the specific test function MAY use `#[allow(clippy::float_cmp)]`
- **AND** the suppression SHALL include a comment: `// Testing exact float values`
- **AND** crate-level `#![allow(clippy::float_cmp)]` SHALL NOT be used

### Requirement: Test Reliability

Integration tests SHALL be reliable and not exhibit flaky behavior due to timing, resource contention, or environmental factors.

#### Scenario: No flaky tests
- **WHEN** integration tests are run multiple times consecutively
- **THEN** all tests SHALL pass consistently
- **AND** no test SHALL fail intermittently due to timing issues

#### Scenario: Process cleanup tests use serial execution
- **WHEN** tests verify process cleanup (zombie detection, resource cleanup)
- **THEN** those tests SHALL use serial execution to prevent interference
- **AND** the `serial_test` crate SHALL be used for test isolation

#### Scenario: Adequate timing for async operations
- **WHEN** tests wait for process state changes (e.g., process termination, zombie reaping)
- **THEN** wait durations SHALL be sufficient for the operation to complete reliably
- **AND** Drop handlers SHALL wait long enough to reap child processes before returning

#### Scenario: Test isolation
- **WHEN** tests interact with external resources (browser processes, file system)
- **THEN** tests SHALL not affect each other's state
- **AND** cleanup operations SHALL complete before the next test begins

### Requirement: Convention Compliance Auditing

The project SHALL periodically audit source code for compliance with established conventions in `project.md` and related specs.

#### Scenario: File size audit execution
- **WHEN** an audit is requested for file size compliance
- **THEN** all `.rs` files in the `crates/` directory SHALL be scanned
- **AND** files exceeding 500 lines SHALL be reported with their line counts
- **AND** test files (in `tests/` directories or crate-level `tests/`) MAY exceed 500 lines with justification

#### Scenario: Module structure audit execution
- **WHEN** an audit is requested for module structure compliance
- **THEN** all modules SHALL be checked for folder module pattern usage
- **AND** modules with more than 100 lines or multiple concerns SHALL use folder structure
- **AND** violations SHALL be reported with remediation suggestions

#### Scenario: Test organization audit execution
- **WHEN** an audit is requested for test organization compliance
- **THEN** source files SHALL be checked for inline test blocks
- **AND** any `#[cfg(test)] mod tests { ... }` inline blocks SHALL be flagged
- **AND** proper pattern `#[cfg(test)] mod tests;` with external `tests/` directory SHALL be verified

#### Scenario: JavaScript code audit execution
- **WHEN** an audit is requested for JavaScript code compliance
- **THEN** source files SHALL be checked for raw JavaScript strings
- **AND** any JavaScript code not using the `js!` macro SHALL be flagged
- **AND** exceptions for `viewpoint_js_core` utilities SHALL be noted

#### Scenario: Audit report format
- **WHEN** an audit is completed
- **THEN** findings SHALL be categorized by type (file size, structure, tests, JS usage)
- **AND** each finding SHALL include file path, description, and severity
- **AND** remediation recommendations SHALL be provided

