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

/// The position/z-height of an element in the PDF document, in millimeters/i8 respectively.
#[derive(Debug)]
pub struct Position {
    pub x_coordinate: mm::MM,
    pub y_coordinate: mm::MM,
    pub z_index: i8,
}

impl Position {
    pub fn new(x_coordinate: impl Into<mm::MM>,
               y_coordinate: impl Into<mm::MM>,
               z_index: i8) -> Position {
        Position {
            x_coordinate: x_coordinate.into(),
            y_coordinate: y_coordinate.into(),
            z_index,
        }
    }
}
