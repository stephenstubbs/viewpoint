# Change: Rename project from RustRight to Viewpoint

## Why

The project needs a new name. "Viewpoint" better reflects the purpose of a browser automation and testing framework - providing a viewpoint into web application behavior. This is a comprehensive rename affecting all project artifacts.

## What Changes

- **BREAKING**: All crate names change from `rustright-*` to `viewpoint-*`
- **BREAKING**: All public API imports change (e.g., `use rustright_core::*` becomes `use viewpoint_core::*`)
- Crate directory names renamed
- All internal references, doc comments, and documentation updated
- Workspace Cargo.toml updated with new member paths
- README and project documentation updated
- OpenSpec project.md updated to reflect new name

## Impact

- Affected specs: All specs reference "RustRight" in documentation
- Affected code: All 4 crates and ~40 source files
- Affected docs: README.md, project.md, archived change proposals

### Crate Mapping

| Old Name | New Name |
|----------|----------|
| `rustright-cdp` | `viewpoint-cdp` |
| `rustright-core` | `viewpoint-core` |
| `rustright-test` | `viewpoint-test` |
| `rustright-test-macros` | `viewpoint-test-macros` |

### Import Mapping

| Old Import | New Import |
|------------|------------|
| `rustright_cdp` | `viewpoint_cdp` |
| `rustright_core` | `viewpoint_core` |
| `rustright_test` | `viewpoint_test` |
| `rustright_test_macros` | `viewpoint_test_macros` |
