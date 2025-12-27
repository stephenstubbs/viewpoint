# device-emulation Specification

## Purpose
TBD - created by archiving change add-emulation-features. Update Purpose after archive.
## Requirements
### Requirement: Device Descriptors

The system SHALL provide predefined device descriptors.

#### Scenario: Use iPhone descriptor

- **GIVEN** an iPhone device descriptor
- **WHEN** `browser.new_context().device(devices::IPHONE_13).build().await` is called
- **THEN** the context has iPhone viewport, user agent, and touch

#### Scenario: Use Android descriptor

- **GIVEN** a Pixel device descriptor
- **WHEN** the descriptor is used
- **THEN** the context emulates a Pixel phone

#### Scenario: List available devices

- **GIVEN** the devices module
- **WHEN** device descriptors are listed
- **THEN** common devices are available

### Requirement: Viewport Emulation

The system SHALL emulate device viewports.

#### Scenario: Set viewport size

- **GIVEN** a browser context
- **WHEN** `context.new_page().await` with device viewport
- **THEN** the page has the specified viewport

#### Scenario: Device scale factor

- **GIVEN** a device with scale factor 2
- **WHEN** the device is emulated
- **THEN** devicePixelRatio is 2

### Requirement: User Agent Emulation

The system SHALL emulate device user agents.

#### Scenario: Mobile user agent

- **GIVEN** a mobile device descriptor
- **WHEN** the device is emulated
- **THEN** requests have mobile user agent

#### Scenario: Custom user agent

- **GIVEN** context options with user_agent
- **WHEN** `browser.new_context().user_agent("Custom UA").build().await`
- **THEN** requests use the custom user agent

### Requirement: Touch Emulation

The system SHALL emulate touch capability.

#### Scenario: Enable touch

- **GIVEN** a device with has_touch: true
- **WHEN** the device is emulated
- **THEN** touch events are supported

#### Scenario: Mobile mode

- **GIVEN** a device with is_mobile: true
- **WHEN** the device is emulated
- **THEN** the browser operates in mobile mode

### Requirement: Locale and Timezone

The system SHALL emulate locale and timezone.

#### Scenario: Set locale

- **GIVEN** context options
- **WHEN** `browser.new_context().locale("fr-FR").build().await`
- **THEN** navigator.language returns "fr-FR"

#### Scenario: Set timezone

- **GIVEN** context options
- **WHEN** `browser.new_context().timezone_id("Europe/Paris").build().await`
- **THEN** Date objects use Paris timezone

### Requirement: Vision Deficiency

The system SHALL emulate vision deficiencies.

#### Scenario: Emulate color blindness

- **GIVEN** a page
- **WHEN** `page.emulate_vision_deficiency(VisionDeficiency::Deuteranopia).await`
- **THEN** colors are rendered as if viewed with deuteranopia

#### Scenario: Clear vision deficiency

- **GIVEN** vision deficiency is set
- **WHEN** `page.emulate_vision_deficiency(VisionDeficiency::None).await`
- **THEN** normal color vision is restored

