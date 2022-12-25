use clap::{Parser, Subcommand};
// exec get (all)
// exec get -c|-clipboard <name_of_folder_favorite>
// exec rename <name_of_folder_favorite> <new_name_of_folder_favorite>
// exec set <name_of_folder_favorite> <new_path>
// exec delete <name_of_folder_favorite>
#[derive(Parser, Debug)]
pub struct MainCommand {
    #[command(subcommand)]
    pub sub_commands: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Get {
        name: Option<String>,
        #[arg(short, long)]
        clipboard: bool,
    },
    Rename {
        old_name_favorite: String,
        new_name_favorite: String,
    },
    Set {
        name_favorite: String,
        new_path: String,
    },
    Delete {
        name_favorite: String,
    },
}
