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

use std::borrow::Cow;

const CONVERSION_TABLE: [u32; 5] = [85_u32.pow(4), 85_u32.pow(3), 85_u32.pow(2), 85, 1];

/// This function encodes bytes of binary data into binary data in ASCII base-85,
/// as defined in ISO 32000-1:2008, 7 "Syntax", 7.4 "Filters",
/// 7.4.3 "ASCII85Decode Filter".
///
/// # Arguments
///
/// - `input` - The bytes of binary data to be encoded
///
/// # Returns
///
/// The encoded `input` if no error occurs, else an `Err`.
pub fn encode(input: &[u8]) -> Result<String, ()> {
    let mut encoded_input: String = String::new();
    let mut is_partial_group: bool = false;

    // Group input bytes into groups of 4 bytes. If there is a last, partial
    // group of 4, append 4-n bytes to make a complete group of 4 and write
    // the first n+1 characters of the resulting encoded group of 5 to the
    // returned String.
    for chunk in input.chunks(4) {
        let (group, size) = if chunk.len() == 4 {
            (Cow::from(chunk), 5)
        } else {
            is_partial_group = true;

            let mut new_group: Vec<u8> = Vec::new();
            new_group.resize_with(4, || 0);
            new_group[..chunk.len()].copy_from_slice(chunk);
            (Cow::from(new_group), 5 - (4 - chunk.len()))
        };

        // Result of (b_1 * 256^3) + (b_2 * 256^2) + (b_3 * 256^1) + b_4
        let base256_sum: u32 = u32::from_be_bytes(group.as_ref().try_into().map_err(|_| ())?);

        // Get solution for c_1, c_2, c_3, c_4 in the relation
        // (b_1 * 256^3) + (b_2 * 256^2) + (b_3 * 256^1) + b_4 =
        // (c_1 * 85^4) + (c_2 * 85^3) + (c_3 * 85^2) + (c_4 * 85^1) + c
        let mut base85_bytes: Vec<u8> = Vec::new();
        for item in CONVERSION_TABLE.iter().take(size) {
            let ascii85_encoded_byte: u8 = ((base256_sum / item) % 85) as u8;
            base85_bytes.push(ascii85_encoded_byte);
        }

        // Special case: an all-zero group
        let base85_sum: u32 = base85_bytes.iter().map(|&x| x as u32).sum();
        if !is_partial_group && base85_sum == 0 {
            encoded_input.push('z');
            continue;
        }

        for byte in base85_bytes {
            encoded_input.push((byte + 33) as char);
        }
    }

    // EOD marker
    encoded_input.push_str("~>");

    Ok(encoded_input)
}

#[cfg(test)]
mod tests {
    use crate::filters::ascii85decode;

    #[test]
    fn encoding_test() {
        let input0: &str = "Man is distinguished, not only by his reason, but by this singular passion from other \
                            animals, which is a lust of the mind, that by a perseverance of delight in the continued \
                            and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.";
        // ASCII characters for "hello NUL NUL NUL NUL NUL NUL NUL NUL world"
        let input1: [u8; 18] = [104, 101, 108, 108, 111, 0, 0, 0, 0, 0, 0, 0, 0, 119, 111, 114, 108, 100];

        assert_eq!(
            r#"9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,O<DJ+*.@<*K0@<6L(Df-\0Ec5e;DffZ(EZee.Bl.9pF"AGXBPCsi+DGm>@3BB/F*&OCAfu2/AKYi(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolDIal(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG%G>uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c~>"#,
            ascii85decode::encode(input0.as_bytes()).unwrap()
        );
        assert_eq!(r#"BOu!rDZBb;z!-`@%Ch*~>"#, ascii85decode::encode(&input1).unwrap());
    }
}
