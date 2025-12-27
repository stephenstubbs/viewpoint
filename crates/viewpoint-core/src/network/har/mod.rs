//! HAR (HTTP Archive) format support.
//!
//! This module provides types for generating HAR files from network traffic,
//! which can be used for tracing and debugging.

use std::collections::HashMap;

// Re-export types from har_types
pub use super::har_types::{
    Har, HarCache, HarCacheEntry, HarContent, HarCookie, HarCreator, HarEntry, HarHeader, HarLog,
    HarPage, HarPageTimings, HarParam, HarPostData, HarQueryParam, HarRequest, HarResponse,
    HarTimings,
};

impl Har {
    /// Create a new HAR with the given creator name.
    pub fn new(creator_name: &str, creator_version: &str) -> Self {
        Self {
            log: HarLog {
                version: "1.2".to_string(),
                creator: HarCreator {
                    name: creator_name.to_string(),
                    version: creator_version.to_string(),
                },
                browser: None,
                pages: Vec::new(),
                entries: Vec::new(),
                comment: None,
            },
        }
    }

    /// Add a page to the HAR.
    pub fn add_page(&mut self, page: HarPage) {
        self.log.pages.push(page);
    }

    /// Add an entry to the HAR.
    pub fn add_entry(&mut self, entry: HarEntry) {
        self.log.entries.push(entry);
    }

    /// Set the browser info.
    pub fn set_browser(&mut self, name: &str, version: &str) {
        self.log.browser = Some(HarCreator {
            name: name.to_string(),
            version: version.to_string(),
        });
    }
}

impl HarPage {
    /// Create a new page entry.
    pub fn new(id: &str, title: &str, started: &str) -> Self {
        Self {
            started_date_time: started.to_string(),
            id: id.to_string(),
            title: title.to_string(),
            page_timings: HarPageTimings::default(),
            comment: None,
        }
    }

    /// Set page timing information.
    pub fn set_timings(&mut self, on_content_load: Option<f64>, on_load: Option<f64>) {
        self.page_timings.on_content_load = on_content_load;
        self.page_timings.on_load = on_load;
    }
}

impl HarEntry {
    /// Create a new entry.
    pub fn new(started: &str) -> Self {
        Self {
            pageref: None,
            started_date_time: started.to_string(),
            time: 0.0,
            request: HarRequest::default(),
            response: HarResponse::default(),
            cache: HarCache::default(),
            timings: HarTimings::default(),
            server_ip_address: None,
            connection: None,
            comment: None,
        }
    }

    /// Set the request.
    pub fn set_request(&mut self, request: HarRequest) {
        self.request = request;
    }

    /// Set the response.
    pub fn set_response(&mut self, response: HarResponse) {
        self.response = response;
    }

    /// Set timing information.
    pub fn set_timings(&mut self, timings: HarTimings) {
        self.time = timings.total();
        self.timings = timings;
    }

    /// Set server IP address.
    pub fn set_server_ip(&mut self, ip: &str) {
        self.server_ip_address = Some(ip.to_string());
    }
}

impl HarRequest {
    /// Create a new request.
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_string(),
            url: url.to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookies: Vec::new(),
            headers: Vec::new(),
            query_string: Vec::new(),
            post_data: None,
            headers_size: -1,
            body_size: -1,
            comment: None,
        }
    }

    /// Set headers from a `HashMap`.
    pub fn set_headers(&mut self, headers: &HashMap<String, String>) {
        self.headers = headers
            .iter()
            .map(|(name, value)| HarHeader {
                name: name.clone(),
                value: value.clone(),
            })
            .collect();

        // Calculate headers size
        self.headers_size = self
            .headers
            .iter()
            .map(|h| (h.name.len() + h.value.len() + 4) as i64) // ": " + "\r\n"
            .sum();
    }

    /// Set POST data.
    pub fn set_post_data(&mut self, data: Option<&str>, mime_type: Option<&str>) {
        if let Some(text) = data {
            self.post_data = Some(HarPostData {
                mime_type: mime_type.unwrap_or("application/octet-stream").to_string(),
                text: text.to_string(),
                params: None,
            });
            self.body_size = text.len() as i64;
        }
    }

    /// Parse query string from URL.
    pub fn parse_query_string(&mut self) {
        if let Ok(url) = url::Url::parse(&self.url) {
            self.query_string = url
                .query_pairs()
                .map(|(name, value)| HarQueryParam {
                    name: name.to_string(),
                    value: value.to_string(),
                })
                .collect();
        }
    }
}

impl HarResponse {
    /// Create a new response.
    pub fn new(status: i32, status_text: &str) -> Self {
        Self {
            status,
            status_text: status_text.to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookies: Vec::new(),
            headers: Vec::new(),
            content: HarContent::default(),
            redirect_url: String::new(),
            headers_size: -1,
            body_size: -1,
            comment: None,
        }
    }

    /// Create an error response.
    pub fn error(error_text: &str) -> Self {
        Self {
            status: 0,
            status_text: error_text.to_string(),
            http_version: "HTTP/1.1".to_string(),
            cookies: Vec::new(),
            headers: Vec::new(),
            content: HarContent {
                size: 0,
                compression: None,
                mime_type: "x-unknown".to_string(),
                text: Some(error_text.to_string()),
                encoding: None,
                comment: None,
            },
            redirect_url: String::new(),
            headers_size: -1,
            body_size: -1,
            comment: None,
        }
    }

    /// Set headers from a `HashMap`.
    pub fn set_headers(&mut self, headers: &HashMap<String, String>) {
        self.headers = headers
            .iter()
            .map(|(name, value)| HarHeader {
                name: name.clone(),
                value: value.clone(),
            })
            .collect();

        // Calculate headers size
        self.headers_size = self
            .headers
            .iter()
            .map(|h| (h.name.len() + h.value.len() + 4) as i64)
            .sum();
    }

    /// Set response content.
    pub fn set_content(&mut self, text: Option<&str>, mime_type: &str, encoding: Option<&str>) {
        self.content = HarContent {
            size: text.map_or(0, |t| t.len() as i64),
            compression: None,
            mime_type: mime_type.to_string(),
            text: text.map(String::from),
            encoding: encoding.map(String::from),
            comment: None,
        };
        self.body_size = self.content.size;
    }

    /// Set cookies from name-value pairs.
    pub fn set_cookies(&mut self, cookies: &[(String, String)]) {
        self.cookies = cookies
            .iter()
            .map(|(name, value)| HarCookie {
                name: name.clone(),
                value: value.clone(),
                ..Default::default()
            })
            .collect();
    }

    /// Set redirect URL.
    pub fn set_redirect_url(&mut self, url: &str) {
        self.redirect_url = url.to_string();
    }
}

impl HarTimings {
    /// Calculate total time.
    pub fn total(&self) -> f64 {
        let mut total = 0.0;
        if self.blocked > 0.0 {
            total += self.blocked;
        }
        if self.dns > 0.0 {
            total += self.dns;
        }
        if self.connect > 0.0 {
            total += self.connect;
        }
        if self.send > 0.0 {
            total += self.send;
        }
        if self.wait > 0.0 {
            total += self.wait;
        }
        if self.receive > 0.0 {
            total += self.receive;
        }
        total
    }

    /// Create from CDP resource timing.
    pub fn from_resource_timing(
        dns_start: f64,
        dns_end: f64,
        connect_start: f64,
        connect_end: f64,
        ssl_start: f64,
        ssl_end: f64,
        send_start: f64,
        send_end: f64,
        receive_headers_end: f64,
    ) -> Self {
        Self {
            blocked: if dns_start > 0.0 { dns_start } else { -1.0 },
            dns: if dns_end > dns_start {
                dns_end - dns_start
            } else {
                -1.0
            },
            connect: if connect_end > connect_start && ssl_start <= 0.0 {
                connect_end - connect_start
            } else if connect_end > connect_start {
                ssl_start - connect_start
            } else {
                -1.0
            },
            ssl: if ssl_end > ssl_start {
                ssl_end - ssl_start
            } else {
                -1.0
            },
            send: if send_end > send_start {
                send_end - send_start
            } else {
                -1.0
            },
            wait: if receive_headers_end > send_end {
                receive_headers_end - send_end
            } else {
                -1.0
            },
            receive: -1.0, // Will be calculated when response finishes
            comment: None,
        }
    }
}
