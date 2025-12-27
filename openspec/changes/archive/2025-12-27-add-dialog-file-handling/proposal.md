# Change: Add Dialog & File Handling

## Why

Web applications commonly use:
- **Dialogs**: Alert, confirm, prompt, and beforeunload dialogs
- **Downloads**: File downloads triggered by user actions
- **Uploads**: File input elements for uploading files

Testing these interactions is essential for complete coverage.

This is proposal 6 of 11 in the Playwright feature parity series (Chromium only).

## What Changes

### New Capabilities

1. **Dialog Handling** - Handle browser dialogs
   - `page.on('dialog')` - listen for dialogs
   - `dialog.accept()` - accept dialog
   - `dialog.dismiss()` - dismiss dialog
   - `dialog.message()` - get dialog message
   - `dialog.type()` - alert, confirm, prompt, beforeunload

2. **Download Handling** - Handle file downloads
   - `page.on('download')` - listen for downloads
   - `download.path()` - get downloaded file path
   - `download.save_as(path)` - save to specific location
   - `download.suggested_filename()` - get suggested name
   - `download.cancel()` - cancel download

3. **File Upload** - Upload files via input elements
   - `locator.set_input_files(paths)` - set file input
   - `page.on('filechooser')` - intercept file chooser
   - Support multiple files
   - Support file buffers (in-memory content)

4. **Locator Handlers** - Handle overlay elements that interfere with actions
   - `page.add_locator_handler(locator, handler)` - register handler for overlay
   - `page.remove_locator_handler(locator)` - remove handler
   - Handler runs automatically when overlay blocks actions
   - Useful for cookie banners, notification popups, etc.

## Roadmap Position

| # | Proposal | Status |
|---|----------|--------|
| 1-5 | High Priority | Complete |
| **6** | **Dialog & File Handling** (this) | **Current** |
| 7-11 | Medium Priority | Pending |

## Impact

- **New specs**: `dialogs`, `downloads`, `file-uploads`
- **Affected code**: 
  - `viewpoint-core/src/page/` - dialog, download, filechooser types
  - `viewpoint-cdp/` - Page.javascriptDialogOpening, Page.downloadWillBegin
- **Breaking changes**: None
- **Dependencies**: Proposal 1 (page operations)
