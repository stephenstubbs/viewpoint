//! Time value types for clock mocking.

/// A time value that can be either a timestamp or an ISO string.
#[derive(Debug, Clone)]
pub enum TimeValue {
    /// Unix timestamp in milliseconds.
    Timestamp(i64),
    /// ISO 8601 formatted string.
    IsoString(String),
}

impl From<i64> for TimeValue {
    fn from(ts: i64) -> Self {
        TimeValue::Timestamp(ts)
    }
}

impl From<u64> for TimeValue {
    fn from(ts: u64) -> Self {
        TimeValue::Timestamp(ts as i64)
    }
}

impl From<&str> for TimeValue {
    fn from(s: &str) -> Self {
        TimeValue::IsoString(s.to_string())
    }
}

impl From<String> for TimeValue {
    fn from(s: String) -> Self {
        TimeValue::IsoString(s)
    }
}
