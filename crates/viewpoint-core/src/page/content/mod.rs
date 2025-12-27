//! Content manipulation functionality.
//!
//! This module provides methods for manipulating page HTML content,
//! injecting scripts and styles.

use std::time::Duration;

use tracing::{debug, info, instrument};
use viewpoint_cdp::protocol::page::SetDocumentContentParams;
use viewpoint_cdp::protocol::runtime::EvaluateParams;

use crate::error::PageError;
use crate::wait::DocumentLoadState;

use super::Page;

/// Script type for injection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScriptType {
    /// Regular script (default).
    #[default]
    Script,
    /// ES6 module.
    Module,
}

/// Builder for injecting script tags.
#[derive(Debug)]
pub struct ScriptTagBuilder<'a> {
    page: &'a Page,
    url: Option<String>,
    content: Option<String>,
    script_type: ScriptType,
}

impl<'a> ScriptTagBuilder<'a> {
    /// Create a new script tag builder.
    pub(crate) fn new(page: &'a Page) -> Self {
        Self {
            page,
            url: None,
            content: None,
            script_type: ScriptType::default(),
        }
    }

    /// Set the script URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the script content.
    #[must_use]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the script type.
    #[must_use]
    pub fn script_type(mut self, script_type: ScriptType) -> Self {
        self.script_type = script_type;
        self
    }

    /// Inject the script tag into the page.
    ///
    /// # Errors
    ///
    /// Returns an error if the injection fails.
    #[instrument(level = "debug", skip(self), fields(has_url = self.url.is_some(), has_content = self.content.is_some()))]
    pub async fn inject(self) -> Result<(), PageError> {
        if self.page.is_closed() {
            return Err(PageError::Closed);
        }

        let script_js = if let Some(url) = self.url {
            format!(
                r"
                new Promise((resolve, reject) => {{
                    const script = document.createElement('script');
                    script.src = '{}';
                    script.setAttribute('type', '{}');
                    script.onload = resolve;
                    script.onerror = reject;
                    document.head.appendChild(script);
                }})
                ",
                url.replace('\'', "\\'"),
                if self.script_type == ScriptType::Module { "module" } else { "text/javascript" }
            )
        } else if let Some(content) = self.content {
            format!(
                r"
                (() => {{
                    const script = document.createElement('script');
                    script.textContent = {};
                    {}
                    document.head.appendChild(script);
                }})()
                ",
                serde_json::to_string(&content).unwrap_or_default(),
                if self.script_type == ScriptType::Module { "script.type = 'module';" } else { "" }
            )
        } else {
            return Err(PageError::EvaluationFailed(
                "Either url or content must be provided".to_string(),
            ));
        };

        debug!("Injecting script tag");

        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: script_js,
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(false),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(true),
                }),
                Some(self.page.session_id()),
            )
            .await?;

        Ok(())
    }
}

/// Builder for injecting style tags.
#[derive(Debug)]
pub struct StyleTagBuilder<'a> {
    page: &'a Page,
    url: Option<String>,
    content: Option<String>,
}

impl<'a> StyleTagBuilder<'a> {
    /// Create a new style tag builder.
    pub(crate) fn new(page: &'a Page) -> Self {
        Self {
            page,
            url: None,
            content: None,
        }
    }

    /// Set the stylesheet URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the CSS content.
    #[must_use]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Inject the style into the page.
    ///
    /// # Errors
    ///
    /// Returns an error if the injection fails.
    #[instrument(level = "debug", skip(self), fields(has_url = self.url.is_some(), has_content = self.content.is_some()))]
    pub async fn inject(self) -> Result<(), PageError> {
        if self.page.is_closed() {
            return Err(PageError::Closed);
        }

        let style_js = if let Some(url) = self.url {
            format!(
                r"
                new Promise((resolve, reject) => {{
                    const link = document.createElement('link');
                    link.rel = 'stylesheet';
                    link.href = '{}';
                    link.onload = resolve;
                    link.onerror = reject;
                    document.head.appendChild(link);
                }})
                ",
                url.replace('\'', "\\'")
            )
        } else if let Some(content) = self.content {
            format!(
                r"
                (() => {{
                    const style = document.createElement('style');
                    style.textContent = {};
                    document.head.appendChild(style);
                }})()
                ",
                serde_json::to_string(&content).unwrap_or_default()
            )
        } else {
            return Err(PageError::EvaluationFailed(
                "Either url or content must be provided".to_string(),
            ));
        };

        debug!("Injecting style tag");

        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: style_js,
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(false),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(true),
                }),
                Some(self.page.session_id()),
            )
            .await?;

        Ok(())
    }
}

/// Builder for setting page content.
#[derive(Debug)]
pub struct SetContentBuilder<'a> {
    page: &'a Page,
    html: String,
    wait_until: DocumentLoadState,
    timeout: Duration,
}

impl<'a> SetContentBuilder<'a> {
    /// Create a new set content builder.
    pub(crate) fn new(page: &'a Page, html: String) -> Self {
        Self {
            page,
            html,
            wait_until: DocumentLoadState::Load,
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the load state to wait for.
    #[must_use]
    pub fn wait_until(mut self, state: DocumentLoadState) -> Self {
        self.wait_until = state;
        self
    }

    /// Set the timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the page content.
    ///
    /// # Errors
    ///
    /// Returns an error if setting content fails.
    #[instrument(level = "info", skip(self), fields(html_len = self.html.len(), wait_until = ?self.wait_until))]
    pub async fn set(self) -> Result<(), PageError> {
        if self.page.is_closed() {
            return Err(PageError::Closed);
        }

        info!("Setting page content");

        // Use Page.setDocumentContent
        self.page
            .connection()
            .send_command::<_, serde_json::Value>(
                "Page.setDocumentContent",
                Some(SetDocumentContentParams {
                    frame_id: self.page.frame_id().to_string(),
                    html: self.html,
                }),
                Some(self.page.session_id()),
            )
            .await?;

        // Wait for the specified load state
        // For setDocumentContent, the content is set synchronously,
        // but we may need to wait for any scripts/resources to load
        if self.wait_until != DocumentLoadState::Commit {
            // Small delay to allow the document to settle
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        info!("Page content set");
        Ok(())
    }
}

impl Page {
    /// Get the full HTML content of the page including the doctype.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// let html = page.content().await?;
    /// println!("Page HTML: {}", html);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the page is closed or evaluation fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn content(&self) -> Result<String, PageError> {
        if self.closed {
            return Err(PageError::Closed);
        }

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(EvaluateParams {
                    expression: r#"
                        (() => {
                            const doctype = document.doctype;
                            const doctypeString = doctype 
                                ? `<!DOCTYPE ${doctype.name}${doctype.publicId ? ` PUBLIC "${doctype.publicId}"` : ''}${doctype.systemId ? ` "${doctype.systemId}"` : ''}>`
                                : '';
                            return doctypeString + document.documentElement.outerHTML;
                        })()
                    "#.to_string(),
                    object_group: None,
                    include_command_line_api: None,
                    silent: Some(true),
                    context_id: None,
                    return_by_value: Some(true),
                    await_promise: Some(false),
                }),
                Some(&self.session_id),
            )
            .await?;

        result
            .result
            .value
            .and_then(|v| v.as_str().map(ToString::to_string))
            .ok_or_else(|| PageError::EvaluationFailed("Failed to get content".to_string()))
    }

    /// Set the page HTML content.
    ///
    /// Returns a builder for additional options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::DocumentLoadState;
    ///
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Set simple content
    /// page.set_content("<html><body>Hello</body></html>").set().await?;
    ///
    /// // Set content and wait for network idle
    /// page.set_content("<html><body><script src='app.js'></script></body></html>")
    ///     .wait_until(DocumentLoadState::NetworkIdle)
    ///     .set()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_content(&self, html: impl Into<String>) -> SetContentBuilder<'_> {
        SetContentBuilder::new(self, html.into())
    }

    /// Create a builder for injecting script tags.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Add script by URL
    /// page.add_script_tag().url("https://example.com/script.js").inject().await?;
    ///
    /// // Add inline script
    /// page.add_script_tag().content("console.log('Hello')").inject().await?;
    ///
    /// // Add ES6 module
    /// use viewpoint_core::page::ScriptType;
    /// page.add_script_tag()
    ///     .content("export const x = 1;")
    ///     .script_type(ScriptType::Module)
    ///     .inject()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_script_tag(&self) -> ScriptTagBuilder<'_> {
        ScriptTagBuilder::new(self)
    }

    /// Create a builder for injecting style tags.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example(page: viewpoint_core::Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Add stylesheet by URL
    /// page.add_style_tag().url("https://example.com/style.css").inject().await?;
    ///
    /// // Add inline CSS
    /// page.add_style_tag().content("body { background: red; }").inject().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_style_tag(&self) -> StyleTagBuilder<'_> {
        StyleTagBuilder::new(self)
    }
}
