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

#[derive(Debug)]
pub struct MM(f64);

impl From<i8> for MM {
    fn from(millimeter: i8) -> MM {
        MM(millimeter.into())
    }
}

impl From<i16> for MM {
    fn from(millimeter: i16) -> MM {
        MM(millimeter.into())
    }
}

impl From<u8> for MM {
    fn from(millimeter: u8) -> MM {
        MM(millimeter.into())
    }
}

impl From<u16> for MM {
    fn from(millimeter: u16) -> MM {
        MM(millimeter.into())
    }
}
