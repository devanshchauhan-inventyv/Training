/// Calculates the width of the table based on the page width and both-side margins.
/// 
/// # Arguments
/// 
/// * `page_width` - The total width of the page.
/// * `both_side_margin` - The width of the margins on both sides of the table.
/// 
/// # Returns
/// 
/// Returns the calculated width of the table.
/// 
/// # Example
/// 
/// ```
/// let page_width = 792;
/// let both_side_margin = 20;
/// let table_width = calculate_table_width(page_width, both_side_margin);
/// assert_eq!(table_width, 752);
/// ```
pub fn calculate_table_width(page_width:usize,both_side_margin:usize)->usize{
    let mut table_width=0;
    table_width=page_width-both_side_margin;
    table_width
}

/// Calculates the width of each cell in the table based on the total table width and the number of columns.
/// 
/// # Arguments
/// 
/// * `table_width` - The total width of the table.
/// * `number_of_columns` - The total number of columns in the table.
/// 
/// # Returns
/// 
/// Returns the calculated width of each cell.
/// 
/// # Example
/// 
/// ```
/// let table_width = 752;
/// let number_of_columns = 5;
/// let cell_width = calculate_cell_width(table_width, number_of_columns);
/// assert_eq!(cell_width, 150);
/// ```
pub fn calculate_cell_width(table_width:usize,number_of_columns:usize)->usize{
    let mut cell_width=0;
    cell_width=table_width/number_of_columns;
    cell_width
}

/// Calculates the width of the content within each cell based on the cell width and padding.
/// 
/// # Arguments
/// 
/// * `cell_width` - The width of each cell.
/// * `padding` - The padding applied to the content within each cell.
/// 
/// # Returns
/// 
/// Returns the calculated width of the content within each cell.
/// 
/// # Example
/// 
/// ```
/// let cell_width = 150;
/// let padding = 10;
/// let cell_content_width = calculate_cell_content(cell_width, padding);
/// assert_eq!(cell_content_width, 130);
/// ```

pub fn calculate_cell_content(cell_width:usize,padding:usize)->usize{
    let mut cell_content_width=0;
    cell_content_width=cell_width-(2*padding);
    cell_content_width
}