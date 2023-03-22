// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Image, Workbook, XlsxError};

mod common;

// Test to demonstrate adding image scaling.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width_pixels(0, 192)?;
    worksheet.set_row_height_pixels(2, 64)?;
    worksheet.set_row_height_pixels(4, 64)?;

    let mut image = Image::new("tests/input/images/red.png")?;
    image.set_alt_text("red.png");

    worksheet.insert_image(0, 0, &image)?;

    image.set_scale_width(6.0).set_scale_height(2.0);
    worksheet.insert_image(2, 0, &image)?;

    image.set_scale_width(2.0).set_scale_height(2.0);
    worksheet.insert_image(4, 0, &image)?;

    workbook.save(filename)?;

    Ok(())
}

// Test to demonstrate adding image scaling.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width_pixels(0, 192)?;
    worksheet.set_row_height_pixels(2, 64)?;
    worksheet.set_row_height_pixels(4, 64)?;

    let mut image = Image::new("tests/input/images/red.png")?;
    image.set_alt_text("red.png");

    worksheet.insert_image(0, 0, &image)?;

    worksheet.insert_image_fit_to_cell(2, 0, &image, false)?;

    worksheet.insert_image_fit_to_cell(4, 0, &image, true)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_image_scale01_1() {
    let test_runner = common::TestRunner::new()
        .set_name("image_scale01")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_image_scale01_2() {
    let test_runner = common::TestRunner::new()
        .set_name("image_scale01")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
