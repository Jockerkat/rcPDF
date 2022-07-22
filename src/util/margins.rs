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

/// The margins of an element in the PDF document, in millimeters.
#[derive(Copy, Clone, Debug)]
pub struct Margins {
    top_margin: MM,
    right_margin: MM,
    bottom_margin: MM,
    left_margin: MM,
}

impl Margins {
    pub fn new(
        top_margin: impl Into<MM>,
        right_margin: impl Into<MM>,
        bottom_margin: impl Into<MM>,
        left_margin: impl Into<MM>,
    ) -> Margins {
        Margins {
            top_margin: top_margin.into(),
            right_margin: right_margin.into(),
            bottom_margin: bottom_margin.into(),
            left_margin: left_margin.into(),
        }
    }

    pub fn inches_to_millimetres(
        top_margin_in_inches: impl Into<MM>,
        right_margin_in_inches: impl Into<MM>,
        bottom_margin_in_inches: impl Into<MM>,
        left_margin_in_inches: impl Into<MM>,
    ) -> Margins {
        Margins {
            top_margin: top_margin_in_inches.into() * 25.4,
            right_margin: right_margin_in_inches.into() * 25.4,
            bottom_margin: bottom_margin_in_inches.into() * 25.4,
            left_margin: left_margin_in_inches.into() * 25.4,
        }
    }

    pub fn top_margin(&self) -> MM {
        self.top_margin
    }

    pub fn right_margin(&self) -> MM {
        self.right_margin
    }

    pub fn bottom_margin(&self) -> MM {
        self.bottom_margin
    }

    pub fn left_margin(&self) -> MM {
        self.left_margin
    }
}

#[cfg(test)]
mod tests {
    use crate::util::margins::Margins;
    use crate::util::mm::MM;

    #[test]
    fn positive_integer_values_test() {
        let margins: Margins = Margins::new(5, 5, 15, 15);

        assert_eq!(margins.top_margin(), MM::from(5.0));
        assert_eq!(margins.right_margin(), MM::from(5.0));
        assert_eq!(margins.bottom_margin(), MM::from(15.0));
        assert_eq!(margins.left_margin(), MM::from(15.0));
    }

    #[test]
    fn negative_integer_values_test() {
        let margins: Margins = Margins::new(-5, -5, -15, -15);

        assert_eq!(margins.top_margin(), MM::from(-5.0));
        assert_eq!(margins.right_margin(), MM::from(-5.0));
        assert_eq!(margins.bottom_margin(), MM::from(-15.0));
        assert_eq!(margins.left_margin(), MM::from(-15.0));
    }

    #[test]
    fn mixed_integer_values_test() {
        let margins: Margins = Margins::new(-5, 5, 15, -15);

        assert_eq!(margins.top_margin(), MM::from(-5.0));
        assert_eq!(margins.right_margin(), MM::from(5.0));
        assert_eq!(margins.bottom_margin(), MM::from(15.0));
        assert_eq!(margins.left_margin(), MM::from(-15.0));
    }

    #[test]
    fn positive_float_values_test() {
        let margins: Margins = Margins::new(17.0, 17.0, 3.0, 3.0);

        assert_eq!(margins.top_margin(), MM::from(17.0));
        assert_eq!(margins.right_margin(), MM::from(17.0));
        assert_eq!(margins.bottom_margin(), MM::from(3.0));
        assert_eq!(margins.left_margin(), MM::from(3.0));
    }

    #[test]
    fn negative_float_values_test() {
        let margins: Margins = Margins::new(-17.0, -17.0, -3.0, -3.0);

        assert_eq!(margins.top_margin(), MM::from(-17.0));
        assert_eq!(margins.right_margin(), MM::from(-17.0));
        assert_eq!(margins.bottom_margin(), MM::from(-3.0));
        assert_eq!(margins.left_margin(), MM::from(-3.0));
    }

    #[test]
    fn mixed_float_values_test() {
        let margins: Margins = Margins::new(-17.0, 17.0, 3.0, -3.0);

        assert_eq!(margins.top_margin(), MM::from(-17.0));
        assert_eq!(margins.right_margin(), MM::from(17.0));
        assert_eq!(margins.bottom_margin(), MM::from(3.0));
        assert_eq!(margins.left_margin(), MM::from(-3.0));
    }
}
