use futures::stream::StreamExt;
use num_stream::num_stream;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    let mut nums = num_stream(0, 3, Duration::from_millis(500));
    loop {
        println!("Got: {:?}", nums.next().await);
    }
}