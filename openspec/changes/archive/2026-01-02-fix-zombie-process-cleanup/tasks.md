# Tasks

## 1. Update Browser Close Method

- [x] 1.1 Modify `Browser::close()` to call `wait()` after `kill()` to reap the child process
- [x] 1.2 Handle the case where kill fails (process already dead) gracefully
- [x] 1.3 Add error handling for wait failures

## 2. Update Browser Drop Implementation

- [x] 2.1 Add `try_wait()` call after `kill()` in `Drop` implementation
- [x] 2.2 Add small retry loop (e.g., 3 attempts with 1ms delay) if `try_wait()` returns `Ok(None)`
- [x] 2.3 Log warning if process cannot be reaped in Drop (fallback behavior)

## 3. Add Helper Method

- [x] 3.1 Extract common kill+reap logic into a helper method usable by both `close()` and `Drop`
- [x] 3.2 Document the method's behavior for sync vs async contexts

## 4. Testing

- [x] 4.1 Add test that verifies no zombie processes after browser close
- [x] 4.2 Add test that verifies no zombie processes after browser drop
- [x] 4.3 Add test for process that dies before explicit close

## 5. Validation

- [x] 5.1 Run `cargo test --workspace`
- [x] 5.2 Run `cargo clippy --workspace`
- [ ] 5.3 Manual verification: launch browser, kill chromium process, verify no zombies after drop
