//! Selector types for element location strategies.

/// ARIA roles for accessibility-based element selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRole {
    Alert,
    AlertDialog,
    Application,
    Article,
    Banner,
    Button,
    Cell,
    Checkbox,
    ColumnHeader,
    Combobox,
    Complementary,
    ContentInfo,
    Definition,
    Dialog,
    Directory,
    Document,
    Feed,
    Figure,
    Form,
    Grid,
    GridCell,
    Group,
    Heading,
    Img,
    Link,
    List,
    ListBox,
    ListItem,
    Log,
    Main,
    Marquee,
    Math,
    Menu,
    MenuBar,
    MenuItem,
    MenuItemCheckbox,
    MenuItemRadio,
    Navigation,
    None,
    Note,
    Option,
    Presentation,
    ProgressBar,
    Radio,
    RadioGroup,
    Region,
    Row,
    RowGroup,
    RowHeader,
    ScrollBar,
    Search,
    SearchBox,
    Separator,
    Slider,
    SpinButton,
    Status,
    Switch,
    Tab,
    Table,
    TabList,
    TabPanel,
    Term,
    TextBox,
    Timer,
    Toolbar,
    Tooltip,
    Tree,
    TreeGrid,
    TreeItem,
}

impl AriaRole {
    /// Get the role name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            AriaRole::Alert => "alert",
            AriaRole::AlertDialog => "alertdialog",
            AriaRole::Application => "application",
            AriaRole::Article => "article",
            AriaRole::Banner => "banner",
            AriaRole::Button => "button",
            AriaRole::Cell => "cell",
            AriaRole::Checkbox => "checkbox",
            AriaRole::ColumnHeader => "columnheader",
            AriaRole::Combobox => "combobox",
            AriaRole::Complementary => "complementary",
            AriaRole::ContentInfo => "contentinfo",
            AriaRole::Definition => "definition",
            AriaRole::Dialog => "dialog",
            AriaRole::Directory => "directory",
            AriaRole::Document => "document",
            AriaRole::Feed => "feed",
            AriaRole::Figure => "figure",
            AriaRole::Form => "form",
            AriaRole::Grid => "grid",
            AriaRole::GridCell => "gridcell",
            AriaRole::Group => "group",
            AriaRole::Heading => "heading",
            AriaRole::Img => "img",
            AriaRole::Link => "link",
            AriaRole::List => "list",
            AriaRole::ListBox => "listbox",
            AriaRole::ListItem => "listitem",
            AriaRole::Log => "log",
            AriaRole::Main => "main",
            AriaRole::Marquee => "marquee",
            AriaRole::Math => "math",
            AriaRole::Menu => "menu",
            AriaRole::MenuBar => "menubar",
            AriaRole::MenuItem => "menuitem",
            AriaRole::MenuItemCheckbox => "menuitemcheckbox",
            AriaRole::MenuItemRadio => "menuitemradio",
            AriaRole::Navigation => "navigation",
            AriaRole::None => "none",
            AriaRole::Note => "note",
            AriaRole::Option => "option",
            AriaRole::Presentation => "presentation",
            AriaRole::ProgressBar => "progressbar",
            AriaRole::Radio => "radio",
            AriaRole::RadioGroup => "radiogroup",
            AriaRole::Region => "region",
            AriaRole::Row => "row",
            AriaRole::RowGroup => "rowgroup",
            AriaRole::RowHeader => "rowheader",
            AriaRole::ScrollBar => "scrollbar",
            AriaRole::Search => "search",
            AriaRole::SearchBox => "searchbox",
            AriaRole::Separator => "separator",
            AriaRole::Slider => "slider",
            AriaRole::SpinButton => "spinbutton",
            AriaRole::Status => "status",
            AriaRole::Switch => "switch",
            AriaRole::Tab => "tab",
            AriaRole::Table => "table",
            AriaRole::TabList => "tablist",
            AriaRole::TabPanel => "tabpanel",
            AriaRole::Term => "term",
            AriaRole::TextBox => "textbox",
            AriaRole::Timer => "timer",
            AriaRole::Toolbar => "toolbar",
            AriaRole::Tooltip => "tooltip",
            AriaRole::Tree => "tree",
            AriaRole::TreeGrid => "treegrid",
            AriaRole::TreeItem => "treeitem",
        }
    }
}

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
    Text {
        text: String,
        exact: bool,
    },

    /// ARIA role selector.
    Role {
        role: AriaRole,
        name: Option<String>,
    },

    /// Test ID selector (data-testid attribute).
    TestId(String),

    /// Label selector (for form controls).
    Label(String),

    /// Placeholder selector (for inputs).
    Placeholder(String),

    /// Chained selector (parent >> child).
    Chained(Box<Selector>, Box<Selector>),

    /// Nth element selector.
    Nth {
        base: Box<Selector>,
        index: i32, // Negative for last (-1 = last)
    },
}

impl Selector {
    /// Convert selector to a JavaScript expression that returns element(s).
    pub fn to_js_expression(&self) -> String {
        match self {
            Selector::Css(css) => {
                format!(
                    "document.querySelectorAll({})",
                    js_string_literal(css)
                )
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
                match name {
                    Some(n) => format!(
                        r#"Array.from(document.querySelectorAll('[role="{}"]')).concat(Array.from(document.querySelectorAll('{}'))).filter(el => (el.getAttribute('aria-label') || el.textContent?.trim()) === {})"#,
                        role_str,
                        implicit_role_selector(*role),
                        js_string_literal(n)
                    ),
                    None => format!(
                        r#"Array.from(document.querySelectorAll('[role="{}"]')).concat(Array.from(document.querySelectorAll('{}')))"#,
                        role_str,
                        implicit_role_selector(*role)
                    ),
                }
            }

            Selector::TestId(id) => {
                format!(
                    "document.querySelectorAll('[data-testid={}]')",
                    js_string_literal(id)
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
                    js_string_literal(placeholder)
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
        }
    }
}

/// Escape a string for use in JavaScript.
pub(crate) fn js_string_literal(s: &str) -> String {
    let escaped = s
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("'{escaped}'")
}

/// Get CSS selector for elements with implicit ARIA roles.
fn implicit_role_selector(role: AriaRole) -> &'static str {
    match role {
        AriaRole::Button => "button, input[type='button'], input[type='submit'], input[type='reset']",
        AriaRole::Link => "a[href]",
        AriaRole::Heading => "h1, h2, h3, h4, h5, h6",
        AriaRole::ListItem => "li",
        AriaRole::List => "ul, ol",
        AriaRole::TextBox => "input[type='text'], input:not([type]), textarea",
        AriaRole::Checkbox => "input[type='checkbox']",
        AriaRole::Radio => "input[type='radio']",
        AriaRole::Combobox => "select",
        AriaRole::Img => "img",
        AriaRole::Navigation => "nav",
        AriaRole::Main => "main",
        AriaRole::Banner => "header",
        AriaRole::ContentInfo => "footer",
        AriaRole::Form => "form",
        AriaRole::Search => "[role='search']",
        AriaRole::Table => "table",
        AriaRole::Row => "tr",
        AriaRole::Cell => "td",
        AriaRole::ColumnHeader => "th",
        _ => "", // No implicit role mapping
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_selector_js() {
        let selector = Selector::Css("button.submit".to_string());
        let js = selector.to_js_expression();
        assert!(js.contains("querySelectorAll"));
        assert!(js.contains("button.submit"));
    }

    #[test]
    fn test_text_selector_exact_js() {
        let selector = Selector::Text {
            text: "Hello".to_string(),
            exact: true,
        };
        let js = selector.to_js_expression();
        assert!(js.contains("textContent"));
        assert!(js.contains("=== 'Hello'"));
    }

    #[test]
    fn test_text_selector_partial_js() {
        let selector = Selector::Text {
            text: "Hello".to_string(),
            exact: false,
        };
        let js = selector.to_js_expression();
        assert!(js.contains("includes"));
    }

    #[test]
    fn test_role_selector_js() {
        let selector = Selector::Role {
            role: AriaRole::Button,
            name: Some("Submit".to_string()),
        };
        let js = selector.to_js_expression();
        assert!(js.contains("role=\"button\""));
        assert!(js.contains("Submit"));
    }

    #[test]
    fn test_testid_selector_js() {
        let selector = Selector::TestId("my-button".to_string());
        let js = selector.to_js_expression();
        assert!(js.contains("data-testid"));
        assert!(js.contains("my-button"));
    }

    #[test]
    fn test_js_string_escaping() {
        let result = js_string_literal("it's a \"test\"\nwith newlines");
        assert_eq!(result, "'it\\'s a \"test\"\\nwith newlines'");
    }
}
