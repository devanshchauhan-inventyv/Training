use std::fs;
use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize,Serialize)]
struct Student{
    name:String,
    phone:String,
    email:String,
    city:String,
    address:String,
    marks:Vec<f32>,
    percentage:Option<f32>,
    grade:Option<char>
}
impl Student{
    fn calculate_percentage(&self) -> Option<f32>{
        let mut sum: f32=0.0;
        for iter in 0..self.marks.len()  {
             sum+= self.marks[iter];
        }
        Some(sum/self.marks.len()as f32)
      
    }
    fn calculate_grade(&self)->Option<char>{
       if self.percentage>Some(85.0){
        Some('A')
       }
       else if self.percentage>Some(80.0){
         Some('B')   
        }
        else if self.percentage>Some(70.0){
            Some('C')    
          }
          else{
            Some('D')
          }
        
    }

}

fn main() {
  
   let data =  fs::read_to_string("C:\\Users\\devan\\Downloads\\StudentData.json").expect("Failed to read");
   let mut final_data:Vec<Student>= serde_json::from_str(&data).expect("something went wrong");
 
    
    for iterator in &mut final_data  {
        iterator.percentage=iterator.calculate_percentage() ;
        iterator.grade=iterator.calculate_grade();
    }
    println!("{:#?}",final_data);

    let updated_data=serde_json::to_string_pretty(&final_data)
        .expect("failed to convert it into JSON");

    fs::write("C:\\Users\\devan\\Downloads\\StudentData.json",
         updated_data).expect("Somethng went wrong");

}
