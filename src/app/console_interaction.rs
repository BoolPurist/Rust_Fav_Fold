use std::io::Write;

use crate::{cli_args::GetParams, favorite_folder_record::FavoriteFolderPath, AppResult};

pub fn ask_possible_prompt_for_ask_number(
    paths: &Vec<FavoriteFolderPath>,
    get_params: &GetParams,
) -> AppResult<Option<usize>> {
    if !get_params.copy_ask_number() {
        return Ok(None);
    }
    let len = paths.len() - 1;
    let prompt_message = format!("Enter a whole number between {} and {}: ", 1, len);
    let user_input = read_line_from_user(&prompt_message)?;

    let parsed_number: usize = user_input
        .trim()
        .parse()
        .map_err(|_| "please provide a number for the selection".to_string())?;

    if parsed_number > len {
        Err(format!("Number is not between {} and {}", 1, len).into())
    } else {
        Ok(Some(parsed_number))
    }
}

pub fn read_line_from_user(prompt: &str) -> AppResult<String> {
    print!("{prompt}");
    std::io::stdout().flush()?;

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
