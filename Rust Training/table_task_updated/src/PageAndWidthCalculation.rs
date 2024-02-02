pub fn calculate_table_width(page_width:usize,both_side_margin:usize)->usize{
    let mut table_width=0;
    table_width=page_width-both_side_margin;
    table_width
}
pub fn calculate_cell_width(table_width:usize,number_of_columns:usize)->usize{
    let mut cell_width=0;
    cell_width=table_width/number_of_columns;
    cell_width
}

pub fn calculate_cell_content(cell_width:usize,padding:usize)->usize{
    let mut cell_content_width=0;
    cell_content_width=cell_width-(2*padding);
    cell_content_width
}