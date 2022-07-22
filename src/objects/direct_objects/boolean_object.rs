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

/// Structure for boolean object, as defined in ISO 32000-1:2008, 7 "Syntax",
/// 7.3 "Objects", 7.3.2 "Boolean Objects".
#[derive(Copy, Clone)]
pub struct BooleanObject {
    value: bool,
}

impl Object for BooleanObject {}
impl DirectObject for BooleanObject {}

impl fmt::Display for BooleanObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl BooleanObject {
    pub fn new(value: bool) -> BooleanObject {
        BooleanObject { value }
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::boolean_object::BooleanObject;

    #[test]
    fn display_test() {
        let boolean_object0: BooleanObject = BooleanObject::new(true);
        let boolean_object1: BooleanObject = BooleanObject::new(false);

        assert_eq!(format!("{}", boolean_object0), "true");
        assert_eq!(format!("{}", boolean_object1), "false");
    }
}
