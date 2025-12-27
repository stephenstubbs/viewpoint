# Design: Dialog & File Handling

## Context

Browser dialogs and file operations require special handling as they involve native browser UI that isn't part of the DOM.

## Goals

- Handle all JavaScript dialog types
- Support file downloads with save options
- Support file uploads via input elements and file chooser

## Decisions

### Decision 1: Dialog Auto-Dismiss

**Choice**: Auto-dismiss dialogs when no listener is registered.

**Rationale**:
- Prevents tests from hanging on unexpected dialogs
- Matches Playwright behavior
- Explicit listeners for tests that need to verify dialogs

### Decision 2: Download Behavior

**Choice**: Store downloads in temp directory, provide save_as for custom location.

```rust
let download = page.wait_for_download(async {
    page.locator("a.download").click().await?;
}).await?;

// Get temp path
let path = download.path().await?;

// Or save to specific location
download.save_as("./downloads/file.pdf").await?;
```

### Decision 3: File Upload API

**Choice**: Support both direct path setting and file chooser interception.

```rust
// Direct (recommended)
page.locator("input[type=file]")
    .set_input_files(vec!["file1.txt", "file2.txt"])
    .await?;

// Via file chooser (for complex cases)
page.on_filechooser(|chooser| async move {
    chooser.set_files(vec!["file.txt"]).await
});
page.locator("button.upload").click().await?;
```

## CDP Commands Required

| Feature | CDP Command | Domain |
|---------|-------------|--------|
| Dialog event | Page.javascriptDialogOpening | Page |
| Dialog action | Page.handleJavaScriptDialog | Page |
| Download start | Page.downloadWillBegin | Page |
| Download progress | Page.downloadProgress | Page |
| File chooser | Page.fileChooserOpened | Page |
| Set files | DOM.setFileInputFiles | DOM |

## Open Questions

None.
