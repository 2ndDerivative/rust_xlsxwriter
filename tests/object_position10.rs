// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Image, ObjectMovement, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate object positioning options.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let mut image = Image::new("tests/input/images/red.png")?;
    image.set_alt_text("red.png");
    image.set_object_movement(ObjectMovement::MoveAndSizeWithCellsAfter);

    worksheet.insert_image(8, 4, &image)?;

    worksheet.set_column_hidden(4)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_object_position10() {
    let test_runner = common::TestRunner::new()
        .set_name("object_position10")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
