use std::{collections::HashMap, fs, iter};
mod PageAndWidthCalculation;
use PageAndWidthCalculation::*;
use serde::{Deserialize, Serialize};
pub use lazy_static::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputData {
    pub headerRow: HeaderData,
    pub dataRows: RowData,
    pub pageWidth: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderData {
    pub fontSize: usize,
    pub title: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RowData {
    pub fontSize: usize,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell{
    pub cell_content:String,
    pub cell_width:usize,
    pub cell_content_width:usize,
   
}

 impl Cell{
    pub fn new(content:String,width:usize,content_width:usize)->Cell{
        Cell {
             cell_content: content,
             cell_width: width,
             cell_content_width: content_width, 
              }

    } 

}
#[derive(Debug, Deserialize, Serialize)]
pub enum RowType{
    HeaderRow,
    DataRow
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub row_height: usize,
    pub row_width: usize,
    pub row_type:RowType
}

impl Row {
    pub fn calculate_max_height(cells_height:&Vec<usize>) -> usize {
        let mut max_height: usize = 0;
        for i in 0..cells_height.len() {
            if cells_height[i] > max_height {
                max_height = cells_height[i];
            }
          
        }
        max_height
    }

    pub fn new_header_row(cells: Vec<Cell>,cell_height:&Vec<usize>) -> Row {
        let row_height = Row::calculate_max_height(cell_height);
        let row_width: usize = cells.len();
        let row_type=RowType::HeaderRow;
       
        Row {
            cells,
            row_height,
            row_width,
            row_type,
        }
    }

    pub fn new_data_row(cells: Vec<Cell>,cell_height:&Vec<usize>) -> Row {
        let row_height = Row::calculate_max_height(cell_height);
        let row_width: usize = cells.len();
        let row_type=RowType::DataRow;
       
        Row {
            cells,
            row_height,
            row_width,
            row_type,
        }
    }
   
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub rows: Vec<Row>,
    pub table_height: usize,
    pub table_width: usize,
}

impl Table {
    pub fn new(rows: Vec<Row>) -> Table {
        let mut table_height = 0;
        let table_width: usize = rows.len();
        for i in 0..rows.len() {
            table_height += rows[i].row_height
        }
        Table {
            rows,
            table_height,
            table_width,
        }
    }
}

fn main() {
    let input_data_result = fs::read_to_string("./src/data/data.json");
    let mut input_data:InputData=match input_data_result{
        Ok(val)=> match serde_json::from_str(&val){
            Ok(data)=>data,
            Err(error)=>{
                println!("{}",error);
                return;
            }
        },
        Err(error)=>{
            println!("{}",error);
            return;
        }
    };

    let table_width=calculate_table_width(792, 10);
    let cell_width=calculate_cell_width(table_width, 16);
    let cell_content_width=calculate_cell_content(cell_width, 2);
    
    let len_of_dataRows=input_data.dataRows.rows.len();
    let header_cell_height = spliting_heights_for_header(&mut input_data.headerRow.title,cell_content_width,18,20);
    let mut data_row_cell_height=looping_for_datarow_cells(len_of_dataRows, &mut input_data.dataRows.rows,cell_content_width); 
    let mut vec_of_cells:Vec<Vec<Cell>>=vec![];
    creating_cells(&mut vec_of_cells, input_data, cell_width, cell_content_width);
    let vec_of_row=creating_rows(vec_of_cells,header_cell_height,data_row_cell_height);
    let table= Table::new(vec_of_row);
    match serde_json::to_string_pretty(&table){
        Ok(val)=>match fs::write("./src/data/table.json", val){
            Ok(val)=>val,
            Err(err)=>{
                println!("{}",err);
                return;
            }
        },
        Err(err)=>{
            println!("{}",err);
            return;
        }
    }



    
}

pub fn creating_rows(vec_of_cells:Vec<Vec<Cell>>,header_cell_height:Vec<usize>,data_row_cell_height:Vec<Vec<usize>>)->Vec<Row>{
    let mut row:Vec<Row>=vec![];
    let mut flag_for_header=true;
    let mut count=0;
    for i in vec_of_cells{
        if flag_for_header{
        row.push(Row::new_header_row(i,&header_cell_height ));
        flag_for_header=false;
        }else{
            row.push(Row::new_data_row(i,&data_row_cell_height[count] ));
            count+=1;
        }
     
    }
    row
}

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

pub fn looping_for_datarow_cells(len_of_dataRows:usize,data:&mut Vec<Vec<String>>,cell_content_width:usize)->Vec<Vec<usize>>{
    let mut data_row_cell_height:Vec<Vec<usize>>=vec![]; 
    for i in 0..len_of_dataRows{
        data_row_cell_height.push(spliting_heights_for_header
            (&mut data[i],cell_content_width,18,20))
    }   
    data_row_cell_height
}

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


   


lazy_static!{
    pub static ref HASHMAP: HashMap<char, f64> = {
     let mut char_weights = HashMap::new();
     char_weights.insert('0', 0.5);
     char_weights.insert('1', 0.5);
     char_weights.insert('2', 0.5);
     char_weights.insert('3', 0.5);
     char_weights.insert('4', 0.5);
     char_weights.insert('5', 0.5);
     char_weights.insert('6', 0.5);
     char_weights.insert('7', 0.5);
     char_weights.insert('8', 0.5);
     char_weights.insert('9', 0.5);
     char_weights.insert(' ', 0.0);
     char_weights.insert('!', 0.333);
     char_weights.insert('"', 0.555);
     char_weights.insert('#', 0.5);
     char_weights.insert('$', 0.5);
     char_weights.insert('%', 1.0);
     char_weights.insert('&', 0.83300006);
     char_weights.insert('\'', 0.27800003);
     char_weights.insert('(', 0.333);
     char_weights.insert(')', 0.333);
     char_weights.insert('*', 0.5);
     char_weights.insert('+', 0.57000005);
     char_weights.insert(':', 0.25);
     char_weights.insert('-', 0.333);
     char_weights.insert('.', 0.25);
     char_weights.insert('/', 0.27800003);
     char_weights.insert(',', 0.333);
     char_weights.insert(';', 0.333);
     char_weights.insert('<', 0.57000005);
     char_weights.insert('=', 0.57000005);
     char_weights.insert('>', 0.57000005);
     char_weights.insert('?', 0.5);
     char_weights.insert('@', 0.93000007);
     char_weights.insert('A', 0.72200006);
     char_weights.insert('B', 0.66700006);
     char_weights.insert('C', 0.72200006);
     char_weights.insert('D', 0.72200006);
     char_weights.insert('E', 0.66700006);
     char_weights.insert('F', 0.611);
     char_weights.insert('G', 0.77800006);
     char_weights.insert('H', 0.77800006);
     char_weights.insert('I', 0.38900003);
     char_weights.insert('J', 0.5);
     char_weights.insert('K', 0.77800006);
     char_weights.insert('L', 0.66700006);
     char_weights.insert('M', 0.94400007);
     char_weights.insert('N', 0.72200006);
     char_weights.insert('O', 0.77800006);
     char_weights.insert('P', 0.611);
     char_weights.insert('Q', 0.77800006);
     char_weights.insert('R', 0.72200006);
     char_weights.insert('S', 0.55600005);
     char_weights.insert('T', 0.66700006);
     char_weights.insert('U', 0.72200006);
     char_weights.insert('V', 0.72200006);
     char_weights.insert('W', 1.0);
     char_weights.insert('X', 0.72200006);
     char_weights.insert('Y', 0.72200006);
     char_weights.insert('Z', 0.66700006);
     char_weights.insert('[', 0.333);
     char_weights.insert('\\', 0.27800003);
     char_weights.insert(']', 0.333);
     char_weights.insert('^', 0.58100003);
     char_weights.insert('_', 0.5);
     char_weights.insert('`', 0.333);
     char_weights.insert('a', 0.5);
     char_weights.insert('b', 0.55600005);
     char_weights.insert('c', 0.44400004);
     char_weights.insert('d', 0.55600005);
     char_weights.insert('e', 0.44400004);
     char_weights.insert('f', 0.333);
     char_weights.insert('g', 0.5);
     char_weights.insert('h', 0.55600005);
     char_weights.insert('i', 0.27800003);
     char_weights.insert('j', 0.333);
     char_weights.insert('k', 0.55600005);
     char_weights.insert('l', 0.27800003);
     char_weights.insert('m', 0.83300006);
     char_weights.insert('n', 0.55600005);
     char_weights.insert('o', 0.5);
     char_weights.insert('p', 0.55600005);
     char_weights.insert('q', 0.55600005);
     char_weights.insert('r', 0.44400004);
     char_weights.insert('s', 0.38900003);
     char_weights.insert('t', 0.333);
     char_weights.insert('u', 0.55600005);
     char_weights.insert('v', 0.5);
     char_weights.insert('w', 0.72200006);
     char_weights.insert('x', 0.5);
     char_weights.insert('y', 0.5);
     char_weights.insert('z', 0.44400004);
     char_weights.insert('{', 0.39400002);
     char_weights.insert('|', 0.22000001);
     char_weights.insert('}', 0.39400002);
     char_weights.insert('~', 0.52000004);
     char_weights
 
   };
 
 }
