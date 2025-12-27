# Media Emulation

## ADDED Requirements

### Requirement: Media Type

The system SHALL emulate CSS media types.

#### Scenario: Emulate print media

- **GIVEN** a page
- **WHEN** `page.emulate_media().media(MediaType::Print).apply().await`
- **THEN** CSS @media print rules apply

#### Scenario: Emulate screen media

- **GIVEN** a page in print mode
- **WHEN** `page.emulate_media().media(MediaType::Screen).apply().await`
- **THEN** CSS @media screen rules apply

### Requirement: Color Scheme

The system SHALL emulate color scheme preferences.

#### Scenario: Emulate dark mode

- **GIVEN** a page
- **WHEN** `page.emulate_media().color_scheme(ColorScheme::Dark).apply().await`
- **THEN** prefers-color-scheme: dark matches

#### Scenario: Emulate light mode

- **GIVEN** a page
- **WHEN** `page.emulate_media().color_scheme(ColorScheme::Light).apply().await`
- **THEN** prefers-color-scheme: light matches

#### Scenario: No preference

- **GIVEN** a page
- **WHEN** `page.emulate_media().color_scheme(ColorScheme::NoPreference).apply().await`
- **THEN** no color scheme preference is set

### Requirement: Reduced Motion

The system SHALL emulate reduced motion preference.

#### Scenario: Emulate reduced motion

- **GIVEN** a page
- **WHEN** `page.emulate_media().reduced_motion(ReducedMotion::Reduce).apply().await`
- **THEN** prefers-reduced-motion: reduce matches

#### Scenario: No motion preference

- **GIVEN** a page with reduced motion
- **WHEN** `page.emulate_media().reduced_motion(ReducedMotion::NoPreference).apply().await`
- **THEN** animations are not reduced

### Requirement: Forced Colors

The system SHALL emulate forced colors mode.

#### Scenario: Emulate forced colors

- **GIVEN** a page
- **WHEN** `page.emulate_media().forced_colors(ForcedColors::Active).apply().await`
- **THEN** forced-colors: active matches

### Requirement: Combined Media Emulation

The system SHALL support combining multiple media settings.

#### Scenario: Dark mode with reduced motion

- **GIVEN** a page
- **WHEN** color scheme dark AND reduced motion are set
- **THEN** both media queries match

#### Scenario: Clear all media emulation

- **GIVEN** a page with media emulation
- **WHEN** `page.emulate_media().clear().await`
- **THEN** all emulation is removed
