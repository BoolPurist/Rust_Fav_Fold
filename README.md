# Folder Favorite

CLI tool to save and see paths with labels. 
The paths are saved in a JSON file at the typical app data location of your os.

Indented to be used on Windows, Linux and Mac.
Functionality only confirmed under Linux with the distribution Fedora so far.


## Installation

1. Install rust like described here (Link)[https://www.rust-lang.org/tools/install]
2. Clone this repository  via git clone
3. Go into clone repository 
4. Execute the following command
5. You might need to restart your shell so the command "folder_favorite" is available.

```
cargo install --bin folder_favorite --path "."
```

## Usage

This CLI app has subcommands to execute action like saving or showing a path.
Paths are created and addressed via label.

## Features

- Can save paths with label.
- Can copy saved paths into OS clipboard via option flag.
- Shows with colors which paths exits on file system
- Can remember files with a certain name.
- Can rename label for a path
- Can remove paths on this app by providing label. Does not remove the folder of file under the path. 

## Examples 

### Create a path  with label 

Creating a path to "/home/dummy" with label "some_path"
```sh
folder_favorite set some_path /home/some_path
```

### Show all saved paths. 

In this example the path /home/dummy under the label "some_path" and the path
/home/awesome under label "another_path" are saved.

```
folder_favorite get
```

Output
```
some_path     /home/dummy
another path  /home/awesome
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

