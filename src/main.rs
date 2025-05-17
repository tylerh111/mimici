
mod cli;

use clap::Parser;

fn main() {
    println!("Hello, world!");
    let args = cli::Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
