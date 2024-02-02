// common.rs
//! This module contains all the structures and Enum which is used in this library

use std::{collections::HashMap, time::Duration};
use chrono::prelude::*;
///
use serde::{Deserialize, Serialize};
pub use lazy_static::*;

#[derive(Debug, Deserialize, Serialize)]
/// This is a Student Structure
pub struct Student {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<char>,
}
/// This is a implementation block for the student struct
impl Student {
    /// This function calculates percentage of student and return an [Option<f32>]
    pub fn calculate_percentage(&self) -> Option<f32> {
        let mut sum: f32 = 0.0;
        for iter in 0..self.marks.len() {
            sum += self.marks[iter];
        }
        Some(sum / self.marks.len() as f32)
    }
    /// This function calculates grades of student and return an grade in [Option<char>]
    ///
    pub fn calculate_grade(&self) -> Option<char> {
        // Here if percentage is 85 up this return an 'A'
        if self.percentage > Some(85.0) {
            Some('A')
        // For 80 up this returns a 'B'
        } else if self.percentage > Some(80.0) {
            Some('B')
            // For 70 up this returns a 'C'
        } else if self.percentage > Some(70.0) {
            Some('C')
            // For 70 and below this returns a 'D'
        } else {
            Some('D')
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
/// This is a Employee struct
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub skills: Vec<String>,
    pub position: Option<Position>,
    #[serde(rename = "experiance(y)")]
    pub experiance_y: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Hash, Eq)]
/// This is a Position Enum of the employee struct
pub enum Position {
    #[serde(rename = "Sr. Software Developer")]
    SrSoftwareDeveloper,
    #[serde(rename = "Jr. Software Developer")]
    JrSoftwareDeveloper,
    #[serde(rename = "Software Developer")]
    SoftwareDeveloper,
    #[serde(rename = "Team Lead")]
    TeamLead,
    #[serde(rename = "Project Manager")]
    ProjectManager,
}

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
    pub number_of_columns:usize,
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

    pub fn new(cells: Vec<Cell>,cell_height:&Vec<usize>,row_type:RowType) -> Row {
        let row_height = Row::calculate_max_height(cell_height);
        let number_of_columns: usize = cells.len();
        Row {
            cells,
            row_height,
            number_of_columns,
            row_type,
        }
    }

   
   
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub rows: Vec<Row>,
    pub table_height: usize,
    pub number_of_rows: usize
}

impl Table {
    pub fn new(rows: Vec<Row>) -> Table {
        let mut table_height = 0;
        let number_of_rows: usize = rows.len();
        for i in 0..rows.len() {
            table_height += rows[i].row_height
        }
        Table {
            rows,
            table_height,
            number_of_rows
        }
    }
}

#[derive(Debug)]
pub struct thread_data{
    pub id:usize,
    pub username:String,
    pub timestamp: i64,
    
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

