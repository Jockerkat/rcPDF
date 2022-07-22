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

use crate::util::margins::Margins;
use crate::util::mm::MM;
use crate::Textbox;

/// Struct for a page in the PDF document. Holds the page size, page orientation,
/// page margins (all three as an `Option`), and a vector of `Textbox`es.
#[derive(Debug)]
pub struct Page {
    page_size: Option<PageSize>,
    page_orientation: Option<PageOrientation>,
    page_margins: Option<Margins>,
    text_boxes: Vec<Textbox>,
}

impl Page {
    /// Add a `Textbox` to the page.
    pub fn add_textbox(&mut self, textbox: Textbox) -> &mut Self {
        self.text_boxes.push(textbox);
        self
    }
}

/// Struct for creating a `Page` struct using the builder pattern.
pub struct PageBuilder {
    page_size: Option<PageSize>,
    page_orientation: Option<PageOrientation>,
    page_margins: Option<Margins>,
    text_boxes: Vec<Textbox>,
}

impl Default for PageBuilder {
    /// Create a new default page.
    fn default() -> PageBuilder {
        PageBuilder {
            page_size: None,
            page_orientation: None,
            page_margins: None,
            text_boxes: Vec::new(),
        }
    }
}

impl PageBuilder {
    /// Set the page's size.
    pub fn page_size(&mut self, page_size: PageSize) -> &mut Self {
        self.page_size = Option::from(page_size);
        self
    }

    /// Set the page's orientation.
    pub fn page_orientation(&mut self, page_orientation: PageOrientation) -> &mut Self {
        self.page_orientation = Option::from(page_orientation);
        self
    }

    /// Set the page's margins (in millimetres).
    pub fn page_margins_in_millimetres(
        &mut self,
        top_margin: impl Into<MM>,
        right_margin: impl Into<MM>,
        bottom_margin: impl Into<MM>,
        left_margin: impl Into<MM>,
    ) -> &mut Self {
        self.page_margins = Option::from(Margins::new(top_margin, right_margin, bottom_margin, left_margin));
        self
    }

    /// Set the page's margins (in inches).
    pub fn page_margins_in_inches(
        &mut self,
        top_margin: impl Into<MM>,
        right_margin: impl Into<MM>,
        bottom_margin: impl Into<MM>,
        left_margin: impl Into<MM>,
    ) -> &mut Self {
        self.page_margins = Option::from(Margins::inches_to_millimetres(
            top_margin,
            right_margin,
            bottom_margin,
            left_margin,
        ));
        self
    }

    /// Construct a `Page` struct by copying the values of the `PageBuilder`
    /// struct.
    pub fn build(&self) -> Page {
        Page {
            page_size: self.page_size,
            page_orientation: self.page_orientation,
            page_margins: self.page_margins,
            text_boxes: self.text_boxes.to_vec(),
        }
    }
}

/// The size of a page in the PDF document. Possible values:
/// - A0
/// - A1
/// - A2
/// - A3
/// - A4
/// - A5
/// - A6
/// - Executive 1
/// - Executive 2
/// - Executive 3
/// - Legal
/// - Letter
#[derive(Copy, Clone, Debug)]
pub enum PageSize {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    Executive1,
    Executive2,
    Executive3,
    Legal,
    Letter,
}

/// The orientation of a page in the PDF document. Possible values:
/// - Portrait
/// - Landscape
#[derive(Copy, Clone, Debug)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}
