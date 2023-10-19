mod base10x10_codec;
mod base57_codec;
mod threading_test_1;
mod threading_test_2;
mod threading_test_3;

use threading_test_3::example::*;

fn main() {
    let mut thread = NetworkingThread::new();
    match thread.send_task_request(NetworkingTask::ConnectToServer(ConnectToServerTask {
        parameters: ConnectToServerTaskParameters {
            server_address: "lauchsreborn.net".to_string(),
            server_port: 25565,
        },
    })) {
        Ok(_) => println!("Sent task request"),
        Err(_) => println!("Failed to send task request"),
    }
}
