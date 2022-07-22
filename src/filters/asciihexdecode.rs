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

use std::fmt::Write;

/// This function encodes bytes of binary data into binary data in Hexadecimal,
/// as defined in ISO 32000-1:2008, 7 "Syntax", 7.4 "Filters",
/// 7.4.2 "ASCIIHexDecode Filter".
///
/// # Arguments
///
/// * `input` - The bytes of binary data to be encoded
///
/// # Returns
///
/// The encoded `input` if no error occurs, else an `Err`.
pub fn encode(input: &[u8]) -> Result<String, ()> {
    let mut output: String = String::with_capacity(input.len() * 2);

    for &byte in input {
        write!(&mut output, "{:02X}", byte).map_err(|_| ())?;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::filters::asciihexdecode;

    #[test]
    fn encoding_test() {
        assert_eq!(asciihexdecode::encode("foobar".as_bytes()).unwrap(), "666F6F626172");
        assert_eq!(
            asciihexdecode::encode("Umlauts: öäü".as_bytes()).unwrap(),
            "556D6C617574733A20C3B6C3A4C3BC"
        );
    }
}
