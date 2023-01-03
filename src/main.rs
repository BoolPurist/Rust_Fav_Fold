use std::error::Error;

use clap::Parser;
use colored::*;

use folder_favorite::AppResult;
use folder_favorite::{cli_args::Commands, command_handling, linux_clipboard};

fn main() {
    // if any error is comes from the lib crate then the user gets the error displayed as an red
    // text. The program always exits with error code 1 in these cases.

    // Need to check if the program was started as child process to keep set clipbaord alive
    // for linux
    if let Err(error) = linux_clipboard::execute_as_possible_daemon_clipboard() {
        exit_with_error(&*error);
    }

    let args = Commands::parse();
    if let Err(error) = handle_subcommand(&args) {
        exit_with_error(&*error);
    }
}

fn handle_subcommand(sub_commands: &Commands) -> AppResult {
    return match sub_commands {
        Commands::Set {
            name_favorite,
            new_path,
        } => {
            command_handling::set_favorite_data(name_favorite, new_path)?;

            Ok(())
        }
        Commands::Get { name, clipboard } => match name.as_ref() {
            Some(get_name) => {
                let favorite = command_handling::get_fav(get_name)?;
                put_into_clipboard_or_print(favorite.get_path(), *clipboard)?;
                return Ok(());
            }
            None => {
                let table = command_handling::get_all_fav_table(*clipboard)?;
                put_into_clipboard_or_print(&table, *clipboard)?;
                return Ok(());
            }
        },
        Commands::Delete { name_favorite } => {
            command_handling::remove_from_fav(name_favorite)?;

            Ok(())
        }
        Commands::Rename {
            old_name_favorite,
            new_name_favorite,
        } => {
            command_handling::rename_fav(old_name_favorite, new_name_favorite)?;
            Ok(())
        }
        Commands::PwdSet { name_favorite } => {
            command_handling::set_label_to_cwd(name_favorite)?;
            Ok(())
        }
    };
}

fn put_into_clipboard_or_print(content: &str, clipboard: bool) -> AppResult {
    if clipboard {
        linux_clipboard::put_into_clipboard(content)?;
    } else {
        println!("{content}");
    }

    Ok(())
}
fn exit_with_error(message: &dyn Error) {
    let red_msg = format!("Error: {message}").red().to_string();
    eprint!("{red_msg}");
    std::process::exit(1);
}
