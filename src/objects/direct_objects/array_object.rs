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
use crate::objects::object::Object;
use std::fmt;

/// Structure for an array object, as defined in ISO 32000-1:2008, 7 "Syntax",
/// 7.3 "Objects", 7.3.6 "Array Objects".
pub struct ArrayObject {
    array: Vec<Box<dyn Object>>,
}

impl Object for ArrayObject {}
impl DirectObject for ArrayObject {}

impl fmt::Display for ArrayObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: &str = "";

        f.write_str("[")?;

        for object in &self.array {
            f.write_str(string)?;
            f.write_str(&object.to_string())?;
            string = " ";
        }

        f.write_str("]")?;

        Ok(())
    }
}

impl ArrayObject {
    pub fn new() -> ArrayObject {
        ArrayObject { array: vec![] }
    }

    pub fn add(&mut self, object: impl Object + 'static) {
        self.array.push(Box::new(object))
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::array_object::ArrayObject;
    use crate::objects::direct_objects::boolean_object::BooleanObject;
    use crate::objects::direct_objects::literal_string_object::LiteralStringObject;
    use crate::objects::direct_objects::real_number_object::RealObject;

    #[test]
    fn display_test() {
        let mut array_object0: ArrayObject = ArrayObject::new();

        let boolean_object: BooleanObject = BooleanObject::new(false);
        let literal_string_object: LiteralStringObject = LiteralStringObject::new("A string".to_string());
        let real_number_object: RealObject = RealObject::new(25.125);
        let array_object1: ArrayObject = ArrayObject::new();

        assert_eq!(format!("{}", array_object0), "[]");

        array_object0.add(boolean_object);
        array_object0.add(literal_string_object.clone());
        array_object0.add(real_number_object);
        array_object0.add(array_object1);

        assert_eq!(format!("{}", array_object0), "[false (A string) 25.12500 []]")
    }
}
