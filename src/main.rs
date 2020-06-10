pub mod redis;

#[tokio::main]
async fn main() {
    redis::main().await.unwrap();
}