use std::process::Command;
use tokio::time::{Duration, sleep};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Number of requests
    let requests = 1;
    // let time_start = 

    let mut handles = vec![];

    for i in 0..requests {
        let handle = tokio::spawn(open_and_run_client());
        handles.push(handle);
    }

    // Wait for all coroutines to complete
    for handle in handles {
        let _ = handle.await;
    }

    println!("All coroutines are complete");
    Ok(())
}

async fn open_and_run_client() {
    // Open terminal
    let command = "cargo run --bin ppc-client";
    let mut status = Command::new("open");
    status.arg(command);
    let output = status.output()

    // Doing an input command
    // Doing an update command
    // Doing a query command
    // Doing a delete command
}