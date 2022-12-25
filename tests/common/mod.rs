// Test helper functions for integration tests. These functions convert Excel
// xml files into vectors of xml elements to make comparison testing easier.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

#[macro_export]
macro_rules! assert_result {
    ( $x:expr ) => {
        match $x {
            Ok(result) => result,
            Err(e) => panic!("\n!\n! XlsxError:\n! {:?}\n!\n", e),
        }
    };
}

#[cfg(test)]
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;

use pretty_assertions::assert_eq;
use regex::Regex;
use rust_xlsxwriter::Worksheet;
use rust_xlsxwriter::XlsxError;

// Simple test runner struct and methods to create a new xlsx output file and
// compare it with an input xlsx file created by Excel.
#[allow(dead_code)]
pub struct TestRunner<'a, F>
where
    F: FnOnce(&str) -> Result<(), XlsxError> + Copy,
{
    test_name: &'a str,
    test_function: Option<F>,
    unique: &'a str,
    input_filename: String,
    output_filename: String,
    ignore_files: HashSet<&'a str>,
    ignore_elements: HashMap<&'a str, &'a str>,
}

impl<'a, F> TestRunner<'a, F>
where
    F: FnOnce(&str) -> Result<(), XlsxError> + Copy,
{
    pub fn new() -> TestRunner<'a, F> {
        TestRunner {
            test_name: "",
            test_function: None,
            unique: "",
            input_filename: "".to_string(),
            output_filename: "".to_string(),
            ignore_files: HashSet::new(),
            ignore_elements: HashMap::new(),
        }
    }

    // Set the testcase name.
    pub fn set_name(mut self, testcase: &'a str) -> TestRunner<F> {
        self.test_name = testcase;
        self
    }

    // Set the test function pointer.
    pub fn set_function(mut self, test_function: F) -> TestRunner<'a, F> {
        self.test_function = Some(test_function);
        self
    }

    // Set string to add to the default output filename to make it unique so
    // that the multiple tests can be run in parallel.
    #[allow(dead_code)]
    pub fn unique(mut self, unique_string: &'a str) -> TestRunner<F> {
        self.unique = unique_string;
        self
    }

    // Ignore certain xml files within the test xlsx files.
    #[allow(dead_code)]
    pub fn ignore_file(mut self, filename: &'a str) -> TestRunner<'a, F> {
        self.ignore_files.insert(filename);
        self
    }

    // Ignore the files associated with the formula xl/calcChain.xml.
    #[allow(dead_code)]
    pub fn ignore_calc_chain(mut self) -> TestRunner<'a, F> {
        self.ignore_files.insert("xl/calcChain.xml");
        self.ignore_files.insert("[Content_Types].xml");
        self.ignore_files.insert("xl/_rels/workbook.xml.rels");
        self
    }

    // Ignore certain elements with xml files.
    #[allow(dead_code)]
    pub fn ignore_elements(mut self, filename: &'a str, pattern: &'a str) -> TestRunner<'a, F> {
        self.ignore_elements.insert(filename, pattern);
        self
    }

    // Initialize the in/out filenames once other properties have been set.
    pub fn initialize(mut self) -> TestRunner<'a, F> {
        self.input_filename = format!("tests/input/{}.xlsx", self.test_name);

        if self.unique.is_empty() {
            self.output_filename = format!("tests/output/rs_{}.xlsx", self.test_name);
        } else {
            self.output_filename =
                format!("tests/output/rs_{}_{}.xlsx", self.test_name, self.unique);
        }

        self
    }

    // Run the test function, check its result, and then test if the input and
    // generated output file are equal.
    pub fn assert_eq(&self) {
        // Get the test function and run it to generate the output file.
        let testcode = (self.test_function).unwrap();
        let result = (testcode)(&self.output_filename);

        // Check for any XlsxError errors from the test code.
        assert_result!(result);

        // If the function ran correctly then compare the input/reference file
        // with the output/generated file.
        let (exp, got) = compare_xlsx_files(
            &self.input_filename,
            &self.output_filename,
            &self.ignore_files,
            &self.ignore_elements,
        );

        assert_eq!(exp, got);
    }

    // Clean up any the temp output file.
    pub fn cleanup(&self) {
        fs::remove_file(&self.output_filename).unwrap();
    }
}

// Unzip 2 xlsx files and compare whether they have the same filenames and
// structure. If they are the same then we compare each xml file to ensure that
// files created by rust_xlsxwriter are the same as test files created in Excel.
// Returns two String vectors for comparison testing.
fn compare_xlsx_files(
    exp_file: &str,
    got_file: &str,
    ignore_files: &HashSet<&str>,
    ignore_elements: &HashMap<&str, &str>,
) -> (Vec<String>, Vec<String>) {
    // Open the xlsx files.
    let exp_fh = match File::open(exp_file) {
        Ok(fh) => fh,
        Err(err) => {
            return (
                vec![exp_file.to_string(), err.to_string()],
                vec![got_file.to_string()],
            )
        }
    };
    let got_fh = match File::open(got_file) {
        Ok(fh) => fh,
        Err(err) => {
            return (
                vec![exp_file.to_string()],
                vec![got_file.to_string(), err.to_string()],
            )
        }
    };

    // Open the zip structure that comprises an xlsx file.
    let mut exp_zip = match zip::ZipArchive::new(exp_fh) {
        Ok(fh) => fh,
        Err(err) => {
            return (
                vec![exp_file.to_string(), err.to_string()],
                vec![got_file.to_string()],
            )
        }
    };
    let mut got_zip = match zip::ZipArchive::new(got_fh) {
        Ok(fh) => fh,
        Err(err) => {
            return (
                vec![exp_file.to_string()],
                vec![got_file.to_string(), err.to_string()],
            )
        }
    };

    // Iterate through each xml file in the xlsx/zip container and read the
    // xml data as a string.
    let mut exp_filenames = vec![];
    let mut got_filenames = vec![];
    let mut exp_xml: HashMap<String, String> = HashMap::new();
    let mut got_xml: HashMap<String, String> = HashMap::new();

    for i in 0..exp_zip.len() {
        let mut file = match exp_zip.by_index(i) {
            Ok(file) => file,
            Err(err) => {
                return (
                    vec![exp_file.to_string(), err.to_string()],
                    vec![got_file.to_string()],
                )
            }
        };

        // Ignore any test specific files like "xl/calcChain.xml".
        if ignore_files.contains(file.name()) {
            continue;
        }

        // Store the filenames for comparison of the file structure.
        exp_filenames.push(file.name().to_string());

        if is_binary_file(file.name()) {
            // Get a checksum for binary files.
            let mut bin_data: Vec<u8> = vec![];
            file.read_to_end(&mut bin_data).unwrap();
            let mut hasher = DefaultHasher::new();
            bin_data.hash(&mut hasher);
            let xml_data = format!("checksum = {}", hasher.finish());
            exp_xml.insert(file.name().to_string(), xml_data);
        } else {
            // Read XML data from non-binary files.
            let mut xml_data = String::new();
            file.read_to_string(&mut xml_data).unwrap();
            exp_xml.insert(file.name().to_string(), xml_data);
        }
    }

    for i in 0..got_zip.len() {
        let mut file = match got_zip.by_index(i) {
            Ok(file) => file,
            Err(err) => {
                return (
                    vec![exp_file.to_string()],
                    vec![got_file.to_string(), err.to_string()],
                )
            }
        };

        // Ignore any test specific files like "xl/calcChain.xml".
        if ignore_files.contains(file.name()) {
            continue;
        }

        // Store the filenames for comparison of the file structure.
        got_filenames.push(file.name().to_string());

        if is_binary_file(file.name()) {
            // Get a checksum for binary files.
            let mut bin_data: Vec<u8> = vec![];
            file.read_to_end(&mut bin_data).unwrap();
            let mut hasher = DefaultHasher::new();
            bin_data.hash(&mut hasher);
            let xml_data = format!("checksum = {}", hasher.finish());
            got_xml.insert(file.name().to_string(), xml_data);
        } else {
            // Read XML data from non-binary files.
            let mut xml_data = String::new();
            file.read_to_string(&mut xml_data).unwrap();
            got_xml.insert(file.name().to_string(), xml_data);
        }
    }

    // Sort the xlsx filenames/structure
    exp_filenames.sort();
    got_filenames.sort();

    if exp_filenames != got_filenames {
        return (exp_filenames, got_filenames);
    }

    for filename in exp_filenames {
        let mut exp_xml_string = exp_xml.get(&filename).unwrap().to_string();
        let mut got_xml_string = got_xml.get(&filename).unwrap().to_string();

        // Remove author name and creation date metadata from core.x¦ml file.
        if filename == "docProps/core.xml" {
            // Removed author name from test input files created in Excel.
            exp_xml_string = exp_xml_string.replace("John", "");

            // Remove creation date from core.xml file.
            let re = Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z").unwrap();
            exp_xml_string = re.replace_all(&exp_xml_string, "").to_string();
            got_xml_string = re.replace_all(&got_xml_string, "").to_string();
        }

        // Remove workbookView dimensions which are almost always different and
        // calcPr which can have different Excel version ids.
        if filename == "xl/workbook.xml" {
            let re = Regex::new(
                r#"<workbookView xWindow="\d+" yWindow="\d+" windowWidth="\d+" windowHeight="\d+""#,
            )
            .unwrap();
            exp_xml_string = re.replace(&exp_xml_string, "<workbookView").to_string();
            got_xml_string = re.replace(&got_xml_string, "<workbookView").to_string();

            let re = Regex::new(r"<calcPr[^>]*>").unwrap();
            exp_xml_string = re.replace(&exp_xml_string, "<calcPr/>").to_string();
            got_xml_string = re.replace(&got_xml_string, "<calcPr/>").to_string();
        }

        // Convert the xml strings to vectors for easier comparison.
        let mut exp_xml_vec;
        let mut got_xml_vec;
        if filename.ends_with(".vml") {
            exp_xml_vec = vml_to_vec(&exp_xml_string);
            got_xml_vec = vml_to_vec(&got_xml_string);
        } else {
            exp_xml_vec = xml_to_vec(&exp_xml_string);
            got_xml_vec = xml_to_vec(&got_xml_string);
        }

        // Reorder randomized XML elements in some xlsx xml files to
        // allow comparison testing.
        if filename == "[Content_Types].xml" || filename.ends_with(".rels") {
            exp_xml_vec = sort_xml_file_data(exp_xml_vec);
            got_xml_vec = sort_xml_file_data(got_xml_vec);
        }

        // Ignore certain elements within files, for example <pageMargins> which
        // changes in the lower decimal places.
        if ignore_elements.contains_key(filename.as_str()) {
            let pattern = ignore_elements.get(filename.as_str()).unwrap();
            let re = Regex::new(pattern).unwrap();

            exp_xml_vec = exp_xml_vec
                .into_iter()
                .filter(|x| !re.is_match(x))
                .collect::<Vec<String>>();

            got_xml_vec = got_xml_vec
                .into_iter()
                .filter(|x| !re.is_match(x))
                .collect::<Vec<String>>();
        }

        // Add the filename to the xml vector to help identify where
        // differences occurs.
        exp_xml_vec.insert(0, filename.to_string());
        got_xml_vec.insert(0, filename.to_string());

        if exp_xml_vec != got_xml_vec {
            return (exp_xml_vec, got_xml_vec);
        }
    }

    (vec![String::from("Ok")], vec![String::from("Ok")])
}

// Convert XML string/doc into a vector for comparison testing.
fn xml_to_vec(xml_string: &str) -> Vec<String> {
    let mut xml_elements: Vec<String> = Vec::new();
    let re = regex::Regex::new(r">\s*<").unwrap();
    let tokens: Vec<&str> = re.split(xml_string).collect();

    for token in &tokens {
        let mut element = token.trim().to_string();
        element = element.replace("\r", "");

        // Add back the removed brackets.
        if !element.starts_with('<') {
            element = format!("<{}", element);
        }
        if !element.ends_with('>') {
            element = format!("{}>", element);
        }

        xml_elements.push(element);
    }
    xml_elements
}

// Convert VML string/doc into a vector for comparison testing. Excel VML tends
// to be less structured than other XML so it needs more massaging.
pub(crate) fn vml_to_vec(vml_string: &str) -> Vec<String> {
    let mut vml_string = vml_string.replace(['\r', '\n'], "");

    let re = regex::Regex::new(r"\s+").unwrap();
    vml_string = re.replace_all(&vml_string, " ").into();

    vml_string = vml_string.replace("; ", ";");
    vml_string = vml_string.replace('\'', "\"");
    vml_string = vml_string.replace("<x:Anchor> ", "<x:Anchor>");

    xml_to_vec(&vml_string)
}

// Re-order the elements in an vec of XML elements for comparison purposes. This
// is necessary since Excel can produce the elements of some files, for example
// Content_Types and relationship/.rel files, in a semi-random/hash order.
fn sort_xml_file_data(mut xml_elements: Vec<String>) -> Vec<String> {
    // We don't want to sort the start and end elements.
    let first = xml_elements.remove(0);
    let second = xml_elements.remove(0);
    let last = xml_elements.pop().unwrap();

    // Sort the rest of the elements.
    xml_elements.sort();

    // Add back the start and end elements.
    xml_elements.insert(0, second);
    xml_elements.insert(0, first);
    xml_elements.push(last);

    xml_elements
}

// Check for binary files (as opposed to XML files).
fn is_binary_file(filename: &str) -> bool {
    filename.ends_with(".png")
        || filename.ends_with(".jpeg")
        || filename.ends_with(".bmp")
        || filename.ends_with(".gif")
}

// Create the data structure used in the autofilter tests.
#[allow(dead_code)]
pub fn populate_autofilter_data(worksheet: &mut Worksheet) {
    let data = vec![
        ("East", "Apple", 9000, "July"),
        ("East", "Apple", 5000, "July"),
        ("South", "Orange", 9000, "September"),
        ("North", "Apple", 2000, "November"),
        ("West", "Apple", 9000, "November"),
        ("South", "Pear", 7000, "October"),
        ("North", "Pear", 9000, "August"),
        ("West", "Orange", 1000, "December"),
        ("West", "Grape", 1000, "November"),
        ("South", "Pear", 10000, "April"),
        ("West", "Grape", 6000, "January"),
        ("South", "Orange", 3000, "May"),
        ("North", "Apple", 3000, "December"),
        ("South", "Apple", 7000, "February"),
        ("West", "Grape", 1000, "December"),
        ("East", "Grape", 8000, "February"),
        ("South", "Grape", 10000, "June"),
        ("West", "Pear", 7000, "December"),
        ("South", "Apple", 2000, "October"),
        ("East", "Grape", 7000, "December"),
        ("North", "Grape", 6000, "April"),
        ("East", "Pear", 8000, "February"),
        ("North", "Apple", 7000, "August"),
        ("North", "Orange", 7000, "July"),
        ("North", "Apple", 6000, "June"),
        ("South", "Grape", 8000, "September"),
        ("West", "Apple", 3000, "October"),
        ("South", "Orange", 10000, "November"),
        ("West", "Grape", 4000, "July"),
        ("North", "Orange", 5000, "August"),
        ("East", "Orange", 1000, "November"),
        ("East", "Orange", 4000, "October"),
        ("North", "Grape", 5000, "August"),
        ("East", "Apple", 1000, "December"),
        ("South", "Apple", 10000, "March"),
        ("East", "Grape", 7000, "October"),
        ("West", "Grape", 1000, "September"),
        ("East", "Grape", 10000, "October"),
        ("South", "Orange", 8000, "March"),
        ("North", "Apple", 4000, "July"),
        ("South", "Orange", 5000, "July"),
        ("West", "Apple", 4000, "June"),
        ("East", "Apple", 5000, "April"),
        ("North", "Pear", 3000, "August"),
        ("East", "Grape", 9000, "November"),
        ("North", "Orange", 8000, "October"),
        ("East", "Apple", 10000, "June"),
        ("South", "Pear", 1000, "December"),
        ("North", "Grape", 10000, "July"),
        ("East", "Grape", 6000, "February"),
    ];

    worksheet.write_string_only(0, 0, "Region").unwrap();
    worksheet.write_string_only(0, 1, "Item").unwrap();
    worksheet.write_string_only(0, 2, "Volume").unwrap();
    worksheet.write_string_only(0, 3, "Month").unwrap();

    for (row, data) in data.iter().enumerate() {
        let row = 1 + row as u32;
        worksheet.write_string_only(row, 0, data.0).unwrap();
        worksheet.write_string_only(row, 1, data.1).unwrap();
        worksheet.write_number_only(row, 2, data.2).unwrap();
        worksheet.write_string_only(row, 3, data.3).unwrap();
    }
}
