use std::str::FromStr;

use derive_more::{AsRef, Deref, Display, Into};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type ResultNonEmptyText = Result<NonEmptyText, NotEmptyTextError>;

#[derive(
    Debug,
    Into,
    AsRef,
    Deref,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct NonEmptyText(String);

#[derive(Debug, Error, PartialEq, Eq)]
#[error("Trimmed text must not only contain whitespaces")]
pub struct NotEmptyTextError;

impl NonEmptyText {
    pub fn new(value: impl AsRef<str>) -> ResultNonEmptyText {
        let value = value.as_ref();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(NotEmptyTextError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    #[cfg(test)]
    pub fn unwrap(value: impl AsRef<str>) -> Self {
        Self::new(value).unwrap()
    }
}

impl TryFrom<&str> for NonEmptyText {
    type Error = NotEmptyTextError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for NonEmptyText {
    type Error = NotEmptyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for NonEmptyText {
    type Err = NotEmptyTextError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod testing {
    use crate::trimmed_not_empty_text::NonEmptyText;

    use super::*;

    #[test]
    fn result_if_not_empty_trimmed_text() {
        fn assert_case(given: &str, expected: ResultNonEmptyText) {
            let actual = NonEmptyText::new(given);
            assert_eq!(expected, actual, "Given: {}", given);
        }
        assert_case("Some Text", Ok(NonEmptyText("Some Text".to_string())));
        assert_case("  Some Text  ", Ok(NonEmptyText("Some Text".to_string())));
        assert_case("  Some  Text  ", Ok(NonEmptyText("Some  Text".to_string())));
        assert_case("   ", Err(NotEmptyTextError));
        assert_case("", Err(NotEmptyTextError));
        assert_case("\n", Err(NotEmptyTextError));
        assert_case("\t", Err(NotEmptyTextError));
    }
}
