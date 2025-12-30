# Change: Add User Data Directory Support

## Why

MCP servers and browser automation tools often need persistent browser profiles for:
- Preserving authentication state across sessions
- Maintaining browser settings and extensions
- Reusing cookies and localStorage between runs

Currently, `viewpoint-mcp` accepts a `--user-data-dir` CLI argument but cannot use it because `viewpoint-core`'s `BrowserBuilder` lacks user data directory support.

## What Changes

- Add `user_data_dir()` method to `BrowserBuilder`
- Pass `--user-data-dir=<path>` argument to Chromium when launching

## Impact

- Affected specs: `browser-connection`
- Affected code: `crates/viewpoint-core/src/browser/launcher/mod.rs`
- Enables `viewpoint-mcp` to fully utilize its existing `--user-data-dir` configuration
