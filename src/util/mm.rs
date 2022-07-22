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

use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MM(f64);

impl From<i8> for MM {
    fn from(millimeters: i8) -> MM {
        MM(millimeters.into())
    }
}

impl From<i16> for MM {
    fn from(millimeters: i16) -> MM {
        MM(millimeters.into())
    }
}

impl From<i32> for MM {
    fn from(millimeters: i32) -> MM {
        MM(millimeters.into())
    }
}

impl From<u8> for MM {
    fn from(millimeters: u8) -> MM {
        MM(millimeters.into())
    }
}

impl From<u16> for MM {
    fn from(millimeters: u16) -> MM {
        MM(millimeters.into())
    }
}

impl From<u32> for MM {
    fn from(millimeters: u32) -> MM {
        MM(millimeters.into())
    }
}

impl From<f32> for MM {
    fn from(millimeters: f32) -> MM {
        MM(millimeters.into())
    }
}

impl From<f64> for MM {
    fn from(millimeters: f64) -> MM {
        MM(millimeters)
    }
}

impl Mul<f64> for MM {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let result = self.0 * rhs;
        MM(result)
    }
}
