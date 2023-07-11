
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::io::{BufReader, Read, Write, Error};


// Verify the iso extension
fn is_iso(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
        _ => false,
    }
}

fn sha256_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}


fn sha256_digest_file<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}


fn main() -> Result<(), Error> {

    // Hash several files given a directory
    const DIRECTORY: &str = "/Users/m/src";

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for entry in WalkDir::new(DIRECTORY)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path())) {
            let path = entry.path().to_owned();
            println!("path: {:?}", path);
            let tx = tx.clone();
            pool.execute(move || {
                let digest = sha256_digest(path);
                println!("digest: {:?}", digest);
                tx.send(digest).expect("Could not send data!");
            });
        }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }

    // Hash a single file
    const PATH: &str = "main.rs";

    let mut output = File::create(PATH)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(PATH)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest_file(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));


    Ok(())

}
