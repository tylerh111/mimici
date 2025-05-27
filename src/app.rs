use crate::error::MimiciResult;
use crate::cli::options::*;
use crate::cli::{Arguments, Commands};

pub fn mimici_run_git(
    opts: &GlobalArgs,
    args: &GitArgs,
) -> MimiciResult {
    use crate::git::*;
    execute_git_command(&opts.git, &opts.repo, &args.args);
    Ok(())
}

pub fn mimici_run_init(
    opts: &GlobalArgs,
    args: &InitArgs,
) -> MimiciResult {
    use crate::init::*;
    if args.clone {
        let remote = args.remote.as_ref().ok_or(String::from("require remote"))?;
        init_repo_clone(&opts.git, &opts.repo, &remote)
    } else {
        init_repo(&opts.git, &opts.repo, &args.remote)
    }
}

pub fn run() -> MimiciResult {
    use clap::Parser;
    let args = Arguments::parse();
    match &args.command {
        Commands::Git(cargs) => mimici_run_git(&args.options, &cargs),
        Commands::Init(cargs) => mimici_run_init(&args.options, &cargs),
    }
}
