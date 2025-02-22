// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write(1, 7, "Donut")?;
    worksheet.write(2, 7, 25)?;
    worksheet.write(3, 7, 50)?;
    worksheet.write(4, 7, 25)?;
    worksheet.write(5, 7, 100)?;

    worksheet.write(1, 8, "Pie")?;
    worksheet.write(2, 8, 75)?;
    worksheet.write(3, 8, 1)?;
    worksheet.write(4, 8, 124)?;

    let mut chart = Chart::new(ChartType::Doughnut);
    chart
        .add_series()
        .set_values("=Sheet1!$H$3:$H$6")
        .set_name("=Sheet1!$H$2");
    chart
        .add_series()
        .set_values("=Sheet1!$I$3:$I$6")
        .set_name("=Sheet1!$I$2");

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_doughnut07() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_doughnut07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
