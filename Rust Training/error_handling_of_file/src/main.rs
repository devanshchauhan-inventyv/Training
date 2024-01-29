use std::{fs::{self, File}, io::Read};
fn main() {
    //error handling of a file using match
    // let file_data_result = File::open("./src/test.json");
    // let file_data= match file_data_result{
    //     Ok(file)=>file,
    //     Err(err)=> match err.kind(){
    //             std::io::ErrorKind::NotFound=>match File::create("./src/test.json"){
    //                         Ok(new_file)=>new_file,
    //                         Err(err)=>panic!("problem creating a new file {:?}",err)
    //                     },
    //             other_error=>panic!("not open this file because: {:?}",other_error),
   

    //     }
    // };
    //alternate way of error handling without match expressions 
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == std::io::ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    //reading username from file and if genertes error returning an error using the concept (propagating errors)
    // println!("{:?}",reading_username_from_file());

    //reading username from file with ? operator and if generates error returning an error
    // println!("{:#?}",read_username_with_questionmark_operator());

    //reading username from file with ? operator and if generates error returning an error
    // println!("{:#?}",reading_username_using_standardlib_func());
    
}
fn reading_username_using_standardlib_func()->Result<String,std::io::Error>{
    fs::read_to_string("./src/name1.txt")
}



fn read_username_with_questionmark_operator()->Result<String,std::io::Error>{
    let username_file_result=File::open("./src/name1.txt");
    let mut username_file= match username_file_result{
        Ok(username)=> username,
        Err(err)=>return Err(err)
    };
    let mut username=String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn reading_username_from_file()->Result<String,std::io::Error>{
    let username_file_result= File::open("./src/name.txt");
    let mut username_file = match username_file_result{
        Ok(name)=> name,
        Err(err)=>return Err(err),
    };
    let mut username= String::new();
   

    match username_file.read_to_string(&mut username){

        Ok(_)=>Ok(username),
        Err(err)=>return Err(err),

    }
}