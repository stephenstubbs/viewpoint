//! Popup window handling.
//!
//! This module provides functionality for detecting and handling popup windows
//! that are opened by JavaScript code (e.g., via `window.open()`).

// Allow dead code for popup scaffolding (spec: page-operations)

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{oneshot, RwLock};
use tracing::debug;

use viewpoint_cdp::CdpConnection;

use crate::error::PageError;
use crate::page::Page;

/// Type alias for the popup event handler function.
pub type PopupEventHandler = Box<
    dyn Fn(Page) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Manager for page-level popup events.
pub struct PopupManager {
    /// Popup event handler.
    handler: RwLock<Option<PopupEventHandler>>,
    /// Target ID of the owning page.
    target_id: String,
    /// CDP connection for subscribing to events.
    connection: Arc<CdpConnection>,
    /// Session ID of the owning page.
    session_id: String,
}

impl PopupManager {
    /// Create a new popup manager for a page.
    pub fn new(connection: Arc<CdpConnection>, session_id: String, target_id: String) -> Self {
        Self {
            handler: RwLock::new(None),
            target_id,
            connection,
            session_id,
        }
    }

    /// Set a handler for popup events.
    pub async fn set_handler<F, Fut>(&self, handler: F)
    where
        F: Fn(Page) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed_handler: PopupEventHandler = Box::new(move |page| {
            Box::pin(handler(page))
        });
        let mut h = self.handler.write().await;
        *h = Some(boxed_handler);
    }

    /// Remove the popup handler.
    pub async fn remove_handler(&self) {
        let mut h = self.handler.write().await;
        *h = None;
    }

    /// Emit a popup event to the handler.
    pub async fn emit(&self, popup: Page) {
        let handler = self.handler.read().await;
        if let Some(ref h) = *handler {
            h(popup).await;
        }
    }

    /// Check if this popup was opened by the page with the given `target_id`.
    pub fn is_opener(&self, opener_id: &str) -> bool {
        self.target_id == opener_id
    }
}

/// Builder for waiting for a popup during an action.
pub struct WaitForPopupBuilder<'a, F, Fut>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), crate::error::LocatorError>>,
{
    page: &'a Page,
    action: Option<F>,
    timeout: Duration,
}

// =========================================================================
// Page impl - Popup Handling Methods
// =========================================================================

impl Page {
    /// Set a handler for popup window events.
    ///
    /// The handler will be called whenever a popup window is opened
    /// from this page (e.g., via `window.open()` or `target="_blank"` links).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.on_popup(|mut popup| async move {
    ///     println!("Popup opened: {}", popup.url().await.unwrap_or_default());
    ///     // Work with the popup
    ///     let _ = popup.close().await;
    /// }).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_popup<F, Fut>(&self, handler: F)
    where
        F: Fn(Page) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.popup_manager.set_handler(handler).await;
    }

    /// Remove the popup handler.
    pub async fn off_popup(&self) {
        self.popup_manager.remove_handler().await;
    }

    /// Wait for a popup to be opened during an action.
    ///
    /// This is useful for handling popups that are opened by clicking links
    /// or buttons that open new windows.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::page::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// let mut popup = page.wait_for_popup(|| async {
    ///     page.locator("a[target=_blank]").click().await
    /// }).wait().await?;
    ///
    /// // Now work with the popup page
    /// println!("Popup URL: {}", popup.url().await?);
    /// popup.close().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The action fails
    /// - No popup is opened within the timeout (30 seconds by default)
    pub fn wait_for_popup<F, Fut>(
        &self,
        action: F,
    ) -> WaitForPopupBuilder<'_, F, Fut>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), crate::error::LocatorError>>,
    {
        WaitForPopupBuilder::new(self, action)
    }

    /// Get the opener page that opened this popup.
    ///
    /// Returns `None` if this page is not a popup (was created via `context.new_page()`).
    ///
    /// Note: This method currently returns `None` because tracking opener pages
    /// requires context-level state management. For now, you can check if a page
    /// is a popup by examining whether it was returned from `wait_for_popup()`.
    pub fn opener(&self) -> Option<&str> {
        self.opener_target_id.as_deref()
    }
}

impl<'a, F, Fut> WaitForPopupBuilder<'a, F, Fut>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), crate::error::LocatorError>>,
{
    /// Create a new builder.
    pub fn new(page: &'a Page, action: F) -> Self {
        Self {
            page,
            action: Some(action),
            timeout: Duration::from_secs(30),
        }
    }

    /// Set the timeout for waiting for the popup.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Execute the action and wait for a popup.
    ///
    /// Returns the popup page that was opened during the action.
    pub async fn wait(mut self) -> Result<Page, PageError> {
        use viewpoint_cdp::protocol::target_domain::{AttachToTargetParams, AttachToTargetResult, TargetCreatedEvent};

        let connection = self.page.connection().clone();
        let target_id = self.page.target_id().to_string();
        let _session_id = self.page.session_id().to_string();
        
        // Create a channel to receive the popup
        let (tx, rx) = oneshot::channel::<Page>();
        let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

        // Subscribe to target events
        let mut events = connection.subscribe_events();
        let tx_clone = tx.clone();
        let connection_clone = connection.clone();
        let target_id_clone = target_id.clone();

        // Spawn a task to listen for popup events
        let popup_listener = tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                if event.method == "Target.targetCreated" {
                    if let Some(params) = &event.params {
                        if let Ok(created_event) = serde_json::from_value::<TargetCreatedEvent>(params.clone()) {
                            let info = &created_event.target_info;
                            
                            // Check if this is a popup opened by our page
                            if info.target_type == "page" 
                                && info.opener_id.as_deref() == Some(&target_id_clone)
                            {
                                debug!("Popup detected: {}", info.target_id);
                                
                                // Attach to the popup target
                                let attach_result: Result<AttachToTargetResult, _> = connection_clone
                                    .send_command(
                                        "Target.attachToTarget",
                                        Some(AttachToTargetParams {
                                            target_id: info.target_id.clone(),
                                            flatten: Some(true),
                                        }),
                                        None,
                                    )
                                    .await;

                                if let Ok(attach) = attach_result {
                                    // Enable required domains on the popup
                                    let popup_session = &attach.session_id;
                                    
                                    let _ = connection_clone
                                        .send_command::<(), serde_json::Value>("Page.enable", None, Some(popup_session))
                                        .await;
                                    let _ = connection_clone
                                        .send_command::<(), serde_json::Value>("Network.enable", None, Some(popup_session))
                                        .await;
                                    let _ = connection_clone
                                        .send_command::<(), serde_json::Value>("Runtime.enable", None, Some(popup_session))
                                        .await;

                                    // Get the main frame ID
                                    let frame_tree: Result<viewpoint_cdp::protocol::page::GetFrameTreeResult, _> = connection_clone
                                        .send_command("Page.getFrameTree", None::<()>, Some(popup_session))
                                        .await;

                                    if let Ok(tree) = frame_tree {
                                        let frame_id = tree.frame_tree.frame.id;
                                        
                                        // Create the popup Page
                                        let popup = Page::new(
                                            connection_clone.clone(),
                                            info.target_id.clone(),
                                            attach.session_id.clone(),
                                            frame_id,
                                        );

                                        // Send the popup
                                        let mut guard = tx_clone.lock().await;
                                        if let Some(sender) = guard.take() {
                                            let _ = sender.send(popup);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        // Execute the action
        let action = self.action.take().expect("action already consumed");
        let action_result = action().await;

        // Wait for the popup or timeout
        let result = match action_result {
            Ok(()) => {
                match tokio::time::timeout(self.timeout, rx).await {
                    Ok(Ok(popup)) => Ok(popup),
                    Ok(Err(_)) => Err(PageError::EvaluationFailed(
                        "Popup channel closed unexpectedly".to_string(),
                    )),
                    Err(_) => Err(PageError::EvaluationFailed(
                        format!("wait_for_popup timed out after {:?}", self.timeout),
                    )),
                }
            }
            Err(e) => Err(PageError::EvaluationFailed(e.to_string())),
        };

        // Clean up the listener
        popup_listener.abort();

        result
    }
}
