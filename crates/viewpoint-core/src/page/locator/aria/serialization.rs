//! ARIA snapshot YAML serialization and deserialization.

use crate::error::LocatorError;

use super::{AriaCheckedState, AriaSnapshot};

impl AriaSnapshot {
    /// Convert to a YAML-like string for comparison.
    ///
    /// This format is similar to Playwright's aria snapshot format.
    pub fn to_yaml(&self) -> String {
        let mut output = String::new();
        self.write_yaml(&mut output, 0);
        output
    }

    pub(crate) fn write_yaml(&self, output: &mut String, indent: usize) {
        let prefix = "  ".repeat(indent);

        // Write role and name on the same line
        if let Some(ref role) = self.role {
            output.push_str(&prefix);
            output.push_str("- ");
            output.push_str(role);

            if let Some(ref name) = self.name {
                output.push_str(" \"");
                output.push_str(&name.replace('"', "\\\""));
                output.push('"');
            }

            // Add relevant attributes
            if let Some(disabled) = self.disabled {
                if disabled {
                    output.push_str(" [disabled]");
                }
            }
            if let Some(ref checked) = self.checked {
                match checked {
                    AriaCheckedState::True => output.push_str(" [checked]"),
                    AriaCheckedState::Mixed => output.push_str(" [mixed]"),
                    AriaCheckedState::False => {}
                }
            }
            if let Some(selected) = self.selected {
                if selected {
                    output.push_str(" [selected]");
                }
            }
            if let Some(expanded) = self.expanded {
                if expanded {
                    output.push_str(" [expanded]");
                }
            }
            if let Some(level) = self.level {
                output.push_str(&format!(" [level={level}]"));
            }

            // Add frame boundary marker
            if self.is_frame == Some(true) {
                output.push_str(" [frame-boundary]");
                // Include frame URL if available
                if let Some(ref url) = self.frame_url {
                    output.push_str(&format!(" [frame-url=\"{}\"]", url.replace('"', "\\\"")));
                }
                // Include frame name if available
                if let Some(ref name) = self.frame_name {
                    if !name.is_empty() {
                        output.push_str(&format!(
                            " [frame-name=\"{}\"]",
                            name.replace('"', "\\\"")
                        ));
                    }
                }
            }

            // Add node reference if present
            if let Some(ref node_ref) = self.node_ref {
                output.push_str(&format!(" [ref={node_ref}]"));
            }

            output.push('\n');

            // Write children
            for child in &self.children {
                child.write_yaml(output, indent + 1);
            }
        }
    }

    /// Parse from YAML-like string.
    ///
    /// This supports a simplified YAML-like format for snapshot comparison.
    pub fn from_yaml(yaml: &str) -> Result<Self, LocatorError> {
        let mut root = AriaSnapshot::new();
        root.role = Some("root".to_string());

        let mut stack: Vec<(usize, AriaSnapshot)> = vec![(0, root)];

        for line in yaml.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Calculate indent
            let indent = line.chars().take_while(|c| *c == ' ').count() / 2;
            let trimmed = line.trim();

            if !trimmed.starts_with('-') {
                continue;
            }

            let content = trimmed[1..].trim();

            // Parse role and name
            let (role, name, attrs) = parse_aria_line(content)?;

            let mut node = AriaSnapshot::with_role(role);
            if let Some(n) = name {
                node.name = Some(n);
            }

            // Apply attributes
            for attr in attrs {
                match attr.as_str() {
                    "disabled" => node.disabled = Some(true),
                    "checked" => node.checked = Some(AriaCheckedState::True),
                    "mixed" => node.checked = Some(AriaCheckedState::Mixed),
                    "selected" => node.selected = Some(true),
                    "expanded" => node.expanded = Some(true),
                    "frame-boundary" => node.is_frame = Some(true),
                    s if s.starts_with("level=") => {
                        if let Ok(level) = s[6..].parse() {
                            node.level = Some(level);
                        }
                    }
                    s if s.starts_with("frame-url=\"") && s.ends_with('"') => {
                        // Parse frame-url="value"
                        let url = &s[11..s.len() - 1];
                        node.frame_url = Some(url.replace("\\\"", "\""));
                    }
                    s if s.starts_with("frame-name=\"") && s.ends_with('"') => {
                        // Parse frame-name="value"
                        let name = &s[12..s.len() - 1];
                        node.frame_name = Some(name.replace("\\\"", "\""));
                    }
                    s if s.starts_with("ref=") => {
                        // Parse ref=e12345
                        node.node_ref = Some(s[4..].to_string());
                    }
                    _ => {}
                }
            }

            // Find parent and add as child
            while stack.len() > 1 && stack.last().is_some_and(|(i, _)| *i >= indent) {
                let (_, child) = stack.pop().unwrap();
                if let Some((_, parent)) = stack.last_mut() {
                    parent.children.push(child);
                }
            }

            stack.push((indent, node));
        }

        // Pop remaining items
        while stack.len() > 1 {
            let (_, child) = stack.pop().unwrap();
            if let Some((_, parent)) = stack.last_mut() {
                parent.children.push(child);
            }
        }

        Ok(stack.pop().map(|(_, s)| s).unwrap_or_default())
    }
}

/// Parse an aria line into role, optional name, and attributes.
fn parse_aria_line(content: &str) -> Result<(String, Option<String>, Vec<String>), LocatorError> {
    let mut parts = content.splitn(2, ' ');
    let role = parts.next().unwrap_or("").to_string();

    if role.is_empty() {
        return Err(LocatorError::EvaluationError(
            "Empty role in aria snapshot".to_string(),
        ));
    }

    let rest = parts.next().unwrap_or("");
    let mut name = None;
    let mut attrs = Vec::new();

    // Parse name (quoted string)
    if let Some(start) = rest.find('"') {
        if let Some(end) = rest[start + 1..].find('"') {
            name = Some(rest[start + 1..start + 1 + end].replace("\\\"", "\""));
        }
    }

    // Parse attributes [attr] or [attr=value]
    for part in rest.split('[') {
        if let Some(end) = part.find(']') {
            attrs.push(part[..end].to_string());
        }
    }

    Ok((role, name, attrs))
}
