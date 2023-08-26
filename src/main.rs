// use server::server::run_server;
// use client::client::ClientCalls;
use std::process::Command;

// mod client;
// mod server;
// mod ppc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let _ = run_server().await;
    // // Run server and client in seperate thread
    // let (tx, mut rx) = mpsc::channel(32);
    // tokio::spawn(async move {
    //     // Initialize the client, make gRPC calls, and send responses through the channel
    //     let mut client_calls = ClientCalls::new().await.expect("ClientCalls returned.");
    //     // Make requests
    //     let res = client_calls.query_call("user".to_string()).await.expect("Returne QueryResponse");
    //     tx.send(res).await.expect("Send error");
    // });
    
    // // Simulate receiving!
    // while let Some(response) = rx.recv().await {
    //     println!("Received response: {:?}", response);
    // }

    Ok(())
}