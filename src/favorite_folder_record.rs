use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::trimmed_not_empty_text::{NonEmptyText, NotEmptyTextError};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
/// Struct to encapsulate a valid favorite created by the user
/// This favorite is a not empty label with an absolute path
/// That path does need to point to existing location.
pub struct FavoriteFolderPath {
    name: NonEmptyText,
    location: NonEmptyText,
}

impl FavoriteFolderPath {
    /// # Errors
    ///
    /// - If name or path are empty or white spaces.
    /// - If path can not be resolved to a valid utf 8 string.
    /// - If a relative path can not be resolved to an absolute one.
    pub fn new(name: NonEmptyText, location: NonEmptyText) -> Self {
        Self { name, location }
    }

    pub fn set_name(&mut self, new_name: NonEmptyText) {
        self.name = new_name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn path_str(&self) -> &str {
        &self.location
    }
    pub fn path(&self) -> &Path {
        Path::new(self.location.as_str())
    }
    pub fn does_exit(&self) -> bool {
        Path::new(self.location.as_str()).exists()
    }
}

/// Error to describe the cause of failing to create a struct [`FavoriteFolderPath`]
use thiserror::Error;
#[derive(Debug, Error)]
pub enum InvalidFavoriteFields {
    #[error("Name of favorite folder should not be empty or only whitespace")]
    EmptyName(#[from] NotEmptyTextError),
    #[error("Path of favorite folder should not be empty or only whitespace")]
    EmptyPath,
    #[error("Path of favorite folder is not a valid utf 8 text")]
    InvalidUtf8PathStr,
}
