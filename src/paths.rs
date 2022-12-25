use directories::ProjectDirs;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
#[derive(Debug)]
pub struct DataDirError(String);

impl Error for DataDirError {}
impl Display for DataDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

pub fn get_path_to_data() -> Result<PathBuf, DataDirError> {
    let mut data_dir = get_data_dir()?;
    data_dir.push("favorites.json");

    Ok(data_dir)
}
fn get_data_dir() -> Result<PathBuf, DataDirError> {
    let project_path = ProjectDirs::from("", "", "folder favorite").ok_or(DataDirError(
        "Could not get project folder for user".to_string(),
    ))?;

    let data_folder = project_path.data_dir();

    if !data_folder.exists() {
        fs::create_dir(&data_folder).map_err(|error| {
            DataDirError(format!(
                "Data folder doesn not exits and could not be created at {}.\n Inner error; {}",
                data_folder
                    .to_str()
                    .expect("Could not convert data folder to utf 8 string"),
                error
            ))
        })?;
    }

    Ok(PathBuf::from(data_folder))
}
