use clap::Parser;

use crate::cli::options::*;
use crate::cli::{Arguments, Commands};
use crate::git::*;
use crate::init::*;

pub fn mimici_run_git(
    opts: &GlobalArgs,
    args: &GitArgs,
) {
    execute_git_command(&opts.git, &opts.repo, &args.args);
}

pub fn mimici_run_init(
    opts: &GlobalArgs,
    args: &InitArgs,
) {
    init_repo(&opts.repo, &args.remote);
}

pub fn run() {
    let args = Arguments::parse();
    match &args.command {
        Commands::Git(cargs) => mimici_run_git(&args.options, &cargs),
        Commands::Init(cargs) => mimici_run_init(&args.options, &cargs),
    }
}
