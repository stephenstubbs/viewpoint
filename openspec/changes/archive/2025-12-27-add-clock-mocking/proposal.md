# Change: Add Clock Mocking

## Why

Testing time-dependent features requires controlling time:
- **Animations**: Test animation states at specific points
- **Timers**: Test setTimeout/setInterval behavior
- **Dates**: Test date-dependent logic (expiration, scheduling)
- **Rate limiting**: Test without waiting for real time

This is proposal 11 of 12 in the Playwright feature parity series.

## What Changes

### New Capabilities

1. **Clock Control** - Mock the passage of time
   - `context.clock()` - access clock API
   - `clock.install()` - install mocked clock
   - `clock.set_fixed_time(time)` - freeze time
   - `clock.set_system_time(time)` - set but let flow

2. **Time Advancement** - Control time passage
   - `clock.run_for(duration)` - advance by duration
   - `clock.pause_at(time)` - pause at specific time
   - `clock.resume()` - resume normal flow
   - `clock.fast_forward(duration)` - skip ahead

3. **Timer Control** - Run pending timers
   - `clock.run_all_timers()` - run all pending timers
   - `clock.run_to_last()` - run to last scheduled timer

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1-10 | Previous | Complete |
| **11** | **Clock Mocking** (this) | **Current** |
| 12 | Advanced Locators | Pending |

## Impact

- **New specs**: `clock`
- **Affected code**: 
  - `viewpoint-core/src/context/clock.rs` - Clock type
  - Uses CDP Runtime.evaluate to inject clock mocking
- **Breaking changes**: None
- **Dependencies**: Proposal 1 (JavaScript evaluation)
