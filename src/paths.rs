use crate::prelude::*;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const fn is_in_debug() -> bool {
    if cfg!(debug_assertions) {
        true
    } else {
        false
    }
}

#[derive(Debug, Error)]
pub enum DataFolderError {
    #[error("Could not find a location where a data folder is located in general")]
    CouldNotLocate,
    #[error("Could not make sure there is a data folder at ({0:?}) for this application")]
    CouldNotEnsure(PathBuf),
}

pub fn get_path_to_data() -> Result<PathBuf, DataFolderError> {
    let mut data_dir = get_data_dir()?;
    data_dir.push(constants::APP_DATA_FILE);
    debug!("Path to application data {:?}", data_dir);
    Ok(data_dir)
}

fn locate_data_folder_of_user() -> Result<PathBuf, DataFolderError> {
    dirs::data_dir().ok_or(DataFolderError::CouldNotLocate)
}

fn get_tmp_folder_root() -> PathBuf {
    std::env::temp_dir().join(Path::new(constants::TMP_PREFIX))
}

fn get_data_dir() -> Result<PathBuf, DataFolderError> {
    let data_folder = if is_in_debug() {
        get_tmp_folder_root()
    } else {
        locate_data_folder_of_user()?
    };
    debug!("Located data folder at {:?}", &data_folder);

    let data_folder_for_this_app = data_folder.join(constants::APP_NAME);
    debug!("Ensuring data folder at {:?}", &data_folder_for_this_app);
    std::fs::create_dir_all(&data_folder_for_this_app)
        .map_err(|_| DataFolderError::CouldNotEnsure(data_folder_for_this_app.clone()))?;
    Ok(data_folder_for_this_app)
}
