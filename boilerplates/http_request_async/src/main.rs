// handles all the heavy executor setup and allows sequential 
// code implemented without blocking until .await

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[tokio::main]
async fn main() -> Result<()> {

    const URL: &str = "http://google.com";
    let res = reqwest::get(URL).await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
