## ADDED Requirements

### Requirement: Browser Context Proxy Configuration

The system SHALL support proxy configuration when creating a browser context.

#### Scenario: Create context with HTTP proxy

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("http://proxy:8080")).build().await` is called
- **THEN** all network traffic from pages in that context routes through the proxy

#### Scenario: Create context with SOCKS5 proxy

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("socks5://proxy:1080")).build().await` is called
- **THEN** all network traffic from pages in that context routes through the SOCKS5 proxy

#### Scenario: Proxy with authentication

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("http://proxy:8080").credentials("user", "pass")).build().await` is called
- **THEN** the proxy connection uses the provided credentials

#### Scenario: Proxy with bypass list

- **GIVEN** a browser instance
- **WHEN** `browser.new_context_builder().proxy(ProxyConfig::new("http://proxy:8080").bypass("localhost,*.local")).build().await` is called
- **THEN** requests to localhost and *.local domains bypass the proxy

#### Scenario: Proxy configuration via ContextOptions

- **GIVEN** a ContextOptionsBuilder
- **WHEN** `ContextOptionsBuilder::new().proxy(config).build()` is called
- **THEN** the resulting ContextOptions contains the proxy configuration
