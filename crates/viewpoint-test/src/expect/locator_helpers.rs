//! Internal helper functions for locator assertions.

use std::future::Future;
use std::time::Duration;

use viewpoint_cdp::protocol::dom::{BackendNodeId, ResolveNodeParams, ResolveNodeResult};
use viewpoint_core::Selector;

use crate::error::AssertionError;

/// Escape strings for JavaScript string literals.
pub fn js_string_literal(s: &str) -> String {
    let escaped = s
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("'{escaped}'")
}

/// Evaluate JavaScript on a page.
pub async fn evaluate_js(
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
        .map_err(|e| {
            AssertionError::new("Failed to evaluate JavaScript", "success", e.to_string())
        })?;

    if let Some(exception) = result.exception_details {
        return Err(AssertionError::new(
            "JavaScript error",
            "no error",
            exception.text,
        ));
    }

    result.result.value.ok_or_else(|| {
        AssertionError::new("No result from JavaScript", "a value", "null/undefined")
    })
}

/// Get input value from an element.
pub async fn get_input_value(
    locator: &viewpoint_core::Locator<'_>,
) -> Result<String, AssertionError> {
    let page = locator.page();
    let selector = locator.selector();

    // Handle Ref selector - lookup in ref map and resolve via CDP
    if let Selector::Ref(ref_str) = selector {
        let backend_node_id = page.get_backend_node_id_for_ref(ref_str)
            .map_err(|e| AssertionError::new("Ref not found", "ref to exist", e.to_string()))?;
        return get_input_value_by_backend_id(page, backend_node_id).await;
    }

    // Handle BackendNodeId selector
    if let Selector::BackendNodeId(backend_node_id) = selector {
        return get_input_value_by_backend_id(page, *backend_node_id).await;
    }

    let js = format!(
        r"(function() {{
            const elements = {};
            if (elements.length === 0) return {{ found: false }};
            const el = elements[0];
            return {{ found: true, value: el.value || '' }};
        }})()",
        selector.to_js_expression()
    );

    let result = evaluate_js(page, &js).await?;

    let found = result
        .get("found")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if !found {
        return Err(AssertionError::new(
            "Element not found",
            "element to exist",
            "element not found",
        ));
    }

    Ok(result
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string())
}

/// Get input value by backend node ID.
async fn get_input_value_by_backend_id(
    page: &viewpoint_core::Page,
    backend_node_id: BackendNodeId,
) -> Result<String, AssertionError> {
    let result = call_function_on_backend_id(
        page,
        backend_node_id,
        r#"function() {
            return { value: this.value || '' };
        }"#,
    ).await?;

    Ok(result
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string())
}

/// Get selected values from a select element.
pub async fn get_selected_values(
    locator: &viewpoint_core::Locator<'_>,
) -> Result<Vec<String>, AssertionError> {
    let page = locator.page();
    let selector = locator.selector();

    // Handle Ref selector - lookup in ref map and resolve via CDP
    if let Selector::Ref(ref_str) = selector {
        let backend_node_id = page.get_backend_node_id_for_ref(ref_str)
            .map_err(|e| AssertionError::new("Ref not found", "ref to exist", e.to_string()))?;
        return get_selected_values_by_backend_id(page, backend_node_id).await;
    }

    // Handle BackendNodeId selector
    if let Selector::BackendNodeId(backend_node_id) = selector {
        return get_selected_values_by_backend_id(page, *backend_node_id).await;
    }

    let js = format!(
        r"(function() {{
            const elements = {};
            if (elements.length === 0) return {{ found: false }};
            const el = elements[0];
            if (el.tagName.toLowerCase() !== 'select') {{
                return {{ found: true, values: [el.value || ''] }};
            }}
            const values = [];
            for (const opt of el.selectedOptions) {{
                values.push(opt.value);
            }}
            return {{ found: true, values: values }};
        }})()",
        selector.to_js_expression()
    );

    let result = evaluate_js(page, &js).await?;

    let found = result
        .get("found")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if !found {
        return Err(AssertionError::new(
            "Element not found",
            "element to exist",
            "element not found",
        ));
    }

    Ok(result
        .get("values")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(std::string::ToString::to_string)
                .collect()
        })
        .unwrap_or_default())
}

/// Get selected values by backend node ID.
async fn get_selected_values_by_backend_id(
    page: &viewpoint_core::Page,
    backend_node_id: BackendNodeId,
) -> Result<Vec<String>, AssertionError> {
    let result = call_function_on_backend_id(
        page,
        backend_node_id,
        r#"function() {
            const el = this;
            if (el.tagName.toLowerCase() !== 'select') {
                return { values: [el.value || ''] };
            }
            const values = [];
            for (const opt of el.selectedOptions) {
                values.push(opt.value);
            }
            return { values: values };
        }"#,
    ).await?;

    Ok(result
        .get("values")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(std::string::ToString::to_string)
                .collect()
        })
        .unwrap_or_default())
}

/// Get attribute value from an element.
pub async fn get_attribute(
    locator: &viewpoint_core::Locator<'_>,
    name: &str,
) -> Result<Option<String>, AssertionError> {
    let page = locator.page();
    let selector = locator.selector();

    // Handle Ref selector - lookup in ref map and resolve via CDP
    if let Selector::Ref(ref_str) = selector {
        let backend_node_id = page.get_backend_node_id_for_ref(ref_str)
            .map_err(|e| AssertionError::new("Ref not found", "ref to exist", e.to_string()))?;
        return get_attribute_by_backend_id(page, backend_node_id, name).await;
    }

    // Handle BackendNodeId selector
    if let Selector::BackendNodeId(backend_node_id) = selector {
        return get_attribute_by_backend_id(page, *backend_node_id, name).await;
    }

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

    let found = result
        .get("found")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if !found {
        return Ok(None);
    }

    Ok(result
        .get("value")
        .and_then(|v| v.as_str())
        .map(String::from))
}

/// Get attribute by backend node ID.
async fn get_attribute_by_backend_id(
    page: &viewpoint_core::Page,
    backend_node_id: BackendNodeId,
    name: &str,
) -> Result<Option<String>, AssertionError> {
    let name_escaped = js_string_literal(name);
    let result = call_function_on_backend_id_with_fn(
        page,
        backend_node_id,
        &format!(r#"function() {{
            const value = this.getAttribute({name_escaped});
            return {{ value: value }};
        }}"#),
    ).await?;

    Ok(result
        .get("value")
        .and_then(|v| if v.is_null() { None } else { v.as_str() })
        .map(String::from))
}

/// Check if an element is enabled.
pub async fn is_enabled(locator: &viewpoint_core::Locator<'_>) -> Result<bool, AssertionError> {
    let page = locator.page();
    let selector = locator.selector();

    // Handle Ref selector - lookup in ref map and resolve via CDP
    if let Selector::Ref(ref_str) = selector {
        let backend_node_id = page.get_backend_node_id_for_ref(ref_str)
            .map_err(|e| AssertionError::new("Ref not found", "ref to exist", e.to_string()))?;
        return is_enabled_by_backend_id(page, backend_node_id).await;
    }

    // Handle BackendNodeId selector
    if let Selector::BackendNodeId(backend_node_id) = selector {
        return is_enabled_by_backend_id(page, *backend_node_id).await;
    }

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

    let found = result
        .get("found")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if !found {
        return Err(AssertionError::new(
            "Element not found",
            "element to exist",
            "element not found",
        ));
    }

    Ok(result
        .get("enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true))
}

/// Check if element is enabled by backend node ID.
async fn is_enabled_by_backend_id(
    page: &viewpoint_core::Page,
    backend_node_id: BackendNodeId,
) -> Result<bool, AssertionError> {
    let result = call_function_on_backend_id(
        page,
        backend_node_id,
        r#"function() {
            return { enabled: !this.disabled };
        }"#,
    ).await?;

    Ok(result
        .get("enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true))
}

/// Retry loop helper for assertions.
///
/// This encapsulates the common retry-with-timeout pattern used by all assertions.
pub async fn retry_until<F, Fut, T>(
    timeout: Duration,
    is_negated: bool,
    mut check_fn: F,
    error_message: impl Fn(bool) -> String,
    expected_value: impl Fn(bool) -> String,
    actual_value: impl Fn(&T) -> String,
) -> Result<(), AssertionError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<(bool, T), AssertionError>>,
{
    let start = std::time::Instant::now();

    loop {
        let (matches, actual) = check_fn().await?;
        let expected_match = !is_negated;

        if matches == expected_match {
            return Ok(());
        }

        if start.elapsed() >= timeout {
            return Err(AssertionError::new(
                error_message(is_negated),
                expected_value(is_negated),
                actual_value(&actual),
            ));
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Helper to call a function on a backend node ID and return the result.
async fn call_function_on_backend_id(
    page: &viewpoint_core::Page,
    backend_node_id: BackendNodeId,
    function_declaration: &str,
) -> Result<serde_json::Value, AssertionError> {
    call_function_on_backend_id_with_fn(page, backend_node_id, function_declaration).await
}

/// Helper to call a custom function on a backend node ID.
async fn call_function_on_backend_id_with_fn(
    page: &viewpoint_core::Page,
    backend_node_id: BackendNodeId,
    function_declaration: &str,
) -> Result<serde_json::Value, AssertionError> {
    // Resolve the backend node ID to a RemoteObject
    let result: ResolveNodeResult = page
        .connection()
        .send_command(
            "DOM.resolveNode",
            Some(ResolveNodeParams {
                node_id: None,
                backend_node_id: Some(backend_node_id),
                object_group: Some("viewpoint-test-query".to_string()),
                execution_context_id: None,
            }),
            Some(page.session_id()),
        )
        .await
        .map_err(|_| {
            AssertionError::new(
                "Element not found",
                "element to exist",
                format!("Could not resolve backend node ID {backend_node_id}: element may no longer exist"),
            )
        })?;

    let object_id = result.object.object_id.ok_or_else(|| {
        AssertionError::new(
            "Element not found",
            "element to exist",
            format!("No object ID for backend node ID {backend_node_id}"),
        )
    })?;

    // Call the function on the resolved element using viewpoint_cdp::protocol::runtime::CallFunctionOnResult
    let call_result: viewpoint_cdp::protocol::runtime::CallFunctionOnResult = page
        .connection()
        .send_command(
            "Runtime.callFunctionOn",
            Some(serde_json::json!({
                "objectId": object_id,
                "functionDeclaration": function_declaration,
                "returnByValue": true
            })),
            Some(page.session_id()),
        )
        .await
        .map_err(|e| {
            AssertionError::new(
                "Failed to call function",
                "success",
                e.to_string(),
            )
        })?;

    // Release the object
    let _ = page
        .connection()
        .send_command::<_, serde_json::Value>(
            "Runtime.releaseObject",
            Some(serde_json::json!({ "objectId": object_id })),
            Some(page.session_id()),
        )
        .await;

    if let Some(exception) = call_result.exception_details {
        return Err(AssertionError::new(
            "JavaScript error",
            "no error",
            exception.text,
        ));
    }

    call_result.result.value.ok_or_else(|| {
        AssertionError::new(
            "No result from query",
            "a value",
            "null/undefined",
        )
    })
}
