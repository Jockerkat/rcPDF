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

/// This struct represents the basic `Boolean` object type. `Boolean` objects
/// represent the logical values `true` and `false` and appear in PDF files
/// using those keywords.
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
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `BooleanObject::new()` objects have the same generation number,
/// the copied/cloned objects' indirect references wouldn't be unique, violating
/// the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct BooleanObject {
    object_number: u32,
    generation_number: u32,
    value: bool,
}

impl BooleanObject {
    /// Returns a `BooleanObject` with the given Boolean value.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `value`: the Boolean value the `BooleanObject` should represent
    pub fn new(global_object_number_counter: &AtomicU32, value: bool) -> BooleanObject {
        BooleanObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            value,
        }
    }
}

impl Display for BooleanObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Object for BooleanObject {}

impl IndirectObject for BooleanObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

impl Hash for BooleanObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl PartialEq<Self> for BooleanObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BooleanObject {}

#[cfg(test)]
mod tests {
    use crate::objects::boolean_object::BooleanObject;
    use std::sync::atomic::AtomicU32;

    static GLOBAL_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    #[test]
    fn display_test() {
        let boolean_object_true = BooleanObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, true);
        let boolean_object_false = BooleanObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, false);

        assert_eq!(String::from("true"), boolean_object_true.to_string());
        assert_eq!(String::from("false"), boolean_object_false.to_string());
    }

    #[test]
    fn equality_test() {
        let boolean_object_true_0 = BooleanObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, true);
        let boolean_object_true_1 = BooleanObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, true);
        let boolean_object_false_0 = BooleanObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, false);
        let boolean_object_false_1 = BooleanObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, false);

        // equality
        assert_eq!(boolean_object_true_0, boolean_object_true_1);
        assert_eq!(boolean_object_false_0, boolean_object_false_1);

        // inequality
        assert_ne!(boolean_object_true_0, boolean_object_false_0);
        assert_ne!(boolean_object_true_1, boolean_object_false_1)
    }
}
