use std::{error::Error, fs};

pub struct Config {
    pub query: String,
 pub file_path: String,
 }
 
  impl Config{
     pub fn build(mut args: Vec<String>) -> Result<Config,&'static str> {
         if args.len()<3{
             return Err("not enough arguments")
         }else{
             args.remove(0);
             let query = args.remove(0);
             let file_path = args.remove(0);
             Ok(Config { query, file_path })
         }
     }
 }
 
 pub fn run(config:Config)-> Result<(),Box<dyn Error>>{
     let contents = fs::read_to_string(config.file_path)?;
     println!("With text:\n{contents}");
     Ok(())
 }