// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test case to demonstrate creating a basic file with boolean types.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let format1 = Format::new().set_bold();

    worksheet.write_boolean(0, 0, true)?;
    worksheet.write_boolean(1, 0, false)?;
    worksheet.write_boolean_with_format(2, 0, true, &format1)?;
    worksheet.write_boolean_with_format(3, 0, false, &format1)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case with boolean types and generics.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let format1 = Format::new().set_bold();

    worksheet.write(0, 0, true)?;
    worksheet.write(1, 0, false)?;
    worksheet.write_with_format(2, 0, true, &format1)?;
    worksheet.write_with_format(3, 0, false, &format1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap37_boolean_types_1() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap37")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap37_boolean_types_2() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap37")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
