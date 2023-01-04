use crate::cli_args::GetParams;
use crate::favorite_folder_record::FavoriteFolderPath;
use std::error::Error;

use crate::{clipboard, data_access, file_access, AppResult};

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
    eprint!("{red_msg}");
    std::process::exit(1);
}
pub fn handle_get_subcommand(get_params: &GetParams) -> AppResult {
    let clipboard = get_params.copy_has_clipboard();
    return match get_params.get_name() {
        Some(name_given) => {
            let (may_found, read_records) = data_access::get_fav(name_given)?;

            match may_found {
                Some(index) => {
                    let found = read_records.get(index)
                    .expect(
                        "Unexpected error: get_fav function returned an index to found location but out of bound index occured",
                    );
                    put_into_clipboard_or_print(found.get_path(), clipboard)?;
                }
                None => {
                    return match (get_params.copy_fuzzy(), get_params.copy_ask_number()) {
                        (false, false) => {
                            Err(format!("No path found for name: {}", name_given).into())
                        }
                        (_, _) => get_all(get_params),
                    };
                }
            }

            Ok(())
        }
        None => get_all(get_params),
    };

    fn get_all(get_params: &GetParams) -> AppResult {
        let mut all_locations = file_access::get_favorites()?;

        if let Some(name) = get_params.get_name() {
            if get_params.copy_fuzzy() {
                all_locations.retain(|possible_match| possible_match.get_name().contains(name));
            }
        }

        darw_table_and_prompt(&all_locations, get_params)?;

        Ok(())
    }
}

fn darw_table_and_prompt(
    all_locations: &[FavoriteFolderPath],
    get_params: &GetParams,
) -> AppResult {
    let clipboard = get_params.copy_has_clipboard();
    let table = data_access::get_all_fav_table(all_locations, get_params)?;
    if get_params.copy_ask_number() {
        println!("{table}");

        let given_number =
            console_interaction::ask_possible_prompt_for_ask_number(all_locations, get_params)?;

        match given_number {
            Some(index_start_from_one) => {
                let index = index_start_from_one - 1;
                // function for asking number of user ensures that the index will not
                // be out of bounds
                let to_put = all_locations.get(index).unwrap();

                put_into_clipboard_or_print(to_put.get_path(), clipboard)?;
                Ok(())
            }
            None => {
                put_into_clipboard_or_print(&table, clipboard)?;
                Ok(())
            }
        }
    } else {
        put_into_clipboard_or_print(&table, clipboard)?;
        Ok(())
    }
}
