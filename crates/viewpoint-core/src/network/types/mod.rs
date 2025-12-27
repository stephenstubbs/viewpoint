//! Core network types.

use std::fmt;

/// Resource type as it was perceived by the rendering engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// Document resource (HTML).
    Document,
    /// Stylesheet resource (CSS).
    Stylesheet,
    /// Image resource.
    Image,
    /// Media resource (audio/video).
    Media,
    /// Font resource.
    Font,
    /// Script resource (JavaScript).
    Script,
    /// Text track resource.
    TextTrack,
    /// `XMLHttpRequest` resource.
    Xhr,
    /// Fetch API resource.
    Fetch,
    /// Prefetch resource.
    Prefetch,
    /// `EventSource` resource.
    EventSource,
    /// WebSocket resource.
    WebSocket,
    /// Manifest resource.
    Manifest,
    /// Signed exchange resource.
    SignedExchange,
    /// Ping resource.
    Ping,
    /// CSP violation report.
    CspViolationReport,
    /// Preflight request.
    Preflight,
    /// Other resource type.
    Other,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Document => write!(f, "document"),
            Self::Stylesheet => write!(f, "stylesheet"),
            Self::Image => write!(f, "image"),
            Self::Media => write!(f, "media"),
            Self::Font => write!(f, "font"),
            Self::Script => write!(f, "script"),
            Self::TextTrack => write!(f, "texttrack"),
            Self::Xhr => write!(f, "xhr"),
            Self::Fetch => write!(f, "fetch"),
            Self::Prefetch => write!(f, "prefetch"),
            Self::EventSource => write!(f, "eventsource"),
            Self::WebSocket => write!(f, "websocket"),
            Self::Manifest => write!(f, "manifest"),
            Self::SignedExchange => write!(f, "signedexchange"),
            Self::Ping => write!(f, "ping"),
            Self::CspViolationReport => write!(f, "cspviolationreport"),
            Self::Preflight => write!(f, "preflight"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl From<viewpoint_cdp::protocol::network::ResourceType> for ResourceType {
    fn from(cdp_type: viewpoint_cdp::protocol::network::ResourceType) -> Self {
        use viewpoint_cdp::protocol::network::ResourceType as CdpResourceType;
        match cdp_type {
            CdpResourceType::Document => Self::Document,
            CdpResourceType::Stylesheet => Self::Stylesheet,
            CdpResourceType::Image => Self::Image,
            CdpResourceType::Media => Self::Media,
            CdpResourceType::Font => Self::Font,
            CdpResourceType::Script => Self::Script,
            CdpResourceType::TextTrack => Self::TextTrack,
            CdpResourceType::XHR => Self::Xhr,
            CdpResourceType::Fetch => Self::Fetch,
            CdpResourceType::Prefetch => Self::Prefetch,
            CdpResourceType::EventSource => Self::EventSource,
            CdpResourceType::WebSocket => Self::WebSocket,
            CdpResourceType::Manifest => Self::Manifest,
            CdpResourceType::SignedExchange => Self::SignedExchange,
            CdpResourceType::Ping => Self::Ping,
            CdpResourceType::CSPViolationReport => Self::CspViolationReport,
            CdpResourceType::Preflight => Self::Preflight,
            CdpResourceType::Other => Self::Other,
        }
    }
}

/// Error code for aborting requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum AbortError {
    /// Generic failure.
    #[default]
    Failed,
    /// Request was aborted.
    Aborted,
    /// Request timed out.
    TimedOut,
    /// Access was denied.
    AccessDenied,
    /// Connection was closed.
    ConnectionClosed,
    /// Connection was reset.
    ConnectionReset,
    /// Connection was refused.
    ConnectionRefused,
    /// Connection was aborted.
    ConnectionAborted,
    /// Connection failed.
    ConnectionFailed,
    /// Name could not be resolved.
    NameNotResolved,
    /// Internet is disconnected.
    InternetDisconnected,
    /// Address is unreachable.
    AddressUnreachable,
    /// Blocked by client.
    BlockedByClient,
    /// Blocked by response.
    BlockedByResponse,
}


impl fmt::Display for AbortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed => write!(f, "Failed"),
            Self::Aborted => write!(f, "Aborted"),
            Self::TimedOut => write!(f, "TimedOut"),
            Self::AccessDenied => write!(f, "AccessDenied"),
            Self::ConnectionClosed => write!(f, "ConnectionClosed"),
            Self::ConnectionReset => write!(f, "ConnectionReset"),
            Self::ConnectionRefused => write!(f, "ConnectionRefused"),
            Self::ConnectionAborted => write!(f, "ConnectionAborted"),
            Self::ConnectionFailed => write!(f, "ConnectionFailed"),
            Self::NameNotResolved => write!(f, "NameNotResolved"),
            Self::InternetDisconnected => write!(f, "InternetDisconnected"),
            Self::AddressUnreachable => write!(f, "AddressUnreachable"),
            Self::BlockedByClient => write!(f, "BlockedByClient"),
            Self::BlockedByResponse => write!(f, "BlockedByResponse"),
        }
    }
}

impl From<AbortError> for viewpoint_cdp::protocol::fetch::ErrorReason {
    fn from(error: AbortError) -> Self {
        use viewpoint_cdp::protocol::fetch::ErrorReason;
        match error {
            AbortError::Failed => ErrorReason::Failed,
            AbortError::Aborted => ErrorReason::Aborted,
            AbortError::TimedOut => ErrorReason::TimedOut,
            AbortError::AccessDenied => ErrorReason::AccessDenied,
            AbortError::ConnectionClosed => ErrorReason::ConnectionClosed,
            AbortError::ConnectionReset => ErrorReason::ConnectionReset,
            AbortError::ConnectionRefused => ErrorReason::ConnectionRefused,
            AbortError::ConnectionAborted => ErrorReason::ConnectionAborted,
            AbortError::ConnectionFailed => ErrorReason::ConnectionFailed,
            AbortError::NameNotResolved => ErrorReason::NameNotResolved,
            AbortError::InternetDisconnected => ErrorReason::InternetDisconnected,
            AbortError::AddressUnreachable => ErrorReason::AddressUnreachable,
            AbortError::BlockedByClient => ErrorReason::BlockedByClient,
            AbortError::BlockedByResponse => ErrorReason::BlockedByResponse,
        }
    }
}

/// URL pattern for matching requests.
#[derive(Debug, Clone)]
pub enum UrlPattern {
    /// Glob pattern (e.g., "**/api/**", "*.png").
    /// - `*` matches any characters except `/`
    /// - `**` matches any characters including `/`
    Glob(String),

    /// Regular expression pattern.
    Regex(regex::Regex),
}

impl UrlPattern {
    /// Create a glob pattern.
    pub fn glob(pattern: impl Into<String>) -> Self {
        Self::Glob(pattern.into())
    }

    /// Create a regex pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regex is invalid.
    pub fn regex(pattern: &str) -> Result<Self, regex::Error> {
        Ok(Self::Regex(regex::Regex::new(pattern)?))
    }

    /// Check if the URL matches this pattern.
    pub fn matches(&self, url: &str) -> bool {
        match self {
            Self::Glob(pattern) => glob_match(pattern, url),
            Self::Regex(regex) => regex.is_match(url),
        }
    }
}

impl From<&str> for UrlPattern {
    fn from(s: &str) -> Self {
        Self::Glob(s.to_string())
    }
}

impl From<String> for UrlPattern {
    fn from(s: String) -> Self {
        Self::Glob(s)
    }
}

impl From<regex::Regex> for UrlPattern {
    fn from(regex: regex::Regex) -> Self {
        Self::Regex(regex)
    }
}

/// Trait for types that can match URLs.
pub trait UrlMatcher: Send + Sync {
    /// Check if the URL matches.
    fn matches(&self, url: &str) -> bool;
}

impl UrlMatcher for UrlPattern {
    fn matches(&self, url: &str) -> bool {
        self.matches(url)
    }
}

impl UrlMatcher for str {
    fn matches(&self, url: &str) -> bool {
        glob_match(self, url)
    }
}

impl UrlMatcher for String {
    fn matches(&self, url: &str) -> bool {
        glob_match(self, url)
    }
}

impl UrlMatcher for regex::Regex {
    fn matches(&self, url: &str) -> bool {
        self.is_match(url)
    }
}

impl<F> UrlMatcher for F
where
    F: Fn(&str) -> bool + Send + Sync,
{
    fn matches(&self, url: &str) -> bool {
        self(url)
    }
}

/// Match a URL against a glob pattern.
///
/// Pattern syntax:
/// - `*` matches any characters except `/`
/// - `**` matches any characters including `/`
/// - `?` matches a literal `?` (not single character like shell globs)
fn glob_match(pattern: &str, url: &str) -> bool {
    // Convert glob pattern to regex
    let mut regex_pattern = String::with_capacity(pattern.len() * 2);
    regex_pattern.push('^');

    let mut chars = pattern.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if chars.peek() == Some(&'*') {
                    // ** matches anything including /
                    chars.next();
                    // Skip any following / since ** already matches it
                    if chars.peek() == Some(&'/') {
                        chars.next();
                        // **/ matches any path prefix including empty
                        regex_pattern.push_str("(?:.*/)?");
                    } else {
                        regex_pattern.push_str(".*");
                    }
                } else {
                    // * matches anything except /
                    regex_pattern.push_str("[^/]*");
                }
            }
            '?' => {
                // In Playwright's glob, ? matches literal ?
                regex_pattern.push_str("\\?");
            }
            '\\' => {
                // Escape next character
                if let Some(next) = chars.next() {
                    regex_pattern.push('\\');
                    regex_pattern.push(next);
                }
            }
            '.' | '+' | '^' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '|' => {
                // Escape regex special characters
                regex_pattern.push('\\');
                regex_pattern.push(c);
            }
            _ => {
                regex_pattern.push(c);
            }
        }
    }

    regex_pattern.push('$');

    // Compile and match
    regex::Regex::new(&regex_pattern)
        .map(|re| re.is_match(url))
        .unwrap_or(false)
}
