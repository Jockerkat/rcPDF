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
use crate::objects::direct_objects::name_object::NameObject;
use crate::objects::object::Object;
use std::collections::HashMap;
use std::fmt;

/// Structure for a dictionary object, as defined in ISO 32000-1:2008, 7 "Syntax",
/// 7.3 "Objects", 7.3.7 "Dictionary Objects".
pub struct DictionaryObject {
    dictionary: HashMap<NameObject, Box<dyn Object>>,
}

impl Object for DictionaryObject {}
impl DirectObject for DictionaryObject {}

impl fmt::Display for DictionaryObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<<\n")?;

        // Order is arbitrary!
        for (key, value) in self.dictionary.iter() {
            f.write_str(&key.to_string())?;
            f.write_str(" ")?;
            f.write_str(&value.to_string())?;
            f.write_str("\n")?;
        }

        f.write_str(">>")?;

        Ok(())
    }
}

impl DictionaryObject {
    pub fn new() -> DictionaryObject {
        DictionaryObject {
            dictionary: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: NameObject, value: impl Object + 'static) -> Option<Box<dyn Object>> {
        self.dictionary.insert(key, Box::new(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::direct_objects::boolean_object::BooleanObject;
    use crate::objects::direct_objects::dictionary_object::DictionaryObject;
    use crate::objects::direct_objects::integer_number_object::IntegerObject;
    use crate::objects::direct_objects::literal_string_object::LiteralStringObject;
    use crate::objects::direct_objects::name_object::NameObject;

    #[test]
    fn display_test() {
        let mut dictionary_object0: DictionaryObject = DictionaryObject::new();
        let mut dictionary_object1: DictionaryObject = DictionaryObject::new();

        let name_object0: NameObject = NameObject::new("Type".to_string());
        let name_object1: NameObject = NameObject::new("Example".to_string());
        let name_object2: NameObject = NameObject::new("IntegerItem".to_string());
        let name_object3: NameObject = NameObject::new("StringItem".to_string());
        let name_object4: NameObject = NameObject::new("Subdictionary".to_string());
        let name_object5: NameObject = NameObject::new("Item1".to_string());

        let integer_number_object: IntegerObject = IntegerObject::new(5);
        let literal_string_object: LiteralStringObject = LiteralStringObject::new("a string".to_string());
        let boolean_object: BooleanObject = BooleanObject::new(true);

        assert_eq!(format!("{}", dictionary_object0), "<<\n>>");

        dictionary_object0.add(name_object0, name_object1);
        dictionary_object0.add(name_object2, integer_number_object);
        dictionary_object0.add(name_object3, literal_string_object);
        dictionary_object1.add(name_object5, boolean_object);
        dictionary_object0.add(name_object4, dictionary_object1);

        // println!("{}", dictionary_object0);
    }
}
