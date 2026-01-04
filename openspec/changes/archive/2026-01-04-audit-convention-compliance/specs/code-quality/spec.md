## ADDED Requirements

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
