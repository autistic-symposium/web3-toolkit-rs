pub mod maths;
pub mod commands;
pub mod cli_handler;

pub use commands::{CliEnum, CliStruct};
pub use cli_handler::{handle_http, handle_ws};
