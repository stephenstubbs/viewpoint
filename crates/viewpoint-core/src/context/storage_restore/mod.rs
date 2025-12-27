//! Storage state restoration functions.
//!
//! This module contains functions for restoring browser storage state
//! including localStorage and IndexedDB.

use std::sync::Arc;

use tracing::debug;
use viewpoint_cdp::CdpConnection;

use super::types::{IndexedDbDatabase, LocalStorageEntry};
use crate::error::ContextError;

/// Restore localStorage entries to a page.
///
/// # Errors
///
/// Returns an error if the storage state cannot be restored.
pub async fn restore_local_storage(
    connection: &Arc<CdpConnection>,
    session_id: &str,
    entries: &[LocalStorageEntry],
) -> Result<(), ContextError> {
    if entries.is_empty() {
        return Ok(());
    }

    let entries_json = serde_json::to_string(entries)
        .map_err(|e| ContextError::Internal(format!("Failed to serialize localStorage: {e}")))?;

    let js = format!(
        r"
        (function() {{
            const entries = {entries_json};
            for (const entry of entries) {{
                localStorage.setItem(entry.name, entry.value);
            }}
        }})()
    "
    );

    connection
        .send_command::<_, serde_json::Value>(
            "Runtime.evaluate",
            Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                expression: js,
                object_group: None,
                include_command_line_api: None,
                silent: Some(true),
                context_id: None,
                return_by_value: Some(true),
                await_promise: Some(false),
            }),
            Some(session_id),
        )
        .await?;

    debug!("Restored {} localStorage entries", entries.len());
    Ok(())
}

/// Restore IndexedDB databases to a page.
///
/// # Errors
///
/// Returns an error if the IndexedDB state cannot be restored.
pub async fn restore_indexed_db(
    connection: &Arc<CdpConnection>,
    session_id: &str,
    databases: &[IndexedDbDatabase],
) -> Result<(), ContextError> {
    if databases.is_empty() {
        return Ok(());
    }

    let db_json = serde_json::to_string(databases)
        .map_err(|e| ContextError::Internal(format!("Failed to serialize IndexedDB: {e}")))?;

    let js = format!(
        r"
        (async function() {{
            const databases = {db_json};
            
            for (const dbData of databases) {{
                // Delete existing database to ensure clean state
                await new Promise((resolve, reject) => {{
                    const request = indexedDB.deleteDatabase(dbData.name);
                    request.onerror = () => reject(request.error);
                    request.onsuccess = () => resolve();
                    request.onblocked = () => resolve(); // Proceed even if blocked
                }});
                
                // Create database with schema
                const db = await new Promise((resolve, reject) => {{
                    const request = indexedDB.open(dbData.name, dbData.version);
                    request.onerror = () => reject(request.error);
                    request.onupgradeneeded = (event) => {{
                        const db = event.target.result;
                        for (const storeData of dbData.stores) {{
                            const options = {{}};
                            if (storeData.keyPath) {{
                                options.keyPath = storeData.keyPath.includes(',') 
                                    ? storeData.keyPath.split(',') 
                                    : storeData.keyPath;
                            }}
                            if (storeData.autoIncrement) {{
                                options.autoIncrement = true;
                            }}
                            
                            const store = db.createObjectStore(storeData.name, options);
                            
                            // Create indexes
                            for (const indexData of (storeData.indexes || [])) {{
                                store.createIndex(indexData.name, 
                                    indexData.keyPath.includes(',') 
                                        ? indexData.keyPath.split(',') 
                                        : indexData.keyPath,
                                    {{ unique: indexData.unique, multiEntry: indexData.multiEntry }}
                                );
                            }}
                        }}
                    }};
                    request.onsuccess = () => resolve(request.result);
                }});
                
                // Restore data
                for (const storeData of dbData.stores) {{
                    if (storeData.entries.length > 0) {{
                        const tx = db.transaction(storeData.name, 'readwrite');
                        const store = tx.objectStore(storeData.name);
                        
                        for (const entry of storeData.entries) {{
                            if (storeData.keyPath) {{
                                store.put(entry.value);
                            }} else {{
                                store.put(entry.value, entry.key);
                            }}
                        }}
                        
                        await new Promise((resolve, reject) => {{
                            tx.oncomplete = () => resolve();
                            tx.onerror = () => reject(tx.error);
                        }});
                    }}
                }}
                
                db.close();
            }}
        }})()
    "
    );

    connection
        .send_command::<_, serde_json::Value>(
            "Runtime.evaluate",
            Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                expression: js,
                object_group: None,
                include_command_line_api: None,
                silent: Some(true),
                context_id: None,
                return_by_value: Some(true),
                await_promise: Some(true),
            }),
            Some(session_id),
        )
        .await?;

    debug!("Restored {} IndexedDB databases", databases.len());
    Ok(())
}
