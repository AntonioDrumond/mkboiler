use std::fs::File;
use std::io::{Write};

pub fn default_nix(name: Option<&str>) {
    let content = "{ lib, self, ... }:
{
  imports = [
    # Add imports here
  ];
}
    ";
    let fname = match name {
        Some(s) => s,
        None => "./default.nix",
    };
    let mut fil = File::create(fname).expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}

pub fn nix_module(name: Option<&str>) {
    let content = "{ lib, self, ... }:
{
}
    ";
    let fname = match name {
        Some(s) => s,
        None => "./module.nix",
    };
    let mut fil = File::create(fname).expect("ERROR: Could not open file");
    match write!(fil, "{content}") {
        Ok(()) => (),
        Err(e) => eprintln!("ERROR: Could not write to file.\n{e}"),
    }
}

pub fn shell_nix(name: Option<&str>) {
    let content = "{
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
