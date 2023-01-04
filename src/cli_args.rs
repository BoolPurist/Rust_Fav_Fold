use clap::Parser;

/// Structs to define the allowed and passable arguments for app over cli
/// All positional arguments are validated to be non-empty or not only whitespaces
/// The argument is made up of subcommands
#[derive(Parser, Debug)]
#[command(
    author = "BoolPurist",
    version = "1.0.0",
    about = "Tool to save and load paths to file or folders",
    propagate_version = true
)]
pub enum Commands {
    #[command(visible_alias = "g")]
    /// Outputs location of given name or all paths if no name is given.
    /// Location to existing files/folders will be shown in green otherwise red.
    Get(GetParams),
    #[command(visible_alias = "r")]
    /// Changes name of favorite path.
    Rename {
        /// name/label to change.
        #[arg(value_parser = parse_trimmed_not_empty)]
        old_name_favorite: String,
        /// new name/label to use for a location.
        #[arg(value_parser = parse_trimmed_not_empty)]
        new_name_favorite: String,
    },
    #[command(visible_alias = "s")]
    /// Creates or changes location under a given name.
    Set {
        /// Name for a new or existing location.
        #[arg(value_parser = parse_trimmed_not_empty)]
        name_favorite: String,
        /// Location under the new or new location under a existing name.
        #[arg(value_parser = parse_trimmed_not_empty)]
        new_path: String,
    },
    #[command(visible_alias = "d")]
    /// Removes given name with its path. Note: The location on your files system will not be
    /// removed of course.
    Delete {
        /// Name with its location to be removed.
        #[arg(value_parser = parse_trimmed_not_empty)]
        name_favorite: String,
    },
    #[command(visible_alias = "p", about = "")]
    /// Creates or changes path under given label with current working directory
    PwdSet {
        /// New name or existing name under which the current working directory is to be written.
        #[arg(value_parser = parse_trimmed_not_empty)]
        name_favorite: String,
    },
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
    /// The path of the location with the respective line is then outputed.
    #[arg(short, long)]
    ask_number: bool,
    #[arg(short, long)]
    /// if given name is not found then all paths are listed with a name in which the given name
    /// occures
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

/// Function used to make sure no positional arguments are empty or only whitespaces
fn parse_trimmed_not_empty(to_parse: &str) -> Result<String, String> {
    let trimmed = to_parse.trim().to_string();

    if trimmed.is_empty() {
        Err("Must not be emtpy or only whitespaces".into())
    } else {
        Ok(trimmed)
    }
}
