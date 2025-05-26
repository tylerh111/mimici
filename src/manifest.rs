
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct File {
    app: Option<String>,
    path: String,
    install: PathBuf,
    permissions: String,
}

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    profile: String,
    prefix: Option<PathBuf>,
    special: String,
    files: Vec<File>,
}

