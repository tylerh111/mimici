
use clap::Parser;

use crate::cli::{Arguments, Commands};
use crate::cli::options::*;

pub fn modal_run_init(_opts: &GlobalArgs, _args: &InitArgs) {
    println!("running init");
}


pub fn modal_run_activate(_opts: &GlobalArgs, _args: &ActivateArgs) {
    println!("running activate");
}


pub fn modal_run_deactivate(_opts: &GlobalArgs, _args: &DeactivateArgs) {
    println!("running deactivate");
}


pub fn modal_run_switch(_opts: &GlobalArgs, _args: &SwitchArgs) {
    println!("running switch");
}


pub fn modal_run_add(_opts: &GlobalArgs, _args: &AddArgs) {
    println!("running add");
}


pub fn modal_run_remove(_opts: &GlobalArgs, _args: &RemoveArgs) {
    println!("running remove");
}


pub fn modal_run_backup(_opts: &GlobalArgs, _args: &BackupArgs) {
    println!("running backup");
}


pub fn modal_run_diff(_opts: &GlobalArgs, _args: &DiffArgs) {
    println!("running diff");
}


pub fn modal_run_sync(_opts: &GlobalArgs, _args: &SyncArgs) {
    println!("running sync");
}


pub fn modal_run_git(_opts: &GlobalArgs, _args: &GitArgs) {
    println!("running git");
}


pub fn run() {
    println!("Hello, world!");
    let args = Arguments::parse();

    if let Some(config_path) = args.options.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match &args.command {
        Commands::Init(cargs) => { modal_run_init(&args.options, cargs) }
        Commands::Activate(cargs) => { modal_run_activate(&args.options, cargs) }
        Commands::Deactivate(cargs) => { modal_run_deactivate(&args.options, cargs) }
        Commands::Switch(cargs) => { modal_run_switch(&args.options, cargs) }
        Commands::Add(cargs) => { modal_run_add(&args.options, cargs) }
        Commands::Remove(cargs) => { modal_run_remove(&args.options, cargs) }
        Commands::Backup(cargs) => { modal_run_backup(&args.options, cargs) }
        Commands::Diff(cargs) => { modal_run_diff(&args.options, cargs) }
        Commands::Sync(cargs) => { modal_run_sync(&args.options, cargs) }
        Commands::Git(cargs) => { modal_run_git(&args.options, cargs) }
        // _ => { println!("not implemented") }
    }

}
