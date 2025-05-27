use clap;

pub mod defaults {
    pub const DEFAULT_REPO_DIR: &str = "./.mimici/";
    pub const DEFAULT_GIT_EXEC: &str = "git";
}

pub mod options {

    use std::path::PathBuf;

    use clap;

    #[derive(clap::Args)]
    #[group(required = false, multiple = false)]
    pub struct GlobalArgs {
        /// Mimici dot file repository
        #[arg(
            short = 'C',
            value_name = "dir",
            default_value = super::defaults::DEFAULT_REPO_DIR,
        )]
        pub repo: PathBuf,

        /// Git executable to run git commands (only for `mimici git`)
        #[arg(
            long,
            value_name = "git",
            default_value = super::defaults::DEFAULT_GIT_EXEC,
        )]
        pub git: PathBuf,
    }

    #[derive(clap::Args)]
    pub struct InitArgs {
        /// Git remote for dot file repostiroy
        #[arg(value_name = "url")]
        pub remote: Option<String>,
    }

    #[derive(clap::Args)]
    pub struct GitArgs {
        /// Arguments to git command
        #[arg(value_name = "arg")]
        pub args: Vec<String>,
    }
}

/// mimici configuration manager
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[command(flatten)]
    pub options: options::GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

/// mimici commands
#[derive(clap::Subcommand)]
pub enum Commands {
    /// Execute git commands in the mimici config repo
    Git(options::GitArgs),
    /// Initialize mimici config repo
    Init(options::InitArgs),
}
