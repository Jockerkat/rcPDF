// Copyright (C) 2022 Alexander Rolley
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate log;

use crate::util::margins::Margins;
use crate::util::mm::MM;
use std::sync::atomic::AtomicU32;

// Re-exports
pub use page::{Page, PageBuilder, PageOrientation, PageSize};
pub use text::{TextAlignment, Textbox, TextboxBuilder};

mod filters;
mod objects;
mod page;
mod renderer;
mod text;
mod util;

/// Global object number, used by any indirect object in a PDF document.
static INDIRECT_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

/// Struct for a PDF document. Holds the output file's name, the document's pages,
/// and the following default values:
/// - font family
/// - font size
/// - page size
/// - page orientation
/// - page margins
pub struct PDFDocument {
    file_name: String,
    default_font_family: String,
    default_font_size: u8,
    default_page_size: PageSize,
    default_page_orientation: PageOrientation,
    default_page_margins: Margins,
    pages: Vec<Page>,
}

impl PDFDocument {
    /// Add a page to the document.
    pub fn add_page(&mut self, page: Page) -> &mut Self {
        self.pages.push(page);
        self
    }

    /// Render the document and output it to a file.
    pub fn render(&self) {
        renderer::render(self)
    }
}

/// Struct for creating a `PDFDocument` struct using the builder pattern.
pub struct PDFDocumentBuilder {
    file_name: String,
    default_font_family: String,
    default_font_size: u8,
    default_page_size: PageSize,
    default_page_orientation: PageOrientation,
    default_page_margins: Margins,
}

impl PDFDocumentBuilder {
    /// Create a new PDF document by providing the file's name, e.g.,
    /// `mydocument.pdf`. The following values are set by default:
    /// - font family: TBD
    /// - font size: 12
    /// - page size: A4
    /// - page orientation: Portrait
    /// - page margins: no margins
    pub fn new<T: Into<String>>(file_name: T) -> PDFDocumentBuilder {
        PDFDocumentBuilder {
            file_name: file_name.into(),
            default_font_family: String::from("TBD"), // TODO
            default_font_size: 12,
            default_page_size: PageSize::A4,
            default_page_orientation: PageOrientation::Portrait,
            default_page_margins: Margins::new(0, 0, 0, 0),
        }
    }

    /// Set the default/fallback font family used throughout the document.
    pub fn default_font_family<T: Into<String>>(&mut self, font_family: T) -> &mut Self {
        self.default_font_family = font_family.into();
        self
    }

    /// Set the default/fallback font size used throughout the document.
    pub fn default_font_size(&mut self, font_size: u8) -> &mut Self {
        self.default_font_size = font_size;
        self
    }

    /// Set the default/fallback page size used throughout the document.
    pub fn default_page_size(&mut self, page_size: PageSize) -> &mut Self {
        self.default_page_size = page_size;
        self
    }

    /// Set the default/fallback page orientation used throughout the document.
    pub fn default_page_orientation(&mut self, page_orientation: PageOrientation) -> &mut Self {
        self.default_page_orientation = page_orientation;
        self
    }

    /// Set the default/fallback page margins (in millimetres) used throughout the
    /// document.
    pub fn default_page_margins_in_millimetres(
        &mut self,
        top_margin: impl Into<MM>,
        right_margin: impl Into<MM>,
        bottom_margin: impl Into<MM>,
        left_margin: impl Into<MM>,
    ) -> &mut Self {
        self.default_page_margins = Margins::new(top_margin, right_margin, bottom_margin, left_margin);
        self
    }

    /// Set the default/fallback page margins (in inches) used throughout the
    /// document.
    pub fn default_page_margins_in_inches(
        &mut self,
        top_margin: impl Into<MM>,
        right_margin: impl Into<MM>,
        bottom_margin: impl Into<MM>,
        left_margin: impl Into<MM>,
    ) -> &mut Self {
        self.default_page_margins =
            Margins::inches_to_millimetres(top_margin, right_margin, bottom_margin, left_margin);
        self
    }

    /// Construct a `PDFDocument` struct by copying the values of the
    /// `PDFDocumentBuilder` struct.
    pub fn build(&self) -> PDFDocument {
        PDFDocument {
            file_name: self.file_name.clone(),
            default_font_family: self.default_font_family.clone(),
            default_font_size: self.default_font_size,
            default_page_size: self.default_page_size,
            default_page_orientation: self.default_page_orientation,
            default_page_margins: self.default_page_margins,
            pages: Vec::new(),
        }
    }
}
