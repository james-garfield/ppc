use tonic::{Status, Response};
use tonic::transport::Channel;
use std::time::Instant;

use ppc::ppc::user_service_client::UserServiceClient;
use ppc::ppc::{QueryResponse, QueryRequest, InsertResponse, InsertRequest, DeleteRequest, DeleteResponse, UpdateRequest, UpdateResponse};

// use crate::server::server::ppc::{QueryResponse, QueryRequest InsertResponse, InsertRequest, DeleteRequest, DeleteResponse, UpdateRequest, UpdateResponse};

use crate::ppc;

#[tokio::main]
async fn _main() -> Result<(), Box<dyn std::error::Error>> {
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

    pub async fn query_call(&mut self, search: String) -> Result<Response<QueryResponse>, Status>  {
        let request = tonic::Request::new(QueryRequest {
            search: search
        });
        let response = self.client.query(request).await?;
        Ok(response)
    }

    pub async fn insert_call(&mut self, username: String, email: String, password: String) -> Result<Response<InsertResponse>, Status> {
        let request = tonic::Request::new(InsertRequest {
            username,
            email,
            password
        });
        let response = self.client.insert(request).await?;
        Ok(response)
    }

    pub async fn update_call(&mut self, username: String, email: String, password: String, id: i32) -> Result<Response<UpdateResponse>, Status> {
        let request = tonic::Request::new(UpdateRequest {
            username,
            email,
            password,
            id: id.to_string()
        });
        let response = self.client.update(request).await?;
        Ok(response)
    }

    pub async fn delete_call(&mut self, id: i32) -> Result<Response<DeleteResponse>, Status> {
        let request = tonic::Request::new(DeleteRequest {
            id: id.to_string()
        });
        let response = self.client.delete(request).await?;
        Ok(response)
    }
            
}