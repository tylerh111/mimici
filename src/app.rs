use crate::cli::options::*;
use crate::cli::{Arguments, Commands};

pub fn mimici_run_git(
    opts: &GlobalArgs,
    args: &GitArgs,
) {
    use crate::git::*;
    execute_git_command(&opts.git, &opts.repo, &args.args);
}

pub fn mimici_run_init(
    opts: &GlobalArgs,
    args: &InitArgs,
) {
    use crate::init::*;
    init_repo(&opts.repo, &args.remote);
}

pub fn run() {
    use clap::Parser;
    let args = Arguments::parse();
    match &args.command {
        Commands::Git(cargs) => mimici_run_git(&args.options, &cargs),
        Commands::Init(cargs) => mimici_run_init(&args.options, &cargs),
    }
}
