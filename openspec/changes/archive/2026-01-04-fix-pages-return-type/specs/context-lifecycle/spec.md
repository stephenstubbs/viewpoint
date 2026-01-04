## MODIFIED Requirements

### Requirement: List Context Pages

The system SHALL provide access to all pages in a browser context by storing and returning actual `Page` objects.

Implementation note: The internal storage changes from `Vec<PageInfo>` to `Vec<Page>` to support returning fully-functional Page instances.

#### Scenario: Get all pages returns Page objects

- **GIVEN** a browser context with multiple pages open
- **WHEN** `context.pages()` is called
- **THEN** a `Vec<Page>` is returned (not `Vec<PageInfo>`)
- **AND** each `Page` object is fully functional (can call `url()`, `click()`, etc.)

#### Scenario: Externally-opened pages included

- **GIVEN** a browser context
- **WHEN** a page is opened via `window.open()`, `target="_blank"`, or Ctrl+Click
- **THEN** the new page appears in `context.pages()`
- **AND** the returned `Page` object is fully functional
