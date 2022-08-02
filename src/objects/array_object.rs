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
use std::fmt::{Display, Formatter, Write};
use std::sync::atomic::{AtomicU32, Ordering};

/// This struct represents the basic `Array` object type. An `Array` object is
/// a one-dimensional collection of objects arranged sequentially; it may be
/// heterogeneous and may have zero elements.
///
/// # Traits
///
/// This struct derives the following traits:
///
/// - `Debug`
///
/// Additionally, it implements the following traits:
///
/// - [Object](crate::objects::object::Object)
/// - [IndirectObject](crate::objects::indirect_object::IndirectObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `ArrayObject::new()`/`ArrayObject::new_from_into_iterator()`
/// objects have the same generation number, the copied/cloned objects' indirect
/// references wouldn't be unique, violating the *ISO 32000-1:2008*,
/// 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct ArrayObject {
    object_number: u32,
    generation_number: u32,
    objects: Vec<Box<dyn Object>>,
}

impl ArrayObject {
    /// Returns an empty `ArrayObject` (i.e., the array contains no elements).
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    pub fn new(global_object_number_counter: &AtomicU32) -> ArrayObject {
        ArrayObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            objects: vec![],
        }
    }

    /// Returns an `ArrayObjects` containing the given objects.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `objects`: anything that can be converted into an iterator and which
    ///              contains the objects to be added to the `ArrayObject`
    pub fn new_from_into_iterator(
        global_object_number_counter: &AtomicU32,
        objects: impl IntoIterator<Item = impl Object + 'static>,
    ) -> ArrayObject {
        let mut objects_boxed: Vec<Box<dyn Object>> = vec![];

        for object in objects.into_iter() {
            objects_boxed.push(Box::new(object));
        }

        ArrayObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            objects: objects_boxed,
        }
    }

    /// Appends an object to the `ArrayObject`.
    ///
    /// # Arguments
    ///
    /// - `object`: the object to be appended to the `ArrayObject`
    pub fn push(&mut self, object: impl Object + 'static) {
        self.objects.push(Box::new(object))
    }
}

impl Display for ArrayObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut spacing = "";

        f.write_char('[')?;

        for object in self.objects.iter() {
            f.write_str(spacing)?;
            f.write_str(&*object.to_string())?;
            spacing = " ";
        }

        f.write_char(']')
    }
}

impl Object for ArrayObject {}

impl IndirectObject for ArrayObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::array_object::ArrayObject;
    use crate::objects::boolean_object::BooleanObject;
    use crate::objects::name_object::NameObject;
    use crate::objects::null_object::NullObject;
    use crate::objects::numeric_object::{IntegerObject, RealObject};
    use std::sync::atomic::AtomicU32;

    static GLOBAL_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    #[test]
    fn display_test() {
        let mut array_object = ArrayObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER);

        array_object.push(ArrayObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER));
        array_object.push(BooleanObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, true));
        array_object.push(NameObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, "A_Name"));
        array_object.push(NullObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER));
        array_object.push(IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 25));
        array_object.push(RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, -12.3456));
        // TODO: add more objects

        assert_eq!(
            String::from("[[] true /A_Name null 25 -12.34560]"),
            array_object.to_string()
        );
    }
}
