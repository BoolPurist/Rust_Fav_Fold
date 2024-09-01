use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{favorite_folder_record::FavoriteFolderPath, trimmed_not_empty_text::NonEmptyText};

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AllFavorites(Vec<FavoriteFolderPath>);

impl FromIterator<FavoriteFolderPath> for AllFavorites {
    fn from_iter<T: IntoIterator<Item = FavoriteFolderPath>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AfterInsertion {
    Changed,
    Added,
}

impl AllFavorites {
    pub fn new(folders: Vec<FavoriteFolderPath>) -> Self {
        Self(folders)
    }

    pub fn iter(&self) -> impl Iterator<Item = &FavoriteFolderPath> {
        self.0.iter()
    }

    pub fn as_slice(&self) -> &[FavoriteFolderPath] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, new_favorite: FavoriteFolderPath) -> AfterInsertion {
        match self.find_by_name_index(new_favorite.get_name()) {
            Some(index) => {
                let to_set = self.0.get_mut(index).unwrap();
                *to_set = new_favorite;
                AfterInsertion::Changed
            }
            None => {
                self.0.push(new_favorite);
                AfterInsertion::Added
            }
        }
    }

    pub fn get(&self, name: NonEmptyText) -> Option<&FavoriteFolderPath> {
        self.find_by_name(&name)
    }

    pub fn filtered_containing_name(self, name: NonEmptyText) -> AllFavorites {
        let filtered = self
            .0
            .into_iter()
            .filter(|possible_match| possible_match.get_name().contains(name.as_str()))
            .collect();
        Self::new(filtered)
    }

    pub fn rename(&mut self, old_name: &NonEmptyText, new_name: NonEmptyText) -> bool {
        match self.find_by_name_mut(old_name) {
            Some(to_rename) => {
                to_rename.set_name(new_name);
                true
            }
            None => false,
        }
    }

    pub fn remove_with_name(&mut self, name: &NonEmptyText) -> bool {
        match self.find_by_name_index(name) {
            Some(index) => {
                self.0.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn clean_all_dangling(&mut self, mut on_is_dangling_check: impl FnMut(&Path) -> bool) {
        self.0
            .retain_mut(|maybe_dangling| !on_is_dangling_check(maybe_dangling.path()));
    }

    fn find_by_name_mut(&mut self, name: &str) -> Option<&mut FavoriteFolderPath> {
        self.0.iter_mut().find(|fav| fav.get_name() == name)
    }

    fn find_by_name(&self, name: &str) -> Option<&FavoriteFolderPath> {
        self.0.iter().find(|fav| fav.get_name() == name)
    }

    pub fn find_by_name_index(&self, name: &str) -> Option<usize> {
        self.0.iter().position(|fav| fav.get_name() == name)
    }
}

impl From<Vec<FavoriteFolderPath>> for AllFavorites {
    fn from(value: Vec<FavoriteFolderPath>) -> Self {
        Self(value)
    }
}

impl From<AllFavorites> for Vec<FavoriteFolderPath> {
    fn from(value: AllFavorites) -> Self {
        value.0
    }
}

#[cfg(test)]
mod testing {

    use crate::{
        all_favorites::AfterInsertion, favorite_folder_record::FavoriteFolderPath,
        trimmed_not_empty_text::NonEmptyText, AllFavorites,
    };

    const INPUT: &str = include_str!("test_input.json");
    const INPUT_LONGER: &str = include_str!("longer_test_input.json");
    fn given_initial() -> AllFavorites {
        let parsed: Vec<FavoriteFolderPath> = serde_json::from_str(INPUT).unwrap();
        AllFavorites::new(parsed)
    }

    fn given_longer_initial() -> AllFavorites {
        let parsed: Vec<FavoriteFolderPath> = serde_json::from_str(INPUT_LONGER).unwrap();
        AllFavorites::new(parsed)
    }

    #[test]
    fn if_dangling_delete_a_path() {
        let expected = AllFavorites::from_iter([FavoriteFolderPath::new(
            NonEmptyText::unwrap("proc_macro"),
            NonEmptyText::unwrap("/home/some_user/Code/rust/proc-macro-workshop"),
        )]);

        let mut given = given_initial();
        given.clean_all_dangling(|path| {
            let path = path.to_str().unwrap();
            path == "/home/some_user/Documents/Studium" || path == "/home/some_user/Code/rust"
        });
        assert_eq!(expected, given);
    }

    #[test]
    fn find_a_path_by_name() {
        fn assert_case(name: &str, expected: Option<usize>) {
            let given = given_initial();
            let actual = given.find_by_name_index(name);
            assert_eq!(expected, actual, "Name: {}", name);
        }

        assert_case("a", None);
        assert_case("dev_rust", Some(1));
    }

    #[test]
    fn name_not_found_dont_remove_it() {
        let (actual, has_deleted) = set_up_and_act_remove(NonEmptyText::unwrap("not_there"));
        const EXPECTED: bool = false;
        assert_eq!(EXPECTED, has_deleted);
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn name_found_then_remove_it() {
        let (actual, has_deleted) = set_up_and_act_remove(NonEmptyText::unwrap("dev_rust"));
        const EXPECTED: bool = true;
        assert_eq!(EXPECTED, has_deleted);
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn name_found_rename_it() {
        let (actual, has_deleted) = set_up_and_act_rename(
            NonEmptyText::unwrap("dev_rust"),
            NonEmptyText::unwrap("other_dev_rust"),
        );
        const EXPECTED: bool = true;
        assert_eq!(EXPECTED, has_deleted);
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn no_name_found_dont_rename() {
        let (actual, has_deleted) = set_up_and_act_rename(
            NonEmptyText::unwrap("not_found"),
            NonEmptyText::unwrap("is_an_error"),
        );
        const EXPECTED: bool = false;
        assert_eq!(EXPECTED, has_deleted);
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn add_non_existing_name() {
        let mut given_data = given_initial();
        let given = FavoriteFolderPath::new(
            NonEmptyText::unwrap("new_added"),
            NonEmptyText::unwrap("~/some_data"),
        );

        const EXPECTED: AfterInsertion = AfterInsertion::Added;
        let actual = given_data.insert(given);
        assert_eq!(EXPECTED, actual);
        insta::assert_debug_snapshot!(given_data);
    }

    #[test]
    fn set_existing_favorite() {
        let mut given_data = given_initial();
        let given = FavoriteFolderPath::new(
            NonEmptyText::unwrap("dev_rust"),
            NonEmptyText::unwrap("~/new_some_data"),
        );

        const EXPECTED: AfterInsertion = AfterInsertion::Changed;
        let actual = given_data.insert(given);
        assert_eq!(EXPECTED, actual);
        insta::assert_debug_snapshot!(given_data);
    }

    #[test]
    fn filter_for_containing_names() {
        let given_data = given_longer_initial();
        let param = NonEmptyText::unwrap("ping");
        let actual = given_data.filtered_containing_name(param);
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn filter_out_all_for_no_matching() {
        let given_data = given_longer_initial();
        let param = NonEmptyText::unwrap("xxxx");
        let actual = given_data.filtered_containing_name(param);
        assert!(actual.as_slice().is_empty());
    }

    fn set_up_and_act_rename(
        old_name: NonEmptyText,
        new_name: NonEmptyText,
    ) -> (AllFavorites, bool) {
        let mut given = given_initial();
        let actual = given.rename(&old_name, new_name);
        (given, actual)
    }

    fn set_up_and_act_remove(name: NonEmptyText) -> (AllFavorites, bool) {
        let mut given = given_initial();
        let actual = given.remove_with_name(&name);
        (given, actual)
    }
}
