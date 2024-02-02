// processing.rs

use crate::common::*;
use super::file_operations::{read_input_data, write_table_data};
use super::PageAndWidthCalculation::*;
/// Processes the input data, generates a table, and writes the table data to an output file.
/// 
/// # Arguments
/// 
/// * `input_file_path` - The path to the input JSON file containing the data.
/// * `output_file_path` - The path to the output JSON file where the table data will be written.
pub fn process_input_data(input_file_path: &str, output_file_path: &str) {
    // Read input data from file
    let input_data = match read_input_data(input_file_path) {
        Ok(data) => data,
        Err(error) => {
            println!("Error reading input data: {}", error);
            return;
        }
    };

      // Process input data and generate the table
    let table = process_data(input_data);

     // Write table data to output file
    if let Err(error) = write_table_data(&table, output_file_path) {
        println!("Error writing table data: {}", error);
    }
}
/// Processes the input data to generate a table.
/// 
/// # Arguments
/// 
/// * `input_data` - The input data read from the JSON file.
/// 
/// # Returns
/// 
/// Returns the generated table.

fn process_data(mut input_data: InputData) -> Table {
    // Process input data here and generate the table
    // Example: Extract data and create cells and rows
    let table_width=calculate_table_width(792, 10);
    let cell_width=calculate_cell_width(table_width, 16);
    let cell_content_width=calculate_cell_content(cell_width, 2);
    println!("{:#?}",input_data);
    let len_of_dataRows=input_data.dataRows.rows.len();
    let header_cell_height = spliting_heights_for_header(&mut input_data.headerRow.title,cell_content_width,18,20);
    let mut data_row_cell_height=looping_for_datarow_cells(len_of_dataRows, &mut input_data.dataRows.rows,cell_content_width); 
    let mut vec_of_cells:Vec<Vec<Cell>>=vec![];
    creating_cells(&mut vec_of_cells, input_data, cell_width, cell_content_width);
    let vec_of_row = creating_rows(vec_of_cells, header_cell_height, data_row_cell_height);
    Table::new(vec_of_row)
}
/// Creates rows for the table.
/// 
/// # Arguments
/// 
/// * `vec_of_cells` - Vector of vectors containing the cells for each row.
/// * `header_cell_height` - Vector containing the heights of cells in the header row.
/// * `data_row_cell_height` - Vector of vectors containing the heights of cells in each data row.
/// 
/// # Returns
/// 
/// Returns a vector containing the rows of the table.
pub fn creating_rows(vec_of_cells:Vec<Vec<Cell>>,header_cell_height:Vec<usize>,data_row_cell_height:Vec<Vec<usize>>)->Vec<Row>{
    let mut row:Vec<Row>=vec![];
    let mut flag_for_header=true;
    let mut count=0;
    for i in vec_of_cells{
        if flag_for_header{
        row.push(Row::new(i,&header_cell_height ,RowType::HeaderRow));
        flag_for_header=false;
        }else{
            row.push(Row::new(i,&data_row_cell_height[count] ,RowType::DataRow));
            count+=1;
        }
     
    }
    row
}
/// Creates cells for the table based on the input data.
/// 
/// # Arguments
/// 
/// * `vec_of_cells` - A mutable reference to a vector of vectors containing the cells for each row.
/// * `data` - The input data containing the header row and data rows.
/// * `cell_width` - The width of each cell.
/// * `cell_content_width` - The width of the content area of each cell.
pub fn creating_cells(Vec_of_Cells:&mut Vec<Vec<Cell>>,data:InputData,cell_width:usize,cell_content_width:usize){
    let mut header_cells:Vec<Cell>=vec![];

    for i in data.headerRow.title{
        header_cells.push(Cell::new(i, cell_width, cell_content_width))

    }
    Vec_of_Cells.push(header_cells);
    
    for i in data.dataRows.rows{
        let mut rowdata_cell:Vec<Cell>=vec![];
        for j in i{ 
        rowdata_cell.push(Cell::new(j, cell_width, cell_content_width));
        }
        Vec_of_Cells.push(rowdata_cell);
        
    }




}
/// Loops through data rows to calculate cell heights for each row.
/// 
/// # Arguments
/// 
/// * `len_of_dataRows` - The number of data rows.
/// * `data` - A mutable reference to a vector of vectors containing the data rows.
/// * `cell_content_width` - The width of the content area of each cell.
/// 
/// # Returns
/// 
/// Returns a vector of vectors containing the heights of cells in each data row.
pub fn looping_for_datarow_cells(len_of_dataRows:usize,data:&mut Vec<Vec<String>>,cell_content_width:usize)->Vec<Vec<usize>>{
    let mut data_row_cell_height:Vec<Vec<usize>>=vec![]; 
    for i in 0..len_of_dataRows{
        data_row_cell_height.push(spliting_heights_for_header
            (&mut data[i],cell_content_width,18,20))
    }   
    data_row_cell_height
}
/// Splits content of header rows into multiple lines if it exceeds the width of a cell.
/// 
/// # Arguments
/// 
/// * `data` - A mutable reference to a vector of strings containing the content of header rows.
/// * `cell_content_width` - The width of the content area of each cell.
/// * `header_font_size` - The font size of the header.
/// * `cell_height` - The height of each cell.
/// 
/// # Returns
/// 
/// Returns a vector containing the heights of cells in the header row.
pub fn spliting_heights_for_header(data:&mut Vec<String>,cell_content_width:usize,header_font_size:usize,mut cell_height:usize)->Vec<usize>{
    let mut vec_cells_height:Vec<usize>=vec![];
    for content in data{
        let mut width=0.0;
        let mut index=0;
        let mut cell_index=0;
        let mut temp_cell_height=cell_height;
        while index < content.len() {
            let c = content.chars().nth(index).unwrap();
            width += *HASHMAP.get(&c).expect("Character is not in hashmap")*header_font_size as f64;
            if width > cell_content_width as f64{
                content.insert_str(index, "\n");
                temp_cell_height += 20;
                width = 0.0;
                index += 1; 
            } else {
                index += 1;
            }
        }
        vec_cells_height.push(temp_cell_height);
    }
    vec_cells_height
}