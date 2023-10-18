mod base10x10_codec;
mod base57_codec;
mod threading_test_1;
mod threading_test_2;
mod threading_test_3;

use threading_test_3::{example::*, task::*, thread::*};

fn main() {
    let mut thread = NetworkingThread::new();
    thread.send_task_request(NetworkingTask::ConnectToServer(ConnectToServerTask::new(ConnectToServerTaskInput)));
    match thread.receive_result() {
        
    }
}
