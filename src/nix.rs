use std::fs;

use crate::{Config, MkboilerError};

pub fn default(config: &Config) -> Result<(), MkboilerError> {
    let content = "{ lib, self, ... }:
{
  imports = [
    # Add imports here
  ];
}";

    let output = config.output.unwrap_or("./default.nix"); 
    fs::write(output, content)
        .map_err(|e| MkboilerError::new(format!("could not write to file '{output}': {e}")))
}

pub fn module(config: &Config) -> Result<(), MkboilerError> {
    let content = "{ lib, self, ... }:
{
}";

    let output = config.output.unwrap_or("./module.nix"); 
    fs::write(output, content)
        .map_err(|e| MkboilerError::new(format!("could not write to file '{output}': {e}")))
}

pub fn shell(config: &Config) -> Result<(), MkboilerError> {
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
}";

    let output = config.output.unwrap_or("./shell.nix"); 
    fs::write(output, content)
        .map_err(|e| MkboilerError::new(format!("could not write to file '{output}': {e}")))
}
