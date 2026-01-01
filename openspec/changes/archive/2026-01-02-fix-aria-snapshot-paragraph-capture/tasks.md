# Tasks

## 1. Update ARIA Snapshot JavaScript

- [x] 1.1 Add `"p": "paragraph"` to `getImplicitRole` roleMap in `snapshot_with_refs.rs`
- [x] 1.2 Add `"p": "paragraph"` to `getImplicitRole` roleMap in `snapshot_basic.rs`
- [x] 1.3 Add `"paragraph"` to `nameFromContentRoles` array in `snapshot_with_refs.rs`
- [x] 1.4 Add `"paragraph"` to `nameFromContentRoles` array in `snapshot_basic.rs`

## 2. Add Test Coverage

- [x] 2.1 Add integration test for paragraph element capture in ARIA snapshot
- [x] 2.2 Add test for multiple paragraphs with different content
- [x] 2.3 Add test verifying paragraph text appears in YAML output

## 3. Validation

- [x] 3.1 Run `cargo test --workspace` (unit tests)
- [x] 3.2 Run `cargo test --workspace --features integration` (integration tests)
- [x] 3.3 Run `cargo clippy --workspace` (lints)
