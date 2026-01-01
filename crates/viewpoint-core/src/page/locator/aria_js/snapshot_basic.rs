//! Basic ARIA snapshot capture JavaScript.
//!
//! This module contains the JavaScript code for capturing ARIA accessibility
//! snapshots without element references.

use viewpoint_js::js;

/// JavaScript code to capture ARIA snapshot from an element.
///
/// This function returns JavaScript code that captures the accessibility tree
/// of a DOM element. It handles:
///
/// - Standard ARIA roles and attributes
/// - Implicit roles from HTML semantics
/// - Frame boundaries for `<iframe>` and `<frame>` elements
/// - Accessible names from various sources (aria-label, labels, content)
///
/// Frame boundaries are marked but their content is NOT traversed (for security
/// reasons). Use `Page.aria_snapshot_with_frames()` to capture multi-frame trees.
pub fn aria_snapshot_js() -> &'static str {
    js! {
        (function(element) {
            // Collect iframe refs during traversal (stored at root level)
            const iframeRefs = [];
            let iframeCounter = 0;

            function getAriaSnapshot(el) {
                if (!el || el.nodeType !== Node.ELEMENT_NODE) {
                    return null;
                }

                // Handle iframe and frame elements as frame boundaries
                // Do NOT attempt to access contentDocument (cross-origin security)
                const tagName = el.tagName.toUpperCase();
                if (tagName === "IFRAME" || tagName === "FRAME") {
                    const frameRef = "frame-" + (iframeCounter++);
                    iframeRefs.push(frameRef);

                    // Get accessible name from name, title, or aria-label
                    const name = el.getAttribute("aria-label") ||
                                 el.getAttribute("title") ||
                                 el.getAttribute("name") ||
                                 null;

                    return {
                        role: "iframe",
                        name: name,
                        isFrame: true,
                        frameUrl: el.src || null,
                        frameName: el.getAttribute("name") || null,
                        frameRef: frameRef
                    };
                }

                // Get computed accessibility info
                const role = el.getAttribute("role") || getImplicitRole(el);
                if (!role) {
                    // For elements without a role, just get children
                    const children = [];
                    for (const child of el.children) {
                        const childSnapshot = getAriaSnapshot(child);
                        if (childSnapshot) {
                            children.push(childSnapshot);
                        }
                    }
                    if (children.length === 1) {
                        return children[0];
                    }
                    if (children.length > 0) {
                        return { role: "group", children: children };
                    }
                    return null;
                }

                const snapshot = { role: role };

                // Get accessible name (pass role for name-from-content logic)
                const name = getAccessibleName(el, role);
                if (name) {
                    snapshot.name = name;
                }

                // Get accessible description
                const desc = el.getAttribute("aria-describedby");
                if (desc) {
                    const descEl = document.getElementById(desc);
                    if (descEl) {
                        snapshot.description = descEl.textContent.trim();
                    }
                }

                // Get state attributes
                if (el.getAttribute("aria-disabled") === "true" || el.disabled) {
                    snapshot.disabled = true;
                }

                const checked = el.getAttribute("aria-checked");
                if (checked === "true") {
                    snapshot.checked = "true";
                } else if (checked === "mixed") {
                    snapshot.checked = "mixed";
                } else if (checked === "false") {
                    snapshot.checked = "false";
                } else if (el.type === "checkbox" || el.type === "radio") {
                    snapshot.checked = el.checked ? "true" : "false";
                }

                if (el.getAttribute("aria-expanded") === "true") {
                    snapshot.expanded = true;
                }

                if (el.getAttribute("aria-selected") === "true") {
                    snapshot.selected = true;
                }

                if (el.getAttribute("aria-pressed") === "true") {
                    snapshot.pressed = true;
                }

                const level = el.getAttribute("aria-level") ||
                    (role === "heading" && el.tagName.match(/H([1-6])/) ? el.tagName[1] : null);
                if (level) {
                    snapshot.level = parseInt(level, 10);
                }

                // Get value attributes
                const valueNow = el.getAttribute("aria-valuenow");
                if (valueNow) snapshot.valueNow = parseFloat(valueNow);
                const valueMin = el.getAttribute("aria-valuemin");
                if (valueMin) snapshot.valueMin = parseFloat(valueMin);
                const valueMax = el.getAttribute("aria-valuemax");
                if (valueMax) snapshot.valueMax = parseFloat(valueMax);
                const valueText = el.getAttribute("aria-valuetext");
                if (valueText) snapshot.valueText = valueText;

                // Get children
                const children = [];
                for (const child of el.children) {
                    const childSnapshot = getAriaSnapshot(child);
                    if (childSnapshot) {
                        children.push(childSnapshot);
                    }
                }
                if (children.length > 0) {
                    snapshot.children = children;
                }

                return snapshot;
            }

            function getImplicitRole(el) {
                const tag = el.tagName.toLowerCase();

                const roleMap = {
                    "a": el.hasAttribute("href") ? "link" : null,
                    "article": "article",
                    "aside": "complementary",
                    "button": "button",
                    "dialog": "dialog",
                    "footer": "contentinfo",
                    "form": "form",
                    "h1": "heading", "h2": "heading", "h3": "heading",
                    "h4": "heading", "h5": "heading", "h6": "heading",
                    "header": "banner",
                    "img": "img",
                    "input": getInputRole(el),
                    "li": "listitem",
                    "main": "main",
                    "nav": "navigation",
                    "ol": "list",
                    "option": "option",
                    "p": "paragraph",
                    "progress": "progressbar",
                    "section": "region",
                    "select": "combobox",
                    "table": "table",
                    "tbody": "rowgroup",
                    "td": "cell",
                    "textarea": "textbox",
                    "th": "columnheader",
                    "tr": "row",
                    "ul": "list"
                };

                return roleMap[tag] || null;
            }

            function getInputRole(el) {
                const type = el.type || "text";
                const typeRoles = {
                    "button": "button",
                    "checkbox": "checkbox",
                    "email": "textbox",
                    "number": "spinbutton",
                    "radio": "radio",
                    "range": "slider",
                    "search": "searchbox",
                    "submit": "button",
                    "tel": "textbox",
                    "text": "textbox",
                    "url": "textbox"
                };
                return typeRoles[type] || "textbox";
            }

            // Roles that support "name from content" per W3C ARIA 1.2 spec
            // https://www.w3.org/TR/wai-aria-1.2/#namefromcontent
            // Note: "paragraph" is added for automation purposes even though W3C spec
            // marks it as "name prohibited". This ensures paragraph text content
            // appears in snapshots for testing/automation use cases.
            const nameFromContentRoles = [
                "button", "cell", "checkbox", "columnheader", "gridcell",
                "heading", "link", "menuitem", "menuitemcheckbox", "menuitemradio",
                "option", "paragraph", "radio", "row", "rowheader", "sectionhead",
                "switch", "tab", "tooltip", "treeitem", "listitem"
            ];

            function getAccessibleName(el, role) {
                // aria-label takes precedence
                const ariaLabel = el.getAttribute("aria-label");
                if (ariaLabel) return ariaLabel;

                // aria-labelledby
                const labelledBy = el.getAttribute("aria-labelledby");
                if (labelledBy) {
                    const labels = labelledBy.split(" ").map(function(id) {
                        const labelEl = document.getElementById(id);
                        return labelEl ? labelEl.textContent.trim() : "";
                    }).filter(Boolean);
                    if (labels.length) return labels.join(" ");
                }

                // For inputs, check associated label
                if (el.tagName === "INPUT" || el.tagName === "TEXTAREA" || el.tagName === "SELECT") {
                    if (el.id) {
                        const label = document.querySelector("label[for=\"" + el.id + "\"]");
                        if (label) return label.textContent.trim();
                    }
                    // Check for wrapping label
                    const parent = el.closest("label");
                    if (parent) {
                        const clone = parent.cloneNode(true);
                        const inner = clone.querySelector("input,textarea,select");
                        if (inner) inner.remove();
                        const text = clone.textContent.trim();
                        if (text) return text;
                    }
                }

                // For images, use alt
                if (el.tagName === "IMG") {
                    return el.getAttribute("alt") || "";
                }

                // For elements with roles that support "name from content",
                // derive the accessible name from text content
                if (role && nameFromContentRoles.includes(role)) {
                    const text = el.textContent.trim();
                    if (text) return text;
                }

                // title attribute as fallback
                const title = el.getAttribute("title");
                if (title) return title;

                return null;
            }

            const result = getAriaSnapshot(element);

            // Attach collected iframe refs to root if any were found
            if (result && iframeRefs.length > 0) {
                result.iframeRefs = iframeRefs;
            }

            return result;
        })
    }
}
