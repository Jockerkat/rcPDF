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
use crate::objects::name_object::NameObject;
use crate::objects::object::Object;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU32, Ordering};

/// This struct represents the basic `Dictionary` object type. A `Dictionary`
/// object is an associative table containing pairs of objects, known as the
/// dictionary's entries.
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
/// number; as all `DictionaryObject::new()`/`DictionaryObject::new_from_hashmap()`
/// objects have the same generation number, the copied/cloned objects' indirect
/// references wouldn't be unique, violating the *ISO 32000-1:2008*,
/// 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct DictionaryObject {
    object_number: u32,
    generation_number: u32,
    dictionary: HashMap<NameObject, Box<dyn Object>>,
}

impl DictionaryObject {
    /// Returns an empty `DictionaryObject` (i.e., the dictionary has no entries).
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    pub fn new(global_object_number_counter: &AtomicU32) -> DictionaryObject {
        DictionaryObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary: Default::default(),
        }
    }

    /// Returns a `DictionaryObject` from the given hashmap.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `hashmap`: the hashmap to be converted to a `DictionaryObject`
    pub fn new_from_hashmap(
        global_object_number_counter: &AtomicU32,
        hashmap: HashMap<NameObject, Box<dyn Object>>,
    ) -> DictionaryObject {
        DictionaryObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            dictionary: hashmap,
        }
    }

    /// Inserts a key-value pair into the dictionary. If the dictionary did not
    /// have this key present, `None` is returned. If the dictionary did have
    /// this key present, the value is updated, and the old value is returned.
    ///
    /// # Arguments
    ///
    /// - `key`:   the key to insert or update
    /// - `value`: the new value of the key
    pub fn insert(&mut self, key: NameObject, value: impl Object + 'static) -> Option<Box<dyn Object>> {
        self.dictionary.insert(key, Box::new(value))
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Arguments
    ///
    /// - `key`: the key's value to get
    pub fn get(&self, key: &NameObject) -> Option<&Box<dyn Object>> {
        self.dictionary.get(key)
    }
}

impl Display for DictionaryObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<<\n")?;

        // order is arbitrary
        for (key, value) in &self.dictionary {
            f.write_str(&*format!("{} {}\n", key, value))?;
        }

        f.write_str(">>")
    }
}

impl Object for DictionaryObject {}

impl IndirectObject for DictionaryObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}
