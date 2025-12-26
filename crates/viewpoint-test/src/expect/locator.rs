//! Locator assertions for testing element state.

use std::time::Duration;

use viewpoint_core::Locator;

use crate::error::AssertionError;

/// Default timeout for assertions.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Assertions for locators (elements).
pub struct LocatorAssertions<'a> {
    locator: &'a Locator<'a>,
    timeout: Duration,
    is_negated: bool,
}

impl<'a> LocatorAssertions<'a> {
    /// Create a new `LocatorAssertions` for the given locator.
    pub fn new(locator: &'a Locator<'a>) -> Self {
        Self {
            locator,
            timeout: DEFAULT_TIMEOUT,
            is_negated: false,
        }
    }

    /// Set the timeout for this assertion.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Negate the assertion.
    ///
    /// This is an alias for the `not` method to avoid conflict with `std::ops::Not`.
    #[must_use]
    pub fn negated(mut self) -> Self {
        self.is_negated = !self.is_negated;
        self
    }

    /// Negate the assertion.
    ///
    /// Note: This method name shadows the `Not` trait's method. Use `negated()` if
    /// you need to avoid this conflict.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        self.negated()
    }

    /// Assert that the element is visible.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_visible(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_visible = self
                .locator
                .is_visible()
                .await
                .map_err(|e| AssertionError::new("Failed to check visibility", "visible", e.to_string()))?;

            let expected = !self.is_negated;
            if is_visible == expected {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be visible"
                    } else {
                        "Element should be visible"
                    },
                    if expected { "visible" } else { "hidden" },
                    if is_visible { "visible" } else { "hidden" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is hidden.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_hidden(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_visible = self
                .locator
                .is_visible()
                .await
                .map_err(|e| AssertionError::new("Failed to check visibility", "hidden", e.to_string()))?;

            let expected_hidden = !self.is_negated;
            let is_hidden = !is_visible;

            if is_hidden == expected_hidden {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be hidden"
                    } else {
                        "Element should be hidden"
                    },
                    if expected_hidden { "hidden" } else { "visible" },
                    if is_hidden { "hidden" } else { "visible" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element has the exact text content.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_text(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let text = self
                .locator
                .text_content()
                .await
                .map_err(|e| AssertionError::new("Failed to get text content", expected, e.to_string()))?;

            let actual = text.as_deref().unwrap_or("");
            let matches = actual.trim() == expected;
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not have text"
                    } else {
                        "Element should have text"
                    },
                    if self.is_negated {
                        format!("not \"{expected}\"")
                    } else {
                        format!("\"{expected}\"")
                    },
                    format!("\"{actual}\""),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element contains the specified text.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_contain_text(&self, expected: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let text = self
                .locator
                .text_content()
                .await
                .map_err(|e| AssertionError::new("Failed to get text content", expected, e.to_string()))?;

            let actual = text.as_deref().unwrap_or("");
            let contains = actual.contains(expected);
            let expected_match = !self.is_negated;

            if contains == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not contain text"
                    } else {
                        "Element should contain text"
                    },
                    if self.is_negated {
                        format!("not containing \"{expected}\"")
                    } else {
                        format!("containing \"{expected}\"")
                    },
                    format!("\"{actual}\""),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element has the specified attribute value.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_attribute(&self, name: &str, value: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let actual = self.get_attribute(name).await?;
            let matches = actual.as_deref() == Some(value);
            let expected_match = !self.is_negated;

            if matches == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element should not have attribute {name}=\"{value}\"")
                    } else {
                        format!("Element should have attribute {name}=\"{value}\"")
                    },
                    if self.is_negated {
                        format!("not {name}=\"{value}\"")
                    } else {
                        format!("{name}=\"{value}\"")
                    },
                    match actual {
                        Some(v) => format!("{name}=\"{v}\""),
                        None => format!("{name} not present"),
                    },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element has the specified class.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_have_class(&self, class_name: &str) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let class_attr = self.get_attribute("class").await?;
            let classes = class_attr.as_deref().unwrap_or("");
            let has_class = classes.split_whitespace().any(|c| c == class_name);
            let expected_match = !self.is_negated;

            if has_class == expected_match {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        format!("Element should not have class \"{class_name}\"")
                    } else {
                        format!("Element should have class \"{class_name}\"")
                    },
                    if self.is_negated {
                        format!("not containing class \"{class_name}\"")
                    } else {
                        format!("class \"{class_name}\"")
                    },
                    format!("classes: \"{classes}\""),
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is enabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_enabled(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_enabled = self.is_enabled().await?;
            let expected_enabled = !self.is_negated;

            if is_enabled == expected_enabled {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be enabled"
                    } else {
                        "Element should be enabled"
                    },
                    if expected_enabled { "enabled" } else { "disabled" },
                    if is_enabled { "enabled" } else { "disabled" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is disabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_disabled(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_enabled = self.is_enabled().await?;
            let expected_disabled = !self.is_negated;
            let is_disabled = !is_enabled;

            if is_disabled == expected_disabled {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be disabled"
                    } else {
                        "Element should be disabled"
                    },
                    if expected_disabled { "disabled" } else { "enabled" },
                    if is_disabled { "disabled" } else { "enabled" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Assert that the element is checked (for checkboxes/radios).
    ///
    /// # Errors
    ///
    /// Returns an error if the assertion fails or the element cannot be queried.
    pub async fn to_be_checked(&self) -> Result<(), AssertionError> {
        let start = std::time::Instant::now();

        loop {
            let is_checked = self
                .locator
                .is_checked()
                .await
                .map_err(|e| AssertionError::new("Failed to check checked state", "checked", e.to_string()))?;

            let expected_checked = !self.is_negated;

            if is_checked == expected_checked {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(AssertionError::new(
                    if self.is_negated {
                        "Element should not be checked"
                    } else {
                        "Element should be checked"
                    },
                    if expected_checked { "checked" } else { "unchecked" },
                    if is_checked { "checked" } else { "unchecked" },
                ));
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    async fn get_attribute(&self, name: &str) -> Result<Option<String>, AssertionError> {
        let page = self.locator.page();
        let selector = self.locator.selector();

        // Build JS to get attribute
        let js = format!(
            r"(function() {{
                const elements = {};
                if (elements.length === 0) return {{ found: false }};
                const el = elements[0];
                const value = el.getAttribute({});
                return {{ found: true, value: value }};
            }})()",
            selector.to_js_expression(),
            js_string_literal(name)
        );

        let result = evaluate_js(page, &js).await?;

        let found = result.get("found").and_then(serde_json::Value::as_bool).unwrap_or(false);
        if !found {
            return Ok(None);
        }

        Ok(result.get("value").and_then(|v| v.as_str()).map(String::from))
    }

    async fn is_enabled(&self) -> Result<bool, AssertionError> {
        let page = self.locator.page();
        let selector = self.locator.selector();

        let js = format!(
            r"(function() {{
                const elements = {};
                if (elements.length === 0) return {{ found: false }};
                const el = elements[0];
                return {{ found: true, enabled: !el.disabled }};
            }})()",
            selector.to_js_expression()
        );

        let result = evaluate_js(page, &js).await?;

        let found = result.get("found").and_then(serde_json::Value::as_bool).unwrap_or(false);
        if !found {
            return Err(AssertionError::new(
                "Element not found",
                "element to exist",
                "element not found",
            ));
        }

        Ok(result.get("enabled").and_then(serde_json::Value::as_bool).unwrap_or(true))
    }
}

// Helper to escape strings for JavaScript
fn js_string_literal(s: &str) -> String {
    let escaped = s
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("'{escaped}'")
}

// Helper to evaluate JavaScript on a page
async fn evaluate_js(
    page: &viewpoint_core::Page,
    expression: &str,
) -> Result<serde_json::Value, AssertionError> {
    use viewpoint_cdp::protocol::runtime::EvaluateParams;

    if page.is_closed() {
        return Err(AssertionError::new(
            "Page is closed",
            "page to be open",
            "page is closed",
        ));
    }

    let params = EvaluateParams {
        expression: expression.to_string(),
        object_group: None,
        include_command_line_api: None,
        silent: Some(true),
        context_id: None,
        return_by_value: Some(true),
        await_promise: Some(false),
    };

    let result: viewpoint_cdp::protocol::runtime::EvaluateResult = page
        .connection()
        .send_command("Runtime.evaluate", Some(params), Some(page.session_id()))
        .await
        .map_err(|e| AssertionError::new("Failed to evaluate JavaScript", "success", e.to_string()))?;

    if let Some(exception) = result.exception_details {
        return Err(AssertionError::new(
            "JavaScript error",
            "no error",
            exception.text,
        ));
    }

    result
        .result
        .value
        .ok_or_else(|| AssertionError::new("No result from JavaScript", "a value", "null/undefined"))
}
