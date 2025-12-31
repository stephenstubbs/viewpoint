//! JavaScript code for ARIA snapshot capture.
//!
//! This module contains the JavaScript code used to capture ARIA accessibility
//! snapshots from DOM elements.
//!
//! # Frame Boundary Support
//!
//! The JavaScript code detects `<iframe>` and `<frame>` elements and marks them
//! as frame boundaries with the following properties:
//!
//! - `role: "iframe"` - The ARIA role for frame elements
//! - `isFrame: true` - Marks this as a frame boundary
//! - `frameUrl: string | null` - The src URL of the frame
//! - `frameName: string | null` - The name attribute of the frame
//!
//! The code intentionally does NOT access `contentDocument` to avoid security
//! issues with cross-origin frames. Frame content must be captured separately
//! via CDP frame targeting.
//!
//! # Node References
//!
//! When capturing snapshots with node references enabled, elements are collected
//! in an array during traversal. Each snapshot node gets an `elementIndex` that
//! maps to the position in the elements array. After JavaScript execution, the
//! Rust code resolves each element to its CDP `backendNodeId` and formats the
//! ref as `e{backendNodeId}`.

mod snapshot_basic;
mod snapshot_with_refs;

pub use snapshot_basic::aria_snapshot_js;
pub use snapshot_with_refs::aria_snapshot_with_refs_js;
