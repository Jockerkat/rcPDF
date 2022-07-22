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

use crate::MM;

/// The position of an element in the PDF document, in millimeters.
#[derive(Copy, Clone, Debug)]
pub struct Position {
    x_lower_left: MM,
    y_lower_left: MM,
}

impl Position {
    pub fn new(x_lower_left: impl Into<MM>, y_lower_left: impl Into<MM>) -> Position {
        Position {
            x_lower_left: x_lower_left.into(),
            y_lower_left: y_lower_left.into(),
        }
    }

    pub fn x_lower_left(&self) -> MM {
        self.x_lower_left
    }

    pub fn y_lower_left(&self) -> MM {
        self.y_lower_left
    }
}

#[cfg(test)]
mod tests {
    use crate::util::mm::MM;
    use crate::util::position::Position;

    #[test]
    fn positive_integer_values_test() {
        let position: Position = Position::new(5, 5);

        assert_eq!(position.x_lower_left(), MM::from(5.0));
        assert_eq!(position.y_lower_left(), MM::from(5.0));
    }

    #[test]
    fn negative_integer_values_test() {
        let position: Position = Position::new(-5, -5);

        assert_eq!(position.x_lower_left(), MM::from(-5.0));
        assert_eq!(position.y_lower_left(), MM::from(-5.0));
    }

    #[test]
    fn mixed_integer_values_test() {
        let position: Position = Position::new(5, -5);

        assert_eq!(position.x_lower_left(), MM::from(5.0));
        assert_eq!(position.y_lower_left(), MM::from(-5.0));
    }

    #[test]
    fn positive_float_values_test() {
        let position: Position = Position::new(175.0, 175.0);

        assert_eq!(position.x_lower_left(), MM::from(175.0));
        assert_eq!(position.y_lower_left(), MM::from(175.0));
    }

    #[test]
    fn negative_float_values_test() {
        let position: Position = Position::new(-175.0, -175.0);

        assert_eq!(position.x_lower_left(), MM::from(-175.0));
        assert_eq!(position.y_lower_left(), MM::from(-175.0));
    }

    #[test]
    fn mixed_float_values_test() {
        let position: Position = Position::new(175.0, -175.0);

        assert_eq!(position.x_lower_left(), MM::from(175.0));
        assert_eq!(position.y_lower_left(), MM::from(-175.0));
    }
}
