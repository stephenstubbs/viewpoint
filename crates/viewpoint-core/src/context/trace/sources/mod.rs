//! Source file collection for tracing.

use crate::error::ContextError;

/// Collect source files from a directory recursively.
///
/// Returns a list of (relative_path, content) tuples.
pub fn collect_sources_from_dir(
    dir: &std::path::Path,
    extensions: &[&str],
) -> Result<Vec<(String, String)>, ContextError> {
    let mut files = Vec::new();
    collect_recursive(dir, dir, extensions, &mut files)?;
    Ok(files)
}

/// Recursively collect source files.
fn collect_recursive(
    dir: &std::path::Path,
    base: &std::path::Path,
    extensions: &[&str],
    files: &mut Vec<(String, String)>,
) -> Result<(), ContextError> {
    if !dir.is_dir() {
        return Ok(());
    }

    let entries = std::fs::read_dir(dir)
        .map_err(|e| ContextError::Internal(format!("Failed to read directory: {e}")))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_recursive(&path, base, extensions, files)?;
        } else if let Some(ext) = path.extension() {
            if extensions.iter().any(|e| ext == *e) {
                let rel_path = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| ContextError::Internal(format!("Failed to read file: {e}")))?;
                files.push((rel_path, content));
            }
        }
    }

    Ok(())
}
