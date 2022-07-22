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

use crate::util::xref_table_entry::XRefTableEntry;
use crate::PDFDocument;
use log::{info, warn};
use std::fs::File;
use std::io::Write;

/// To keep I/O usage low, the output file is written to four times:
/// 1. After generating the file header.
/// 2. After generating the file body.
/// 3. After generating the cross-reference table.
/// 4. After generating the file trailer.
pub fn render(pdf_document: &PDFDocument) {
    // TODO: Instead of unwrap, propagate errors to caller
    // TODO: Add logging

    let mut file = File::create(&pdf_document.file_name).unwrap();

    write!(&mut file, "{}", generate_header(1.7)).unwrap();

    todo!()
}

/// Generates the file header of the PDF file.
/// It is assumed that the PDF file will contain binary data, thus the header
/// line is followed by a comment line containing the following four binary
/// characters: `££££` (`£`'s decimal code is 163, well above the minimum
/// requirement of 128).
fn generate_header(pdf_version: f64) -> String {
    let mut header = String::with_capacity(19);

    header.push_str(&*format!("%PDF-{}\n", pdf_version));
    header.push_str("%££££\n");

    header
}

fn generate_body() {
    todo!()
}

/// Generates the cross reference table of the PDF file.
/// The first entry of the table is `0000000000 65535 f \n`, representing the
/// start of the free entries list.
fn generate_cross_reference_table(table_entries: Vec<XRefTableEntry>) -> String {
    // each entry (line) is 20 bytes + xref + contiguous range of object numbers
    let mut xref_table = String::with_capacity((table_entries.len() * 20) + 5 + 4);

    xref_table.push_str("xref\n");
    xref_table.push_str(&*format!("0 {}\n", table_entries.len() + 1));

    // start of free entries list
    xref_table.push_str("0000000000 65535 f \n");

    for table_entry in table_entries {
        xref_table.push_str(&*format!(
            "{:010} {:05} n \n",
            table_entry.byte_offset(),
            table_entry.generation_number()
        ));
    }

    xref_table
}

fn generate_trailer() {
    todo!()
}
