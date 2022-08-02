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

use crate::objects::dictionary_object::DictionaryObject;
use crate::objects::indirect_object::IndirectObject;
use crate::objects::name_object::NameObject;
use crate::objects::numeric_object::IntegerObject;
use crate::objects::object::Object;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU32, Ordering};

/// This struct represents the basic `Stream` object type. A `Stream` object is
/// a potentially unlimited sequence of bytes used for large amounts of data
/// (e.g., images and page descriptions). All `Stream` object shall be indirect
/// objects.
///
/// # Traits
///
/// This struct derives the following traits:
///
/// - `Debug`
///
/// Additionally, it implements the following traits:
///
/// - [Object](crate::objects::object::Object)
/// - [IndirectObject](crate::objects::indirect_object::IndirectObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `StreamObject::new()`/`StreamObject::new_from_stream()`/
/// `StreamObject::new_from_dictionary_stream()` objects have the same generation
/// number, the copied/cloned objects' indirect references wouldn't be unique,
/// violating the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct StreamObject {
    object_number: u32,
    generation_number: u32,
    dictionary: DictionaryObject,
    stream: String,
}

impl StreamObject {
    /// Returns an empty `StreamObject` (i.e., the stream contains zero bytes).
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    pub fn new(global_object_number_counter: &AtomicU32) -> StreamObject {
        let mut dictionary = DictionaryObject::new(global_object_number_counter);
        dictionary.insert(
            NameObject::new(global_object_number_counter, "Length"),
            IntegerObject::new(global_object_number_counter, 0),
        );

        StreamObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary,
            stream: String::new(),
        }
    }

    /// Returns a `StreamObject` from the given stream and its length.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `stream`: the stream content the `StreamObject` shall assume
    /// - `length`: the length of the stream content
    pub fn new_from_stream<T: Into<String>>(
        global_object_number_counter: &AtomicU32,
        stream: T,
        length: i32,
    ) -> StreamObject {
        let mut dictionary = DictionaryObject::new(global_object_number_counter);
        dictionary.insert(
            NameObject::new(global_object_number_counter, "Length"),
            IntegerObject::new(global_object_number_counter, length),
        );

        StreamObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary,
            stream: stream.into(),
        }
    }

    /// Returns a `StreamObject` from the given stream and dictionary.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `dictionary`: the `StreamObject`'s dictionary
    /// - `stream`:     the stream content the `StreamObject` shall assume
    pub fn new_from_dictionary_stream<T: Into<String>>(
        global_object_number_counter: &AtomicU32,
        dictionary: DictionaryObject,
        stream: T,
    ) -> StreamObject {
        StreamObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary,
            stream: stream.into(),
        }
    }
}

impl Display for StreamObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{}\n", &self.dictionary))?;
        f.write_str("stream\n")?;
        f.write_str(&*format!("{}\n", &self.stream))?;
        f.write_str("endstream")
    }
}

impl Object for StreamObject {}

impl IndirectObject for StreamObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::indirect_object::IndirectObject;
    use crate::objects::stream_object::StreamObject;
    use std::sync::atomic::AtomicU32;

    static GLOBAL_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    #[test]
    fn display_test() {
        let stream_object_0 = StreamObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER);
        let stream_object_1 = StreamObject::new_from_stream(
            &GLOBAL_OBJECT_NUMBER_COUNTER,
            "BT\n/F1 12 Tf\n72 712 Td\n(A stream with an indirect length) Tj\nET",
            63,
        );

        assert_eq!(
            String::from("4 0 obj\n<<\n/Length 0\n>>\nstream\n\nendstream\nendobj"),
            stream_object_0.indirect_definition()
        );
        assert_eq!(
            String::from("8 0 obj\n<<\n/Length 63\n>>\nstream\nBT\n/F1 12 Tf\n72 712 Td\n(A stream with an indirect length) Tj\nET\nendstream\nendobj"),
            stream_object_1.indirect_definition()
        );
    }
}
