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

use crate::objects::direct_objects::dictionary_object::DictionaryObject;
use crate::objects::direct_objects::integer_number_object::IntegerObject;
use crate::objects::direct_objects::name_object::NameObject;
use crate::objects::object::Object;
use crate::INDIRECT_OBJECT_NUMBER_COUNTER;
use std::fmt;
use std::sync::atomic::Ordering;

/// Structure for a stream object, as defined in ISO 32000-1:2008, 7 "Syntax",
/// 7.3 "Objects", 7.3.8 "Stream Objects".
pub struct StreamObject {
    object_number: u32,
    generation_number: u32,
    dictionary: DictionaryObject,
    stream_content: String,
}

impl Object for StreamObject {}
impl crate::objects::indirect_object::IndirectObject for StreamObject {}

impl fmt::Display for StreamObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&*format!("{} {} obj\n", self.object_number, self.generation_number))?;
        f.write_str(&*self.dictionary.to_string())?;
        f.write_str("\nstream\n")?;
        f.write_str(&*self.stream_content)?;
        f.write_str("\nendstream")?;
        f.write_str("\nendobj")?;

        Ok(())
    }
}

impl StreamObject {
    pub fn new() -> StreamObject {
        let mut dictionary: DictionaryObject = DictionaryObject::new();
        dictionary.add(NameObject::new("Length".to_string()), IntegerObject::new(0));

        StreamObject {
            object_number: INDIRECT_OBJECT_NUMBER_COUNTER.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary,
            stream_content: "".to_string(),
        }
    }

    pub fn new_from_stream_content(stream_content: String, length: i32) -> StreamObject {
        let mut dictionary: DictionaryObject = DictionaryObject::new();
        dictionary.add(NameObject::new("Length".to_string()), IntegerObject::new(length));

        StreamObject {
            object_number: INDIRECT_OBJECT_NUMBER_COUNTER.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary,
            stream_content,
        }
    }

    pub fn new_from_stream_content_and_dictionary(
        stream_content: String,
        dictionary: DictionaryObject,
    ) -> StreamObject {
        StreamObject {
            object_number: INDIRECT_OBJECT_NUMBER_COUNTER.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary,
            stream_content,
        }
    }

    pub fn indirect_reference(&self) -> String {
        format!("{} {} R", self.object_number, self.generation_number)
    }
}

// INFO: The tests' success depend on the order they are executed.
// #[cfg(test)]
// mod tests {
//     use crate::objects::direct_objects::{dictionary_object, integer_number_object, name_object};
//     use crate::objects::indirect_objects::stream_object;
//
//     #[test]
//     fn display_test() {
//         let stream_object0: stream_object::StreamObject = stream_object::StreamObject::new();
//
//         let stream_content0: String =
//             String::from("BT\n/F1 12 Tf\n72 712 Td\n(A stream with an indirect length) Tj\nET");
//         let stream_object1: stream_object::StreamObject =
//             stream_object::StreamObject::new_from_stream_content(stream_content0, 77);
//
//         let stream_content1: String = String::from("");
//         let mut dictionary_object: dictionary_object::DictionaryObject = dictionary_object::DictionaryObject::new();
//         dictionary_object.add(
//             name_object::NameObject::new("Length".to_string()),
//             integer_number_object::IntegerObject::new(0),
//         );
//         let stream_object2: stream_object::StreamObject =
//             stream_object::StreamObject::new_from_stream_content_and_dictionary(stream_content1, dictionary_object);
//
//         assert_eq!(
//             format!("{}", stream_object0.to_string()),
//             "4 0 obj\n<<\n/Length 0\n>>\nstream\n\nendstream\nendobj"
//         );
//         assert_eq!(
//             format!("{}", stream_object1.to_string()),
//             "5 0 obj\n<<\n/Length 77\n>>\nstream\nBT\n/F1 12 Tf\n72 712 Td\n(A stream with an indirect length) Tj\nET\nendstream\nendobj"
//         );
//         assert_eq!(
//             format!("{}", stream_object2.to_string()),
//             "6 0 obj\n<<\n/Length 0\n>>\nstream\n\nendstream\nendobj"
//         );
//     }
//
//     #[test]
//     fn indirect_reference_test() {
//         let stream_object: stream_object::StreamObject = stream_object::StreamObject::new();
//
//         assert_eq!(format!("{}", stream_object.indirect_reference()), "7 0 R");
//     }
// }
