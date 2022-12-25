// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test to demonstrate autofilters.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add the data used in the autofilter tests.
    common::populate_autofilter_data(worksheet);

    worksheet.set_autofilter(0, 0, 50, 3)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofilter01() {
    let test_runner = common::TestRunner::new()
        .set_name("autofilter01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
