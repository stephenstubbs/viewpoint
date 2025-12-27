# Change: Add Core Page Operations

## Why

Viewpoint currently supports basic navigation and element interactions, but lacks essential page-level operations that Playwright provides. These operations are fundamental for:
- Visual regression testing (screenshots)
- Document generation (PDFs)
- Custom JavaScript execution in page context
- Direct HTML content manipulation
- Page state inspection

This is the first in a series of proposals to achieve full Playwright feature parity.

## What Changes

### New Capabilities

1. **Screenshots** - Capture full page, viewport, or element screenshots
   - PNG and JPEG formats
   - Full page scrolling capture
   - Element-specific screenshots
   - Clipping regions
   - Mask sensitive elements
   - Animation handling

2. **PDF Generation** - Generate PDF documents from pages
   - Configurable paper size and orientation
   - Headers and footers
   - Page ranges
   - Scale and margins
   - Print backgrounds

3. **JavaScript Evaluation** - Execute JavaScript in page context
   - `evaluate()` - run script and return serializable result
   - `evaluateHandle()` - return JSHandle for complex objects
   - Pass arguments to scripts
   - Access DOM elements

4. **Content Manipulation** - Direct HTML/DOM operations
   - `setContent()` - set page HTML directly
   - `content()` - get full page HTML including doctype
   - `addScriptTag()` - inject script tags
   - `addStyleTag()` - inject style tags
   - `addInitScript()` - run scripts before page loads

5. **Page State** - Query page properties
   - `title()` - get page title
   - `url()` - get current URL
   - `viewportSize()` - get viewport dimensions
   - `setViewportSize()` - resize viewport
   - `isClosed()` - check if page is closed
   - `bringToFront()` - activate tab

6. **Navigation History** - Browser history operations
   - `goBack()` - navigate back in history
   - `goForward()` - navigate forward in history
   - `reload()` - reload current page

7. **Popup Handling** - Handle popup windows
   - `page.on('popup')` - listen for new popup windows
   - Access popup as Page instance
   - Wait for popup with `page.wait_for_popup()`

## Roadmap

This proposal is **Part 1 of 11** in the Playwright feature parity series (Chromium only):

| # | Proposal | Priority | Dependencies |
|---|----------|----------|--------------|
| 1 | **Core Page Operations** (this) | High | None |
| 2 | Network Interception | High | None |
| 3 | Input Devices | High | None |
| 4 | Browser Context Features | High | None |
| 5 | Frame Support | Medium | 1 |
| 6 | Dialog & File Handling | Medium | 1 |
| 7 | Tracing & Debugging | Medium | 1-4 |
| 8 | Emulation Features | Medium | 4 |
| 9 | API Testing | Medium | 2 |
| 10 | Clock Mocking | Medium | 1 |
| 11 | Advanced Locators & Assertions | Medium | 1 |

## Impact

- **Affected specs**: New `page-operations` and `javascript-evaluation` capabilities
- **Affected code**: 
  - `viewpoint-core/src/page/` - add screenshot, PDF, content methods
  - `viewpoint-cdp/src/protocol/` - add Page.captureScreenshot, Page.printToPDF, Runtime.evaluate CDP commands
- **Breaking changes**: None
