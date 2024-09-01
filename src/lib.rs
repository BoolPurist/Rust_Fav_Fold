//! # Error handling strategy
//!
//! If an result is an error then it will be propagated up to the binary crate currently.

pub use all_favorites::AllFavorites;

pub mod all_favorites;
pub mod app;
pub mod cli_args;
pub mod clipboard;
pub mod constants;
pub mod data_access;
pub mod favorite_folder_record;
pub mod file_access;
pub mod logging;
pub mod paths;
pub mod prelude;

mod trimmed_not_empty_text;

use std::error::Error;

pub mod favorite_table;

pub type AppResult<T = ()> = Result<T, Box<dyn Error>>;
