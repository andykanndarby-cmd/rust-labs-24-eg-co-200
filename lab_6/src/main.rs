use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("lab_6: hello from async main");
    // small await to validate async runtime startup
    sleep(Duration::from_millis(10)).await;
}