use data_encoding::{HEXUPPER, DecodeError};

fn main() -> Result<(), DecodeError> {
    let original = b"Stoic cypherpunks in the future.";
    println!("Original in bytes: {:?}", &original);

    let encoded = HEXUPPER.encode(original);
    println!("Encoded: {encoded}");

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    println!("Decoded in bytes: {:?}", &decoded);
    println!("Decoded as string: {}", String::from_utf8_lossy(&decoded));

    Ok(())
}
