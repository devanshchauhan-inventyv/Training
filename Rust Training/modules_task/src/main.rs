mod frequency_task;
use chrono::DateTime;
use chrono::Utc;
use frequency_task::*;
use std::cell;
use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::sync::Arc;
use std::sync::RwLock;
mod common;
use common::*;
mod student_task;
use student_task::*;
mod employee_task;
use employee_task::*;
mod hashmap;
use hashmap::employee_task as employee_hashmap;
use hashmap::student_task as student_hashmap;
mod table_task;
use table_task::*;
mod threads_task;
use threads_task::*;
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

    // Process Employee Data
    // Uncomment the following below 2 lines to process employee data
    // let employee_file_path = "src/employee_task/data/Employee.json";
    // process_employee_data(employee_file_path);

    // Process Employee Data in Hashmap
    // Uncomment the following below 2 lines to process employee data in hashmap
    // let employee_file_path = "src/employee_task/data/Employee.json";
    // employee_hashmap::employee_processing::process_employee_data(employee_file_path);

    // Process Student Data in Hashmap
    // Uncomment the following below 2 lines to process student data in hashmap
    // let student_file_path = "src/hashmap/student_task/data/StudentData.json";
    // student_hashmap::process_student_data(student_file_path);

    //Process Table Task 
    //Uncomment the following below 3 lines to make a table
    // let input_file_path = "src/table_task/data/data.json";
    // let output_file_path = "src/table_task/data/table.json";
    // table_task::processing::process_input_data(input_file_path, output_file_path);
    
    //start thread task
    /*it adds data to vec every 5sec displays every 30sec 
    and deletes the data from vec which was added before 60sec */ 
    // let vector: Vec<thread_data> =vec![];
    // threads_task::processing::process_data(vector);
   


}




