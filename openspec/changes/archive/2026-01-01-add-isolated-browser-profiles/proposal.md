# Change: Add Isolated Browser Profiles

## Why

When no `user_data_dir` is specified, Chromium uses `~/.config/chromium/` by default. This causes conflicts when:
1. A personal Chromium browser is already running (locks the profile)
2. Multiple automation sessions run concurrently (each needs isolated state)
3. A previous session crashed without cleanup (stale lock files)

Additionally, browser automation often needs:
4. Pre-configured profiles with extensions, settings, or cookies as a starting template

Browser automation tools need isolated, ephemeral profiles by default while supporting persistent profiles and template-based profiles when needed.

## What Changes

### User Data Directory Options

- Add `UserDataDir` enum with four variants:
  - `Temp` - Create unique temporary directory per session (new default)
  - `TempFromTemplate(PathBuf)` - Copy template profile to temp directory, clean up on close
  - `Persist(PathBuf)` - Use specified directory for persistent state
  - `System` - Use system default (`~/.config/chromium/`)
- Change `BrowserBuilder` to use `UserDataDir::Temp` by default
- Browser owns the temp directory and cleans it up on close/drop
- Existing `.user_data_dir(path)` API remains unchanged (maps to `Persist`)
- Add `.user_data_dir_template_from(path)` for template-based temp profiles
- Add `.user_data_dir_system()` for explicit system profile access

### Extension Loading (existing capability)

Extensions can already be loaded via the `.args()` method:

```rust
Browser::launch()
    .args(["--load-extension=/path/to/unpacked-extension"])
    .launch()
    .await?;
```

This works with all profile modes including `Temp`. Extensions must be unpacked (not `.crx` files). For extensions that require configuration (permissions, settings), use `.user_data_dir_template_from()` with a pre-configured template profile.

## Impact

- Affected specs: `browser-connection`
- Affected code: 
  - `crates/viewpoint-core/src/browser/launcher/mod.rs`
  - `crates/viewpoint-core/src/browser/mod.rs`
- **BREAKING**: Default behavior changes from system profile to isolated temp profile
  - Users relying on implicit system profile access must explicitly use `.user_data_dir_system()`
