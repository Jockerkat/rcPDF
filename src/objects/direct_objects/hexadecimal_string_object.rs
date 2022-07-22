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
use crate::objects::direct_objects::string_object::StringObject;
use crate::objects::object::Object;
use std::fmt;

/// Structure for a hexadecimal string object, as defined in ISO 32000-1:2008,
/// 7 "Syntax", 7.3 "Objects", 7.3.4 "String Objects", 7.3.4.3 "Hexadecimal Strings".
#[derive(Clone)]
pub struct HexadecimalStringObject(String);

impl Object for HexadecimalStringObject {}
impl DirectObject for HexadecimalStringObject {}
impl StringObject for HexadecimalStringObject {}

impl fmt::Display for HexadecimalStringObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

impl HexadecimalStringObject {
    pub fn new(value: String) -> HexadecimalStringObject {
        HexadecimalStringObject(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::hexadecimal_string_object::HexadecimalStringObject;

    #[test]
    fn display_test() {
        let hexadecimal_string_object0: HexadecimalStringObject =
            HexadecimalStringObject::new("4E6F762073686D6F7A206B6120706F702E".to_string());
        let hexadecimal_string_object1: HexadecimalStringObject =
            HexadecimalStringObject::new("5468697320697320612074657374".to_string());

        assert_eq!(
            format!("{}", hexadecimal_string_object0),
            "<4E6F762073686D6F7A206B6120706F702E>"
        );
        assert_eq!(
            format!("{}", hexadecimal_string_object1),
            "<5468697320697320612074657374>"
        );
    }
}
