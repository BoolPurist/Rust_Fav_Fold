pub use draw_params::DrawParam;
mod draw_params;

use crate::app::term_colors;

use crate::favorite_folder_record::FavoriteFolderPath;
use shellexpand;
use std::borrow::Cow;
use std::path::Path;

#[derive(Debug)]
struct LineNameAndPath {
    name: String,
    path: String,
}

pub fn draw_favorite_table(all_locations: &[FavoriteFolderPath], params: DrawParam) -> String {
    let ask_numbers = params.ask_number();
    if params.clipboard() {
        draw_without_colors(all_locations, ask_numbers)
    } else {
        draw_with_colors(all_locations, ask_numbers)
    }
}

fn draw_without_colors(all_locations: &[FavoriteFolderPath], ask_numbers: bool) -> String {
    prepare_drawing(all_locations, ask_numbers)
        .into_iter()
        .map(|line| format!("{}{}", line.name, line.path))
        .collect::<Vec<String>>()
        .join("\n")
}

fn draw_with_colors(all_locations: &[FavoriteFolderPath], ask_numbers: bool) -> String {
    prepare_drawing(all_locations, ask_numbers)
        .into_iter()
        .map(|line| {
            let colored_path = get_colored_path_if_no_clipboard(&line.path);
            format!("{}{}", line.name, colored_path)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn prepare_drawing(
    all_locations: &[FavoriteFolderPath],
    ask_numbers: bool,
) -> Vec<LineNameAndPath> {
    let spacing_padding = get_spacing_padding();

    let records_without_line_numbers =
        construct_recores_without_numbers(all_locations, &spacing_padding);
    let lines = if ask_numbers {
        handle_line_number_if_needed(
            all_locations,
            records_without_line_numbers,
            &spacing_padding,
        )
    } else {
        records_without_line_numbers.collect()
    };

    return lines;

    fn calc_max_width(all_locations: &[FavoriteFolderPath]) -> usize {
        all_locations
            .iter()
            .fold(0, |akk, next| akk.max(next.get_name().len()))
    }

    fn construct_recores_without_numbers<'a>(
        all_locations: &'a [FavoriteFolderPath],
        spacing_padding: &'a str,
    ) -> impl Iterator<Item = LineNameAndPath> + 'a {
        let max_width = calc_max_width(all_locations);
        all_locations.iter().map(move |next_record| {
            let name = next_record.get_name();
            assert!(
                name.len() <= max_width,
                "Could not find max width for all labels"
            );

            let padded_name = pad_from_right_until(name, max_width);
            let raw_path = next_record.path_str();

            let padded_name = format!("{padded_name}{spacing_padding}");
            LineNameAndPath {
                name: padded_name,
                path: raw_path.to_string(),
            }
        })
    }

    fn handle_line_number_if_needed(
        all_locations: &[FavoriteFolderPath],
        records_without_line_numbers: impl Iterator<Item = LineNameAndPath>,
        spacing_padding: &str,
    ) -> Vec<LineNameAndPath> {
        let max_number_width = all_locations.len().to_string().chars().count();

        records_without_line_numbers
            .enumerate()
            .map(move |(index, line)| {
                let number = index + 1;
                let to_pad = number.to_string();
                let padded_number = format!(
                    "{}{}{}",
                    pad_from_right_until(&to_pad, max_number_width),
                    spacing_padding,
                    line.name
                );

                LineNameAndPath {
                    name: padded_number,
                    path: line.path,
                }
            })
            .collect()
    }
}

fn get_colored_path_if_no_clipboard(raw_path: &str) -> String {
    let expanded = shellexpand::tilde(raw_path);
    let expanded_path = Path::new(expanded.as_ref());
    check_if_exits(expanded_path, raw_path)
}

fn check_if_exits(path: &Path, to_check_color: &str) -> String {
    if path.exists() {
        term_colors::color_exists_msg(to_check_color)
    } else {
        term_colors::color_not_found(to_check_color)
    }
}

/// Size of space between columns in a output.
/// Example: `Spacing` is 2 then between name and path is 2 whitespaces in the output for the
/// user.
const SPACING: usize = 2;

fn get_spacing_padding() -> String {
    " ".repeat(SPACING)
}

/// # Summary
///
/// Makes sure the return value is at least as long as `max_with` chars.
///
/// # Returns
///
/// Either returns the `to_pad` unchanged as ref or as new owned value with spaces appended from right
/// so the string is exactly `max_with` chars long.
fn pad_from_right_until(to_pad: &str, max_width: usize) -> Cow<'_, str> {
    let actual_len = to_pad.len();

    if actual_len >= max_width {
        return to_pad.into();
    }

    let diff = max_width - actual_len;
    let padding = " ".repeat(diff);
    let mut padded = String::from(to_pad);
    padded.push_str(&padding);

    padded.into()
}

#[cfg(test)]
mod testing {
    use crate::AllFavorites;

    use super::*;
    const INPUT: &str = include_str!("longer_test_input.json");

    fn given_initial() -> AllFavorites {
        let parsed: Vec<FavoriteFolderPath> = serde_json::from_str(INPUT).unwrap();
        AllFavorites::new(parsed)
    }

    #[test]
    fn list_all_every_favorite() {
        let given = given_initial();
        let actual = draw_without_colors(given.as_slice(), false);
        insta::assert_snapshot!(actual);
    }

    #[test]
    fn list_all_with_prompt_every_favorite() {
        let given = given_initial();
        let actual = draw_without_colors(given.as_slice(), true);
        insta::assert_snapshot!(actual);
    }
}
