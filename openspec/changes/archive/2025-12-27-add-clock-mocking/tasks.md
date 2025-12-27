# Tasks: Add Clock Mocking

## 1. Clock Mocking Library

- [x] 1.1 Create clock mocking JavaScript code
- [x] 1.2 Mock Date constructor and Date.now()
- [x] 1.3 Mock setTimeout and setInterval
- [x] 1.4 Mock clearTimeout and clearInterval
- [x] 1.5 Mock requestAnimationFrame
- [x] 1.6 Mock cancelAnimationFrame
- [x] 1.7 Mock performance.now()

## 2. Clock Type Implementation

- [x] 2.1 Create `Clock` struct
- [x] 2.2 Implement `page.clock()` accessor
- [x] 2.3 Implement `clock.install()`
- [x] 2.4 Implement `clock.uninstall()`

## 3. Time Setting

- [x] 3.1 Implement `set_fixed_time()` with ISO string
- [x] 3.2 Implement `set_fixed_time()` with timestamp
- [x] 3.3 Implement `set_system_time()`
- [x] 3.4 Support TimeValue type (ISO string or timestamp)

## 4. Time Advancement

- [x] 4.1 Implement `run_for(duration)`
- [x] 4.2 Implement `fast_forward(duration)`
- [x] 4.3 Implement `pause_at(time)`
- [x] 4.4 Implement `resume()`

## 5. Timer Control

- [x] 5.1 Implement `run_all_timers()`
- [x] 5.2 Implement `run_to_last()`
- [x] 5.3 Implement `pending_timer_count()`
- [x] 5.4 Track timer queue (in JS library)

## 6. Injection

- [x] 6.1 Inject clock library via Runtime.evaluate
- [x] 6.2 Handle page navigation (library re-injected on install)
- [x] 6.3 Clock is per-page (accessible via page.clock())

## 7. Testing

- [x] 7.1 Add tests for Date mocking
- [x] 7.2 Add tests for timer mocking
- [x] 7.3 Add tests for time advancement
- [x] 7.4 Add tests for timer control
- [x] 7.5 Add tests for pause/resume

## 8. Documentation

- [x] 8.1 Document clock API (rustdoc)
- [x] 8.2 Document time testing patterns (in code comments)
- [x] 8.3 Add examples in Clock struct documentation

## Dependencies

- Clock library (1) must be done first
- Clock type (2) depends on (1)
- Time setting (3) and advancement (4) depend on (2)
- Timer control (5) depends on (1-2)

## Parallelizable Work

- Time setting (3) and Time advancement (4) can parallel
- Timer control (5) is independent once (2) is done

## Implementation Notes

- Clock is implemented per-page via `page.clock()`
- JavaScript clock mocking library is injected via Runtime.evaluate
- TimeValue enum supports both ISO strings and Unix timestamps
- All timer callbacks are tracked in a Map for proper execution order
- requestAnimationFrame callbacks are tracked separately
- performance.now() is mocked relative to the mocked start time
