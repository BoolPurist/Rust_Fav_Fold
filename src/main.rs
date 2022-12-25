use std::error::Error;

use arboard::Clipboard;
use clap::Parser;
use colored::*;
use folder_favorite::cli_args::MainCommand;
use folder_favorite::cli_args::Subcommands;
use folder_favorite::command_handling;

fn main() {
    let args = MainCommand::parse();

    handle_subcommand(&args.sub_commands);
}

fn handle_subcommand(sub_commands: &Subcommands) {
    return match sub_commands {
        Subcommands::Set {
            name_favorite,
            new_path,
        } => match command_handling::set_favorite_data(&name_favorite, &new_path) {
            Ok(_) => (),
            Err(error) => exit_with_error(&*error),
        },
        Subcommands::Get { name, clipboard } => match name {
            Some(get_name) => {
                let favorite = command_handling::get_fav(&get_name);

                match favorite {
                    Ok(read) => {
                        put_into_clipboard_or_print(read.get_path(), *clipboard);
                    }
                    Err(error) => exit_with_error(&*error),
                }
            }
            None => match command_handling::get_all_fav_table(*clipboard) {
                Ok(table) => put_into_clipboard_or_print(&table, *clipboard),
                Err(error) => exit_with_error(&*error),
            },
        },
        Subcommands::Delete { name_favorite } => {
            match command_handling::remove_from_fav(name_favorite) {
                Ok(_) => (),
                Err(error) => exit_with_error(&*error),
            }
        }
        Subcommands::Rename {
            old_name_favorite,
            new_name_favorite,
        } => match command_handling::rename_fav(&old_name_favorite, &new_name_favorite) {
            Ok(_) => (),
            Err(error) => exit_with_error(&*error),
        },
    };

    fn put_into_clipboard_or_print(content: &str, clipboard: bool) {
        if clipboard {
            if let Err(clip_error) = put_into_clipboard(content) {
                exit_with_error(&*clip_error)
            }
        } else {
            println!("{content}")
        };
    }
}

fn exit_with_error(message: &dyn Error) {
    let red_msg = format!("Error: {message}").red().to_string();
    eprint!("{red_msg}");
    std::process::exit(1);
}

fn put_into_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;

    Ok(())
}
