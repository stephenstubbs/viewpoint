//! Cookie synchronization between browser and API contexts.

use std::sync::Arc;

use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::HeaderValue;
use tracing::debug;

use crate::context::Cookie;

/// Sync cookies from browser context to a reqwest cookie jar.
///
/// This converts browser cookies to reqwest cookies and adds them to the jar.
pub fn sync_to_jar(cookies: &[Cookie], jar: &Arc<Jar>) {
    for cookie in cookies {
        // Build a cookie URL for reqwest
        let url = cookie_to_url(cookie);

        if let Ok(parsed_url) = url::Url::parse(&url) {
            // Build Set-Cookie header string
            let cookie_str = cookie_to_string(cookie);

            // Create header value and add to jar
            if let Ok(header_value) = HeaderValue::from_str(&cookie_str) {
                // Use a Vec to create a slice iterator that yields references
                let headers = [header_value];
                jar.set_cookies(&mut headers.iter(), &parsed_url);
                debug!("Synced cookie {} to API jar for {}", cookie.name, url);
            }
        }
    }
}

/// Convert a cookie to a URL for use with reqwest's jar.
fn cookie_to_url(cookie: &Cookie) -> String {
    if let Some(ref url) = cookie.url {
        return url.clone();
    }

    let scheme = if cookie.secure.unwrap_or(false) {
        "https"
    } else {
        "http"
    };

    let domain = cookie.domain.as_deref().unwrap_or("localhost");
    let domain = domain.trim_start_matches('.');
    let path = cookie.path.as_deref().unwrap_or("/");

    format!("{scheme}://{domain}{path}")
}

/// Convert a cookie to a Set-Cookie header string.
fn cookie_to_string(cookie: &Cookie) -> String {
    let mut parts = vec![format!("{}={}", cookie.name, cookie.value)];

    if let Some(ref domain) = cookie.domain {
        parts.push(format!("Domain={domain}"));
    }
    if let Some(ref path) = cookie.path {
        parts.push(format!("Path={path}"));
    }
    if cookie.secure.unwrap_or(false) {
        parts.push("Secure".to_string());
    }
    if cookie.http_only.unwrap_or(false) {
        parts.push("HttpOnly".to_string());
    }
    if let Some(same_site) = &cookie.same_site {
        parts.push(format!(
            "SameSite={}",
            match same_site {
                crate::context::SameSite::Strict => "Strict",
                crate::context::SameSite::Lax => "Lax",
                crate::context::SameSite::None => "None",
            }
        ));
    }
    if let Some(expires) = cookie.expires {
        // Convert Unix timestamp to HTTP date
        if let Some(dt) = chrono::DateTime::from_timestamp(expires as i64, 0) {
            parts.push(format!(
                "Expires={}",
                dt.format("%a, %d %b %Y %H:%M:%S GMT")
            ));
        }
    }

    parts.join("; ")
}

/// Extract cookies from a reqwest cookie jar for a given URL.
///
/// Returns a list of cookies that would be sent for the given URL.
pub fn extract_from_jar(jar: &Arc<Jar>, url: &str) -> Vec<Cookie> {
    let mut cookies = Vec::new();

    if let Ok(parsed_url) = url::Url::parse(url) {
        if let Some(cookie_header) = jar.cookies(&parsed_url) {
            // Parse the cookie header
            let header_str = cookie_header.to_str().unwrap_or("");
            for cookie_str in header_str.split("; ") {
                if let Some((name, value)) = cookie_str.split_once('=') {
                    cookies.push(Cookie {
                        name: name.to_string(),
                        value: value.to_string(),
                        domain: parsed_url.host_str().map(String::from),
                        path: Some(parsed_url.path().to_string()),
                        url: Some(url.to_string()),
                        expires: None,
                        http_only: None,
                        secure: Some(parsed_url.scheme() == "https"),
                        same_site: None,
                    });
                }
            }
        }
    }

    cookies
}

#[cfg(test)]
mod tests;
