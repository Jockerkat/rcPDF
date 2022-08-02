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

use std::fmt::{Debug, Display, Formatter};

/// This trait represents a generic, basic PDF object. All basic PDF objects
/// implement this trait.
///
/// # Traits
///
/// This trait inherits the `Display` trait, used to ensure that implementors
/// of the `Object` trait are correctly written to the output PDF file. Further,
/// the `Object` trait implements the `Debug` trait for `dyn Object`.
pub trait Object: Display {}

impl Debug for dyn Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
