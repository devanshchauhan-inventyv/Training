mod file_operations;
use crate::common::*;
use file_operations::*;
pub fn process_student_data(file_path: &str) {
    let mut final_data: Vec<Student> = read_data(file_path);
    let file_path1 = "src/hashmap/student_task/data/result_student.json";

    for iterator in &mut final_data {
        iterator.percentage = iterator.calculate_percentage();
        iterator.grade = iterator.calculate_grade();
    }

    let hashmap_data = vec_to_hashmap(final_data);

    write_data(file_path1, &hashmap_data);
}
