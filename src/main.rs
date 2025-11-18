mod nix;
mod c;
mod python;

use std::env;
use crate::{
    nix::*,
    c::*,
    python::*,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let fname = if args.len() > 2 {
            Some(args[2].as_str())
        } else { None };
        match args[1].as_str() {
            "default.nix" => default_nix(fname),
            "shell.nix" => shell_nix(fname),
            "nix_module" | "mod.nix" | "module.nix" => nix_module(fname),
            "c" => c(fname),
            "cc" | "cplusplus" | "c++" | "cpp" => cpp(fname),
            "python13" => python13(fname),
            _ => eprintln!("Unknown boilerplate: {}", args[1]),
        };
    }
    else {
        println!("Usage:");
        println!("mkboiler [boiler type] [file name (optional)]")
    }
}
