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

/// This trait represents a generic numeric object, i.e., integer and real
/// objects. Throughout the *ISO 32000-1:2008* standard, a number may be either
/// of the type integer or real. Wherever a real number is expected, an integer
/// may be used instead.
///
/// # Traits
///
/// This trait inherits the [Object](crate::objects::object::Object) trait.
pub trait NumericObject: Object {}

/// This struct represents the basic `Integer` object type. `Integer` objects
/// represent signed mathematical integers.
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
/// - [NumericObject](crate::objects::numeric_object::NumericObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
/// - `Hash`
/// - `Eq`
/// - `PartialEq`
/// - `Ord`
/// - `PartialOrd`
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `IntegerObject::new()` objects have the same generation number,
/// the copied/cloned objects' indirect references wouldn't be unique, violating
/// the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct IntegerObject {
    object_number: u32,
    generation_number: u32,
    value: i32, // as defined in Annex C
}

/// This struct represents the basic `Real` object type. `Real` objects represent
/// signed mathematical real numbers.
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
/// - [NumericObject](crate::objects::numeric_object::NumericObject)
/// - `Display` (as required by the [Object](crate::objects::object::Object) trait)
/// - `PartialEq`
/// - `PartialOrd`
///
/// This struct does not implement nor derive the `Copy` and `Clone` traits, as
/// copying/cloning an object would result in two objects having the same object
/// number; as all `RealObject::new()` objects have the same generation number,
/// the copied/cloned objects' indirect references wouldn't be unique, violating
/// the *ISO 32000-1:2008*, 7.3.10 "Indirect Objects" specification.
#[derive(Debug)]
pub struct RealObject {
    object_number: u32,
    generation_number: u32,
    value: f32, // as defined in Annex C
}

impl IntegerObject {
    /// Returns an `IntegerObject` with the given signed integer.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `value`: the signed integer the `IntegerObject` should represent
    pub fn new(global_object_number_counter: &AtomicU32, value: i32) -> IntegerObject {
        IntegerObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            value,
        }
    }
}

impl NumericObject for IntegerObject {}

impl Display for IntegerObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Object for IntegerObject {}

impl IndirectObject for IntegerObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

impl Hash for IntegerObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl PartialEq<Self> for IntegerObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for IntegerObject {}

impl PartialOrd<Self> for IntegerObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for IntegerObject {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl RealObject {
    /// Returns a `RealObject` with the given float (real) number.
    ///
    /// # Arguments
    ///
    /// - `global_object_number_counter`: a reference to a global object number
    ///                                   counter which is used for the object's
    ///                                   indirect reference
    /// - `value`: the float (real) number the `RealObject` should represent. If
    ///            the value lies between 0 and +/- 1.175 * 10<sup>-38</sup>, it
    ///            will be converted to 0
    pub fn new(global_object_number_counter: &AtomicU32, value: f32) -> RealObject {
        RealObject {
            object_number: global_object_number_counter.fetch_add(1, Ordering::Relaxed),
            generation_number: 0,
            value: RealObject::sanitise_input(value),
        }
    }

    /// Checks whether the passed in value lies between 0 and +/- 1.175 * 10<sup>-38</sup>.
    /// If it does, the returned value is 0, else the passed in value is returned.
    ///
    /// # Arguments
    ///
    /// - `value`: the value to check
    fn sanitise_input(value: f32) -> f32 {
        if value < 1.175 * 10_f32.powi(-38) && value > -1.175 * 10_f32.powi(-38) {
            return 0.0;
        }

        value
    }
}

impl NumericObject for RealObject {}

impl Display for RealObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5}", self.value) // as defined in Annex C
    }
}

impl Object for RealObject {}

impl IndirectObject for RealObject {
    fn object_number(&self) -> u32 {
        self.object_number
    }

    fn generation_number(&self) -> u32 {
        self.generation_number
    }
}

impl PartialEq for RealObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for RealObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::numeric_object::{IntegerObject, RealObject};
    use std::sync::atomic::AtomicU32;

    static GLOBAL_OBJECT_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    #[test]
    fn display_test() {
        let integer_object_0 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 5);
        let integer_object_1 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, -25);
        let real_object_0 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 5.2);
        let real_object_1 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, -25.2);

        assert_eq!(String::from("5"), integer_object_0.to_string());
        assert_eq!(String::from("-25"), integer_object_1.to_string());
        assert_eq!(String::from("5.20000"), real_object_0.to_string());
        assert_eq!(String::from("-25.20000"), real_object_1.to_string());
    }

    #[test]
    fn integer_object_equality_test() {
        let integer_object_0 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 5);
        let integer_object_1 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 5);
        let integer_object_2 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 7);

        // equality
        assert_eq!(integer_object_0, integer_object_1);

        // inequality
        assert_ne!(integer_object_0, integer_object_2);
        assert_ne!(integer_object_1, integer_object_2);
    }

    #[test]
    fn real_object_equality_test() {
        let real_object_0 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 0.2);
        let real_object_1 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 0.2);
        let real_object_2 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 0.4);

        // equality
        assert_eq!(real_object_0, real_object_1);

        // inequality
        assert_ne!(real_object_0, real_object_2);
        assert_ne!(real_object_1, real_object_2);
    }

    #[test]
    fn integer_object_ordering_test() {
        let integer_object_0 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 5);
        let integer_object_1 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 6);
        let integer_object_2 = IntegerObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 7);

        // less than
        assert!(integer_object_0.lt(&integer_object_1));
        assert!(integer_object_0.lt(&integer_object_2));
        assert!(integer_object_1.lt(&integer_object_2));

        // less or equal
        assert!(integer_object_0.le(&integer_object_0));
        assert!(integer_object_1.le(&integer_object_1));
        assert!(integer_object_2.le(&integer_object_2));

        // greater than
        assert!(integer_object_2.gt(&integer_object_1));
        assert!(integer_object_2.gt(&integer_object_0));
        assert!(integer_object_1.gt(&integer_object_0));

        // greater than or equal
        assert!(integer_object_0.ge(&integer_object_0));
        assert!(integer_object_1.ge(&integer_object_1));
        assert!(integer_object_2.ge(&integer_object_2));
    }

    #[test]
    fn real_object_ordering_test() {
        let real_object_0 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 0.2);
        let real_object_1 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 0.3);
        let real_object_2 = RealObject::new(&GLOBAL_OBJECT_NUMBER_COUNTER, 0.4);

        // less than
        assert!(real_object_0.lt(&real_object_1));
        assert!(real_object_0.lt(&real_object_2));
        assert!(real_object_1.lt(&real_object_2));

        // less or equal
        assert!(real_object_0.le(&real_object_0));
        assert!(real_object_1.le(&real_object_1));
        assert!(real_object_2.le(&real_object_2));

        // greater than
        assert!(real_object_2.gt(&real_object_1));
        assert!(real_object_2.gt(&real_object_0));
        assert!(real_object_1.gt(&real_object_0));

        // greater than or equal
        assert!(real_object_0.ge(&real_object_0));
        assert!(real_object_1.ge(&real_object_1));
        assert!(real_object_2.ge(&real_object_2));
    }

    #[test]
    fn real_object_sanitise_input_test() {
        // values large enough
        assert_eq!(
            1.176 * 10_f32.powi(-38),
            RealObject::sanitise_input(1.176 * 10_f32.powi(-38))
        );
        assert_eq!(
            -1.176 * 10_f32.powi(-38),
            RealObject::sanitise_input(-1.176 * 10_f32.powi(-38))
        );

        // values too small
        assert_eq!(0.0, RealObject::sanitise_input(1.174 * 10_f32.powi(-38)));
        assert_eq!(0.0, RealObject::sanitise_input(-1.174 * 10_f32.powi(-38)));
    }
}
