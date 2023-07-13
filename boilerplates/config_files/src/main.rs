#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

fn main() -> Result<(), io::Error> {
    let cfg: MyConfig = confy::load("my_app")?;
    println!("{:#?}", cfg);
    Ok(())
}
