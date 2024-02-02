use std::{sync::{Arc, RwLock}, thread, time::Duration};

use chrono::Utc;
use rand::{thread_rng, Rng};

use crate::thread_data;

/// Processes data using multiple threads.
///
/// This function takes a vector of `thread_data` and processes it using multiple threads. It spawns three threads:
/// 
/// 1. Thread 1: Adds data to the vector every 5 seconds.
/// 2. Thread 2: Prints the content of the vector every 30 seconds.
/// 3. Thread 3: Deletes data older than 60 seconds from the vector every 60 seconds.
///
/// # Arguments
///
/// * `data` - A vector of `thread_data` structs to be processed.
///
/// # Example
///
/// ```rust
/// use crate::thread_data;
/// 
/// let data = vec![
///     thread_data { id: 1, username: String::from("user1"), timestamp: 1643792186 },
///     thread_data { id: 2, username: String::from("user2"), timestamp: 1643792246 },
///     thread_data { id: 3, username: String::from("user3"), timestamp: 1643792306 },
/// ];
/// process_data(data);
/// ```
pub fn process_data(data: Vec<thread_data>) {
    // Vector to store thread_data structs
    let vector: Vec<thread_data> = vec![];

    // Create an Arc wrapped RwLock to safely share data among threads
    let rwlock = RwLock::new(vector);
    let arc = Arc::new(rwlock);

    // Clone Arc for each thread
    let ref1 = arc.clone();
    let ref2 = arc.clone();
    let ref3 = arc.clone();

    // Counter to assign unique IDs to data elements
    let mut count = 1;

    // Thread 1: Add data to the vector every 5 seconds
    let thread1 = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let id = count;
            let username = create_name(5);
            let now = Utc::now();
            let timestamp = now.timestamp();
            ref1.write().unwrap().push(thread_data { id, username, timestamp });
            count += 1;
            println!("data is added");
        }
    });

    // Thread 2: Print the content of the vector every 30 seconds
    let thread2 = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30));
            let vector = ref2.read().unwrap();
            println!("{:#?}", vector);
        }
    });

    // Thread 3: Delete data older than 60 seconds from the vector every 60 seconds
    let thread3 = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            ref3.write().unwrap().retain(|e| Utc::now().timestamp() - e.timestamp > 60);
            println!("data is deleted ")
        }
    });

    // Wait for all threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}

/// Generates a random username of a specified length.
///
/// # Arguments
///
/// * `length` - The length of the username to be generated.
///
/// # Returns
///
/// A randomly generated username as a String.
///
/// # Example
///
/// ```rust
/// let username = create_name(8);
/// println!("Generated username: {}", username);
/// ```
pub fn create_name(length: usize) -> String {
    let mut string = String::new();
    for _ in 0..length {
        let number = thread_rng().gen_range(65..91);
        let char = std::char::from_u32(number as u32).unwrap_or_else(|| 'f');
        string.push(char);
    }
    string.to_ascii_uppercase()
}
