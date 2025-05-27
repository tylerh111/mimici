
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct File {
    pub path: PathBuf,
    pub install: PathBuf,
    pub app: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub profile: String,
    pub prefix: Option<PathBuf>,
    pub special: String,
    pub files: Vec<File>,
}

