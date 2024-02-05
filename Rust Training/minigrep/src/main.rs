use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:#?}",args); //same as dbg! macro
    // dbg!(&args);

    //first apporach
    /*let config = match Config::build(args){
        Ok(val)=>val,
        Err(err)=>{
            println!("{}",err);
            return
        }
    };*/

    //second approach
    let config = Config::build(args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) =minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);  
    };
    
    

  
}


