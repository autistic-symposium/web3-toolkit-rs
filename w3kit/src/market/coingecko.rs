// market/coingecko.rs
// author: steinkirch

use std::env;
use serde_json;


////////////////////////////
// Private functions
////////////////////////////

async fn get_request(url: &str) -> serde_json::Value {

    println!("✅ GET {}", url);
    let response = reqwest::get(url)
                        .await.unwrap()
                        .json::<serde_json::Value>()
                        .await.unwrap();

    if response["error"].is_string() {
        println!("⛔️ {}", response["error"]);
        std::process::exit(1);
    }
    
    return response

}


#[allow(dead_code)]
async fn coin_price(coin: &str, currency: &str) {

    println!("✅ fetching price for {} in {}", coin, currency);

    let coingecko_url = &env::var("COINGECKO_API_URL").expect("⛔️ No COINGECKO_API_URL on .env file");
    let url = format!("{}simple/price?ids={}&vs_currencies={}", coingecko_url, coin, currency);

    let coin_price = get_request(&url).await;
    println!("      🪙 {} price 👉 {}", coin, coin_price);
}


async fn coin_marketcap(coin: &str, currency: &str) {

    println!("✅ fetching marketcap for {}", coin);

    let coingecko_url = &env::var("COINGECKO_API_URL").expect("⛔️ No COINGECKO_API_URL on .env file");
    let url = format!("{}simple/price?ids={}&vs_currencies={}&include_market_cap=true", coingecko_url, coin, currency);

    let coin_marketcap = get_request(&url).await;

    println!("      🪙 price     👉 ${}", coin_marketcap[coin][currency]);
    println!("      📊 marketcap 👉 {}", coin_marketcap[coin]["usd_market_cap"]);
}


////////////////////////////
// Public functions
////////////////////////////


pub async fn get_coin_info(coin_name: &str) {

    // TODO: add support for multiple currencies
    let currency = "usd";

    coin_marketcap(coin_name, currency).await;

}

