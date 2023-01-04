use crate::app::term_colors;
use crate::cli_args::GetParams;
use crate::favorite_folder_record::FavoriteFolderPath;
use crate::{file_access, AppResult};
use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

type Favorites = Vec<FavoriteFolderPath>;

/// Size of space between columns in a output.
/// Example: `Spacing` is 2 then between name and path is 2 whitespaces in the output for the
/// user.
const SPACING: usize = 2;

fn get_spacing_padding() -> String {
    " ".repeat(SPACING)
}

pub fn get_fav(name: &str) -> AppResult<(Option<usize>, Vec<FavoriteFolderPath>)> {
    let records = file_access::get_favorites()?;

    let found = records
        .iter()
        .position(|nxt_fav| nxt_fav.get_name() == name);

    Ok((found, records))
}

pub fn rename_fav(name: &str, new_name: &str) -> AppResult {
    let mut records = file_access::get_favorites()?;

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

    file_access::save_favorites(records).map_err(Box::new)?;

    Ok(())
}

pub fn remove_from_fav(name: &str) -> AppResult {
    let mut records = file_access::get_favorites()?;
    match find_by_name(&records, name) {
        Some(to_delete) => {
            records.remove(to_delete);
            file_access::save_favorites(records)?;

            Ok(())
        }
        None => Err(format!("No favorite with name {} to be deleted", name))?,
    }
}

pub fn get_all_fav_table(
    all_locations: &[FavoriteFolderPath],
    get_params: &GetParams,
) -> AppResult<String> {
    let spacing_padding = get_spacing_padding();
    let max_width = all_locations
        .iter()
        .fold(0, |akk, next| akk.max(next.get_name().len()));

    let records_without_line_numbers: Vec<String> = all_locations
        .iter()
        .map(|next_record| {
            let name = next_record.get_name();
            assert!(
                name.len() <= max_width,
                "Could not find max width for all labels"
            );

            let padded_name = pad_from_right_until(name, max_width);
            let raw_path = next_record.get_path();

            let path_processed =
                get_colored_path_if_no_clipboard(raw_path, get_params.copy_has_clipboard());

            format!("{padded_name}{spacing_padding}{path_processed}")
        })
        .collect();
    let records =
        handle_line_number_if_needed(&records_without_line_numbers, get_params, &spacing_padding);
    return Ok(records.join("\n"));

    fn handle_line_number_if_needed<'a>(
        records_without_line_numbers: &'a Vec<String>,
        get_params: &GetParams,
        spacing_padding: &str,
    ) -> Cow<'a, Vec<String>> {
        if !get_params.copy_ask_number() {
            Cow::Borrowed(records_without_line_numbers)
        } else {
            let last_line_number = records_without_line_numbers.len();
            let max_width_line_number = last_line_number.to_string().len();

            let line_numbers_padded = (1..=last_line_number).map(|next_line_number| {
                let to_pad = next_line_number.to_string();
                assert!(
                    to_pad.len() <= max_width_line_number,
                    "Could not find max width for all labels"
                );

                let padded_line_number = pad_from_right_until(&to_pad, max_width_line_number);

                format!("{padded_line_number}{spacing_padding}")
            });

            Cow::Owned(
                records_without_line_numbers
                    .iter()
                    .zip(line_numbers_padded)
                    .map(|to_merge| {
                        let (right, left) = to_merge;
                        format!("{}{}", left, right)
                    })
                    .collect::<Vec<String>>(),
            )
        }
    }

    fn get_colored_path_if_no_clipboard(path: &str, for_clipboard: bool) -> Cow<'_, str> {
        if for_clipboard {
            return path.into();
        }

        if PathBuf::from(path).exists() {
            term_colors::color_exists_msg(path).into()
        } else {
            term_colors::color_not_found(path).into()
        }
    }
}

pub fn set_favorite_data(name: &str, path: &str) -> AppResult {
    let mut records = file_access::get_favorites()?;

    let new_path = FavoriteFolderPath::new(name, &PathBuf::from(path))?;
    match find_by_name(&records, name) {
        Some(index) => {
            records[index] = new_path;
        }
        None => records.push(new_path),
    };

    file_access::save_favorites(records).map_err(Box::new)?;

    Ok(())
}

pub fn set_label_to_cwd(name: &str) -> AppResult {
    let cwd = env::current_dir()?;
    let cwd_str = cwd
        .to_str()
        .ok_or("Could not get working directory as new path value")?;
    set_favorite_data(name, cwd_str)?;
    Ok(())
}
/// # Summary
///
/// Makes sure the return value is at least as long as `max_with` chars.
///
/// # Returns
///
/// Either returns the `to_pad` unchanged as ref or as new owned value with spaces appended from right
/// so the string is exactly `max_with` chars long.
fn pad_from_right_until(to_pad: &str, max_width: usize) -> Cow<'_, str> {
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
