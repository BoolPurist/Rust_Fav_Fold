use clap::Parser;
use folder_favorite::{cli_args::CliCommands, clipboard, data_access, AppResult};

use folder_favorite::{app, logging};

fn main() {
    logging::set_up_logging();
    // if any error is comes from the lib crate then the user gets the error displayed as an red
    // text. The program always exits with error code 1 in these cases.

    // Need to check if the program was started as child process to keep set clipbaord alive
    // for linux
    if let Err(error) = clipboard::execute_as_possible_daemon_clipboard() {
        app::exit_with_error(&*error);
    }

    let args = CliCommands::parse();
    if let Err(error) = handle_subcommand(&args) {
        app::exit_with_error(&*error);
    }
}

fn handle_subcommand(sub_commands: &CliCommands) -> AppResult {
    match sub_commands {
        CliCommands::Set {
            name_favorite,
            new_path,
        } => {
            data_access::set_favorite_data(name_favorite, new_path)?;

            Ok(())
        }
        CliCommands::Get(get_params) => app::handle_get_subcommand(get_params),
        CliCommands::Delete { name_favorite } => {
            data_access::remove_from_fav(name_favorite)?;

            Ok(())
        }
        CliCommands::Rename {
            old_name_favorite,
            new_name_favorite,
        } => {
            data_access::rename_fav(old_name_favorite, new_name_favorite)?;
            Ok(())
        }
        CliCommands::PwdSet { name_favorite } => {
            data_access::set_label_to_cwd(name_favorite)?;
            Ok(())
        }
        CliCommands::Clean => data_access::remove_all_non_existing(),
    }
}
