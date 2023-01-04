use colored::*;

pub fn color_promt_msg(to_color: &str) -> String {
    to_color.blue().to_string()
}
pub fn color_error_msg(to_color: &str) -> String {
    to_color.red().to_string()
}
pub fn color_exists_msg(to_color: &str) -> String {
    to_color.green().to_string()
}
pub fn color_not_found(to_color: &str) -> String {
    to_color.red().to_string()
}
