use axum::Json;
use tikv_client::{codec::{ApiV1RawCodec, EncodedRequest}, RawClient};
use uuid::Uuid;

// use crate::EMPLOYEE;

pub async fn connect_client()->Result<RawClient,String>{
    let client=RawClient::new(vec!["127.0.0.1:2379"]).await;
    match client{
        Ok(client)=>{
            println!("Database has connected");
            Ok(client)
        }
        Err(error)=>{
            Err(error.to_string())
            // println!("{:?}",error);
        }
    }
    
} 

pub async fn put_employee(){
    let client=connect_client().await.unwrap();
    // println!("{:?}",client.);
    let id = format!("E-{}",Uuid::new_v4());
    // let data= EMPLOYEE.read().unwrap().clone();
    // let serialized_data=serde_json::to_string_pretty(&data).map_err(|err|println!("{}",err)).unwrap();
    let res = client.put ("id".to_string(),"heloo".to_string() ).await;
    match res {
        Ok(res) => {
            println!("Record Added");
        }
        Err(error) => {
            println!("{:?}",error);
        }
    }
    // let data=client.get(id.to_string()).await.unwrap().unwrap();
    // println!("{:#?}",data);
}
