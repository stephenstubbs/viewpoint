## 1. Implementation

- [x] 1.1 Add `user_data_dir: Option<PathBuf>` field to `BrowserBuilder`
- [x] 1.2 Add `user_data_dir()` builder method
- [x] 1.3 Pass `--user-data-dir=<path>` to Chromium in `launch()` when set
- [x] 1.4 Add integration test for user data directory persistence

## 2. Validation

- [x] 2.1 Verify cookies persist across browser restarts with same user data dir
- [x] 2.2 Run `cargo test --features integration` to confirm no regressions
- [x] 2.3 Run `cargo clippy` and `cargo fmt --check`
