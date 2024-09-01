use crate::all_favorites::AfterInsertion;

use crate::favorite_folder_record::FavoriteFolderPath;
use crate::trimmed_not_empty_text::NonEmptyText;
use crate::{file_access, AppResult};
use log::info;
use std::env;
use std::path::Path;

pub fn reset() -> AppResult {
    file_access::delete_everything()
}

pub fn rename_fav(name: &NonEmptyText, new_name: NonEmptyText) -> AppResult {
    let mut favorites = file_access::get_favorites()?;

    favorites.rename(name, new_name);

    file_access::save_favorites(favorites).map_err(Box::new)?;

    Ok(())
}

pub fn remove_from_fav(name: &NonEmptyText) -> AppResult {
    let mut favorites = file_access::get_favorites()?;
    if favorites.remove_with_name(name) {
        file_access::save_favorites(favorites)?;
        Ok(())
    } else {
        Err(format!("No favorite with name {} to be deleted", name).into())
    }
}

pub fn remove_all_non_existing() -> AppResult {
    let mut records = file_access::get_favorites()?;
    records.clean_all_dangling(matches_on_all_non_existing_paths);
    file_access::save_favorites(records)?;
    Ok(())
}

pub fn set_favorite_data(name: NonEmptyText, path: NonEmptyText) -> AppResult {
    let mut records = file_access::get_favorites()?;

    info!(
        "About to use add or change name {} with path {}",
        name, path
    );
    let new_favorite = FavoriteFolderPath::new(name, path);
    match records.insert(new_favorite) {
        AfterInsertion::Changed => info!("Changed: A new path was set for the name",),
        AfterInsertion::Added => {
            info!("Added: New path was added")
        }
    }

    file_access::save_favorites(records)?;

    Ok(())
}

pub fn set_label_to_cwd(name: NonEmptyText) -> AppResult {
    let cwd = env::current_dir()?;
    let cwd_str = cwd
        .to_str()
        .ok_or("Could not get working directory as new path value")?
        .try_into()?;

    set_favorite_data(name, cwd_str)?;
    Ok(())
}

fn matches_on_all_non_existing_paths(path: &Path) -> bool {
    match path.try_exists() {
        Ok(true) => false,
        Ok(false) => {
            log::info!("Removing dangling path at {:?}.", path);
            true
        }
        Err(error) => {
            log::warn!(
                "Could not determined if path {:?} exits due to IO error.\
            \n Details: {}",
                path,
                error
            );
            false
        }
    }
}
