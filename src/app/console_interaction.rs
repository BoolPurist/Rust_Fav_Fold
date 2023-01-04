use super::term_colors;
use crate::{cli_args::GetParams, favorite_folder_record::FavoriteFolderPath, AppResult};
use std::io::Write;

pub fn ask_possible_prompt_for_ask_number(
    paths: &[FavoriteFolderPath],
    get_params: &GetParams,
) -> AppResult<Option<usize>> {
    if !get_params.copy_ask_number() {
        return Ok(None);
    }
    let len = paths.len();
    let prompt_message = format!("Enter a whole number between {} and {}: ", 1, len);
    let user_input = read_line_from_user(&prompt_message)?;

    let parsed_number: usize = user_input
        .trim()
        .parse()
        .map_err(|_| gen_req_number_message(len))?;

    if parsed_number == 0 || parsed_number > len {
        Err(gen_req_number_message(len).into())
    } else {
        Ok(Some(parsed_number))
    }
}

pub fn read_line_from_user(prompt: &str) -> AppResult<String> {
    let colored_prompt = term_colors::color_promt_msg(prompt);
    print!("{colored_prompt}");
    std::io::stdout().flush()?;

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn gen_req_number_message(max: usize) -> String {
    format!("Number is not between {} and {}", 1, max)
}
