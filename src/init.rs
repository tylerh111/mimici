use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

use git2::Repository;
use serde_json;

use crate::error::MimiciError;
use crate::manifest::Manifest;


const TEMPLATE_README_CONTENTS: &str = r##"
# Dotfiles

This is a repository of config files (dotfiles) managed by [mimici](https://crates.io/crates/mimici).
"##;


pub fn init_readme(
    workdir: &PathBuf,
) -> Result<(), MimiciError> {
    let file = workdir.join("README.md");
    let readme = TEMPLATE_README_CONTENTS;

    std::fs::write(file, readme)?;

    Ok(())
}

pub fn init_manifest(
    workdir: &PathBuf,
) -> Result<(), MimiciError> {
    let file = workdir.join("manifest.json");
    let manifest = Manifest{
        profile: "default".to_string(),
        prefix: None,
        special: "home".to_string(),
        files: vec![],
    };

    std::fs::write(file, serde_json::to_string_pretty(&manifest)?)?;

    Ok(())
}

pub fn get_manifest(
    workdir: &PathBuf,
) -> Result<Manifest, MimiciError> {
    let path = workdir.join("manifest.json");
    let file = File::open(path)?;

    // checking if the manifest can be parsed by the `Manifest` struct
    let reader = BufReader::new(file);
    let manifest: Manifest = serde_json::from_reader(reader)?;
    Ok(manifest)
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
    let workdir = repo.workdir().ok_or(String::from("error: no workdir"))?.to_path_buf();

    let manifest = get_manifest(&workdir);
    if manifest.is_ok() {
        return Ok(());
    }

    init_manifest(&workdir)?;

    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    execute_git_command(
        &exec,
        &workdir,
        &vec![
            String::from("commit"),
            String::from("-m"),
            String::from("'mimici setup'")
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
    let workdir = &repo.workdir().ok_or(String::from("error: no workdir"))?.to_path_buf();

    remote.as_ref().inspect(|url| {
        let _ = repo.remote("origin", url.as_str());
    });

    init_readme(&workdir)?;
    init_manifest(&workdir)?;

    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    execute_git_command(
        &exec,
        &workdir,
        &vec![
            String::from("commit"),
            String::from("-m"),
            String::from("'initial commit'")
        ],
    );

    Ok(())
}
