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

/// The rotation of an element in the PDF document, in arc degrees.
#[derive(Debug)]
pub struct Rotation {
    pub arc_degrees: f64,
}

impl Rotation {
    pub fn new(arc_degrees: f64) -> Rotation {
        let normalised_degrees: f64 = arc_degrees % 360.0;

        let degrees: f64 = if normalised_degrees > 180.0 {
            normalised_degrees - 360.0
        } else if normalised_degrees < -180.0 {
            360.0 + normalised_degrees
        } else {
            normalised_degrees
        };

        Rotation {
            arc_degrees: degrees,
        }
    }
}
