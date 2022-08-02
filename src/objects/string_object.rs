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
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};

/// This trait represents a generic string object, i.e., literal and hexadecimal
/// strings.
///
/// # Traits
///
/// This trait inherits the [Object](crate::objects::object::Object) trait.
pub trait StringObject: Object {}

/// This struct represents the basic `Literal String` object. `Literal String`
/// objects represent a sequence of literal characters enclosed in parentheses.
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
/// - [StringObject](crate::objects::string_object::StringObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
/// - `Hash`
/// - `Eq`
/// - `PartialEq`
/// - `Ord`
/// - `PartialOrd`
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `LiteralStringObject::new()` objects have the same generation
/// number, the copied/cloned objects' indirect references wouldn't be unique,
/// violating the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct LiteralStringObject {
    object_number: u32,
    generation_number: u32,
    string: String,
}

/// This struct represents the basic `Hexadecimal String` object. `Hexadecimal String`
/// objects represent a sequence of hexadecimal data enclosed in angle brackets.
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
/// - [StringObject](crate::objects::string_object::StringObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
/// - `Hash`
/// - `Eq`
/// - `PartialEq`
/// - `Ord`
/// - `PartialOrd`
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `HexadecimalStringObject::new()` objects have the same generation
/// number, the copied/cloned objects' indirect references wouldn't be unique,
/// violating the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct HexadecimalStringObject {
    object_number: u32,
    generation_number: u32,
    string: String,
}

impl LiteralStringObject {
    /// Returns a `LiteralStringObject` with the given String.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `string`: the string the `LiteralStringObject` should contain
    pub fn new<T: Into<String>>(global_object_number_counter: &AtomicU32, string: T) -> LiteralStringObject {
        LiteralStringObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            string: LiteralStringObject::sanitise_input(string.into()),
        }
    }

    /// Returns the passed in string reference as a String such that it follows
    /// these rules:
    ///
    /// - '\n', '\r', '\t', '\b', '\f' are kept as is
    /// - '(', ')', '\' are escaped to '\(', '\)', and '\\'
    /// - any non-ASCII character is converted to its octal representation
    ///
    /// # Arguments
    ///
    /// - `string`: the string reference to check
    fn sanitise_input<T: AsRef<str>>(string: T) -> String {
        let mut output = String::new();
        let iterator = string.as_ref().chars();

        for character in iterator {
            match character {
                '\n' | '\r' | '\t' | '\x08' | '\x0C' => {
                    output.push(character);
                }
                c if character.is_ascii() => match c {
                    '(' | ')' | '\\' => output.push_str(&*format!("\\{}", c)),
                    _ => {
                        output.push(c);
                    }
                },
                _ => {
                    output.push_str(&*LiteralStringObject::string_to_octal(character.to_string()));
                }
            }
        }

        output
    }

    /// Returns the octal representation of the passed in string reference. Each
    /// octal escape sequence always has three octal digits, with leading zeros
    /// if necessary.
    ///
    /// # Arguments
    ///
    /// - `string`: the string reference to be converted
    fn string_to_octal<T: AsRef<str>>(string: T) -> String {
        let bytes = string.as_ref().as_bytes();
        let mut output = String::new();
        let mut index = 0;

        for byte in bytes {
            let mut b = *byte;
            let mut character_boundary_counter = 0;

            while b > 0 {
                output.insert_str(index, &*(b % 8).to_string());
                b /= 8;
                character_boundary_counter += 1;
            }

            if character_boundary_counter % 3 == 2 {
                output.insert(index, '0');
                character_boundary_counter += 1;
            } else if character_boundary_counter % 3 == 1 {
                output.insert_str(index, "00");
                character_boundary_counter += 2;
            }

            output.insert(index, '\\');
            index += character_boundary_counter + 1;
        }

        output
    }
}

impl StringObject for LiteralStringObject {}

impl Display for LiteralStringObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.string)
    }
}

impl Object for LiteralStringObject {}

impl IndirectObject for LiteralStringObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

impl Hash for LiteralStringObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state)
    }
}

impl PartialEq<Self> for LiteralStringObject {
    fn eq(&self, other: &Self) -> bool {
        self.string.eq(&other.string)
    }
}

impl Eq for LiteralStringObject {}

impl PartialOrd<Self> for LiteralStringObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.string.cmp(&other.string))
    }
}

impl Ord for LiteralStringObject {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.string.cmp(&other.string)
    }
}

impl HexadecimalStringObject {
    /// Returns a `HexadecimalStringObject` with the given String converted to
    /// hexadecimal.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `string`: the string to be converted to hexadecimal, which the
    ///             `LiteralStringObject` should contain
    pub fn new<T: Into<String>>(global_object_number_counter: &AtomicU32, string: T) -> HexadecimalStringObject {
        HexadecimalStringObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            string: HexadecimalStringObject::string_to_hexadecimal(string.into()),
        }
    }

    /// Returns a `HexadecimalStringObject` with the given hexadecimal String.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `string`: the hexadecimal string the `LiteralStringObject` should contain
    ///
    /// # Panics
    ///
    /// Panics if the passed in string reference is not valid hexadecimal
    pub fn new_from_hexadecimal<T: Into<String>>(
        global_object_number_counter: &AtomicU32,
        string: T,
    ) -> HexadecimalStringObject {
        let string = string.into();

        HexadecimalStringObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            string: match HexadecimalStringObject::is_valid_hexadecimal(&string) {
                true => string,
                false => {
                    panic!("Invalid hexadecimal found for input '{}'. Can't construct 'HexadecimalStringObject' from invalid input.", string);
                }
            },
        }
    }

    /// Returns the hexadecimal representation of the passed in string reference.
    ///
    /// # Arguments
    ///
    /// `string`: the string reference to be converted to its hexadecimal
    ///           representation
    fn string_to_hexadecimal<T: AsRef<str>>(string: T) -> String {
        let mut output = String::with_capacity(string.as_ref().len() * 2);

        for character in string.as_ref().as_bytes() {
            output.push_str(&*format!("{:X}", character))
        }

        output
    }

    /// Returns a bool depending on if the passed in string reference is valid
    /// hexadecimal or not.
    ///
    /// # Arguments
    ///
    /// - `string`: the string reference to check
    fn is_valid_hexadecimal<T: AsRef<str>>(string: T) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
        }

        RE.is_match(string.as_ref())
    }
}

impl StringObject for HexadecimalStringObject {}

impl Display for HexadecimalStringObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.string)
    }
}

impl Object for HexadecimalStringObject {}

impl IndirectObject for HexadecimalStringObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

impl Hash for HexadecimalStringObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state)
    }
}

impl PartialEq<Self> for HexadecimalStringObject {
    fn eq(&self, other: &Self) -> bool {
        self.string.eq(&other.string)
    }
}

impl Eq for HexadecimalStringObject {}

impl PartialOrd<Self> for HexadecimalStringObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.string.cmp(&other.string))
    }
}

impl Ord for HexadecimalStringObject {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.string.cmp(&other.string)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::string_object::{HexadecimalStringObject, LiteralStringObject};
    use std::sync::atomic::AtomicU32;

    static GLOBAL_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    #[test]
    fn display_test() {
        let literal_string_object_0 =
            LiteralStringObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "Strings may contain newlines\nand such.");
        let literal_string_object_1 = LiteralStringObject::new(
            &GLOBAL_OBJECT_NUMBER_COUNTER,
            "Strings may contain balanced parentheses () and\nspecial characters (*!&}^% and so on).",
        );
        let literal_string_object_2 = LiteralStringObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "");
        let hexadecimal_string_0 = HexadecimalStringObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "A hexadecimal object.");
        let hexadecimal_string_1 = HexadecimalStringObject::new_from_hexadecimal(
            &GLOBAL_OBJECT_NUMBER_COUNTER,
            "4E6F762073686D6F7A206B6120706F702E",
        );

        assert_eq!(
            String::from("(Strings may contain newlines\nand such.)"),
            literal_string_object_0.to_string()
        );
        assert_eq!(
            String::from(
                "(Strings may contain balanced parentheses \\(\\) and\nspecial characters \\(*!&}^% and so on\\).)"
            ),
            literal_string_object_1.to_string()
        );
        assert_eq!(String::from("()"), literal_string_object_2.to_string());
        assert_eq!(
            String::from("<412068657861646563696D616C206F626A6563742E>"),
            hexadecimal_string_0.to_string()
        );
        assert_eq!(
            String::from("<4E6F762073686D6F7A206B6120706F702E>"),
            hexadecimal_string_1.to_string()
        );
    }

    #[test]
    fn literal_string_sanitise_input_test() {
        assert_eq!(
            "Some special characters: /*-+#^%&\\(\\)=!?\n\r\t\\\\",
            LiteralStringObject::sanitise_input("Some special characters: /*-+#^%&()=!?\n\r\t\\")
        );
        assert_eq!(
            r"Some umlauts: \303\266, \303\244, \303\274",
            LiteralStringObject::sanitise_input("Some umlauts: ö, ä, ü")
        );
    }

    #[test]
    fn string_to_octal_test() {
        assert_eq!(
            r"\127\150\171\040\144\157\156\047\164\040\152\157\153\145\163\040\167\157\162\153\040\151\156\040\157\143\164\141\154\077\040\102\145\143\141\165\163\145\040\067\040\061\060\040\061\061\056",
            LiteralStringObject::string_to_octal("Why don't jokes work in octal? Because 7 10 11.")
        );
        assert_eq!(
            r"\126\157\155\040\303\226\144\151\160\165\163\153\157\155\160\154\145\170\040\155\141\303\237\154\157\163\040\147\145\161\165\303\244\154\164\054\040\303\274\142\164\040\127\151\154\146\162\151\145\144\040\172\171\153\154\151\163\143\150\145\163\040\112\157\144\145\154\156\056",
            LiteralStringObject::string_to_octal("Vom Ödipuskomplex maßlos gequält, übt Wilfried zyklisches Jodeln.")
        );
        assert_eq!(
            r"\123\143\150\167\145\151\303\237\147\145\161\165\303\244\154\164\040\172\303\274\156\144\145\164\040\124\171\160\157\147\162\141\146\040\112\141\153\157\142\040\166\145\162\146\154\151\170\164\040\303\266\144\145\040\120\141\156\147\162\141\155\155\145\040\141\156\056",
            LiteralStringObject::string_to_octal("Schweißgequält zündet Typograf Jakob verflixt öde Pangramme an.")
        );
    }

    #[test]
    fn is_valid_hexadecimal_test() {
        assert!(HexadecimalStringObject::is_valid_hexadecimal(
            "4E6F762073686D6F7A206B6120706F702E"
        ));
        assert!(HexadecimalStringObject::is_valid_hexadecimal(
            "5468697320697320612074657374"
        ));
        assert!(!HexadecimalStringObject::is_valid_hexadecimal("This is not valid hex."));
    }

    #[test]
    fn string_to_hexadecimal() {
        assert_eq!(
            String::from("436F6E76657274206D6521"),
            HexadecimalStringObject::string_to_hexadecimal("Convert me!")
        )
    }
}
