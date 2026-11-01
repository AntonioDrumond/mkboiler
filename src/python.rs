use std::fs;

use crate::{Config, MkboilerError};

pub fn python3(config: &Config) -> Result<(), MkboilerError> {
    let content = "{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = [
    (pkgs.python3.withPackages (python-pkgs: [
      # Insert python packages here
    ]))
  ];

  shellHook = ''
    export PYTHONPATH=$PWD:$PYTHONPATH
    alias py=\"python\";
  '';
}";

    let output = config.output.unwrap_or("./shell.nix"); 
    fs::write(output, content)
        .map_err(|e| MkboilerError::new(format!("could not write to file '{output}': {e}")))
}
