//! # Error handling strategy
//!
//! If an result is an error then it will be propagated up to the binary crate currently.

use std::error::Error;
pub mod cli_args;
pub mod command_handling;
pub mod favorite_folder_record;
pub mod file_data;
pub mod linux_clipboard;
pub mod paths;
pub type AppResult<T = ()> = Result<T, Box<dyn Error>>;
