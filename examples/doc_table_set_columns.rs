// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! Example of creating a worksheet table.

use rust_xlsxwriter::{Table, TableColumn, TableFunction, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Some sample data for the table.
    let items = ["Apples", "Pears", "Bananas", "Oranges"];
    let data = [
        [10000, 5000, 8000, 6000],
        [2000, 3000, 4000, 5000],
        [6000, 6000, 6500, 6000],
        [500, 300, 200, 700],
    ];

    // Write the table data.
    worksheet.write_column(3, 1, items)?;
    worksheet.write_row_matrix(3, 2, data)?;

    // Set the columns widths for clarity.
    for col_num in 1..=6u16 {
        worksheet.set_column_width(col_num, 12)?;
    }

    // Create a new table and configure it.
    let mut table = Table::new();

    let columns = vec![
        TableColumn::new()
            .set_header("Product")
            .set_total_label("Totals"),
        TableColumn::new()
            .set_header("Quarter 1")
            .set_total_function(TableFunction::Sum),
        TableColumn::new()
            .set_header("Quarter 2")
            .set_total_function(TableFunction::Sum),
        TableColumn::new()
            .set_header("Quarter 3")
            .set_total_function(TableFunction::Sum),
        TableColumn::new()
            .set_header("Quarter 4")
            .set_total_function(TableFunction::Sum),
        TableColumn::new()
            .set_header("Year")
            .set_total_function(TableFunction::Sum)
            .set_formula("SUM(Table1[@[Quarter 1]:[Quarter 4]])"),
    ];

    table.set_columns(&columns);
    table.set_total_row(true);

    // Add the table to the worksheet.
    worksheet.add_table(2, 1, 7, 6, &table)?;

    // Save the file to disk.
    workbook.save("tables.xlsx")?;

    Ok(())
}
