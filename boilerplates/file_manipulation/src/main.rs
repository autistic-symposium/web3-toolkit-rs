use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};


fn main() -> Result<(), Error> {
    const FILEPATH: &str = "result.txt";

    // write to file
    let mut output = File::create(FILEPATH)?;
    write!(output, "Cyph3r\n👾")?;

    // read from file
    let input = File::open(FILEPATH)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
