use tonic::transport::Channel;

use tokio::time::{Instant};

use client::ClientCalls;

use std::env;

mod client;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // Number of requests
    let mut requests = 1000;
    // let max_req = 1500;

    // loop {
        run_stuff(requests).await;
    //     requests = requests - max_req;
    //     if requests <= 0 {
    //         break;
    //     }
    // }

    println!("All coroutines are complete");
    Ok(())
}

async fn run_stuff(requests: i32) {
    // Record the start time
    let start_time = Instant::now();

    let mut handles = vec![];
    for _ in 0..requests {
        let handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
            // http://[::1]:50051
            let channel = Channel::from_shared("http://[::1]:50051")
                .unwrap()
                .connect()
                .await
                .unwrap();
            let mut client = ClientCalls::new_with_channel(channel).await.expect("Client calls received");

            let i = client.insert_call("john".to_string(), "john@doe.com".to_string(), "JohnsPassword".to_string()).await;
            let res = i.expect("Exepct insert").into_inner();
            let u = client.update_call("jordan".to_string(), "email".to_string(), "pass".to_string(), res.id.parse::<i32>().expect("")).await;
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    // Record the end time
    let end_time = Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time);

    // Print the elapsed time in seconds and milliseconds
    println!("Elapsed time: {} seconds", elapsed_time.as_secs());
    println!("Elapsed time: {} milliseconds", elapsed_time.as_millis());

}

// fn open_and_run_client() {
//     // Open terminal
//     // Start the cargo run command
//     let mut child = Command::new("cargo")
//     .arg("run")
//     .arg("--bin")
//     .arg("ppc-client")
//     .stdin(Stdio::piped())
//     .stdout(Stdio::piped())
//     .stderr(Stdio::piped())
//     .spawn()
//     .expect("Failed to start cargo run");

//     // Define the input data
//     let input_data = "insert\nusername=john\nemail=john@doe.com\npassword=JohnsPassword";

//     // Write the input data to the child process's stdin
//     if let Some(mut stdin) = child.stdin.take() {
//         stdin.write_all(input_data.as_bytes()).expect("Failed to write to stdin");
//     }

//     // Wait for the child process to complete and capture its output
//     let output = child.wait_with_output().expect("Failed to wait for child process");
//     // child.kill();

//     // Print the output
//     println!("Exit status: {:?}", output.status);
//     println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

//     // Doing an input command
//     // Doing an update command
//     // Doing a query command
//     // Doing a delete command
// }
