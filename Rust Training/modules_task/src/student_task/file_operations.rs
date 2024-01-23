//file_operations.rs
//! Module for file operations related to Student data.
//!
//! This module provides functions to read and write student data from/to a JSON file.

use crate::common::Student;
use serde_json;
use std::fs;

/// Reads student data from a JSON file and Returns a vector of `Student` structs read from the file.
///
pub fn read_data(file_path: &str) -> Vec<Student> {
    let data = fs::read_to_string(file_path).expect("Failed to read");
    serde_json::from_str(&data).expect("Something went wrong")
}

/// Writes student data to a JSON file.
///
pub fn write_data(file_path: &str, data: &Vec<Student>) {
    let updated_data = serde_json::to_string_pretty(data).expect("Failed to convert to JSON");
    fs::write(file_path, updated_data).expect("Something went wrong");
}
