#![allow(dead_code)]

use ppc::user_service_client::UserServiceClient;
use ppc::{QueryResponse, QueryRequest, InsertResponse, InsertRequest, DeleteRequest, DeleteResponse, UpdateRequest, UpdateResponse};

use tonic::{Status, Response};
use tonic::transport::Channel;

use std::collections::HashMap;
use std::io::{self};

pub mod ppc {
    tonic::include_proto!("myapp");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client_calls = ClientCalls::new().await?;

    // Loop until quit
    loop {
        // Read the command from stdin.
        let mut command = String::new();
        println!("Enter command (insert/update/delete/query):");
        io::stdin().read_line(&mut command)?;

        // The first argument (the command) is read from stdin.
        let command = command.trim();
        if command.to_ascii_lowercase() == "q" || command.to_ascii_lowercase() == "quit" {
            break;
        }

        match command {
            "insert" => {
                // Read key-value pairs until an empty line is encountered.
                let data: HashMap<String, String> = hash_data(command.to_string(), vec!["username", "email", "password"]);
                if data.contains_key("error") {
                    print_error_message();
                    // Continue the loop
                    continue;
                }
               
                let res = client_calls.insert_call(
                    force_get(&data, "username", "INSERT username", None),
                    force_get(&data, "email", "INSERT email", None), 
                    force_get(&data, "password", "INSERT password", None)
                ).await?;
                println!("{:?}", res);
            }
            "update" => {
                let data: HashMap<String, String> = hash_data(command.to_string(), vec!["username", "email", "password", "id"]);
                if data.contains_key("error") {
                    print_error_message();
                    // Continue the loop
                    continue;
                }
               
                // let s: String = data.get("username").expect("msg").to_string();
                let user_id = force_get(&data, "id", "ID must be passed to UPDATE", None);
                let mut cc = client_calls.clone();
                let query_response = cc.query_call(user_id).await?;
                let query_response = query_response.into_inner();

                let res = client_calls.update_call(
                    force_get(&data, "username", "UPDATE username", Some(query_response.username)), 
                    force_get(&data, "email", "UPDATE email", Some(query_response.email)), 
                    force_get(&data, "password", "UPDATE password", Some(query_response.password)), 
                    force_get(&data, "id", "UPDATE id", None).parse::<i32>().expect("UPDATE id String to i32")
                ).await?;
                println!("{:?}", res);

            }
            "delete" => {
                let data: HashMap<String, String> = hash_data(command.to_string(), vec!["id"]);
                if data.contains_key("error") {
                    print_error_message();
                    // Continue the loop
                    continue;
                }

                let res = client_calls.delete_call(
                    force_get(&data, "id", "DELETE id", None).parse::<i32>().expect("DELETE id String to i32")
                ).await?;
                println!("{:?}", res);
            }
            "query" => {
                let data: HashMap<String, String> = hash_data(command.to_string(), vec!["search"]);
                if data.contains_key("error") {
                    print_error_message();
                    // Continue the loop
                    continue;
                }

                let res = client_calls.query_call(
                    force_get(&data, "search", "QUERY search", None)
                ).await?;
                println!("{:?}", res);
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }

    Ok(())
}

/// Print error message
fn print_error_message() {
    println!("That key is not a valid key.");
}

/// A get helper for a HashMap<String, String>
fn force_get(map: &HashMap<String, String>, key: &str, expect_message: &str, default_value: Option<String>) -> String {
    let value = map.get(key);
    match default_value {
        Some(default) => {
            match value {
                Some(data) => data.to_owned(),
                None => default,
            }
        },
        None => value.expect(expect_message).to_string(),
    }
    // return map.get(key).expect(expect_message).to_string();
}

/// Convert the input into a HashMap
fn hash_data(command: String, allowed_fields: Vec<&str>) -> HashMap<String, String> {
    // The data to be returned
    let mut data = HashMap::new();
    // If ERROR
    let error = |input: &str, mut data: HashMap<String, String>| -> HashMap<String, String> {
        data.insert(
            "error".to_string(),
            format!("Invalid key-value pair: {}, For command: {}", input, command),
        );

        data
    };
    // Loop until empty string passed into arg
    loop {
        // Grab input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read line");
        let input = input.trim();

        // If empty break out of loop, i.e. no more args to pass
        if input.is_empty() {
            break;
        }

        // Process the insert command.
        let parts: Vec<&str> = input.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0];
            // Check that key is a valid field
            if allowed_fields.contains(&key) {
                let value = parts[1];
                data.insert(key.to_string(), value.to_string());
            } else {
                data = error(input, data.clone());
                break;
            }
        } else {
            data = error(input, data.clone());
            break;
        }
    }
    data
}

#[derive(Clone)]
pub struct ClientCalls {
    client: UserServiceClient<Channel>
}

impl ClientCalls {
    async fn new() -> Result<ClientCalls, tonic::transport::Error> {
        let client = ClientCalls {
            client: UserServiceClient::connect("http://[::1]:50051").await?,
        };
        Ok(client)
    }

    async fn query_call(&mut self, search: String) -> Result<Response<QueryResponse>, Status>  {
        let request = tonic::Request::new(QueryRequest {
            search: search
        });
        let response = self.client.query(request).await?;
        Ok(response)
    }

    async fn insert_call(&mut self, username: String, email: String, password: String) -> Result<Response<InsertResponse>, Status> {
        let request = tonic::Request::new(InsertRequest {
            username,
            email,
            password
        });
        let response = self.client.insert(request).await?;
        Ok(response)
    }

    async fn update_call(&mut self, username: String, email: String, password: String, id: i32) -> Result<Response<UpdateResponse>, Status> {
        let request = tonic::Request::new(UpdateRequest {
            username,
            email,
            password,
            id: id.to_string()
        });
        let response = self.client.update(request).await?;
        Ok(response)
    }

    async fn delete_call(&mut self, id: i32) -> Result<Response<DeleteResponse>, Status> {
        let request = tonic::Request::new(DeleteRequest {
            id: id.to_string()
        });
        let response = self.client.delete(request).await?;
        Ok(response)
    }   
}