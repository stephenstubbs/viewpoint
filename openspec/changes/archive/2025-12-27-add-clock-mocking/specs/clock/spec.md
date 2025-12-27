# Clock

## ADDED Requirements

### Requirement: Clock Installation

The system SHALL allow installing a mocked clock.

#### Scenario: Install clock

- **GIVEN** a browser context
- **WHEN** `context.clock().install().await` is called
- **THEN** time-related functions are mocked

#### Scenario: Clock affects Date

- **GIVEN** clock is installed
- **WHEN** JavaScript calls `new Date()`
- **THEN** the mocked time is returned

#### Scenario: Clock affects setTimeout

- **GIVEN** clock is installed
- **WHEN** JavaScript calls `setTimeout(fn, 1000)`
- **THEN** the timer doesn't fire until time is advanced

### Requirement: Fixed Time

The system SHALL support freezing time.

#### Scenario: Set fixed time

- **GIVEN** clock is installed
- **WHEN** `clock.set_fixed_time("2024-01-01T00:00:00Z").await` is called
- **THEN** Date.now() always returns that timestamp

#### Scenario: Time stays fixed

- **GIVEN** fixed time is set
- **WHEN** real time passes
- **THEN** Date.now() still returns the fixed time

### Requirement: System Time

The system SHALL support setting time that still flows.

#### Scenario: Set system time

- **GIVEN** clock is installed
- **WHEN** `clock.set_system_time("2024-01-01T00:00:00Z").await` is called
- **THEN** Date.now() starts from that timestamp and flows normally

### Requirement: Time Advancement

The system SHALL support advancing time.

#### Scenario: Run for duration

- **GIVEN** clock is installed and timers are scheduled
- **WHEN** `clock.run_for(Duration::from_secs(5)).await` is called
- **THEN** time advances 5 seconds and due timers fire

#### Scenario: Fast forward

- **GIVEN** clock is installed
- **WHEN** `clock.fast_forward(Duration::from_millis(500)).await` is called
- **THEN** time jumps forward 500ms

#### Scenario: Pause at time

- **GIVEN** clock is installed
- **WHEN** `clock.pause_at("2024-01-01T12:00:00Z").await` is called
- **THEN** time stops at noon

### Requirement: Timer Control

The system SHALL provide timer control.

#### Scenario: Run all timers

- **GIVEN** multiple timers are scheduled
- **WHEN** `clock.run_all_timers().await` is called
- **THEN** all timers fire in order

#### Scenario: Run to last timer

- **GIVEN** timers scheduled for various times
- **WHEN** `clock.run_to_last().await` is called
- **THEN** time advances to the last timer

#### Scenario: Pending timer count

- **GIVEN** timers are scheduled
- **WHEN** `clock.pending_timer_count()` is called
- **THEN** the number of pending timers is returned

### Requirement: Clock Resume

The system SHALL allow resuming normal time.

#### Scenario: Resume time flow

- **GIVEN** clock is paused
- **WHEN** `clock.resume().await` is called
- **THEN** time flows normally again

#### Scenario: Uninstall clock

- **GIVEN** clock is installed
- **WHEN** `clock.uninstall().await` is called
- **THEN** original time functions are restored
