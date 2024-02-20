use tikv_client::RawClient;

#[tokio::main]
async fn main(){
    let client=connect_client().await.unwrap();
    // println!("{:?}",client.);
    let id = 1;

    println!("{}",id);
    // let data= EMPLOYEE.read().unwrap().clone();
    // let serialized_data=serde_json::to_string_pretty(&data).map_err(|err|println!("{}",err)).unwrap();
    let res = client.put(id.to_string() ,"hello".to_string()).await;
    match res {
        Ok(res) => {
            println!("Record Added");
        }
        Err(error) => {
            println!("{:?}",error);
        }
    }
}
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

