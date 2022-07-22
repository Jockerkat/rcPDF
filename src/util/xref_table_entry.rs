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

/// Struct that holds a cross-reference table entry of an object. The correct
/// formatting of the entry is left up to the struct's user.
pub struct XRefTableEntry {
    byte_offset: u32,
    generation_number: u32,
}

impl XRefTableEntry {
    pub fn new(byte_offset: u32, generation_number: u32) -> XRefTableEntry {
        XRefTableEntry {
            byte_offset,
            generation_number,
        }
    }

    /// Get the byte offset in the decoded stream of the object.
    pub fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    /// Get the generation number of the object.
    pub fn generation_number(&self) -> u32 {
        self.generation_number
    }
}
