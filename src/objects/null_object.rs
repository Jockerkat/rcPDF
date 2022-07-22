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

use crate::objects::object::Object;
use std::fmt;

/// Structure for a null object, as defined in ISO 32000-1:2008, 7 "Syntax",
/// 7.3 "Objects", 7.3.9 "Null Object".
#[derive(Copy, Clone)]
pub struct NullObject();

impl Object for NullObject {}

impl fmt::Display for NullObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("null")
    }
}

impl NullObject {
    pub fn new() -> NullObject {
        NullObject()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::null_object::NullObject;

    #[test]
    fn display_test() {
        let null_object: NullObject = NullObject::new();

        assert_eq!(format!("{}", null_object), "null");
    }
}
