use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]

/// Struct to encapsulate a valid favorite created by the user
/// This favorite is a not empty label with an absolute path
/// That path does need to point to existing location.
pub struct FavoriteFolderPath {
    name: String,
    path: String,
}

impl FavoriteFolderPath {
    /// # Errors
    ///
    /// - If name or path are empty or white spaces.
    /// - If path can not be resolved to a valid utf 8 string.
    /// - If a relative path can not be resolved to an absolute one.
    pub fn new(name: &str, path: &Path) -> Result<Self, InvalidFavoriteFields> {
        let trimmed_name = name.trim().to_string();
        if trimmed_name.is_empty() {
            return Err(InvalidFavoriteFields::EmptyName);
        }

        let utf_from_path = path
            .to_str()
            .ok_or(InvalidFavoriteFields::InvalidUtf8PathStr)?;
        let trimmed_path = utf_from_path.trim().to_string();

        if trimmed_path.is_empty() {
            return Err(InvalidFavoriteFields::EmptyPath);
        }

        Ok(Self {
            name: trimmed_name,
            path: trimmed_path,
        })
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn path_str(&self) -> &str {
        &self.path
    }
    pub fn path(&self) -> &Path {
        &Path::new(&self.path)
    }
    pub fn does_exit(&self) -> bool {
        Path::new(&self.path).exists()
    }
}

/// Error to describe the cause of failing to create a struct [`FavoriteFolderPath`]
#[derive(Debug)]
pub enum InvalidFavoriteFields {
    EmptyName,
    EmptyPath,
    InvalidUtf8PathStr,
}

impl Display for InvalidFavoriteFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidFavoriteFields::EmptyName => write!(
                f,
                "Name of favorite folder should not be empty or only whitespace"
            ),
            InvalidFavoriteFields::EmptyPath => write!(
                f,
                "Path of favorite folder should not be empty or only whitespace"
            ),
            InvalidFavoriteFields::InvalidUtf8PathStr => {
                write!(f, "Path of favorite folder is not a valid utf 8 text")
            }
        }
    }
}

impl Error for InvalidFavoriteFields {}
