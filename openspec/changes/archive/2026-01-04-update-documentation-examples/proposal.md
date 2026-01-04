# Change: Update Documentation with Comprehensive Examples

## Why
The project has grown significantly but documentation has not kept pace. While basic README files and crate-level docs exist, many public APIs lack examples, some crates are missing READMEs entirely (viewpoint-js-core), and the existing documentation spec needs updating to reflect current project structure and standards.

## What Changes
- Comprehensive audit of all documentation against spec requirements
- Add missing README.md files (viewpoint-js-core)
- Update all lib.rs crate-level documentation with complete examples
- Add doc comments with examples to all public types, traits, and methods
- Enhance module-level documentation with usage patterns
- Ensure all doc examples use `compile` or `compile_fail` checks where possible
- Update documentation spec to reflect current best practices
- Update Context7 configuration for better AI assistant support

## Impact
- Affected specs: `documentation`
- Affected code: All 6 crates in workspace
  - `viewpoint-core` - Primary browser automation API
  - `viewpoint-test` - Test framework with assertions
  - `viewpoint-cdp` - Low-level CDP protocol
  - `viewpoint-js` - JavaScript macro
  - `viewpoint-js-core` - JavaScript utilities (missing README)
  - `viewpoint-test-macros` - Proc macros
- No breaking changes - documentation only
