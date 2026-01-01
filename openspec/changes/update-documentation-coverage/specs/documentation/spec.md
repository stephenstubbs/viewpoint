# Documentation Specification

## ADDED Requirements

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

### Requirement: Public API Documentation
All public types, traits, methods, and functions SHALL have `///` doc comments that include:
- A brief description of what the item does
- At least one code example (where practical)
- Documentation of parameters and return values
- Documentation of error conditions with `# Errors` section
- Cross-references to related types using `[Type]` links

#### Scenario: Page type documentation
- **GIVEN** a user views the `Page` type documentation
- **WHEN** they read the type and method docs
- **THEN** each public method has a description and example
- **AND** navigation methods document timeout and error behavior

#### Scenario: Locator type documentation  
- **GIVEN** a user views the `Locator` type documentation
- **WHEN** they read the type and method docs
- **THEN** they understand how to locate elements
- **AND** each action method (click, fill, etc.) has an example

### Requirement: Module-Level Documentation
Each public module SHALL have `//!` documentation in its `mod.rs` file that explains:
- The module's purpose and scope
- Key types defined in the module
- Typical usage patterns
- Links to related modules

#### Scenario: browser module documentation
- **GIVEN** a user views the `browser` module documentation
- **WHEN** they read the module-level docs
- **THEN** they understand the three connection methods (launch, connect, connect_over_cdp)
- **AND** they see examples of each connection method

#### Scenario: network module documentation
- **GIVEN** a user views the `network` module documentation
- **WHEN** they read the module-level docs
- **THEN** they understand network interception capabilities
- **AND** they see how Route, Request, and Response types relate

### Requirement: Example Coverage
Documentation examples SHALL:
- Compile successfully (verified by `cargo test --doc`)
- Use `no_run` attribute only when browser interaction is required
- Show realistic usage patterns, not just API calls
- Include error handling with `?` operator

#### Scenario: Documentation examples compile
- **GIVEN** the codebase has documentation examples
- **WHEN** `cargo test --doc` is run
- **THEN** all examples compile without errors
- **AND** runnable examples execute successfully

### Requirement: README Documentation
Each crate's README.md SHALL:
- Mirror the crate-level docs for consistency
- Include installation instructions
- Show the primary use cases with examples
- Link to docs.rs for full API documentation
- Include any platform or browser requirements

#### Scenario: README matches crate docs
- **GIVEN** a user reads a crate's README on GitHub
- **WHEN** they compare it to docs.rs
- **THEN** the key information is consistent
- **AND** examples work as documented

### Requirement: Context7 Configuration
The `context7.json` file SHALL be configured to:
- Exclude test files and internal modules from indexing
- Include rules/best practices for AI coding assistants
- Have an accurate project title and description
- Optimize for maximum useful snippet extraction

#### Scenario: Context7 excludes test code
- **GIVEN** Context7 indexes the repository
- **WHEN** the exclusion patterns are applied
- **THEN** test files are not indexed as documentation
- **AND** only public API documentation is included

#### Scenario: Context7 provides usage rules
- **GIVEN** an AI assistant queries Context7 for Viewpoint docs
- **WHEN** Context7 provides documentation context
- **THEN** the rules field provides best practices
- **AND** the AI understands idioms like `js!` macro usage
