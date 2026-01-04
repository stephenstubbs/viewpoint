## MODIFIED Requirements

### Requirement: Crate-Level Documentation
Each crate in the workspace SHALL have comprehensive `//!` documentation in its `lib.rs` file that includes:
- A brief description of the crate's purpose
- Feature highlights as a bullet list
- A "Quick Start" or "Usage" section with a runnable example
- Links to related crates and documentation
- Any feature flags and their effects

#### Scenario: viewpoint-core crate documentation
- **GIVEN** a user views the `viewpoint-core` crate documentation
- **WHEN** they read the crate-level docs
- **THEN** they see a clear explanation of browser automation capabilities
- **AND** they see a working example of launching a browser and navigating

#### Scenario: viewpoint-test crate documentation
- **GIVEN** a user views the `viewpoint-test` crate documentation
- **WHEN** they read the crate-level docs
- **THEN** they see how to write browser tests with TestHarness
- **AND** they see examples of assertions and fixture scoping

#### Scenario: viewpoint-js-core crate documentation
- **GIVEN** a user views the `viewpoint-js-core` crate documentation
- **WHEN** they read the crate-level docs
- **THEN** they see how to use ToJsValue for type conversion
- **AND** they see examples of string escaping utilities

### Requirement: Example Coverage
Documentation examples SHALL:
- Compile successfully (verified by `cargo test --doc`)
- Use `compile` attribute for async code that cannot run in doc tests
- Use `no_run` attribute only when browser interaction is required
- Avoid `ignore` attribute unless compilation would fail
- Show realistic usage patterns, not just API calls
- Include error handling with `?` operator

#### Scenario: Documentation examples compile
- **GIVEN** the codebase has documentation examples
- **WHEN** `cargo test --doc` is run
- **THEN** all examples compile without errors
- **AND** examples marked `compile` verify syntax correctness

#### Scenario: Minimal use of ignore attribute
- **GIVEN** a documentation example exists
- **WHEN** reviewing its attributes
- **THEN** `ignore` is only used when the code genuinely cannot compile
- **AND** `compile` is preferred over `ignore` for async examples

### Requirement: README Documentation
Each crate in the workspace SHALL have a README.md file that:
- Mirrors the crate-level docs for consistency
- Includes installation instructions with version placeholder
- Shows the primary use cases with examples
- Links to docs.rs for full API documentation
- Includes any platform or browser requirements

#### Scenario: README matches crate docs
- **GIVEN** a user reads a crate's README on GitHub
- **WHEN** they compare it to docs.rs
- **THEN** the key information is consistent
- **AND** examples work as documented

#### Scenario: All crates have READMEs
- **GIVEN** the workspace contains multiple crates
- **WHEN** listing README.md files
- **THEN** every crate directory contains a README.md
- **AND** each README has at minimum: description, installation, and example

## ADDED Requirements

### Requirement: Primary Type Documentation
The primary public types (Browser, BrowserContext, Page, Locator, Frame, ElementHandle) SHALL have comprehensive documentation that includes:
- A type-level description explaining its role
- Examples for each public method
- `# Errors` section documenting failure conditions
- Cross-references to related types using `[Type]` links

#### Scenario: Page type has method examples
- **GIVEN** a user views the `Page` type documentation
- **WHEN** they browse the method list
- **THEN** each method has at least one code example
- **AND** the examples use `compile` checks where possible

#### Scenario: Locator type documents all actions
- **GIVEN** a user views the `Locator` type documentation
- **WHEN** they look for action methods
- **THEN** click, fill, type, check, select_option all have examples
- **AND** each example shows realistic usage with error handling

### Requirement: Test Framework Documentation
The viewpoint-test crate SHALL document all assertion types with examples:
- Element visibility assertions (to_be_visible, to_be_hidden)
- Text assertions (to_have_text, to_contain_text)
- Attribute assertions (to_have_attribute, to_have_class)
- State assertions (to_be_enabled, to_be_checked)
- Page assertions (to_have_url, to_have_title)
- Count assertions (to_have_count)

#### Scenario: Each assertion type has example
- **GIVEN** a user views the expect module documentation
- **WHEN** they look for assertion methods
- **THEN** each assertion type has a clear example
- **AND** examples show both positive and negated forms

### Requirement: Network Interception Documentation
Network interception types (Route, Request, Response) SHALL document:
- How to set up route handlers
- How to mock responses with different content types
- How to modify requests before they are sent
- How to use HAR replay for deterministic testing

#### Scenario: Route handler documentation
- **GIVEN** a user wants to mock an API endpoint
- **WHEN** they read the Route documentation
- **THEN** they find examples of fulfill(), abort(), and continue()
- **AND** they see how to return JSON, HTML, and binary responses
