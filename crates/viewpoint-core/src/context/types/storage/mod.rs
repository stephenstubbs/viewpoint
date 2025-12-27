//! Storage state types for browser contexts.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::cookies::Cookie;

/// Browser storage state.
///
/// Contains cookies and localStorage data for persistence across test runs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageState {
    /// Cookies.
    #[serde(default)]
    pub cookies: Vec<Cookie>,
    /// Origins with localStorage data.
    #[serde(default)]
    pub origins: Vec<StorageOrigin>,
}

impl StorageState {
    /// Create a new empty storage state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load storage state from a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub async fn load(path: impl Into<PathBuf>) -> Result<Self, std::io::Error> {
        let path = path.into();
        let content = tokio::fs::read_to_string(&path).await?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Save storage state to a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub async fn save(&self, path: impl Into<PathBuf>) -> Result<(), std::io::Error> {
        let path = path.into();
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        tokio::fs::write(&path, content).await
    }

    /// Generate a JavaScript init script to restore localStorage for all origins.
    ///
    /// The script runs on page load and restores localStorage entries for matching origins.
    /// It checks the current origin and applies only the matching localStorage entries.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let state = StorageState::load("state.json").await?;
    /// let script = state.to_local_storage_init_script();
    /// context.add_init_script(&script).await?;
    /// ```
    pub fn to_local_storage_init_script(&self) -> String {
        if self.origins.is_empty() {
            return String::new();
        }

        // Build a map of origin -> [(key, value), ...]
        let mut origin_data = Vec::new();
        for origin in &self.origins {
            if origin.local_storage.is_empty() {
                continue;
            }

            let entries: Vec<String> = origin
                .local_storage
                .iter()
                .map(|entry| {
                    format!(
                        "[{},{}]",
                        serde_json::to_string(&entry.name).unwrap_or_default(),
                        serde_json::to_string(&entry.value).unwrap_or_default()
                    )
                })
                .collect();

            origin_data.push(format!(
                "[{},[{}]]",
                serde_json::to_string(&origin.origin).unwrap_or_default(),
                entries.join(",")
            ));
        }

        if origin_data.is_empty() {
            return String::new();
        }

        // Generate JavaScript that restores localStorage for the current origin
        format!(
            r"(function() {{
    const storageData = new Map([{}]);
    const currentOrigin = window.location.origin;
    const entries = storageData.get(currentOrigin);
    if (entries) {{
        for (const [key, value] of entries) {{
            try {{
                localStorage.setItem(key, value);
            }} catch (e) {{
                console.warn('Failed to restore localStorage item:', key, e);
            }}
        }}
    }}
}})()",
            origin_data.join(",")
        )
    }

    /// Generate a JavaScript init script to restore `IndexedDB` for all origins.
    ///
    /// The script creates databases and populates them with stored data.
    /// This is more complex than localStorage restoration as it's async.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let state = StorageState::load("state.json").await?;
    /// let script = state.to_indexed_db_init_script();
    /// context.add_init_script(&script).await?;
    /// ```
    pub fn to_indexed_db_init_script(&self) -> String {
        // Collect origins with IndexedDB data
        let origins_with_idb: Vec<_> = self
            .origins
            .iter()
            .filter(|o| !o.indexed_db.is_empty())
            .collect();

        if origins_with_idb.is_empty() {
            return String::new();
        }

        // Build JSON representation of the IndexedDB data
        let mut origin_data = Vec::new();
        for origin in &origins_with_idb {
            let mut db_data = Vec::new();
            for db in &origin.indexed_db {
                let mut store_data = Vec::new();
                for store in &db.stores {
                    let entries: Vec<String> = store
                        .entries
                        .iter()
                        .map(|e| {
                            format!(
                                "{{k:{},v:{}}}",
                                serde_json::to_string(&e.key).unwrap_or_default(),
                                serde_json::to_string(&e.value).unwrap_or_default()
                            )
                        })
                        .collect();

                    let indexes: Vec<String> = store
                        .indexes
                        .iter()
                        .map(|idx| {
                            format!(
                                "{{n:{},kp:{},u:{},me:{}}}",
                                serde_json::to_string(&idx.name).unwrap_or_default(),
                                serde_json::to_string(&idx.key_path).unwrap_or_default(),
                                idx.unique,
                                idx.multi_entry
                            )
                        })
                        .collect();

                    store_data.push(format!(
                        "{{n:{},kp:{},ai:{},e:[{}],i:[{}]}}",
                        serde_json::to_string(&store.name).unwrap_or_default(),
                        store
                            .key_path
                            .as_ref()
                            .map_or("null".to_string(), |kp| serde_json::to_string(kp)
                                .unwrap_or_default()),
                        store.auto_increment,
                        entries.join(","),
                        indexes.join(",")
                    ));
                }

                db_data.push(format!(
                    "{{n:{},v:{},s:[{}]}}",
                    serde_json::to_string(&db.name).unwrap_or_default(),
                    db.version,
                    store_data.join(",")
                ));
            }

            origin_data.push(format!(
                "[{},[{}]]",
                serde_json::to_string(&origin.origin).unwrap_or_default(),
                db_data.join(",")
            ));
        }

        // Generate JavaScript that restores IndexedDB for the current origin
        format!(
            r"(function() {{
    const idbData = new Map([{}]);
    const currentOrigin = window.location.origin;
    const databases = idbData.get(currentOrigin);
    if (!databases) return;
    
    for (const db of databases) {{
        const request = indexedDB.open(db.n, db.v);
        request.onupgradeneeded = (event) => {{
            const idb = event.target.result;
            for (const store of db.s) {{
                const options = {{}};
                if (store.kp !== null) options.keyPath = store.kp;
                if (store.ai) options.autoIncrement = true;
                
                const objectStore = idb.createObjectStore(store.n, options);
                
                for (const idx of store.i) {{
                    objectStore.createIndex(idx.n, idx.kp, {{
                        unique: idx.u,
                        multiEntry: idx.me
                    }});
                }}
            }}
        }};
        request.onsuccess = (event) => {{
            const idb = event.target.result;
            for (const store of db.s) {{
                if (store.e.length === 0) continue;
                try {{
                    const tx = idb.transaction(store.n, 'readwrite');
                    const objectStore = tx.objectStore(store.n);
                    for (const entry of store.e) {{
                        objectStore.put(entry.v, entry.k);
                    }}
                }} catch (e) {{
                    console.warn('Failed to restore IndexedDB store:', store.n, e);
                }}
            }}
            idb.close();
        }};
    }}
}})()",
            origin_data.join(",")
        )
    }
}

/// Origin with localStorage data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageOrigin {
    /// Origin URL.
    pub origin: String,
    /// localStorage entries.
    #[serde(default)]
    pub local_storage: Vec<LocalStorageEntry>,
    /// `IndexedDB` databases (optional).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indexed_db: Vec<IndexedDbDatabase>,
}

impl StorageOrigin {
    /// Create a new storage origin.
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            local_storage: Vec::new(),
            indexed_db: Vec::new(),
        }
    }
}

/// An `IndexedDB` database snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedDbDatabase {
    /// Database name.
    pub name: String,
    /// Database version.
    pub version: u64,
    /// Object stores in the database.
    #[serde(default)]
    pub stores: Vec<IndexedDbObjectStore>,
}

impl IndexedDbDatabase {
    /// Create a new `IndexedDB` database snapshot.
    pub fn new(name: impl Into<String>, version: u64) -> Self {
        Self {
            name: name.into(),
            version,
            stores: Vec::new(),
        }
    }
}

/// An `IndexedDB` object store snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedDbObjectStore {
    /// Object store name.
    pub name: String,
    /// Key path (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
    /// Whether the store uses auto-incrementing keys.
    #[serde(default)]
    pub auto_increment: bool,
    /// Entries in the object store.
    #[serde(default)]
    pub entries: Vec<IndexedDbEntry>,
    /// Index definitions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<IndexedDbIndex>,
}

impl IndexedDbObjectStore {
    /// Create a new object store snapshot.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            key_path: None,
            auto_increment: false,
            entries: Vec::new(),
            indexes: Vec::new(),
        }
    }
}

/// An `IndexedDB` entry (key-value pair).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedDbEntry {
    /// Entry key (JSON-serialized).
    pub key: serde_json::Value,
    /// Entry value (JSON-serialized).
    pub value: serde_json::Value,
}

impl IndexedDbEntry {
    /// Create a new `IndexedDB` entry.
    pub fn new(key: serde_json::Value, value: serde_json::Value) -> Self {
        Self { key, value }
    }
}

/// An `IndexedDB` index definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedDbIndex {
    /// Index name.
    pub name: String,
    /// Key path for the index.
    pub key_path: String,
    /// Whether the index is unique.
    #[serde(default)]
    pub unique: bool,
    /// Whether the index is multi-entry.
    #[serde(default)]
    pub multi_entry: bool,
}

/// A localStorage entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalStorageEntry {
    /// Entry name/key.
    pub name: String,
    /// Entry value.
    pub value: String,
}

impl LocalStorageEntry {
    /// Create a new localStorage entry.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
