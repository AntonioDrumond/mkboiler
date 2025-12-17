# mkboiler

- Simple CLI for generating boilerplates for multiple kinds of file types and languages
- If your language is not supported, please send us a pull request :)


## Available options
- `c` -> main.c file with main function and basic includes
- `cpp` -> main.cc file with main functions and basic includes
- `default.nix` -> Simple default.nix file with room for imports, etc 
- `shell.nix` -> Simple shell.nix file with empty shellHook and buildInputs sections
- `python13` -> Simple shell.nix file with the python package and an empty section for including dependencies
- `rust_shell` -> SImple shell.nix file with rustc, cargo and other necessary packages for developing rust projects.
- `iced_shell` -> SImple shell.nix file with everything needed for a general rust project, plus all the fluff necessary to run GUIs made with the iced crate.
