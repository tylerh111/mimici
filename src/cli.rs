use clap::{Parser, Subcommand};

pub mod options {

    use std::path::PathBuf;

    use clap::Args;

    #[derive(Args)]
    #[group(required = false, multiple = false)]
    pub struct GlobalArgs {
        /// mimici config file for operation
        #[arg(short, long, value_name = "file")]
        pub config: Option<PathBuf>,

        /// mimici dot file repository
        #[arg(short, long, value_name = "dir")]
        pub repository: Option<PathBuf>,
    }

    #[derive(Args)]
    pub struct InitArgs {}
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
