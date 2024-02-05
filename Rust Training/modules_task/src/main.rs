mod frequency_task;
use frequency_task::*;
use rand::thread_rng;
use rand::Rng;
use std::cell;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use task_assigner::processing::*;
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
mod task_assigner;
use task_assigner::*;
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

    let creating_user = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(3));
        PENDING_USER_QUEUE
            .write()
            .unwrap()
            .push_front(task_assigner::processing::generate_user());
        println!("NEW USER REQUEST GENERATED");
    });


    let bifurcator = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(4));
        bifurcate();
    });

    let executives_skill_changer = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(10));
        executives_skill_changer();
    });

    let executives_language_changer = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(10));
        executives_language_changer();
    });

    let executives_status_changer = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(10));
        executives_status_changer();
    });

    let decision_maker_chat = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(7));
        decision_maker_for_chat();
    });

    let decision_maker_call = thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(7
        ));
        decision_maker_for_call();
    });

    creating_user.join().unwrap();
    bifurcator.join().unwrap();
    executives_language_changer.join().unwrap();
    executives_skill_changer.join().unwrap();
    executives_status_changer.join().unwrap();
    decision_maker_chat.join().unwrap();
    decision_maker_call.join().unwrap();
}
