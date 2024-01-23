//student_task.rs
//! A module which calculates the student's percentage and student's grade
/// 
/** This module also uses a child module file_operations to read and write
  data in JSON and also uses the common.rs module for accessing the struct and
   implementation function of student  */
mod file_operations;
use crate::{
    common::Student,
    student_task::file_operations::{read_data, write_data},
};
/**This function processes the student data applies the function for calculating percentage and grade
    writes the data into the same JSON file overriding the previous data*/ 
pub fn process_student_data(file_path: &str) {
    let mut final_data: Vec<Student> = read_data(file_path);

    for iterator in &mut final_data {
        iterator.percentage = iterator.calculate_percentage();
        iterator.grade = iterator.calculate_grade();
    }

    println!("{:#?}", final_data);

    write_data(file_path, &final_data);
}
