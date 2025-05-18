
mod cli;

use clap::Parser;

fn main() {
    println!("Hello, world!");
    let args = cli::Args::parse();

}
