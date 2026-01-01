# Tasks

## 1. Implementation

- [x] 1.1 Add `tempfile` dependency to `viewpoint-core/Cargo.toml`
- [x] 1.2 Create `UserDataDir` enum in `browser/launcher/mod.rs`
- [x] 1.3 Update `BrowserBuilder` to use `UserDataDir` instead of `Option<PathBuf>`
- [x] 1.4 Add `.user_data_dir_system()` method for explicit system profile
- [x] 1.5 Add `.user_data_dir_template_from(path)` method for template-based temp profiles
- [x] 1.6 Update `launch()` to create temp directory when `UserDataDir::Temp`
- [x] 1.7 Update `launch()` to copy template and create temp when `UserDataDir::TempFromTemplate`
- [x] 1.8 Store `TempDir` handle in `Browser` struct to ensure cleanup on drop
- [x] 1.9 Update existing `.user_data_dir(path)` to map to `UserDataDir::Persist`

## 2. Testing

- [x] 2.1 Add unit test for `UserDataDir::Temp` creating unique directories
- [x] 2.2 Add unit test for `UserDataDir::Persist` using specified path
- [x] 2.3 Add unit test for `UserDataDir::System` passing no `--user-data-dir` flag
- [x] 2.4 Add unit test for `UserDataDir::TempFromTemplate` copying template contents
- [x] 2.5 Add integration test for concurrent browser launches (no conflicts)
- [x] 2.6 Add integration test for temp directory cleanup on browser close
- [x] 2.7 Add integration test for template profile with extensions preserved
- [x] 2.8 Update existing `user_data_dir` integration tests

## 3. Documentation

- [x] 3.1 Update doc comments on `BrowserBuilder` methods
- [x] 3.2 Document extension loading via `--load-extension` arg
- [x] 3.3 Add migration note about default behavior change
