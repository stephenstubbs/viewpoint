# cookies Specification

## Purpose
TBD - created by archiving change add-context-features. Update Purpose after archive.
## Requirements
### Requirement: Add Cookies

The system SHALL allow adding cookies to a browser context.

#### Scenario: Add simple cookie

- **GIVEN** a browser context
- **WHEN** `context.add_cookies(vec![Cookie { name: "session", value: "abc123", url: Some("https://example.com"), ..Default::default() }]).await` is called
- **THEN** the cookie is set for the specified URL

#### Scenario: Add cookie with domain and path

- **GIVEN** a browser context
- **WHEN** a cookie with domain="example.com" and path="/api" is added
- **THEN** the cookie applies to matching requests

#### Scenario: Add cookie with subdomain prefix

- **GIVEN** a browser context
- **WHEN** a cookie with domain=".example.com" is added
- **THEN** the cookie applies to all subdomains

#### Scenario: Add cookie with expiration

- **GIVEN** a browser context
- **WHEN** a cookie with expires set to a future Unix timestamp is added
- **THEN** the cookie expires at that time

#### Scenario: Add httpOnly cookie

- **GIVEN** a browser context
- **WHEN** a cookie with http_only=true is added
- **THEN** the cookie is not accessible via JavaScript

#### Scenario: Add secure cookie

- **GIVEN** a browser context
- **WHEN** a cookie with secure=true is added
- **THEN** the cookie is only sent over HTTPS

#### Scenario: Add SameSite cookie

- **GIVEN** a browser context
- **WHEN** a cookie with same_site=Strict is added
- **THEN** the cookie is only sent for same-site requests

#### Scenario: Add multiple cookies

- **GIVEN** a browser context
- **WHEN** `context.add_cookies(vec![cookie1, cookie2, cookie3]).await` is called
- **THEN** all cookies are added to the context

### Requirement: Get Cookies

The system SHALL allow retrieving cookies from a browser context.

#### Scenario: Get all cookies

- **GIVEN** a browser context with cookies set
- **WHEN** `context.cookies().await` is called
- **THEN** all cookies in the context are returned

#### Scenario: Get cookies for URL

- **GIVEN** a browser context with cookies for multiple domains
- **WHEN** `context.cookies_for_url("https://example.com").await` is called
- **THEN** only cookies that would be sent to that URL are returned

#### Scenario: Get cookies for multiple URLs

- **GIVEN** a browser context with cookies
- **WHEN** `context.cookies_for_urls(vec!["https://a.com", "https://b.com"]).await` is called
- **THEN** cookies for both URLs are returned

#### Scenario: Cookie object contains all properties

- **GIVEN** a cookie was added with all properties set
- **WHEN** `context.cookies().await` is called
- **THEN** the returned cookie contains name, value, domain, path, expires, httpOnly, secure, sameSite

### Requirement: Clear Cookies

The system SHALL allow clearing cookies from a browser context.

#### Scenario: Clear all cookies

- **GIVEN** a browser context with multiple cookies
- **WHEN** `context.clear_cookies().await` is called
- **THEN** all cookies are removed

#### Scenario: Clear cookies by name

- **GIVEN** a browser context with multiple cookies
- **WHEN** `context.clear_cookies().name("session").await` is called
- **THEN** only cookies named "session" are removed

#### Scenario: Clear cookies by domain

- **GIVEN** a browser context with cookies for multiple domains
- **WHEN** `context.clear_cookies().domain("example.com").await` is called
- **THEN** only cookies for that domain are removed

#### Scenario: Clear cookies by domain regex

- **GIVEN** a browser context with cookies for multiple subdomains
- **WHEN** `context.clear_cookies().domain(Regex::new(r".*\.example\.com")?).await` is called
- **THEN** cookies matching the pattern are removed

#### Scenario: Clear cookies by path

- **GIVEN** a browser context with cookies for different paths
- **WHEN** `context.clear_cookies().path("/api").await` is called
- **THEN** only cookies with that path are removed

#### Scenario: Clear cookies with combined filters

- **GIVEN** a browser context with multiple cookies
- **WHEN** `context.clear_cookies().name("token").domain("api.example.com").await` is called
- **THEN** only cookies matching all filters are removed

### Requirement: Cookie Persistence

The system SHALL maintain cookies across page navigations.

#### Scenario: Cookies persist across navigation

- **GIVEN** a cookie is added to the context
- **WHEN** a page navigates to different URLs within the same domain
- **THEN** the cookie is sent with each request

#### Scenario: Cookies shared between pages

- **GIVEN** a cookie is added to the context
- **WHEN** multiple pages are opened in the context
- **THEN** all pages share the same cookies

