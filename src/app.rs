use clap::Parser;

use crate::cli::{Arguments, Commands};
use crate::cli::options::*;

pub fn mimici_run_init(_opts: &GlobalArgs, _args: &InitArgs) {
    println!("mimici init");
}

pub fn run() {
    let args = Arguments::parse();
    match &args.command {
        Commands::Init(cargs) => mimici_run_init(&args.options, &cargs),
    }
}
