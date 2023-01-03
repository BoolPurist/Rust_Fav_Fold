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
        #[arg(value_parser = parse_trimmed_not_empty)]
        old_name_favorite: String,
        #[arg(value_parser = parse_trimmed_not_empty)]
        new_name_favorite: String,
    },
    #[command(
        visible_alias = "s",
        about = "Creates or changes path under a given name"
    )]
    Set {
        #[arg(value_parser = parse_trimmed_not_empty)]
        name_favorite: String,
        #[arg(value_parser = parse_trimmed_not_empty)]
        new_path: String,
    },
    #[command(visible_alias = "d", about = "Removes given name with its path")]
    Delete {
        #[arg(value_parser = parse_trimmed_not_empty)]
        name_favorite: String,
    },
    #[command(
        visible_alias = "p",
        about = "Creates or changes path under given label with current working directory"
    )]
    PwdSet {
        #[arg(value_parser = parse_trimmed_not_empty)]
        name_favorite: String,
    },
}

fn parse_trimmed_not_empty(to_parse: &str) -> Result<String, String> {
    let trimmed = to_parse.trim().to_string();

    if trimmed.is_empty() {
        Err("Must not be emtpy or only whitespaces".into())
    } else {
        Ok(trimmed)
    }
}
