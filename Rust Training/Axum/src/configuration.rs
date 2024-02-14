use config::{Config, ConfigBuilder, File, FileFormat};
use serde::Deserialize;
use std::{default, env};



pub fn config_example(){
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
    let builder = Config::builder()
    .set_default("default", "1").unwrap()
    .add_source(File::with_name("./src/resources/config"))
    .add_source(File::with_name(&format!("./src/resources/config-{}",run_mode)).required(true))
    .add_source(File::with_name(&format!("./src/resources/config-test")).required(true));
    let app=builder.build().unwrap();

    println!("{:#?}",app.get_string("database.username").unwrap());

}
