use std::path::Path;

use crate::favorite_folder_record::FavoriteFolderPath;

#[derive(Debug, PartialEq, Eq)]
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
}
