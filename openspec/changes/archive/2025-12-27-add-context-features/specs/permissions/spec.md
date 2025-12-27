# Permissions

## ADDED Requirements

### Requirement: Grant Permissions

The system SHALL allow granting browser permissions to a context.

#### Scenario: Grant single permission

- **GIVEN** a browser context
- **WHEN** `context.grant_permissions(vec!["geolocation"]).await` is called
- **THEN** geolocation permission is granted for all origins

#### Scenario: Grant multiple permissions

- **GIVEN** a browser context
- **WHEN** `context.grant_permissions(vec!["geolocation", "notifications"]).await` is called
- **THEN** both permissions are granted

#### Scenario: Grant permission for specific origin

- **GIVEN** a browser context
- **WHEN** `context.grant_permissions(vec!["camera"]).origin("https://example.com").await` is called
- **THEN** camera permission is granted only for that origin

#### Scenario: Geolocation permission

- **GIVEN** geolocation permission is granted
- **WHEN** a page calls navigator.geolocation.getCurrentPosition()
- **THEN** the permission prompt is skipped
- **AND** location is returned (if set)

#### Scenario: Notifications permission

- **GIVEN** notifications permission is granted
- **WHEN** a page checks Notification.permission
- **THEN** the value is "granted"

#### Scenario: Clipboard read permission

- **GIVEN** clipboard-read permission is granted
- **WHEN** a page calls navigator.clipboard.readText()
- **THEN** the clipboard content is accessible

#### Scenario: Clipboard write permission

- **GIVEN** clipboard-write permission is granted
- **WHEN** a page calls navigator.clipboard.writeText()
- **THEN** the write succeeds without prompt

#### Scenario: Camera permission

- **GIVEN** camera permission is granted
- **WHEN** a page calls navigator.mediaDevices.getUserMedia({ video: true })
- **THEN** video access is granted without prompt

#### Scenario: Microphone permission

- **GIVEN** microphone permission is granted
- **WHEN** a page calls navigator.mediaDevices.getUserMedia({ audio: true })
- **THEN** audio access is granted without prompt

### Requirement: Clear Permissions

The system SHALL allow resetting permissions to defaults.

#### Scenario: Clear all permissions

- **GIVEN** a context with granted permissions
- **WHEN** `context.clear_permissions().await` is called
- **THEN** all permission overrides are removed

#### Scenario: Permissions require prompt after clear

- **GIVEN** permissions were granted then cleared
- **WHEN** a page requests a permission
- **THEN** the default browser behavior applies

### Requirement: Permission Context Options

The system SHALL support setting permissions at context creation.

#### Scenario: Create context with permissions

- **GIVEN** permissions specified in context options
- **WHEN** `browser.new_context().permissions(vec!["geolocation"]).build().await` is called
- **THEN** the context starts with those permissions granted

#### Scenario: Permissions combined with geolocation

- **GIVEN** context created with geolocation permission and location
- **WHEN** a page requests geolocation
- **THEN** the mocked location is returned without prompt

### Requirement: Unsupported Permissions

The system SHALL handle unsupported permissions gracefully.

#### Scenario: Unsupported permission error

- **GIVEN** a browser context
- **WHEN** an unsupported permission is requested
- **THEN** an error is returned indicating the permission is not supported

#### Scenario: Permission support varies by browser

- **GIVEN** different browsers
- **WHEN** permissions are granted
- **THEN** only browser-supported permissions work
