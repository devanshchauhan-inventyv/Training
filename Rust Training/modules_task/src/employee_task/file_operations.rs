// file_operations.rs
use crate::common::Employee;
use serde_json;
use std::fs;
/// Reads employee data from a JSON file and deserializes it into a vector of Employee structs.
///
pub fn read_data(file_path: &str) -> Vec<Employee> {
    let data = fs::read_to_string(file_path).expect("Failed to read");
    serde_json::from_str(&data).expect("Something went wrong")
}
/// Writes employee data to a JSON file in a pretty-printed format.
///
pub fn write_data(file_path: &str, data: &Vec<Employee>) {
    let updated_data = serde_json::to_string_pretty(data).expect("Failed to convert to JSON");
    fs::write(file_path, updated_data).expect("Something went wrong");
}
