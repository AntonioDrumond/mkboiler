use std::fs::File;
use std::io::{Write};

pub fn python13(name: Option<&str>) {
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
}
    ";
    let fname = match name {
        Some(s) => s,
        None => "./shell.nix",
    };
    let mut fil = File::create(fname).expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}
