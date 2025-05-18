
use clap::{Parser, Subcommand};


pub mod options {

use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct GlobalArgs {
    /// Modal config file for operation
    #[arg(
        short,
        long,
        value_name = "file",
    )]
    pub config: Option<PathBuf>,

    /// Modal dot file repository
    #[arg(
        short,
        long,
        value_name = "dir",
    )]
    pub repository: Option<PathBuf>,
}


#[derive(Args)]
pub struct InitArgs {

}


#[derive(Args)]
pub struct ActivateArgs {

}


#[derive(Args)]
pub struct DeactivateArgs {

}


#[derive(Args)]
pub struct SwitchArgs {

}


#[derive(Args)]
pub struct AddArgs {

}


#[derive(Args)]
pub struct RemoveArgs {

}


#[derive(Args)]
pub struct BackupArgs {

}


#[derive(Args)]
pub struct DiffArgs {

}


#[derive(Args)]
pub struct SyncArgs {

}


#[derive(Args)]
pub struct GitArgs {

}

}


/// Modal argument parser
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {

    #[command(flatten)]
    pub options: options::GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}


/// Modal commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize modal config repo
    Init(options::InitArgs),
    /// Activate modal config profile
    Activate(options::ActivateArgs),
    /// Deactivate modal config profile (resets to default)
    Deactivate(options::DeactivateArgs),
    /// Switch modal config profiles
    Switch(options::SwitchArgs),
    /// Add file to modal config repo
    Add(options::AddArgs),
    /// Remove file from modal config repo
    Remove(options::RemoveArgs),
    /// Show differences between backup files and installed files
    Diff(options::DiffArgs),
    /// Backup tracked files from installed files to modal config repo
    Backup(options::BackupArgs),
    /// Sync tracked files from modal config repo to installed files
    Sync(options::SyncArgs),
    /// Use git commands to manage the modal config repo
    Git(options::GitArgs),
}
