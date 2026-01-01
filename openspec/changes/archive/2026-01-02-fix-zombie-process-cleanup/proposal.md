# Change: Fix Zombie Process Cleanup on Browser Drop

## Why

When a browser process dies unexpectedly (crash, killed externally, or timeout), the `Browser` struct's `Drop` implementation calls `child.kill()` but never calls `child.wait()` to reap the zombie process. This leaves `<defunct>` chromium processes accumulating until the parent process (the application using viewpoint-core) exits.

On Unix systems, when a child process exits, it becomes a zombie (defunct) process until the parent calls `wait()` or `waitpid()` to collect its exit status. The current `Drop` implementation:
1. Tries to `kill()` the process (which may already be dead)
2. Never calls `wait()` to reap the zombie

This is problematic for long-running applications like MCP servers that may experience multiple browser crashes over their lifetime.

## What Changes

1. **Update `Browser::close()` method** to call `wait()` after `kill()` to properly reap the child process.

2. **Update `Browser::Drop` implementation** to attempt to reap the child process. Since `Drop` cannot be async, we need to use `try_wait()` (non-blocking) to avoid blocking the runtime.

3. **Add helper method** for synchronous process cleanup that can be used from both `close()` and `Drop`.

## Design Considerations

**Why `try_wait()` in Drop?**
- `Drop` cannot be async, so we can't use `child.wait().await`
- Using blocking `wait()` in Drop could block the async runtime
- `try_wait()` is non-blocking and will succeed immediately if the process has already exited
- For processes still running, `kill()` + `try_wait()` with a small retry may be needed

**Kill signal timing:**
- After `kill()`, the process may take a moment to actually exit
- A small delay or retry loop may be needed before `try_wait()` succeeds
- Alternative: use `SIGKILL` (force kill) which is nearly instant

**Fallback behavior:**
- If `try_wait()` fails (process hasn't exited yet), log a warning
- The zombie will eventually be cleaned up when the parent process exits
- This is acceptable as a fallback since it's rare and temporary

## Impact

- Affected specs: `browser-connection` (Browser Lifecycle requirement)
- Affected code:
  - `crates/viewpoint-core/src/browser/mod.rs` - `Browser::close()` and `Drop` impl

## Alternative Considered

**Spawning a reaper thread:**
- Could spawn a dedicated thread to call blocking `wait()`
- Adds complexity and potential resource leaks
- Overkill for this use case since `try_wait()` should work for killed processes

**Using `tokio::process::Child` instead of `std::process::Child`:**
- Would allow async `wait()` but requires significant refactoring
- The launcher currently uses std::process for synchronous spawning
- Could be considered for a future refactoring but out of scope here
