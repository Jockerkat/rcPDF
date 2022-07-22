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

/// Structure for a name object, as defined in ISO 32000-1:2008, 7 "Syntax",
/// 7.3 "Objects", 7.3.5 "Name Objects".
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct NameObject(String);

impl Object for NameObject {}
impl DirectObject for NameObject {}

impl fmt::Display for NameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}", self.0)
    }
}

impl NameObject {
    pub fn new(value: String) -> NameObject {
        NameObject(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::name_object::NameObject;

    #[test]
    fn display_test() {
        let name_object0: NameObject = NameObject::new("Name1".to_string());
        let name_object1: NameObject = NameObject::new("ASomewhatLongerName".to_string());
        let name_object2: NameObject = NameObject::new("@pattern".to_string());
        let name_object3: NameObject = NameObject::new("A;Name_With-Various***Characters?".to_string());

        assert_eq!(format!("{}", name_object0), "/Name1");
        assert_eq!(format!("{}", name_object1), "/ASomewhatLongerName");
        assert_eq!(format!("{}", name_object2), "/@pattern");
        assert_eq!(format!("{}", name_object3), "/A;Name_With-Various***Characters?");
    }
}
