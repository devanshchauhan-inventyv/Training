use serde::Serialize;
use std::fs;

#[derive(Debug,Serialize)]
struct Cell{
    cell_height:usize,
    cell_width:usize,
    cell_content:String
}

impl Cell {
    fn change_cell_height(&mut self,cell_height:usize){
        self.cell_height = cell_height;
    }
}
#[derive(Debug,Serialize)]
struct Row{
    cells:Vec<Cell>,
    row_height:usize,
    row_width:usize
}

impl Row{
    fn calculate_max_height(cells:&Vec<Cell>)->usize{
        let mut max_height:usize=0;
        for i in 0..cells.len(){
            if(cells[i].cell_height>max_height){
                max_height=cells[i].cell_height;
            }

        }
        max_height
    }

    fn new(cells: Vec<Cell>) -> Row {
        let row_height = Row::calculate_max_height(&cells);
        let row_width: usize=cells.len();
        Row {
            cells,
            row_height,
            row_width,
        }
    }

}
#[derive(Debug,Serialize)]
struct Table{
    Rows:Vec<Row>,
    table_height:usize,
    table_width:usize
}

impl Table{
    fn new(Rows:Vec<Row>)->Table{
        let mut table_height=0;
        let table_width: usize=Rows.len();
        for i in 0..Rows.len() {
           table_height+= Rows[i].row_height
        }
    Table{
        Rows,
        table_height,
        table_width
    }
  }

}  
fn main() {
    let cell_11 = Cell{
        cell_height:4,
        cell_width:3,
        cell_content:String::from("this is cell 11")
    };
    let cell_12 = Cell{
        cell_height:3,
        cell_width:3,
        cell_content:String::from("this is cell 12")
    };
    let cell_13 = Cell{
        cell_height:3,
        cell_width:3,
        cell_content:String::from("this is cell 13")
    };
    let cell_14 = Cell{
        cell_height:2,
        cell_width:3,
        cell_content:String::from("this is cell 14")
    };
    let cell_15 = Cell{
        cell_height:4,
        cell_width:3,
        cell_content:String::from("this is cell 15")
    };

    let row1 = Row::new(vec![cell_11, cell_12, cell_13, cell_14, cell_15]);
   

    let cell_21 = Cell{
        cell_height:4,
        cell_width:3,
        cell_content:String::from("this is cell 21")
    };
    let cell_22 = Cell{
        cell_height:3,
        cell_width:3,
        cell_content:String::from("this is cell 22")
    };
    let cell_23 = Cell{
        cell_height:3,
        cell_width:3,
        cell_content:String::from("this is cell 23")
    };
    let cell_24 = Cell{
        cell_height:2,
        cell_width:3,
        cell_content:String::from("this is cell 24")
    };
    let cell_25 = Cell{
        cell_height:4,
        cell_width:3,
        cell_content:String::from("this is cell 25")
    };

    let row2 = Row::new(vec![cell_21, cell_22, cell_23, cell_24, cell_25]);

    let cell_31 = Cell{
        cell_height:6,
        cell_width:3,
        cell_content:String::from("this is cell 31")
    };
    let cell_32 = Cell{
        cell_height:3,
        cell_width:3,
        cell_content:String::from("this is cell 32")
    };
    let cell_33 = Cell{
        cell_height:3,
        cell_width:3,
        cell_content:String::from("this is cell 33")
    };
    let cell_34 = Cell{
        cell_height:2,
        cell_width:3,
        cell_content:String::from("this is cell 34")
    };
    let mut cell_35 = Cell{
        cell_height:4,
        cell_width:3,
        cell_content:String::from("this is cell 35")
    };

    
    Cell::change_cell_height(&mut cell_35, 8);
    let row3 = Row::new(vec![cell_31, cell_32, cell_33, cell_34, cell_35]);

    
    let table=Table::new(vec![row1,row2,row3]);
    
    println!("{:#?}",table);
    
    let data=serde_json::to_string_pretty(&table).expect("failed to convert it onto JSON");
    fs::write("./src/table.json", data).expect("cannot write the data");
    

}
