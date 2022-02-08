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

use super::mm;

/// The margins of an element in the PDF document, in millimeters.
#[derive(Debug)]
pub struct Margins {
    pub left_margin: mm::MM,
    pub right_margin: mm::MM,
    pub top_margin: mm::MM,
    pub bottom_margin: mm::MM,
}

impl Margins {
    pub fn new(left_margin: impl Into<mm::MM>,
               right_margin: impl Into<mm::MM>,
               top_margin: impl Into<mm::MM>,
               bottom_margin: impl Into<mm::MM>) -> Margins {
        Margins {
            left_margin: left_margin.into(),
            right_margin: right_margin.into(),
            top_margin: top_margin.into(),
            bottom_margin: bottom_margin.into(),
        }
    }
}
