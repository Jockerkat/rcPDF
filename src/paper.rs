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

use crate::util::mm;
use crate::util::size;
use crate::util::margins;
use crate::util::rotation;

/// The paper size of a page in the PDF document.
#[derive(Debug)]
pub enum PaperSize {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    Executive1,
    Executive2,
    Executive3,
    Legal,
    Letter,
}

/// The paper orientation of a page in the PDF document.
#[derive(Debug)]
pub enum PaperOrientation {
    Portrait,
    Landscape,
}

impl From<PaperSize> for size::Size {
    fn from(paper_size: PaperSize) -> size::Size {
        match paper_size {
            PaperSize::A0 => size::Size::new(841, 1189),
            PaperSize::A1 => size::Size::new(594, 841),
            PaperSize::A2 => size::Size::new(420, 594),
            PaperSize::A3 => size::Size::new(297, 420),
            PaperSize::A4 => size::Size::new(210, 297),
            PaperSize::A5 => size::Size::new(148, 210),
            PaperSize::A6 => size::Size::new(105, 148),
            PaperSize::Executive1 => size::Size::new(177.8, 266.7),  // 7 x 10.5 inches
            PaperSize::Executive2 => size::Size::new(184.15, 266.7), // 7.25 x 10.5 inches
            PaperSize::Executive3 => size::Size::new(190.5, 266.7),  // 7.5 x 10.5 inches
            PaperSize::Legal => size::Size::new(215.9, 355.6),       // 8.5 x 14 inches
            PaperSize::Letter => size::Size::new(215.9, 279.4),      // 8.5 x 11 inches
        }
    }
}

impl From<PaperOrientation> for rotation::Rotation {
    fn from(paper_orientation: PaperOrientation) -> rotation::Rotation {
        match paper_orientation {
            PaperOrientation::Portrait => rotation::Rotation::new(0.0),
            PaperOrientation::Landscape => rotation::Rotation::new(90.0),
        }
    }
}
