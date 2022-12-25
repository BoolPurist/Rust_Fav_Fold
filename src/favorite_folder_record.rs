use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteFolderPath {
    name: String,
    path: String,
}

impl FavoriteFolderPath {
    pub fn new(name: &str, path: &PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path: path
                .to_str()
                .expect("Path can not be converted to string ")
                .to_string(),
        }
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn does_exit(&self) -> bool {
        Path::new(&self.path).exists()
    }
}
