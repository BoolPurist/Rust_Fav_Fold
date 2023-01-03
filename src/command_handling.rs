use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

use crate::favorite_folder_record::FavoriteFolderPath;
use crate::{file_data, AppResult};

use colored::*;

type Favorites = Vec<FavoriteFolderPath>;

pub fn get_fav(name: &str) -> AppResult<FavoriteFolderPath> {
    let records = file_data::get_favorites()?;

    let found = records
        .into_iter()
        .find(|nxt_fav| nxt_fav.get_name() == name)
        .ok_or_else(|| format!("No favortie path found with name {}", name))?;

    Ok(found)
}

pub fn rename_fav(name: &str, new_name: &str) -> AppResult {
    let mut records = file_data::get_favorites()?;

    let found = records
        .iter_mut()
        .find(|nxt_fav| nxt_fav.get_name() == name)
        .ok_or_else(|| {
            format!(
                "No favortie path found with name {} for renaming to new_name {}",
                name, new_name,
            )
        })?;

    found.set_name(new_name);

    file_data::save_favorites(records).map_err(Box::new)?;

    Ok(())
}

pub fn remove_from_fav(name: &str) -> AppResult {
    let mut records = file_data::get_favorites()?;
    match find_by_name(&records, name) {
        Some(to_delete) => {
            records.remove(to_delete);
            file_data::save_favorites(records)?;

            Ok(())
        }
        None => Err(format!("No favorite with name {} to be deleted", name))?,
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
            let name = next_record.get_name();
            assert!(
                name.len() <= max_width,
                "Could not find max width for all labels"
            );

            let padded_name = pad_from_right(name, max_width);
            let raw_path = next_record.get_path();

            let path_processed = get_folder_with_color(raw_path, for_clipboard);

            format!("{}  {}", padded_name, path_processed)
        })
        .collect::<Vec<String>>()
        .join("\n"));

    fn get_folder_with_color(path: &str, for_clipboard: bool) -> Cow<'_, str> {
        if for_clipboard {
            return path.into();
        }

        if PathBuf::from(&path).exists() {
            path.green().to_string().into()
        } else {
            path.red().to_string().into()
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

    file_data::save_favorites(records).map_err(Box::new)?;

    Ok(())
}

fn pad_from_right(to_pad: &str, max_width: usize) -> Cow<'_, str> {
    let actual_len = to_pad.len();

    if actual_len >= max_width {
        return to_pad.into();
    }

    let diff = max_width - actual_len;
    let padding = " ".repeat(diff);
    let mut padded = String::from(to_pad);
    padded.push_str(&padding);

    padded.into()
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
