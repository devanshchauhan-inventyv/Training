use serde_json;
use std::fs;
use crate::Table;
use crate::common::*;
/// Reads input data from a JSON file located at the specified `file_path`.
/// 
/// # Arguments
/// 
/// * `file_path` - A string slice that holds the path to the JSON file.
/// 
/// # Returns
/// 
/// Returns a `Result` containing `InputData` if successful, or a `String` error message if an error occurs.
/// 
/// # Errors
/// 
/// This function returns an error if there are any issues reading the file or parsing the JSON data.
/// 
/// # Example
/// 
/// ```
/// use crate::common::*;
/// use crate::Table;
/// 
/// let input_data = read_input_data("data.json");
/// match input_data {
///     Ok(data) => println!("Input data: {:?}", data),
///     Err(error) => eprintln!("Error: {}", error),
/// }
/// ```
pub fn read_input_data(file_path: &str) -> Result<InputData, String> {
     // Implementation details...
    let data = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };
    
    match serde_json::from_str(&data) {
        Ok(input_data) => Ok(input_data),
        Err(e) => Err(e.to_string()),
    }
}
/// Writes table data to a JSON file located at the specified `file_path`.
/// 
/// # Arguments
/// 
/// * `table_data` - A reference to a `Table` object containing the data to be written.
/// * `file_path` - A string slice that holds the path to the JSON file.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if successful, or a `String` error message if an error occurs.
/// 
/// # Errors
/// 
/// This function returns an error if there are any issues writing the data to the file.
/// 
/// # Example
/// 
/// ```
/// use crate::Table;
/// 
/// let table_data = Table::new(/* initialize table data */);
/// let result = write_table_data(&table_data, "output.json");
/// match result {
///     Ok(()) => println!("Table data written successfully."),
///     Err(error) => eprintln!("Error: {}", error),
/// }
/// ```

pub fn write_table_data(table_data: &Table, file_path: &str) -> Result<(), String> {
     // Implementation details...
    let data = match serde_json::to_string_pretty(table_data) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };
    match fs::write(file_path, data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}


