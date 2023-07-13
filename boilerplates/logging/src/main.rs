// run with env RUST_LOG=info cargo run --bin logging

use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
