use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "BoolPurist",
    version = "1.0.0",
    about = "Tool to save and load paths to file or folders",
    propagate_version = true
)]
pub enum Commands {
    #[command(
        visible_alias = "g",
        about = "Outputs path of given name or all paths if no name is given"
    )]
    Get {
        name: Option<String>,
        #[arg(short, long)]
        clipboard: bool,
    },
    #[command(visible_alias = "r", about = "Changes name of favorite path")]
    Rename {
        old_name_favorite: String,
        new_name_favorite: String,
    },
    #[command(
        visible_alias = "s",
        about = "Creates or changes path under a given name"
    )]
    Set {
        name_favorite: String,
        new_path: String,
    },
    #[command(visible_alias = "d", about = "Removes given name with its path")]
    Delete { name_favorite: String },
}
