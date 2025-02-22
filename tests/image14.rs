// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Image, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate adding images to worksheets.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_row_height(1, 4.5)?;
    worksheet.set_row_height(2, 35.25)?;
    worksheet.set_column_width(2, 3.29)?;
    worksheet.set_column_width(3, 3.29)?;
    worksheet.set_column_width(4, 3.29)?;
    worksheet.set_column_width(5, 10.71)?;

    let mut image = Image::new("tests/input/images/logo.png")?;
    image.set_alt_text("logo.png");

    worksheet.insert_image(1, 2, &image)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_image14() {
    let test_runner = common::TestRunner::new()
        .set_name("image14")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
