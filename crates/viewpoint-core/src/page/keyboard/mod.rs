//! Keyboard input handling.
//!
//! Provides direct keyboard control for simulating key presses, key holds,
//! and text input.

mod keys;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tracing::{debug, instrument, trace};
use viewpoint_cdp::CdpConnection;
use viewpoint_cdp::protocol::input::{
    DispatchKeyEventParams, InsertTextParams, KeyEventType, modifiers,
};

use crate::error::LocatorError;
use crate::wait::NavigationWaiter;

pub use keys::{KeyDefinition, get_key_definition};

/// Check if a key is an uppercase letter that requires Shift.
fn is_uppercase_letter(key: &str) -> bool {
    key.len() == 1 && key.chars().next().is_some_and(|c| c.is_ascii_uppercase())
}

/// Check if a key is a modifier key.
fn is_modifier_key(key: &str) -> bool {
    matches!(
        key,
        "Alt"
            | "AltLeft"
            | "AltRight"
            | "Control"
            | "ControlLeft"
            | "ControlRight"
            | "Meta"
            | "MetaLeft"
            | "MetaRight"
            | "Shift"
            | "ShiftLeft"
            | "ShiftRight"
    )
}

/// Keyboard state tracking.
#[derive(Debug)]
struct KeyboardState {
    /// Currently pressed modifier keys.
    modifiers: i32,
    /// Set of currently held keys.
    pressed_keys: HashSet<String>,
}

impl KeyboardState {
    fn new() -> Self {
        Self {
            modifiers: 0,
            pressed_keys: HashSet::new(),
        }
    }

    fn key_down(&mut self, key: &str) -> bool {
        let is_repeat = self.pressed_keys.contains(key);
        self.pressed_keys.insert(key.to_string());

        // Update modifiers
        match key {
            "Alt" | "AltLeft" | "AltRight" => self.modifiers |= modifiers::ALT,
            "Control" | "ControlLeft" | "ControlRight" => self.modifiers |= modifiers::CTRL,
            "Meta" | "MetaLeft" | "MetaRight" => self.modifiers |= modifiers::META,
            "Shift" | "ShiftLeft" | "ShiftRight" => self.modifiers |= modifiers::SHIFT,
            _ => {}
        }

        is_repeat
    }

    fn key_up(&mut self, key: &str) {
        self.pressed_keys.remove(key);

        // Update modifiers
        match key {
            "Alt" | "AltLeft" | "AltRight" => self.modifiers &= !modifiers::ALT,
            "Control" | "ControlLeft" | "ControlRight" => self.modifiers &= !modifiers::CTRL,
            "Meta" | "MetaLeft" | "MetaRight" => self.modifiers &= !modifiers::META,
            "Shift" | "ShiftLeft" | "ShiftRight" => self.modifiers &= !modifiers::SHIFT,
            _ => {}
        }
    }
}

/// Keyboard controller for direct keyboard input.
///
/// Provides methods for pressing keys, typing text, and managing modifier state.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "integration")]
/// # tokio_test::block_on(async {
/// # use viewpoint_core::Browser;
/// # let browser = Browser::launch().headless(true).launch().await.unwrap();
/// # let context = browser.new_context().await.unwrap();
/// # let page = context.new_page().await.unwrap();
/// # page.goto("about:blank").goto().await.unwrap();
///
/// // Press a single key
/// page.keyboard().press("Enter").await.unwrap();
///
/// // Type text character by character
/// page.keyboard().type_text("Hello").await.unwrap();
///
/// // Use key combinations
/// page.keyboard().press("Control+a").await.unwrap();
///
/// // Hold modifier and press keys
/// page.keyboard().down("Shift").await.unwrap();
/// page.keyboard().press("a").await.unwrap(); // Types 'A'
/// page.keyboard().up("Shift").await.unwrap();
/// # });
/// ```
#[derive(Debug)]
pub struct Keyboard {
    /// CDP connection.
    connection: Arc<CdpConnection>,
    /// Session ID for the page.
    session_id: String,
    /// Main frame ID for navigation detection.
    frame_id: String,
    /// Keyboard state.
    state: Mutex<KeyboardState>,
}

impl Keyboard {
    /// Create a new keyboard controller.
    pub(crate) fn new(
        connection: Arc<CdpConnection>,
        session_id: String,
        frame_id: String,
    ) -> Self {
        Self {
            connection,
            session_id,
            frame_id,
            state: Mutex::new(KeyboardState::new()),
        }
    }

    /// Press and release a key or key combination.
    ///
    /// Returns a builder that can be configured with additional options, or awaited
    /// directly for a simple key press.
    ///
    /// # Arguments
    ///
    /// * `key` - Key to press. Can be:
    ///   - A single key: `"Enter"`, `"a"`, `"F1"`
    ///   - A key combination: `"Control+c"`, `"Shift+Tab"`
    ///   - `ControlOrMeta` for cross-platform shortcuts
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// // Simple press - await directly
    /// page.keyboard().press("Enter").await?;
    ///
    /// // Press without waiting for navigation
    /// page.keyboard().press("Enter").no_wait_after(true).await?;
    ///
    /// // Press with delay
    /// page.keyboard().press("Control+a").delay(std::time::Duration::from_millis(100)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn press(&self, key: &str) -> KeyboardPressBuilder<'_> {
        KeyboardPressBuilder::new(self, key)
    }

    /// Internal method to perform the actual key press.
    async fn press_internal(
        &self,
        key: &str,
        delay: Option<Duration>,
    ) -> Result<(), LocatorError> {
        // Parse key combination
        let parts: Vec<&str> = key.split('+').collect();
        let actual_key = parts.last().copied().unwrap_or(key);

        // Press modifiers
        for part in &parts[..parts.len().saturating_sub(1)] {
            let modifier_key = self.resolve_modifier(part);
            self.down(&modifier_key).await?;
        }

        // Check if actual_key is uppercase and we need to add Shift
        let need_shift = is_uppercase_letter(actual_key);
        if need_shift {
            self.down("Shift").await?;
        }

        // Press the actual key
        self.down(actual_key).await?;

        if let Some(d) = delay {
            tokio::time::sleep(d).await;
        }

        self.up(actual_key).await?;

        // Release Shift if we added it
        if need_shift {
            self.up("Shift").await?;
        }

        // Release modifiers in reverse order
        for part in parts[..parts.len().saturating_sub(1)].iter().rev() {
            let modifier_key = self.resolve_modifier(part);
            self.up(&modifier_key).await?;
        }

        Ok(())
    }

    /// Resolve platform-specific modifier keys.
    fn resolve_modifier(&self, key: &str) -> String {
        match key {
            "ControlOrMeta" => {
                // On macOS use Meta, on other platforms use Control
                if cfg!(target_os = "macos") {
                    "Meta".to_string()
                } else {
                    "Control".to_string()
                }
            }
            _ => key.to_string(),
        }
    }

    /// Press and hold a key.
    ///
    /// The key will remain pressed until `up()` is called.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.keyboard().down("Shift").await?;
    /// page.keyboard().press("a").await?; // Types 'A'
    /// page.keyboard().up("Shift").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self), fields(key = %key))]
    pub async fn down(&self, key: &str) -> Result<(), LocatorError> {
        let def = get_key_definition(key)
            .ok_or_else(|| LocatorError::EvaluationError(format!("Unknown key: {key}")))?;

        let is_repeat = {
            let mut state = self.state.lock().await;
            state.key_down(key)
        };

        let state = self.state.lock().await;
        let current_modifiers = state.modifiers;
        drop(state);

        debug!(code = def.code, key = def.key, is_repeat, "Key down");

        let params = DispatchKeyEventParams {
            event_type: KeyEventType::KeyDown,
            modifiers: Some(current_modifiers),
            timestamp: None,
            text: def.text.map(String::from),
            unmodified_text: def.text.map(String::from),
            key_identifier: None,
            code: Some(def.code.to_string()),
            key: Some(def.key.to_string()),
            windows_virtual_key_code: Some(def.key_code),
            native_virtual_key_code: Some(def.key_code),
            auto_repeat: Some(is_repeat),
            is_keypad: Some(def.is_keypad),
            is_system_key: None,
            commands: None,
        };

        self.dispatch_key_event(params).await?;

        // Send char event for printable characters
        if !is_modifier_key(key) {
            if let Some(text) = def.text {
                let char_params = DispatchKeyEventParams {
                    event_type: KeyEventType::Char,
                    modifiers: Some(current_modifiers),
                    timestamp: None,
                    text: Some(text.to_string()),
                    unmodified_text: Some(text.to_string()),
                    key_identifier: None,
                    code: Some(def.code.to_string()),
                    key: Some(def.key.to_string()),
                    windows_virtual_key_code: Some(def.key_code),
                    native_virtual_key_code: Some(def.key_code),
                    auto_repeat: None,
                    is_keypad: Some(def.is_keypad),
                    is_system_key: None,
                    commands: None,
                };
                self.dispatch_key_event(char_params).await?;
            }
        }

        Ok(())
    }

    /// Release a held key.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.keyboard().down("Shift").await?;
    /// // ... do stuff with Shift held
    /// page.keyboard().up("Shift").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self), fields(key = %key))]
    pub async fn up(&self, key: &str) -> Result<(), LocatorError> {
        let def = get_key_definition(key)
            .ok_or_else(|| LocatorError::EvaluationError(format!("Unknown key: {key}")))?;

        {
            let mut state = self.state.lock().await;
            state.key_up(key);
        }

        let state = self.state.lock().await;
        let current_modifiers = state.modifiers;
        drop(state);

        debug!(code = def.code, key = def.key, "Key up");

        let params = DispatchKeyEventParams {
            event_type: KeyEventType::KeyUp,
            modifiers: Some(current_modifiers),
            timestamp: None,
            text: None,
            unmodified_text: None,
            key_identifier: None,
            code: Some(def.code.to_string()),
            key: Some(def.key.to_string()),
            windows_virtual_key_code: Some(def.key_code),
            native_virtual_key_code: Some(def.key_code),
            auto_repeat: None,
            is_keypad: Some(def.is_keypad),
            is_system_key: None,
            commands: None,
        };

        self.dispatch_key_event(params).await
    }

    /// Type text character by character with key events.
    ///
    /// This generates individual key events for each character.
    /// Use `insert_text()` for faster text entry without key events.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.keyboard().type_text("Hello, World!").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self), fields(text_len = text.len()))]
    pub async fn type_text(&self, text: &str) -> Result<(), LocatorError> {
        self.type_text_with_delay(text, None).await
    }

    /// Type text with a delay between each character.
    #[instrument(level = "debug", skip(self), fields(text_len = text.len()))]
    pub async fn type_text_with_delay(
        &self,
        text: &str,
        delay: Option<Duration>,
    ) -> Result<(), LocatorError> {
        for ch in text.chars() {
            let char_str = ch.to_string();

            // Get key definition if available, otherwise just send char event
            if let Some(_def) = get_key_definition(&char_str) {
                // Check if we need Shift for this character
                let need_shift = ch.is_ascii_uppercase();
                if need_shift {
                    self.down("Shift").await?;
                }

                self.down(&char_str).await?;
                self.up(&char_str).await?;

                if need_shift {
                    self.up("Shift").await?;
                }
            } else {
                // For characters without key definitions, send char event directly
                let params = DispatchKeyEventParams {
                    event_type: KeyEventType::Char,
                    modifiers: None,
                    timestamp: None,
                    text: Some(char_str.clone()),
                    unmodified_text: Some(char_str),
                    key_identifier: None,
                    code: None,
                    key: None,
                    windows_virtual_key_code: None,
                    native_virtual_key_code: None,
                    auto_repeat: None,
                    is_keypad: None,
                    is_system_key: None,
                    commands: None,
                };
                self.dispatch_key_event(params).await?;
            }

            if let Some(d) = delay {
                tokio::time::sleep(d).await;
            }
        }

        Ok(())
    }

    /// Insert text directly without generating key events.
    ///
    /// This is faster than `type_text()` and works with non-ASCII characters.
    /// No keydown/keyup events are dispatched.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::Page;
    ///
    /// # async fn example(page: &Page) -> Result<(), viewpoint_core::CoreError> {
    /// page.keyboard().insert_text("Hello 👋 你好").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(level = "debug", skip(self), fields(text_len = text.len()))]
    pub async fn insert_text(&self, text: &str) -> Result<(), LocatorError> {
        debug!("Inserting text directly");

        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.insertText",
                Some(InsertTextParams {
                    text: text.to_string(),
                }),
                Some(&self.session_id),
            )
            .await?;

        Ok(())
    }

    /// Dispatch a key event to the browser.
    async fn dispatch_key_event(&self, params: DispatchKeyEventParams) -> Result<(), LocatorError> {
        self.connection
            .send_command::<_, serde_json::Value>(
                "Input.dispatchKeyEvent",
                Some(params),
                Some(&self.session_id),
            )
            .await?;
        Ok(())
    }
}

// =============================================================================
// KeyboardPressBuilder
// =============================================================================

/// Builder for keyboard press operations with configurable options.
///
/// Created via [`Keyboard::press`].
#[derive(Debug)]
pub struct KeyboardPressBuilder<'a> {
    keyboard: &'a Keyboard,
    key: String,
    delay: Option<Duration>,
    no_wait_after: bool,
}

impl<'a> KeyboardPressBuilder<'a> {
    fn new(keyboard: &'a Keyboard, key: &str) -> Self {
        Self {
            keyboard,
            key: key.to_string(),
            delay: None,
            no_wait_after: false,
        }
    }

    /// Set a delay between key down and key up.
    #[must_use]
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Whether to skip waiting for navigation after the key press.
    ///
    /// By default, the press will wait for any triggered navigation to complete.
    /// Set to `true` to return immediately after the key is pressed.
    #[must_use]
    pub fn no_wait_after(mut self, no_wait_after: bool) -> Self {
        self.no_wait_after = no_wait_after;
        self
    }

    /// Execute the press operation.
    #[instrument(level = "debug", skip(self), fields(key = %self.key))]
    pub async fn send(self) -> Result<(), LocatorError> {
        // Set up navigation waiter before the action if needed
        let navigation_waiter = if self.no_wait_after {
            None
        } else {
            Some(NavigationWaiter::new(
                self.keyboard.connection.subscribe_events(),
                self.keyboard.session_id.clone(),
                self.keyboard.frame_id.clone(),
            ))
        };

        // Perform the press action
        self.keyboard.press_internal(&self.key, self.delay).await?;

        // Wait for navigation if triggered
        if let Some(waiter) = navigation_waiter {
            match waiter.wait_for_navigation_if_triggered().await {
                Ok(navigated) => {
                    if navigated {
                        trace!("Navigation completed after keyboard press");
                    }
                }
                Err(e) => {
                    debug!(error = ?e, "Navigation wait failed after keyboard press");
                    return Err(LocatorError::WaitError(e));
                }
            }
        }

        Ok(())
    }
}

impl<'a> std::future::IntoFuture for KeyboardPressBuilder<'a> {
    type Output = Result<(), LocatorError>;
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
