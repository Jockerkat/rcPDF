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
use crate::objects::direct_objects::array_object::ArrayObject;
use crate::objects::direct_objects::real_number_object::RealObject;

/// An array object used to describe locations on a page and bounding boxes for
/// a variety of objects.
pub struct Rectangle {
    array_object: ArrayObject,
}

// TODO: Use this for the renderer
impl Rectangle {
    pub fn new(x_lower_left: f32, y_lower_left: f32, x_upper_left: f32, y_upper_left: f32) -> Rectangle {
        let mut array_object = ArrayObject::new();

        array_object.add(RealObject::new(x_lower_left));
        array_object.add(RealObject::new(y_lower_left));
        array_object.add(RealObject::new(x_upper_left));
        array_object.add(RealObject::new(y_upper_left));

        Rectangle { array_object }
    }

    pub fn array_object(&self) -> &ArrayObject {
        &self.array_object
    }
}

#[cfg(test)]
mod tests {
    use crate::util::rectangle::Rectangle;

    #[test]
    fn basic_rectangle_test() {
        let rectangle = Rectangle::new(0.0, 0.0, 100.0, 100.0);

        assert_eq!(
            "[0.00000 0.00000 100.00000 100.00000]",
            format!("{}", rectangle.array_object())
        );
    }
}
