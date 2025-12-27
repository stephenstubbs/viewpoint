# Page Operations

## ADDED Requirements

### Requirement: Page Screenshots

The system SHALL capture screenshots of pages with configurable options.

#### Scenario: Capture viewport screenshot

- **GIVEN** a page with content loaded
- **WHEN** `page.screenshot().capture().await` is called
- **THEN** a PNG image buffer of the current viewport is returned

#### Scenario: Capture full page screenshot

- **GIVEN** a page with scrollable content
- **WHEN** `page.screenshot().full_page(true).capture().await` is called
- **THEN** a screenshot of the entire scrollable page is returned

#### Scenario: Capture JPEG screenshot with quality

- **GIVEN** a page with content loaded
- **WHEN** `page.screenshot().format(Jpeg).quality(80).capture().await` is called
- **THEN** a JPEG image with 80% quality is returned

#### Scenario: Save screenshot to file

- **GIVEN** a page with content loaded
- **WHEN** `page.screenshot().path("screenshot.png").capture().await` is called
- **THEN** the screenshot is saved to the specified path
- **AND** the image buffer is returned

#### Scenario: Capture clipped region

- **GIVEN** a page with content loaded
- **WHEN** `page.screenshot().clip(x, y, width, height).capture().await` is called
- **THEN** only the specified region is captured

#### Scenario: Capture with element masking

- **GIVEN** a page with dynamic content
- **WHEN** `page.screenshot().mask(vec![locator]).capture().await` is called
- **THEN** the specified elements are covered with a mask color

#### Scenario: Capture with animations disabled

- **GIVEN** a page with CSS animations
- **WHEN** `page.screenshot().animations(Disabled).capture().await` is called
- **THEN** animations are stopped before capture

#### Scenario: Capture element screenshot

- **GIVEN** a page with a specific element
- **WHEN** `locator.screenshot().await` is called
- **THEN** only that element is captured in the screenshot

### Requirement: PDF Generation

The system SHALL generate PDF documents from pages.

#### Scenario: Generate default PDF

- **GIVEN** a page with content loaded
- **WHEN** `page.pdf().generate().await` is called
- **THEN** a PDF buffer with Letter format is returned

#### Scenario: Generate PDF with custom paper size

- **GIVEN** a page with content loaded
- **WHEN** `page.pdf().format(A4).generate().await` is called
- **THEN** a PDF with A4 dimensions is returned

#### Scenario: Generate landscape PDF

- **GIVEN** a page with wide content
- **WHEN** `page.pdf().landscape(true).generate().await` is called
- **THEN** a landscape-oriented PDF is returned

#### Scenario: Generate PDF with margins

- **GIVEN** a page with content loaded
- **WHEN** `page.pdf().margin(top, right, bottom, left).generate().await` is called
- **THEN** a PDF with specified margins is returned

#### Scenario: Generate PDF with header and footer

- **GIVEN** a page with content loaded
- **WHEN** `page.pdf().header_template(html).footer_template(html).generate().await` is called
- **THEN** a PDF with custom header and footer on each page is returned

#### Scenario: Generate PDF with page ranges

- **GIVEN** a page that generates multiple PDF pages
- **WHEN** `page.pdf().page_ranges("1-3, 5").generate().await` is called
- **THEN** only the specified pages are included

#### Scenario: Generate PDF with background graphics

- **GIVEN** a page with background colors and images
- **WHEN** `page.pdf().print_background(true).generate().await` is called
- **THEN** background graphics are included in the PDF

#### Scenario: Save PDF to file

- **GIVEN** a page with content loaded
- **WHEN** `page.pdf().path("document.pdf").generate().await` is called
- **THEN** the PDF is saved to the specified path

### Requirement: Page Content Access

The system SHALL provide access to page HTML content.

#### Scenario: Get full page content

- **GIVEN** a page with content loaded
- **WHEN** `page.content().await` is called
- **THEN** the full HTML including doctype is returned as a string

#### Scenario: Set page content

- **GIVEN** a page instance
- **WHEN** `page.set_content(html).await` is called
- **THEN** the page displays the provided HTML
- **AND** load events are fired appropriately

#### Scenario: Set content with wait option

- **GIVEN** a page instance
- **WHEN** `page.set_content(html).wait_until(NetworkIdle).await` is called
- **THEN** the method waits for the specified load state

### Requirement: Script and Style Injection

The system SHALL allow injecting scripts and styles into pages.

#### Scenario: Add script tag with URL

- **GIVEN** a page with content loaded
- **WHEN** `page.add_script_tag().url(script_url).inject().await` is called
- **THEN** a script tag with the URL is added to the page
- **AND** the method returns when the script loads

#### Scenario: Add script tag with content

- **GIVEN** a page with content loaded
- **WHEN** `page.add_script_tag().content(js_code).inject().await` is called
- **THEN** an inline script with the content is added

#### Scenario: Add ES module script

- **GIVEN** a page with content loaded
- **WHEN** `page.add_script_tag().content(code).script_type(Module).inject().await` is called
- **THEN** an ES6 module script is added

#### Scenario: Add style tag with URL

- **GIVEN** a page with content loaded
- **WHEN** `page.add_style_tag().url(css_url).inject().await` is called
- **THEN** a link tag with the stylesheet URL is added

#### Scenario: Add style tag with content

- **GIVEN** a page with content loaded
- **WHEN** `page.add_style_tag().content(css_code).inject().await` is called
- **THEN** an inline style tag with the content is added

### Requirement: Init Scripts

The system SHALL support scripts that run before page loads.

#### Scenario: Add init script from content

- **GIVEN** a browser context
- **WHEN** `context.add_init_script(script_content).await` is called
- **AND** a new page navigates to a URL
- **THEN** the script runs before any page scripts

#### Scenario: Add init script from file

- **GIVEN** a browser context
- **WHEN** `context.add_init_script_path(path).await` is called
- **AND** a new page navigates to a URL
- **THEN** the script from the file runs before any page scripts

#### Scenario: Init script persists across navigations

- **GIVEN** a context with an init script added
- **WHEN** a page navigates to different URLs
- **THEN** the init script runs before each page load

#### Scenario: Page-level init script

- **GIVEN** a page instance
- **WHEN** `page.add_init_script(script_content).await` is called
- **AND** the page navigates to a URL
- **THEN** the script runs before page scripts on that page only

### Requirement: Page State

The system SHALL provide access to page state properties.

#### Scenario: Get page title

- **GIVEN** a page with a title element
- **WHEN** `page.title().await` is called
- **THEN** the document title is returned

#### Scenario: Get page URL

- **GIVEN** a page that has navigated
- **WHEN** `page.url()` is called
- **THEN** the current URL is returned synchronously

#### Scenario: Get viewport size

- **GIVEN** a page with a configured viewport
- **WHEN** `page.viewport_size()` is called
- **THEN** the viewport dimensions are returned

#### Scenario: Set viewport size

- **GIVEN** a page instance
- **WHEN** `page.set_viewport_size(width, height).await` is called
- **THEN** the viewport is resized to the specified dimensions

#### Scenario: Check if page is closed

- **GIVEN** a page instance
- **WHEN** `page.is_closed()` is called
- **THEN** false is returned for open pages
- **AND** true is returned for closed pages

#### Scenario: Bring page to front

- **GIVEN** multiple pages in a context
- **WHEN** `page.bring_to_front().await` is called
- **THEN** the page becomes the active tab

### Requirement: Navigation History

The system SHALL support browser history navigation.

#### Scenario: Navigate back in history

- **GIVEN** a page that has navigated to multiple URLs
- **WHEN** `page.go_back().await` is called
- **THEN** the page navigates to the previous URL in history
- **AND** the response from that navigation is returned

#### Scenario: Navigate back with wait option

- **GIVEN** a page with navigation history
- **WHEN** `page.go_back().wait_until(NetworkIdle).await` is called
- **THEN** the method waits for the specified load state

#### Scenario: Navigate back with no history

- **GIVEN** a page with no back history
- **WHEN** `page.go_back().await` is called
- **THEN** None is returned (no navigation occurs)

#### Scenario: Navigate forward in history

- **GIVEN** a page that has navigated back
- **WHEN** `page.go_forward().await` is called
- **THEN** the page navigates forward in history
- **AND** the response from that navigation is returned

#### Scenario: Navigate forward with no history

- **GIVEN** a page with no forward history
- **WHEN** `page.go_forward().await` is called
- **THEN** None is returned (no navigation occurs)

#### Scenario: Reload page

- **GIVEN** a page with content loaded
- **WHEN** `page.reload().await` is called
- **THEN** the page is reloaded
- **AND** the response is returned

#### Scenario: Reload with wait option

- **GIVEN** a page with content loaded
- **WHEN** `page.reload().wait_until(DomContentLoaded).await` is called
- **THEN** the method waits for the specified load state

### Requirement: Popup Handling

The system SHALL handle popup windows opened by the page.

#### Scenario: Listen for popup

- **GIVEN** a page with a link that opens a popup
- **WHEN** `page.on('popup', handler)` is registered
- **AND** a popup is triggered
- **THEN** the handler receives the popup as a Page instance

#### Scenario: Wait for popup

- **GIVEN** a page with a link that opens a popup
- **WHEN** `page.wait_for_popup(action).await` is called
- **AND** the action triggers a popup
- **THEN** the popup Page is returned

#### Scenario: Popup inherits context

- **GIVEN** a page opens a popup
- **WHEN** the popup is accessed
- **THEN** the popup shares the same browser context as the opener

#### Scenario: Access popup opener

- **GIVEN** a popup page
- **WHEN** `popup.opener()` is called
- **THEN** the page that opened the popup is returned
