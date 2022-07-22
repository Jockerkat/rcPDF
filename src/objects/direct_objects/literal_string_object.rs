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

/// Structure for a literal string object, as defined in ISO 32000-1:2008,
/// 7 "Syntax", 7.3 "Objects", 7.3.4 "String Objects", 7.3.4.2 "Literal Strings".
#[derive(Clone)]
pub struct LiteralStringObject(String);

impl Object for LiteralStringObject {}
impl DirectObject for LiteralStringObject {}
impl StringObject for LiteralStringObject {}

impl fmt::Display for LiteralStringObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl LiteralStringObject {
    pub fn new(value: String) -> LiteralStringObject {
        LiteralStringObject(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::literal_string_object::LiteralStringObject;

    #[test]
    fn display_test() {
        let literal_string_object0: LiteralStringObject = LiteralStringObject::new("This is a string".to_string());
        let literal_string_object1: LiteralStringObject = LiteralStringObject::new(
            "Strings may contain balanced parentheses () and special characters (*!&}^% and so on).".to_string(),
        );

        assert_eq!(format!("{}", literal_string_object0), "(This is a string)");
        assert_eq!(
            format!("{}", literal_string_object1),
            "(Strings may contain balanced parentheses () and special characters (*!&}^% and so on).)"
        );
    }
}
