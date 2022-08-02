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

use crate::objects::indirect_object::IndirectObject;
use crate::objects::object::Object;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};

/// This struct represents the basic `Name` object type. A `Name` object is an
/// atomic symbol uniquely defined by a sequence of any characters (8-bit values)
/// except null (character code 0).
///
/// # Traits
///
/// This struct derives the following trait:
///
/// - `Debug`
///
/// Additionally, it implements the following traits:
///
/// - [Object](crate::objects::object::Object)
/// - [IndirectObject](crate::objects::indirect_object::IndirectObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
/// - `Hash`
/// - `Eq`
/// - `PartialEq`
/// - `Ord`
/// - `PartialOrd`
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `NameObject::new()` objects have the same generation number,
/// the copied/cloned objects' indirect references wouldn't be unique, violating
/// the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct NameObject {
    object_number: u32,
    generation_number: u32,
    name: String,
}

impl NameObject {
    /// Returns a `NameObject` with the given name.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `name`: the name the `NameObject` should have
    pub fn new<T: Into<String>>(global_object_number_counter: &AtomicU32, name: T) -> NameObject {
        NameObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            name: NameObject::sanitise_input(name.into()),
        }
    }

    /// Returns the passed in sequence of characters as a String such that they
    /// follow the rules for names:
    ///
    /// - A '#' (23h) is written as "#23".
    /// - Any regular character (i.e., any character which is not a whitespace
    /// or delimiter character) is written as itself if it is inside the range
    /// of '!' (21h) to '~' (7Eh).
    /// - Any regular character outside the range of '!' (21h) to '~' (7Eh) is
    /// written as its 2-digit hexadecimal code, preceded by a '#'.
    /// - Any character that is not a regular character is written as its 2-digit
    /// hexadecimal code, preceded by a '#'.
    ///
    /// # Arguments
    ///
    /// - `name`: the sequence of characters to check
    fn sanitise_input<T: AsRef<str>>(name: T) -> String {
        // TODO: Limit maximum length of `name` to 127 bytes

        let mut output = String::new();
        let iterator = name.as_ref().chars();

        for character in iterator {
            match character {
                c if character.is_ascii_graphic() => {
                    match c {
                        // hash character or delimiter characters
                        '#' | '(' | ')' | '<' | '>' | '[' | ']' | '{' | '}' | '/' | '%' => {
                            output.push_str(&*format!("#{:X}", c as i32));
                        }
                        // regular characters inside the range of '!' (21h) to '~' (7Eh)
                        _ => {
                            output.push(c);
                        }
                    }
                }
                _ => {
                    output.push_str(&*format!("#{:02X}", character as i32));
                }
            }
        }

        output
    }
}

impl Display for NameObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.name)
    }
}

impl Object for NameObject {}

impl IndirectObject for NameObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

impl Hash for NameObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq<Self> for NameObject {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for NameObject {}

impl PartialOrd<Self> for NameObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for NameObject {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::name_object::NameObject;
    use std::sync::atomic::AtomicU32;

    static GLOBAL_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    #[test]
    fn display_test() {
        let name_object_0 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "");
        let name_object_1 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "Name1");
        let name_object_2 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "ASomewhatLongerName");
        let name_object_3 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "A;Name_With-Various***Characters?");
        let name_object_4 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "1.2");
        let name_object_5 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "$$");
        let name_object_6 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "@pattern");
        let name_object_7 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, ".notdef");
        let name_object_8 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "Lime Green");
        let name_object_9 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "paired()parentheses");
        let name_object_10 = NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "The_Key_of_F#_Minor");

        assert_eq!(String::from("/"), name_object_0.to_string());
        assert_eq!(String::from("/Name1"), name_object_1.to_string());
        assert_eq!(String::from("/ASomewhatLongerName"), name_object_2.to_string());
        assert_eq!(
            String::from("/A;Name_With-Various***Characters?"),
            name_object_3.to_string()
        );
        assert_eq!(String::from("/1.2"), name_object_4.to_string());
        assert_eq!(String::from("/$$"), name_object_5.to_string());
        assert_eq!(String::from("/@pattern"), name_object_6.to_string());
        assert_eq!(String::from("/.notdef"), name_object_7.to_string());
        assert_eq!(String::from("/Lime#20Green"), name_object_8.to_string());
        assert_eq!(String::from("/paired#28#29parentheses"), name_object_9.to_string());
        assert_eq!(String::from("/The_Key_of_F#23_Minor"), name_object_10.to_string());
    }

    #[test]
    fn sanitise_input_test() {
        assert_eq!(String::from("#23"), NameObject::sanitise_input("#"));
        // delimiter characters
        assert_eq!(
            String::from("#28#29#3C#3E#5B#5D#7B#7D#2F#25"),
            NameObject::sanitise_input("()<>[]{}/%")
        );
        // whitespace characters
        assert_eq!(
            String::from("#09#0A#0C#0D#20"),
            NameObject::sanitise_input("\t\n\x0C\r ")
        );
        // regular characters outside of the range of '!' (21h) to '~' (7Eh)
        assert_eq!(String::from("#E4#E9"), NameObject::sanitise_input("äé"));
    }
}
