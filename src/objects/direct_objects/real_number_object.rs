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
use crate::objects::direct_objects::numeric_object::NumericObject;
use crate::objects::object::Object;
use std::fmt;

/// Structure for a real numeric object, as defined in ISO 32000-1:2008,
/// 7 "Syntax", 7.3 "Objects", 7.3.3 "Numeric Objects".
#[derive(Copy, Clone)]
pub struct RealObject(f32); // As defined in Annex C

impl Object for RealObject {}
impl DirectObject for RealObject {}
impl NumericObject for RealObject {}

impl fmt::Display for RealObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.5}", self.0) // As defined in Annex C
    }
}

impl RealObject {
    pub fn new(value: f32) -> RealObject {
        RealObject(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::real_number_object::RealObject;

    #[test]
    fn display_test() {
        let real_object0: RealObject = RealObject::new(5.0);
        let real_object1: RealObject = RealObject::new(-25.0);

        assert_eq!(format!("{}", real_object0), "5.00000");
        assert_eq!(format!("{}", real_object1), "-25.00000");
    }
}
