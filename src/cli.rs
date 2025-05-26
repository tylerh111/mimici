use clap::{Parser, Subcommand};

pub mod options {

    use std::path::PathBuf;

    use clap::Args;

    #[derive(Args)]
    #[group(required = false, multiple = false)]
    pub struct GlobalArgs {
        /// mimici dot file repository
        #[arg(short, long, value_name = "dir", default_value = "./.mimici/")]
        pub repository: PathBuf,
    }

    #[derive(Args)]
    pub struct InitArgs {
        /// git remote for dot file repostiroy
        #[arg(value_name = "url")]
        pub remote: Option<String>,
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
    /// Initialize mimici config repo
    Init(options::InitArgs),
}
