use clap::Parser;

use crate::cli::options::*;
use crate::cli::{Arguments, Commands};
use crate::git::*;
use crate::init::*;

pub fn mimici_run_git(
    _opts: &GlobalArgs,
    _args: &GitArgs,
) {
    execute_git_command(&_opts.git, &_opts.repo, &_args.args);
}

pub fn mimici_run_init(
    _opts: &GlobalArgs,
    _args: &InitArgs,
) {
    init_repo(&_opts.repo, &_args.remote);
}

pub fn run() {
    let args = Arguments::parse();
    match &args.command {
        Commands::Git(cargs) => mimici_run_git(&args.options, &cargs),
        Commands::Init(cargs) => mimici_run_init(&args.options, &cargs),
    }
}
