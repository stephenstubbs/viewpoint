# Change: Update All Dependencies to Latest Versions

## Why
Keep dependencies current to benefit from bug fixes, security patches, and performance improvements. Outdated dependencies accumulate technical debt and may introduce compatibility issues over time.

## What Changes
- Update all workspace dependencies in root `Cargo.toml` to latest compatible versions
- Run full test suite to verify no breaking changes
- Update `Cargo.lock` with resolved versions

### Dependencies to Update
| Category | Crates |
|----------|--------|
| Async runtime | tokio, tokio-tungstenite, futures-util |
| Serialization | serde, serde_json |
| Error handling | thiserror |
| Logging | tracing, tracing-subscriber |
| HTTP | reqwest, url |
| Proc macros | proc-macro2, quote, syn |
| JavaScript parsing | swc_common, swc_ecma_ast, swc_ecma_parser |
| Utilities | base64, bytes, chrono, glob, parking_lot, regex, uuid, zip |
| Dev/test | tempfile, trybuild |

## Impact
- Affected specs: code-quality (dependency management)
- Affected code: All crates (transitive dependency updates)
- Risk: Low for semver-compatible updates; SWC crates may require code changes if major versions changed
