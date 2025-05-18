use clap::{Parser, Subcommand};

pub mod options {

    use std::path::PathBuf;

    use clap::Args;

    #[derive(Args)]
    #[group(required = false, multiple = false)]
    pub struct GlobalArgs {
        /// Modal config file for operation
        #[arg(short, long, value_name = "file")]
        pub config: Option<PathBuf>,

        /// Modal dot file repository
        #[arg(short, long, value_name = "dir")]
        pub repository: Option<PathBuf>,
    }

    #[derive(Args)]
    pub struct InitArgs {}
}

/// Modal configuration manager
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
}
