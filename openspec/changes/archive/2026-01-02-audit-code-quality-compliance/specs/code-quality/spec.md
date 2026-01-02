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

## ADDED Requirements

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
