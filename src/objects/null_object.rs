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
use std::sync::atomic::{AtomicU32, Ordering};

/// This struct represents the basic `Null` object type. The `Null` object has
/// a type and value that are unequal to those of any other object.
///
/// # Traits
///
/// This struct derives the following traits:
///
/// - `Debug`
/// - `Hash`
///
/// Additionally, it implements the following traits:
///
/// - [Object](crate::objects::object::Object)
/// - [IndirectObject](crate::objects::indirect_object::IndirectObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `NullObject::new()` objects have the same generation number,
/// the copied/cloned objects' indirect references wouldn't be unique, violating
/// the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug, Hash)]
pub struct NullObject {
    object_number: u32,
    generation_number: u32,
}

impl NullObject {
    /// Returns a `NullObject`.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    pub fn new(global_object_number_counter: &AtomicU32) -> NullObject {
        NullObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
        }
    }
}

impl Display for NullObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("null")
    }
}

impl Object for NullObject {}

impl IndirectObject for NullObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::null_object::NullObject;
    use std::sync::atomic::AtomicU32;

    static GLOBAL_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    #[test]
    fn display_test() {
        let null_object = NullObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER);

        assert_eq!(String::from("null"), null_object.to_string());
    }
}
