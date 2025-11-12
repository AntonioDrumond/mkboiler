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
    let mut i = 1;
    while i < args.len() {
        // println!("{}", args[i]);
        match args[i].as_str() {
            "default.nix" => default_nix(),
            "c" => c(),
            "cc" | "cplusplus" | "c++" | "cpp" => cpp(),
            "shell.nix" => shell_nix(),
            "python13" => python13(),
            _ => eprintln!("Unknown boilerplate: {}", args[i]),
        };
        i += 1;
    }
}
