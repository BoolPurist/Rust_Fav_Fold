use crate::favorite_folder_record::FavoriteFolderPath;
use crate::paths::{self, DataDirError};
use std::error::Error;
use std::fmt::Display;
use std::fs;
#[derive(Debug)]
pub enum DataIoError {
    Io(std::io::Error),
    DataDir(DataDirError),
    InvalidAppDataFormat(serde_json::Error),
}

impl Display for DataIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataIoError::Io(error) => writeln!(f, "Error in file operation: {}", error,),
            DataIoError::DataDir(error) => writeln!(f, "Error in getting data dir: {}", error),
            DataIoError::InvalidAppDataFormat(error) => {
                writeln!(f, "Error in parsing app data: {}", error)
            }
        }
    }
}

impl Error for DataIoError {}

pub fn get_favorites() -> Result<Vec<FavoriteFolderPath>, DataIoError> {
    let to_load_from = paths::get_path_to_data().map_err(|error| DataIoError::DataDir(error))?;

    if !to_load_from.exists() {
        return Ok(Vec::new());
    }

    let raw_content = fs::read_to_string(&to_load_from).map_err(|error| DataIoError::Io(error))?;
    let favorites = serde_json::from_str(&raw_content)
        .map_err(|error| DataIoError::InvalidAppDataFormat(error))?;

    Ok(favorites)
}

pub fn save_favorites(to_save: Vec<FavoriteFolderPath>) -> Result<(), DataIoError> {
    let to_save = serde_json::to_string(&to_save)
        .map_err(|error| DataIoError::InvalidAppDataFormat(error))?;
    let save_location = paths::get_path_to_data().map_err(|error| DataIoError::DataDir(error))?;

    fs::write(save_location, to_save).map_err(|error| DataIoError::Io(error))
}
