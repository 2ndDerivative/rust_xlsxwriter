// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates setting the quote prefix property for a
//! format.

use rust_xlsxwriter::{Format, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let format = Format::new().set_quote_prefix();

    // If the "=Hello" string was edited in Excel it would change into an
    // invalid formula and raise an error. The quote prefix adds a virtual quote
    // to the start of the string and prevents this from happening.
    worksheet.write_string_with_format(0, 0, "=Hello", &format)?;

    workbook.save("formats.xlsx")?;

    Ok(())
}
