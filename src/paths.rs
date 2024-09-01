use log::debug;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

///
const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Error)]
pub enum DataFolderError {
    #[error("Could not find a location where a data folder is located in general")]
    CouldNotLocate,
    #[error("Could not make sure there is a data folder at ({0:?}) for this application")]
    CouldNotEnsure(PathBuf),
}

pub fn get_path_to_data() -> Result<PathBuf, DataFolderError> {
    let mut data_dir = get_data_dir()?;
    data_dir.push("favorites.json");
    debug!("Path to application data {:?}", data_dir);
    Ok(data_dir)
}

fn get_data_dir() -> Result<PathBuf, DataFolderError> {
    let data_folder = dirs::data_dir().ok_or(DataFolderError::CouldNotLocate)?;
    debug!("Located data folder at {:?}", &data_folder);
    let data_folder_for_this_app = data_folder.join(APP_NAME);
    debug!("Ensuring data folder at {:?}", &data_folder_for_this_app);
    fs::create_dir_all(&data_folder_for_this_app)
        .map_err(|_| DataFolderError::CouldNotEnsure(data_folder_for_this_app.clone()))?;
    Ok(data_folder_for_this_app)
}
