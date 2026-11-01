use std::fs;

use crate::{Config, MkboilerError};

pub fn rust(config: &Config) -> Result<(), MkboilerError> {
    let content = "{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = [
    rustc
    rustfmt
    cargo
    gcc
    clippy
  ];

  shellHook = ''
  '';
}";

    let output = config.output.unwrap_or("./shell.nix"); 
    fs::write(output, content)
        .map_err(|e| MkboilerError::new(format!("could not write to file '{output}': {e}")))
}

pub fn iced(config: &Config) -> Result<(), MkboilerError> {
    let content = "{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = [
    rustc
    rustfmt
    cargo
    clippy
    gcc
    expat
    fontconfig
    freetype
    freetype.dev
    libGL
    pkg-config
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    wayland
    libxkbcommon
  ];

  LD_LIBRARY_PATH = builtins.foldl' (a: b: \"${a}:${b}/lib\") \"${pkgs.vulkan-loader}/lib\" buildInputs;

  shellHook = ''
  '';
}";

    let output = config.output.unwrap_or("./shell.nix"); 
    fs::write(output, content)
        .map_err(|e| MkboilerError::new(format!("could not write to file '{output}': {e}")))
}
