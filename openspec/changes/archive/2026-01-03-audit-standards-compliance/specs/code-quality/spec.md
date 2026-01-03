## MODIFIED Requirements

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
