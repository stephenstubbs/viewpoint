# Code Quality - Dependency Updates

## ADDED Requirements

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
