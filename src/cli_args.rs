use clap::Parser;

use crate::trimmed_not_empty_text::NonEmptyText;

/// Structs to define the allowed and passable arguments for app over cli
/// All positional arguments are validated to be non-empty or not only whitespaces
/// The argument is made up of subcommands
#[derive(Parser, Debug)]
#[command(
    author = "BoolPurist",
    version,
    about = "Tool to save and load paths to file or folders",
    propagate_version = true
)]
pub enum CliCommands {
    #[command(visible_alias = "g")]
    /// Outputs location of given name or all paths if no name is given.
    /// Location to existing files/folders will be shown in green otherwise red.
    Get(GetParams),
    #[command(visible_alias = "r")]
    /// Changes name of favorite path.
    Rename {
        /// name/label to change.
        old_name_favorite: NonEmptyText,
        /// new name/label to use for a location.
        new_name_favorite: NonEmptyText,
    },
    #[command(visible_alias = "s")]
    /// Creates or changes location under a given name.
    Set {
        /// Name for a new or existing location.
        name_favorite: NonEmptyText,
        /// Location under the new or new location under a existing name.
        new_path: NonEmptyText,
    },
    #[command(visible_alias = "d")]
    /// Removes given name with its path. Note: The location on your files system will not be
    /// removed of course.
    Delete {
        /// Name with its location to be removed.
        name_favorite: NonEmptyText,
    },
    /// Removes all non-existing paths.
    #[command(visible_alias = "c")]
    Clean,
    #[command(visible_alias = "p")]
    /// Creates or changes path under given label with current working directory
    PwdSet {
        /// New name or existing name under which the current working directory is to be written.
        name_favorite: NonEmptyText,
    },
    Reset,
}

#[derive(Parser, Debug)]
#[command(author = "BoolPurist")]
pub struct GetParams {
    /// Label/name to get the location from. If left out then all names with their location are
    /// shown.
    name: Option<String>,
    /// If provided then the output will be written to clipboard instead of stdout.
    #[arg(short, long)]
    clipboard: bool,
    /// list all names and paths with line numbers. Waits for one line to accept a line number.
    /// The path of the location with the respective line is then outputted.
    #[arg(short, long)]
    ask_number: bool,
    #[arg(short, long)]
    /// if given name is not found then all paths are listed with a name in which the given name
    /// occurs
    fuzzy: bool,
}

impl GetParams {
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn copy_has_clipboard(&self) -> bool {
        self.clipboard
    }
    pub fn copy_ask_number(&self) -> bool {
        self.ask_number
    }
    pub fn copy_fuzzy(&self) -> bool {
        self.fuzzy
    }
}

#[cfg(test)]
mod testing {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        CliCommands::command().debug_assert();
    }
}
