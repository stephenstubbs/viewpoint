# Tasks: Add Emulation Features

## 1. CDP Protocol Extensions

- [x] 1.1 Add `Emulation.setDeviceMetricsOverride` command
- [x] 1.2 Add `Emulation.setUserAgentOverride` command
- [x] 1.3 Add `Emulation.setTouchEmulationEnabled` command
- [x] 1.4 Add `Emulation.setEmulatedMedia` command
- [x] 1.5 Add `Emulation.setTimezoneOverride` command
- [x] 1.6 Add `Emulation.setLocaleOverride` command
- [x] 1.7 Add `Emulation.setEmulatedVisionDeficiency` command

## 2. Device Descriptors

- [x] 2.1 Create `DeviceDescriptor` struct
- [x] 2.2 Create `devices` module with constants
- [x] 2.3 Add iPhone device descriptors
- [x] 2.4 Add Android device descriptors
- [x] 2.5 Add iPad/tablet descriptors
- [x] 2.6 Add desktop descriptors

## 3. Context Device Emulation

- [x] 3.1 Add `device()` to context builder
- [x] 3.2 Add `user_agent()` to context builder
- [x] 3.3 Add `viewport()` to context builder
- [x] 3.4 Implement device scale factor
- [x] 3.5 Implement touch emulation setup
- [x] 3.6 Implement mobile mode

## 4. Locale and Timezone

- [x] 4.1 Add `locale()` to context builder
- [x] 4.2 Add `timezone_id()` to context builder
- [x] 4.3 Implement locale override
- [x] 4.4 Implement timezone override

## 5. Media Emulation

- [x] 5.1 Create `EmulateMediaBuilder`
- [x] 5.2 Implement `media()` for media type
- [x] 5.3 Implement `color_scheme()`
- [x] 5.4 Implement `reduced_motion()`
- [x] 5.5 Implement `forced_colors()`
- [x] 5.6 Implement `clear()` to reset

## 6. Vision Deficiency

- [x] 6.1 Create `VisionDeficiency` enum
- [x] 6.2 Implement `page.emulate_vision_deficiency()`
- [x] 6.3 Support all deficiency types

## 7. Testing

- [x] 7.1 Add tests for device emulation
- [x] 7.2 Add tests for viewport/scale
- [x] 7.3 Add tests for locale/timezone
- [x] 7.4 Add tests for media emulation
- [x] 7.5 Add tests for vision deficiency

## 8. Documentation

- [x] 8.1 Document device descriptors
- [x] 8.2 Document media emulation
- [x] 8.3 Document locale/timezone
- [x] 8.4 Add mobile testing examples

## Dependencies

- CDP extensions (1.x) first
- Device descriptors (2) before context emulation (3)
- Context emulation (3-4) and Media (5-6) are independent

## Parallelizable Work

- Device emulation and Media emulation are independent
