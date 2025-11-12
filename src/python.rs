use std::fs::File;
use std::io::{Write};

pub fn python13() {
    let content = "default 
{
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
  '';
}
    ";
    let mut fil = File::create("./shell.nix").expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}
