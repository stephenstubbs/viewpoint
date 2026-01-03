//! Storage state collection and restoration.
//!
//! This module provides functionality for collecting and restoring browser
//! storage state including cookies, localStorage, and `IndexedDB`.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::debug;
use viewpoint_cdp::CdpConnection;
use viewpoint_js::js;

use tracing::instrument;

use super::BrowserContext;
use super::types::{Cookie, IndexedDbDatabase, LocalStorageEntry, StorageOrigin, StorageState};
use crate::error::ContextError;

// Re-export restore functions for external use
pub use super::storage_restore::{restore_indexed_db, restore_local_storage};

impl BrowserContext {
    /// Get the storage state (cookies and localStorage).
    ///
    /// This method collects cookies and localStorage for all pages in the context.
    /// For more advanced options including `IndexedDB`, use `storage_state_builder()`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::BrowserContext;
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let state = context.storage_state().await?;
    /// state.save("auth.json").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if getting storage state fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn storage_state(&self) -> Result<StorageState, ContextError> {
        self.storage_state_builder().collect().await
    }

    /// Create a builder for collecting storage state with options.
    ///
    /// Use this method when you need to include `IndexedDB` data or configure
    /// other collection options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use viewpoint_core::BrowserContext;
    ///
    /// # async fn example(context: &BrowserContext) -> Result<(), Box<dyn std::error::Error>> {
    /// // Include IndexedDB data
    /// let state = context.storage_state_builder()
    ///     .indexed_db(true)
    ///     .collect()
    ///     .await?;
    ///
    /// state.save("full-state.json").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn storage_state_builder(&self) -> StorageStateBuilder<'_> {
        StorageStateBuilder::new(self.connection(), self.context_id(), &self.pages)
    }
}

/// Options for collecting storage state.
#[derive(Debug, Clone, Default)]
pub struct StorageStateOptions {
    /// Include `IndexedDB` data in the snapshot.
    pub indexed_db: bool,
    /// Maximum entries per `IndexedDB` object store.
    /// Set to 0 for unlimited (default: 1000).
    pub indexed_db_max_entries: usize,
}

impl StorageStateOptions {
    /// Create new default options.
    pub fn new() -> Self {
        Self {
            indexed_db: false,
            indexed_db_max_entries: 1000,
        }
    }

    /// Include `IndexedDB` data in the snapshot.
    #[must_use]
    pub fn indexed_db(mut self, include: bool) -> Self {
        self.indexed_db = include;
        self
    }

    /// Set maximum entries per `IndexedDB` object store.
    #[must_use]
    pub fn indexed_db_max_entries(mut self, max: usize) -> Self {
        self.indexed_db_max_entries = max;
        self
    }
}

/// Builder for collecting storage state with options.
pub struct StorageStateBuilder<'a> {
    connection: &'a Arc<CdpConnection>,
    context_id: &'a str,
    pages: &'a Arc<RwLock<Vec<super::PageInfo>>>,
    options: StorageStateOptions,
}

impl<'a> StorageStateBuilder<'a> {
    pub(crate) fn new(
        connection: &'a Arc<CdpConnection>,
        context_id: &'a str,
        pages: &'a Arc<RwLock<Vec<super::PageInfo>>>,
    ) -> Self {
        Self {
            connection,
            context_id,
            pages,
            options: StorageStateOptions::default(),
        }
    }

    /// Include `IndexedDB` data in the storage state.
    #[must_use]
    pub fn indexed_db(mut self, include: bool) -> Self {
        self.options.indexed_db = include;
        self
    }

    /// Set maximum entries per `IndexedDB` object store.
    #[must_use]
    pub fn indexed_db_max_entries(mut self, max: usize) -> Self {
        self.options.indexed_db_max_entries = max;
        self
    }

    /// Collect the storage state.
    ///
    /// # Errors
    ///
    /// Returns an error if collecting storage state fails.
    pub async fn collect(self) -> Result<StorageState, ContextError> {
        // Collect cookies using the Storage domain
        let cookies = self.collect_cookies().await?;

        let mut origins: HashMap<String, StorageOrigin> = HashMap::new();

        // Get all page sessions for evaluation
        let pages = self.pages.read().await;

        for page in pages.iter() {
            if page.session_id.is_empty() {
                continue;
            }

            // Get the current page URL/origin
            let origin = self.get_page_origin(&page.session_id).await?;
            if origin.is_empty() || origin == "null" {
                continue;
            }

            // Get localStorage for this page
            let local_storage = self.collect_local_storage(&page.session_id).await?;

            // Get IndexedDB if requested
            let indexed_db = if self.options.indexed_db {
                self.collect_indexed_db(&page.session_id).await?
            } else {
                Vec::new()
            };

            // Merge into origins map
            let storage_origin = origins
                .entry(origin.clone())
                .or_insert_with(|| StorageOrigin::new(origin));
            storage_origin.local_storage.extend(local_storage);
            storage_origin.indexed_db.extend(indexed_db);
        }

        Ok(StorageState {
            cookies,
            origins: origins.into_values().collect(),
        })
    }

    /// Collect cookies from the browser context.
    async fn collect_cookies(&self) -> Result<Vec<Cookie>, ContextError> {
        use super::types::SameSite;
        use viewpoint_cdp::protocol::storage::{GetCookiesParams, GetCookiesResult};

        let result: GetCookiesResult = self
            .connection
            .send_command(
                "Storage.getCookies",
                Some(GetCookiesParams::new().browser_context_id(self.context_id.to_string())),
                None,
            )
            .await?;

        let cookies = result
            .cookies
            .into_iter()
            .map(|c| Cookie {
                name: c.name,
                value: c.value,
                domain: Some(c.domain),
                path: Some(c.path),
                url: None,
                expires: if c.expires > 0.0 {
                    Some(c.expires)
                } else {
                    None
                },
                http_only: Some(c.http_only),
                secure: Some(c.secure),
                same_site: c.same_site.map(|s| match s {
                    viewpoint_cdp::protocol::CookieSameSite::Strict => SameSite::Strict,
                    viewpoint_cdp::protocol::CookieSameSite::Lax => SameSite::Lax,
                    viewpoint_cdp::protocol::CookieSameSite::None => SameSite::None,
                }),
            })
            .collect();

        Ok(cookies)
    }

    /// Get the origin URL for a page.
    async fn get_page_origin(&self, session_id: &str) -> Result<String, ContextError> {
        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js! { window.location.origin }.to_string(),
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

        Ok(result
            .result
            .value
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default())
    }

    /// Collect localStorage entries from a page.
    async fn collect_local_storage(
        &self,
        session_id: &str,
    ) -> Result<Vec<LocalStorageEntry>, ContextError> {
        let js = r"
            (function() {
                const entries = [];
                for (let i = 0; i < localStorage.length; i++) {
                    const key = localStorage.key(i);
                    if (key !== null) {
                        entries.push({ name: key, value: localStorage.getItem(key) || '' });
                    }
                }
                return entries;
            })()
        ";

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js.to_string(),
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

        if let Some(value) = result.result.value {
            let entries: Vec<LocalStorageEntry> = serde_json::from_value(value).unwrap_or_default();
            debug!("Collected {} localStorage entries", entries.len());
            Ok(entries)
        } else {
            Ok(Vec::new())
        }
    }

    /// Collect `IndexedDB` databases from a page.
    async fn collect_indexed_db(
        &self,
        session_id: &str,
    ) -> Result<Vec<IndexedDbDatabase>, ContextError> {
        let max_entries = self.options.indexed_db_max_entries.to_string();

        // JavaScript to collect IndexedDB data
        let js_code = js! {
            (async function() {
                const maxEntries = @{max_entries};
                const databases = [];

                if (!window.indexedDB || !window.indexedDB.databases) {
                    return databases;
                }

                const dbList = await window.indexedDB.databases();

                for (const dbInfo of dbList) {
                    if (!dbInfo.name) continue;

                    try {
                        const db = await new Promise((resolve, reject) => {
                            const request = indexedDB.open(dbInfo.name, dbInfo.version);
                            request.onerror = () => reject(request.error);
                            request.onsuccess = () => resolve(request.result);
                        });

                        const dbData = {
                            name: dbInfo.name,
                            version: db.version,
                            stores: []
                        };

                        for (const storeName of db.objectStoreNames) {
                            const tx = db.transaction(storeName, "readonly");
                            const store = tx.objectStore(storeName);

                            const storeData = {
                                name: storeName,
                                keyPath: store.keyPath ? (typeof store.keyPath === "string" ? store.keyPath : store.keyPath.join(",")) : null,
                                autoIncrement: store.autoIncrement,
                                entries: [],
                                indexes: []
                            };

                            // Collect index definitions
                            for (const indexName of store.indexNames) {
                                const index = store.index(indexName);
                                storeData.indexes.push({
                                    name: index.name,
                                    keyPath: typeof index.keyPath === "string" ? index.keyPath : index.keyPath.join(","),
                                    unique: index.unique,
                                    multiEntry: index.multiEntry
                                });
                            }

                            // Collect entries (limited)
                            const entries = await new Promise((resolve, reject) => {
                                const entries = [];
                                const request = store.openCursor();
                                request.onerror = () => reject(request.error);
                                request.onsuccess = (event) => {
                                    const cursor = event.target.result;
                                    if (cursor && (maxEntries === 0 || entries.length < maxEntries)) {
                                        entries.push({ key: cursor.key, value: cursor.value });
                                        cursor.continue();
                                    } else {
                                        resolve(entries);
                                    }
                                };
                            });

                            storeData.entries = entries;
                            dbData.stores.push(storeData);
                        }

                        db.close();
                        databases.push(dbData);
                    } catch (e) {
                        console.warn("Failed to read IndexedDB:", dbInfo.name, e);
                    }
                }

                return databases;
            })()
        };

        let result: viewpoint_cdp::protocol::runtime::EvaluateResult = self
            .connection
            .send_command(
                "Runtime.evaluate",
                Some(viewpoint_cdp::protocol::runtime::EvaluateParams {
                    expression: js_code,
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

        if let Some(value) = result.result.value {
            let databases: Vec<IndexedDbDatabase> =
                serde_json::from_value(value).unwrap_or_default();
            debug!("Collected {} IndexedDB databases", databases.len());
            Ok(databases)
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests;
