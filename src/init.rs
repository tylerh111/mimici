use std::path::PathBuf;

use git2::Repository;
use serde_json;

use crate::error::MimiciError;


const TEMPLATE_README_CONTENTS: &str = r##"
# Dotfiles

This is a repository of config files (dotfiles) managed by [mimici](https://crates.io/crates/mimici).
"##;


pub fn init_readme(
    repo: &Repository,
) -> Result<(), MimiciError> {
    let workdir = repo.workdir().unwrap();
    let file = workdir.join("README.md");

    if file.exists() {
        println!("skipping initialization of 'README.md': file already exists");
    }

    let readme = TEMPLATE_README_CONTENTS;

    std::fs::write(file, readme)?;

    Ok(())
}

pub fn init_manifest(
    repo: &Repository,
) -> Result<(), MimiciError> {
    use crate::manifest::Manifest;

    let workdir = repo.workdir().unwrap();
    let file = workdir.join("manifest.json");

    if file.exists() {
        println!("skipping initialization of 'manifest.json': file already exists");
    }

    let manifest = Manifest{
        profile: "default".to_string(),
        prefix: None,
        special: "home".to_string(),
        files: vec![],
    };

    std::fs::write(file, serde_json::to_string(&manifest)?)?;

    Ok(())
}

pub fn init_repo_clone(
    exec: &PathBuf,
    path: &PathBuf,
    remote: &String,
) -> Result<(), MimiciError> {
    use crate::git::execute_git_command;

    if let Ok(_) = Repository::open(path) {
        println!("mimici: repository already exists: {}", path.display());
        return Ok(());
    }

    // clone repository
    let repo  = Repository::clone(remote, path)?;

    init_readme(&repo)?;
    init_manifest(&repo)?;

    execute_git_command(
        &exec,
        &repo.workdir().unwrap().to_path_buf(),
        &vec![
            String::from("add"),
            String::from("*"),
        ],
    );

    Ok(())
}

pub fn init_repo(
    exec: &PathBuf,
    path: &PathBuf,
    remote: &Option<String>,
) -> Result<(), MimiciError> {
    use crate::git::execute_git_command;

    if let Ok(_) = Repository::open(path) {
        println!("mimici: repository already exists: {}", path.display());
        return Ok(());
    }

    // initialize repository and set the remote to
    // added initial files for mimici operations
    let repo = Repository::init(path)?;
    remote.as_ref().inspect(|url| {
        let _ = repo.remote("origin", url.as_str());
    });

    init_readme(&repo)?;
    init_manifest(&repo)?;

    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    execute_git_command(
        &exec,
        &repo.workdir().unwrap().to_path_buf(),
        &vec![
            String::from("add"),
            String::from("*"),
        ],
    );

    Ok(())
}
