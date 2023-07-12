use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    
    const DEFAULT_CONFIG: &str = ".env";
    let config_path = env::var("CONFIG")
        .unwrap_or(DEFAULT_CONFIG.to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}
