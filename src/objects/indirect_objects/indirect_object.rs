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
use crate::INDIRECT_OBJECT_NUMBER_COUNTER;
use std::fmt;
use std::sync::atomic::Ordering;

/// Structure for an indirect object, as defined in ISO 32000-1:2008, 7 "Syntax",
/// 7.3 "Objects", 7.3.10 "Indirect Objects".
pub struct IndirectObject {
    object_number: u32,
    generation_number: u32,
    object: Box<dyn DirectObject>,
}

impl Object for IndirectObject {}
impl crate::objects::indirect_object::IndirectObject for IndirectObject {}

impl fmt::Display for IndirectObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&*format!("{} {} obj\n", self.object_number, self.generation_number))?;
        f.write_str(&self.object.to_string())?;
        f.write_str("\nendobj")?;

        Ok(())
    }
}

impl IndirectObject {
    pub fn new(object: impl DirectObject + 'static) -> IndirectObject {
        IndirectObject {
            object_number: INDIRECT_OBJECT_NUMBER_COUNTER.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            object: Box::new(object),
        }
    }

    pub fn indirect_reference(&self) -> String {
        format!("{} {} R", self.object_number, self.generation_number)
    }
}

// INFO: The tests' success depend on the order they are executed.
// #[cfg(test)]
// mod tests {
//     use crate::objects::direct_objects::{boolean_object, literal_string_object};
//     use crate::objects::indirect_objects::indirect_object;
//
//     #[test]
//     fn display_test() {
//         let boolean_object: boolean_object::BooleanObject = boolean_object::BooleanObject::new(true);
//         let literal_string_object: literal_string_object::LiteralStringObject =
//             literal_string_object::LiteralStringObject::new("Brillig".to_string());
//         let indirect_object0: indirect_object::IndirectObject = indirect_object::IndirectObject::new(boolean_object);
//         let indirect_object1: indirect_object::IndirectObject =
//             indirect_object::IndirectObject::new(literal_string_object);
//
//         assert_eq!(format!("{}", indirect_object0), "1 0 obj\ntrue\nendobj");
//         assert_eq!(format!("{}", indirect_object1), "2 0 obj\n(Brillig)\nendobj");
//     }
//
//     #[test]
//     fn indirect_reference_test() {
//         let boolean_object: boolean_object::BooleanObject = boolean_object::BooleanObject::new(false);
//         let indirect_object: indirect_object::IndirectObject = indirect_object::IndirectObject::new(boolean_object);
//
//         assert_eq!(format!("{}", indirect_object.indirect_reference()), "3 0 R");
//     }
// }
