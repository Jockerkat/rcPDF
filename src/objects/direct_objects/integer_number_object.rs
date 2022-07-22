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

use crate::objects::direct_object::DirectObject;
use crate::objects::direct_objects::numeric_object::NumericObject;
use crate::objects::object::Object;
use std::fmt;

/// Structure for an integer numeric object, as defined in ISO 32000-1:2008,
/// 7 "Syntax", 7.3 "Objects", 7.3.3 "Numeric Objects".
#[derive(Copy, Clone)]
pub struct IntegerObject(i32); // As defined in Annex C

impl Object for IntegerObject {}
impl DirectObject for IntegerObject {}
impl NumericObject for IntegerObject {}

impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntegerObject {
    pub fn new(value: i32) -> IntegerObject {
        IntegerObject(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::integer_number_object::IntegerObject;

    #[test]
    fn display_test() {
        let integer_object0: IntegerObject = IntegerObject::new(5);
        let integer_object1: IntegerObject = IntegerObject::new(-25);

        assert_eq!(format!("{}", integer_object0), "5");
        assert_eq!(format!("{}", integer_object1), "-25");
    }
}
