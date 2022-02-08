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

/// The size of an element in the PDF document, in millimeters.
#[derive(Debug)]
pub struct Size {
    pub width: mm::MM,
    pub height: mm::MM,
}

impl Size {
    pub fn new(width: impl Into<mm::MM>, height: impl Into<mm::MM>) -> Size {
        Size {
            width: width.into(),
            height: height.into(),
        }
    }
}
