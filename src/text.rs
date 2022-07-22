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

use crate::util::mm::MM;
use crate::util::position::Position;

/// Struct for a textbox which can be added to a page in the PDF document. Holds
/// the the text's alignment,  font family, font size (as an `Option`), bold,
/// italic, the absolute position of the textbox on the page, whether its
/// position should take the page's margins into account, and the width and
/// height (as an `Option`).
#[derive(Clone, Debug)]
pub struct Textbox {
    text: String,
    text_alignment: Option<TextAlignment>,
    font_family: Option<String>,
    font_size: Option<u8>,
    bold: bool,
    italic: bool,
    absolute_position: Position,
    position_from_page_margins: bool,
    width: Option<u32>,
    height: Option<u32>,
}

pub struct TextboxBuilder {
    text: String,
    text_alignment: Option<TextAlignment>,
    font_family: Option<String>,
    font_size: Option<u8>,
    bold: bool,
    italic: bool,
    absolute_position: Position,
    position_from_page_margins: bool,
    width: Option<u32>,
    height: Option<u32>,
}

impl TextboxBuilder {
    /// Create a new textbox positioned at (0,0).
    pub fn new<T: Into<String>>(text: T) -> TextboxBuilder {
        TextboxBuilder {
            text: text.into(),
            text_alignment: None,
            font_family: None,
            font_size: None,
            bold: false,
            italic: false,
            absolute_position: Position::new(0, 0),
            position_from_page_margins: true,
            width: None,
            height: None,
        }
    }

    pub fn text_alignment() {
        todo!()
    }

    /// Set the textbox's font family.
    pub fn font_family<T: Into<String>>(&mut self, font_family: T) -> &mut Self {
        self.font_family = Option::from(font_family.into());
        self
    }

    /// Set the textbox's font size.
    pub fn font_size(&mut self, font_size: u8) -> &mut Self {
        self.font_size = Option::from(font_size);
        self
    }

    /// Set if the textbox's font is bold.
    pub fn bold(&mut self, value: bool) -> &mut Self {
        self.bold = value;
        self
    }

    /// Set if the textbox's font is italic.
    pub fn italic(&mut self, value: bool) -> &mut Self {
        self.italic = value;
        self
    }

    /// Set the absolute position of the textbox on the page (in millimetres) and
    /// whether the page's margins are taken into account (if true, (0,0) is
    /// effectively (`left_margin`,`top_margin`) on the page).
    pub fn absolute_position_in_millimetres(
        &mut self,
        x_lower_left: impl Into<MM>,
        y_lower_left: impl Into<MM>,
        position_from_page_margins: bool,
    ) -> &mut Self {
        self.absolute_position = Position::new(x_lower_left, y_lower_left);
        self.position_from_page_margins = position_from_page_margins;
        self
    }

    /// Set the absolute position of the textbox on the page (in inches).
    pub fn absolute_position_in_inches() {
        todo!()
    }

    pub fn width_in_millimetres() {
        todo!()
    }

    pub fn height_in_millimetres() {
        todo!()
    }

    pub fn width_in_inches() {
        todo!()
    }

    pub fn height_in_inches() {
        todo!()
    }

    /// Construct a `Textbox` struct by copying the values of the `TextboxBuilder`
    /// struct.
    pub fn build(&self) -> Textbox {
        Textbox {
            text: self.text.clone(),
            text_alignment: self.text_alignment,
            font_family: self.font_family.as_ref().cloned(),
            font_size: self.font_size,
            bold: self.bold,
            italic: self.italic,
            absolute_position: self.absolute_position,
            position_from_page_margins: self.position_from_page_margins,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TextAlignment {
    Left,
    Centre,
    Right,
}
