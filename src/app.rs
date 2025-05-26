use clap::Parser;

use crate::cli::options::*;
use crate::cli::{Arguments, Commands};
use crate::init::*;

pub fn mimici_run_init(_opts: &GlobalArgs, _args: &InitArgs) {
    init_repo(&_opts.repository, &_args.remote);
}

pub fn run() {
    let args = Arguments::parse();
    match &args.command {
        Commands::Init(cargs) => mimici_run_init(&args.options, &cargs),
    }
}
