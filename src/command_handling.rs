use crate::favorite_folder_record::FavoriteFolderPath;
use crate::{file_data, AppResult};
use colored::*;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;
type Favorites = Vec<FavoriteFolderPath>;

#[derive(Debug)]
struct AppArgError(String);

impl Display for AppArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl Error for AppArgError {}

pub fn get_fav(name: &str) -> AppResult<FavoriteFolderPath> {
    let records = file_data::get_favorites()?;

    let found = records
        .into_iter()
        .find(|nxt_fav| nxt_fav.get_name() == name)
        .ok_or(AppArgError(format!(
            "No favortie path found with name {}",
            name.to_string()
        )))?;

    Ok(found)
}

pub fn rename_fav(name: &str, new_name: &str) -> AppResult {
    let mut records = file_data::get_favorites()?;

    let found = records
        .iter_mut()
        .find(|nxt_fav| nxt_fav.get_name() == name)
        .ok_or(AppArgError(format!(
            "No favortie path found with name {} for renaming to new_name {}",
            name.to_string(),
            new_name.to_string(),
        )))?;

    found.set_name(new_name);

    let _ = file_data::save_favorites(records).map_err(|error| Box::new(error))?;

    Ok(())
}

pub fn remove_from_fav(name: &str) -> AppResult {
    let mut records = file_data::get_favorites()?;
    match find_by_name(&records, name) {
        Some(to_delete) => {
            records.remove(to_delete);
            file_data::save_favorites(records)?;
            return Ok(());
        }
        None => Err(AppArgError(format!(
            "No favorite with name {} to be deleted",
            name
        )))?,
    }
}
pub fn get_all_fav_table(for_clipboard: bool) -> AppResult<String> {
    let records = file_data::get_favorites()?;

    let max_width = records
        .iter()
        .fold(0usize, |akk_max, next| akk_max.max(next.get_name().len()));

    return Ok(records
        .into_iter()
        .map(|next_record| {
            let padded_name = pad_from_right(next_record.get_name(), max_width);
            let raw_path = next_record.get_path();

            let path_processed = if for_clipboard {
                raw_path.to_string()
            } else {
                get_folder_with_color(&raw_path)
            };

            format!("{}  {}", padded_name, path_processed)
        })
        .collect::<Vec<String>>()
        .join("\n"));

    fn get_folder_with_color(path: &str) -> String {
        if PathBuf::from(&path).exists() {
            path.green().to_string()
        } else {
            path.red().to_string()
        }
    }
}

pub fn set_favorite_data(name: &str, path: &str) -> AppResult {
    let mut records = file_data::get_favorites()?;

    let new_path = FavoriteFolderPath::new(name, &PathBuf::from(path))?;
    match find_by_name(&records, name) {
        Some(index) => {
            records[index] = new_path;
        }
        None => records.push(new_path),
    };

    let _ = file_data::save_favorites(records).map_err(|error| Box::new(error))?;

    Ok(())
}

fn pad_from_right(to_pad: &str, max_width: usize) -> String {
    let actual_len = to_pad.len();

    assert!(actual_len <= max_width);

    let diff = max_width - actual_len;
    let padding = " ".repeat(diff);
    let mut padded = String::from(to_pad);
    padded.push_str(&padding);

    padded
}

fn find_by_name(records: &Favorites, name: &str) -> Option<usize> {
    records.iter().position(|fav| fav.get_name() == name)
}

pub fn set_label_to_cwd(name: &str) -> AppResult {
    let cwd = env::current_dir()?;
    let cwd_str = cwd
        .to_str()
        .ok_or("Could not get working directory as new path value")?;
    set_favorite_data(name, cwd_str)?;
    Ok(())
}
