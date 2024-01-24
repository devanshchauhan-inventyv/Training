// employee_task.rs

//! This module provides functionality for processing employee data based on positions and skills.
///
/// Module for processing employee data.
///
pub mod employee_processing;

/// Module for file operations related to employee data.
///
/// This module provides functions for reading and writing employee data to and from files in JSON format.
pub mod file_operations;

/// Re-exports common items from the `employee_processing` and `file_operations` modules.
pub use crate::common::Employee;
pub use employee_processing::process_employee_data;
 use crate::employee_task::file_operations::{read_data, write_data};

