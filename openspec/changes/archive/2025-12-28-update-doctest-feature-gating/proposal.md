# Change: Update doc tests to use integration feature flag

## Why
Doc tests that demonstrate browser automation APIs currently use `ignore` attribute, which means they are never executed. This creates risk of documentation examples becoming out of sync with the actual API. Other tests requiring Chromium use the `integration` feature flag convention, and doc tests should follow the same pattern.

## What Changes
- Replace ```` ```ignore ```` in doc tests with feature-gated doc tests that run when `--features integration` is passed
- Add proper setup/teardown boilerplate (hidden with `#` prefix) to make doc tests self-contained
- Doc tests will compile-check always but only execute with `cargo test --features integration`

## Impact
- Affected specs: `code-quality`
- Affected code: All doc tests in `crates/viewpoint-core/src/` and `crates/viewpoint-test/src/` that currently use `ignore`
- ~32 doc test blocks across 22 files need updating
