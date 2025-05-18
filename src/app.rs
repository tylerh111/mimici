use clap::Parser;

use crate::cli::Arguments;

pub fn run() {
    let args = Arguments::parse();
    match &args.command {
        _ => println!("Command not implemented"),
    }
}
