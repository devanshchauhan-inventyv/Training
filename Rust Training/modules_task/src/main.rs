mod frequency_task;
use frequency_task::*;
mod common;
mod student_task;
use student_task::*;
mod employee_task;
use employee_task::*;
// main.rs

/// The main function of the program.
///
/// This function serves as the entry point of the program. It currently demonstrates the
/// processing of frequency tasks. Additional calls to process student and employee data
/// are commented out and can be uncommented as needed.
fn main() {
    // Process frequency task
    //Uncomment the following below line to perform frequency task
    // process_frequency_task();

    //Process Student Data
    // Uncomment the following below 2 lines to process student data
    // let student_file_path = "src/student_task/data/StudentData.json";
    // process_student_data(student_file_path);

    //Process Employee Data
    // Uncomment the following below 2 lines to process employee data
    // let employee_file_path = "src/employee_task/data/Employee.json";
    // process_employee_data(employee_file_path);
}

