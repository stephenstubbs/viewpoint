## MODIFIED Requirements

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
