use accounts::account;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    account().await;
}