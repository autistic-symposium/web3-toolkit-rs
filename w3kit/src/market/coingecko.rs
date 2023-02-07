// market/coingecko.rs
// author: steinkirch

use std::env;
use serde_json::{Value};


////////////////////////////
// Private functions
////////////////////////////

async fn get_price(coin: &str, currency: &str) -> String {

    println!("✅ fetching price for {} in {}", coin, currency);
    let coingecko_url = &env::var("COINGECKO_API_URL").expect("⛔️ No COINGECKO_API_URL on .env file");
    let url = format!("{}simple/price?ids={}&vs_currencies={}", coingecko_url, coin, currency);
    println!("✅ GET {}", url);

    reqwest::get(url).await.unwrap().text().await.unwrap()
}


////////////////////////////
// Public functions
////////////////////////////

pub async fn get_coin_price(coin_name: &str) {

    let currency = "usd";
    let coin_price = get_price(coin_name, currency).await;
    println!("      💰 {} price 👉 {}", coin_name, coin_price);
}
