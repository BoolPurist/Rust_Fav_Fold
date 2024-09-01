use log::error;

use crate::favorite_folder_record::FavoriteFolderPath;
use crate::{cli_args::GetParams, favorite_table};
use std::error::Error;

use crate::{clipboard, file_access, AppResult};

pub mod console_interaction;
pub mod term_colors;

pub fn put_into_clipboard_or_print(content: &str, clipboard: bool) -> AppResult {
    if clipboard {
        clipboard::put_into_clipboard(content)?;
    } else {
        println!("{content}");
    }

    Ok(())
}

pub fn exit_with_error(message: &dyn Error) {
    let red_msg = term_colors::color_error_msg(&format!("Error: {message}"));
    error!("{}", red_msg);
    std::process::exit(1);
}

pub fn handle_get_subcommand(get_params: &GetParams) -> AppResult<String> {
    return match get_params.get_name() {
        Some(name_given) => {
            let favorites = file_access::get_favorites()?;
            let name = name_given.try_into()?;
            let content = match favorites.get(name) {
                Some(found) => found.path_str(),
                None => {
                    return match (get_params.copy_fuzzy(), get_params.copy_ask_number()) {
                        (false, false) => {
                            Err(format!("No path found for the name: {}", name_given).into())
                        }
                        (_, _) => get_all(&get_params),
                    };
                }
            };

            Ok(content.to_string())
        }
        None => get_all(&get_params),
    };

    fn get_all(get_params: &GetParams) -> AppResult<String> {
        let all_locations = file_access::get_favorites()?;

        let all_locations = if let Some(name) = get_params.get_name() {
            if get_params.copy_fuzzy() {
                let name = name.try_into()?;
                all_locations.filtered_containing_name(name)
            } else {
                all_locations
            }
        } else {
            all_locations
        };

        let content = draw_table_and_prompt(all_locations.as_slice(), get_params)?;

        Ok(content)
    }
}

fn draw_table_and_prompt(
    all_locations: &[FavoriteFolderPath],
    get_params: &GetParams,
) -> AppResult<String> {
    if all_locations.is_empty() {
        return Err(
            "No match found for given name or no labels were created so far"
                .to_string()
                .into(),
        );
    }

    let table = favorite_table::draw_favorite_table(all_locations, get_params.into());
    if get_params.copy_ask_number() {
        println!("{table}");

        let given_number =
            console_interaction::ask_possible_prompt_for_ask_number(all_locations, get_params)?;

        match given_number {
            Some(index_start_from_one) => {
                let index = index_start_from_one - 1;
                // function for asking number of user ensures that the index will not
                // be out of bounds
                let to_put = all_locations.get(index).unwrap().path_str().to_string();

                Ok(to_put)
            }
            None => Ok(table),
        }
    } else {
        Ok(table)
    }
}
