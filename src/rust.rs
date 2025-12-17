use std::fs::File;
use std::io::{Write};

pub fn rustShell(name: Option<&str>) {
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

pub fn icedShell(name: Option<&str>) {
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
