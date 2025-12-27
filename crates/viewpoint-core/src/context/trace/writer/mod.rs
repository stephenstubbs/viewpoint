//! Trace file writing - zip file creation, JSON serialization, HAR generation.

use std::io::Write;

use chrono::DateTime;

use crate::error::ContextError;
use crate::network::har::{Har, HarEntry, HarRequest, HarResponse};

use super::types::{ResourceEntry, TracingState, TraceFile};

/// Write a trace to a zip file.
///
/// Creates a zip archive containing:
/// - trace.json: The trace data
/// - network.har: Network activity in HAR format
/// - resources/: Screenshots and snapshots
/// - sources/: Source files
pub fn write_trace_zip(
    path: &std::path::Path,
    state: &TracingState,
) -> Result<(), ContextError> {
    use std::fs::File;

    // Create the output file
    let file = File::create(path)
        .map_err(|e| ContextError::Internal(format!("Failed to create trace file: {e}")))?;

    let mut zip = zip::ZipWriter::new(file);

    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // Write the main trace JSON
    let trace_data = build_trace_json(state)?;
    zip.start_file("trace.json", options)
        .map_err(|e| ContextError::Internal(format!("Failed to write trace.json: {e}")))?;
    zip.write_all(trace_data.as_bytes())
        .map_err(|e| ContextError::Internal(format!("Failed to write trace data: {e}")))?;

    // Write HAR file for network activity
    let har_data = build_har(state)?;
    zip.start_file("network.har", options)
        .map_err(|e| ContextError::Internal(format!("Failed to write network.har: {e}")))?;
    zip.write_all(har_data.as_bytes())
        .map_err(|e| ContextError::Internal(format!("Failed to write HAR data: {e}")))?;

    // Write screenshots as resources
    write_screenshots(&mut zip, state, options)?;

    // Write DOM snapshots as resources
    write_snapshots(&mut zip, state, options)?;

    // Write source files
    write_source_files(&mut zip, state, options)?;

    zip.finish()
        .map_err(|e| ContextError::Internal(format!("Failed to finalize zip: {e}")))?;

    Ok(())
}

/// Write screenshots to the zip archive.
fn write_screenshots<W: Write + std::io::Seek>(
    zip: &mut zip::ZipWriter<W>,
    state: &TracingState,
    options: zip::write::SimpleFileOptions,
) -> Result<(), ContextError> {
    for (i, screenshot) in state.screenshots.iter().enumerate() {
        let filename = format!("resources/screenshot-{i}.png");
        zip.start_file(&filename, options)
            .map_err(|e| ContextError::Internal(format!("Failed to write screenshot: {e}")))?;

        use base64::Engine;
        let data = base64::engine::general_purpose::STANDARD
            .decode(&screenshot.data)
            .map_err(|e| ContextError::Internal(format!("Failed to decode screenshot: {e}")))?;
        zip.write_all(&data)
            .map_err(|e| ContextError::Internal(format!("Failed to write screenshot data: {e}")))?;
    }
    Ok(())
}

/// Write DOM snapshots to the zip archive.
fn write_snapshots<W: Write + std::io::Seek>(
    zip: &mut zip::ZipWriter<W>,
    state: &TracingState,
    options: zip::write::SimpleFileOptions,
) -> Result<(), ContextError> {
    for (i, snapshot) in state.snapshots.iter().enumerate() {
        let filename = format!("resources/snapshot-{i}.json");
        zip.start_file(&filename, options)
            .map_err(|e| ContextError::Internal(format!("Failed to write snapshot: {e}")))?;

        let snapshot_data = serde_json::to_string(snapshot)
            .map_err(|e| ContextError::Internal(format!("Failed to serialize snapshot: {e}")))?;
        zip.write_all(snapshot_data.as_bytes())
            .map_err(|e| ContextError::Internal(format!("Failed to write snapshot data: {e}")))?;
    }
    Ok(())
}

/// Write source files to the zip archive.
fn write_source_files<W: Write + std::io::Seek>(
    zip: &mut zip::ZipWriter<W>,
    state: &TracingState,
    options: zip::write::SimpleFileOptions,
) -> Result<(), ContextError> {
    for source in &state.source_files {
        let filename = format!("sources/{}", source.path.replace('\\', "/"));
        zip.start_file(&filename, options)
            .map_err(|e| ContextError::Internal(format!("Failed to write source file: {e}")))?;
        zip.write_all(source.content.as_bytes())
            .map_err(|e| ContextError::Internal(format!("Failed to write source content: {e}")))?;
    }
    Ok(())
}

/// Build the trace JSON data in Playwright-compatible format.
fn build_trace_json(state: &TracingState) -> Result<String, ContextError> {
    // Build resources from screenshots
    let mut resources: Vec<ResourceEntry> = state
        .screenshots
        .iter()
        .enumerate()
        .map(|(i, s)| ResourceEntry {
            name: s.name.clone().unwrap_or_else(|| format!("screenshot-{i}")),
            timestamp: s.timestamp,
            resource_type: "screenshot".to_string(),
            path: format!("resources/screenshot-{i}.png"),
        })
        .collect();

    // Add DOM snapshots as resources
    for (i, snapshot) in state.snapshots.iter().enumerate() {
        let timestamp = snapshot
            .get("timestamp")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.0);
        resources.push(ResourceEntry {
            name: format!("snapshot-{i}"),
            timestamp,
            resource_type: "snapshot".to_string(),
            path: format!("resources/snapshot-{i}.json"),
        });
    }

    // Add source files as resources
    for source in &state.source_files {
        resources.push(ResourceEntry {
            name: source.path.clone(),
            timestamp: 0.0,
            resource_type: "source".to_string(),
            path: format!("sources/{}", source.path.replace('\\', "/")),
        });
    }

    // Build a simplified trace format
    let trace = TraceFile {
        version: "1.0".to_string(),
        name: state.options.name.clone(),
        title: state.options.title.clone(),
        actions: state.actions.clone(),
        events: state.events.clone(),
        resources,
        network: Some("network.har".to_string()),
    };

    serde_json::to_string_pretty(&trace)
        .map_err(|e| ContextError::Internal(format!("Failed to serialize trace: {e}")))
}

/// Build HAR data from network entries.
fn build_har(state: &TracingState) -> Result<String, ContextError> {
    let mut har = Har::new("viewpoint", env!("CARGO_PKG_VERSION"));
    har.set_browser("Chrome", "120.0.0.0");

    // Add pages
    for page in &state.har_pages {
        har.add_page(page.clone());
    }

    // Add network entries
    for entry_state in &state.network_entries {
        let entry = build_har_entry(entry_state, state.current_page_id.as_ref());
        har.add_entry(entry);
    }

    serde_json::to_string_pretty(&har)
        .map_err(|e| ContextError::Internal(format!("Failed to serialize HAR: {e}")))
}

/// Build a single HAR entry from network state.
fn build_har_entry(
    entry_state: &super::types::NetworkEntryState,
    current_page_id: Option<&String>,
) -> HarEntry {
    let wt = entry_state.request.wall_time;
    let started_at = DateTime::from_timestamp(wt as i64, ((wt.fract()) * 1_000_000_000.0) as u32)
        .unwrap_or(entry_state.request.started_at);

    let mut entry = HarEntry::new(&started_at.to_rfc3339());
    entry.pageref = current_page_id.cloned();

    // Build request
    let mut request = HarRequest::new(&entry_state.request.method, &entry_state.request.url);
    request.set_headers(&entry_state.request.headers);
    request.set_post_data(
        entry_state.request.post_data.as_deref(),
        entry_state
            .request
            .headers
            .get("Content-Type")
            .map(std::string::String::as_str),
    );
    request.parse_query_string();
    entry.set_request(request);

    // Build response
    if entry_state.failed {
        let response = HarResponse::error(
            entry_state
                .error_text
                .as_deref()
                .unwrap_or("Request failed"),
        );
        entry.set_response(response);
    } else {
        let mut response = HarResponse::new(entry_state.status, &entry_state.status_text);
        response.set_headers(&entry_state.response_headers);
        response.set_content(None, &entry_state.mime_type, None);
        entry.set_response(response);
    }

    // Set timing
    if let Some(timing) = &entry_state.timing {
        entry.set_timings(timing.clone());
    }

    // Set server IP
    entry.server_ip_address = entry_state.server_ip.clone();

    entry
}
