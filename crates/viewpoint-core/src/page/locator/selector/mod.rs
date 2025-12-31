//! Selector types for element location strategies.

// Re-export AriaRole from aria_role module
pub use super::aria_role::{AriaRole, implicit_role_selector};

use viewpoint_cdp::protocol::dom::BackendNodeId;

/// Options for text-based locators.
#[derive(Debug, Clone, Default)]
pub struct TextOptions {
    /// Whether to match exact text.
    pub exact: bool,
}

/// Selector for finding elements.
#[derive(Debug, Clone)]
pub enum Selector {
    /// CSS selector.
    Css(String),
    /// Text content selector.
    Text { text: String, exact: bool },
    /// ARIA role selector with optional accessible name.
    Role {
        role: AriaRole,
        name: Option<String>,
    },
    /// Test ID selector (data-testid attribute).
    TestId(String),
    /// Test ID with custom attribute.
    TestIdCustom { id: String, attribute: String },
    /// Label selector (finds form elements by label text).
    Label(String),
    /// Placeholder selector.
    Placeholder(String),
    /// Chained selector (parent >> child).
    Chained(Box<Selector>, Box<Selector>),
    /// Nth element in a collection (0-based index, negative from end).
    Nth { base: Box<Selector>, index: i32 },
    /// Alt text selector for images.
    AltText { text: String, exact: bool },
    /// Title attribute selector.
    Title { text: String, exact: bool },
    /// AND combinator - matches elements that match both selectors.
    And(Box<Selector>, Box<Selector>),
    /// OR combinator - matches elements that match either selector.
    Or(Box<Selector>, Box<Selector>),
    /// Filter by text content.
    FilterText {
        base: Box<Selector>,
        text: String,
        exact: bool,
        has_not: bool,
    },
    /// Filter by having a child that matches another selector.
    FilterHas {
        base: Box<Selector>,
        child: Box<Selector>,
        has_not: bool,
    },
    /// Backend node ID selector (from ARIA snapshot refs).
    ///
    /// This selector targets a specific element by its CDP backend node ID,
    /// which is extracted from ARIA snapshot refs (format: `e{backendNodeId}`).
    BackendNodeId(BackendNodeId),
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Css(css) => write!(f, "css={css}"),
            Selector::Text { text, exact } => {
                if *exact {
                    write!(f, "text={text}")
                } else {
                    write!(f, "text*={text}")
                }
            }
            Selector::Role { role, name } => match name {
                Some(n) => write!(f, "role={}[name={}]", role.as_str(), n),
                None => write!(f, "role={}", role.as_str()),
            },
            Selector::TestId(id) => write!(f, "testid={id}"),
            Selector::TestIdCustom { id, attribute } => write!(f, "testid[{attribute}]={id}"),
            Selector::Label(label) => write!(f, "label={label}"),
            Selector::Placeholder(placeholder) => write!(f, "placeholder={placeholder}"),
            Selector::Chained(parent, child) => write!(f, "{parent} >> {child}"),
            Selector::Nth { base, index } => write!(f, "{base}.nth({index})"),
            Selector::AltText { text, exact } => {
                if *exact {
                    write!(f, "alt={text}")
                } else {
                    write!(f, "alt*={text}")
                }
            }
            Selector::Title { text, exact } => {
                if *exact {
                    write!(f, "title={text}")
                } else {
                    write!(f, "title*={text}")
                }
            }
            Selector::And(a, b) => write!(f, "({a}).and({b})"),
            Selector::Or(a, b) => write!(f, "({a}).or({b})"),
            Selector::FilterText {
                base,
                text,
                exact,
                has_not,
            } => {
                let op = if *has_not { "hasNotText" } else { "hasText" };
                let eq = if *exact { "" } else { "*" };
                write!(f, "{base}.filter({op}{eq}={text})")
            }
            Selector::FilterHas {
                base,
                child,
                has_not,
            } => {
                if *has_not {
                    write!(f, "{base}.filter(hasNot={child})")
                } else {
                    write!(f, "{base}.filter(has={child})")
                }
            }
            Selector::BackendNodeId(id) => write!(f, "ref=e{id}"),
        }
    }
}

impl Selector {
    /// Convert selector to a JavaScript expression that returns element(s).
    pub fn to_js_expression(&self) -> String {
        match self {
            Selector::Css(css) => {
                format!("document.querySelectorAll({})", js_string_literal(css))
            }

            Selector::Text { text, exact } => {
                if *exact {
                    format!(
                        r"Array.from(document.querySelectorAll('*')).filter(el => el.textContent?.trim() === {})",
                        js_string_literal(text)
                    )
                } else {
                    format!(
                        r"Array.from(document.querySelectorAll('*')).filter(el => el.textContent?.includes({}))",
                        js_string_literal(text)
                    )
                }
            }

            Selector::Role { role, name } => {
                let role_str = role.as_str();
                // Escape the CSS selector for embedding in a single-quoted JS string
                let implicit_css =
                    viewpoint_js_core::escape_js_contents_single(implicit_role_selector(*role));
                match name {
                    Some(n) => format!(
                        r#"Array.from(document.querySelectorAll('[role="{}"]')).concat(Array.from(document.querySelectorAll('{}'))).filter(el => (el.getAttribute('aria-label') || el.textContent?.trim()) === {})"#,
                        role_str,
                        implicit_css,
                        js_string_literal(n)
                    ),
                    None => format!(
                        r#"Array.from(document.querySelectorAll('[role="{role_str}"]')).concat(Array.from(document.querySelectorAll('{implicit_css}')))"#
                    ),
                }
            }

            Selector::TestId(id) => {
                format!(
                    "document.querySelectorAll('[data-testid={}]')",
                    css_attr_value(id)
                )
            }

            Selector::TestIdCustom { id, attribute } => {
                format!(
                    "document.querySelectorAll('[{}={}]')",
                    attribute,
                    css_attr_value(id)
                )
            }

            Selector::Label(label) => {
                format!(
                    r"(function() {{
                        const labels = Array.from(document.querySelectorAll('label'));
                        const matching = labels.filter(l => l.textContent?.trim() === {});
                        return matching.flatMap(l => {{
                            if (l.htmlFor) return Array.from(document.querySelectorAll('#' + l.htmlFor));
                            return Array.from(l.querySelectorAll('input, textarea, select'));
                        }});
                    }})()",
                    js_string_literal(label)
                )
            }

            Selector::Placeholder(placeholder) => {
                format!(
                    "document.querySelectorAll('[placeholder={}]')",
                    css_attr_value(placeholder)
                )
            }

            Selector::Chained(parent, child) => {
                format!(
                    r"(function() {{
                        const parents = {};
                        const results = [];
                        for (const parent of parents) {{
                            const children = parent.querySelectorAll ? 
                                Array.from(parent.querySelectorAll('*')) : [];
                            const childSelector = {};
                            for (const child of childSelector) {{
                                if (parent.contains(child)) results.push(child);
                            }}
                        }}
                        return results;
                    }})()",
                    parent.to_js_expression(),
                    child.to_js_expression()
                )
            }

            Selector::Nth { base, index } => {
                let base_expr = base.to_js_expression();
                if *index >= 0 {
                    format!(
                        r"(function() {{
                            const elements = Array.from({base_expr});
                            return elements[{index}] ? [elements[{index}]] : [];
                        }})()"
                    )
                } else {
                    // Negative index: -1 = last, -2 = second to last, etc.
                    format!(
                        r"(function() {{
                            const elements = Array.from({base_expr});
                            const idx = elements.length + {index};
                            return idx >= 0 && elements[idx] ? [elements[idx]] : [];
                        }})()"
                    )
                }
            }

            Selector::AltText { text, exact } => {
                if *exact {
                    format!(
                        r"Array.from(document.querySelectorAll('[alt]')).filter(el => el.alt === {})",
                        js_string_literal(text)
                    )
                } else {
                    format!(
                        r"Array.from(document.querySelectorAll('[alt]')).filter(el => (el.alt || '').includes({}))",
                        js_string_literal(text)
                    )
                }
            }

            Selector::Title { text, exact } => {
                if *exact {
                    format!(
                        r"Array.from(document.querySelectorAll('[title]')).filter(el => el.title === {})",
                        js_string_literal(text)
                    )
                } else {
                    format!(
                        r"Array.from(document.querySelectorAll('[title]')).filter(el => (el.title || '').includes({}))",
                        js_string_literal(text)
                    )
                }
            }

            Selector::And(a, b) => {
                let a_expr = a.to_js_expression();
                let b_expr = b.to_js_expression();
                format!(
                    r"(function() {{
                        const setA = new Set({a_expr});
                        const setB = new Set({b_expr});
                        return Array.from(setA).filter(el => setB.has(el));
                    }})()"
                )
            }

            Selector::Or(a, b) => {
                let a_expr = a.to_js_expression();
                let b_expr = b.to_js_expression();
                format!(
                    r"(function() {{
                        const results = new Set({a_expr});
                        for (const el of {b_expr}) results.add(el);
                        return Array.from(results);
                    }})()"
                )
            }

            Selector::FilterText {
                base,
                text,
                exact,
                has_not,
            } => {
                let base_expr = base.to_js_expression();
                let text_lit = js_string_literal(text);
                if *has_not {
                    if *exact {
                        format!(
                            r"Array.from({base_expr}).filter(el => el.textContent?.trim() !== {text_lit})"
                        )
                    } else {
                        format!(
                            r"Array.from({base_expr}).filter(el => !el.textContent?.includes({text_lit}))"
                        )
                    }
                } else if *exact {
                    format!(
                        r"Array.from({base_expr}).filter(el => el.textContent?.trim() === {text_lit})"
                    )
                } else {
                    format!(
                        r"Array.from({base_expr}).filter(el => el.textContent?.includes({text_lit}))"
                    )
                }
            }

            Selector::FilterHas {
                base,
                child,
                has_not,
            } => {
                let base_expr = base.to_js_expression();
                let child_expr = child.to_js_expression();
                if *has_not {
                    format!(
                        r"(function() {{
                            const childSet = new Set({child_expr});
                            return Array.from({base_expr}).filter(el => {{
                                for (const c of childSet) {{
                                    if (el.contains(c)) return false;
                                }}
                                return true;
                            }});
                        }})()"
                    )
                } else {
                    format!(
                        r"(function() {{
                            const childSet = new Set({child_expr});
                            return Array.from({base_expr}).filter(el => {{
                                for (const c of childSet) {{
                                    if (el.contains(c)) return true;
                                }}
                                return false;
                            }});
                        }})()"
                    )
                }
            }

            Selector::BackendNodeId(id) => {
                // Backend node ID selectors are resolved via CDP, not JS
                // This JS expression is a placeholder that will be replaced
                // by the actual element resolution in the Rust code
                format!(
                    r"(function() {{
                        // Backend node ID selector (resolved via CDP DOM.resolveNode)
                        // ID: {id}
                        throw new Error('BackendNodeId selectors must be resolved via CDP');
                    }})()"
                )
            }
        }
    }
}

// Re-export escape functions from viewpoint-js-core for use in selectors
use viewpoint_js_core::{escape_for_css_attr, escape_js_string_single};

/// Escape a string for use in JavaScript (single-quoted).
/// This is a convenience wrapper around `viewpoint_js_core::escape_js_string_single`.
pub(crate) fn js_string_literal(s: &str) -> String {
    escape_js_string_single(s)
}

/// Escape a string for use in a CSS attribute selector within JavaScript.
/// This is a convenience wrapper around `viewpoint_js_core::escape_for_css_attr`.
pub(crate) fn css_attr_value(s: &str) -> String {
    escape_for_css_attr(s)
}

#[cfg(test)]
mod tests;
