use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::favorite_folder_record::FavoriteFolderPath;

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AllFavorites(Vec<FavoriteFolderPath>);

impl From<AllFavorites> for Vec<FavoriteFolderPath> {
    fn from(value: AllFavorites) -> Self {
        value.0
    }
}

impl AllFavorites {
    pub fn new(folders: impl IntoIterator<Item = FavoriteFolderPath>) -> Self {
        Self(folders.into_iter().collect())
    }

    fn find_by_name(&self, name: &str) -> Option<usize> {
        self.0.iter().position(|fav| fav.get_name() == name)
    }

    pub fn remove_with_name(&mut self, name: &str) -> bool {
        match self.find_by_name(name) {
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
}

#[cfg(test)]
mod testing {
    use std::path::Path;

    use crate::{favorite_folder_record::FavoriteFolderPath, AllFavorites};

    const INPUT: &str = include_str!("test_input.json");
    fn given() -> AllFavorites {
        let parsed: Vec<FavoriteFolderPath> = serde_json::from_str(INPUT).unwrap();
        AllFavorites::new(parsed)
    }

    #[test]
    fn if_dangling_delete_a_path() {
        let expected = AllFavorites::new([FavoriteFolderPath::new(
            "proc_macro",
            &Path::new("/home/some_user/Code/rust/proc-macro-workshop"),
        )
        .unwrap()]);

        let mut given = given();
        given.clean_all_dangling(|path| {
            let path = path.to_str().unwrap();
            path == "/home/some_user/Documents/Studium" || path == "/home/some_user/Code/rust"
        });
        assert_eq!(expected, given);
    }

    #[test]
    fn find_a_path_by_name() {
        fn assert_case(name: &str, expected: Option<usize>) {
            let given = given();
            let actual = given.find_by_name(name);
            assert_eq!(expected, actual, "Name: {}", name);
        }

        assert_case("a", None);
        assert_case("dev_rust", Some(1));
    }

    #[test]
    fn name_not_found_dont_remove_it() {
        let (actual, has_deleted) = set_up_and_act_remove("not_there");
        const EXPECTED: bool = false;
        assert_eq!(EXPECTED, has_deleted);
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn name_found_then_remove_it() {
        let (actual, has_deleted) = set_up_and_act_remove("dev_rust");
        const EXPECTED: bool = true;
        assert_eq!(EXPECTED, has_deleted);
        insta::assert_debug_snapshot!(actual);
    }

    fn set_up_and_act_remove(name: &str) -> (AllFavorites, bool) {
        let mut given = given();
        let actual = given.remove_with_name(name);
        (given, actual)
    }
}
