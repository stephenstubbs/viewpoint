# Design: Rename to Viewpoint

## Context

This is a project-wide rename from "RustRight" to "Viewpoint". The rename affects the entire codebase including crate names, directory structure, imports, documentation, and specifications.

### Stakeholders
- All users of the library (breaking change to imports)
- Contributors (directory structure changes)

### Constraints
- Must maintain all functionality - purely cosmetic change
- Must update all references consistently to avoid broken imports
- Cargo.lock will be regenerated with new package names

## Goals / Non-Goals

### Goals
- Rename all crates from `rustright-*` to `viewpoint-*`
- Update all source code imports and references
- Update all documentation
- Maintain working build and tests after rename

### Non-Goals
- No functional changes to code
- No API design changes
- No new features or capabilities

## Decisions

### Decision: Direct rename approach
Perform a straightforward find-and-replace rename across all files rather than creating aliases or deprecation paths.

**Rationale**: This is a new project without external users yet. A clean break is simpler than maintaining backwards compatibility.

### Decision: Rename order
1. Rename crate directories first
2. Update Cargo.toml files
3. Update source code imports
4. Update documentation
5. Clean and rebuild

**Rationale**: Cargo needs valid directory paths before it can resolve dependencies. Source code updates depend on correct crate names being in place.

### Decision: Case handling
- Directory/crate names: kebab-case (`viewpoint-core`)
- Rust imports: snake_case (`viewpoint_core`)
- Documentation: Title case ("Viewpoint")

## Risks / Trade-offs

### Risk: Incomplete rename
Some references might be missed, causing build failures or documentation inconsistencies.

**Mitigation**: 
- Use comprehensive grep to find all occurrences
- Verify build passes after rename
- Run all tests

### Risk: Git history fragmentation
Directory renames may complicate git history tracking.

**Mitigation**: Not applicable - project uses jj (Jujutsu), which handles renames well.

## Migration Plan

1. **Preparation**: Ensure clean working state, all tests passing
2. **Directory renames**: Move `crates/rustright-*` to `crates/viewpoint-*`
3. **Cargo.toml updates**: Update workspace and crate manifests
4. **Source updates**: Replace all `rustright` references with `viewpoint`
5. **Documentation updates**: Update README, project.md, and specs
6. **Verification**: Build and test
7. **Cleanup**: Remove old build artifacts (`cargo clean`)

### Rollback
If issues arise, revert the commit and restore original names.

## Open Questions

None - this is a straightforward rename operation.
