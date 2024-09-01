use log::info;

use crate::paths::{self, DataDirError};
use crate::{AllFavorites, AppResult};
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

pub fn delete_everything() -> AppResult {
    let to_load_from = paths::get_path_to_data().map_err(DataIoError::DataDir)?;
    std::fs::remove_file(to_load_from)?;
    info!("All favorites paths were deleted.");
    Ok(())
}

pub fn get_favorites() -> Result<AllFavorites, DataIoError> {
    let to_load_from = paths::get_path_to_data().map_err(DataIoError::DataDir)?;

    if !to_load_from.exists() {
        return Ok(AllFavorites::default());
    }

    let raw_content = fs::read_to_string(&to_load_from).map_err(DataIoError::Io)?;
    let favorites: AllFavorites =
        serde_json::from_str(&raw_content).map_err(DataIoError::InvalidAppDataFormat)?;

    Ok(favorites)
}

pub fn save_favorites(to_save: AllFavorites) -> Result<(), DataIoError> {
    let to_save = serde_json::to_string(&to_save).map_err(DataIoError::InvalidAppDataFormat)?;
    let save_location = paths::get_path_to_data().map_err(DataIoError::DataDir)?;

    fs::write(save_location, to_save).map_err(DataIoError::Io)
}
