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
use std::fmt::Write;

/// This trait represents an indirect object. Any object in a PDF file may be
/// labelled as an indirect object. This gives the object a unique object
/// identifier by which other objects can refer to it. All basic PDF objects
/// implement this trait.
///
/// # Traits
///
/// This trait inherits the [Object](crate::objects::object::Object) trait.
pub trait IndirectObject: Object {
    /// Gets the positive integer object number of an indirect object. Indirect
    /// objects may be numbered sequentially or arbitrarily within a PDF file.
    fn object_number(&self) -> u32;
    /// Gets the non-negative integer generation number of an indirect object.
    /// In a newly PDF created file, all indirect objects have a generation number
    /// of 0.
    fn generation_number(&self) -> u32;
    /// Get the (unique) indirect reference to an indirect object. This method
    /// provides a default implementation as defined in *ISO 32000-1:2008*,
    /// 7.3.10 "Indirect Objects".
    fn indirect_reference(&self) -> String {
        format!("{} {} R", self.object_number(), self.generation_number())
    }
    /// Get the (unique) indirect object definition of an indirect object. This
    /// method provides a default implementation as defined in *ISO 32000-1:2008*,
    /// 7.3.10 "Indirect Objects". This method shall be used when writing an
    /// object to a file.
    fn indirect_definition(&self) -> String {
        let mut output = String::new();

        output.push_str(&*format!("{} {} obj\n", self.object_number(), self.generation_number()));
        output.push_str(&*format!("{}\n", self));
        output.push_str("endobj");

        output
    }
}
