use clap::{Parser, Subcommand};

pub mod options {

    use std::path::PathBuf;

    use clap::Args;

    #[derive(Args)]
    #[group(required = false, multiple = false)]
    pub struct GlobalArgs {
        /// mimici dot file repository
        #[arg(short = 'C', value_name = "dir", default_value = "./.mimici/")]
        pub repo: PathBuf,

        /// git executable to run git commands (only for `mimici git`)
        #[arg(long, value_name = "git", default_value = "git")]
        pub git: PathBuf,
    }

    #[derive(Args)]
    pub struct InitArgs {
        /// git remote for dot file repostiroy
        #[arg(value_name = "url")]
        pub remote: Option<String>,
    }

    #[derive(Args)]
    pub struct GitArgs {
        /// arguments to git command
        #[arg(value_name = "arg")]
        pub args: Vec<String>,
    }
}

/// mimici configuration manager
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[command(flatten)]
    pub options: options::GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

/// mimici commands
#[derive(Subcommand)]
pub enum Commands {
    /// Execute git commands in the mimici config repo
    Git(options::GitArgs),
    /// Initialize mimici config repo
    Init(options::InitArgs),
}
