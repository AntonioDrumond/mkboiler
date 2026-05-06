mod c;
mod nix;
mod python;
mod rust;

use crate::{c::*, nix::*, python::*, rust::*};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let fname = if args.len() > 2 {
            Some(args[2].as_str())
        } else {
            None
        };
        match args[1].as_str() {
            "default.nix" => default_nix(fname),
            "shell.nix" => shell_nix(fname),
            "nix_module" | "mod.nix" | "module.nix" => nix_module(fname),
            "c" => c(fname),
            "cc" | "cplusplus" | "c++" | "cpp" => cpp(fname),
            "python3" => python3(fname),
            "rustshell" | "rustShell" | "rust_shell" => rustShell(fname),
            "icedshell" | "icedShell" | "iced_shell" => icedShell(fname),
            _ => eprintln!("Unknown boilerplate: {}", args[1]),
        };
    } else {
        println!("Usage:");
        println!("mkboiler [boiler type] [file name (optional)]")
    }
}
