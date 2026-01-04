# Change: Audit Project Convention Compliance

## Why
The project has established conventions in `project.md` and `code-quality` spec that ensure maintainability, consistency, and code quality. A systematic audit is needed to identify any deviations from these conventions, create a prioritized remediation plan, and fix all violations.

## What Changes
- Systematic audit of all source files against `project.md` conventions
- Identification of files exceeding 500-line limit
- Verification of folder module structure compliance
- Check for inline test blocks vs external `tests/` directories
- Verification of error type naming conventions
- Audit of JavaScript code usage (should use `js!` macro)
- Report generation with categorized findings
- **Remediation of all findings:**
  - Extract inline test block to external `tests/` module
  - Split oversized integration test files into smaller, focused modules
  - Enhance shared test utilities

## Impact
- Affected specs: `code-quality`
- Affected code: All crates in workspace
- Files modified:
  - `viewpoint-cdp/src/protocol/target_domain/mod.rs` - removed inline tests
  - `viewpoint-cdp/src/protocol/target_domain/tests/mod.rs` - new external test module
  - `viewpoint-core/tests/common/mod.rs` - enhanced with shared helpers
  - `viewpoint-core/tests/locator_ref_*.rs` - split from single 792-line file
  - `viewpoint-core/tests/aria_snapshot_ref_*.rs` - split from single 638-line file
  - `viewpoint-core/tests/context_*.rs` - split from single 522-line file
