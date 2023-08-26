use std::process::Command;

use server::server::run_server;

mod client;
mod server;

#[tokio::main]
async fn main() {
    run_server();
}

// async fn run_server() {

//     // let command = "cargo ";
//     // let args = ["run", "--bin", "ppc-server"];

//     // // Create a Command object
//     // let mut cmd = Command::new(command);

//     // // Add arguments to the command
//     // for arg in &args {
//     //     cmd.arg(arg);
//     // }

//     // // Execute the command
//     // let output = cmd.output().expect("Failed to execute command");

//     // // Check if the command was successful
//     // if output.status.success() {
//     //     // Convert the command's output (stdout) to a String
//     //     let output_str = String::from_utf8_lossy(&output.stdout);
//     //     println!("Command output: {}", output_str);
//     // } else {
//     //     // Print an error message
//     //     eprintln!("Command failed: {:?}", output.status);
//     // }
// }

// fn run_client() {
//     let command = "cargo ";
//     let args = ["run", "--bin", "ppc-client"];

//     // Create a Command object
//     let mut cmd = Command::new(command);

//     // Add arguments to the command
//     for arg in &args {
//         cmd.arg(arg);
//     }

//     // Execute the command
//     let output = cmd.output().expect("Failed to execute command");

//     // Check if the command was successful
//     if output.status.success() {
//         // Convert the command's output (stdout) to a String
//         let output_str = String::from_utf8_lossy(&output.stdout);
//         println!("Command output: {}", output_str);
//     } else {
//         // Print an error message
//         eprintln!("Command failed: {:?}", output.status);
//     }

// }
