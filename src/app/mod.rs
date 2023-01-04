use crate::cli_args::GetParams;
use colored::*;
use std::error::Error;

use crate::{clipboard, data_access, file_access, AppResult};

pub mod console_interaction;

pub fn put_into_clipboard_or_print(content: &str, clipboard: bool) -> AppResult {
    if clipboard {
        clipboard::put_into_clipboard(content)?;
    } else {
        println!("{content}");
    }

    Ok(())
}
pub fn exit_with_error(message: &dyn Error) {
    let red_msg = format!("Error: {message}").red().to_string();
    eprint!("{red_msg}");
    std::process::exit(1);
}
pub fn handle_get_subcommand(get_params: &GetParams) -> AppResult {
    let clipboard = get_params.copy_has_clipboard();
    match get_params.get_name() {
        Some(_) => {
            let favorite = data_access::get_fav(get_params)?;
            put_into_clipboard_or_print(favorite.get_path(), clipboard)?;

            Ok(())
        }
        None => {
            let all_locations = file_access::get_favorites()?;
            let table = data_access::get_all_fav_table(&all_locations, get_params)?;
            if get_params.copy_ask_number() {
                println!("{table}");

                let given_number = console_interaction::ask_possible_prompt_for_ask_number(
                    &all_locations,
                    get_params,
                )?;

                match given_number {
                    Some(index_start_from_one) => {
                        let index = index_start_from_one - 1;
                        // function for asking number of user ensures that the index will not
                        // be out of bounds
                        let to_put = &all_locations[index];

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
    }
}
