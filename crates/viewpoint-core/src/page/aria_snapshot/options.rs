//! Configuration options for ARIA snapshot capture.

/// Default maximum number of concurrent CDP calls for node resolution.
pub const DEFAULT_MAX_CONCURRENCY: usize = 50;

/// Configuration options for ARIA snapshot capture.
///
/// Use this struct to tune snapshot performance and behavior.
///
/// # Example
///
/// ```no_run
/// use viewpoint_core::SnapshotOptions;
///
/// // Default options
/// let options = SnapshotOptions::default();
///
/// // Skip ref resolution for faster snapshots
/// let options = SnapshotOptions::default().include_refs(false);
///
/// // Increase concurrency for fast networks
/// let options = SnapshotOptions::default().max_concurrency(100);
/// ```
#[derive(Debug, Clone)]
pub struct SnapshotOptions {
    /// Maximum number of concurrent CDP calls for node resolution.
    ///
    /// Higher values improve performance but may overwhelm slow connections.
    /// Default: 50
    pub(crate) max_concurrency: usize,

    /// Whether to include element refs (backendNodeIds) in the snapshot.
    ///
    /// Set to `false` to skip ref resolution for maximum performance when
    /// you only need the accessibility tree structure.
    /// Default: true
    pub(crate) include_refs: bool,
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            max_concurrency: DEFAULT_MAX_CONCURRENCY,
            include_refs: true,
        }
    }
}

impl SnapshotOptions {
    /// Set the maximum number of concurrent CDP calls for node resolution.
    ///
    /// Higher values improve performance but may overwhelm slow connections.
    /// Default: 50
    #[must_use]
    pub fn max_concurrency(mut self, max: usize) -> Self {
        self.max_concurrency = max;
        self
    }

    /// Set whether to include element refs (backendNodeIds) in the snapshot.
    ///
    /// Set to `false` to skip ref resolution for maximum performance when
    /// you only need the accessibility tree structure.
    /// Default: true
    #[must_use]
    pub fn include_refs(mut self, include: bool) -> Self {
        self.include_refs = include;
        self
    }

    /// Get the maximum concurrency setting.
    pub fn get_max_concurrency(&self) -> usize {
        self.max_concurrency
    }

    /// Get whether refs should be included.
    pub fn get_include_refs(&self) -> bool {
        self.include_refs
    }
}
