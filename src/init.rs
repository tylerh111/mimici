use std::path::PathBuf;

use git2::Repository;

pub fn init_repo(
    path: &PathBuf,
    remote: &Option<String>,
) {
    if let Ok(_) = Repository::open(path) {
        println!("mimici: repository already exists: {}", path.display());
        return;
    }

    // initialize repository and set the remote to
    if let Ok(repo) = Repository::init(path) {
        remote.as_ref().inspect(|url| {
            let _ = repo.remote("origin", url.as_str());
        });
    }
}
