# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2024-09-01

### Breaking change

- renamed file format: field "path" was renamed to "location".
  The following command with sd applies the needed changes
```sh
# path to favorites.json is under the data folder of this application 
# On linux this would be "~/.local/share/folderfavorite/favorites.json"
cat <path_to_favorites.json> | sd "\"path\"" "\"location\"" <path_to_favorites.json>
```

### Fixed 

- Shown version number aka --version was outdated. 
  It now always sync to the rust package version number
- Fixed problem with subcommand  where dangling paths were not removed and existing ones were removed instead.

## [1.4.4] - 2024-08-31

### Added

- New subcommand "Clean". Removes all non existing paths

[Unreleased]: https://github.com/BoolPurist/Rust_Fav_Fold
[2.0.0]: https://github.com/BoolPurist/Rust_Fav_Fold/releases/tag/v2.0.0
[1.4.4]: https://github.com/BoolPurist/Rust_Fav_Fold/releases/tag/v1.4.3
