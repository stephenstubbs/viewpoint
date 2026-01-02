//! User data directory configuration for browser profiles.

use std::path::PathBuf;

/// User data directory configuration for browser profiles.
///
/// Controls how the browser manages user data (cookies, localStorage, settings).
/// The default is [`UserDataDir::Temp`], which creates an isolated temporary
/// directory that is automatically cleaned up when the browser closes.
///
/// # Breaking Change
///
/// Prior to this change, browsers used the system default profile (`~/.config/chromium/`)
/// by default. To restore the old behavior, use [`UserDataDir::System`] explicitly:
///
/// ```no_run
/// use viewpoint_core::Browser;
///
/// # async fn example() -> Result<(), viewpoint_core::CoreError> {
/// let browser = Browser::launch()
///     .user_data_dir_system()
///     .launch()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub enum UserDataDir {
    /// Create a unique temporary directory per session.
    ///
    /// This is the default mode. Each browser instance gets its own isolated
    /// profile that is automatically deleted when the browser closes or is dropped.
    /// This prevents conflicts when running multiple browser instances concurrently.
    Temp,

    /// Copy a template profile to a temporary directory.
    ///
    /// The template directory contents are copied to a new temporary directory.
    /// The temporary directory is cleaned up when the browser closes.
    /// The original template directory is unchanged.
    ///
    /// Use this when you need pre-configured settings, extensions, or cookies
    /// as a starting point, but still want isolation between sessions.
    TempFromTemplate(PathBuf),

    /// Use a persistent directory for browser data.
    ///
    /// Browser state (cookies, localStorage, settings) persists in the specified
    /// directory across browser restarts. The directory is NOT cleaned up when
    /// the browser closes.
    ///
    /// Note: Using the same persistent directory for multiple concurrent browser
    /// instances will cause profile lock conflicts.
    Persist(PathBuf),

    /// Use the system default profile.
    ///
    /// On Linux, this is typically `~/.config/chromium/`.
    /// No `--user-data-dir` flag is passed to Chromium.
    ///
    /// **Warning**: This can cause conflicts if another Chromium instance is running,
    /// or if a previous session crashed without proper cleanup. Prefer [`UserDataDir::Temp`]
    /// for automation scenarios.
    System,
}
