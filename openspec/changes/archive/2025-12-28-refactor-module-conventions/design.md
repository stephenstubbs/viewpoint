# Design: Module Convention Refactoring

## Context

The `project.md` defines clear module structure conventions that the current codebase does not follow:

1. **Folder modules only** - Directories with `mod.rs`, not single `.rs` files
2. **No inline tests** - Tests in separate `tests/` folders, not `#[cfg(test)] mod tests` blocks
3. **Maximum 500 lines per file** - Refactor into smaller modules if exceeded

An audit of all 6 crates revealed **165+ violations** across the workspace.

## Goals

- Align codebase with `project.md` conventions
- Improve maintainability and contributor experience
- Create consistent, predictable module structure
- Zero changes to public API or behavior

## Non-Goals

- Changing functionality or behavior
- Reorganizing module hierarchy beyond convention compliance
- Adding new tests (only moving existing ones)

## Decisions

### Decision 1: Batch refactoring by crate, ordered by complexity

**What:** Refactor crates from simplest to most complex:
1. viewpoint-test-macros (1 file)
2. viewpoint-js-core (already mostly compliant)
3. viewpoint-js (3 files)
4. viewpoint-test (3 files + 1 inline test)
5. viewpoint-cdp (20 files + 4 inline tests)
6. viewpoint-core (102 files + 27 inline tests)

**Why:** 
- Builds confidence with simpler changes first
- Earlier crates serve as reference implementations
- Allows validation of approach before tackling core crate

**Alternatives considered:**
- Big bang (all at once) - Rejected: too risky, hard to review
- By violation type - Rejected: would touch same files multiple times

### Decision 2: Folder module structure pattern

**What:** Use this consistent pattern for all converted modules:

```
module_name/
├── mod.rs           # Public exports and main logic
└── tests/
    ├── mod.rs       # Test module root
    └── test_*.rs    # Individual test files if needed
```

**Why:**
- Matches `project.md` convention exactly
- Tests are co-located but separate
- Scales well for modules that need multiple test files

### Decision 3: Keys.rs decomposition strategy

**What:** Split `keyboard/keys.rs` (1092 lines) into:
- `keyboard/mod.rs` - Public API and Keyboard struct
- `keyboard/keys.rs` - Key enum definition (~300 lines)
- `keyboard/modifiers.rs` - Modifier handling (~100 lines)
- `keyboard/layout.rs` - Keyboard layout mappings (~500 lines)

**Why:**
- Maintains logical separation of concerns
- Each file stays well under 500 lines
- Key definitions are a natural boundary

**Alternatives considered:**
- Single compressed file - Rejected: still over 500 lines
- More granular split - Rejected: unnecessary complexity

### Decision 4: Test migration approach

**What:** For each inline test block:
1. Create `tests/mod.rs` in the module directory
2. Move test functions to test module
3. Add necessary imports (`use super::*;`)
4. Replace inline block with `#[cfg(test)] mod tests;`

**Why:**
- Minimal changes to test logic
- Maintains test coverage
- Follows `project.md` pattern exactly

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Breaking internal module paths | Thorough `cargo check` and `cargo test` after each crate |
| Merge conflicts with active changes | Coordinate with `enhance-integration-tests` change |
| Missing test imports after move | Run tests frequently during migration |
| Large PR size | Split into multiple PRs, one per crate |

## Migration Plan

### Phase 1: Small crates (Low risk)
1. viewpoint-test-macros
2. viewpoint-js-core (verify compliance)
3. viewpoint-js
4. viewpoint-test

### Phase 2: Protocol crate (Medium risk)
5. viewpoint-cdp

### Phase 3: Core crate (Higher complexity)
6. viewpoint-core (break into sub-phases by subdirectory)
   - api/
   - browser/
   - context/
   - devices/
   - network/
   - page/
   - wait/

### Rollback
Each phase is a separate commit. If issues arise:
1. Revert the problematic commit
2. Investigate and fix
3. Re-apply

## Open Questions

1. **viewpoint-js-core exception?** - The crate is only 352 lines total in `lib.rs`. Should we require folder module structure for such minimal crates, or add an exception to `project.md`?

2. **Test file naming** - Should extracted test files be named `mod.rs` only, or use descriptive names like `unit_tests.rs`? Current convention suggests `mod.rs` but doesn't prohibit descriptive names.
