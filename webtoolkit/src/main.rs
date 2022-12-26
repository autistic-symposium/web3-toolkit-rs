// main.rs - author: steinkirch

use web3toolkit::run;


#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    run().await;
    
}
