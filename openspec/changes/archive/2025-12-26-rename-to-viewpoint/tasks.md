# Tasks: Rename to Viewpoint

## 1. Directory Structure

- [x] 1.1 Rename `crates/rustright-cdp` to `crates/viewpoint-cdp`
- [x] 1.2 Rename `crates/rustright-core` to `crates/viewpoint-core`
- [x] 1.3 Rename `crates/rustright-test` to `crates/viewpoint-test`
- [x] 1.4 Rename `crates/rustright-test-macros` to `crates/viewpoint-test-macros`
- [x] 1.5 Remove duplicate `crates/viewpoint-test-macros` directory if exists (artifact from partial rename)

## 2. Cargo Configuration

- [x] 2.1 Update workspace `Cargo.toml` member paths
- [x] 2.2 Update `crates/viewpoint-cdp/Cargo.toml` (package name)
- [x] 2.3 Update `crates/viewpoint-core/Cargo.toml` (package name and dependencies)
- [x] 2.4 Update `crates/viewpoint-test/Cargo.toml` (package name and dependencies)
- [x] 2.5 Update `crates/viewpoint-test-macros/Cargo.toml` (package name)

## 3. Source Code Updates

- [x] 3.1 Update all `use rustright_cdp` to `use viewpoint_cdp` in source files
- [x] 3.2 Update all `use rustright_core` to `use viewpoint_core` in source files
- [x] 3.3 Update all `use rustright_test` to `use viewpoint_test` in source files
- [x] 3.4 Update all `use rustright_test_macros` to `use viewpoint_test_macros` in source files
- [x] 3.5 Update crate-level doc comments referencing "rustright"
- [x] 3.6 Update `extern crate` declarations if any

## 4. Documentation Updates

- [x] 4.1 Update `README.md` - replace all "RustRight/rustright" with "Viewpoint/viewpoint"
- [x] 4.2 Update `openspec/project.md` - update project name and all references

## 5. Test and Example Files

- [x] 5.1 Update imports in `crates/viewpoint-cdp/tests/`
- [x] 5.2 Update imports in `crates/viewpoint-core/tests/` and `examples/`
- [x] 5.3 Update imports in `crates/viewpoint-test/tests/` and `examples/`

## 6. OpenSpec Updates

- [x] 6.1 Update spec files in `openspec/specs/` that reference "RustRight"

## 7. Verification

- [x] 7.1 Run `cargo clean` to remove old build artifacts
- [x] 7.2 Run `cargo build --workspace` to verify compilation
- [x] 7.3 Run `cargo test --workspace` to verify tests pass
- [x] 7.4 Run `cargo clippy --workspace` to check for any issues
- [x] 7.5 Verify no remaining "rustright" references with `rg -i rustright`

## Dependencies

- Tasks in section 1 must complete before section 2
- Tasks in sections 1-2 must complete before section 3
- All code changes (1-5) must complete before verification (7)
- Documentation updates (4, 6) can proceed in parallel with source updates (3, 5)

## Parallelizable Work

- Within section 1: All directory renames can happen in parallel
- Within section 3: All source file updates can happen in parallel
- Sections 4 and 6 can run in parallel with section 3
