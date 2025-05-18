use clap::Parser;

use crate::cli::options::*;
use crate::cli::{Arguments, Commands};


pub fn run() {
    println!("Hello, world!");
    let args = Arguments::parse();

    if let Some(config_path) = args.options.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match &args.command {
        _ => println!("Command not implemented")
    }
}
