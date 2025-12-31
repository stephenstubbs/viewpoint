//! ARIA snapshot matching and diff functionality.

use super::AriaSnapshot;

impl AriaSnapshot {
    /// Check if this snapshot matches another, allowing regex patterns.
    ///
    /// The `expected` snapshot can contain regex patterns in name fields
    /// when enclosed in `/pattern/` syntax.
    pub fn matches(&self, expected: &AriaSnapshot) -> bool {
        // Check role
        if expected.role.is_some() && self.role != expected.role {
            return false;
        }

        // Check name (supports regex)
        if let Some(ref expected_name) = expected.name {
            match &self.name {
                Some(actual_name) => {
                    if !matches_name(expected_name, actual_name) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        // Check other attributes
        if expected.disabled.is_some() && self.disabled != expected.disabled {
            return false;
        }
        if expected.checked.is_some() && self.checked != expected.checked {
            return false;
        }
        if expected.selected.is_some() && self.selected != expected.selected {
            return false;
        }
        if expected.expanded.is_some() && self.expanded != expected.expanded {
            return false;
        }
        if expected.level.is_some() && self.level != expected.level {
            return false;
        }

        // Check children (order matters)
        if expected.children.len() > self.children.len() {
            return false;
        }

        for (i, expected_child) in expected.children.iter().enumerate() {
            if !self
                .children
                .get(i)
                .is_some_and(|c| c.matches(expected_child))
            {
                return false;
            }
        }

        true
    }

    /// Generate a diff between this snapshot and expected.
    pub fn diff(&self, expected: &AriaSnapshot) -> String {
        let actual_yaml = self.to_yaml();
        let expected_yaml = expected.to_yaml();

        if actual_yaml == expected_yaml {
            return String::new();
        }

        let mut diff = String::new();
        diff.push_str("Expected:\n");
        for line in expected_yaml.lines() {
            diff.push_str("  ");
            diff.push_str(line);
            diff.push('\n');
        }
        diff.push_str("\nActual:\n");
        for line in actual_yaml.lines() {
            diff.push_str("  ");
            diff.push_str(line);
            diff.push('\n');
        }

        diff
    }
}

/// Check if a name matches (supports regex patterns).
fn matches_name(pattern: &str, actual: &str) -> bool {
    // Check for regex pattern /.../ or /.../i
    if pattern.starts_with('/') {
        let flags_end = pattern.rfind('/');
        if let Some(end) = flags_end {
            if end > 0 {
                let regex_str = &pattern[1..end];
                let flags = &pattern[end + 1..];
                let case_insensitive = flags.contains('i');

                let regex_result = if case_insensitive {
                    regex::RegexBuilder::new(regex_str)
                        .case_insensitive(true)
                        .build()
                } else {
                    regex::Regex::new(regex_str)
                };

                if let Ok(re) = regex_result {
                    return re.is_match(actual);
                }
            }
        }
    }

    // Exact match
    pattern == actual
}
