use std::fs::File;
use std::io::{self, Error, Write};

pub fn default_nix() {
    let content = "default 
dot
nix
    ";
    let mut fil = File::create("./default.nix").expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}

pub fn shell_nix() {
    let content = "default 
{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Add the packages here
  ];

  shellHook = ''
    # Add start script here
  '';
}
    ";
    let mut fil = File::create("./default.nix").expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}
