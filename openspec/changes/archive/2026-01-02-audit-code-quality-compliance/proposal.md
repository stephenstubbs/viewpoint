# Change: Audit and Fix Code Quality Compliance

## Why

The codebase has accumulated technical debt in the form of formatting inconsistencies, clippy warnings/errors, and files exceeding the 500-line limit mandated by the code-quality specification. These issues need to be resolved to ensure the project meets its documented quality standards and passes CI checks.

## What Changes

### 1. Fix Formatting Issues
Run `cargo fmt --all` to fix import ordering and formatting across all crates.

### 2. Fix Clippy Errors (Blocking)
- **`viewpoint-js-core/tests/escape_tests.rs:177`**: Replace `3.14_f64` with a non-PI constant (e.g., `3.15_f64`) to avoid `approx_constant` error

### 3. Fix Clippy Warnings
- **`viewpoint-js-core/tests/escape_tests.rs`**: Remove unnecessary raw string hashes, use inlined format args
- **`viewpoint-js/src/parser/tests/mod.rs`**: Remove unnecessary raw string hashes
- **`viewpoint-js/examples/basic_usage.rs`**: Use inlined format args (13 instances)
- **`viewpoint-cdp/src/error/tests/mod.rs`**: Use inlined format args
- **`viewpoint-cdp/src/transport/tests/mod.rs`**: Replace closure with method reference

### 4. Refactor Oversized Files (>500 lines)
Source files requiring refactoring:
| File | Lines | Action |
|------|-------|--------|
| `page/aria_snapshot/mod.rs` | 777 | Extract ref resolution and node processing into submodules |
| `page/mod.rs` | 678 | Extract method groups into submodules |
| `network/websocket/mod.rs` | 619 | Extract event handling and message types |
| `browser/launcher/mod.rs` | 561 | Extract profile management and argument building |
| `browser/mod.rs` | 542 | Extract context management |
| `page/locator/mod.rs` | 510 | Extract builder patterns |

Test files requiring refactoring:
| File | Lines | Action |
|------|-------|--------|
| `browser_tests.rs` | 825 | Split into browser_launch_tests.rs, browser_context_tests.rs |
| `aria_snapshot_basic_tests.rs` | 689 | Split by feature area |

### 5. Fix Test Module Structure
- **`viewpoint-js/src/scanner/tests.rs`**: Convert to `tests/mod.rs` directory structure per project conventions

### 6. Fix Flaky Tests
The zombie process detection tests in `browser_tests.rs` are flaky due to timing issues:

| Test | Issue | Fix |
|------|-------|-----|
| `test_no_zombie_after_drop` | Drop handler waits only 3ms for process reaping, but test checks after 100ms. Race between child process cleanup and zombie detection. | Increase Drop retry attempts and delay (e.g., 10 attempts with 10ms delay = 100ms total) |
| `test_no_zombie_after_close` | Same timing sensitivity as above | Same fix applied via shared implementation |
| `test_no_zombie_when_process_dies_before_close` | Same timing sensitivity | Same fix |
| `test_context_close` | Fails when run in parallel with other tests due to resource contention | Add `#[serial]` attribute or increase isolation |

**Root Cause Analysis:**
- Chromium spawns multiple child processes (renderer, GPU, etc.)
- When the main process is killed, child processes may briefly become zombies
- The `ps` command used in tests may pick up these transient zombies from other parallel tests
- The Drop handler's 3ms timeout is insufficient for reliable process reaping

**Fix Strategy:**
1. Increase `kill_and_reap_sync` timeout to allow more time for process tree cleanup
2. Use `serial_test` crate for zombie tests to prevent interference
3. Add retry/tolerance logic in zombie counting to handle transient states

## Impact

- Affected specs: `code-quality`
- Affected crates:
  - `viewpoint-core` (formatting, refactoring)
  - `viewpoint-cdp` (clippy fixes)
  - `viewpoint-js` (clippy fixes, test structure)
  - `viewpoint-js-core` (clippy fixes)

## Validation

After implementation:
- `cargo fmt --all -- --check` passes with no output
- `cargo clippy --workspace --all-targets` passes with no errors/warnings
- `cargo clippy --workspace --all-targets --features integration` passes with no errors/warnings
- `cargo test --workspace` passes
- `cargo test --workspace --features integration` passes
- No source files exceed 500 lines
