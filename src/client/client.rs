use ppc::user_service_client::UserServiceClient;
use tonic::Status;
use tonic::transport::Channel;
use std::time::Instant;

use crate::server::server::ppc::{QueryResponse, QueryRequest InsertResponse, InsertRequest, DeleteRequest, DeleteResponse, UpdateRequest, UpdateResponse};

pub mod ppc {
    tonic::include_proto!("myapp");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UserServiceClient::connect("http://[::1]:50051").await?;

    // Demoing multiple requests
    let mut tasks = vec![];

    // Number of requests to simulate
    let num_requests = 1000;
    
    for i in 1..num_requests {
        let mut client = client.clone();
        let task = tokio::spawn(async move {
            let request = tonic::Request::new(QueryRequest{
                search: "3".to_string()
            });

            let response = client.query(request).await;
            println!("Response={:?}", response); 
        });

        tasks.push(task);        
    }

    let start_time = Instant::now();

    for task in tasks {
        task.await;
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Time taken: {:?}", elapsed_time);

    Ok(())
}

pub struct ClientCalls {
    client: UserServiceClient<Channel>
}

impl ClientCalls {
    pub async fn new() -> Result<ClientCalls, tonic::transport::Error> {
        let client = ClientCalls {
            client: UserServiceClient::connect("http://[::1]:50051").await?,
        };
        Ok(client)
    }

    pub async fn query_call(&self, search: String) -> Result<QueryResponse, Status>  {
        let request = tonic::Request::new(QueryRequest {
            search: search
        });
        let response = self.client.query(request).await?;
        response
    }

    pub async fn insert_call(&self, username: String, email: String, password: String) -> Result<InsertResponse, Status> {
        let request = tonic::Request::new(InsertRequest {
            username,
            email,
            password
        });
        let response = self.client.insert(request).await?;
        response
    }
    // message InsertRequest {
    //     string username = 1;
    //     string email = 2;
    //     string password = 3;
    // }
    
    // message InsertResponse {
    //     string id = 1;
    // }
    
}