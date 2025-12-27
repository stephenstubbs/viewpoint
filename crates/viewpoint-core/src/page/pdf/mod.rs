//! PDF generation functionality.
//!
//! This module provides the `PdfBuilder` for generating PDFs from pages.

use std::path::Path;

use tracing::{debug, info, instrument};
use viewpoint_cdp::protocol::page::{PrintToPdfParams, PrintToPdfResult};

use crate::error::PageError;

use super::Page;
use super::screenshot::base64_decode;

/// Paper format for PDF generation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum PaperFormat {
    /// Letter size (8.5 x 11 inches).
    #[default]
    Letter,
    /// Legal size (8.5 x 14 inches).
    Legal,
    /// Tabloid size (11 x 17 inches).
    Tabloid,
    /// Ledger size (17 x 11 inches).
    Ledger,
    /// A0 size (33.1 x 46.8 inches).
    A0,
    /// A1 size (23.4 x 33.1 inches).
    A1,
    /// A2 size (16.5 x 23.4 inches).
    A2,
    /// A3 size (11.7 x 16.5 inches).
    A3,
    /// A4 size (8.27 x 11.69 inches).
    A4,
    /// A5 size (5.83 x 8.27 inches).
    A5,
    /// A6 size (4.13 x 5.83 inches).
    A6,
    /// Custom size in inches.
    Custom { width: f64, height: f64 },
}

impl PaperFormat {
    /// Get the width in inches.
    pub fn width(&self) -> f64 {
        match self {
            PaperFormat::Letter => 8.5,
            PaperFormat::Legal => 8.5,
            PaperFormat::Tabloid => 11.0,
            PaperFormat::Ledger => 17.0,
            PaperFormat::A0 => 33.1,
            PaperFormat::A1 => 23.4,
            PaperFormat::A2 => 16.5,
            PaperFormat::A3 => 11.7,
            PaperFormat::A4 => 8.27,
            PaperFormat::A5 => 5.83,
            PaperFormat::A6 => 4.13,
            PaperFormat::Custom { width, .. } => *width,
        }
    }

    /// Get the height in inches.
    pub fn height(&self) -> f64 {
        match self {
            PaperFormat::Letter => 11.0,
            PaperFormat::Legal => 14.0,
            PaperFormat::Tabloid => 17.0,
            PaperFormat::Ledger => 11.0,
            PaperFormat::A0 => 46.8,
            PaperFormat::A1 => 33.1,
            PaperFormat::A2 => 23.4,
            PaperFormat::A3 => 16.5,
            PaperFormat::A4 => 11.69,
            PaperFormat::A5 => 8.27,
            PaperFormat::A6 => 5.83,
            PaperFormat::Custom { height, .. } => *height,
        }
    }
}


/// Margins for PDF generation in inches.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Margins {
    /// Top margin in inches.
    pub top: f64,
    /// Right margin in inches.
    pub right: f64,
    /// Bottom margin in inches.
    pub bottom: f64,
    /// Left margin in inches.
    pub left: f64,
}

impl Margins {
    /// Create uniform margins.
    pub fn uniform(margin: f64) -> Self {
        Self {
            top: margin,
            right: margin,
            bottom: margin,
            left: margin,
        }
    }

    /// Create margins with vertical and horizontal values.
    pub fn symmetric(vertical: f64, horizontal: f64) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create margins with all four values.
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl Default for Margins {
    fn default() -> Self {
        // Default margins match Chromium's defaults
        Self {
            top: 0.4,
            right: 0.4,
            bottom: 0.4,
            left: 0.4,
        }
    }
}

/// Builder for generating PDFs.
#[derive(Debug, Clone)]
pub struct PdfBuilder<'a> {
    page: &'a Page,
    format: PaperFormat,
    landscape: bool,
    margins: Margins,
    scale: f64,
    print_background: bool,
    header_template: Option<String>,
    footer_template: Option<String>,
    page_ranges: Option<String>,
    prefer_css_page_size: bool,
    path: Option<String>,
}

impl<'a> PdfBuilder<'a> {
    /// Create a new PDF builder.
    pub(crate) fn new(page: &'a Page) -> Self {
        Self {
            page,
            format: PaperFormat::default(),
            landscape: false,
            margins: Margins::default(),
            scale: 1.0,
            print_background: false,
            header_template: None,
            footer_template: None,
            page_ranges: None,
            prefer_css_page_size: false,
            path: None,
        }
    }

    /// Set the paper format.
    #[must_use]
    pub fn format(mut self, format: PaperFormat) -> Self {
        self.format = format;
        self
    }

    /// Set landscape orientation.
    #[must_use]
    pub fn landscape(mut self, landscape: bool) -> Self {
        self.landscape = landscape;
        self
    }

    /// Set the margins.
    #[must_use]
    pub fn margins(mut self, margins: Margins) -> Self {
        self.margins = margins;
        self
    }

    /// Set all margins to the same value (in inches).
    #[must_use]
    pub fn margin(mut self, margin: f64) -> Self {
        self.margins = Margins::uniform(margin);
        self
    }

    /// Set each margin individually (in inches).
    #[must_use]
    pub fn margin_all(mut self, top: f64, right: f64, bottom: f64, left: f64) -> Self {
        self.margins = Margins::new(top, right, bottom, left);
        self
    }

    /// Set the scale factor (0.1 to 2.0).
    #[must_use]
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale.clamp(0.1, 2.0);
        self
    }

    /// Print background graphics.
    #[must_use]
    pub fn print_background(mut self, print_background: bool) -> Self {
        self.print_background = print_background;
        self
    }

    /// Set the header template HTML.
    ///
    /// The template can use special classes:
    /// - `date`: current date
    /// - `title`: document title
    /// - `url`: document URL
    /// - `pageNumber`: current page number
    /// - `totalPages`: total pages
    #[must_use]
    pub fn header_template(mut self, template: impl Into<String>) -> Self {
        self.header_template = Some(template.into());
        self
    }

    /// Set the footer template HTML.
    ///
    /// Uses the same special classes as the header template.
    #[must_use]
    pub fn footer_template(mut self, template: impl Into<String>) -> Self {
        self.footer_template = Some(template.into());
        self
    }

    /// Set page ranges (e.g., "1-5, 8, 11-13").
    #[must_use]
    pub fn page_ranges(mut self, ranges: impl Into<String>) -> Self {
        self.page_ranges = Some(ranges.into());
        self
    }

    /// Prefer CSS `@page` size over the specified format.
    #[must_use]
    pub fn prefer_css_page_size(mut self, prefer: bool) -> Self {
        self.prefer_css_page_size = prefer;
        self
    }

    /// Save the PDF to a file.
    #[must_use]
    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_string_lossy().to_string());
        self
    }

    /// Generate the PDF.
    ///
    /// Returns the PDF as a byte buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page is closed
    /// - The CDP command fails
    /// - File saving fails (if a path was specified)
    #[instrument(level = "info", skip(self), fields(format = ?self.format, landscape = self.landscape, has_path = self.path.is_some()))]
    pub async fn generate(self) -> Result<Vec<u8>, PageError> {
        if self.page.is_closed() {
            return Err(PageError::Closed);
        }

        info!("Generating PDF");

        let display_header_footer = self.header_template.is_some() || self.footer_template.is_some();

        let params = PrintToPdfParams {
            landscape: Some(self.landscape),
            display_header_footer: Some(display_header_footer),
            print_background: Some(self.print_background),
            scale: Some(self.scale),
            paper_width: Some(self.format.width()),
            paper_height: Some(self.format.height()),
            margin_top: Some(self.margins.top),
            margin_bottom: Some(self.margins.bottom),
            margin_left: Some(self.margins.left),
            margin_right: Some(self.margins.right),
            page_ranges: self.page_ranges.clone(),
            header_template: self.header_template.clone(),
            footer_template: self.footer_template.clone(),
            prefer_css_page_size: Some(self.prefer_css_page_size),
            transfer_mode: None,
            generate_tagged_pdf: None,
            generate_document_outline: None,
        };

        debug!("Sending Page.printToPDF command");
        let result: PrintToPdfResult = self
            .page
            .connection()
            .send_command("Page.printToPDF", Some(params), Some(self.page.session_id()))
            .await?;

        // Decode base64 data
        let data = base64_decode(&result.data)?;
        debug!(bytes = data.len(), "PDF generated");

        // Save to file if path specified
        if let Some(ref path) = self.path {
            debug!(path = path, "Saving PDF to file");
            tokio::fs::write(path, &data).await.map_err(|e| {
                PageError::EvaluationFailed(format!("Failed to save PDF: {e}"))
            })?;
            info!(path = path, "PDF saved");
        }

        Ok(data)
    }
}
